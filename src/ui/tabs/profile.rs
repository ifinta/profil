use dioxus::prelude::*;
use crate::i18n::Language;
use crate::ui::state::{AppState, Role};
use crate::ui::i18n::UiI18n;

pub fn render_profile_tab(mut s: AppState, i18n: &dyn UiI18n) -> Element {
    let lang = *s.language.read();
    let lang_value = match lang {
        Language::English => "en",
        Language::Hungarian => "hu",
        Language::German => "de",
        Language::French => "fr",
        Language::Finnish => "fi",
        Language::Spanish => "es",
        Language::Greek => "el",
        Language::Italian => "it",
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
                        "fr" => Language::French,
                        "fi" => Language::Finnish,
                        "es" => Language::Spanish,
                        "el" => Language::Greek,
                        "it" => Language::Italian,
                        _ => Language::English,
                    };
                    let mut language = s.language;
                    language.set(lang);
                },
                option { value: "en", selected: lang == Language::English, "English" }
                option { value: "hu", selected: lang == Language::Hungarian, "Magyar" }
                option { value: "de", selected: lang == Language::German, "Deutsch" }
                option { value: "fr", selected: lang == Language::French, "Français" }
                option { value: "fi", selected: lang == Language::Finnish, "Suomi" }
                option { value: "es", selected: lang == Language::Spanish, "Español" }
                option { value: "el", selected: lang == Language::Greek, "Ελληνικά" }
                option { value: "it", selected: lang == Language::Italian, "Italiano" }
            }
        }

        div { style: "text-align: center;",
            // Profile photo (round)
            img {
                src: "photo.png",
                alt: "Istvan Finta",
                style: "width: 120px; height: 120px; border-radius: 50%; object-fit: cover; margin: 0 auto 16px; display: block; box-shadow: 0 4px 15px rgba(102,126,234,0.4); border: 3px solid #667eea;",
            }
        }

        // Role selector
        div { style: "margin-bottom: 16px;",
            select {
                style: "width: 100%; padding: 10px 14px; border: 2px solid #764ba2; border-radius: 8px; font-size: 1em; color: #333; background: white; cursor: pointer; font-weight: 600;",
                value: "{role_key}",
                onchange: move |evt| {
                    let r = Role::from_key(&evt.value());
                    let mut selected_role = s.selected_role;
                    selected_role.set(r);

                    // Clear all checkbox selections
                    s.selected_skills.set(Vec::new());
                    s.selected_companies.set(Vec::new());
                    s.selected_job_roles.set(Vec::new());
                    s.selected_projects.set(Vec::new());
                    s.selected_main_chars.set(Vec::new());
                    s.selected_tools.set(Vec::new());

                    // Auto-select company based on role
                    match r {
                        Role::SoftwareEngineer => s.selected_companies.set(vec!["vilati", "porsche"]),
                        Role::ProjectOwner => s.selected_companies.set(vec!["bitnok"]),
                        Role::TestManager => s.selected_companies.set(vec!["bosch"]),
                    };

                    // Auto-select main characteristics: strengths + languages
                    s.selected_main_chars.set(vec!["mc_strengths", "mc_languages"]);
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
