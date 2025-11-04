use dioxus::signals::Signal;

#[derive(Clone, Copy)]
pub struct MobileMenuOpen {
    pub is_open: Signal<bool>,
}

#[derive(Clone, Copy)]
pub struct NewLessonModalOpen {
    pub is_open: Signal<bool>,
}
