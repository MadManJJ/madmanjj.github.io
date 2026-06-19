use super::aside::Aside;
use crate::route::Route;
use dioxus::prelude::*;

#[component]
pub fn Layout() -> Element {
    rsx! {
        div {
            class: "w-[80%]",
            Outlet::<Route> {}
        }
        div {
            class: "w-[20%]",
            Aside {}
        }
    }
}
