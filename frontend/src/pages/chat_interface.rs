#![allow(non_snake_case)]
use dioxus::prelude::*;
use futures_channel::mpsc::UnboundedReceiver;
use futures_util::StreamExt;
use uuid::Uuid;

use crate::components::microphone_button::MicrophoneButton;
use crate::controllers::api::get_messages;
use crate::controllers::message_bubble::send_message;
use crate::models::api::MessageRole as ApiMessageRole;
use crate::{
    components::message_bubble::MessageBubble,
    models::message_bubble::MessageRole as ViewMessageRole,
};

pub fn ChatInterface() -> Element {
    let mut new_message_text = use_signal(String::new);
    let session_id = Uuid::parse_str("urn:uuid:b9d36136-3fd5-4f92-a015-3f9ec68aec85").unwrap();
    let messages = use_resource(move || get_messages(session_id));
    let mut speech_eval = use_signal(|| None::<document::Eval>);

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

    let _from_js = use_coroutine(move |mut rx: UnboundedReceiver<String>| {
        let mut new_message_text = new_message_text.clone();

        async move {
            while let Some(js_message) = rx.next().await {
                tracing::info!("Got text from JS: {}", js_message);

                if !js_message.is_empty() && !js_message.starts_with("Error") {
                    new_message_text.set(js_message);
                } else if js_message.starts_with("Error") {
                    tracing::error!("JS Speech Error: {}", js_message);
                }
            }
        }
    });

    use_effect(move || {
        let evaluator = document::eval(
            r#"
                const SpeechRecognition = window.SpeechRecognition || window.webkitSpeechRecognition;
                if (!SpeechRecognition) {
                    console.error("SpeechRecognition API not supported in this browser.");
                    dioxus.send("Error: Speech recognition not supported.");
                } else {
                    const recognition = new SpeechRecognition();
                    recognition.continuous = false;
                    recognition.interimResults = false;
                    recognition.lang = 'en-US';

                    recognition.onresult = (event) => {
                        const text = event.results[0][0].transcript;
                        console.log('Speech result:', text);
                        dioxus.send(text); // Send transcribed text to `from_js` coroutine
                    };

                    recognition.onerror = (event) => {
                        console.error('Speech recognition error:', event.error);
                        dioxus.send("Error during recognition: " + event.error);
                    };

                    (async () => {
                        while(true) {
                            const msg = await dioxus.recv(); // Wait for "start" or "stop"
                            if (msg === "start") {
                                try {
                                    console.log("Starting speech recognition...");
                                    recognition.start();
                                } catch(e) {
                                    console.error("Error starting recognition:", e);
                                }
                            } else if (msg === "stop") {
                                try {
                                    console.log("Stopping speech recognition...");
                                    recognition.stop();
                                } catch(e) {
                                    console.error("Error stopping recognition:", e);
                                }
                            }
                        }
                    })();
                }
            "#,
        );
        speech_eval.set(Some(evaluator));
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
                        class: "bg-indigo-600 text-white rounded-full p-3 hover:bg-indigo-700",
                        onclick: move |_| {
                            if !new_message_text.read().is_empty() {
                                sender.send(new_message_text.read().clone());
                                new_message_text.set(String::new());
                            }
                        },
                        "Send"
                    }
                    MicrophoneButton {
                        on_click: move |is_recording| {
                            if let Some(evaluator) = speech_eval.read().as_ref() {
                                if is_recording {
                                    evaluator.send("start").ok();
                                } else {
                                    evaluator.send("stop").ok();
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
