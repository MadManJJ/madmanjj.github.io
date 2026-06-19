use dioxus::prelude::*;

#[component]
pub fn ReadingList() -> Element {
    let books = [
        (
            "The Diamond Age",
            "Neal Stephenson",
            "9/10",
            "Mind-bending infrastructure ideas.",
        ),
        (
            "Neuromancer",
            "William Gibson",
            "10/10",
            "Re-read. Foundational.",
        ),
        (
            "Snow Crash",
            "Neal Stephenson",
            "8/10",
            "A bit dated but the linguistics hold up.",
        ),
    ];

    rsx! {
        h1 {
            class: "font-bold text-lg",
            "READING_LOG"
        }
        div { class: "w-full max-w-4xl font-serif",
            div { class: "grid grid-cols-[2fr_1.5fr_0.5fr_3fr] gap-4 uppercase font-bold text-xs border-b border-black pb-2 mb-6 text-slate-700",
                div { "Title" }
                div { "Author" }
                div { "Score" }
                div { "Notes" }
            }

            div { class: "font-bold text-sm text-slate-500 mb-4", "2025" }

            div { class: "flex flex-col gap-6 pl-2",
                for (title, author, score, notes) in books {
                    div {
                        key: "{title}",
                        class: "grid grid-cols-[2fr_1.5fr_0.5fr_3fr] gap-4 text-sm items-baseline",

                        div { class: "font-bold text-black", "{title}" }
                        div { class: "text-slate-800", "{author}" }
                        div { class: "text-slate-800", "{score}" }
                        div { class: "text-slate-700 leading-relaxed", "{notes}" }
                    }
                }
            }
        }
    }
}
