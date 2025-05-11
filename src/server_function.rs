use leptos::prelude::*;
use leptos::server_fn::serde;
use sqlx::types::{BigDecimal, Decimal};

#[cfg(feature = "ssr")]
use crate::backend::{all_product};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Product {
    pub barcode: String,
    pub name: String,
    pub amount: u16,
    pub cost: Decimal,
    pub price: Decimal,
    pub image: Option<Vec<u8>>
}

#[server]
pub async fn get_all_product() -> Result<Vec<Product>, ServerFnError> {
    Ok(all_product())
}

#[server]
pub async fn search_product(barcode: String) -> Result<String, ServerFnError> {
    todo!();
}
