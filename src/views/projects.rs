use dioxus::prelude::*;

struct Project {
    title: &'static str,
    role: &'static str,
    year: &'static str,
    description: &'static str,
    tech_stack: &'static [&'static str],
    link_text: &'static str,
    link_url: &'static str,
}

#[component]
pub fn Projects() -> Element {
    let projects = vec![
        Project {
            title: "Project Tracker",
            role: "Tech Lead",
            year: "2026",
            description: "A room availability and reservation platform built for the Chulalongkorn University Faculty of Engineering. Enables engineering students and committee members to discover available rooms, manage reservations, and coordinate shared spaces through real-time availability tracking and conflict-free scheduling.",
            tech_stack: &[],
            link_text: "[ GITHUB ]",
            link_url: "",
        },
        Project {
            title: "DevSecOps Security Pipeline",
            role: "",
            year: "2026",
            description: "Automated DevSecOps pipeline script for secret detection, container vulnerability auditing, app deployment, and self-hosted Nginx security dashboards.",
            tech_stack: &["Bash", "Docker", "Nginx", "Gitleaks", "Linux"],
            link_text: "[ GITHUB ]",
            link_url: "",
        },
        Project {
            title: "Concurrent Port Scanner",
            role: "",
            year: "2026",
            description: "A high-performance, concurrent TCP port scanner build in Go. Features a worker pool architecture, banner-based service fingerprinting, graceful shutdown on SIGINT/SIGTERM, and ordered output via an in-flight result buffer.",
            tech_stack: &["Go", "TCP/IP", "Concurrency"],
            link_text: "[ GITHUB ]",
            link_url: "https://github.com/MadManJJ/port-scanner",
        },
        Project {
            title: "PokeBattleP2P",
            role: "",
            year: "2025",
            description: "A decentralized, peer-to-peer multiplayer game built in Java. Utilizes low-level Java network sockets for real-time state synchronization, featuring a custom messaging protocol to handle player actions, combat resolution, and connection lifecycle management without a centralized server.",
            tech_stack: &["Java", "Sockets", "P2P Networking"],
            link_text: "[ GITHUB ]",
            link_url: "https://github.com/MadManJJ/PokeBattleP2P",
        },
        Project {
            title: "Vishnu 23rd and Intania First Date 2025",
            role: "",
            year: "2025",
            description: "A full-stack event website designed to facilitate ice-breaking activities for first-year engineering students. Features include real-time activity tracking, user registration, and an intuitive UI to drive student engagement during orientation.",
            tech_stack: &["Next.js", "Firebase"],
            link_text: "[ GITHUB ]",
            link_url: "https://github.com/vishnu-23rd/vishnu-23-website",
        },
    ];
    rsx! {
        div {
            h1 {
                class: "main-heading mb-0 pb-0",
                "PROJECTS"
            }
            p {
                class: "text-base mb-10",
                "Selected software, systems, and security projects."
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
                            if !project.role.is_empty() {
                                span { "| {project.role}" }
                            }
                            span { class: "text-sm text-gray-500 font-normal", "{project.year}" }
                        }
                        p {
                            class: "text-gray-700 mt-2 mb-4 max-w-4xl",
                            "{project.description}"
                        }
                        if !project.tech_stack.is_empty() {
                            p {
                                class: "font-mono text-slate-600 text-sm tracking-wide mb-2",
                                {project.tech_stack.join(" · ")}
                            }
                        }
                        if !project.link_url.is_empty() {
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
}
