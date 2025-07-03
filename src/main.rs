use iced::{time::{self, milliseconds}, widget::row, Alignment::Center, Element, Font, Task, Theme};
use ui::{
    primary::content::ContentMessage, Primary, PrimaryMessage, Sidebar, SidebarMessage
};

mod ui;
mod utils;

fn main() -> iced::Result {
    iced::application(TableForge::new, TableForge::update, TableForge::view)
        .theme(TableForge::theme)
        .font(include_bytes!("../resource/font.ttf").as_slice())
        .default_font(Font::MONOSPACE)
        .subscription(|_| {
            time::every(milliseconds(50))
                .map(|_| Message::Primary(
                    PrimaryMessage::Content(
                        ContentMessage::Tick
                    )
                ))
        })
        .run()
}

struct TableForge<'a> {
    theme: Theme,
    sidebar: Sidebar<'a>,
    primary: Primary
}

#[derive(Clone, Debug)]
enum Message {
    Sidebar(SidebarMessage),
    Primary(PrimaryMessage)
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
