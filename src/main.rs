mod app;
mod config;
mod error;
mod util;

pub use self::config::Config;
pub use self::error::Error;

fn main() -> cosmic::iced::Result {
    let settings = cosmic::app::Settings::default().size_limits(
        cosmic::iced::Limits::NONE
            .min_width(360.0)
            .min_height(180.0),
    );
    cosmic::app::run::<app::App>(settings, ())
}
