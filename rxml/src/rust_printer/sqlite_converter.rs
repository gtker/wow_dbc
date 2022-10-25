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

    create_and_insert_functions(&mut s, descriptions);

    s
}

fn write_to_sqlite_function(s: &mut Writer, descriptions: &[DbcDescription], o: &Objects) {
    s.open_curly("pub(crate) fn write_to_sqlite(file_name: &str, file_contents: &mut &[u8], options: &Options) -> Result<(), SqliteError>");
    s.wln("let mut conn = Connection::open(&options.output_path)?;");

    s.open_curly("match file_name");

    for description in descriptions {
        s.open_curly(format!("\"{}.dbc\" =>", description.name()));

        s.wln(format!(
            "let data = {module}::{ty}::read(file_contents)?;",
            module = description.name().to_snake_case(),
            ty = description.name(),
        ));
        s.wln(format!("let (table, insert) = {}();", description.name()));
        s.wln("let tx = conn.transaction()?;");
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

        s.wln("tx.commit()?;");

        s.closing_curly(); // => {
    }

    s.wln("v => return Err(SqliteError::FilenameNotFound { name: v.to_string() }),");
    s.closing_curly(); // match file_name

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

fn create_and_insert_functions(s: &mut Writer, descriptions: &[DbcDescription]) {
    for description in descriptions {
        s.wln("#[allow(non_snake_case)]");
        s.bodyn(
            format!(
                "pub(crate) fn {}() -> (&'static str, &'static str)",
                description.name()
            ),
            |s| {
                s.wln("("); // opening tuple

                create_table(s, description);
                s.wln(",");
                create_insert(s, description);

                s.wln(")"); // closing tuple
            },
        )
    }
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
                let (iterator, ty) = match field.ty() {
                    Type::StringRefLoc => (
                        string_ref_loc_members()
                            .iter()
                            .map(|a| a.to_string())
                            .collect::<Vec<_>>(),
                        "TEXT",
                    ),
                    Type::ExtendedStringRefLoc => (
                        extended_string_ref_loc_members()
                            .iter()
                            .map(|a| a.to_string())
                            .collect::<Vec<_>>(),
                        "TEXT",
                    ),
                    Type::Array(array) => (
                        (0..array.size()).map(|a| a.to_string()).collect::<Vec<_>>(),
                        create_table_ty(array.ty()),
                    ),
                    _ => unreachable!(),
                };
                let mut iterator = iterator.iter().peekable();

                while let Some(member) = iterator.next() {
                    s.w(format!(
                        "{name}_{member} {ty} NOT NULL",
                        name = field.name()
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
    s.wln("use crate::{Options, SqliteError};");
    s.wln("use rusqlite::{Connection, params};");
    s.wln("use wow_dbc::DbcTable;");

    s.wln(format!(
        "use wow_dbc::{version}::*;",
        version = version.module_name(),
    ));

    s.newline();
}
