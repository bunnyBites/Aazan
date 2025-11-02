use dioxus::signals::Signal;

#[derive(Clone, Copy)]
pub struct MobileMenuOpen {
    pub is_open: Signal<bool>,
}
