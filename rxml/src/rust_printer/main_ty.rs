use crate::rust_printer::{not_pascal_case_name, print_derives};
use crate::types::{Field, Type};
use crate::{rust_printer, DbcDescription, Objects, Writer};

pub fn create_main_ty(s: &mut Writer, d: &DbcDescription, o: &Objects) {
    let name = d.name();

    if not_pascal_case_name(d.name()) {
        s.wln("#[allow(non_camel_case_types)]");
    }
    print_derives(s, d.fields(), false);
    s.new_struct(d.name(), |s| {
        s.wln(format!("pub rows: Vec<{}Row>,", d.name()));
    });

    s.bodyn(format!("impl DbcTable for {name}"), |s| {
        create_types(s, d);

        create_read(s, d, o);

        create_write(s, d, o);
    });

    if d.primary_key().is_some() {
        s.bodyn(format!("impl Indexable for {name}"), |s| {
            create_index(s, d);
        });
    }
}

fn create_types(s: &mut Writer, d: &DbcDescription) {
    s.wln(format!("type Row = {}Row;", d.name()));
    s.newline();

    // add constants to make generated code more readable
    s.wln(format!(
        "const FILENAME: &'static str = \"{}.dbc\";",
        d.name()
    ));
    s.wln(format!(
        "const FIELD_COUNT: usize = {};",
        d.field_count()
    ));
    s.wln(format!(
        "const ROW_SIZE: usize = {};",
        d.row_size()
    ));
    s.newline();

    s.wln("fn rows(&self) -> &[Self::Row] { &self.rows }");
    s.wln("fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }");
    s.newline();
}

fn create_write(s: &mut Writer, d: &DbcDescription, o: &Objects) {
    s.open_curly("fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error>");

    // header requires the string block size, which we don't know until deduplicating the strings
    s.wln("let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);");
    s.newline();

    // always allocating a string cache, even if not used, simplifies the code
    s.wln(format!(
        "let {} string_cache = StringCache::new();",
        if d.contains_string() {
            "mut"
        } else {
            ""
        }
    ));
    s.newline();

    // write all rows into the buffer while building the string cache
    s.bodyn("for row in &self.rows", |s| {
        for field in d.fields() {
            print_write_field(s, field, o);
        }
    });

    // assert that the buffer matches expected size
    s.wln("assert_eq!(b.len(), self.rows.len() * Self::ROW_SIZE);");

    s.open_curly("let header = DbcHeader");
    s.wln("record_count: self.rows.len() as u32,");
    s.wln("field_count: Self::FIELD_COUNT as u32,");
    s.wln("record_size: Self::ROW_SIZE as u32,");
    s.wln("string_block_size: string_cache.size(),");
    s.closing_curly_with(";");
    s.newline();

    // write header with proper string block size into output writer
    s.wln("w.write_all(&header.write_header())?;");

    // write row data into output writer
    s.wln("w.write_all(&b)?;");

    // write string block into output writer (might be zero size)
    s.wln("w.write_all(string_cache.buffer())?;");

    s.wln("Ok(())");
    s.closing_curly_newline(); // pub fn write_to_bytes
}

fn print_write_field_ty(s: &mut Writer, name: &str, ty: &Type, o: &Objects, prefix: &str, in_array: bool) {
    match ty {
        Type::Float | Type::I8 | Type::I16 | Type::I32 | Type::U8 | Type::U16 | Type::U32 => {
            s.wln(format!(
                "b.write_all(&{prefix}{name}.to_le_bytes())?;",
                prefix = prefix,
                name = name
            ));
        }
        Type::Bool => {
            s.wln(format!(
                "b.write_all(&u8::from({prefix}{name}).to_le_bytes())?;",
                prefix = prefix,
                name = name
            ));
        }
        Type::Bool32 => {
            s.wln(format!(
                "b.write_all(&u32::from({prefix}{name}).to_le_bytes())?;",
                prefix = prefix,
                name = name
            ));
        }
        Type::ForeignKey { table, ty } => {
            if o.table_exists(table) {
                s.wln(format!(
                    "b.write_all(&({prefix}{name}.id as {ty}).to_le_bytes())?;",
                    prefix = prefix,
                    name = name,
                    ty = ty.rust_str(),
                ));
            } else {
                s.wln(format!(
                    "b.write_all(&{prefix}{name}.to_le_bytes())?;",
                    prefix = prefix,
                    name = name
                ));
            }
        }
        Type::PrimaryKey { .. } => {
            s.wln(format!(
                "b.write_all(&{prefix}{name}.id.to_le_bytes())?;",
                prefix = prefix,
                name = name
            ));
        }
        Type::Flag(en) | Type::Enum(en) => {
            s.wln(format!(
                "b.write_all(&({prefix}{name}.as_int() as {ty}).to_le_bytes())?;",
                prefix = prefix,
                name = name,
                ty = en.ty().rust_str(),
            ));
        }
        Type::ExtendedStringRefLoc | Type::StringRefLoc => {
            s.wln(format!(
                "b.write_all(&row.{name}.string_indices_as_array(&mut string_cache))?;"
            ));
        }
        Type::StringRef => {
            s.wln(format!(
                "b.write_all(&string_cache.add_string({ref_prefix}{prefix}{name}).to_le_bytes())?;",
                ref_prefix = if in_array { "" } else { "&" },
                prefix = prefix,
                name = name,
            ));
        }
        Type::Array(array) => {
            s.bodyn(
                format!(
                    "for i in {complex}row.{name}",
                    name = name,
                    complex = if array.ty().is_string() { "&" } else { "" },
                ),
                |s| {
                    print_write_field_ty(s, "i", array.ty(), o, "", true);
                },
            );
        }
    }
}

