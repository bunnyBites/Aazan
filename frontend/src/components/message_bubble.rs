#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::models::message_bubble::{MessageBubbleProps, MessageRole};

pub fn MessageBubble(props: MessageBubbleProps) -> Element {
    let (bubble_class, text_class) = match props.role {
        MessageRole::User => ("bg-indigo-600 self-end", "text-white"),
        MessageRole::Assistant => ("bg-white self-start", "text-gray-800"),
    };

    rsx! {
        div {
            class: "max-w-md p-3 rounded-lg shadow-md {bubble_class}",
            p { class: "{text_class}", "{props.text}" }
        }
    }
}
