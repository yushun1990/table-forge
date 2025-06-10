pub mod diagram;
pub mod ui;

use diagram::Diagram;
use iced::{Element, Task, Theme};

use crate::model::Model;

#[derive(Clone, Debug)]
pub enum Message {
    SchemaChanged(String),
    SearchChanged(String),
    ExportJson,
    ImportJson,
    SyncDatabase,
    LoadSchema(Result<Model, String>),
    SyncComplete(Result<(), String>),
    CanvasEvent(iced::widget::canvas::Event),
}

#[derive(Default)]
pub struct TableForge {
    schemas: Vec<String>,
    current_schema: String,
    diagram: Diagram,
    query: String,
    is_syncing: bool,
}

impl TableForge {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        todo!()
    }

    pub fn view(&self) -> Element<'_, Message, Theme, iced::Renderer> {
        todo!()
    }

    fn update_table_visibility(&mut self) {
        let query = self.query.to_lowercase();
        for table in self.diagram.tables_mut() {
            table.visible = table.schema == self.current_schema
                && (query.is_empty() || table.name.to_lowercase().contains(&query));
        }
    }
}
