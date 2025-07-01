use iced::{
    Length::{self, Fill},
    Padding, Renderer, Theme,
    advanced::Widget,
    widget::{button, center, svg},
};

#[derive(Default, Clone, Copy, Debug)]
pub struct Style {
    pub button_style: button::Style,
    pub svg_style: svg::Style,
}

pub struct SvgButton<'a, Message> {
    button: button::Button<'a, Message>,
    svg_handle: svg::Handle,
    style: Box<dyn Fn(&Theme, button::Status) -> Style + 'a>,
}

impl<'a, Message: Clone + 'a> SvgButton<'a, Message> {
    pub fn new(svg_handle: svg::Handle) -> Self {
        let button = button(center(svg(svg_handle.clone()).width(Fill).height(Fill)));
        Self {
            button,
            svg_handle,
            style: Box::new(|theme: &Theme, status: button::Status| {
                let palette = theme.extended_palette();
                let button_style = match status {
                    button::Status::Active | button::Status::Hovered | button::Status::Pressed => {
                        button::Style {
                            background: Some(palette.background.strong.color.into()),
                            ..button::Style::default()
                        }
                    }
                    button::Status::Disabled => button::Style {
                        background: Some(palette.background.base.color.scale_alpha(0.5).into()),
                        ..button::Style::default()
                    },
                };

                Style {
                    button_style,
                    svg_style: svg::Style::default(),
                }
            }),
        }
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.button = self.button.width(width);
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.button = self.button.height(height);
        self
    }

    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.button = self.button.padding(padding);
        self
    }

    pub fn on_press_with(mut self, on_press: impl Fn() -> Message + 'a) -> Self {
        self.button = self.button.on_press_with(on_press);
        self
    }

    pub fn on_press(mut self, on_press: Message) -> Self {
        self.button = self.button.on_press(on_press);
        self
    }

    pub fn on_press_maybe(mut self, on_press: Option<Message>) -> Self {
        self.button = self.button.on_press_maybe(on_press);
        self
    }

    pub fn clip(mut self, clip: bool) -> Self {
        self.button = self.button.clip(clip);
        self
    }

    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme, button::Status) -> Style + 'a) -> Self {
        self.style = Box::new(style);
        self
    }
}

impl<'a, Message: Clone + 'a> Widget<Message, Theme, Renderer> for SvgButton<'a, Message> {
    fn size(&self) -> iced::Size<Length> {
        self.button.size()
    }

    fn layout(
        &self,
        tree: &mut iced::advanced::widget::Tree,
        renderer: &Renderer,
        limits: &iced::advanced::layout::Limits,
    ) -> iced::advanced::layout::Node {
        self.button.layout(tree, renderer, limits)
    }

    fn draw(
        &self,
        tree: &iced::advanced::widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &iced::advanced::renderer::Style,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        let bounds = layout.bounds();
        let state = tree.state.downcast_ref::<button::State>();


    }
}
