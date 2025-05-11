use leptos::form::ActionForm;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use crate::server_function::SearchProduct;

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
fn ProductPage() -> impl IntoView {
    let search_product = ServerAction::<SearchProduct>::new();
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

#[component]
fn InventoryPage() -> impl IntoView {
    view! {
        <div>
            <h1>"คลังสินค้า"</h1>
            <table>
                <thead>
                    <tr>
                        <th>"ลำดับ"</th>
                        <th>"รหัสสินค้า"</th>
                        <th>"ชื่อ"</th>
                        <th>"จำนวน"</th>
                        <th>"ทุน"</th>
                        <th>"ราคา"</th>
                    </tr>
                </thead>
                <tbody></tbody>
            </table>
        </div>
    }
}
