use iced::{
    Element, Length, Point, Rectangle, Theme,
    mouse::Cursor,
    widget::{
        Canvas,
        canvas::{self, Cache, Frame, Geometry, Program},
    },
};

use crate::model::Table;

#[derive(Default)]
pub struct Diagram {
    cache: canvas::Cache,
    tables: Vec<Table>,
    simplified: bool,
    zoom: f32,
    pan: Point,
    selected_table: Option<String>,
    last_position: Option<Point>,
}

impl Diagram {
    pub fn new(tables: Vec<Table>) -> Self {
        Diagram {
            cache: Cache::new(),
            tables,
            simplified: true,
            zoom: 1.0,
            pan: Point::new(0.0, 0.0),
            ..Default::default()
        }
    }

    pub fn selected_table(&self) -> Option<String> {
        if let Some(selected_table) = &self.selected_table {
            Some(selected_table.to_string())
        } else {
            None
        }
    }

    pub fn tables(&self) -> &Vec<Table> {
        &self.tables
    }

    pub fn tables_mut(&mut self) -> &mut Vec<Table> {
        &mut self.tables
    }

    pub fn set_tables(&mut self, tables: Vec<Table>) {
        self.tables = tables;
        self.clear_cache();
    }

    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}

impl Program<super::Message, Theme> for Diagram {
    type State = Diagram;

    fn draw(
        &self,
        state: &Self::State,
        _renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        todo!()
    }

    fn update(
        &self,
        state: &mut Self::State,
        event: canvas::Event,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> (iced::event::Status, Option<super::Message>) {
        todo!()
    }
}

impl Diagram {
    pub fn view(&self) -> Element<'_, super::Message, Theme, iced::Renderer> {
        Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
