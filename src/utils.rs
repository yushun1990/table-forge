use iced::{
    mouse,
    padding::all,
    widget::{button, canvas},
    Background, Color, Degrees, Element, Length, Point, Radians, Rectangle, Renderer, Theme,
    Vector,
};
use tf_widget::{Handle, Svg};

use crate::constants;

#[derive(Clone, Debug)]
pub struct SvgButton<'a, Message: Clone + 'a> {
    pub svg_path: &'a str,
    pub style: SvgButtonStyle<'a, Message>,
}

#[derive(Clone, Debug)]
pub struct SvgButtonStyle<'a, Message: Clone + 'a> {
    svg_color: Option<Color>,
    btn_width: f32,
    btn_height: f32,
    padding: f32,
    label: Option<&'a str>,
    on_press: Option<Message>,
    custom_svg_style: bool,
    is_active: bool,
    disable_background: bool,
}

#[allow(dead_code)]
impl<'a, Message: Clone + 'a> SvgButtonStyle<'a, Message> {
    pub fn new(width: f32, height: f32, padding: f32) -> Self {
        SvgButtonStyle {
            svg_color: None,
            btn_width: width,
            btn_height: height,
            padding,
            label: None,
            on_press: None,
            custom_svg_style: true,
            is_active: false,
            disable_background: false,
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

    pub fn none_color(mut self) -> Self {
        self.svg_color = None;
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

    pub fn custom_svg_style(mut self, ok: bool) -> Self {
        self.custom_svg_style = ok;
        self
    }

    pub fn enable_background(mut self) -> Self {
        self.disable_background = false;
        self
    }

    pub fn disable_background(mut self) -> Self {
        self.disable_background = true;
        self
    }
}

pub fn svg_button<'a, Message: Clone + 'a>(
    svg_path: &'a str,
    style: SvgButtonStyle<'a, Message>,
) -> Element<'a, Message> {
    let handle = get_handle(svg_path);
    let svg = Svg::new(handle)
        .using_parent_style(style.custom_svg_style)
        .width(Length::Fill)
        .height(Length::Fill);
    let btn = button(svg)
        .width(style.btn_width)
        .height(style.btn_height)
        .padding(all(style.padding))
        .style(move |theme: &Theme, status: button::Status| {
            let palette = theme.extended_palette();
            let base_style = button::Style {
                background: Some(Background::Color(
                    if style.is_active && !style.disable_background {
                        palette.background.weakest.color
                    } else {
                        palette.background.base.color
                    },
                )),
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

            let backgroud_color = Some(Background::Color(if style.disable_background {
                palette.background.base.color
            } else {
                palette.background.weakest.color
            }));
            match status {
                button::Status::Active => base_style,
                button::Status::Pressed => button::Style {
                    background: backgroud_color,
                    text_color: if style.svg_color.is_some() {
                        style.svg_color.unwrap()
                    } else {
                        palette.background.strongest.color
                    },
                    ..base_style
                },
                button::Status::Hovered => button::Style {
                    background: backgroud_color,
                    text_color: constants::ICON_HOVER,
                    ..base_style
                },
                button::Status::Disabled => button::Style {
                    background: Some(Background::Color(palette.background.base.color)),
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

#[derive(Clone, Copy)]
pub struct Line {
    pub size: f32,
    pub thick: f32,
    pub color: Option<Color>,
    pub angle: Degrees,
}
impl<'a, Message> canvas::Program<Message> for Line {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        let center = frame.center();
        frame.translate(Vector::new(center.x, center.y));

        let path = canvas::Path::line(
            Point::new(-bounds.size().width / 2.0, 0.0),
            Point::new(bounds.size().width / 2.0, 0.0),
        );

        let palette = theme.extended_palette();
        let mut color = palette.background.strong.color;
        if let Some(c) = self.color {
            color = c;
        }
        frame.rotate(Degrees::from(self.angle));
        frame.stroke(
            &path,
            canvas::Stroke {
                width: self.thick,
                style: canvas::stroke::Style::Solid(color),
                line_cap: canvas::LineCap::Round,
                ..canvas::Stroke::default()
            },
        );

        vec![frame.into_geometry()]
    }
}

pub fn line<'a, Message: Clone + 'a>(line: Line) -> Element<'a, Message> {
    let mut height = Radians::from(line.angle).0 * line.size;
    if height == 0.0 {
        height = line.thick;
    }

    canvas(line).width(line.size).height(height).into()
}

fn get_handle<'a>(svg_path: &'a str) -> Handle {
    return Handle::from_path(format!(
        "{}/resource/{}",
        env!("CARGO_MANIFEST_DIR"),
        svg_path
    ));
}
