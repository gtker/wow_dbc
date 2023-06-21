use crate::rust_printer::{can_derive_copy, not_pascal_case_name, print_derives};
use crate::types::{DbcDescription, Field, Type};
use crate::writer::Writer;
use crate::{rust_printer, Objects};
use std::ops::Deref;

pub fn create_main_ty(s: &mut Writer, d: &DbcDescription, o: &Objects) {
    let name = format!("Const{}", d.name());
    if not_pascal_case_name(&name) {
        s.wln("#[allow(non_camel_case_types)]");
    }

    let row_name = if can_derive_copy(d.fields()) {
        format!("{}Row", d.name())
    } else {
        format!("{name}Row")
    };

    print_derives(s, d.fields(), true, false);
    s.bodyn(format!("pub struct {name}<const S: usize>"), |s| {
        s.wln(format!("pub rows: [{row_name}; S],"));
    });

    s.bodyn(format!("impl<const S: usize> {name}<S>"), |s| {
        create_read(s, d, o, &row_name);
        s.newline();

        create_conversion(s, d);

        if !d.primary_keys().is_empty() {
            s.wln("// TODO: Indexable?");
        }
    });
}

fn create_conversion(s: &mut Writer, d: &DbcDescription) {
    let name = d.name();
    s.open_curly(format!("pub fn to_owned(&self) -> {name}"));
    s.open_curly(name);

    s.open_curly(format!("rows: self.rows.iter().map(|s| {name}Row"));

    for field in d.fields() {
        let name = field.name();
        let extra = match field.ty() {
            Type::StringRef | Type::StringRefLoc | Type::ExtendedStringRefLoc => ".to_string()",
            Type::Array(array) => {
                if matches!(
                    array.ty(),
                    Type::StringRef | Type::StringRefLoc | Type::ExtendedStringRefLoc
                ) {
                    ".map(|a| a.to_string())"
                } else {
                    ""
                }
            }
            _ => "",
        };

        s.wln(format!("{name}: s.{name}{extra},"));
    }

    s.closing_curly_with(").collect(),");

    s.closing_curly(); // name
    s.closing_curly(); // pub fn to_owned
}

fn create_read(s: &mut Writer, d: &DbcDescription, o: &Objects, row_name: &str) {
    s.open_curly("pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self");

    s.bodyn(format!("if header.record_size != {}", d.row_size()), |s| {
        s.wln(format!(
            "panic!(\"invalid record size, expected {}\")",
            d.row_size()
        ));
    });

    s.bodyn(
        format!("if header.field_count != {}", d.field_count()),
        |s| {
            s.wln(format!(
                "panic!(\"invalid field count, expected {}\")",
                d.field_count()
            ));
        },
    );

    if d.contains_string() {
        s.wln(
            "let string_block = HEADER_SIZE + (header.record_count * header.record_size) as usize;",
        );
        s.wln("let string_block = crate::util::subslice(b, string_block..b.len());");
    }

    s.wln("let mut b_offset = HEADER_SIZE;");
    s.wln("let mut rows = [");
    s.inc_indent();

    s.open_curly(format!("{row_name}"));
    for field in d.fields() {
        let name = field.name();
        let value = field.default_value(o);

        s.wln(format!("{name}: {value},"))
    }
    s.closing_curly();

    s.dec_indent();
    s.wln("; S];");
    s.newline();

    s.wln("let mut i = 0;");

    s.bodyn("while i < S", |s| {
        for field in d.fields() {
            print_read_field(s, field, o);
        }

        s.open_curly(format!("rows[i] = {row_name}"));

        for field in d.fields() {
            s.wln(format!("{},", field.name()));
        }

        s.closing_curly_with(";"); // rows[i]

        s.wln("i += 1;");
    });

    s.wln("Self { rows }");

    s.closing_curly(); // const fn
}

fn print_read_field(s: &mut Writer, field: &Field, o: &Objects) {
    rust_printer::print_field_comment(s, field);
    s.w(format!("let {name} = ", name = field.name()));
    print_read_field_ty(s, field.ty(), o);

    s.newline();
}

fn print_simple_field_ty(ty: &Type) -> &'static str {
    match ty {
        Type::I8 => {
            "i8::from_le_bytes([b[b_offset + 0]])"
        }
        Type::I16 => {
            "i16::from_le_bytes([b[b_offset + 0], b[b_offset + 1])"
        }
        Type::I32 => {
            "i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]])"
        }
        Type::U8 => {
            "u8::from_le_bytes([b[b_offset + 0]])"
        }
        Type::U16 => {
            "u16::from_le_bytes([b[b_offset + 0], b[b_offset + 1])"
        }
        Type::U32 => {
            "u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]])"
        }
        Type::Float => {
            "crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]])"
        }
        Type::Bool => {
             "if b[b_offset + 0] != 0 {true} else {false}"
        }
        Type::Bool32 => {
          "if (b[b_offset + 0] | b[b_offset + 1] | b[b_offset + 2] | b[b_offset + 3]) != 0 {true} else {false}"
        }
        v => unreachable!("{:?}", v),
    }
}

