#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::{
    components::session_item::SessionItem, controllers::api::list_sessions,
    models::main::NewLessonModalOpen, Route,
};

#[derive(Props, PartialEq, Clone)]
pub struct SidebarProps {
    pub on_close_menu: EventHandler<()>,
}

pub fn Sidebar(props: SidebarProps) -> Element {
    let sessions = use_resource(list_sessions);
    let route = use_route::<Route>();
    let mut new_lesson_modal = use_context::<NewLessonModalOpen>();

    let is_session_active = move |session_id: uuid::Uuid| {
        if let Route::Chat { id: active_id } = route {
            let is_active = session_id == active_id;

            is_active
        } else {
            false
        }
    };

    rsx! {
        nav { class: "w-80 h-full bg-white border-r border-gray-200 flex flex-col shadow-lg",

          // header
          div { class: "p-4 border-b border-gray-200",
                h2 { class: "text-xl font-bold text-indigo-600", "Aazan" }
                p { class: "text-sm text-gray-500", "Your Teaching Sessions" }
            }

            // list of sessions
            div { class: "flex-1 overflow-y-auto",
                match &*sessions.read() {
                  Some(Ok(session_list)) => rsx! {
                      for session in session_list {
                        SessionItem {
                            key: "{session.id}",
                            id: session.id,
                            title: session.topic.clone(),
                            last_updated: session.updated_at.format("%Y-%m-%d").to_string(),
                            is_active: is_session_active(session.id),
                            on_click: move |_| props.on_close_menu.call(()),
                        }
                      }
                    },
                    Some(Err(e)) => rsx! {
                        div { class: "p-4 text-red-600", "Error: {e}" }
                    },
                    None => rsx! {
                        div { class: "p-4 text-gray-500", "Loading sessions..." }
                    }
                }
            }

          // footer
          footer { class: "p-4 border-t border-gray-200",
              button {
                  class: "w-full bg-indigo-600 text-white py-2 px-5 rounded-lg hover:bg-indigo-700",
                  onclick: move |_| {
                    new_lesson_modal.is_open.set(true);
                  },
                  "New Lesson"
              }
          }
        }
    }
}
