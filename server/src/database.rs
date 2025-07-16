use chrono::NaiveDate;
use dotenv::dotenv;
use rust_decimal::Decimal;
use sqlx::{FromRow, MySqlPool};
use tokio::sync::OnceCell;

use crate::AppError;

static POOL: OnceCell<MySqlPool> = OnceCell::const_new();

#[derive(Debug)]
pub(crate) struct Item {
    pub(crate) barcode: String,
    pub(crate) name: String,
    pub(crate) cost: Decimal,
    pub(crate) price: Decimal,
    pub(crate) quantity: i32,
    pub(crate) image: Option<Vec<u8>>,
}

async fn pool() -> &'static MySqlPool {
    POOL.get_or_init(|| async {
        dotenv().ok();
        let database_url =
            std::env::var("DATABASE_URL").expect("DATABASE_URL not found, pleas check .env file.");
        MySqlPool::connect(&database_url)
            .await
            .expect("Database connection failed.")
    })
    .await
}

pub(crate) async fn sync_database() -> Result<(), AppError> {
    let pool = pool().await;

    let old_database_url = std::env::var("OLD_DATABASE_URL")
        .expect("OLD_DATABASE_URL not found, pleas check .env file.");
    let old_pool = MySqlPool::connect(&old_database_url).await?;

    println!("start syncing....");
    let items: Vec<(String, String, f32, f32, i32, String)> =
        sqlx::query_as("SELECT b01100, b01110, b01140, b01150, b01160, b01211 FROM ab01f")
            .fetch_all(&old_pool)
            .await?;
    for item in items {
        let mut transaction = pool.begin().await?;
        if let Ok(barcode) = item.0.parse::<i32>() {
            if barcode <= 1000 {
                continue;
            }
        }
        sqlx::query!(
            "
                INSERT INTO items (barcode, name, cost, price, quantity)
                VALUES (?, ?, ?, ?, ?)
                ON DUPLICATE KEY UPDATE
                name = VALUES(name),
                cost = VALUES(cost),
                price = VALUES(price),
                quantity = VALUES(quantity),
                image = VALUES(image);
            ",
            item.0,
            item.1,
            item.2,
            item.3,
            item.4
        )
        .execute(&mut *transaction)
        .await?;

        let ymd: Vec<&str> = item.5.split('-').collect();
        if ymd.len() == 3 && ymd[0].len() == 4 && ymd[1].len() == 2 && ymd[2].len() == 2 {
            if let Some(date) =
                NaiveDate::from_ymd_opt(ymd[2].parse()?, ymd[1].parse()?, ymd[0].parse()?)
            {
                sqlx::query!(
                    "
                        INSERT INTO expire_dates (ref_barcode, expire_date)
                        VALUES (?, ?)
                    ",
                    item.0,
                    date
                )
                .execute(&mut *transaction)
                .await?;
            }
        }
        transaction.commit().await?
    }
    println!("end syncing");
    Ok(())
}

pub(crate) async fn select_headers() -> sqlx::Result<Vec<shared::Header>> {
    sqlx::query_as!(shared::Header, "SELECT barcode, name FROM items")
        .fetch_all(pool().await)
        .await
}

pub(crate) async fn select_header(keyword: &str) -> sqlx::Result<shared::Header> {
    sqlx::query_as!(shared::Header, "SELECT barcode, name FROM items")
        .fetch_one(pool().await)
        .await
}

pub(crate) async fn select_items() -> sqlx::Result<Vec<Item>> {
    sqlx::query_as!(
        Item,
        "SELECT barcode, name, cost, price, quantity, image FROM items"
    )
    .fetch_all(pool().await)
    .await
}

pub(crate) async fn select_expire_dates(barcode: &String) -> sqlx::Result<Vec<NaiveDate>> {
    #[derive(FromRow)]
    struct ExpireDate {
        expire_date: NaiveDate,
    }

    let expire_dates: Vec<ExpireDate> = sqlx::query_as!(
        ExpireDate,
        "
        SELECT expire_date FROM expire_dates
        WHERE ref_barcode = ?;
        ",
        barcode
    )
    .fetch_all(pool().await)
    .await?;

    let expire_dates = expire_dates
        .into_iter()
        .map(|expire_date| expire_date.expire_date)
        .collect();

    Ok(expire_dates)
}

pub(crate) async fn select_bulk_items(ref_barcode: &String) -> sqlx::Result<Vec<shared::BulkItem>> {
    let bulk_items: Vec<shared::BulkItem> = sqlx::query_as!(
        shared::BulkItem,
        "
        SELECT barcode, name, price, quantity, image FROM bulk_items
        WHERE ref_barcode = ?;
        ",
        ref_barcode
    )
    .fetch_all(pool().await)
    .await?;
    Ok(bulk_items)
}
