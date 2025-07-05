use std::cell::RefCell;

use iced::{
    advanced::text,
    alignment,
    widget::{canvas, center, container},
    Element, Font,
    Length::Fill,
    Renderer, Task,
};

#[derive(Default)]
pub struct Content {}

#[derive(Clone, Debug, Copy)]
pub enum ContentMessage {}

impl Content {
    pub fn update(&mut self, _message: ContentMessage) -> Task<ContentMessage> {
        Task::none()
    }

    pub fn view(&self) -> Element<'_, ContentMessage> {
        container(center(canvas(self).width(Fill).height(Fill)))
            .padding(20)
            .width(Fill)
            .height(Fill)
            .into()
    }
}

impl<Message> canvas::Program<Message> for Content {
    type State = RefCell<Vec<canvas::Cache>>;

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        theme: &iced::Theme,
        bounds: iced::Rectangle,
        _cursor: iced::advanced::mouse::Cursor,
    ) -> Vec<canvas::Geometry<Renderer>> {
        let font = Font::with_name("JetBrainsMono NF");
        let palette = theme.extended_palette();
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        frame.fill_text(canvas::Text {
            content: String::from("Under development..."),
            size: 64.0.into(),
            position: frame.center(),
            color: palette.background.strongest.color,
            align_x: text::Alignment::Center,
            align_y: alignment::Vertical::Center,
            font: font,
            ..canvas::Text::default()
        });

        vec![frame.into_geometry()]
    }
}
