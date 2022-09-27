mod file_utils;
pub mod parser;
mod rust_printer;
pub mod types;
pub mod writer;

use crate::file_utils::overwrite_if_not_same_contents;
use crate::types::DbcDescription;
use crate::writer::Writer;
use std::path::PathBuf;

fn workspace_directory() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    assert!(p.pop());
    p
}

fn xml_location(version: DbcVersion) -> PathBuf {
    let p = workspace_directory().join("rxml");
    match version {
        DbcVersion::Vanilla => p.join(VANILLA_XML_LOCATION),
        DbcVersion::Tbc => p.join(TBC_XML_LOCATION),
        DbcVersion::Wrath => p.join(WRATH_XML_LOCATION),
    }
}

fn table_location(version: DbcVersion) -> PathBuf {
    let p = workspace_directory().join("wow_vanilla_dbc").join("src");
    match version {
        DbcVersion::Vanilla => p.join(VANILLA_TABLE_LOCATION),
        DbcVersion::Tbc => p.join(TBC_TABLE_LOCATION),
        DbcVersion::Wrath => p.join(WRATH_TABLE_LOCATION),
    }
}

const VANILLA_XML_LOCATION: &str = "vanilla_xml";
const VANILLA_TABLE_LOCATION: &str = "vanilla_tables";
const VANILLA_MODULE_NAME: &str = "vanilla_tables";
const VANILLA_TEST_DIR_NAME: &str = "vanilla";

const TBC_XML_LOCATION: &str = "tbc_xml";
const TBC_TABLE_LOCATION: &str = "tbc_tables";
const TBC_MODULE_NAME: &str = "tbc_tables";
const TBC_TEST_DIR_NAME: &str = "tbc";

const WRATH_XML_LOCATION: &str = "wrath_xml";
const WRATH_TABLE_LOCATION: &str = "wrath_tables";
const WRATH_MODULE_NAME: &str = "wrath_tables";
const WRATH_TEST_DIR_NAME: &str = "wrath";

const BUILD_TESTS: bool = false;

#[derive(Debug, Clone)]
pub struct Expansion {
    module_name: &'static str,
    test_dir_name: &'static str,
    version: DbcVersion,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum DbcVersion {
    Vanilla,
    Tbc,
    Wrath,
}

fn main() {
    let expansions = [
        Expansion {
            module_name: VANILLA_MODULE_NAME,
            test_dir_name: VANILLA_TEST_DIR_NAME,
            version: DbcVersion::Vanilla,
        },
        Expansion {
            module_name: TBC_MODULE_NAME,
            test_dir_name: TBC_TEST_DIR_NAME,
            version: DbcVersion::Tbc,
        },
        Expansion {
            module_name: WRATH_MODULE_NAME,
            test_dir_name: WRATH_TEST_DIR_NAME,
            version: DbcVersion::Wrath,
        },
    ];

    for location in expansions {
        let paths = std::fs::read_dir(xml_location(location.version))
            .unwrap()
            .filter_map(|a| a.ok());

        let mut o = Objects::new();
        for path in paths {
            let d = parser::parse_dbc_xml_file(&path.path(), location.version);
            o.push_description(d);
        }

        let mut modules = Vec::with_capacity(o.descriptions().len());

        for d in o.descriptions() {
            let s =
                rust_printer::create_table(&d, &o, location.module_name, location.test_dir_name);

            modules.push(s.module_name());

            let mut file_path = table_location(location.version);
            file_path.push(s.file_name());
            overwrite_if_not_same_contents(s.inner(), &file_path);
        }

        modules.sort();

        let mut module_file = Writer::new("");
        for module in modules {
            module_file.wln(format!("pub mod {};", module));
        }

        let mut mod_rs_path = table_location(location.version);
        mod_rs_path.push("mod.rs");
        overwrite_if_not_same_contents(module_file.inner(), &mod_rs_path);
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