fn print_write_field(s: &mut Writer, field: &Field, o: &Objects) {
    rust_printer::print_field_comment(s, field);

    print_write_field_ty(s, field.name(), field.ty(), o, "row.", false);

    s.newline();
}

fn create_index(s: &mut Writer, d: &DbcDescription) {
    if d.primary_key().is_none() {
        return;
    }

    s.wln(format!("type PrimaryKey = {name}Key;", name = d.name()));

    s.bodyn(
        "fn get(&self, key: impl TryInto<Self::PrimaryKey>) -> Option<&Self::Row>",
        |s| {
            s.wln("let key = key.try_into().ok()?;");
            s.wln("self.rows.iter().find(|a| a.id.id == key.id)");
        },
    );

    s.body(
        "fn get_mut(&mut self, key: impl TryInto<Self::PrimaryKey>) -> Option<&mut Self::Row>",
        |s| {
            s.wln("let key = key.try_into().ok()?;");
            s.wln("self.rows.iter_mut().find(|a| a.id.id == key.id)");
        },
    );
}

fn create_read(s: &mut Writer, d: &DbcDescription, o: &Objects) {
    s.open_curly("fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError>");

    s.wln("let mut header = [0_u8; HEADER_SIZE];");
    s.wln("b.read_exact(&mut header)?;");
    s.wln("let header = parse_header(&header)?;");
    s.newline();

    s.bodyn("if header.record_size != Self::ROW_SIZE as u32", |s| {
        s.wln("return Err(crate::DbcError::InvalidHeader(");
        s.inc_indent();

        s.open_curly("crate::InvalidHeaderError::RecordSize");
        s.wln("expected: Self::ROW_SIZE as u32,");
        s.wln("actual: header.record_size,");
        s.closing_curly_with(","); // InvalidHeaderError::RecordSize

        s.dec_indent();
        s.wln("));"); // return Err(
    });

    s.bodyn(
        "if header.field_count != Self::FIELD_COUNT as u32",
        |s| {
            s.wln("return Err(crate::DbcError::InvalidHeader(");
            s.inc_indent();

            s.open_curly("crate::InvalidHeaderError::FieldCount");
            s.wln("expected: Self::FIELD_COUNT as u32,");
            s.wln("actual: header.field_count,");
            s.closing_curly_with(","); // InvalidHeaderError::RecordSize

            s.dec_indent();
            s.wln("));"); // return Err(
        },
    );

    s.wln("let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];");
    s.wln("b.read_exact(&mut r)?;");

    if d.contains_string() {
        s.wln("let mut string_block = vec![0_u8; header.string_block_size as usize];");
        s.wln("b.read_exact(&mut string_block)?;");
    }

    s.newline();

    s.wln("let mut rows = Vec::with_capacity(header.record_count as usize);");
    s.newline();

    s.open_curly("for mut chunk in r.chunks(header.record_size as usize)");
    s.wln("let chunk = &mut chunk;");
    s.newline();

    for field in d.fields() {
        print_read_field(s, field, o);
    }
    s.newline();

    s.open_curly(format!("rows.push({}Row", d.name()));

    for field in d.fields() {
        s.wln(format!("{},", field.name()));
    }

    s.closing_curly_with(");");

    s.closing_curly_newline(); // for mut chunk

    s.wln(format!("Ok({} {{ rows, }})", d.name()));
    s.closing_curly_newline(); // fn read_
}

