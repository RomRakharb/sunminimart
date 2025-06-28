use crate::Message;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{
    container, text, Container,
};
use iced::{Length, Pixels};

pub(crate) fn title<'a>(value: impl text::IntoFragment<'a>) -> Container<'a, Message> {
    container(text(value).size(Pixels(30.0))).width(Length::Fill).height(Length::Fill).align_x(Horizontal::Center).align_y(Vertical::Center)
}

mod search_table {
    use iced::alignment::Horizontal;
    use iced::widget::{
        button, column, container, keyed_column, row, scrollable, text, text_input,
    };
    use iced::{color, Color, Element, Length, Pixels};
    use shared::Item;

    #[derive(Default)]
    pub(crate) struct State {
        all_items: Vec<Item>,
        filtered_items: Vec<Item>,
        search: String,
        position: usize,
        focused: bool,
    }

    #[derive(Clone)]
    enum Message {
        OnSearchChange(String),
        Refresh,
    }

    impl State {
        pub(crate) fn create(&mut self) -> Element<Message> {
            column![
                row![
                    text("ค้นหา: "),
                    text_input("", &self.search)
                        .id(text_input::Id::new("search"))
                        .on_input(|input| { Message::OnSearchChange(input) }),
                    button("refresh").on_press(Message::Refresh)
                ]
                .spacing(Pixels(10.0)),
                scrollable(keyed_column(self.filtered_items.iter().enumerate().map(
                    |(i, item)| {
                        (
                            i,
                            container(row![
                                text(item.barcode.clone())
                                    .width(Length::Fill)
                                    .align_x(Horizontal::Center),
                                text(item.name.clone())
                                    .shaping(text::Shaping::Advanced)
                                    .width(Length::Fill)
                                    .align_x(Horizontal::Center)
                            ])
                            .style({
                                let current_position = self.position;
                                move |_| {
                                    if i == current_position {
                                        container::Style {
                                            background: Some(iced::Background::Color(color!(0x4169e1))),
                                            text_color: Some(Color::WHITE),
                                            ..Default::default()
                                        }
                                    } else {
                                        container::Style::default()
                                    }
                                }
                            })
                            .into(),
                        )
                    }
                )))
                .width(Length::Fill)
            ].into()
        }
    }
}
