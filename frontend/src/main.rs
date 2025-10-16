#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::pages::chat_interface::ChatInterface;

mod components;
mod controllers;
mod models;
mod pages;

fn main() {
    launch(App);
}

fn App() -> Element {
    rsx! {
        ChatInterface {}
    }
}
