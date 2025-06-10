use app::TableForge;
use iced::{Size, application, window::Settings};

mod app;
mod db;
mod model;
mod util;

fn main() -> iced::Result {
    application("TableForge", TableForge::update, TableForge::view)
        .window(Settings {
            size: Size::new(1280.0, 728.0),
            ..Default::default()
        })
        .run()
}
