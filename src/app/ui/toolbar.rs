use iced::{
    Element, Length, Theme,
    widget::{button, row},
};

use crate::app::Message;

pub fn toolbar(is_syncing: bool) -> Element<'static, Message, Theme, iced::Renderer> {
    row![
        button("Export JSON").on_press(Message::ExportJson),
        button("Import JSON").on_press(Message::ImportJson),
        button(if is_syncing { "Syncing..." } else { "Sync DB" }).on_press_maybe(if is_syncing {
            None
        } else {
            Some(Message::SyncDatabase)
        }),
    ]
    .spacing(10)
    .padding(10)
    .width(Length::Fill)
    .into()
}
