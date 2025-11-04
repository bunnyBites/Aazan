#![allow(non_snake_case)]
use dioxus::{document::Stylesheet, prelude::*};
use uuid::Uuid;

use crate::pages::chat::Chat;
use crate::pages::sidebar::Sidebar;

mod components;
mod controllers;
mod models;
mod pages;

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[layout(AppLayout)] // All routes will use this layout
    // The chat interface, which takes a session ID from the URL
    #[route("/session/:id")]
    Chat { id: Uuid },
    // A welcome page for the root URL
    #[route("/")]
    Welcome {},
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn AppLayout() -> Element {
    rsx! {
      Stylesheet { href: asset!("assets/output.css") }
      div { class: "flex h-screen",
            Sidebar {}
            Outlet::<Route> {}
        }
    }
}

#[component]
fn Welcome() -> Element {
    rsx! {
        div { class: "flex-1 flex justify-center items-center bg-gray-100",
            h1 { class: "text-2xl text-center text-gray-500", "Select a lesson to get started." }
        }
    }
}
