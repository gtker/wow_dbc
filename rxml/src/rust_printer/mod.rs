use crate::types::{Field, Type};
use crate::{DbcDescription, Objects, Writer, BUILD_TESTS};
use heck::ToSnakeCase;

mod main_ty;

fn not_pascal_case_name(s: &str) -> bool {
    s.contains('_')
}

pub fn create_table(d: &DbcDescription, o: &Objects, include_path: &str, test_dir_name: &str) -> Writer {
    let mut s = Writer::new(d.name());

    includes(&mut s, d, o, include_path);

    main_ty::create_main_ty(&mut s, d, o);

    create_primary_keys(&mut s, d);

    create_enums(&mut s, d);

    create_flags(&mut s, d);

    create_row(&mut s, d, o);

    create_test(&mut s, d, test_dir_name);

    s
}

fn includes(s: &mut Writer, d: &DbcDescription, o: &Objects, include_path: &str) {
    s.wln("use crate::header::{HEADER_SIZE, DbcHeader};");
    s.wln("use crate::header;");
    s.wln("use crate::DbcTable;");
    s.wln("use std::io::Write;");

    if !d.primary_keys().is_empty() {
        s.wln("use crate::Indexable;");
    }

    if d.contains_localized_string() {
        s.wln("use crate::LocalizedString;");
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

fn print_derives(s: &mut Writer ,fields: &[Field])  {
    s.w("#[derive(Debug, Clone, PartialEq");
    if can_derive_eq(fields) {
        s.w_no_indent(", Eq");
    }
    s.wln_no_indent(")]");
}

fn can_derive_eq(fields: &[Field]) -> bool {
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

    print_derives(s, d.fields());

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
    for key in d.primary_keys() {
        let native_ty = match key.ty() {
            Type::PrimaryKey { ty, .. } => ty.rust_str(),
            _ => unreachable!(),
        };
        if not_pascal_case_name(d.name()) {
            s.wln("#[allow(non_camel_case_types)]");
        }
        s.wln("#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]");
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
    }
}

fn create_enums(s: &mut Writer, d: &DbcDescription) {
    for en in d.enums() {
        if en.name() == "Gender" || en.name() == "SizeClass" {
            continue;
        }

        s.wln("#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]");
        s.new_enum(en.name(), |s| {
            for enumerator in en.enumerators() {
                s.wln(format!("{},", enumerator.name()));
            }
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
                        s.open_curly("Ok(match value");

                        for en in en.enumerators() {
                            s.wln(format!("{} => Self::{},", en.value(), en.name()));
                        }
                        s.wln(format!(
                            "val => return Err(crate::InvalidEnumError::new(\"{}\", val as i64)),",
                            en.name()
                        ));

                        s.closing_curly_with(")");
                    },
                );
            },
        );

        s.bodyn(format!("impl {name}", name = en.name()), |s| {
            s.bodyn(
                format!("const fn as_int(&self) -> {}", en.ty().rust_str()),
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
        s.wln("#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Default)]");
        s.new_struct(en.name(), |s| {
            s.wln(format!("value: {},", en.ty().rust_str()));
        });

        s.bodyn(format!("impl {name}", name = en.name()), |s| {
            s.bodyn(
                format!("const fn new(value: {}) -> Self", en.ty().rust_str()),
                |s| {
                    s.wln("Self { value }");
                },
            );

            s.bodyn(
                format!("const fn as_int(&self) -> {}", en.ty().rust_str()),
                |s| {
                    s.wln("self.value");
                },
            );
        });
    }
}

fn print_field_comment(s: &mut Writer, field: &Field) {
    s.wln(format!("// {}: {}", field.name(), field.ty().str()));
}

fn create_test(s: &mut Writer, d: &DbcDescription, test_dir_name: &str) {
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

    s.wln(format!(
        "let contents = include_bytes!(\"../../../{}-dbc/{}.dbc\");",
        test_dir_name,
        d.name(),
    ));
    s.wln(format!(
        "let actual = {}::read(&mut contents.as_slice()).unwrap();",
        d.name()
    ));
    s.wln("let mut v = Vec::with_capacity(contents.len());");
    s.wln("actual.write(&mut v).unwrap();");

    s.wln(format!(
        "let new = {ty}::read(&mut v.as_slice()).unwrap();",
        ty = d.name()
    ));
    s.wln("assert_eq!(actual, new);");

    s.closing_curly(); // fn

    s.closing_curly(); // mod test
}
