use iced::{
    Element, Length, Point,
    widget::{
        Canvas,
        canvas::{self, Frame},
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
}
