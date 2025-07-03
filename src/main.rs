use iced::{Alignment::Center, Element, Font, Task, Theme, widget::row};
use ui::{
    Sidebar,
    header::{Header, HeaderMessage},
    sidebar::SidebarMessage,
};

mod ui;
mod utils;

fn main() -> iced::Result {
    iced::application(TableForge::new, TableForge::update, TableForge::view)
        .theme(TableForge::theme)
        .font(include_bytes!("../resource/font.ttf").as_slice())
        .default_font(Font::MONOSPACE)
        .run()
}

struct TableForge<'a> {
    theme: Theme,
    sidebar: Sidebar<'a>,
    header: Header,
}

#[derive(Clone, Debug)]
enum Message {
    Sidebar(SidebarMessage),
    Header(HeaderMessage),
}

impl<'a> TableForge<'a> {
    fn new() -> Self {
        Self {
            theme: Theme::Nord,
            sidebar: Sidebar::new(),
            header: Header::default(),
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Sidebar(message) => self.sidebar.update(message).map(Message::Sidebar),
            Message::Header(message) => self.header.update(message).map(Message::Header),
        }
    }

    fn view(&self) -> Element<Message> {
        let sidebar = self.sidebar.view().map(Message::Sidebar);
        let header = self.header.view().map(Message::Header);
        row![sidebar, header].align_y(Center).into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
