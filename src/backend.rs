use dotenv::dotenv;
use sqlx::mysql::MySqlPool;
use std::env;
use tokio::sync::OnceCell;

static POOL: OnceCell<MySqlPool> = OnceCell::const_new();
use crate::server_function::Product;

pub async fn connect_database() -> &'static MySqlPool {
    POOL.get_or_init(async || {
        dotenv().ok();
        let url = env::var("DATABASE_URL").unwrap_or_default();
        MySqlPool::connect(&url).await.unwrap()
    })
    .await
}

pub async fn all_product() -> Vec<Product> {
    let pool = connect_database().await;

    sqlx::query_as!(Product,
        "
            SELECT Barcode AS barcode, Name AS name, Amount AS amount, Cost AS cost, Price AS price, Image AS image
            FROM products
        "
    ).fetch_all(pool).await.unwrap()
}
