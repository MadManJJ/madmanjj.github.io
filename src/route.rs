use crate::components::{Home, PageNotFound};
use dioxus::prelude::*;

#[derive(Routable, PartialEq, Clone)]
pub enum Route {
    #[route("/")]
    Home,
    #[route("/:..segments")]
    PageNotFound { segments: Vec<String> },
}
