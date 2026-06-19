use dioxus::prelude::*;

#[component]
pub fn ReadingList() -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-5 min-h-full mr-5",
            div {
                class: "w-[80%]",
                "reading list"
            }
        }
    }
}
