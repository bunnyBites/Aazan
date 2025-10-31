#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::models::session_item::SessionItemProps;

pub fn SessionItem(props: SessionItemProps) -> Element {
    let active_class = if props.is_active {
        "bg-indigo-100 border-l-4 border-indigo-600"
    } else {
        "hover:bg-gray-100"
    };

    rsx! {
        div {
            class: "p-4 cursor-pointer {active_class} border-b border-gray-200",
            onclick: move |_| {
                tracing::info!("Clicked on session: {}", props.title);
            },
            h3 { class: "font-semibold text-gray-800", "{props.title}" }
            p { class: "text-sm text-gray-500", "{props.last_updated}" }
        }
    }
}
