use crate::types::{Array, DbcDescription, Definer, Enumerator, Field, Type};
use heck::ToUpperCamelCase;
use roxmltree::Node;
use std::fs::read_to_string;
use std::path::Path;

pub fn parse_dbc_xml_file(path: &Path) -> DbcDescription {
    let contents = read_to_string(path).unwrap();
    let dbc = roxmltree::Document::parse(&contents).unwrap();

    let dbc = dbc.root_element();
    assert_eq!(dbc.tag_name().name(), "dbc");

    let table_name = dbc
        .descendants()
        .find(|a| a.tag_name().name() == "name")
        .unwrap()
        .text()
        .unwrap();

    let enums = parse_definers(&dbc, "enum");
    let flags = parse_definers(&dbc, "flag");
    let fields = parse_fields(&dbc, table_name, &enums, &flags);

    DbcDescription::new(table_name, fields, enums, flags)
}

pub const U8_NAME: &str = "uint8";
pub const U16_NAME: &str = "uint16";
pub const U32_NAME: &str = "uint32";
pub const I8_NAME: &str = "int8";
pub const I16_NAME: &str = "int16";
pub const I32_NAME: &str = "int32";
pub const BOOL_NAME: &str = "bool";
pub const BOOL32_NAME: &str = "bool32";
pub const STRING_REF_NAME: &str = "string_ref";
pub const STRING_REF_LOC_NAME: &str = "string_ref_loc";
pub const FLOAT_NAME: &str = "float";

fn parse_type(ty: &str, key: &Option<Type>, enums: &[Definer], flags: &[Definer]) -> Type {
    match ty {
        U8_NAME => {
            if let Some(key) = key {
                key.clone()
            } else {
                Type::U8
            }
        }
        U16_NAME => {
            if let Some(key) = key {
                key.clone()
            } else {
                Type::U16
            }
        }
        U32_NAME => {
            if let Some(key) = key {
                key.clone()
            } else {
                Type::U32
            }
        }
        I8_NAME => {
            if let Some(key) = key {
                key.clone()
            } else {
                Type::I8
            }
        }
        I16_NAME => {
            if let Some(key) = key {
                key.clone()
            } else {
                Type::I16
            }
        }
        I32_NAME => {
            if let Some(key) = key {
                key.clone()
            } else {
                Type::I32
            }
        }
        STRING_REF_LOC_NAME => {
            if key.is_some() {
                panic!()
            }
            Type::StringRefLoc
        }
        STRING_REF_NAME => {
            if key.is_some() {
                panic!()
            }
            Type::StringRef
        }
        FLOAT_NAME => {
            if key.is_some() {
                panic!()
            }
            Type::Float
        }
        BOOL_NAME => {
            if key.is_some() {
                panic!()
            }
            Type::Bool
        }
        BOOL32_NAME => {
            if key.is_some() {
                panic!()
            }
            Type::Bool32
        }
        ty => {
            if ty.contains('[') {
                let (ty, size) = ty.split_once('[').unwrap();
                let size: i32 = size.replace(']', "").parse().unwrap();

                return Type::Array(Box::new(Array::new(
                    parse_type(ty, &None, enums, flags),
                    size,
                )));
            }

            if let Some(e) = enums.iter().find(|a| a.name() == ty) {
                return Type::Enum(Box::new(e.clone()));
            }

            if let Some(e) = flags.iter().find(|a| a.name() == ty) {
                return Type::Flag(Box::new(e.clone()));
            }

            panic!("{}", ty)
        }
    }
}

fn parse_definers(dbc: &Node, definer_ty: &str) -> Vec<Definer> {
    let mut enums = Vec::new();

    let xml_enums = dbc.children().filter(|a| a.tag_name().name() == definer_ty);

    for en in xml_enums {
        let ty = en
            .descendants()
            .find(|a| a.tag_name().name() == "type")
            .unwrap()
            .text()
            .unwrap();
        let ty = parse_type(ty, &None, &enums, &[]);

        let name = en
            .descendants()
            .find(|a| a.tag_name().name() == "name")
            .unwrap()
            .text()
            .unwrap();

        let mut enumerators = Vec::new();

        let options = en
            .descendants()
            .find(|a| a.tag_name().name() == "options")
            .unwrap();

        for option in options
            .descendants()
            .filter(|a| a.tag_name().name() == "option")
        {
            let name = option.attribute("name").unwrap().to_upper_camel_case();

            let value = option.attribute("value").unwrap();
            let value: i32 = if value.contains("0x") {
                i32::from_str_radix(&value.replace("0x", ""), 16).unwrap()
            } else {
                value.parse::<i32>().unwrap()
            };

            enumerators.push(Enumerator::new(&name, value));
        }

        enums.push(Definer::new(name, ty, enumerators));
    }

    enums
}

fn get_field_name(s: &str) -> String {
    let s = s.trim_end_matches('_');
    match s {
        "type" => "ty",
        "enum" => "en",
        s => s,
    }
    .to_string()
}

fn parse_fields(dbc: &Node, table_name: &str, enums: &[Definer], flags: &[Definer]) -> Vec<Field> {
    let xml_fields = dbc.children().filter(|a| a.tag_name().name() == "field");

    let mut fields = Vec::new();

    for field in xml_fields {
        let name = get_field_name(
            field
                .descendants()
                .find(|a| a.tag_name().name() == "name")
                .unwrap()
                .text()
                .unwrap(),
        );

        let ty = field
            .descendants()
            .find(|a| a.tag_name().name() == "type")
            .unwrap()
            .text()
            .unwrap();

        let key = if let Some(key) = field.descendants().find(|a| a.tag_name().name() == "key") {
            let key_ty = key
                .descendants()
                .find(|a| a.tag_name().name() == "type")
                .unwrap()
                .text()
                .unwrap();

            if key_ty == "primary" {
                Some(Type::PrimaryKey {
                    table: table_name.to_string(),
                    ty: Box::new(parse_type(ty, &None, enums, flags)),
                })
            } else if key_ty == "foreign" {
                let table = key
                    .descendants()
                    .find(|a| a.tag_name().name() == "parent")
                    .unwrap()
                    .text()
                    .unwrap();
                Some(Type::ForeignKey {
                    table: table.to_string(),
                    ty: Box::new(parse_type(ty, &None, enums, flags)),
                })
            } else {
                unreachable!()
            }
        } else {
            None
        };

        let ty = parse_type(ty, &key, enums, flags);

        fields.push(Field::new(&name, ty))
    }

    fields
}
