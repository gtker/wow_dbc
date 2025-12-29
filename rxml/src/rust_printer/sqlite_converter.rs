use crate::types::{extended_string_ref_loc_members, string_ref_loc_members, DbcDescription, Type};
use crate::writer::Writer;
use crate::{DbcVersion, Objects};
use heck::ToSnakeCase;

pub fn sqlite_converter(
    descriptions: &[DbcDescription],
    version: DbcVersion,
    o: &Objects,
) -> Writer {
    let mut s = Writer::new_no_name();

    includes(&mut s, version);

    write_to_sqlite_function(&mut s, descriptions, o);

    sqlite_table_functions(&mut s, descriptions);

    create_dispatch_functions(&mut s, descriptions);

    generate_get_sql_for_table(&mut s, descriptions);

    s
}

fn write_to_sqlite_function(s: &mut Writer, descriptions: &[DbcDescription], o: &Objects) {
    s.open_curly("pub(crate) fn write_to_sqlite(conn: &mut Connection, file_name: &str, file_contents: &mut &[u8]) -> Result<(), SqliteError>");

    s.wln("let tx = conn.transaction()?;");
    s.newline();

    s.open_curly("match file_name");

    for description in descriptions {
        s.open_curly(format!("\"{}.dbc\" =>", description.name()));

        s.wln(format!(
            "let data = {module}::{ty}::read(file_contents)?;",
            module = description.name().to_snake_case(),
            ty = description.name(),
        ));
        s.wln(format!(
            "let (table, insert, _select) = {}();",
            description.name()
        ));
        s.wln("tx.execute(table, ())?;");
        s.newline();

        s.open_curly("for row in data.rows()");

        s.wln("tx.execute(insert, params![");

        for field in description.fields() {
            let (prefix, postfix) = insert_prefix_and_postifx(field.ty(), o);
            match field.ty() {
                Type::I8
                | Type::I16
                | Type::I32
                | Type::U8
                | Type::U16
                | Type::U32
                | Type::Bool
                | Type::PrimaryKey { .. }
                | Type::ForeignKey { .. }
                | Type::Enum(_)
                | Type::Flag(_)
                | Type::Float
                | Type::StringRef
                | Type::Bool32 => {
                    s.wln(format!("{prefix}row.{name}{postfix},", name = field.name()));
                }
                Type::StringRefLoc => {
                    for member in string_ref_loc_members() {
                        s.wln(format!("&row.{name}.{member},", name = field.name()));
                    }
                }
                Type::ExtendedStringRefLoc => {
                    for member in extended_string_ref_loc_members() {
                        s.wln(format!("&row.{name}.{member},", name = field.name()));
                    }
                }
                Type::Array(array) => {
                    let (_, postfix) = insert_prefix_and_postifx(array.ty(), o);

                    for member in 0..array.size() {
                        s.wln(format!(
                            "row.{name}[{member}]{postfix},",
                            name = field.name()
                        ));
                    }
                }
            }
        }

        s.wln("])?;");

        s.closing_curly(); // for row in data.rows()

        s.closing_curly(); // => {
    }

    s.wln("v => return Err(SqliteError::FilenameNotFound { name: v.to_string() }),");
    s.closing_curly(); // match file_name

    s.newline();
    s.wln("tx.commit()?;");

    s.newline();
    s.wln("Ok(())");

    s.closing_curly(); // fn write_to_sqlite
}

fn insert_prefix_and_postifx(ty: &Type, o: &Objects) -> (&'static str, &'static str) {
    match ty {
        Type::Float
        | Type::I8
        | Type::I16
        | Type::I32
        | Type::U8
        | Type::U16
        | Type::U32
        | Type::Bool
        | Type::Bool32 => ("", ""),
        Type::StringRef => ("&", ""),
        Type::StringRefLoc => ("&", ""),
        Type::ExtendedStringRefLoc => ("&", ""),
        Type::Array(_) => ("", ""),
        Type::PrimaryKey { .. } => ("", ".id"),
        Type::ForeignKey { table, .. } => {
            if o.table_exists(table) {
                ("", ".id")
            } else {
                ("", "")
            }
        }
        Type::Enum(_) | Type::Flag(_) => ("", ".as_int()"),
    }
}

