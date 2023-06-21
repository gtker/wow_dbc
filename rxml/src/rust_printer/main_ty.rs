use crate::rust_printer::{not_pascal_case_name, print_derives};
use crate::types::{Field, Type};
use crate::{rust_printer, DbcDescription, Objects, Writer};

pub fn create_main_ty(s: &mut Writer, d: &DbcDescription, o: &Objects) {
    if not_pascal_case_name(d.name()) {
        s.wln("#[allow(non_camel_case_types)]");
    }
    print_derives(s, d.fields(), false, false);
    s.new_struct(d.name(), |s| {
        s.wln(format!("pub rows: Vec<{}Row>,", d.name()));
    });

    s.bodyn(format!("impl DbcTable for {name}", name = d.name()), |s| {
        create_types(s, d);

        create_read(s, d, o);

        create_write(s, d, o);
    });

    if !d.primary_keys().is_empty() {
        s.bodyn(format!("impl Indexable for {name}", name = d.name()), |s| {
            create_index(s, d);
        });
    }

    if d.contains_string() {
        s.bodyn(format!("impl {name}", name = d.name()), |s| {
            create_string_size_block(s, d);

            create_string_block_size(s, d);
        });
    }
}

fn create_types(s: &mut Writer, d: &DbcDescription) {
    s.wln(format!("type Row = {}Row;", d.name()));
    s.newline();

    s.wln(format!(
        "fn filename() -> &'static str {{ \"{}.dbc\" }}",
        d.name()
    ));
    s.newline();

    s.wln("fn rows(&self) -> &[Self::Row] { &self.rows }");
    s.wln("fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }");
    s.newline();
}

fn create_write(s: &mut Writer, d: &DbcDescription, o: &Objects) {
    s.open_curly("fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error>");
    s.open_curly("let header = DbcHeader");
    s.wln("record_count: self.rows.len() as u32,");
    s.wln(format!("field_count: {},", d.field_count()));
    s.wln(format!("record_size: {},", d.row_size()));
    if d.contains_string() {
        s.wln("string_block_size: self.string_block_size(),");
    } else {
        s.wln("string_block_size: 1,");
    }
    s.closing_curly_with(";");
    s.newline();

    s.wln("b.write_all(&header.write_header())?;");
    s.newline();
    if d.contains_string() {
        s.wln("let mut string_index = 1;");
    }

    s.bodyn("for row in &self.rows", |s| {
        for field in d.fields() {
            print_write_field(s, field, o);
        }
    });

    if d.contains_string() {
        s.wln("self.write_string_block(b)?;");
    } else {
        s.wln("b.write_all(&[0_u8])?;")
    }
    s.newline();

    s.wln("Ok(())");
    s.closing_curly_newline(); // pub fn write_to_bytes
}

fn print_write_field_ty(s: &mut Writer, name: &str, ty: &Type, o: &Objects, prefix: &str) {
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
                "b.write_all(&row.{name}.string_indices_as_array(&mut string_index))?;"
            ));
        }
        Type::StringRef => {
            s.open_curly(format!(
                "if !{prefix}{name}.is_empty()",
                prefix = prefix,
                name = name,
            ));
            s.wln("b.write_all(&(string_index as u32).to_le_bytes())?;");
            s.wln(format!(
                "string_index += {prefix}{name}.len() + 1;",
                prefix = prefix,
                name = name,
            ));
            s.closing_curly();
            s.open_curly("else");
            s.wln("b.write_all(&(0_u32).to_le_bytes())?;");
            s.closing_curly();
        }
        Type::Array(array) => {
            s.bodyn(
                format!(
                    "for i in {complex}row.{name}",
                    name = name,
                    complex = if array.ty().is_string() { "&" } else { "" },
                ),
                |s| {
                    print_write_field_ty(s, "i", array.ty(), o, "");
                },
            );
        }
    }
}

fn print_write_field(s: &mut Writer, field: &Field, o: &Objects) {
    rust_printer::print_field_comment(s, field);

    print_write_field_ty(s, field.name(), field.ty(), o, "row.");

    s.newline();
}

