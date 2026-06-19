use crate::components::{aside, main};
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class: "flex flex-row p-18 min-h-screen",
            div {
                class: "w-[80%]",
                main::Main {}
            }

            div {
                class: "w-[20%]",
                aside::Aside {}
            }
        }
    }
}
