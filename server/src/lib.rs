mod database;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};
use futures::future::try_join_all;
use serde_json::json;
use shared::{BulkItem, ExpireDate, Item};

#[derive(Debug)]
pub enum AppError {
    DatabaseError(sqlx::Error),
}

impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        AppError::DatabaseError(error)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::DatabaseError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": e.to_string() })),
            ),
        }
        .into_response()
    }
}

pub async fn sync_database() {
    let _ = database::sync_database().await;
}

pub async fn get_items() -> Result<Json<Vec<Item>>, AppError> {
    let item_details = database::select_items().await?;

    let items = item_details.into_iter().map(|item| async move {
        let bulk_items: Vec<BulkItem> = database::select_bulk_items(&item.barcode).await?;
        let expire_dates: Vec<ExpireDate> = database::select_expire_dates(&item.barcode).await?;

        Ok::<Item, AppError>(Item {
            barcode: item.barcode,
            name: item.name,
            cost: item.cost,
            price: item.price,
            quantity: item.quantity,
            image: item.image,
            expire_date: expire_dates,
            bulk_item: bulk_items,
        })
    });

    let items: Vec<Item> = try_join_all(items).await?;

    Ok(Json(items))
}
