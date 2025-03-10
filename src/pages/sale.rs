use iced::widget::{button, column, container, keyed_column, row, scrollable, text};
use iced::Alignment::Center;
use iced::Element;
use iced::Length::{Fill, FillPortion};
use iced::Theme;

use crate::widget::{labeled_text_input, labeled_value, Position};
use crate::{Message, MessageSale, Pages, Sale, State};

pub fn sale_page<'a>(state: &State, sale: &Sale) -> Element<'a, Message> {
    // Right
    let total_price = labeled_value("รวม / Total Price", 40, Position::Top, &sale.total);
    let current_price = labeled_value("ราคา / Price", 40, Position::Top, &sale.item.price);
    let received = labeled_text_input(
        "รับเงิน / Received",
        40,
        &sale.received,
        |input: String| Message::Sale(MessageSale::Receive(input)),
        Message::Sale(MessageSale::Pay),
    );
    let change = labeled_value("เงินทอน / Change", 40, Position::Top, &sale.change);
    let pay_button = container(
        button(text("จ่ายเงิน / Pay").center().size(40).width(Fill))
            .on_press(Message::Sale(MessageSale::EnterPay)),
    )
    .height(Fill)
    .align_y(Center);

    // Bottom
    let amount = labeled_text_input(
        "จำนวน / Amount",
        25,
        sale.item.amount.clone(),
        |input: String| Message::Sale(MessageSale::AmountChanged(input)),
        Message::Sale(MessageSale::AmountSubmit),
    );
    let barcode = labeled_text_input(
        "บาร์โค๊ด / Barcode",
        25,
        sale.item.barcode.clone(),
        |input: String| Message::Sale(MessageSale::BarcodeChanged(input)),
        Message::Sale(MessageSale::BarcodeSubmit),
    );
    let name = labeled_value("ชื่อสินค้า / Name", 25, Position::Top, &sale.item.name);
    let price = labeled_value("ราคา / Price", 25, Position::Top, &sale.item.price);
    let sum = labeled_value("รวม / Sum", 25, Position::Top, &sale.item.sum);

    // Grid
    let title = row![
        text("order").width(Fill).center().size(25),
        text("barcode").width(FillPortion(2)).center().size(25),
        text("name").width(FillPortion(2)).center().size(25),
        text("price").width(Fill).center().size(25),
        text("amount").width(Fill).center().size(25),
        text("sum").width(Fill).center().size(25),
    ];
    let list = keyed_column(sale.items.iter().enumerate().map(|x| {
        (
            x.0,
            container(row![
                text!("{}", x.0 + 1).width(Fill).center().size(25),
                text!("{}", x.1.barcode)
                    .width(FillPortion(2))
                    .center()
                    .size(25),
                text!("{}", x.1.name)
                    .width(FillPortion(2))
                    .center()
                    .size(25),
                text!("{}", x.1.price).width(Fill).center().size(25),
                text!("{}", x.1.amount).width(Fill).center().size(25),
                text!("{}", x.1.sum).width(Fill).center().size(25),
            ])
            .style(|_| container::bordered_box(&Theme::Light))
            .into(),
        )
    }));

    // Sale view starts here
    container(column![
        row![
            // Grid
            column![title, scrollable(list)]
                .height(FillPortion(2))
                .width(FillPortion(4)),
            // Right panel
            column![
                total_price,
                current_price,
                if let Pages::Sale(sale) = &state.pages {
                    if !sale.paying {
                        column![pay_button].height(Fill)
                    } else {
                        column![received, change].height(Fill)
                    }
                } else {
                    column![text("2")]
                }
            ]
            .width(Fill)
        ]
        .spacing(10)
        .padding(10),
        // Bottom Text Input
        row![amount, barcode, name, price, sum,]
            .spacing(10)
            .padding(10)
    ])
    .center(Fill)
    .into()
}
