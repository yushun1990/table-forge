use iced::{
    Alignment::Center,
    Element,
    Length::{self, Fill},
    Padding, Point, Rectangle, Renderer, Theme, border, mouse,
    widget::{button, canvas, center, column, container, svg, tooltip, vertical_space},
};

pub struct Sidebar {}

#[derive(Clone, Debug)]
pub enum SidebarMessage {}

impl Sidebar {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, message: SidebarMessage) {}
    pub fn view<'a>(&self) -> Element<'a, SidebarMessage> {
        container(
            column![
                action("logo.svg", 62, 0, "", None),
                action("diagram.svg", 32, 4, "", None),
                action("explorer.svg", 32, 4, "", None),
                line(32),
                action("gen.svg", 32, 4, "", None),
                action("export.svg", 32, 4, "", None),
                vertical_space(),
                action("help.svg", 32, 4, "", None)
            ]
            .spacing(32)
            .width(62)
            .align_x(Center),
        )
        .style(|theme| {
            let pallete = theme.extended_palette();
            container::Style::default()
                .border(border::color(pallete.background.strong.color).width(1))
        })
        .into()
    }
}

fn action<'a, Message: Clone + 'a>(
    svg_name: &'a str,
    size: impl Into<Length> + Copy,
    padding: impl Into<Padding> + Copy,
    label: &'a str,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    let handle = svg::Handle::from_path(format!(
        "{}/resource/header/{}",
        env!("CARGO_MANIFEST_DIR"),
        svg_name
    ));

    let content = svg(handle).width(Fill).height(Fill);
    let action = button(center(content))
        .width(size)
        .height(size)
        .padding(padding);
    if let Some(on_press) = on_press {
        tooltip(action.on_press(on_press), label, tooltip::Position::Right)
            .style(container::rounded_box)
            .into()
    } else {
        action.style(button::secondary).into()
    }
}

fn line<'a>(size: impl Into<Length> + Copy) -> Element<'a, SidebarMessage> {
    struct Line;
    impl canvas::Program<SidebarMessage> for Line {
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
            let palette = theme.extended_palette();
            frame.fill_rectangle(
                Point::ORIGIN,
                bounds.size(),
                palette.background.strong.color,
            );

            vec![frame.into_geometry()]
        }
    }

    canvas(Line).width(size).height(1).into()
}
