use crate::types::{Field, Type};
use crate::{DbcDescription, DbcVersion, Objects, Writer};
use heck::ToSnakeCase;

mod main_ty;
mod sqlite_converter;

pub use sqlite_converter::sqlite_converter;

fn not_pascal_case_name(s: &str) -> bool {
    s.contains('_')
}

pub fn create_table(d: &DbcDescription, o: &Objects, version: DbcVersion) -> Writer {
    let mut s = Writer::new(d.name());

    includes(&mut s, d, o, &version.module_name());

    main_ty::create_main_ty(&mut s, d, o);

    create_primary_keys(&mut s, d);

    create_enums(&mut s, d);

    create_flags(&mut s, d);

    create_row(&mut s, d, o);

    create_test(&mut s, d, &version.test_dir_name());

    s
}

fn includes(s: &mut Writer, d: &DbcDescription, o: &Objects, include_path: &str) {
    s.wln("use crate::header::{HEADER_SIZE, DbcHeader};");
    s.wln("use crate::header;");
    s.wln("use crate::DbcTable;");
    s.wln("use std::io::Write;");

    if d.primary_key().is_some() {
        s.wln("use crate::Indexable;");
    }

    if d.contains_localized_string() {
        s.wln("use crate::LocalizedString;");
    }

    if d.contains_extended_localized_string() {
        s.wln("use crate::ExtendedLocalizedString;");
    }

    if d.contains_gender_enum() {
        s.wln("use crate::Gender;");
    }

    if d.contains_size_class_enum() {
        s.wln("use crate::SizeClass;");
    }

    for foreign_key in d.foreign_keys() {
        if foreign_key == d.name() || !o.table_exists(foreign_key) {
            continue;
        }

        s.wln(format!(
            "use crate::{include_path}::{name}::*;",
            name = foreign_key.to_snake_case()
        ));
    }

    s.newline();
}

fn print_derives(s: &mut Writer, fields: &[Field], derive_copy: bool) {
    s.w("#[derive(Debug, Clone");
    if can_derive_copy(fields) && derive_copy {
        s.w_no_indent(", Copy");
    }

    s.w(", PartialEq");
    if can_derive_eq(fields) {
        s.w_no_indent(", Eq");
    }

    s.w(", PartialOrd");
    if can_derive_ord(fields) {
        s.w(", Ord");
    }

    if can_derive_hash(fields) {
        s.w(", Hash");
    }

    s.wln_no_indent(")]");
}

fn can_derive_copy(fields: &[Field]) -> bool {
    for field in fields {
        match field.ty() {
            Type::ExtendedStringRefLoc | Type::StringRefLoc | Type::StringRef => return false,
            Type::Array(array) => {
                if matches!(
                    array.ty(),
                    Type::ExtendedStringRefLoc | Type::StringRefLoc | Type::StringRef
                ) {
                    return false;
                }
            }
            _ => {}
        }
    }

    true
}
fn can_derive_hash(fields: &[Field]) -> bool {
    does_not_contain_float(fields)
}
fn can_derive_ord(fields: &[Field]) -> bool {
    does_not_contain_float(fields)
}
fn can_derive_eq(fields: &[Field]) -> bool {
    does_not_contain_float(fields)
}

fn does_not_contain_float(fields: &[Field]) -> bool {
    for field in fields {
        match field.ty() {
            Type::Float => return false,
            Type::Array(array) => {
                if matches!(array.ty(), Type::Float) {
                    return false;
                }
            }
            _ => {}
        }
    }

    true
}

fn create_row(s: &mut Writer, d: &DbcDescription, o: &Objects) {
    if not_pascal_case_name(d.name()) {
        s.wln("#[allow(non_camel_case_types)]");
    }

    print_derives(s, d.fields(), true);

    s.new_struct(format!("{}Row", d.name()), |s| {
        for field in d.fields() {
            match field.ty() {
                Type::ForeignKey { table, ty } => {
                    if o.table_exists(table) {
                        s.wln(format!("pub {}: {},", field.name(), field.ty().rust_str()));
                    } else {
                        s.wln(format!("pub {}: {},", field.name(), ty.rust_str()));
                    }
                }
                _ => {
                    s.wln(format!("pub {}: {},", field.name(), field.ty().rust_str()));
                }
            }
        }
    });
}

fn create_primary_keys(s: &mut Writer, d: &DbcDescription) {
    if let Some((key, ty)) = d.primary_key() {
        let native_ty = ty.rust_str();

        if not_pascal_case_name(d.name()) {
            s.wln("#[allow(non_camel_case_types)]");
        }
        s.wln("#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]");
        s.new_struct(key.ty().rust_str(), |s| {
            s.wln(format!("pub id: {}", native_ty));
        });

        s.bodyn(format!("impl {}", key.ty().rust_str()), |s| {
            s.bodyn(
                format!("pub const fn new(id: {}) -> Self", native_ty),
                |s| {
                    s.wln("Self { id }");
                },
            );
        });

        create_primary_key_froms(s, key, ty);
    }
}

