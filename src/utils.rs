use iced::{
    Background, Color, Element,
    Length::Fill,
    Theme,
    padding::all,
    widget::{button, svg},
};
use tf_widget::icon::Icon;

#[derive(Clone, Debug)]
pub struct SvgButtonStyle<'a, Message: Clone + 'a> {
    svg_color: Option<Color>,
    btn_width: f32,
    btn_height: f32,
    padding: f32,
    label: Option<&'a str>,
    on_press: Option<Message>,
    is_active: bool,
}

impl<'a, Message: Clone + 'a> SvgButtonStyle<'a, Message> {
    pub fn new(width: f32, height: f32, padding: f32) -> Self {
        SvgButtonStyle {
            svg_color: None,
            btn_width: width,
            btn_height: height,
            padding,
            label: None,
            on_press: None,
            is_active: false,
        }
    }

    pub fn width(mut self, width: f32) -> Self {
        self.btn_width = width;
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.btn_height = height;
        self
    }

    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    pub fn svg_color(mut self, svg_color: Color) -> Self {
        self.svg_color = Some(svg_color);
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

    pub fn is_active(mut self, is_active: bool) -> Self {
        self.is_active = is_active;
        self
    }

    pub fn active(&mut self) {
        self.is_active = true;
    }

    pub fn inactive(&mut self) {
        self.is_active = false;
    }
}

pub fn svg_logo<'a, Message: Clone + 'a>(
    svg_path: &'a str,
    style: SvgButtonStyle<'a, Message>,
) -> Element<'a, Message> {
    let svg = svg(get_handle(svg_path));
    let btn = button(svg)
        .width(style.btn_width)
        .height(style.btn_height)
        .padding(style.padding);
    if let Some(on_press) = style.on_press {
        btn.on_press(on_press).into()
    } else {
        btn.into()
    }
}

pub fn svg_button<'a, Message: Clone + 'a>(
    svg_path: &'a str,
    style: SvgButtonStyle<'a, Message>,
) -> Element<'a, Message> {
    let handle = get_handle(svg_path);
    let icon = Icon::new(handle).width(Fill).height(Fill);
    let btn = button(icon)
        .width(style.btn_width)
        .height(style.btn_height)
        .padding(all(style.padding))
        .style(move |theme: &Theme, status: button::Status| {
            let palette = theme.extended_palette();
            let base_style = button::Style {
                background: Some(Background::Color(if style.is_active {
                    palette.background.weakest.color
                } else {
                    palette.background.base.color
                })),
                text_color: if style.is_active {
                    if style.svg_color.is_some() {
                        style.svg_color.unwrap()
                    } else {
                        palette.background.strongest.color
                    }
                } else {
                    palette.background.strongest.color
                },
                ..button::Style::default()
            };

            println!("button status: {:?}", status);

            match status {
                button::Status::Active => base_style,
                button::Status::Pressed => button::Style {
                    background: Some(Background::Color(palette.background.weakest.color)),
                    text_color: if style.svg_color.is_some() {
                        style.svg_color.unwrap()
                    } else {
                        palette.background.strongest.color
                    },
                    ..base_style
                },
                button::Status::Hovered => button::Style {
                    background: Some(Background::Color(palette.background.weakest.color)),
                    text_color: Color::from_rgb8(0xf1, 0xf4, 0xf7),
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
        // tooltip(
        //     btn.on_press(on_press),
        //     if style.label.is_some() {
        //         style.label.unwrap()
        //     } else {
        //         ""
        //     },
        //     tooltip::Position::Right,
        // )
        // .style(container::rounded_box)
        // .into()
        btn.on_press(on_press).into()
    } else {
        btn.into()
    }
}

fn get_handle<'a>(svg_path: &'a str) -> svg::Handle {
    return svg::Handle::from_path(format!(
        "{}/resource/{}",
        env!("CARGO_MANIFEST_DIR"),
        svg_path
    ));
}
