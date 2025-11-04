use dioxus::prelude::*;

#[derive(PartialEq, Clone, Copy)]
pub enum MessageRole {
    User,
    Assistant,
}

#[derive(Props, PartialEq, Clone)]
pub struct MessageBubbleProps {
    pub text: String,
    pub role: MessageRole,
}
