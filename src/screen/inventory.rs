use chrono::NaiveDate;
use rust_decimal::Decimal;

#[derive(Default, Debug, PartialEq)]
pub struct State {
    pub items: Vec<Item>,
}

#[derive(Default, Debug, PartialEq)]
pub struct Item {
    pub barcode: String,
    pub name: String,
    pub cost: Decimal,
    pub price: Decimal,
    pub amount: i32,
    pub image: Vec<u8>,
    pub expire_date: Vec<NaiveDate>,
    pub bulk_item: Vec<BulkItem>,
}

#[derive(Default, Debug, PartialEq)]
pub struct BulkItem {
    pub barcode: Option<String>,
    pub name: Option<String>,
    pub price: Decimal,
    pub amount: i32,
    pub image: Option<Vec<u8>>,
}

fn update() {}

fn view() {}

#[cfg(test)]
mod test {}
