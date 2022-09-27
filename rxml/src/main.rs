mod file_utils;
pub mod parser;
mod rust_printer;
pub mod types;
pub mod writer;

use crate::file_utils::overwrite_if_not_same_contents;
use crate::types::DbcDescription;
use crate::writer::Writer;
use std::path::Path;

const VANILLA_XML_LOCATION: &str = "rxml/vanilla_xml/";
const VANILLA_TABLE_LOCATION: &str = "wow_vanilla_dbc/src/vanilla_tables";
const VANILLA_MODULE_NAME: &str = "vanilla_tables";
const VANILLA_TEST_DIR_NAME: &str = "vanilla";

const TBC_XML_LOCATION: &str = "rxml/tbc_xml/";
const TBC_TABLE_LOCATION: &str = "wow_vanilla_dbc/src/tbc_tables";
const TBC_MODULE_NAME: &str = "tbc_tables";
const TBC_TEST_DIR_NAME: &str = "tbc";

const WRATH_XML_LOCATION: &str = "rxml/wrath_xml/";
const WRATH_TABLE_LOCATION: &str = "wow_vanilla_dbc/src/wrath_tables";
const WRATH_MODULE_NAME: &str = "wrath_tables";
const WRATH_TEST_DIR_NAME: &str = "wrath";

const BUILD_TESTS: bool = false;

#[derive(Debug, Clone)]
struct Expansion {
    xml_location: &'static str,
    tables_location: &'static str,
    module_name: &'static str,
    test_dir_name: &'static str,
}

const EXPANSIONS: [Expansion; 3] = [
    Expansion {
        xml_location: VANILLA_XML_LOCATION,
        tables_location: VANILLA_TABLE_LOCATION,
        module_name: VANILLA_MODULE_NAME,
        test_dir_name: VANILLA_TEST_DIR_NAME,
    },
    Expansion {
        xml_location: TBC_XML_LOCATION,
        tables_location: TBC_TABLE_LOCATION,
        module_name: TBC_MODULE_NAME,
        test_dir_name: TBC_TEST_DIR_NAME,
    },
    Expansion {
        xml_location: WRATH_XML_LOCATION,
        tables_location: WRATH_TABLE_LOCATION,
        module_name: WRATH_MODULE_NAME,
        test_dir_name: WRATH_TEST_DIR_NAME,
    },
];

fn main() {
    for location in EXPANSIONS {
        let paths = std::fs::read_dir(location.xml_location)
            .unwrap()
            .filter_map(|a| a.ok());

        let mut o = Objects::new();
        for path in paths {
            let d = parser::parse_dbc_xml_file(&path.path());
            o.push_description(d);
        }

        let mut modules = Vec::with_capacity(o.descriptions().len());

        for d in o.descriptions() {
            let s =
                rust_printer::create_table(&d, &o, location.module_name, location.test_dir_name);

            modules.push(s.file_name());

            overwrite_if_not_same_contents(
                s.inner(),
                Path::new(&format!(
                    "{}/{}.rs",
                    location.tables_location,
                    s.file_name()
                )),
            );
        }

        modules.sort();

        let mut module_file = Writer::new("");
        for module in modules {
            module_file.wln(format!("pub mod {};", module));
        }

        overwrite_if_not_same_contents(
            module_file.inner(),
            Path::new(&format!("{}/mod.rs", location.tables_location)),
        );
    }
}

#[derive(Default)]
pub struct Objects {
    descriptions: Vec<DbcDescription>,
}

impl Objects {
    pub fn push_description(&mut self, d: DbcDescription) {
        self.descriptions.push(d);
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn descriptions(&self) -> Vec<DbcDescription> {
        self.descriptions.clone()
    }

    pub fn table_exists(&self, table_name: &str) -> bool {
        self.descriptions.iter().any(|a| a.name() == table_name)
    }
}
