use iced::{
    Element, Padding, Task,
    widget::{button, horizontal_space, row, text},
};
use title::{Title, TitleMessage};

pub mod title;

#[derive(Default)]
pub struct Header {
    title: Title,
}

#[derive(Clone, Debug)]
pub enum HeaderMessage {
    Title(TitleMessage),
}

impl Header {
    pub fn update(&mut self, message: HeaderMessage) -> Task<HeaderMessage> {
        match message {
            HeaderMessage::Title(message) => self.title.update(message).map(HeaderMessage::Title),
        }
    }

    pub fn view(&self) -> Element<HeaderMessage> {
        row![
            self.title.view().map(HeaderMessage::Title),
            horizontal_space(),
            button(text('\u{0f13f}')).width(32).height(32),
            button(text('\u{0f13f}')).width(32).height(32),
            button(text('\u{0f13f}')).width(32).height(32),
            button(text('\u{0f13f}')).width(32).height(32),
            button(text('\u{0f13f}')).width(32).height(32),
            button(text('\u{0f13f}')).width(32).height(32),
            button(text('\u{0f13f}')).width(32).height(32),
            button(text('\u{0f13f}')).width(32).height(32),
            button(text('\u{0f13f}')).width(32).height(32),
            button(text('\u{0f13f}')).width(32).height(32),
            horizontal_space(),
            button(text('\u{0f13f}')).width(32).height(32),
            button(text('\u{0f13f}')).width(32).height(32),
            button(text('\u{0f13f}')).width(32).height(32),
            button(text('\u{0f13f}')).width(32).height(32),
            button(text('\u{0f13f}')).width(32).height(32),
            button(text('\u{0f13f}')).width(32).height(32),
        ]
        .spacing(20)
        .height(62)
        .padding(Padding::default().top(9).bottom(9).left(11).right(20))
        .into()
    }
}
