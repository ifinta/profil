pub mod i18n;
pub mod state;
pub mod tabs;
pub mod view;

use dioxus::prelude::*;
use state::use_app_state;

pub fn app() -> Element {
    let state = use_app_state();
    rsx! { {view::render_app(state)} }
}
