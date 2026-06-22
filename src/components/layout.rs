use super::aside::Aside;
use crate::components::footer;
use crate::route::Route;

use dioxus::prelude::*;

#[component]
pub fn Layout() -> Element {
    rsx! {
        div {
            class: "w-[80%]",
            div {
                class: "flex flex-col gap-5 min-h-full mr-5",
                Outlet::<Route> {}
                div {
                    class: "mt-auto",
                    footer::Footer {}
                }
            }
        }
        div {
            class: "w-[20%]",
            Aside {}
        }
    }
}
