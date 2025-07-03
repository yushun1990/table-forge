use iced::{
    Element,
    Length::Fill,
    Task,
    widget::{column, text_input},
};

#[derive(Default)]
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
    pub fn update(&mut self, message: TitleMessage) -> Task<TitleMessage> {
        Task::none()
    }

    pub fn view(&self) -> Element<TitleMessage> {
        let input_name = text_input("Untitled project*", &self.name)
            .on_input(TitleMessage::NameInput)
            .size(20)
            .padding(0);

        let input_version = text_input("initail version", &self.version)
            .size(9)
            .padding(0);

        column![input_name, input_version].height(Fill).into()
    }
}
