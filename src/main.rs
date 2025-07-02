use iced::{widget::row, Alignment::Center, Element, Font, Task, Theme};
use ui::{Sidebar, sidebar::SidebarMessage};

mod ui;
mod utils;
mod widget;

fn main() -> iced::Result {
    iced::application(TableForge::new, TableForge::update, TableForge::view)
        .theme(TableForge::theme)
        .font(include_bytes!("../resource/font.ttf").as_slice())
        .default_font(Font::MONOSPACE)
        .run()
}

struct TableForge<'a> {
    theme: Theme,
    sidebar: Sidebar<'a, SidebarMessage>,
}

#[derive(Clone, Debug)]
enum Message {
    Sidebar(SidebarMessage),
}

impl<'a> TableForge<'a> {
    fn new() -> Self {
        Self {
            theme: Theme::Nord,
            sidebar: Sidebar::new(),
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        let sidebar = self.sidebar.view().map(Message::Sidebar);
        row![sidebar].align_y(Center).into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
