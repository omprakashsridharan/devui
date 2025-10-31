use serde::Serialize;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct TableInfo {
    pub name: String,
    pub schema: String,
    pub columns: Vec<ColumnInfo>,
}

/// Database column information
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ColumnInfo {
    pub name: String,
    pub data_type: String,
    pub is_nullable: bool,
    pub is_primary_key: bool,
    pub default_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enum_values: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foreign_key: Option<ForeignKeyInfo>,
}

/// Foreign key relationship information
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ForeignKeyInfo {
    pub referenced_table: String,
    pub referenced_schema: String,
    pub referenced_column: String,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct TableRow {
    pub columns: HashSet<String>,
    pub data: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct TableData {
    pub rows: Vec<TableRow>,
    pub total_rows: u64,
    pub columns: Vec<ColumnInfo>,
}