fn create_string_size_block(s: &mut Writer, d: &DbcDescription) {
    if !d.contains_string() {
        return;
    }

    s.open_curly("fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error>");
    s.wln("b.write_all(&[0])?;");
    s.newline();

    s.open_curly("for row in &self.rows");
    for field in d.fields() {
        match field.ty() {
            Type::StringRef => s.wln(format!(
                "if !row.{name}.is_empty() {{ b.write_all(row.{name}.as_bytes())?; b.write_all(&[0])?; }};",
                name = field.name()
            )),
            Type::ExtendedStringRefLoc |
            Type::StringRefLoc => {
                s.wln(format!("row.{name}.string_block_as_array(b)?;", name = field.name()));
            }
            Type::Array(array) => {
                match array.ty() {
                    Type::StringRef => {
                        s.bodyn(format!("for s in &row.{name}", name = field.name()), |s| {
                             s.wln(
                                "if !s.is_empty() { b.write_all(s.as_bytes())?; b.write_all(&[0])?; };",
                            );
                        });
                    }
                    Type::ExtendedStringRefLoc |
                    Type::StringRefLoc => {
                        s.wln(format!("row.{name}.string_block_as_array(b)?;", name = field.name()));
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    s.closing_curly_newline();

    s.wln("Ok(())");
    s.closing_curly_newline(); // fn write_string_block
}

fn create_string_block_size(s: &mut Writer, d: &DbcDescription) {
    if !d.contains_string() {
        return;
    }

    s.open_curly("fn string_block_size(&self) -> u32");
    s.wln("let mut sum = 1;");

    s.open_curly("for row in &self.rows");
    for field in d.fields() {
        match field.ty() {
            Type::StringRef => s.wln(format!(
                "if !row.{name}.is_empty() {{ sum += row.{name}.len() + 1; }};",
                name = field.name()
            )),
            Type::ExtendedStringRefLoc | Type::StringRefLoc => {
                s.wln(format!(
                    "sum += row.{name}.string_block_size();",
                    name = field.name()
                ));
            }
            Type::Array(array) => match array.ty() {
                Type::StringRef => {
                    s.bodyn(format!("for s in &row.{name}", name = field.name()), |s| {
                        s.wln("if !s.is_empty() { sum += s.len() + 1; };");
                    });
                }
                Type::ExtendedStringRefLoc | Type::StringRefLoc => {
                    s.wln(format!(
                        "sum += row.{name}.string_block_size();",
                        name = field.name()
                    ));
                }
                _ => {}
            },
            _ => {}
        }
    }
    s.closing_curly_newline();

    s.wln("sum as u32");
    s.closing_curly_newline(); // fn string_block_size
}

fn create_index(s: &mut Writer, d: &DbcDescription) {
    if d.primary_keys().is_empty() {
        return;
    }

    s.wln(format!("type PrimaryKey = {name}Key;", name = d.name()));

    s.bodyn(
        "fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row>",
        |s| {
            s.wln("let key = key.into();");
            s.wln("self.rows.iter().find(|a| a.id.id == key.id)");
        },
    );

    s.bodyn(
        "fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row>",
        |s| {
            s.wln("let key = key.into();");
            s.wln("self.rows.iter_mut().find(|a| a.id.id == key.id)");
        },
    );
}

fn create_read(s: &mut Writer, d: &DbcDescription, o: &Objects) {
    s.open_curly("fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError>");

    s.wln("let mut header = [0_u8; HEADER_SIZE];");
    s.wln("b.read_exact(&mut header)?;");
    s.wln("let header = header::parse_header(&header)?;");
    s.newline();

    s.bodyn(format!("if header.record_size != {}", d.row_size()), |s| {
        s.wln("return Err(crate::DbcError::InvalidHeader(");
        s.inc_indent();

        s.open_curly("crate::InvalidHeaderError::RecordSize");
        s.wln(format!("expected: {},", d.row_size()));
        s.wln("actual: header.record_size,");
        s.closing_curly_with(","); // InvalidHeaderError::RecordSize

        s.dec_indent();
        s.wln("));"); // return Err(
    });

    s.bodyn(
        format!("if header.field_count != {}", d.field_count()),
        |s| {
            s.wln("return Err(crate::DbcError::InvalidHeader(");
            s.inc_indent();

            s.open_curly("crate::InvalidHeaderError::FieldCount");
            s.wln(format!("expected: {},", d.field_count()));
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
                "{en}::new(crate::util::read_{ty}_le(chunk)?);",
                en = en.name(),
                ty = en.ty().rust_str(),
            ));
        }
        Type::Enum(en) => {
            s.wln_no_indent(format!(
                "{en}::try_from(crate::util::read_{ty}_le(chunk)?)?;",
                en = en.name(),
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
