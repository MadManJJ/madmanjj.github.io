use dioxus::prelude::*;

struct Project {
    title: &'static str,
    year: &'static str,
    description: &'static str,
    link_text: &'static str,
    link_url: &'static str,
}

#[component]
pub fn Projects() -> Element {
    let projects = vec![
        Project {
            title: "CUrious",
            year: "2026",
            description: "A live audience engagement platform for Chulalongkorn University inspired by Kahoot and Mentimeter, enabling students to participate in real-time quizzes, polls, and interactive Q&A sessions.",
            link_text: "",
            link_url: "", 
        },
        Project {
            title: "PokeBattleP2P",
            year: "2025",
            description: "A peer-to-peer multiplayer game using Java sockets to enable realtime gameplay between players.",
            link_text: "[ GITHUB ]",
            link_url: "https://github.com/MadManJJ/PokeBattleP2P",
        },
        Project {
            title: "Vishnu 23rd and Intania First Date 2025",
            year: "2025",
            description: "A website for first-year students to participate in activity to get to know each other.",
            link_text: "[ GITHUB ]",
            link_url: "https://github.com/vishnu-23rd/vishnu-23-website",
        }
    ];
    rsx! {
        div {
            h1 {
                class: "main-heading mb-0 pb-0",
                "PROJECTS"
            }
            p {
                class: "text-base mb-10",
                "A selection of technical experiments."
            }
            div {
                class: "flex flex-col gap-12",

                for project in projects {
                    div {
                        key: "{project.title}",
                        class: "project-item",

                        h2 {
                            class: "text-xl font-bold inline-flex items-center gap-2",
                            "{project.title}"
                            span { class: "text-sm text-gray-500 font-normal", "{project.year}" }
                        }
                        p {
                            class: "text-gray-700 mt-2 mb-4 max-w-2xl",
                            "{project.description}"
                        }
                        a {
                            class: "text-blue-800 font-bold hover:underline text-sm tracking-wide",
                            href: "{project.link_url}",
                            "{project.link_text}"
                        }
                    }
                }
            }
        }
    }
}