fn print_read_field_ty(s: &mut Writer, ty: &Type, o: &Objects) {
    match ty {
        Type::I8
        | Type::I16
        | Type::I32
        | Type::U8
        | Type::U16
        | Type::U32
        | Type::Float
        | Type::Bool
        | Type::Bool32 => s.wln_no_indent(format!("{};", print_simple_field_ty(ty))),
        Type::PrimaryKey { ty: inner_ty, .. } => s.wln_no_indent(format!(
            "{}::new({});",
            ty.rust_str(),
            print_simple_field_ty(inner_ty)
        )),
        Type::ForeignKey {
            table,
            ty: inner_ty,
        } => {
            if o.table_exists(table) {
                let int = print_simple_field_ty(inner_ty);
                let ty = ty.rust_str();
                let extra = if let Some((_, primary_main_ty)) = o.table_primary_key_ty(table) {
                    if primary_main_ty != inner_ty.deref() {
                        format!("as {}", primary_main_ty.rust_str())
                    } else {
                        String::new()
                    }
                } else {
                    panic!()
                };
                s.wln_no_indent(format!("{ty}::new({int}{extra});"));
            } else {
                s.wln_no_indent(format!("{};", print_simple_field_ty(inner_ty)));
            }
        }
        Type::Flag(e) => {
            let int_ty = print_simple_field_ty(e.ty());

            s.wln_no_indent(format!("{}::new({int_ty});", e.name()));
        }
        Type::Enum(e) => {
            let int_ty = print_simple_field_ty(e.ty());
            let ty_name = e.name();
            if e.name() == "Gender" {
                s.wln_no_indent(format!("match {ty_name}::from_value({int_ty} as i8) {{"));
            } else {
                s.wln_no_indent(format!("match {ty_name}::from_value({int_ty}) {{"));
            }
            s.inc_indent();
            s.wln("Some(e) => e,");
            s.wln("None => panic!(),");
            s.closing_curly_with(";");
        }
        Type::Array(array) => {
            s.wln_no_indent("{");
            s.inc_indent();
            let value = array.ty().default_value(o);
            let size = array.size();
            s.wln(format!("let mut a = [{value}; {size}];"));

            s.wln("let mut i = 0;");
            s.bodyn(format!("while i < a.len()"), |s| {
                s.w(format!("a[i] = "));
                print_read_field_ty(s, array.ty(), o);
                s.wln("i += 1;");
            });

            s.wln("a");
            s.closing_curly_with(";");
        }
        Type::StringRef => {
            s.wln_no_indent("crate::util::get_string_from_block(b_offset, b, string_block);");
        }
        Type::StringRefLoc => {
            s.wln_no_indent("ConstLocalizedString::new(");
            s.inc_indent();

            s.wln("crate::util::get_string_from_block(b_offset, b, string_block),");
            s.wln("crate::util::get_string_from_block(b_offset + 4, b, string_block),");
            s.wln("crate::util::get_string_from_block(b_offset + 8, b, string_block),");
            s.wln("crate::util::get_string_from_block(b_offset + 12, b, string_block),");
            s.wln("crate::util::get_string_from_block(b_offset + 16, b, string_block),");
            s.wln("crate::util::get_string_from_block(b_offset + 20, b, string_block),");
            s.wln("crate::util::get_string_from_block(b_offset + 24, b, string_block),");
            s.wln("crate::util::get_string_from_block(b_offset + 28, b, string_block),");
            s.wln("u32::from_le_bytes([b[b_offset + 32], b[b_offset + 33], b[b_offset + 34], b[b_offset + 35]]),");

            s.dec_indent();
            s.wln(");");
        }
        Type::ExtendedStringRefLoc => {
            s.wln_no_indent("ConstExtendedLocalizedString::new(");
            s.inc_indent();

            s.wln("crate::util::get_string_from_block(b_offset, b, string_block),");
            s.wln("crate::util::get_string_from_block(b_offset + 4, b, string_block),");
            s.wln("crate::util::get_string_from_block(b_offset + 8, b, string_block),");
            s.wln("crate::util::get_string_from_block(b_offset + 12, b, string_block),");
            s.wln("crate::util::get_string_from_block(b_offset + 16, b, string_block),");
            s.wln("crate::util::get_string_from_block(b_offset + 20, b, string_block),");
            s.wln("crate::util::get_string_from_block(b_offset + 24, b, string_block),");
            s.wln("crate::util::get_string_from_block(b_offset + 28, b, string_block),");
            s.wln("crate::util::get_string_from_block(b_offset + 32, b, string_block),");
            s.wln("crate::util::get_string_from_block(b_offset + 36, b, string_block),");
            s.wln("crate::util::get_string_from_block(b_offset + 40, b, string_block),");
            s.wln("crate::util::get_string_from_block(b_offset + 44, b, string_block),");
            s.wln("crate::util::get_string_from_block(b_offset + 48, b, string_block),");
            s.wln("crate::util::get_string_from_block(b_offset + 52, b, string_block),");
            s.wln("crate::util::get_string_from_block(b_offset + 56, b, string_block),");
            s.wln("crate::util::get_string_from_block(b_offset + 60, b, string_block),");
            s.wln("u32::from_le_bytes([b[b_offset + 64], b[b_offset + 65], b[b_offset + 66], b[b_offset + 67]]),");

            s.dec_indent();
            s.wln(");");
        }
    };

    if !matches!(ty, Type::Array(_)) {
        let size = ty.row_size_count();
        s.wln(format!("b_offset += {size};"));
    }
}
