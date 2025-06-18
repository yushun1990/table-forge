pub mod diagram;
pub mod ui;

use std::sync::Arc;

use diagram::Diagram;
use iced::{Element, Point, Task, Theme, widget::canvas::Cache};

use crate::{
    model::{Model, Table},
    util::TablePoint,
};

#[derive(Clone, Debug)]
pub enum Message {
    SchemaChanged(String),
    SubjectAreaChanged(String),
    SearchChanged(String),
    ExportJson,
    ImportJson,
    ExportDDL,
    SyncDatabase,
    LoadSchema(Result<Model, String>),
    SyncComplete(Result<(), String>),
    CanvasEvent(iced::widget::canvas::Event),
    ShowContextMenu(Point),
    ContextMenuAction(String),
    Undo,
}

#[derive(Default)]
pub struct TableForge {
    schemas: Vec<String>,
    current_schema: String,
    subject_areas: Vec<String>,
    currnet_subject_area: String,
    diagram: Diagram,
    query: String,
    is_syncing: bool,
    context_menu: Option<(Point, String)>,
}

impl TableForge {
    pub fn new() -> Self {
        let mut tables = vec![];
        for i in 1..=10 {
            tables.push(Table {
                name: format!("table{}", i),
                schema: if i <= 5 {
                    "public".to_string()
                } else {
                    "inventory".to_string()
                },
                subject_area: if i <= 3 {
                    "orders".to_string()
                } else {
                    "tables".to_string()
                },
                position: TablePoint(iced::Point {
                    x: (i * 120 % 600) as f32,
                    y: (i / 5 * 60) as f32,
                }),
                columns: vec![crate::model::Column {
                    name: "id".to_string(),
                    data_type: "serial".to_string(),
                }],
                foreign_keys: if i > 1 && i <= 5 {
                    vec![crate::model::ForeignKey {
                        column: "id".to_string(),
                        referenced_table: format!("table{}", i - 1),
                    }]
                } else {
                    vec![]
                },
                visible: i <= 5,
                cache: Arc::new(Cache::new()),
            });
        }
        TableForge {
            schemas: vec!["public".to_string(), "inventory".to_string()],
            subject_areas: vec!["orders".to_string(), "tables".to_string()],
            current_schema: "public".to_string(),
            currnet_subject_area: "orders".to_string(),
            diagram: Diagram::new(tables),
            query: String::new(),
            ..Default::default()
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SchemaChanged(schema) => {
                self.current_schema = schema;
                self.update_table_visibility();
                self.diagram.clear_cache();

                Task::none()
            }
            Message::SubjectAreaChanged(area) => {
                self.currnet_subject_area = area;
                self.update_table_visibility();
                self.diagram.clear_cache();

                Task::none()
            }
            Message::SearchChanged(query) => {
                self.query = query;
                self.update_table_visibility();
                self.diagram.clear_cache();

                Task::none()
            }
            Message::ExportJson => Task::none(),
            Message::ExportDDL => Task::none(),
            Message::ImportJson => Task::none(),
            Message::SyncDatabase => Task::none(),
            Message::Undo => Task::none(),
            Message::LoadSchema(data) => Task::none(),
            Message::SyncComplete(result) => Task::none(),
            Message::CanvasEvent(event) => Task::none(),
            Message::ShowContextMenu(position) => {
                if let Some(table) = self.diagram.selected_table() {
                    self.context_menu = Some((position, String::from(table)));
                }

                Task::none()
            }
            Message::ContextMenuAction(action) => {
                self.context_menu = None;
                if let Some((action, table_name)) = action.split_once(':') {
                    if action == "delete" {
                        self.diagram.tables_mut().retain(|t| t.name != table_name);
                        self.diagram.clear_cache();
                    }
                }
                Task::none()
            }
        }
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
