use chrono::{Datelike, Local};
use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    let current_year = Local::now().year();
    rsx! {
        footer {
            class: "flex flex-row justify-between border-t border-gray-700 pt-4",
            div {
                class: "font-serif uppercase tracking-wider",
                "© {current_year} All Rights Reserved"
            },
            div {
                "abc2"
            }
        }
    }
}
