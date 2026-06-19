use crate::components::footer;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        h1 {
            class: "main-heading",
            "ABOUT_ME"
        }
        section {
            class: "text-xl",
            "I am a fullstack engineer, focusing on backend and infrastructure"
        }
        section {
            p {
                class: "font-serif font-bold uppercase tracking-wider text-slate-600",
                "Location: Bangkok, Thailand"
            }
        }
        section {
            "This website was made by Dioxus Rust."
        }
        div {
            class: "mt-auto",
            footer::Footer {}
        }
    }
}
