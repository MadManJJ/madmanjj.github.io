use chrono::{FixedOffset, Utc};
use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;

fn format_bangkok_time() -> String {
    let bangkok = FixedOffset::east_opt(7 * 3600).unwrap();
    Utc::now()
        .with_timezone(&bangkok)
        .format("%H:%M:%S")
        .to_string()
}

#[component]
pub fn Home() -> Element {
    let mut bangkok_time = use_signal(format_bangkok_time);

    use_effect(move || {
        spawn(async move {
            loop {
                TimeoutFuture::new(1_000).await;
                bangkok_time.set(format_bangkok_time());
            }
        });
    });
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
                section {
                    class: "text-md text-slate-500 space-y-2 uppercase mt-3",
                    p {
                        class: "mb-0",
                        "Local Time in Bangkok"
                    }
                    p {
                        class: "text-2xl text-black font-bold normal-case not-italic",
                        "{bangkok_time}"
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
