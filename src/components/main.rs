use super::Footer;

use dioxus::prelude::*;

#[component]
pub fn Main() -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-5 p-18",
            header {
                h1 {
                    class: "font-bold text-4xl",
                    "Welcome"
                }
            }
            section {
                class: "text-xl",
                "I am a fullstack engineer, focusing on backend and infrastructure"
            }
            section {
                "This website was made by Dioxus Rust."
            }
            Footer {}
        }
    }
}
