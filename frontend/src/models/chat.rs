use dioxus::prelude::*;
use uuid::Uuid;

#[derive(Props, PartialEq, Clone)]
pub struct ChatProps {
    pub id: Uuid,
}
