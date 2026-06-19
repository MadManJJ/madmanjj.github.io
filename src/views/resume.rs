use dioxus::prelude::*;

#[component]
pub fn Resume() -> Element {
    rsx! {
        div {
            class: "flex flex-row p-18 min-h-screen",
            div {
                class: "w-[80%]",
                "hello"
            }
        }
    }
}
