use iced::{
    self, Element, Font,
    widget::{button, column, text},
};

use sunminimart::inventory::Inventory;
use sunminimart::main::{MainMessage, main_update, main_view};
use sunminimart::{Message, Page, State};

fn main() -> iced::Result {
    iced::application("Sunminimart", update, view)
        .font(include_bytes!("../asset/Sarabun-Regular.ttf"))
        .default_font(Font::with_name("Sarabun"))
        .run()
}

fn update(state: &mut State, message: Message) {
    match &state.page {
        Page::MainPage => main_update(state, message),
        Page::InventoryPage(inventory) => {}
    }
}

fn view(state: &State) -> Element<Message> {
    match &state.page {
        Page::MainPage => main_view(),
        Page::InventoryPage(inventory) => column![text("text")].into(),
    }
}
