#![allow(non_snake_case)]
use dioxus::prelude::*;
use uuid::Uuid;

use crate::models::api::MessageRole as ApiMessageRole;
use crate::{
    components::message_bubble::MessageBubble, controllers::api::get_messages,
    models::message_bubble::MessageRole as ViewMessageRole,
};
pub fn ChatInterface() -> Element {
    let session_id = Uuid::parse_str("urn:uuid:b9d36136-3fd5-4f92-a015-3f9ec68aec85").unwrap();

    // 3. use_resource is a hook for running an async task.
    //    Dioxus will re-render the component when the task completes.
    let messages = use_resource(move || get_messages(session_id));

    rsx! {
        div { class: "flex flex-col h-screen bg-gray-100",
            header { class: "bg-white shadow-md p-4",
                h1 { class: "text-2xl font-bold text-gray-800", "Aazan Chat" }
            }

            // message list area
            main { class: "flex-1 overflow-y-auto p-4",
                div { class: "flex flex-col space-y-4",
                    // 4. Handle the different states of the resource
                    match &*messages.read() {
                        Some(Ok(message_list)) => rsx! {
                            {message_list.iter().map(|message| {
                                // We need to convert the API Role to the View Role
                                let view_role = match message.role {
                                    ApiMessageRole::User => ViewMessageRole::User,
                                    ApiMessageRole::Assistant => ViewMessageRole::Assistant,
                                };
                                rsx! {
                                    MessageBubble { text: message.content.clone(), role: view_role }
                                }
                            })}
                        },
                        Some(Err(e)) => rsx! { p { "Error fetching messages: {e}" } },
                        None => rsx! { p { "Loading messages..." } },
                    }
                }
            }

            footer {
                class: "bg-white p-4 shadow-inner",
                div { class: "flex items-center",
                    input {
                        class: "flex-1 border rounded-full py-2 px-4 mr-4",
                        placeholder: "Teach your lesson here...",
                        r#type: "text"
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
