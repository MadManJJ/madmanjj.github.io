use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer {
            class: "flex flex-row justify-between",
            div {
                "abc"
            },
            div {
                "abc2"
            }
        }
    }
}
