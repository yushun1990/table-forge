use iced::widget::canvas::Cache;
use serde::{Deserialize, Serialize};
use std::{fmt::Write, sync::Arc};

use crate::util::TablePoint;

#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    pub name: String,
    pub schema: String,
    pub subject_area: String,
    pub position: TablePoint,
    pub columns: Vec<Column>,
    pub foreign_keys: Vec<ForeignKey>,
    pub visible: bool,
    #[serde(skip)]
    pub cache: Arc<Cache>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Column {
    pub name: String,
    pub data_type: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ForeignKey {
    pub column: String,
    pub referenced_table: String,
}

impl Clone for Table {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            schema: self.schema.clone(),
            subject_area: self.subject_area.clone(),
            position: self.position.clone(),
            columns: self.columns.clone(),
            foreign_keys: self.foreign_keys.clone(),
            visible: self.visible.clone(),
            cache: Arc::new(Cache::new()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
    pub tables: Vec<Table>,
}

impl Model {
    pub fn to_ddl(&self) -> String {
        let mut ddl = String::new();
        for table in &self.tables {
            if !table.visible {
                continue;
            }
            write!(
                &mut ddl,
                "CREATE TABLE {}.`{}` (\n",
                table.schema, table.name
            )
            .unwrap();
            for (i, col) in table.columns.iter().enumerate() {
                let comma = if i < table.columns.len() - 1 { "," } else { "" };
                write!(&mut ddl, "\t`{}`\t{}{}", col.name, col.data_type, comma).unwrap();
            }
            write!(&mut ddl, ");\n").unwrap();
        }

        ddl
    }
}
