#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct NewLessonModalProps {
    pub on_close: EventHandler<()>,
}

pub fn NewLessonModal(props: NewLessonModalProps) -> Element {
    let mut topic = use_signal(String::new);
    let mut material = use_signal(String::new);

    let handle_create_session = move |_| {
        tracing::info!("Creating new session:");
        tracing::info!("Topic: {}", topic());
        tracing::info!("Material: {}", material());
        // TODO: Call API to create session
        // TODO: Navigate to the new session page

        // Close the modal on creation
        props.on_close.call(());
    };

    rsx! {
        div {
            class: "fixed inset-0 z-50 bg-black/60 backdrop-blur-sm flex justify-center items-center p-4",
            onclick: move |_| props.on_close.call(()),

            div {
                class: "bg-white rounded-2xl shadow-2xl w-full max-w-lg mx-auto transform transition-all duration-200 scale-100 hover:scale-105 p-8",
                // stop click events from bubbling up to the overlay and closing the modal
                onclick: move |event| event.stop_propagation(),

                // header
                div { class: "flex justify-between items-center mb-6",
                    h2 { class: "text-2xl font-bold text-gray-900", "Start a New Lesson" }
                    button {
                        class: "text-gray-400 hover:text-gray-600 transition-colors duration-200 p-1 rounded-lg hover:bg-gray-100",
                        onclick: move |_| props.on_close.call(()),
                        // close (X) icon
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
                }

                // Form
                form {
                    class: "space-y-6",
                    onsubmit: move |event| {
                        event.prevent_default();
                        handle_create_session(());
                    },

                    // Topic Input
                    div {
                        label { class: "block text-sm font-semibold text-gray-700 mb-2", r#for: "topic", "Lesson Topic" }
                        input {
                            class: "w-full border border-gray-200 rounded-xl shadow-sm py-3 px-4 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500 transition-all duration-200 bg-gray-50 hover:bg-white focus:bg-white",
                            id: "topic",
                            r#type: "text",
                            placeholder: "e.g., 'The Rust Borrow Checker'",
                            oninput: move |event| topic.set(event.value())
                        }
                    }

                    // Material Text Area
                    div {
                        label { class: "block text-sm font-semibold text-gray-700 mb-2", r#for: "material", "Study Material" }
                        textarea {
                            class: "w-full border border-gray-200 rounded-xl shadow-sm py-3 px-4 h-48 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500 transition-all duration-200 bg-gray-50 hover:bg-white focus:bg-white resize-none",
                            id: "material",
                            placeholder: "Paste your study material here...",
                            oninput: move |event| material.set(event.value())
                        }
                    }

                    // Submit Button
                    div { class: "flex justify-end pt-4",
                        button {
                            class: "bg-indigo-600 text-white py-3 px-6 rounded-xl hover:from-indigo-700 hover:to-purple-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 transition-all duration-200 font-medium shadow-lg hover:shadow-xl hover:-translate-y-0.5",
                            r#type: "submit",
                            "Create Session"
                        }
                    }
                }
            }
        }
    }
}
