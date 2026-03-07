use dioxus::prelude::*;
use super::state::AppState;
use super::i18n::ui_i18n;
use super::tabs::Tab;
use super::tabs::{profile, filter, display};

pub fn render_app(s: AppState) -> Element {
    let lang = *s.language.read();
    let i18n = ui_i18n(lang);
    let active = *s.active_tab.read();

    rsx! {
        div { style: "display: flex; flex-direction: column; height: 100vh; max-width: 550px; margin: auto; font-family: sans-serif;",
            // Header
            div { style: "padding: 15px 30px 0;",
                h2 { style: "margin: 0; color: #333;", "{i18n.profile_name()}" }
            }

            // Tab content (scrollable area)
            div { style: "flex: 1; overflow-y: auto; padding: 20px 30px 90px;",
                match active {
                    Tab::Profile => profile::render_profile_tab(s, i18n.as_ref()),
                    Tab::Display => display::render_display_tab(s, i18n.as_ref()),
                    Tab::Filter => filter::render_filter_tab(s, i18n.as_ref()),
                }
            }

            // Bottom tab bar
            {render_tab_bar(s, i18n.as_ref())}
        }
    }
}

fn render_tab_bar(s: AppState, i18n: &dyn super::i18n::UiI18n) -> Element {
    let active = *s.active_tab.read();

    // Profile icon (person), Display icon (list), Filter icon (funnel)
    let tabs: [(Tab, &str, &str); 3] = [
        (Tab::Profile, "M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z", i18n.tab_profile()),
        (Tab::Display, "M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-3 7h3m-3 4h3m-6-4h.01m-.01 4h.01", i18n.tab_display()),
        (Tab::Filter, "M3 4a1 1 0 011-1h16a1 1 0 011 1v2.586a1 1 0 01-.293.707l-6.414 6.414a1 1 0 00-.293.707V17l-4 4v-6.586a1 1 0 00-.293-.707L3.293 7.293A1 1 0 013 6.586V4z", i18n.tab_filter()),
    ];

    rsx! {
        div { style: "position: fixed; bottom: 0; left: 0; right: 0; max-width: 550px; margin: auto; background: white; border-top: 1px solid #ddd; display: flex; justify-content: space-around; padding: 6px 0; z-index: 500;",
            for (tab, path, label) in tabs {
                {
                    let is_active = active == tab;
                    let color = if is_active { "#667eea" } else { "#999" };
                    let font_weight = if is_active { "bold" } else { "normal" };
                    rsx! {
                        button {
                            key: "{label}",
                            style: "flex: 1; display: flex; flex-direction: column; align-items: center; gap: 2px; background: none; border: none; cursor: pointer; padding: 4px 0; color: {color};",
                            onclick: move |_| {
                                let mut active_tab = s.active_tab;
                                active_tab.set(tab);
                            },
                            svg {
                                width: "24",
                                height: "24",
                                view_box: "0 0 24 24",
                                fill: "none",
                                stroke: "{color}",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path { d: "{path}" }
                            }
                            span { style: "font-size: 0.65em; font-weight: {font_weight};",
                                "{label}"
                            }
                        }
                    }
                }
            }
        }
    }
}