fn sqlite_table_functions(s: &mut Writer, descriptions: &[DbcDescription]) {
    for description in descriptions {
        s.wln("#[allow(non_snake_case)]");
        s.bodyn(
            format!(
                "pub(crate) fn {}() -> (&'static str, &'static str, &'static str)",
                description.name()
            ),
            |s| {
                s.wln("("); // opening tuple

                create_table(s, description);
                s.wln(",");
                create_insert(s, description);
                s.wln(",");
                create_select(s, description);

                s.wln(")"); // closing tuple
            },
        );
        create_from_rows(s, description);
    }
}

fn create_lookup_dispatch(s: &mut Writer, descriptions: &[DbcDescription]) {
    s.newline();
    s.wln("/// Maps a table name to its conversion logic and writes the result to the provided writer.");
    s.open_curly("pub(crate) fn read_table(name: &str, rows: &mut rusqlite::Rows<'_>, mut writer: impl std::io::Write) -> Result<(), SqliteError>");
    s.open_curly("match name");

    for d in descriptions {
        s.open_curly(format!("\"{}\" =>", d.name()));
        // Call the specific from_rows function
        s.wln(format!("let data = {}_from_rows(rows)?;", d.name().to_snake_case()));
        // Use the DbcTable trait (which data implements) to write the file
        s.wln("data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;");
        s.wln("Ok(())");
        s.closing_curly();
    }

    s.wln("_ => Err(SqliteError::FilenameNotFound { name: name.to_string() }),");
    s.closing_curly(); // match
    s.closing_curly(); // fn
}

