use leptos::prelude::*;
use leptos::mount::mount_to_body;
use leptos_router::components::{Router, Route, Routes,};
use leptos_router::path;
use leptos_router::components::A;
use stylance::import_crate_style;

import_crate_style!(style, "src/main.module.scss");


#[component]
fn Home() -> impl IntoView {

    view! {
    <div class="header-area">
        <h1>"placeholder Bots and Sites"</h1>
        <nav class="navbar">

            <ul>
                <li><A href="/">Home</A></li>
                <li><A href="/products">Products</A></li>

            </ul>
        </nav>
    </div>
        
    }
}

#[component]
fn products() -> impl IntoView {

    view! {
        <div class="header-area">
        <h1>"placeholder Bots and Sites"</h1>
        <nav class="navbar">

            <ul>
                <li><A href="/">Home</A></li>
                <li><A href="/products">Products</A></li>

            </ul>
        </nav>
    </div>
    }
}


#[component]
pub fn App() -> impl IntoView {
    view! {
      <Router>
      <nav>
        </nav>
        <main>
            <Routes fallback=|| "Not found.">
                <Route path=path!("/") view=Home/>
                <Route path=path!("/products") view=Products/>
            </Routes>
        </main>      
      </Router>
    }
}

fn main() {
    mount_to_body(App);
    console_error_panic_hook::set_once();
}

// https://coolors.co/palette/606c38-283618-fefae0-dda15e-bc6c25