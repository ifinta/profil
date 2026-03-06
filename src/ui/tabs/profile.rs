use dioxus::prelude::*;
use crate::i18n::Language;
use crate::ui::state::{AppState, Role};
use crate::ui::i18n::UiI18n;

pub fn render_profile_tab(s: AppState, i18n: &dyn UiI18n) -> Element {
    let lang = *s.language.read();
    let lang_value = match lang {
        Language::English => "en",
        Language::Hungarian => "hu",
        Language::German => "de",
    };
    let role = *s.selected_role.read();
    let role_key = role.key();

    rsx! {
        // Language selector
        div { style: "margin-bottom: 16px;",
            select {
                style: "width: 100%; padding: 10px 14px; border: 2px solid #667eea; border-radius: 8px; font-size: 1em; color: #333; background: white; cursor: pointer; font-weight: 600;",
                value: "{lang_value}",
                onchange: move |evt| {
                    let lang = match evt.value().as_str() {
                        "hu" => Language::Hungarian,
                        "de" => Language::German,
                        _ => Language::English,
                    };
                    let mut language = s.language;
                    language.set(lang);
                },
                option { value: "en", selected: lang == Language::English, "English" }
                option { value: "hu", selected: lang == Language::Hungarian, "Magyar" }
                option { value: "de", selected: lang == Language::German, "Deutsch" }
            }
        }

        // Role selector
        div { style: "margin-bottom: 16px;",
            label { style: "display: block; font-size: 0.85em; color: #888; margin-bottom: 4px; font-weight: 600;",
                "{i18n.role_section_label()}"
            }
            select {
                style: "width: 100%; padding: 10px 14px; border: 2px solid #764ba2; border-radius: 8px; font-size: 1em; color: #333; background: white; cursor: pointer; font-weight: 600;",
                value: "{role_key}",
                onchange: move |evt| {
                    let r = Role::from_key(&evt.value());
                    let mut selected_role = s.selected_role;
                    selected_role.set(r);
                },
                option { value: "szm", selected: role == Role::SoftwareEngineer,
                    "{i18n.role_label(\"szm\")}"
                }
                option { value: "po", selected: role == Role::ProjectOwner,
                    "{i18n.role_label(\"po\")}"
                }
                option { value: "tm", selected: role == Role::TestManager,
                    "{i18n.role_label(\"tm\")}"
                }
            }
        }

        div { style: "text-align: center;",
            // Avatar placeholder (circle with initials)
            div { style: "width: 120px; height: 120px; border-radius: 50%; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); display: flex; align-items: center; justify-content: center; margin: 0 auto 16px; box-shadow: 0 4px 15px rgba(102,126,234,0.4);",
                span { style: "font-size: 2.5em; color: white; font-weight: bold;", "IF" }
            }

            p { style: "margin: 0 0 4px; color: #667eea; font-weight: 600; font-size: 1.05em;",
                "{i18n.role_title(role_key)}"
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

        // Target section
        div { style: "text-align: left; padding: 16px; background: white; border-radius: 10px; border: 1px solid #e9ecef; margin-bottom: 16px;",
            h3 { style: "margin: 0 0 10px; color: #333; font-size: 1.1em;",
                "{i18n.role_target_title()}"
            }
            p { style: "margin: 0; color: #555; font-size: 0.9em; line-height: 1.6;",
                "{i18n.role_target_text(role_key)}"
            }
        }

    }
}
