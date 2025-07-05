use iced::{
    border,
    widget::{button, center, container, horizontal_space, row},
    Alignment::Center,
    Background, Color, Element, Padding, Task, Theme,
};
use title::{Title, TitleMessage};

use crate::{
    constants,
    utils::{line, svg_button, Line, SvgButton, SvgButtonStyle},
};

pub mod title;

pub struct Header<'a> {
    title: Title,
    undo: SvgButton<'a, HeaderMessage>,
    redo: SvgButton<'a, HeaderMessage>,
    cursor: SvgButton<'a, HeaderMessage>,
    select: SvgButton<'a, HeaderMessage>,
    reset_zoom: SvgButton<'a, HeaderMessage>,
    fit_zoom: SvgButton<'a, HeaderMessage>,
    table: SvgButton<'a, HeaderMessage>,
    refer: SvgButton<'a, HeaderMessage>,
    note: SvgButton<'a, HeaderMessage>,
    view: SvgButton<'a, HeaderMessage>,
    layout: SvgButton<'a, HeaderMessage>,
    export: SvgButton<'a, HeaderMessage>,
    share: SvgButton<'a, HeaderMessage>,
    revert: SvgButton<'a, HeaderMessage>,
    property: SvgButton<'a, HeaderMessage>,
}

#[derive(Clone, Debug)]
pub enum HeaderMessage {
    Title(TitleMessage),
    Undo,
    Redo,
    Cursor,
    Select,
    ResetZoom,
    FitZoom,
    Table,
    Refer,
    Note,
    View,
    Layout,
    Export,
    Share,
    Revert,
    SignUp,
    Property,
}

impl<'a> Header<'a> {
    pub fn new() -> Self {
        let style = SvgButtonStyle::new(
            constants::ICON_SIZE,
            constants::ICON_SIZE,
            constants::ICON_PADDING,
        )
        .disable_background()
        .svg_color(constants::ICON_HOVER);

        Self {
            title: Title::default(),
            undo: SvgButton {
                svg_path: "header/undo.svg",
                style: style.clone().on_press(HeaderMessage::Undo),
            },
            redo: SvgButton {
                svg_path: "header/redo.svg",
                style: style.clone().on_press(HeaderMessage::Redo),
            },
            cursor: SvgButton {
                svg_path: "header/cursor.svg",
                style: style
                    .clone()
                    .is_active(true)
                    .svg_color(constants::ICON_ACTIVE)
                    .on_press(HeaderMessage::Cursor),
            },
            select: SvgButton {
                svg_path: "header/select.svg",
                style: style.clone().on_press(HeaderMessage::Select),
            },
            reset_zoom: SvgButton {
                svg_path: "header/reset-zoom.svg",
                style: style.clone().on_press(HeaderMessage::ResetZoom),
            },
            fit_zoom: SvgButton {
                svg_path: "header/fit-zoom.svg",
                style: style.clone().on_press(HeaderMessage::FitZoom),
            },
            table: SvgButton {
                svg_path: "header/table.svg",
                style: style.clone().on_press(HeaderMessage::Table),
            },
            refer: SvgButton {
                svg_path: "header/refer-non-id.svg",
                style: style.clone().on_press(HeaderMessage::Refer),
            },
            note: SvgButton {
                svg_path: "header/note.svg",
                style: style.clone().on_press(HeaderMessage::Note),
            },
            view: SvgButton {
                svg_path: "header/vm-physical.svg",
                style: style.clone().on_press(HeaderMessage::View),
            },
            layout: SvgButton {
                svg_path: "header/layout-star.svg",
                style: style.clone().on_press(HeaderMessage::Layout),
            },
            export: SvgButton {
                svg_path: "header/export.svg",
                style: style
                    .clone()
                    .svg_color(Color::from_rgb8(0x50, 0x90, 0xff))
                    .on_press(HeaderMessage::Export),
            },
            share: SvgButton {
                svg_path: "header/share.svg",
                style: style.clone().on_press(HeaderMessage::Share),
            },
            revert: SvgButton {
                svg_path: "header/revert.svg",
                style: style.clone().on_press(HeaderMessage::Revert),
            },
            property: SvgButton {
                svg_path: "header/property.svg",
                style: style
                    .clone()
                    .is_active(true)
                    .svg_color(Color::from_rgb8(0x50, 0x90, 0xff))
                    .on_press(HeaderMessage::Property),
            },
        }
    }
    pub fn update(&mut self, message: HeaderMessage) -> Task<HeaderMessage> {
        match message {
            HeaderMessage::Title(message) => self.title.update(message).map(HeaderMessage::Title),
            HeaderMessage::Undo => Task::none(),
            HeaderMessage::Redo => Task::none(),
            HeaderMessage::Cursor => Task::none(),
            HeaderMessage::Select => Task::none(),
            HeaderMessage::ResetZoom => Task::none(),
            HeaderMessage::FitZoom => Task::none(),
            HeaderMessage::Table => Task::none(),
            HeaderMessage::Refer => Task::none(),
            HeaderMessage::Note => Task::none(),
            HeaderMessage::View => Task::none(),
            HeaderMessage::Layout => Task::none(),
            HeaderMessage::Export => Task::none(),
            HeaderMessage::Share => Task::none(),
            HeaderMessage::Revert => Task::none(),
            HeaderMessage::SignUp => Task::none(),
            HeaderMessage::Property => Task::none(),
        }
    }

