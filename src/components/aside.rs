use crate::route::Route;
use dioxus::prelude::*;

#[component]
pub fn Aside() -> Element {
    let nav_links = [
        (Route::Home {}, "Home"),
        (Route::Resume {}, "Resume"),
        (Route::Projects {}, "Projects"),
        (Route::ReadingList {}, "Reading List"),
    ];
    rsx! {
        aside {
            class: "ml-4 flex flex-col sticky top-[72px] self-start",
            h1 {
                class: "main-heading",
                "ARCHIVE"
            }
            p {
                class: "mb-8 text-sm font-serif font-bold uppercase tracking-wider text-slate-600",
                "Personal"
                br {}
                "Repository"
            }
            div {
                class: "flex flex-col ml-4 gap-5 uppercase cursor-pointer",
                for (route, label) in nav_links {
                    Link {
                        key: "{label}",
                        to: route,
                        class: "hover:font-bold",
                        "{label}"
                    }
                }
            }
        }
    }
}
