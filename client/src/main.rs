use client::State;
use iced::Font;

fn main() -> iced::Result {
    iced::application("Sunminimart", State::update, State::view)
        .font(include_bytes!("../asset/Sarabun-Regular.ttf"))
        .default_font(Font::with_name("Sarabun"))
        .run()
}
