use dioxus::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Clone, PartialEq)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub score: String,
    pub notes: String,
}

pub struct YearBooks {
    pub year: u32,
    pub books: Vec<Book>,
}

include!(concat!(env!("OUT_DIR"), "/books_generated.rs"));

#[component]
pub fn ReadingList() -> Element {
    let all_years = get_all_books();
    let non_empty_years: Vec<&YearBooks> =
        all_years.iter().filter(|y| !y.books.is_empty()).collect();

    rsx! {
        h1 {
            class: "main-heading",
            "READING_LOG"
        }
        div { class: "w-full max-w-4xl font-serif",
            div { class: "grid grid-cols-[2fr_1.5fr_0.5fr_3fr] gap-4 uppercase font-bold text-xs border-b border-black pb-2 mb-6 text-slate-700",
                div { "Title" }
                div { "Author" }
                div { "Score" }
                div { "Notes" }
            }

            for (i, year_group) in non_empty_years.iter().enumerate() {
                if i > 0 {
                    div { class: "border-t border-black/20 my-8" }
                }

                div { class: "font-bold text-sm text-slate-500 mb-4", "{year_group.year}" }

                div { class: "flex flex-col gap-6 pl-2",
                    for book in &year_group.books {
                        div {
                            key: "{book.title}",
                            class: "grid grid-cols-[2fr_1.5fr_0.5fr_3fr] gap-4 text-sm items-baseline",
                            div { class: "font-bold text-black", "{book.title}" }
                            div { class: "text-slate-800", "{book.author}" }
                            div { class: "text-slate-800", "{book.score}" }
                            div { class: "text-slate-700 leading-relaxed", "{book.notes}" }
                        }
                    }
                }
            }
        }
    }
}
