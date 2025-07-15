use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Item {
    pub barcode: String,
    pub name: String,
    pub cost: Decimal,
    pub price: Decimal,
    pub quantity: i32,
    pub image: Option<Vec<u8>>,
    pub expire_date: Vec<NaiveDate>,
    pub bulk_item: Vec<BulkItem>,
}

#[derive(Default, Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct BulkItem {
    pub barcode: Option<String>,
    pub name: String,
    pub price: Decimal,
    pub quantity: i32,
    pub image: Option<Vec<u8>>,
}

pub struct Header {
    barcode: String,
    name: String,
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
