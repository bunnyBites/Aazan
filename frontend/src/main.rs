#![allow(non_snake_case)]
use dioxus::{document::Stylesheet, prelude::*};

use crate::pages::{chat_interface::ChatInterface, sidebar::Sidebar};

mod components;
mod controllers;
mod models;
mod pages;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
      Stylesheet { href: asset!("assets/output.css") }
      div { class: "flex h-screen",
            Sidebar {}
            ChatInterface {}
        }
    }
}
