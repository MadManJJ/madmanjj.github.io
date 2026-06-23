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
                            li { "Architected and led the development of core campus digital platforms, managing a team of developers to deliver scalable software solutions." }
                            li { "Established engineering best practices, including code reviews, comprehensive testing workflows, and automated CI/CD pipelines." }
                            li { "Mentored junior developers in frontend and backend technologies, accelerating sprint velocity and software delivery." }
                        }
                    }
                    div {
                        div {
                            class: "flex justify-between items-baseline",
                            p {
                                class: "font-semibold",
                                span { "Full Stack Freelancer" }
                                " | MEE PALANG MAI CO., LTD."
                            }
                            p { class: "text-slate-700 text-sm", "June 2026 – Present" }
                        }
                        ul {
                            class: "list-disc list-inside text-slate-700",
                            li { "Developing Curious, a live audience engagement platform inspired by Kahoot and Mentimeter for Chulalongkorn University." }
                            li { "Built full-stack features using Next.js, Go, MongoDB, and AWS-based services while collaborating with stakeholders to deliver real-time quizzes, polls, and Q&A functionality." }
                        }
                    }
                    div {
                        div {
                            class: "flex justify-between items-baseline",
                            p {
                                class: "font-semibold",
                                span { "Full Stack Engineer Intern" }
                                " | OxygenAI Co., Ltd. / OMNI DEAL CO., LTD."
                            }
                            p { class: "text-slate-700 text-sm", "May 2026 – July 2026" }
                        }
                        ul {
                            class: "list-disc list-inside text-slate-700",
                            li { "Developed and maintained end-to-end web applications, leveraging Angular for the frontend and Go for the backend architecture." }
                            li { "Integrated secure RESTful APIs using Go and optimized client-side state management and component performance in Angular, significantly improving page responsiveness." }
                            li { "Collaborated with cross-functional teams in an Agile environment to rapidly scope, build, and deploy high-impact features." }
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
                            li { "Engineered highly performant RESTful APIs in Go, adhering strictly to Clean Architecture principles (Handler, Service, Repository layers)." }
                            li { "Authored comprehensive unit tests utilizing Go testing and Testify, significantly increasing code coverage and system reliability." }
                            li { "Integrated LINE Login OAuth for secure, streamlined user authentication." }
                            li { "Enforced rigid security measures, including input sanitization and validation protocols to prevent injection attacks." }
                            li { "Utilized GitLab for version control, actively driving regular code reviews and participating in fast-paced Agile sprints." }
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
                            p { class: "font-semibold", "Curious" }
                        }
                        p {
                            class: "text-slate-700 mb-2",
                            "A live audience engagement platform for Chulalongkorn University inspired by Kahoot and Mentimeter, enabling students to participate in real-time quizzes, polls, and interactive Q&A sessions."
                        }
                        p { class: "text-slate-700", "My Role" }
                        ul {
                            class: "list-disc list-inside text-slate-700",
                            li { "Implement the content management for each Activity, Slide." }
                            li { "Implement token-based authentication." }
                        }
                    }
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
                        p {
                            class: "text-slate-700 mb-2",
                            "A peer-to-peer multiplayer game using Java sockets to enable realtime gameplay between players."
                        }
                        p { class: "text-slate-700", "My Role" }
                        ul {
                            class: "list-disc list-inside text-slate-700",
                            li { "Implemented the logic behind the fighting between two players and sending data from each player via a socket using Java and Java socket." }
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
                        p {
                            class: "text-slate-700 mb-2",
                            "A website for first-year students to participate in activity to get to know each other."
                        }
                        p { class: "text-slate-700", "My Role" }
                        ul {
                            class: "list-disc list-inside text-slate-700",
                            li { "Designed and implemented the database and API for the Bingo page using Firebase, Next.js." }
                        }
                    }
                }
            }
        }
    }
}
