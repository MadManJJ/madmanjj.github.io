use dioxus::prelude::*;

#[component]
pub fn PageNotFound(segments: Vec<String>) -> Element {
    rsx! {
        div { "404 - Page Not Found: {segments.join(\"/\")}" }
    }
}
