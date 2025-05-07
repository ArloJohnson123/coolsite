use leptos::prelude::*;
use leptos::mount::mount_to_body;

#[component]
fn App() -> impl IntoView {

    view! {
        <h1>"RustySoft Bots"</h1>
    }
}

fn main() {
    mount_to_body(App);
    console_error_panic_hook::set_once();
}