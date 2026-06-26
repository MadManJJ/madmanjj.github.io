use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let user = "pannawich.tha";
    let domain = "gmail.com";
    let full_email = format!("{}@{}", user, domain);
    rsx! {
        div {
            class: "flex flex-row",
            div {
                class: "home-content flex flex-col gap-5",
                h1 {
                    class: "main-heading",
                    "ABOUT_ME"
                }
                section {
                    class: "text-xl",
                    p {
                        "I am a fullstack engineer focusing on backend and infrastructure,"
                        br {}
                        "with experience contributing to microservice architectures."
                    }

                    p {
                        class: "mt-4",
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
                    class: "text-sm text-slate-500 space-y-2",
                    p {
                        class: "italic",
                        "This website was built with Rust and Dioxus... mostly because I was bored."
                    }
                    p {
                        "It also gives me a convenient excuse to share my "
                        Link {
                            to: "/reading-list",
                            class: "underline font-semibold",
                            "book collection"
                        }
                    }
                }
                h1 {
                    class: "font-semibold text-xl mt-2 mb-0",
                    "AVAILABILITY"
                }

                section {
                    class: "text-xl",
                    p {
                        "I am currently open to freelance contracts, full-time backend/infra roles, or interesting open-source collaborations. "
                        "Reach me at "
                        a {
                            href: "mailto:{full_email}",
                            class: "font-serif font-bold underline",
                            "{full_email}"
                        }
                        " or check my "
                        a {
                            class: "font-serif font-bold underline",
                            href: "https://github.com/MadManJJ",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            "Github"
                        }
                    }
                }
            }
            img {
                class: "profile-image shrink-0 mr-12",
                src: asset!("/assets/profile.jpg"),
                alt: "Profile picture",
            }
        }
    }
}
