#![allow(non_snake_case)]
use dioxus::prelude::*;
use uuid::Uuid;

use crate::Route;

#[derive(Props, PartialEq, Clone)]
pub struct SessionItemProps {
    pub id: Uuid,
    pub title: String,
    pub last_updated: String,
    #[props(default = false)]
    pub is_active: bool,
}

pub fn SessionItem(props: SessionItemProps) -> Element {
    let active_class = if props.is_active {
        "bg-indigo-100 border-l-4 border-indigo-600"
    } else {
        "hover:bg-gray-100"
    };

    let target_route = Route::Chat { id: props.id };

    rsx! {
      Link {
        to: target_route,
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
}
