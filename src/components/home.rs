use super::Aside;
use super::Main;

use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            class: "flex flex-row",
            div {
                class: "w-[80%]",
                Main {}
            }

            div {
                class: "w-[20%]",
                Aside {}
            }
        }
    }
}
