use dioxus::prelude::*;

const RESUME_FILE: Asset = asset!("/assets/pannawich-resume.pdf");

#[component]
pub fn Resume() -> Element {
    let user = "pannawich.tha";
    let domain = "gmail.com";
    let full_email = format!("{}@{}", user, domain);
    rsx! {
        div {
            h1 {
                class: "main-heading mb-0 pb-0",
                "RESUME"
            }
            div {
                class: "flex flex-row gap-4 mb-10",
                a {
                    class: "border-b-1 border-current pb-[2px] w-fit font-semibold cursor-pointer",
                    href: "{RESUME_FILE}",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    "[ DOWNLOAD_CV ]"
                }
                div {
                    class: "text-slate-700",
                    "/ UPDATED JUN 2026"
                }
            }
            div {
                div {
                    class: "flex flex-col items-center justify-center w-full",
                    h1 {
                        class: "uppercase font-bold text-2xl",
                        "Pannawich Thamart"
                    }
                    div {
                        class: "flex flex-row gap-2 border-b-1 pb-2 w-full justify-center",
                        a {
                            href: "mailto:{full_email}",
                            class: "font-serif underline",
                            "{full_email}"
                        }
                        " | "
                        a {
                            class: "underline font-serif tracking-wide",
                            href: "https://github.com/MadManJJ",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            "Github"
                        }
                        " | "
                        a {
                            class: "underline font-serif tracking-wide",
                            href: "https://www.linkedin.com/in/pannawich-thamart/",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            "Linkedin"
                        }
                    }
                }
            }
            div {
                class: "mt-6",
                h1 {
                    class: "font-semibold text-lg mb-0 pb-0",
                    "EDUCATION"
                }
                div {
                    class: "border-b-1 pb-4 w-full",
                    p {
                        class: "text-xl",
                        "Bachelor of Engineering Program in Computer Engineering and Digital Technology"
                    }
                    p {
                        class: "text-slate-700",
                        "Chulalongkorn University - Bangkok, Thailand"
                    }
                    p {
                        class: "text-slate-700",
                        "Current GPAX: 3.95"
                    }
                }
            }
            div {
                class: "mt-6",
                h1 {
                    class: "font-semibold text-lg mb-0 pb-0",
                    "TECHNICAL SKILLS"
                }
                div {
                    class: "border-b-1 pb-4 w-full space-y-1",
                    ul {
                        class: "list-disc list-inside",
                        li {
                            class: "text-slate-700",
                            span { class: "font-semibold", "Frontend:" }
                            " TypeScript, React, Next.js, Redux, TailwindCSS, Shadcn UI"
                        }
                        li {
                            class: "text-slate-700",
                            span { class: "font-semibold", "Backend:" }
                            " Go (Testify), Node.js (Express, Nest.js), Java (Spring Boot)"
                        }
                        li {
                            class: "text-slate-700",
                            span { class: "font-semibold", "Databases & Caching:" }
                            " PostgreSQL, MySQL, Redis, MongoDB"
                        }
                        li {
                            class: "text-slate-700",
                            span { class: "font-semibold", "Cloud & Infrastructure:" }
                            " AWS, Terraform, Docker, Kubernetes"
                        }
                        li {
                            class: "text-slate-700",
                            span { class: "font-semibold", "CI/CD & Testing:" }
                            " GitHub Actions, GitLab CI/CD, Playwright"
                        }
                    }
                }
            }
            div {
                class: "mt-6",
                h1 {
                    class: "font-semibold text-lg mb-0 pb-0",
                    "PROFESSIONAL EXPERIENCE"
                }
                div {
                    class: "border-b-1 pb-4 w-full space-y-4",
                    div {
                        div {
                            class: "flex justify-between items-baseline",
                            p {
                                class: "font-semibold",
                                span { "Technical Lead" }
                                " | Chula ESC"
                            }
                            p { class: "text-slate-700 text-sm", "August 2026 – Present" }
                        }
                        ul {
                            class: "list-disc list-inside text-slate-700",
                            li { "Established engineering best practices, including code reviews, comprehensive testing workflows, and automated CI/CD pipelines." }
                            li { "Mentored developers across the full stack to ensure high-quality code delivery." }
                        }
                    }
                    div {
                        div {
                            class: "flex justify-between items-baseline",
                            p {
                                class: "font-semibold",
                                span { "FullStack Freelancer" }
                                " | MEE PALANG MAI CO., LTD."
                            }
                            p { class: "text-slate-700 text-sm", "June 2026 – Present" }
                        }
                        ul {
                            class: "list-disc list-inside text-slate-700",
                            li { "Developed 'Curious', a live audience engagement platform for Chulalongkorn University supporting real-time quizzes, polls, and Q&A." }
                            li { "Built full-stack features utilizing Next.js, Go, MongoDB, and AWS to ensure real-time scalability for concurrent users." }
                        }
                    }
                    div {
                        div {
                            class: "flex justify-between items-baseline",
                            p {
                                class: "font-semibold",
                                span { "FullStack Engineer Intern" }
                                " | OxygenAI Co., Ltd. / OMNI DEAL CO., LTD."
                            }
                            p { class: "text-slate-700 text-sm", "May 2026 – July 2026" }
                        }
                        ul {
                            class: "list-disc list-inside text-slate-700",
                            li { "Maintained and extended distributed features across a polyglot microservice ecosystem using Go and Python." }
                            li { "Diagnosed and debugged asynchronous data flows via NATS, troubleshooting event-driven communication issues to ensure reliable message delivery between decoupled services." }
                        }
                    }
                    div {
                        div {
                            class: "flex justify-between items-baseline",
                            p {
                                class: "font-semibold",
                                span { "Backend Developer Intern" }
                                " | Apppi co. ltd"
                            }
                            p { class: "text-slate-700 text-sm", "May 2025 – June 2025" }
                        }
                        ul {
                            class: "list-disc list-inside text-slate-700",
                            li { "Developed scalable RESTful APIs in Go, using Hexagonal Architecture principles to ensure highly maintainable and decoupled codebases." }
                            li { "Designed and integrated secure third-party authentication flows utilizing LINE Login OAuth, streamlining the user onboarding experience." }
                            li { "Built robust API middleware for input validation and data sanitization, safeguarding system endpoints against common vulnerabilities." }
                            li { "Established comprehensive unit testing suites using testing and Testify to drive code reliability and maintain high test coverage." }
                        }
                    }
                }
            }
            div {
                class: "mt-6",
                h1 {
                    class: "font-semibold text-lg mb-0 pb-0",
                    "PROJECTS"
                }
                div {
                    class: "pb-4 w-full space-y-4",
                    div {
                        div {
                            class: "flex justify-between items-baseline",
                            p {
                                class: "font-semibold",
                                "PokeBattleP2P "
                                a {
                                    class: "text-blue-600 underline text-sm font-normal",
                                    href: "https://github.com/MadManJJ/PokeBattleP2P",
                                    target: "_blank",
                                    rel: "noopener noreferrer",
                                    "(github.com/MadManJJ/PokeBattleP2P)"
                                }
                            }
                        }
                        ul {
                            class: "list-disc list-inside text-slate-700",
                            li { "Engineered a peer-to-peer multiplayer game utilizing Java sockets to facilitate low-latency, real-time gameplay and data synchronization between players." }
                        }
                    }
                    div {
                        div {
                            class: "flex justify-between items-baseline",
                            p {
                                class: "font-semibold",
                                "Vishnu 23rd and Intania First Date 2025 "
                                a {
                                    class: "text-blue-600 underline text-sm font-normal",
                                    href: "https://github.com/vishnu-23rd/vishnu-23-website",
                                    target: "_blank",
                                    rel: "noopener noreferrer",
                                    "(github.com/vishnu-23rd/vishnu-23-website)"
                                }
                            }
                        }
                        ul {
                            class: "list-disc list-inside text-slate-700",
                            li { "Designed and implemented the database architecture and API utilizing Firebase and Next.js for a real-time, interactive Bingo application." }
                            li { "Developed a responsive web platform to facilitate engagement and networking for incoming first-year university students." }
                        }
                    }
                    div {
                        div {
                            class: "flex justify-between items-baseline",
                            p {
                                class: "font-semibold",
                                "Concurrent Port Scanner "
                                a {
                                    class: "text-blue-600 underline text-sm font-normal",
                                    href: "https://github.com/MadManJJ/port-scanner",
                                    target: "_blank",
                                    rel: "noopener noreferrer",
                                    "(https://github.com/MadManJJ/port-scanner)"
                                }
                            }
                        }
                        ul {
                            class: "list-disc list-inside text-slate-700",
                            li { "Developed a highly concurrent TCP port scanner and banner grabber using Go." }
                            li { "Implemented worker pools and goroutines to efficiently manage thousands of simultaneous network connections and sequence asynchronous results." }
                        }
                    }
                }
            }
        }
    }
}
