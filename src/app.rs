use leptos::prelude::*;
use leptos::form::ActionForm;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
// use crate::datebase::connect_database;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/sunminimart.css" />

        // sets the document title
        <Title text="Sunminimart" />

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage />
                    <Route path=StaticSegment("/inventory") view=InventoryPage />
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}

#[component]
fn InventoryPage() -> impl IntoView {
    let search_product =ServerAction::<SearchProduct>::new();
    let value = search_product.value();
    let has_error = move || value.with(|val| matches!(val, Some(Err(_))));

    view! {
        <div id="inventory">
            <h1>"คลังสินค้า"</h1>
            <ActionForm action=search_product>
                <label>
                    "รหัสสินค้า: " <input type="text" name="barcode" />
                </label>
                <input type="submit" value="ค้นหา" />
            </ActionForm>
            <p>"value is " {value}</p>
        </div>
    }
}

#[server]
async fn search_product(barcode: String) -> Result<String, ServerFnError> {
    let pool = database::connect_database();
    Ok(barcode)
}

#[cfg(feature = "ssr")]
mod database {
    use sqlx::mysql::MySqlPool;
    use tokio::sync::OnceCell;
    use std::env;
    use dotenv::dotenv;

    static POOL: OnceCell<MySqlPool> = OnceCell::const_new();

    pub async fn connect_database() -> &'static MySqlPool {
        POOL.get_or_init( async || {
            dotenv().ok();
            let url = env::var("DATABASE_URL").unwrap_or_default();
            MySqlPool::connect(&url).await.unwrap()
        }).await
    }
}