fn create_from_rows(s: &mut Writer, description: &DbcDescription) {
    s.newline();
    s.open_curly(format!(
        "pub(crate) fn {module}_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<{module}::{ty}, SqliteError>",
        module = description.name().to_snake_case(),
        ty = description.name(),
    ));

    s.wln("let mut data = Vec::new();");
    s.open_curly("while let Some(row) = rows.next()?");
    s.wln("let row = row;");

    s.open_curly(format!(
        "data.push({module}::{ty}Row",
        module = description.name().to_snake_case(),
        ty = description.name(),
    ));

    let mut index = 0;
    for field in description.fields() {
        match field.ty() {
            Type::PrimaryKey { table, ty } => {
                s.wln(format!(
                    "{name}: row.get::<_, {table_key}>({index})?.into(),",
                    name = field.name(),
                    // table = table.to_snake_case(),
                    table_key = ty.rust_str()
                ));
                index += 1;
            }
            Type::ForeignKey { table, ty } => {
                s.wln(format!(
                    "{name}: row.get::<_, {table_key}>({index})?.into(),",
                    name = field.name(),
                    // table = table.to_snake_case(),
                    table_key = ty.rust_str()
                ));
                index += 1;
            }
            Type::Enum(en) => {
                match en.name()  {
                    "LfgFaction" | "SizeClass" => {
                        s.wln(format!("{field_name}: {module}::{ty}::from_int(row.get::<_, i32>({index})? as i8).map_err(|e| SqliteError::EnumError(e.to_string()))?,",
                                      field_name = field.name(),
                                      module = description.name().to_snake_case(),
                                      ty = en.name()));
                    }
                    "Language" => {
                        s.wln(format!("{field_name}: {module}::{ty}::from_int(row.get::<_, i32>({index})? as u32).map_err(|e| SqliteError::EnumError(e.to_string()))?,",
                                      field_name = field.name(),
                                      module = description.name().to_snake_case(),
                                      ty = en.name()));
                    }
                    _ => {
                        s.wln(format!("{field_name}: {module}::{ty}::from_int(row.get::<_, i32>({index})? as u8).map_err(|e| SqliteError::EnumError(e.to_string()))?,",
                                      field_name = field.name(),
                                      module = description.name().to_snake_case(),
                                      ty = en.name()));
                    }
                }
                index += 1;
            }
            Type::Flag(en) => {
                s.wln(format!(
                    "{name}: row.get::<_, {rust_str}>({index})?.try_into().map_err(|e| SqliteError::EnumError(format!(\"Invalid flag {{e:?}} for {name}\")))?,",
                    name = field.name(),
                    rust_str = en.ty().rust_str()
                ));
                index += 1;
            }
            Type::StringRefLoc => {
                s.open_curly(format!("{name}: LocalizedString", name = field.name()));
                for member in string_ref_loc_members() {
                    match member {
                        &"flags" => {
                            s.wln(format!("{member}: row.get::<_, u32>({index})?.into(),"));
                        }
                        _ => {
                            s.wln(format!("{member}: row.get::<_, String>({index})?.into(),"));
                        }
                    }
                    index += 1;
                }
                s.closing_curly_with(",");
            }
            Type::ExtendedStringRefLoc => {
                s.open_curly(format!(
                    "{name}: ExtendedLocalizedString",
                    name = field.name()
                ));
                for member in extended_string_ref_loc_members() {
                    match member {
                        &"flags" => {
                            s.wln(format!("{member}: row.get::<_, u32>({index})?.into(),"));
                        }
                        _ => {
                            s.wln(format!("{member}: row.get::<_, String>({index})?.into(),"));
                        }
                    }
                    index += 1;
                }
                s.closing_curly_with(",");
            }
            Type::Array(array) => match array.ty() {
                Type::Enum(definer) => {
                    s.w(format!("{name}: [", name = field.name()));
                    for _ in 0..array.size() {
                        match definer.name() {
                            "AuraMod" => {
                                s.wln(format!("{module}::{ty}::from_int(row.get::<_, i32>({index})? as u32).map_err(|e| SqliteError::EnumError(e.to_string()))?,",
                                              module = description.name().to_snake_case(),
                                              ty = definer.name()));
                            }
                            _ => {
                                s.wln(format!("{module}::{ty}::from_int(row.get::<_, i32>({index})? as u8).map_err(|e| SqliteError::EnumError(e.to_string()))?,",
                                              module = description.name().to_snake_case(),
                                              ty = definer.name()));
                            }
                        }
                        index += 1;

                    }
                    s.wln("],");
                }
                Type::Flag(definer) => {
                    s.w(format!("{name}: [", name = field.name()));
                    for _ in 0..array.size() {
                        s.w_no_indent(format!(
                            "row.get::<_, {rust_str}>({index})?.try_into().map_err(|e| SqliteError::EnumError(format!(\"Invalid flag {{e:?}} for {name}\")))?, ",
                            rust_str = definer.ty().rust_str(),
                            name = field.name(),
                        ));
                        index += 1;
                    }
                    s.wln("],");
                }
                Type::I8
                | Type::I16
                | Type::I32
                | Type::U8
                | Type::U16
                | Type::U32
                | Type::Bool
                | Type::Bool32
                | Type::StringRef
                | Type::StringRefLoc
                | Type::ExtendedStringRefLoc
                | Type::Array(_)
                | Type::PrimaryKey { .. }
                | Type::ForeignKey { .. }
                | Type::Float => {
                    s.w(format!("{name}: [", name = field.name()));
                    for _ in 0..array.size() {
                        s.w_no_indent(format!(
                            "row.get::<_, {rust_str}>({index})?.into(), ",
                            rust_str = array.ty().rust_str()
                        ));
                        index += 1;
                    }
                    s.wln("],");
                }
            },
            _ => {
                s.wln(format!(
                    "{name}: row.get::<_, {rust_str}>({index})?.into(),",
                    rust_str = field.ty().rust_str(),
                    name = field.name()
                ));
                index += 1;
            }
        }
    }

    s.closing_curly_with(");"); // End push(...)
    s.closing_curly(); // End while

    s.wln(format!(
        "Ok({module}::{ty} {{ rows: data }})",
        module = description.name().to_snake_case(),
        ty = description.name()
    ));
    s.closing_curly(); // End fn
}

fn create_select(s: &mut Writer, description: &DbcDescription) {
    s.wln("\"SELECT");
    s.inc_indent();

    for (i, field) in description.fields().iter().enumerate() {
        match field.ty() {
            Type::StringRefLoc | Type::ExtendedStringRefLoc | Type::Array(_) => {
                let iterator = match field.ty() {
                    Type::StringRefLoc => string_ref_loc_members()
                        .iter()
                        .map(|a| a.to_string())
                        .collect::<Vec<_>>(),
                    Type::ExtendedStringRefLoc => extended_string_ref_loc_members()
                        .iter()
                        .map(|a| a.to_string())
                        .collect::<Vec<_>>(),
                    Type::Array(array) => {
                        (0..array.size()).map(|a| a.to_string()).collect::<Vec<_>>()
                    }
                    _ => unreachable!(),
                };
                let mut iterator = iterator.iter().peekable();

                while let Some(member) = iterator.next() {
                    s.w(format!("{field_name}_{member}", field_name = field.name()));

                    if iterator.peek().is_some() || i != description.fields().len() - 1 {
                        s.wln_no_indent(",");
                    } else {
                        s.newline();
                    }
                }

                continue;
            }
            _ => {}
        }

        s.w(field.name());
        if i != description.fields().len() - 1 {
            s.wln_no_indent(",");
        } else {
            s.newline();
        }
    }

    s.dec_indent();
    s.wln(format!("FROM `{}`;\"", description.name()));
}