fn create_primary_key_froms(s: &mut Writer, key: &Field, ty: &Type) {
    let primary_key = key.ty().rust_str();

    let from_tys = match ty {
        Type::I8 => [Type::I8].as_slice(),
        Type::I16 => [Type::I8, Type::I16, Type::U8].as_slice(),
        Type::I32 => [Type::I8, Type::I16, Type::I32, Type::U8, Type::U16].as_slice(),
        Type::U8 => [Type::U8].as_slice(),
        Type::U16 => [Type::U8, Type::U16].as_slice(),
        Type::U32 => [Type::U8, Type::U16, Type::U32].as_slice(),
        _ => unreachable!("invalid primary key"),
    };

    for t in from_tys {
        let t = t.rust_str();
        s.bodyn(format!("impl From<{t}> for {primary_key}",), |s| {
            s.bodyn(format!("fn from(v: {t}) -> Self"), |s| {
                if t == ty.rust_str() {
                    s.wln("Self::new(v)");
                } else {
                    s.wln("Self::new(v.into())");
                }
            });
        });
    }
}

fn create_enums(s: &mut Writer, d: &DbcDescription) {
    for en in d.enums() {
        if en.name() == "Gender" || en.name() == "SizeClass" {
            continue;
        }

        s.wln("#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]");
        s.new_enum(en.name(), |s| {
            for enumerator in en.enumerators() {
                s.wln(format!("{},", enumerator.name()));
            }
        });

        s.bodyn(format!("impl {}", en.name()), |s| {
            s.open_curly(format!(
                "const fn from_value(value: {}) -> Option<Self>",
                en.ty().rust_str()
            ));

            s.open_curly("Some(match value");

            for en in en.enumerators() {
                s.wln(format!("{} => Self::{},", en.value(), en.name()));
            }
            s.wln("_ => return None,");

            s.closing_curly_with(")");

            s.closing_curly();
        });

        s.bodyn(
            format!("impl TryFrom<{}> for {}", en.ty().rust_str(), en.name()),
            |s| {
                s.wln("type Error = crate::InvalidEnumError;");
                s.bodyn(
                    format!(
                        "fn try_from(value: {}) -> Result<Self, Self::Error>",
                        en.ty().rust_str()
                    ),
                    |s| {
                        s.wln(format!(
                            "Self::from_value(value).ok_or(crate::InvalidEnumError::new(\"{}\", value as i64))",
                            en.name()
                        ));
                    },
                );
            },
        );

        s.bodyn(format!("impl {name}", name = en.name()), |s| {
            s.bodyn(
                format!("pub const fn as_int(&self) -> {}", en.ty().rust_str()),
                |s| {
                    s.bodyn("match self", |s| {
                        for enumerator in en.enumerators() {
                            s.wln(format!(
                                "Self::{name} => {value},",
                                name = enumerator.name(),
                                value = enumerator.value()
                            ));
                        }
                    });
                },
            );
        });

        s.bodyn(format!("impl Default for {name}", name = en.name()), |s| {
            s.bodyn("fn default() -> Self", |s| {
                s.wln(format!(
                    "Self::{}",
                    en.enumerators().first().unwrap().name()
                ));
            });
        });
    }
}

fn create_flags(s: &mut Writer, d: &DbcDescription) {
    for en in d.flags() {
        s.wln("#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]");
        s.new_struct(en.name(), |s| {
            s.wln(format!("value: {},", en.ty().rust_str()));
        });

        s.bodyn(format!("impl {name}", name = en.name()), |s| {
            s.bodyn(
                format!("pub const fn new(value: {}) -> Self", en.ty().rust_str()),
                |s| {
                    s.wln("Self { value }");
                },
            );

            s.bodyn(
                format!("pub const fn as_int(&self) -> {}", en.ty().rust_str()),
                |s| {
                    s.wln("self.value");
                },
            );

            for enumerator in en.enumerators() {
                s.bodyn(
                    format!(
                        "pub const fn {}(&self) -> bool",
                        enumerator.name().to_snake_case()
                    ),
                    |s| {
                        if enumerator.value() != 0 {
                            s.wln(format!("(self.value & {}) != 0", enumerator.value()));
                        } else {
                            s.wln("self.value == 0");
                        }
                    },
                );
            }
        });
    }
}

fn print_field_comment(s: &mut Writer, field: &Field) {
    s.wln(format!("// {}: {}", field.name(), field.ty().str()));
}

fn create_test(s: &mut Writer, d: &DbcDescription, test_dir_name: &str) {
    const BUILD_TESTS: bool = false;
    if !BUILD_TESTS {
        return;
    }

    if d.name() == "CharacterCreateCameras"
        || d.name() == "SoundCharacterMacroLines"
        || d.name() == "SpellAuraNames"
        || d.name() == "SpellEffectNames"
    {
        // These do not have Dbcs
        return;
    }

    s.wln("#[cfg(test)]");
    s.open_curly("mod test");
    s.wln("use super::*;");
    s.newline();

    s.wln("#[test]");
    s.open_curly(format!("fn {name}()", name = d.name().to_snake_case()));

    let ty = d.name();

    s.wln(format!(
        "let contents = include_bytes!(\"../../../{test_dir_name}/{ty}.dbc\");",
    ));
    s.wln(format!("let actual = {ty}::read(&mut contents).unwrap();",));
    s.wln("let mut v = Vec::with_capacity(contents.len());");
    s.wln("actual.write(&mut v).unwrap();");

    s.wln(format!("let new = {ty}::read(&mut v.as_slice()).unwrap();",));
    s.wln("assert_eq!(actual, new);");

    s.closing_curly(); // fn
    s.closing_curly(); // mod test
}
