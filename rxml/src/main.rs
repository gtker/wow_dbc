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
    workspace_directory()
        .join("rxml")
        .join(version.xml_dir_name())
}

fn table_location(version: DbcVersion) -> PathBuf {
    workspace_directory()
        .join("wow_dbc")
        .join("src")
        .join(version.module_name())
}

const BUILD_TESTS: bool = false;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum DbcVersion {
    Vanilla,
    Tbc,
    Wrath,
}

impl DbcVersion {
    pub const fn to_str(&self) -> &'static str {
        match self {
            DbcVersion::Vanilla => "vanilla",
            DbcVersion::Tbc => "tbc",
            DbcVersion::Wrath => "wrath",
        }
    }

    pub fn module_name(&self) -> String {
        format!("{}_tables", self.to_str())
    }

    pub fn test_dir_name(&self) -> String {
        format!("{}-dbc", self.to_str())
    }

    pub fn xml_dir_name(&self) -> String {
        format!("{}_xml", self.to_str())
    }
}

fn main() {
    let expansions = [DbcVersion::Vanilla, DbcVersion::Tbc, DbcVersion::Wrath];

    for version in expansions {
        let paths = std::fs::read_dir(xml_location(version))
            .unwrap()
            .filter_map(|a| a.ok());

        let mut o = Objects::new();
        for path in paths {
            let d = parser::parse_dbc_xml_file(&path.path(), version);
            o.push_description(d);
        }

        let mut modules = Vec::with_capacity(o.descriptions().len());

        for d in o.descriptions() {
            let s = rust_printer::create_table(&d, &o, version);

            modules.push(s.module_name());

            let mut file_path = table_location(version);
            file_path.push(s.file_name());
            overwrite_if_not_same_contents(s.inner(), &file_path);
        }

        modules.sort();

        let mut module_file = Writer::new("");
        for module in modules {
            module_file.wln(format!("pub mod {};", module));
        }

        let mut mod_rs_path = table_location(version);
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

    pub fn descriptions(&self) -> &[DbcDescription] {
        &self.descriptions
    }

    pub fn table_exists(&self, table_name: &str) -> bool {
        self.descriptions.iter().any(|a| a.name() == table_name)
    }
}
