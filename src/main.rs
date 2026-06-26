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
        document::Link {
            rel: "icon",
            href: asset!("/assets/terminal-svg1.svg")
        }
        document::Title { "Spark Pannawich" }

        document::Stylesheet { href: asset!("/assets/main.css") }
        document::Stylesheet { href: asset!("/assets/tailwind.css") }
        div {
            class: "flex flex-row p-18 min-h-screen",
            Router::<Route> {}
        }
    }
}