    pub fn view(&self) -> Element<HeaderMessage> {
        let vertical_line = Line {
            size: 24.0,
            thick: 0.7,
            angle: 90.0.into(),
            color: None,
        };

        container(
            row![
                self.title.view().map(HeaderMessage::Title),
                container(
                    row![
                        svg_button(self.undo.svg_path, self.undo.style.clone()),
                        svg_button(self.redo.svg_path, self.redo.style.clone()),
                        svg_button(self.cursor.svg_path, self.cursor.style.clone()),
                        svg_button(self.select.svg_path, self.select.style.clone()),
                        line(vertical_line),
                        svg_button(self.reset_zoom.svg_path, self.reset_zoom.style.clone()),
                        svg_button(self.fit_zoom.svg_path, self.fit_zoom.style.clone()),
                        line(vertical_line),
                        svg_button(self.table.svg_path, self.table.style.clone()),
                        svg_button(self.refer.svg_path, self.refer.style.clone()),
                        svg_button(self.note.svg_path, self.note.style.clone()),
                        line(vertical_line),
                        svg_button(self.view.svg_path, self.view.style.clone()),
                        line(vertical_line),
                        svg_button(self.layout.svg_path, self.layout.style.clone()),
                    ]
                    .spacing(10)
                ),
                horizontal_space(),
                svg_button(self.export.svg_path, self.export.style.clone()),
                svg_button(self.share.svg_path, self.share.style.clone()),
                svg_button(self.revert.svg_path, self.revert.style.clone()),
                button(center("Sign up"))
                    .width(100)
                    .height(32)
                    .style(|_theme: &Theme, status: button::Status| {
                        let base = button::Style {
                            background: Some(Background::Color(constants::BTN_PRIMARY)),
                            text_color: Color::WHITE,
                            border: border::rounded(2),
                            ..button::Style::default()
                        };

                        match status {
                            button::Status::Active | button::Status::Pressed => base,
                            button::Status::Hovered => button::Style {
                                background: Some(Background::Color(constants::BTN_PRIMARY_HOVER)),
                                text_color: base.text_color.scale_alpha(0.8),
                                ..base
                            },
                            button::Status::Disabled => button::Style {
                                background: base.background.map(|bg| bg.scale_alpha(0.5)),
                                text_color: base.text_color.scale_alpha(0.5),
                                ..base
                            },
                        }
                    })
                    .on_press(HeaderMessage::SignUp),
                svg_button(self.property.svg_path, self.property.style.clone())
            ]
            .spacing(20)
            .padding(Padding::default().right(32))
            .height(62)
            .align_y(Center),
        )
        .style(|theme: &Theme| {
            let pallete = theme.extended_palette();
            container::Style::default()
                .border(border::color(pallete.background.weak.color).width(0.5))
        })
        .into()
    }
}

impl<'a> Default for Header<'a> {
    fn default() -> Self {
        Self::new()
    }
}
