#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct MicrophoneButtonProps {
    pub on_click: EventHandler<bool>,
}

pub fn MicrophoneButton(props: MicrophoneButtonProps) -> Element {
    // signal will control the button's appearance and behavior.
    let mut is_recording = use_signal(|| false);

    let (button_class, icon_color) = if is_recording() {
        // recording state styles from the design system
        ("bg-red-500 animate-pulse", "white")
    } else {
        // idle state styles
        ("bg-indigo-600", "white")
    };

    rsx! {
        button {
            class: "w-16 h-16 rounded-full flex items-center justify-center text-white shadow-lg transition-colors {button_class} hover:bg-indigo-700",
            // onclick handler toggles the recording state
            onclick: move |_| {
                let new_state = !is_recording();
                is_recording.set(new_state);
                // notify the parent component of the change
                props.on_click.call(new_state);
            },
            // icon for the microphone
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                width: "28",
                height: "28",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "{icon_color}",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path {
                    d: "M12 1a3 3 0 0 0-3 3v8a3 3 0 0 0 6 0V4a3 3 0 0 0-3-3z"
                }
                path {
                    d: "M19 10v2a7 7 0 0 1-14 0v-2"
                }
                line {
                    x1: "12",
                    y1: "19",
                    x2: "12",
                    y2: "22"
                }
            }
        }
    }
}
