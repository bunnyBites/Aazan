#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::components::message_bubble::{MessageBubble, MessageRole};

#[derive(PartialEq, Clone)]
struct Message {
    text: String,
    role: MessageRole,
}

pub fn ChatInterface() -> Element {
    let messages = use_signal(|| {
        vec![
        Message {
            text: "Hello Bodhi! I want to teach you about the Rust programming language.".to_string(),
            role: MessageRole::User,
        },
        Message {
            text: "That sounds exciting! I'm ready to learn. What is the most important feature of Rust?".to_string(),
            role: MessageRole::Assistant,
        },
        Message {
            text: "That would be its ownership and borrowing system. It guarantees memory safety without needing a garbage collector.".to_string(),
            role: MessageRole::User,
        },
    ]
    });

    rsx! {
        div { class: "flex flex-col h-screen bg-gray-100",
            header { class: "bg-white shadow-md p-4",
                h1 { class: "text-2xl font-bold text-gray-800", "Aazan Chat" }
            }

            // message list area
            main { class: "flex-1 overflow-y-auto p-4",
              div { class: "flex flex-col space-y-4",
                  // iterate over the messages and render a MessageBubble for each
                  for message in messages() {
                      MessageBubble { text: message.text.clone(), role: message.role }
                  }
              }
            }

            footer {
              class: "bg-white p-4 shadow-inner",
                  div { class: "flex items-center",
                      input {
                          class: "flex-1 border rounded-full py-2 px-4 mr-4",
                          placeholder: "Teach your lesson here...",
                          "type": "text"
                      }
                      button {
                          class: "bg-indigo-600 text-white rounded-full p-3 hover:bg-indigo-700",
                          "Send"
                      }
                  }
            }
        }
    }
}
