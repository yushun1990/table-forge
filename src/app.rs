pub mod diagram;
pub mod ui;

use diagram::Diagram;

use crate::model::Model;

#[derive(Clone)]
pub enum Messasge {
    SchemaChanged(String),
    SearchChanged(String),
    ExportJson,
    ImportJson,
    SyncDatabase,
    LoadSchema(Result<Model, String>),
    SyncComplete(Result<(), String>),
    CanvasEvent(iced::widget::canvas::Event),
}

pub struct TableForge {
    schemas: Vec<String>,
    current_schema: String,
    diagram: Diagram,
    query: String,
    is_syncing: bool,
}
