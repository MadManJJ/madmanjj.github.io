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
                class: "flex flex-row gap-5",
                a {
                    class: "text-blue-800 underline font-serif tracking-wide uppercase",
                    href: "https://github.com/MadManJJ",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    "Github"
                }
                a {
                    class: "text-blue-800 underline font-serif tracking-wide uppercase",
                    href: "https://www.linkedin.com/in/pannawich-thamart/",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    "Linkedin"
                }
            }
        }
    }
}
