use content::{Content, ContentMessage};
use header::{Header, HeaderMessage};
use iced::{Element, Task, widget::{column}};

pub mod header;
pub mod content;


#[derive(Default)]
pub struct Primary {
    header: Header,
    content: Content
}

#[derive(Clone, Debug)]
pub enum PrimaryMessage {
    Header(HeaderMessage),
    Content(ContentMessage)
}

impl Primary {
    pub fn update(&mut self, message: PrimaryMessage) -> Task<PrimaryMessage> {
        match message {
            PrimaryMessage::Header(message) => self.header.update(message).map(PrimaryMessage::Header),
            PrimaryMessage::Content(message) => self.content.update(message).map(PrimaryMessage::Content)
        }
    }

    pub fn view(&self) -> Element<PrimaryMessage> {
        column![
            self.header.view().map(PrimaryMessage::Header),
            self.content.view().map(PrimaryMessage::Content)
        ].into()
    }
}
