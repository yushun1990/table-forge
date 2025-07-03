use iced::{
    alignment::Horizontal::Left, widget::{button, center, horizontal_space, row, text}, Alignment::Center, Element, Font, Padding, Task
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
        const FONT: Font = Font::with_name("JetBrainsMono NF");
        row![
            self.title.view().map(HeaderMessage::Title),
            horizontal_space(),
            button(center(text('\u{0eaf8}').font(FONT).size(20))).width(32).height(32).padding(0),
            button(center(text('\u{0eaf1}').font(FONT).size(20))).width(32).height(32).padding(0),
            button(center(text('\u{0eaf1}').font(FONT).size(20))).width(32).height(32).padding(0),
            button(center(text('\u{0eaf1}').font(FONT).size(20))).width(32).height(32).padding(0),
            button(center(text('\u{0eaf1}').font(FONT).size(20))).width(32).height(32).padding(0),
            button(center(text('\u{0eaf1}').font(FONT).size(20))).width(32).height(32).padding(0),
            button(center(text('\u{0eaf1}').font(FONT).size(20))).width(32).height(32).padding(0),
            button(center(text('\u{0eaf1}').font(FONT).size(20))).width(32).height(32).padding(0),
            button(center(text('\u{0eaf1}').font(FONT).size(20))).width(32).height(32).padding(0),
            button(center(text('\u{0eaf1}').font(FONT).size(20))).width(32).height(32).padding(0),
            button(center(text('\u{0eaf1}').font(FONT).size(20))).width(32).height(32).padding(0),
            button(center(text('\u{0eaf1}').font(FONT).size(20))).width(32).height(32).padding(0),
            button(center(text('\u{0eaf1}').font(FONT).size(20))).width(32).height(32).padding(0),
            button(center(text('\u{0eaf1}').font(FONT).size(20))).width(32).height(32).padding(0),
            button(center(text('\u{0eaf1}').font(FONT).size(20))).width(32).height(32).padding(0),
            horizontal_space(),
            button(center(text('\u{0f123}').font(FONT).size(20))).width(32).height(32).padding(0),
            button(center(text('\u{0f123}').font(FONT).size(20))).width(32).height(32).padding(0),
            button(center(text('\u{0f123}').font(FONT).size(20))).width(32).height(32).padding(0),
            button(center(text('\u{0f123}').font(FONT).size(20))).width(32).height(32).padding(0),
            button(center(text('\u{0f123}').font(FONT).size(20))).width(32).height(32).padding(0),
        ]
        .spacing(20)
        .padding(Padding::default().right(32))
        .height(62)
        .align_y(Center)
        .into()
    }
}
