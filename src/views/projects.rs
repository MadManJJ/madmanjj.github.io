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
            title: "PokeBattleP2P",
            year: "2025",
            description: "A decentralized, peer-to-peer multiplayer game built in Java. Utilizes low-level Java network sockets for real-time state synchronization, featuring a custom messaging protocol to handle player actions, combat resolution, and connection lifecycle management without a centralized server.",
            link_text: "[ GITHUB ]",
            link_url: "https://github.com/MadManJJ/PokeBattleP2P",
        },
        Project {
            title: "Vishnu 23rd and Intania First Date 2025",
            year: "2025",
            description: "A full-stack event website designed to facilitate ice-breaking activities for first-year engineering students. Features include real-time activity tracking, user registration, and an intuitive UI to drive student engagement during orientation.",
            link_text: "[ GITHUB ]",
            link_url: "https://github.com/vishnu-23rd/vishnu-23-website",
        },
        Project {
            title: "Concurrent Port Scanner",
            year: "2025",
            description: "A high-performance, concurrent TCP port scanner build in Go. Features a worker pool architecture, banner-based service fingerprinting, graceful shutdown on SIGINT/SIGTERM, and ordered output via an in-flight result buffer.",
            link_text: "[ GITHUB ]",
            link_url: "https://github.com/MadManJJ/port-scanner",
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
