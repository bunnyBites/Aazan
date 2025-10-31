#![allow(non_snake_case)]
use dioxus::prelude::*;
use futures_channel::mpsc::UnboundedReceiver;
use futures_util::{FutureExt, StreamExt};
use serde_json::Value;
use uuid::Uuid;

use crate::components::microphone_button::MicrophoneButton;
use crate::controllers::api::get_messages;
use crate::controllers::message_bubble::send_message;
use crate::models::api::MessageRole as ApiMessageRole;
use crate::{
    components::message_bubble::MessageBubble,
    models::message_bubble::MessageRole as ViewMessageRole,
};

enum SpeechAction {
    Start,
    Stop,
}

pub fn ChatInterface() -> Element {
    let mut new_message_text = use_signal(String::new);
    let session_id = Uuid::parse_str("urn:uuid:b9d36136-3fd5-4f92-a015-3f9ec68aec85").unwrap();
    let messages = use_resource(move || get_messages(session_id));

    let sender = use_coroutine(move |mut rx| {
        let mut messages = messages.clone();
        async move {
            while let Some(content) = rx.next().await {
                if let Ok(_new_messages) = send_message(session_id, content).await {
                    messages.restart();
                }
            }
        }
    });

    let speech_manager = use_coroutine(move |mut rx: UnboundedReceiver<SpeechAction>| {
        let sender = sender.clone();

        async move {
            let mut evaluator = document::eval(
                r#"
                    const SpeechRecognition = window.SpeechRecognition || window.webkitSpeechRecognition;
                    if (!SpeechRecognition) {
                        dioxus.send("Error: Speech recognition not supported.");
                    } else {
                        const recognition = new SpeechRecognition();
                        recognition.continuous = false;
                        recognition.interimResults = false;
                        recognition.lang = 'en-US';

                        recognition.onresult = (event) => {
                            const text = event.results[0][0].transcript;
                            dioxus.send(text); // Send transcribed text
                        };

                        recognition.onerror = (event) => {
                            dioxus.send("Error: " + event.error);
                        };

                        (async () => {
                            while(true) {
                                const msg = await dioxus.recv(); // Wait for "start" or "stop"
                                if (msg === "start") {
                                    try { recognition.start(); } catch(e) { console.error(e); }
                                } else if (msg === "stop") {
                                    try { recognition.stop(); } catch(e) { console.error(e); }
                                }
                            }
                        })();
                    }
                "#,
            );

            loop {
                futures::select! {
                    // listen for commands from our Rust UI (the button)
                    msg = rx.next().fuse() => {
                        if let Some(action) = msg {
                            match action {
                                SpeechAction::Start => evaluator.send("start").ok(),
                                SpeechAction::Stop => evaluator.send("stop").ok(),
                            };
                        }
                    },

                    // listen for messages from our JavaScript evaluator
                    js_msg = evaluator.recv().fuse() => {
                        if let Ok(Value::String(text)) = js_msg {
                            if !text.is_empty() && !text.starts_with("Error") {
                                tracing::info!("Got text from JS: {}", text);
                                sender.send(text); // forward to the backend
                            } else if text.starts_with("Error") {
                                tracing::error!("JS Speech Error: {}", text);
                            }
                        }
                    }
                }
            }
        }
    });

    rsx! {
        div { class: "flex flex-col h-screen bg-gray-100",
            header { class: "bg-white shadow-md p-4",
                h1 { class: "text-2xl font-bold text-gray-800", "Aazan Chat" }
            }

            main { class: "flex-1 overflow-y-auto p-4",
                div { class: "flex flex-col space-y-4",
                    match &*messages.read() {
                        Some(Ok(message_list)) => {
                            rsx! {
                                {message_list.iter().map(|message| {
                                    let view_role = match message.role {
                                        ApiMessageRole::User => ViewMessageRole::User,
                                        ApiMessageRole::Assistant => ViewMessageRole::Assistant,
                                    };
                                    rsx! {
                                        MessageBubble {
                                            key: "{message.id}",
                                            text: message.content.clone(),
                                            role: view_role
                                        }
                                    }
                                })}
                            }
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
                        r#type: "text",
                        value: "{new_message_text}",
                        oninput: move |event| new_message_text.set(event.value().clone()),
                    }
                    button {
                        class: "h-25 w-25 text-white rounded-full flex items-center justify-center hover:bg-indigo-700 transition-colors mr-4",
                        onclick: move |_| {
                            if !new_message_text.read().is_empty() {
                                sender.send(new_message_text.read().clone());
                                new_message_text.set(String::new());
                            }
                        },
                        // Send icon
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            width: "30",
                            height: "32",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "indigo",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            line {
                                x1: "22",
                                y1: "2",
                                x2: "11",
                                y2: "13"
                            }
                            polygon {
                                points: "22 2 15 22 11 13 2 9 22 2"
                            }
                        }
                    }
                    MicrophoneButton {
                      on_click: move |is_recording| {
                          if is_recording {
                              speech_manager.send(SpeechAction::Start);
                          } else {
                              speech_manager.send(SpeechAction::Stop);
                          }
                      }
                    }
                }
            }
        }
    }
}
