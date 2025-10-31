use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct SessionItemProps {
    pub title: String,
    pub last_updated: String,
    #[props(default = false)]
    pub is_active: bool,
}
