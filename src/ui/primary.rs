use content::{Content, ContentMessage};
use header::{Header, HeaderMessage};
use iced::{widget::column, Element, Task};

pub mod content;
pub mod header;

#[derive(Default)]
pub struct Primary<'a> {
    header: Header<'a>,
    content: Content,
}

#[derive(Clone, Debug)]
pub enum PrimaryMessage {
    Header(HeaderMessage),
    Content(ContentMessage),
}

impl<'a> Primary<'a> {
    pub fn update(&mut self, message: PrimaryMessage) -> Task<PrimaryMessage> {
        match message {
            PrimaryMessage::Header(message) => {
                self.header.update(message).map(PrimaryMessage::Header)
            }
            PrimaryMessage::Content(message) => {
                self.content.update(message).map(PrimaryMessage::Content)
            }
        }
    }

    pub fn view(&self) -> Element<PrimaryMessage> {
        column![
            self.header.view().map(PrimaryMessage::Header),
            self.content.view().map(PrimaryMessage::Content)
        ]
        .into()
    }
}
