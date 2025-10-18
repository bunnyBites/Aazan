#![allow(non_snake_case)]
use dioxus::{document::Stylesheet, prelude::*};

use crate::pages::chat_interface::ChatInterface;

mod components;
mod controllers;
mod models;
mod pages;

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
      Stylesheet { href: asset!("assets/tailwind.css") }
      ChatInterface {}
    }
}
