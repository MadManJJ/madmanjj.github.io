use crate::views::{home, page_not_found, projects, reading_list, resume};
use dioxus::prelude::*;

#[derive(Routable, PartialEq, Clone)]
pub enum Route {
    #[route("/", home::Home)]
    Home {},

    #[route("/resume", resume::Resume)]
    Resume {},

    #[route("/projects", projects::Projects)]
    Projects {},

    #[route("/reading-list", reading_list::ReadingList)]
    ReadingList {},

    #[route("/:..segments", page_not_found::PageNotFound)]
    PageNotFound { segments: Vec<String> },
}
