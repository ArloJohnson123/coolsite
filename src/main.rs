use leptos::prelude::*;
use leptos::mount::mount_to_body;
use stylance::import_crate_style;

import_crate_style!(style, "src/main.module.scss");

#[component]
fn App() -> impl IntoView {

    view! {
        <div id="ab">
            <h1 id="ab">"RustySoft Bots"</h1>
        </div>
    }
}

fn main() {
    mount_to_body(App);
    console_error_panic_hook::set_once();
}