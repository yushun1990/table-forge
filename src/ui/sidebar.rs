use iced::{
    border,
    widget::{column, container, vertical_space},
    Alignment::Center,
    Element, Task, Theme,
};

use crate::{
    constants,
    utils::{line, svg_button, Line, SvgButton, SvgButtonStyle},
};

pub struct Sidebar<'a> {
    logo: SvgButton<'a, SidebarMessage>,
    diagram: SvgButton<'a, SidebarMessage>,
    table: SvgButton<'a, SidebarMessage>,
    database: SvgButton<'a, SidebarMessage>,
    script: SvgButton<'a, SidebarMessage>,
    export: SvgButton<'a, SidebarMessage>,
    help: SvgButton<'a, SidebarMessage>,
}

#[derive(Clone, Debug)]
pub enum SidebarMessage {
    Logo,
    Diagram,
    Table,
    Database,
    Script,
    Export,
    Help,
}

impl<'a> Sidebar<'a> {
    pub fn new() -> Self {
        let style = SvgButtonStyle::new(
            constants::ICON_SIZE,
            constants::ICON_SIZE,
            constants::ICON_PADDING,
        )
        .svg_color(constants::ICON_ACTIVE);
        Self {
            logo: SvgButton {
                svg_path: "logo.svg",
                style: style
                    .clone()
                    .custom_svg_style(false)
                    .width(constants::BAR_SIZE)
                    .height(constants::BAR_SIZE)
                    .padding(0.0)
                    .on_press(SidebarMessage::Logo),
            },
            diagram: SvgButton {
                svg_path: "sidebar/diagram.svg",
                style: style
                    .clone()
                    .is_active(true)
                    .on_press(SidebarMessage::Diagram)
                    .label("diagram"),
            },
            table: SvgButton {
                svg_path: "sidebar/table.svg",
                style: style.clone().on_press(SidebarMessage::Table).label("table"),
            },
            database: SvgButton {
                svg_path: "sidebar/database.svg",
                style: style
                    .clone()
                    .on_press(SidebarMessage::Database)
                    .label("database"),
            },
            script: SvgButton {
                svg_path: "sidebar/script.svg",
                style: style
                    .clone()
                    .on_press(SidebarMessage::Script)
                    .label("script"),
            },
            export: SvgButton {
                svg_path: "sidebar/export.svg",
                style: style
                    .clone()
                    .on_press(SidebarMessage::Export)
                    .label("export"),
            },
            help: SvgButton {
                svg_path: "sidebar/help.svg",
                style: style.clone().on_press(SidebarMessage::Help).label("help"),
            },
        }
    }

    pub fn update(&mut self, message: SidebarMessage) -> Task<SidebarMessage> {
        match message {
            SidebarMessage::Logo => Task::none(),
            SidebarMessage::Diagram => {
                self.inactive();
                self.diagram.style.active();
                Task::none()
            }
            SidebarMessage::Table => {
                self.inactive();
                self.table.style.active();
                Task::none()
            }
            SidebarMessage::Database => {
                self.inactive();
                self.database.style.active();
                Task::none()
            }
            SidebarMessage::Script => {
                self.inactive();
                self.script.style.active();

                Task::none()
            }
            SidebarMessage::Export => {
                self.inactive();
                self.export.style.active();

                Task::none()
            }
            SidebarMessage::Help => {
                self.inactive();
                self.help.style.active();

                Task::none()
            }
        }
    }
    pub fn view(&self) -> Element<'a, SidebarMessage> {
        container(
            column![
                svg_button(self.logo.svg_path, self.logo.style.clone()),
                svg_button(self.diagram.svg_path, self.diagram.style.clone()),
                svg_button(self.table.svg_path, self.table.style.clone()),
                svg_button(self.database.svg_path, self.database.style.clone()),
                line(Line {
                    size: 32.0,
                    thick: 1.0,
                    angle: 0.0.into(),
                    color: None
                }),
                svg_button(self.script.svg_path, self.script.style.clone()),
                svg_button(self.export.svg_path, self.export.style.clone()),
                vertical_space(),
                svg_button(self.help.svg_path, self.help.style.clone()),
            ]
            .spacing(32)
            .width(62)
            .align_x(Center),
        )
        .style(|theme: &Theme| {
            let pallete = theme.extended_palette();
            container::Style::default()
                .border(border::color(pallete.background.weak.color).width(0.5))
        })
        .into()
    }

    fn inactive(&mut self) {
        self.diagram.style.inactive();
        self.table.style.inactive();
        self.database.style.inactive();
        self.script.style.inactive();
        self.export.style.inactive();
        self.help.style.inactive();
    }
}
