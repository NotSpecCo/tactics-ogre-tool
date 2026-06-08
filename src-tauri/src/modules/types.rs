use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DatFile {
    BattleData,
    MenuData,
}

impl DatFile {
    pub fn relative_path(&self) -> &'static str {
        match self {
            DatFile::BattleData => "battle/battle_data_release.dat",
            DatFile::MenuData => "menu/menu_data.dat",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FieldType {
    Uint,
    Int,
    Dropdown,
    Hex,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldOption {
    pub value: i64,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDefinition {
    pub name: String,
    pub offset: usize,
    pub size: usize,
    pub field_type: FieldType,
    pub options: Option<Vec<FieldOption>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleDefinition {
    pub id: String,
    pub name: String,
    pub description: String,
    pub dat_file: DatFile,
    pub base_offset: usize,
    pub entry_count: usize,
    pub entry_size: usize,
    pub entry_names: Vec<String>,
    pub fields: Vec<FieldDefinition>,
}
