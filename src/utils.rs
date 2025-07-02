use iced::{
    Background, Color, Element, Font,
    Length::Fill,
    Theme,
    widget::{button, center, container, text, tooltip},
};

#[derive(Clone)]
pub struct IconStyle<'a, Message: Clone + 'a> {
    text_color: Option<Color>,
    btn_width: f32,
    btn_height: f32,
    padding: f32,
    label: Option<&'a str>,
    on_press: Option<Message>,
}

impl<'a, Message: Clone + 'a> IconStyle<'a, Message> {
    pub fn new(width: f32, height: f32, padding: f32) -> Self {
        IconStyle {
            text_color: None,
            btn_width: width,
            btn_height: height,
            padding,
            label: None,
            on_press: None,
        }
    }

    pub fn text_color(mut self, text_color: Color) -> Self {
        self.text_color = Some(text_color);
        self
    }

    pub fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }
}

pub fn icon<'a, Message: Clone + 'a>(
    code: char,
    style: IconStyle<'a, Message>,
) -> Element<'a, Message> {
    let icon: Element<'a, Message> = text(code)
        .font(Font::with_name("JetBrainsMono Nerd Font"))
        .into();
    let btn = button(center(icon))
        .width(style.btn_width)
        .height(style.btn_height)
        .padding(style.padding)
        .style(move |theme: &Theme, status: button::Status| {
            let palette = theme.extended_palette();
            let base_style = button::Style {
                background: Some(Background::Color(palette.background.base.color)),
                text_color: palette.background.strong.color,
                ..button::Style::default()
            };

            println!("button status: {:?}", status);

            match status {
                button::Status::Active => base_style,
                button::Status::Pressed => button::Style {
                    background: Some(Background::Color(palette.background.weakest.color)),
                    text_color: if style.text_color.is_some() {
                        style.text_color.unwrap()
                    } else {
                        palette.background.strongest.color
                    },
                    ..base_style
                },
                button::Status::Hovered => button::Style {
                    background: Some(Background::Color(palette.background.weakest.color)),
                    text_color: palette.background.strongest.color,
                    ..base_style
                },
                button::Status::Disabled => button::Style {
                    background: base_style
                        .background
                        .map(|background| background.scale_alpha(0.5)),
                    text_color: base_style.text_color.scale_alpha(0.5),
                    ..base_style
                },
            }
        });

    if let Some(on_press) = style.on_press {
        tooltip(
            btn.on_press(on_press),
            if style.label.is_some() {
                style.label.unwrap()
            } else {
                ""
            },
            tooltip::Position::Right,
        )
        .style(container::rounded_box)
        .into()
    } else {
        btn.into()
    }
}
