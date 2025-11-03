#![allow(non_snake_case)]
use dioxus::document::Stylesheet;
use dioxus::prelude::*;
use uuid::Uuid;

use crate::models::main::{MobileMenuOpen, NewLessonModalOpen};
use crate::pages::chat::Chat;
use crate::pages::new_lesson_modal::NewLessonModal;
use crate::pages::sidebar::Sidebar;
use crate::pages::welcome::Welcome;

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
    let mut is_menu_open = use_signal(|| false);
    let close_menu = move || is_menu_open.set(false);
    let mut is_new_lesson_open = use_signal(|| false);

    use_context_provider(|| NewLessonModalOpen {
        is_open: is_new_lesson_open,
    });

    use_context_provider(|| MobileMenuOpen {
        is_open: is_menu_open,
    });

    rsx! {
        Stylesheet { href: asset!("assets/output.css") }

        div { class: "relative min-h-screen",
            // Main application layout
            div { class: "flex h-screen overflow-hidden",
                div {
                    class: "absolute md:relative transition-transform duration-300",
                    class: "md:translate-x-0",

                    class: if is_menu_open() {
                        "translate-x-0"
                    } else {
                        "-translate-x-full"
                    },
                    Sidebar { on_close_menu: close_menu }
                }

                if is_menu_open() {
                    div {
                        class: "absolute inset-0 bg-black/50 z-10 md:hidden",
                        onclick: move |_| is_menu_open.set(false)
                    }
                }

                Outlet::<Route> {}
            }

            // Modal overlay - as direct child of outer container
            if is_new_lesson_open() {
                NewLessonModal {
                    on_close: move |_| is_new_lesson_open.set(false)
                }
            }
        }
    }
}
