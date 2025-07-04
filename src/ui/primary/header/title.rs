use std::cell::Cell;

use iced::{
    Border, Color, Element,
    Length::{Fill, FillPortion, Shrink},
    Padding, Task, Theme,
    widget::{column, container, text_input},
};

use crate::utils::{self, Line, line};

pub struct Title {
    name: String,
    show_name_btm_line: Cell<bool>,

    version: String,
    show_version_btm_line: Cell<bool>,
}

#[derive(Debug, Clone)]
pub enum TitleMessage {
    NameInput(String),
    VersionInput(String),
}

impl Title {
    pub fn new() -> Self {
        Self {
            name: String::from("Untitled project*"),
            show_name_btm_line: Cell::new(false),
            version: String::from("initial version"),
            show_version_btm_line: Cell::new(false),
        }
    }
    pub fn update(&mut self, message: TitleMessage) -> Task<TitleMessage> {
        match message {
            TitleMessage::NameInput(message) => {
                self.name = message;
                Task::none()
            }
            TitleMessage::VersionInput(message) => {
                self.version = message;
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<TitleMessage> {
        let input_name = text_input("Untitled project*", &self.name)
            .id("title_name")
            .on_input(TitleMessage::NameInput)
            .width(Fill)
            .size(20)
            .style(|theme: &Theme, status: text_input::Status| {
                match status {
                    text_input::Status::Focused { is_hovered } => {
                        self.show_name_btm_line.set(!is_hovered);
                    }
                    _ => self.show_name_btm_line.set(false),
                }
                text_input::Style {
                    value: Color::from_rgb8(0xf1, 0xf4, 0xf7),
                    border: Border::default().width(0),
                    ..text_input::default(theme, status)
                }
            })
            .padding(0);

        let input_version = text_input("initail version", &self.version)
            .id("title_version")
            .on_input(TitleMessage::VersionInput)
            .width(Fill)
            .size(12)
            .style(|theme: &Theme, status: text_input::Status| {
                println!("status : {:?}", status);
                match status {
                    text_input::Status::Focused { is_hovered: _ } => {
                        self.show_version_btm_line.set(true);
                    }
                    text_input::Status::Hovered => {
                        self.show_name_btm_line.set(true);
                    }
                    _ => self.show_version_btm_line.set(false),
                }

                text_input::Style {
                    value: Color::from_rgb8(0xd3, 0xd6, 0xda),
                    border: Border::default().width(0),
                    ..text_input::default(theme, status)
                }
            })
            .padding(0);

        let line = Line {
            size: 900.0,
            thick: 1.0,
            angle: 0.0.into(),
        };

        let mut col_name = column![input_name,];
        if self.show_name_btm_line.get() {
            col_name = col_name.push(utils::line(line));
        }

        let mut col_version = column![input_version];
        if self.show_version_btm_line.get() {
            col_version = col_version.push(utils::line(line));
        }
        column![col_name, col_version]
            .spacing(5)
            .width(Fill)
            .padding(Padding::default().left(9).right(20))
            .into()
    }
}

impl Default for Title {
    fn default() -> Self {
        Self::new()
    }
}
