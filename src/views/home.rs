use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let user = "pannawich.tha";
    let domain = "gmail.com";
    let full_email = format!("{}@{}", user, domain);
    rsx! {
        h1 {
            class: "main-heading",
            "ABOUT_ME"
        }
        section {
            class: "text-xl",
            p { "I am a fullstack engineer, focusing on backend and infrastructure." }

            p {
                "My work email is "
                a {
                    href: "mailto:{full_email}",
                    class: "font-serif font-bold underline",
                    "{full_email}"
                }
            }
        }
        section {
            p {
                class: "font-serif font-bold uppercase tracking-wider text-slate-600",
                "Location: Bangkok, Thailand"
            }
        }
        section {
            "This website was made by Dioxus Rust."
        }
    }
}
