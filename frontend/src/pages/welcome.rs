use dioxus::prelude::*;

pub fn Welcome() -> Element {
    rsx! {
        div { class: "flex-1 flex justify-center items-center bg-gray-100",
            h1 { class: "text-2xl text-center text-gray-500", "Select a lesson to get started." }
        }
    }
}