fn create_insert(s: &mut Writer, description: &DbcDescription) {
    s.wln(format!("\"INSERT INTO {} (", description.name()));
    s.inc_indent();

    for (i, field) in description.fields().iter().enumerate() {
        match field.ty() {
            Type::StringRefLoc | Type::ExtendedStringRefLoc | Type::Array(_) => {
                let iterator = match field.ty() {
                    Type::StringRefLoc => string_ref_loc_members()
                        .iter()
                        .map(|a| a.to_string())
                        .collect::<Vec<_>>(),
                    Type::ExtendedStringRefLoc => extended_string_ref_loc_members()
                        .iter()
                        .map(|a| a.to_string())
                        .collect::<Vec<_>>(),
                    Type::Array(array) => {
                        (0..array.size()).map(|a| a.to_string()).collect::<Vec<_>>()
                    }
                    _ => unreachable!(),
                };
                let mut iterator = iterator.iter().peekable();

                while let Some(member) = iterator.next() {
                    s.w(format!("{field_name}_{member}", field_name = field.name()));

                    if iterator.peek().is_some() || i != description.fields().len() - 1 {
                        s.wln_no_indent(",");
                    } else {
                        s.newline();
                    }
                }

                continue;
            }
            _ => {}
        }

        s.w(field.name());
        if i != description.fields().len() - 1 {
            s.wln_no_indent(",");
        } else {
            s.newline();
        }
    }

    s.wln(") VALUES (");

    let mut counter = 1;
    for (i, field) in description.fields().iter().enumerate() {
        match field.ty() {
            Type::StringRefLoc | Type::ExtendedStringRefLoc | Type::Array(_) => {
                let iterator = match field.ty() {
                    Type::StringRefLoc => string_ref_loc_members()
                        .iter()
                        .enumerate()
                        .map(|(i, _)| i)
                        .collect::<Vec<_>>(),
                    Type::ExtendedStringRefLoc => extended_string_ref_loc_members()
                        .iter()
                        .enumerate()
                        .map(|(i, _)| i)
                        .collect::<Vec<_>>(),
                    Type::Array(array) => (0..array.size())
                        .enumerate()
                        .map(|(i, _)| i)
                        .collect::<Vec<_>>(),
                    _ => unreachable!(),
                };
                let mut iterator = iterator.iter().peekable();

                while let Some(_) = iterator.next() {
                    s.w(format!("?{}", counter));

                    if iterator.peek().is_some() || i != description.fields().len() - 1 {
                        s.wln_no_indent(",");
                    } else {
                        s.newline();
                    }

                    counter += 1;
                }

                continue;
            }
            _ => {}
        }

        s.w(format!("?{}", counter));

        if i != description.fields().len() - 1 {
            s.wln_no_indent(",");
        } else {
            s.newline();
        }

        counter += 1;
    }

    s.dec_indent();
    s.wln(");\"");
}

