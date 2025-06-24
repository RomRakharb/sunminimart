use client::{State, subscription};
use iced::Font;

fn main() -> iced::Result {
    iced::application(State::default, State::update, State::view)
        .title("Sunminimart")
        .theme(|_| iced::Theme::Light)
        .centered()
        .font(include_bytes!("../asset/Sarabun-Regular.ttf"))
        .default_font(Font::with_name("Sarabun"))
        .subscription(subscription)
        .run()
}
