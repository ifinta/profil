pub mod i18n;
pub mod pdf;
pub mod state;
pub mod tabs;
pub mod view;

use dioxus::prelude::*;
use state::{use_app_state, persist};

pub fn app() -> Element {
    let state = use_app_state();

    // Auto-save every signal change to localStorage.
    // Reading each signal inside the effect subscribes to it,
    // so the effect re-runs whenever any of them changes.
    use_effect(move || {
        persist(&state);
    });

    rsx! { {view::render_app(state)} }
}
