use iced::{
    Color, Element, Length, Pixels, Point, Rectangle, Size, Theme, Vector,
    mouse::{
        Button, Cursor,
        Event::{self, ButtonPressed},
    },
    widget::{
        Canvas,
        canvas::{self, Cache, Event::Mouse, Frame, Geometry, Path, Program, Stroke},
    },
};

use crate::model::Table;

use super::Message;

#[derive(Default)]
pub struct Diagram {
    cache: canvas::Cache,
    tables: Vec<Table>,
    simplified: bool,
    zoom: f32,
    pan: Vector,
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
            pan: Vector::new(0.0, 0.0),
            ..Default::default()
        }
    }

    pub fn selected_table(&self) -> Option<&String> {
        self.selected_table.as_ref()
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
        let geometry = state
            .cache
            .draw(_renderer, bounds.size(), |frame: &mut canvas::Frame| {
                frame.scale(state.zoom);
                frame.translate(state.pan);

                for table in state.tables.iter().filter(|x| x.visible) {
                    let pos = table.position.0;
                    let rect = Path::rectangle(pos, Size::new(100.0, 50.0));
                    frame.fill(
                        &rect,
                        if state
                            .selected_table
                            .as_ref()
                            .map_or(false, |s| s == &table.name)
                        {
                            Color::from_rgb(0.9, 0.9, 0.6)
                        } else {
                            Color::from_rgb(0.8, 0.8, 0.8)
                        },
                    );

                    frame.fill_text(canvas::Text {
                        content: if state.simplified {
                            table.name.as_str().to_string()
                        } else {
                            format!("{} ({})", table.name, table.columns.len())
                        },
                        position: pos + Vector::new(5.0, 15.0),
                        color: Color::BLACK,
                        size: Pixels(14.0),
                        ..Default::default()
                    });

                    for fk in &table.foreign_keys {
                        if let Some(target) = state
                            .tables
                            .iter()
                            .find(|t| t.name == fk.referenced_table && t.visible)
                        {
                            let path = Path::line(
                                pos + Vector::new(100.0, 25.0),
                                target.position.0 + Vector::new(0.0, 25.0),
                            );
                            frame.stroke(&path, Stroke::default().with_color(Color::BLACK));
                        }
                    }
                }
            });

        vec![geometry]
    }

    fn update(
        &self,
        state: &mut Self::State,
        event: canvas::Event,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> (iced::event::Status, Option<Message>) {
        match event {
            Mouse(ButtonPressed(Button::Right)) => {
                if let Some(pos) = cursor.position_in(bounds) {
                    let adjusted = Point::new(
                        (pos.x - state.pan.x) / state.zoom,
                        (pos.y - state.pan.y) / state.zoom,
                    );
                    state.selected_table = state
                        .tables
                        .iter()
                        .find(|x| {
                            let p = x.position.0;
                            adjusted.x >= p.x
                                && adjusted.x <= p.x + 100.0
                                && adjusted.y >= p.y
                                && adjusted.y <= adjusted.y + 50.0
                        })
                        .map(|t| t.name.clone());
                    if let Some(_) = state.selected_table {
                        return (
                            iced::event::Status::Captured,
                            Some(Message::ShowContextMenu(pos)),
                        );
                    } else {
                        state.last_position = Some(pos);
                        return (iced::event::Status::Captured, None);
                    }
                }
            }
            Mouse(Event::CursorMoved { position }) => {
                if let Some(selected) = &state.selected_table {
                    // Handle table drag
                    if let Some(table) = state
                        .tables
                        .iter_mut()
                        .find(|t| t.name == selected.as_str())
                    {
                        let delta = Vector::new(
                            (position.x - state.last_position.unwrap_or(position).x) / state.zoom,
                            (position.y - state.last_position.unwrap_or(position).y) / state.zoom,
                        );
                        table.position.0 = table.position.0 + delta;
                        // snap to 10-pixel grid
                        table.position.0.x = (table.position.0.x / 10.0).round() * 10.0;
                        table.position.0.y = (table.position.0.y / 10.0).round() * 10.0;
                        state.cache.clear();
                        state.last_position = Some(position);
                        return (iced::event::Status::Captured, None);
                    }
                }
            }
            Mouse(ButtonPressed(Button::Right)) => {
                todo!()
            }
            Mouse(ButtonPressed(Button::Left)) => {
                todo!()
            }
            Mouse(Event::WheelScrolled { delta }) => {
                todo!()
            }
            _ => {}
        }

        (iced::event::Status::Ignored, None)
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
