#![allow(non_snake_case)]
use dioxus::prelude::*;

fn main() {
    // This tells Dioxus to launch the App component in the browser
    launch(App);
}

fn App() -> Element {
    rsx! {
        div {
            h1 { "Hello, Aazan Frontend!" }
        }
    }
}
