use iced::{
    widget::{column, text_input}, Border, Color, Element, Length::{Fill, FillPortion, Shrink}, Padding, Task, Theme
};

pub struct Title {
    name: String,
    version: String,
}

#[derive(Debug, Clone)]
pub enum TitleMessage {
    NameInput(String),
    VersionInput(String),
}

impl Title {
    pub fn new() -> Self {
        Self {
            name: String::from("Untitled project***********************************************************"),
            version: String::from("initial version")
        }
    }
    pub fn update(&mut self, message: TitleMessage) -> Task<TitleMessage> {
        Task::none()
    }

    pub fn view(&self) -> Element<TitleMessage> {
        let input_name = text_input("Untitled project***************************************************", &self.name)
            .on_input(TitleMessage::NameInput)
            .width(Fill)
            .size(20)
            .style(|theme: &Theme, status: text_input::Status| text_input::Style{
                value: Color::from_rgb8(0xf1, 0xf4, 0xf7),
                border: Border::default().width(0),
                ..text_input::default(theme, status)
            })
            .padding(0);

        let input_version = text_input("initail version", &self.version)
            .on_input(TitleMessage::VersionInput)
            .width(Fill)
            .size(12)
            .style(|theme: &Theme, status: text_input::Status| text_input::Style {
                    value: Color::from_rgb8(0xd3, 0xd6, 0xda),
                    border: Border::default().width(0),
                    ..text_input::default(theme, status)
            })
            .padding(0);

        column![input_name, input_version]
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
