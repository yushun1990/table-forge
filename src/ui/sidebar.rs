use iced::{
    Alignment::Center,
    Color, Element,
    Length::{self, Fill},
    Padding, Point, Rectangle, Renderer, Theme, border, mouse,
    widget::{Svg, button, canvas, center, column, container, svg, tooltip, vertical_space},
};

use crate::utils::{IconStyle, icon};

struct Icon<'a, Message: Clone + 'a> {
    id: u32,
    code: char,
    style: IconStyle<'a, Message>,
}

pub struct Sidebar<'a, Message: Clone + 'a> {
    logo: svg::Handle,
    diagram: Icon<'a, Message>,
    table: Icon<'a, Message>,
    database: Icon<'a, Message>,
    script: Icon<'a, Message>,
    export: Icon<'a, Message>,
    help: Icon<'a, Message>,
}

#[derive(Clone, Debug)]
pub enum SidebarMessage {
    Diagram,
    Table,
    Database,
    Script,
    Export,
    Help,
}

impl<'a> Sidebar<'a, SidebarMessage> {
    pub fn new() -> Self {
        let style = IconStyle::new(32.0, 32.0, 0.0);
        Self {
            logo: svg::Handle::from_path(format!(
                "{}/resource/logo.svg",
                env!("CARGO_MANIFEST_DIR")
            )),
            diagram: Icon {
                id: 1,
                code: '\u{0efce}',
                style: style
                    .clone()
                    .text_color(Color::from_rgb8(0x50, 0x90, 0xff))
                    .label("diagram")
                    .on_press(SidebarMessage::Diagram),
            },
            table: Icon {
                id: 2,
                code: '\u{f13c8}',
                style: style.clone().label("table").on_press(SidebarMessage::Table),
            },
            database: Icon {
                id: 3,
                code: '\u{0eace}',
                style: style
                    .clone()
                    .label("database")
                    .on_press(SidebarMessage::Database),
            },
            script: Icon {
                id: 4,
                code: '\u{f0bc1}',
                style: style
                    .clone()
                    .label("script")
                    .on_press(SidebarMessage::Script),
            },
            export: Icon {
                id: 5,
                code: '\u{f162c}',
                style: style
                    .clone()
                    .label("export")
                    .on_press(SidebarMessage::Export),
            },
            help: Icon {
                id: 6,
                code: '\u{f0625}',
                style: style.clone().label("help").on_press(SidebarMessage::Help),
            },
        }
    }

    pub fn update(&mut self, message: SidebarMessage) {}
    pub fn view(&self) -> Element<'a, SidebarMessage> {
        let svg: Svg<'a> = svg(self.logo.clone()).width(62).height(62);
        container(
            column![
                button(svg).padding(0),
                icon(self.diagram.code, self.diagram.style.clone()),
                icon(self.table.code, self.table.style.clone()),
                icon(self.database.code, self.database.style.clone()),
                line(32),
                icon(self.script.code, self.script.style.clone()),
                icon(self.export.code, self.export.style.clone()),
                vertical_space(),
                icon(self.help.code, self.help.style.clone()),
            ]
            .spacing(32)
            .width(64)
            .align_x(Center),
        )
        .style(|theme: &Theme| {
            let pallete = theme.extended_palette();
            container::Style::default()
                .border(border::color(pallete.background.strong.color).width(1))
        })
        .into()
    }
}

fn line<'a, Message: Clone + 'a>(size: impl Into<Length> + Copy) -> Element<'a, Message> {
    struct Line;
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
