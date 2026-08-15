use dioxus::prelude::*;

struct Project {
    title: &'static str,
    year: &'static str,
    description: &'static str,
    link_text: &'static str,
    link_url: &'static str,
}

#[component]
pub fn Homelabs() -> Element {
    let projects = vec![
        Project {
            title: "Self-Hosted Homelab",
            year: "2026",
            description: "A personal homelab environment for hosting and managing self-hosted services across multiple devices. Runs Homepage as a centralized service dashboard alongside Home Assistant for home automation and File Browser for remote file management. Devices are connected through a private Tailscale mesh network, providing secure remote access to internal services without exposing them directly to the public internet.",
            link_text: "[ GITHUB ]",
            link_url: "https://github.com/MadManJJ/homelab",
        },
        Project {
            title: "Homelab DevSecOps Pipeline",
            year: "2026",
            description: "A multi-node homelab setup featuring a Windows workstation triggering remote automation on a headless Fedora server via SSH. Executes a Bash DevSecOps pipeline that runs secret detection (Gitleaks), filesystem and container image vulnerability audits (Trivy), and target application containerization. Dynamically generates security report artifacts and serves them through an isolated, self-hosted Nginx dashboard.",
            link_text: "[ GITHUB ]",
            link_url: "https://github.com/MadManJJ/appsec-scan-dashboard",
        },
    ];
    rsx! {
        div {
            h1 {
                class: "main-heading mb-0 pb-0",
                "HOMELABS"
            }
            p {
                class: "text-base mb-10",
                "Self-hosted infrastructure, local pipelines, and active nodes."
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
                            target: "_blank",
                            "{project.link_text}"
                        }
                    }
                }
            }
        }
    }
}
