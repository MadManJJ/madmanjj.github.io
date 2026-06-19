use dioxus::prelude::*;

#[component]
pub fn Resume() -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-5 min-h-full mr-5",
            div {
                class: "w-[80%]",
                "resume"
            }
        }
    }
}
