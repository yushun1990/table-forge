use serde::{Deserialize, Serialize};

use crate::util::TablePoint;

#[derive(Clone, Serialize, Deserialize)]
pub struct Table {
    pub name: String,
    pub schema: String,
    pub position: TablePoint,
    pub columns: Vec<Column>,
    pub foreign_keys: Vec<ForeignKey>,
    pub visible: bool
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Column {
    pub name: String,
    pub data_type: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ForeignKey {
    pub column: String,
    pub referenced_table: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Model {
    pub tables: Vec<Table>,
}