fn create_table(s: &mut Writer, description: &DbcDescription) {
    s.wln(format!(
        "\"CREATE TABLE IF NOT EXISTS {} (",
        description.name()
    ));
    s.inc_indent();

    for (i, field) in description.fields().iter().enumerate() {
        match field.ty() {
            Type::StringRefLoc | Type::ExtendedStringRefLoc | Type::Array(_) => {
                let (iterator) = match field.ty() {
                    Type::StringRefLoc => (
                        string_ref_loc_members()
                            .iter()
                            .map(|a| (a.to_string(), match a { &"flags" => "INTEGER", _ => "TEXT"}))
                            .collect::<Vec<_>>()
                    ),
                    Type::ExtendedStringRefLoc => (
                        extended_string_ref_loc_members()
                            .iter()
                            .map(|a| (a.to_string(), match a { &"flags" => "INTEGER", _ => "TEXT"}))
                            .collect::<Vec<_>>()
                    ),
                    Type::Array(array) => (
                        (0..array.size()).map(|a| (a.to_string(), create_table_ty(array.ty()))).collect::<Vec<_>>()
                    ),
                    _ => unreachable!(),
                };
                let mut iterator = iterator.iter().peekable();

                while let Some(member) = iterator.next() {
                    s.w(format!(
                        "{name}_{member} {ty} NOT NULL",
                        name = field.name(),
                        member = member.0,
                        ty = member.1
                    ));

                    if iterator.peek().is_some() || i != description.fields().len() - 1 {
                        s.wln_no_indent(",");
                    } else {
                        s.newline();
                    }
                }

                continue;
            }
            _ => {}
        }

        let ty = create_table_ty(field.ty());
        let primary_key = match field.ty() {
            Type::PrimaryKey { .. } => "PRIMARY KEY",
            _ => "",
        };

        s.w(format!(
            "{name} {ty} {primary_key} NOT NULL",
            name = field.name()
        ));

        if i != description.fields().len() - 1 {
            s.wln_no_indent(",");
        } else {
            s.newline();
        }
    }

    s.dec_indent();
    s.wln(");\"");
}

fn create_table_ty(ty: &Type) -> &'static str {
    match ty {
        Type::Array(array) => create_table_ty(array.ty()),
        Type::Enum(_)
        | Type::Flag(_)
        | Type::I8
        | Type::I16
        | Type::I32
        | Type::U8
        | Type::U16
        | Type::U32
        | Type::Bool
        | Type::Bool32 => "INTEGER",
        Type::ExtendedStringRefLoc | Type::StringRefLoc | Type::StringRef => "TEXT",
        Type::PrimaryKey { ty, .. } => create_table_ty(ty),
        Type::ForeignKey { ty, .. } => create_table_ty(ty),
        Type::Float => "REAL",
    }
}

fn includes(s: &mut Writer, version: DbcVersion) {
    s.wln("use crate::SqliteError;");
    s.wln("use rusqlite::{Connection, params};");
    s.wln("use wow_dbc::{DbcTable, LocalizedString, ExtendedLocalizedString};");

    s.wln(format!(
        "use wow_dbc::{version}::*;",
        version = version.module_name(),
    ));

    s.newline();
}

fn create_dispatch_functions(s: &mut Writer, descriptions: &[DbcDescription]) {
    s.open_curly("pub(crate) fn generate_dbc_for(name: &str, conn: &rusqlite::Connection, mut writer: impl std::io::Write) -> Result<(), SqliteError>");
    s.open_curly("match name");
    for d in descriptions {
        s.open_curly(format!("\"{}\" =>", d.name()));

        // 1. Use the name to retrieve the sql select statement for the table
        s.wln(format!("let (_create, _insert, select) = {}();", d.name()));

        // 2. Use the connection provided to select the rows from the matching table
        s.wln("let mut stmt = conn.prepare(select)?;");
        s.wln("let mut rows = stmt.query([])?;");

        // 3. Call the specific from_rows function to get the dbc struct
        s.wln(format!("let data = {}_from_rows(&mut rows)?;", d.name().to_snake_case()));

        // 4. Write the dbc struct to the writer
        s.wln("data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;");
        s.wln("Ok(())");

        s.closing_curly();
    }
    s.wln("_ => Err(SqliteError::FilenameNotFound { name: name.to_string() }),");
    s.closing_curly(); // match
    s.closing_curly(); // fn

    s.newline();
}

fn generate_get_sql_for_table(s: &mut Writer, descriptions: &[DbcDescription]) {
    s.open_curly("pub(crate) fn get_sql_for(name: &str) -> Result<(&'static str, &'static str, &'static str), SqliteError>");
    s.open_curly("match name");
    for d in descriptions {
        s.wln(format!("\"{}\" => Ok({}()),", d.name(), d.name()));
    }
    s.wln("_ => Err(SqliteError::FilenameNotFound { name: name.to_string() }),");
    s.closing_curly(); // end match
    s.closing_curly();
}
