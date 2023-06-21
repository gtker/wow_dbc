use crate::parser::{
    BOOL32_NAME, BOOL_NAME, FLOAT_NAME, I16_NAME, I32_NAME, I8_NAME, STRING_REF_LOC_NAME,
    STRING_REF_NAME, U16_NAME, U32_NAME, U8_NAME,
};
use crate::Objects;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct DbcDescription {
    name: String,
    fields: Vec<Field>,
    enums: Vec<Definer>,
    flags: Vec<Definer>,
}

impl DbcDescription {
    pub fn new(name: &str, fields: Vec<Field>, enums: Vec<Definer>, flags: Vec<Definer>) -> Self {
        Self {
            name: name.to_string(),
            fields,
            enums,
            flags,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn fields(&self) -> &[Field] {
        &self.fields
    }
    pub fn enums(&self) -> &[Definer] {
        &self.enums
    }
    pub fn flags(&self) -> &[Definer] {
        &self.flags
    }

    pub fn primary_key(&self) -> Option<(&Field, &Type)> {
        let keys = self.primary_keys();
        assert_eq!(keys.len(), 1);
        Some(keys[0])
    }

    pub fn primary_keys(&self) -> Vec<(&Field, &Type)> {
        self.fields
            .iter()
            .filter_map(|a| {
                if let Type::PrimaryKey { ty, .. } = a.ty() {
                    Some((a, ty.as_ref()))
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn contains_gender_enum(&self) -> bool {
        self.fields.iter().any(|a| match a.ty() {
            Type::Enum(en) => en.name() == "Gender",
            _ => false,
        })
    }

    pub fn contains_size_class_enum(&self) -> bool {
        self.fields.iter().any(|a| match a.ty() {
            Type::Enum(en) => en.name() == "SizeClass",
            _ => false,
        })
    }

    pub fn contains_localized_string(&self) -> bool {
        self.fields.iter().any(|a| match a.ty() {
            Type::StringRefLoc => true,
            Type::Array(array) => matches!(array.ty(), Type::StringRefLoc),
            _ => false,
        })
    }

    pub fn contains_extended_localized_string(&self) -> bool {
        self.fields.iter().any(|a| match a.ty() {
            Type::ExtendedStringRefLoc => true,
            Type::Array(array) => matches!(array.ty(), Type::ExtendedStringRefLoc),
            _ => false,
        })
    }

    pub fn contains_string(&self) -> bool {
        self.fields.iter().any(|a| match a.ty() {
            Type::ExtendedStringRefLoc | Type::StringRef | Type::StringRefLoc => true,
            Type::Array(array) => {
                matches!(
                    array.ty(),
                    Type::ExtendedStringRefLoc | Type::StringRef | Type::StringRefLoc
                )
            }
            _ => false,
        })
    }

    pub fn row_size(&self) -> usize {
        self.fields
            .iter()
            .fold(0, |acc, x| acc + x.ty().row_size_count())
    }

    pub fn field_count(&self) -> usize {
        self.fields
            .iter()
            .map(|a| match a.ty() {
                Type::ExtendedStringRefLoc => 17,
                Type::StringRefLoc => 9,
                Type::Array(array) => array.size() as usize,
                _ => 1,
            })
            .sum()
    }

    pub fn foreign_keys(&self) -> Vec<&str> {
        let mut v = Vec::new();

        for field in self.fields() {
            if let Type::ForeignKey { table, .. } = field.ty() {
                v.push(table.as_str());
            }
        }

        v.sort();
        v.dedup();

        v
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Definer {
    name: String,
    ty: Type,
    enumerators: Vec<Enumerator>,
}

impl Definer {
    pub fn new(name: &str, ty: Type, enumerators: Vec<Enumerator>) -> Self {
        Self {
            name: name.to_string(),
            ty,
            enumerators,
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn ty(&self) -> &Type {
        &self.ty
    }
    pub fn enumerators(&self) -> &Vec<Enumerator> {
        &self.enumerators
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Enumerator {
    name: String,
    value: isize,
}

impl Enumerator {
    pub fn new(name: &str, value: isize) -> Self {
        Self {
            name: name.to_string(),
            value,
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn value(&self) -> isize {
        self.value
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Field {
    name: String,
    ty: Type,
}

impl Field {
    pub fn new(name: &str, ty: Type) -> Self {
        Self {
            name: name.to_string(),
            ty,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn ty(&self) -> &Type {
        &self.ty
    }

    pub fn default_value(&self, o: &Objects) -> String {
        self.ty().default_value(o)
    }
}

pub(crate) const fn string_ref_loc_members() -> &'static [&'static str] {
    &[
        "en_gb", "ko_kr", "fr_fr", "de_de", "en_cn", "en_tw", "es_es", "es_mx", "flags",
    ]
}

pub(crate) const fn extended_string_ref_loc_members() -> &'static [&'static str] {
    &[
        "en_gb",
        "ko_kr",
        "fr_fr",
        "de_de",
        "en_cn",
        "en_tw",
        "es_es",
        "es_mx",
        "ru_ru",
        "ja_jp",
        "pt_pt",
        "it_it",
        "unknown_12",
        "unknown_13",
        "unknown_14",
        "unknown_15",
        "flags",
    ]
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Type {
    I8,
    I16,
    I32,
    U8,
    U16,
    U32,
    Bool,
    Bool32,
    StringRef,
    StringRefLoc,
    ExtendedStringRefLoc,
    Array(Box<Array>),
    PrimaryKey { table: String, ty: Box<Type> },
    ForeignKey { table: String, ty: Box<Type> },
    Enum(Box<Definer>),
    Flag(Box<Definer>),
    Float,
}

impl Type {
    pub fn rust_str(&self) -> String {
        match &self {
            Type::I8 => "i8".to_string(),
            Type::I16 => "i16".to_string(),
            Type::I32 => "i32".to_string(),
            Type::U8 => "u8".to_string(),
            Type::U16 => "u16".to_string(),
            Type::U32 => "u32".to_string(),
            Type::Bool | Type::Bool32 => "bool".to_string(),
            Type::StringRefLoc => "LocalizedString".to_string(),
            Type::ExtendedStringRefLoc => "ExtendedLocalizedString".to_string(),
            Type::StringRef => "String".to_string(),
            Type::Array(array) => {
                format!("[{}; {}]", array.ty.rust_str(), array.size)
            }
            Type::ForeignKey { table, .. } | Type::PrimaryKey { table, .. } => {
                format!("{}Key", table)
            }
            Type::Flag(e) | Type::Enum(e) => e.name().to_string(),
            Type::Float => "f32".to_string(),
        }
    }

    pub fn const_rust_str(&self) -> String {
        match self {
            Type::StringRef => "&'static str".to_string(),
            Type::StringRefLoc => "ConstLocalizedString".to_string(),
            Type::ExtendedStringRefLoc => "ConstExtendedLocalizedString".to_string(),
            Type::Array(array) => {
                format!("[{}; {}]", array.ty.const_rust_str(), array.size)
            }
            _ => self.rust_str(),
        }
    }

    pub fn str(&self) -> String {
        match &self {
            Type::I8 => I8_NAME.to_string(),
            Type::I16 => I16_NAME.to_string(),
            Type::I32 => I32_NAME.to_string(),
            Type::U8 => U8_NAME.to_string(),
            Type::U16 => U16_NAME.to_string(),
            Type::U32 => U32_NAME.to_string(),
            Type::Bool => BOOL_NAME.to_string(),
            Type::Bool32 => BOOL32_NAME.to_string(),
            Type::StringRef => STRING_REF_NAME.to_string(),
            Type::ExtendedStringRefLoc => format!("{} (Extended)", STRING_REF_LOC_NAME),
            Type::StringRefLoc => STRING_REF_LOC_NAME.to_string(),
            Type::Array(array) => {
                format!("{ty}[{size}]", ty = array.ty().str(), size = array.size())
            }
            Type::PrimaryKey { table, ty } => format!("primary_key ({}) {}", table, ty.str()),
            Type::ForeignKey { table, ty } => format!("foreign_key ({}) {}", table, ty.str()),
            Type::Flag(en) | Type::Enum(en) => en.name().to_string(),
            Type::Float => FLOAT_NAME.to_string(),
        }
    }

    pub fn row_size_count(&self) -> usize {
        match &self {
            Type::Bool | Type::I8 | Type::U8 => 1,
            Type::I16 | Type::U16 => 2,
            Type::Float | Type::StringRef | Type::Bool32 | Type::I32 | Type::U32 => 4,
            Type::ExtendedStringRefLoc => 16 * 4 + 4,
            Type::StringRefLoc => 9 * 4,
            Type::PrimaryKey { ty, .. } | Type::ForeignKey { ty, .. } => ty.row_size_count(),
            Type::Flag(en) | Type::Enum(en) => en.ty.row_size_count(),
            Type::Array(array) => array.size() as usize * array.ty().row_size_count(),
        }
    }

    pub fn is_string(&self) -> bool {
        matches!(
            self,
            Type::StringRef | Type::StringRefLoc | Type::ExtendedStringRefLoc
        )
    }

    pub fn has_custom_array_impl(&self) -> bool {
        matches!(self, Type::I32 | Type::U32 | Type::Float)
    }

    pub fn default_value(&self, o: &Objects) -> String {
        match self {
            Type::I8 | Type::I16 | Type::I32 | Type::U8 | Type::U16 | Type::U32 => "0".to_string(),
            Type::Bool | Type::Bool32 => "false".to_string(),
            Type::StringRef => "\"\"".to_string(),
            Type::StringRefLoc => "crate::ConstLocalizedString::empty()".to_string(),
            Type::ExtendedStringRefLoc => {
                "crate::ConstExtendedLocalizedString::empty()".to_string()
            }
            Type::Float => "0.0".to_string(),
            Type::ForeignKey { table, .. } => {
                if o.table_exists(table) {
                    format!("{}::new(0)", self.rust_str())
                } else {
                    "0".to_string()
                }
            }
            Type::Flag(_) | Type::PrimaryKey { .. } => {
                format!("{}::new(0)", self.rust_str())
            }
            Type::Enum(e) => format!(
                "{}::{}",
                self.rust_str(),
                e.enumerators().first().unwrap().name()
            ),
            Type::Array(array) => {
                let value = array.ty.default_value(o);
                let size = array.size;
                format!("[{value}; {size}]")
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Array {
    ty: Type,
    size: i32,
}

impl Array {
    pub fn new(ty: Type, size: i32) -> Self {
        Self { ty, size }
    }
    pub fn ty(&self) -> &Type {
        &self.ty
    }
    pub fn size(&self) -> i32 {
        self.size
    }
}
