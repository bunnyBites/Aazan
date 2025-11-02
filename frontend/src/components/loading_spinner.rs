#![allow(non_snake_case)]
use dioxus::prelude::*;

pub fn LoadingSpinner() -> Element {
    rsx! {
        div { class: "flex justify-center items-center w-full h-full",
            div {
                class: "w-12 h-12 border-4 border-indigo-600 border-t-transparent rounded-full animate-spin"
            }
        }
    }
}
