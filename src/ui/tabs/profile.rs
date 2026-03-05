use dioxus::prelude::*;
use crate::ui::i18n::UiI18n;

pub fn render_profile_tab(i18n: &dyn UiI18n) -> Element {
    rsx! {
        div { style: "text-align: center; margin-top: 20px;",
            // Avatar placeholder (circle with initials)
            div { style: "width: 120px; height: 120px; border-radius: 50%; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); display: flex; align-items: center; justify-content: center; margin: 0 auto 20px; box-shadow: 0 4px 15px rgba(102,126,234,0.4);",
                span { style: "font-size: 2.5em; color: white; font-weight: bold;", "IF" }
            }

            h2 { style: "margin: 0 0 4px; color: #333; font-size: 1.5em;",
                "{i18n.profile_name()}"
            }
            p { style: "margin: 0 0 4px; color: #667eea; font-weight: 600; font-size: 1.05em;",
                "{i18n.profile_title()}"
            }
            p { style: "margin: 0 0 20px; color: #888; font-size: 0.9em;",
                "{i18n.profile_location()}"
            }
        }

        // Contact info
        div { style: "background: #f8f9fa; padding: 16px; border-radius: 10px; margin-bottom: 20px; border: 1px solid #e9ecef;",
            div { style: "display: flex; align-items: center; gap: 10px; margin-bottom: 10px;",
                span { style: "font-size: 1.2em;", "\u{2709}" }
                a { style: "color: #667eea; text-decoration: none; font-size: 0.95em;",
                    href: "mailto:{i18n.profile_email()}",
                    "{i18n.profile_email()}"
                }
            }
            div { style: "display: flex; align-items: center; gap: 10px;",
                span { style: "font-size: 1.2em;", "\u{260E}" }
                span { style: "color: #555; font-size: 0.95em;",
                    "{i18n.profile_phone()}"
                }
            }
        }

        // About section
        div { style: "text-align: left; padding: 16px; background: white; border-radius: 10px; border: 1px solid #e9ecef;",
            h3 { style: "margin: 0 0 10px; color: #333; font-size: 1.1em;",
                "{i18n.profile_about()}"
            }
            p { style: "margin: 0; color: #555; font-size: 0.9em; line-height: 1.6;",
                "{i18n.profile_about_text()}"
            }
        }
    }
}
