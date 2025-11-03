#![allow(non_snake_case)]
use anyhow;
use dioxus::prelude::*;

use crate::{controllers::api::create_session, Route};

#[derive(Props, PartialEq, Clone)]
pub struct NewLessonModalProps {
    pub on_close: EventHandler<()>,
}

pub fn NewLessonModal(props: NewLessonModalProps) -> Element {
    let mut topic = use_signal(|| String::new());
    let mut material = use_signal(|| String::new());
    let mut is_loading = use_signal(|| false);
    let mut error_message = use_signal(|| String::new());
    let navigator = use_navigator();

    // Use Dioxus 0.7's new action pattern for better async handling
    let mut create_session_action = use_action(move |(topic, material): (String, String)| {
        let navigator = navigator.clone();
        let on_close = props.on_close.clone();
        async move {
            if topic.is_empty() || material.is_empty() {
                return Err::<(), anyhow::Error>(anyhow::anyhow!(
                    "Topic and material cannot be empty"
                ));
            }

            create_session(topic, material)
                .await
                .map(|session| {
                    on_close.call(());
                    navigator.push(Route::Chat { id: session.id });
                })
                .map_err(|e| anyhow::anyhow!("Failed to create session: {}", e))
        }
    });

    let mut handle_create_session = move |_| {
        if is_loading() {
            return;
        }

        let topic = topic.read().clone();
        let material = material.read().clone();

        if topic.trim().is_empty() || material.trim().is_empty() {
            error_message.set("Please fill in both topic and material".to_string());
            return;
        }

        error_message.set(String::new());
        is_loading.set(true);
        create_session_action.call((topic, material));
    };

    // Handle action results
    use_effect(move || match create_session_action.value().as_ref() {
        Some(Ok(_)) => {
            is_loading.set(false);
        }
        Some(Err(err)) => {
            error_message.set(err.to_string());
            is_loading.set(false);
        }
        None => {}
    });

    // TODO: Add body scroll prevention back when use_effect cleanup syntax is figured out

    rsx! {
        // Modal container with proper z-index handling
        div {
            class: "fixed inset-0 z-[9999] flex justify-center items-center p-4",
            // Backdrop
            div {
                class: "absolute inset-0 bg-black/70 backdrop-blur-md transition-opacity duration-300 ease-out",
                onclick: move |_| props.on_close.call(()),
                role: "button",
                tabindex: "0",
                "aria-label": "Close modal"
            }

            // Modal content
            div {
                class: "relative bg-white rounded-3xl shadow-2xl w-full max-w-xl mx-auto transform transition-all duration-300 ease-out scale-100 max-h-[90vh] overflow-y-auto m-4 border border-gray-100",
                onclick: move |event| event.stop_propagation(),
                role: "dialog",
                "aria-modal": "true",
                "aria-labelledby": "modal-title",

                // Close button in top-right
                button {
                    class: "absolute top-6 right-6 text-gray-400 hover:text-gray-600 hover:bg-gray-100 transition-all duration-200 p-3 rounded-full z-10 hover:scale-110",
                    onclick: move |_| props.on_close.call(()),
                    "aria-label": "Close modal",
                    disabled: is_loading(),
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        width: "24",
                        height: "24",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        line { x1: "18", y1: "6", x2: "6", y2: "18" }
                        line { x1: "6", y1: "6", x2: "18", y2: "18" }
                    }
                }

                // Header
                div { class: "px-8 pt-8 pb-6",
                    h2 {
                        id: "modal-title",
                        class: "text-3xl font-bold text-gray-900 mb-2",
                        "Start a New Lesson"
                    }
                    p { class: "text-base text-gray-500", "Create a teaching session to start learning" }
                }

                // Form content
                div { class: "px-8 pb-8",
                    form {
                        class: "space-y-6",
                        onsubmit: move |event| {
                            event.prevent_default();
                            handle_create_session(());
                        },

                        // Error message
                        if !error_message.read().is_empty() {
                            div {
                                class: "bg-red-50 border border-red-200 text-red-700 px-5 py-4 rounded-xl text-sm",
                                role: "alert",
                                "{error_message}"
                            }
                        }

                        // Topic input
                        div {
                            label {
                                class: "block text-sm font-semibold text-gray-700 mb-3",
                                r#for: "topic",
                                "Lesson Topic"
                            }
                            input {
                                class: "w-full border border-gray-200 rounded-xl shadow-sm py-4 px-5 text-base focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500 transition-all duration-200 bg-gray-50 hover:bg-white focus:bg-white disabled:opacity-50 disabled:bg-gray-100",
                                id: "topic",
                                r#type: "text",
                                placeholder: "e.g., 'The Rust Borrow Checker'",
                                value: "{topic}",
                                oninput: move |event| topic.set(event.value()),
                                disabled: is_loading(),
                                required: true,
                                autofocus: true
                            }
                        }

                        // Material textarea
                        div {
                            label {
                                class: "block text-sm font-semibold text-gray-700 mb-3",
                                r#for: "material",
                                "Study Material"
                            }
                            textarea {
                                class: "w-full border border-gray-200 rounded-xl shadow-sm py-4 px-5 h-40 text-base focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500 transition-all duration-200 bg-gray-50 hover:bg-white focus:bg-white resize-none disabled:opacity-50 disabled:bg-gray-100",
                                id: "material",
                                placeholder: "Paste your study material here...",
                                value: "{material}",
                                oninput: move |event| material.set(event.value()),
                                disabled: is_loading(),
                                required: true
                            }
                        }

                        // Submit button
                        div { class: "flex justify-end space-x-4 pt-4",
                            button {
                                class: "px-6 py-3 text-gray-700 hover:text-gray-900 transition-colors duration-200 font-medium text-base",
                                r#type: "button",
                                onclick: move |_| props.on_close.call(()),
                                disabled: is_loading(),
                                "Cancel"
                            }
                            button {
                                class: "bg-indigo-600 text-white py-3 px-8 rounded-xl hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 transition-all duration-200 font-medium shadow-lg hover:shadow-xl disabled:opacity-50 disabled:cursor-not-allowed disabled:shadow-none flex items-center space-x-2 text-base",
                                r#type: "submit",
                                disabled: is_loading(),

                                if is_loading() {
                                    svg {
                                        class: "animate-spin h-4 w-4",
                                        xmlns: "http://www.w3.org/2000/svg",
                                        fill: "none",
                                        view_box: "0 0 24 24",
                                        circle {
                                            class: "opacity-25",
                                            cx: "12",
                                            cy: "12",
                                            r: "10",
                                            stroke: "currentColor",
                                            stroke_width: "4"
                                        }
                                        path {
                                            class: "opacity-75",
                                            fill: "currentColor",
                                            d: "M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                                        }
                                    }
                                }
                                "Create Session"
                            }
                        }
                    }
                }
            }
        }
    }
}