fn print_read_field_ty(s: &mut Writer, ty: &Type, o: &Objects) {
    match ty {
        Type::PrimaryKey { table, ty } => {
            s.wln_no_indent(format!(
                "{table_name}Key::new(crate::util::read_{ty}_le(chunk)?);",
                table_name = table,
                ty = ty.rust_str(),
            ));
        }
        Type::ForeignKey { table, ty } => {
            if o.table_exists(table) {
                s.wln_no_indent(format!(
                    "{table_name}Key::new(crate::util::read_{ty}_le(chunk)?.into());",
                    table_name = table,
                    ty = ty.rust_str(),
                ));
            } else {
                s.wln_no_indent(format!(
                    "crate::util::read_{ty}_le(chunk)?;",
                    ty = ty.rust_str(),
                ));
            }
        }
        Type::I8 => {
            s.wln_no_indent("crate::util::read_i8_le(chunk)?;");
        }
        Type::I16 => {
            s.wln_no_indent("crate::util::read_i16_le(chunk)?;");
        }
        Type::I32 => {
            s.wln_no_indent("crate::util::read_i32_le(chunk)?;");
        }
        Type::U8 => {
            s.wln_no_indent("crate::util::read_u8_le(chunk)?;");
        }
        Type::U16 => {
            s.wln_no_indent("crate::util::read_u16_le(chunk)?;");
        }
        Type::U32 => {
            s.wln_no_indent("crate::util::read_u32_le(chunk)?;");
        }
        Type::Bool => {
            s.wln_no_indent("crate::util::read_u8_le(chunk)? != 0;");
        }
        Type::Bool32 => {
            s.wln_no_indent("crate::util::read_u32_le(chunk)? != 0;");
        }
        Type::ExtendedStringRefLoc => {
            s.wln_no_indent("crate::util::read_extended_localized_string(chunk, &string_block)?;");
        }
        Type::StringRefLoc => {
            s.wln_no_indent("crate::util::read_localized_string(chunk, &string_block)?;");
        }
        Type::StringRef => {
            s.wln_no_indent("{");
            s.inc_indent();
            s.wln("let s = crate::util::get_string_as_vec(chunk, &string_block)?;");
            s.wln("String::from_utf8(s)?");
            s.closing_curly_with(";");
        }
        Type::Flag(en) => {
            s.wln_no_indent(format!(
                "{en}::new(crate::util::read_{ty}_le(chunk)? as _);",
                en = en.name(),
                ty = en.ty().rust_str(),
            ));
        }
        Type::Enum(en) => {
            s.wln_no_indent(format!(
                "crate::util::read_{ty}_le(chunk)?.try_into()?;",
                ty = en.ty().rust_str(),
            ));
        }
        Type::Float => {
            s.wln_no_indent("crate::util::read_f32_le(chunk)?;");
        }
        Type::Array(array) => {
            if array.ty().has_custom_array_impl() {
                s.wln_no_indent(format!(
                    "crate::util::read_array_{ty}::<{size}>(chunk)?;",
                    ty = array.ty().rust_str(),
                    size = array.size()
                ));

                return;
            }

            s.wln_no_indent("{");
            s.inc_indent();

            if array.ty().is_string() {
                s.wln(format!(
                    "let mut arr = Vec::with_capacity({size});",
                    size = array.size()
                ));

                s.bodyn(format!("for _ in 0..{size}", size = array.size()), |s| {
                    s.w("let i =");
                    print_read_field_ty(s, array.ty(), o);
                    s.wln("arr.push(i);");
                });

                s.wln("arr.try_into().unwrap()");
            } else {
                s.wln(format!(
                    "let mut arr = [{ty}::default(); {size}];",
                    ty = array.ty().rust_str(),
                    size = array.size()
                ));

                s.bodyn("for i in arr.iter_mut()", |s| {
                    s.w("*i = ");
                    print_read_field_ty(s, array.ty(), o);
                });

                s.wln("arr");
            }

            s.closing_curly_with(";");
        }
    }
}

fn print_read_field(s: &mut Writer, field: &Field, o: &Objects) {
    rust_printer::print_field_comment(s, field);

    s.w(format!("let {name} = ", name = field.name()));
    print_read_field_ty(s, field.ty(), o);

    s.newline();
}
