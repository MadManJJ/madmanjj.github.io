use dioxus::prelude::*;

const RESUME_FILE: Asset = asset!("/assets/pannawich-resume.pdf");

#[component]
pub fn Resume() -> Element {
    rsx! {
        div {
            h1 {
                class: "main-heading mb-0 pb-0",
                "RESUME"
            }
            a {
                class: "border-b-1 border-current pb-[2px] w-fit font-semibold cursor-pointer",
                href: "{RESUME_FILE}",
                target: "_blank",
                rel: "noopener noreferrer",
                "[ DOWNLOAD_CV ]"
            }
            object {
                class: "w-full min-h-screen mt-6",
                data: "{RESUME_FILE}",
                r#type: "application/pdf",

                p {
                    "Your browser cannot display the PDF. "
                    a {
                        class: "underline font-semibold",
                        href: "{RESUME_FILE}",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        "Open or download the resume."
                    }
                }
            }
        }
    }
}
