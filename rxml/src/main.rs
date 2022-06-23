mod file_utils;
pub mod parser;
mod rust_printer;
pub mod types;
pub mod writer;

use crate::file_utils::overwrite_if_not_same_contents;
use crate::types::DbcDescription;
use crate::writer::Writer;
use std::path::Path;

const XML_LOCATION: &str = "rxml/xml/";
const TABLE_LOCATION: &str = "wow_vanilla_dbc/src/tables";

const BUILD_TESTS: bool = true;

fn main() {
    let paths = std::fs::read_dir(XML_LOCATION)
        .unwrap()
        .filter_map(|a| a.ok());

    let mut o = Objects::new();
    for path in paths {
        let d = parser::parse_dbc_xml_file(&path.path());
        o.push_description(d);
    }

    let mut modules = Vec::with_capacity(o.descriptions().len());

    for d in o.descriptions() {
        let s = rust_printer::create_table(&d, &o);

        modules.push(s.file_name());

        overwrite_if_not_same_contents(
            s.inner(),
            Path::new(&format!("{}/{}.rs", TABLE_LOCATION, s.file_name())),
        );
    }

    modules.sort();

    let mut module_file = Writer::new("");
    for module in modules {
        module_file.wln(format!("pub mod {};", module));
    }

    overwrite_if_not_same_contents(
        module_file.inner(),
        Path::new(&format!("{}/mod.rs", TABLE_LOCATION)),
    );
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
