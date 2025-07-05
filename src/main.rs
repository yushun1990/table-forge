use iced::{widget::row, Alignment::Center, Element, Font, Size, Task, Theme};
use ui::{Primary, PrimaryMessage, Sidebar, SidebarMessage};

mod constants;
mod ui;
mod utils;

fn main() -> iced::Result {
    iced::application(TableForge::new, TableForge::update, TableForge::view)
        .theme(TableForge::theme)
        .font(include_bytes!("../resource/font.ttf").as_slice())
        .default_font(Font::MONOSPACE)
        .window_size(Size::new(1920.0, 1080.0))
        .run()
}

struct TableForge<'a> {
    theme: Theme,
    sidebar: Sidebar<'a>,
    primary: Primary<'a>,
}

#[derive(Clone, Debug)]
enum Message {
    Sidebar(SidebarMessage),
    Primary(PrimaryMessage),
}

impl<'a> TableForge<'a> {
    fn new() -> Self {
        Self {
            theme: Theme::Nord,
            sidebar: Sidebar::new(),
            primary: Primary::default(),
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Sidebar(message) => self.sidebar.update(message).map(Message::Sidebar),
            Message::Primary(message) => self.primary.update(message).map(Message::Primary),
        }
    }

    fn view(&self) -> Element<Message> {
        let sidebar = self.sidebar.view().map(Message::Sidebar);
        let primary = self.primary.view().map(Message::Primary);
        row![sidebar, primary].align_y(Center).into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
