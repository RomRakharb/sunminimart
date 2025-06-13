use dotenv::dotenv;
use rust_decimal::Decimal;
use sqlx::MySqlPool;
use tokio::sync::OnceCell;

static POOL: OnceCell<MySqlPool> = OnceCell::const_new();

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

pub(crate) async fn sync_database() -> sqlx::Result<()> {
    let pool = pool().await;
    let old_pool = todo!();

    let items: Vec<(String, String, Decimal, Decimal, i16, Option<Vec<u8>>)> =
        sqlx::query_as("").fetch_all(pool).await?;
    Ok(())
}
