use iced::{Alignment::Center, Element, Task, Theme, widget::row};
use ui::{Sidebar, sidebar::SidebarMessage};

mod ui;
mod widget;

fn main() -> iced::Result {
    iced::application(TableForge::new, TableForge::update, TableForge::view)
        .theme(TableForge::theme)
        .run()
}

struct TableForge {
    theme: Theme,
    sidebar: Sidebar,
}

#[derive(Clone, Debug)]
enum Message {
    Sidebar(SidebarMessage),
}

impl TableForge {
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
