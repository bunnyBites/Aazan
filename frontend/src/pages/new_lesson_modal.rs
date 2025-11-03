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
            class: "fixed inset-0 z-50 bg-black/50 flex justify-center items-center",
            onclick: move |_| props.on_close.call(()),

            div {
                class: "bg-white rounded-lg shadow-xl w-full max-w-lg p-6",
                // stop click events from bubbling up to the overlay and closing the modal
                onclick: move |event| event.stop_propagation(),

                // header
                div { class: "flex justify-between items-center border-b pb-3",
                    h2 { class: "text-2xl font-bold text-gray-800", "Start a New Lesson" }
                    button {
                        class: "text-gray-400 hover:text-gray-600",
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
                    class: "mt-4 space-y-4",
                    onsubmit: move |event| {
                        event.prevent_default();
                        handle_create_session(());
                    },

                    // Topic Input
                    div {
                        label { class: "block text-sm font-medium text-gray-700", r#for: "topic", "Lesson Topic" }
                        input {
                            class: "mt-1 w-full border border-gray-300 rounded-md shadow-sm py-2 px-3 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500",
                            id: "topic",
                            r#type: "text",
                            placeholder: "e.g., 'The Rust Borrow Checker'",
                            oninput: move |event| topic.set(event.value())
                        }
                    }

                    // Material Text Area
                    div {
                        label { class: "block text-sm font-medium text-gray-700", r#for: "material", "Study Material" }
                        textarea {
                            class: "mt-1 w-full border border-gray-300 rounded-md shadow-sm py-2 px-3 h-48 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500",
                            id: "material",
                            placeholder: "Paste your study material here...",
                            oninput: move |event| material.set(event.value())
                        }
                    }

                    // Submit Button
                    div { class: "flex justify-end pt-4",
                        button {
                            class: "bg-indigo-600 text-white py-2 px-4 rounded-lg hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2",
                            r#type: "submit",
                            "Create Session"
                        }
                    }
                }
            }
        }
    }
}
