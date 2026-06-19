mod components;
mod route;
mod views;

use dioxus::prelude::*;
use route::Route;

fn main() {
    dioxus::launch(app);
}

#[component]
fn app() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/tailwind.css") }
        document::Stylesheet { href: asset!("/assets/main.css") }
        Router::<Route> {}
    }
}
