use dioxus::prelude::*;
use crate::ui::state::{
    AppState, SKILL_KEYS, COUNTRY_KEYS, LANGUAGE_KEYS, COMPANY_KEYS, CERTIFICATE_KEYS,
};
use crate::ui::i18n::UiI18n;

pub fn render_filter_tab(s: AppState, i18n: &dyn UiI18n) -> Element {
    rsx! {
        // Skills section
        {render_section(s, i18n, i18n.section_skills(), SKILL_KEYS, SectionKind::Skills)}

        // Countries section
        {render_section(s, i18n, i18n.section_countries(), COUNTRY_KEYS, SectionKind::Countries)}

        // Language Skills section
        {render_section(s, i18n, i18n.section_languages(), LANGUAGE_KEYS, SectionKind::Languages)}

        // Companies section
        {render_section(s, i18n, i18n.section_companies(), COMPANY_KEYS, SectionKind::Companies)}

        // Certificates section
        {render_section(s, i18n, i18n.section_certificates(), CERTIFICATE_KEYS, SectionKind::Certificates)}
    }
}

#[derive(Clone, Copy)]
enum SectionKind {
    Skills,
    Countries,
    Languages,
    Companies,
    Certificates,
}

fn get_signal(s: AppState, kind: SectionKind) -> Signal<Vec<&'static str>> {
    match kind {
        SectionKind::Skills => s.selected_skills,
        SectionKind::Countries => s.selected_countries,
        SectionKind::Languages => s.selected_languages,
        SectionKind::Companies => s.selected_companies,
        SectionKind::Certificates => s.selected_certificates,
    }
}

fn render_section(
    s: AppState,
    i18n: &dyn UiI18n,
    section_title: &str,
    keys: &'static [&'static str],
    kind: SectionKind,
) -> Element {
    let signal = get_signal(s, kind);
    let selected = signal.read();
    let all_selected = keys.iter().all(|k| selected.contains(k));

    let title = section_title.to_string();
    let select_all_label = i18n.filter_select_all().to_string();
    let clear_all_label = i18n.filter_clear_all().to_string();

    rsx! {
        div { style: "margin-bottom: 18px; background: #f8f9fa; border-radius: 10px; padding: 14px; border: 1px solid #e9ecef;",
            // Section header with select/clear all
            div { style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 10px;",
                h4 { style: "margin: 0; color: #333; font-size: 1em;", "{title}" }
                button {
                    style: "padding: 4px 12px; font-size: 0.78em; border: 1px solid #667eea; border-radius: 4px; background: white; color: #667eea; cursor: pointer; font-weight: 600;",
                    onclick: move |_| {
                        let mut sig = signal;
                        if all_selected {
                            sig.set(Vec::new());
                        } else {
                            sig.set(keys.to_vec());
                        }
                    },
                    if all_selected { "{clear_all_label}" } else { "{select_all_label}" }
                }
            }

            // Checkboxes
            for key in keys.iter() {
                {render_checkbox_item(s, i18n, key, kind)}
            }
        }
    }
}

fn render_checkbox_item(
    s: AppState,
    i18n: &dyn UiI18n,
    key: &'static str,
    kind: SectionKind,
) -> Element {
    let signal = get_signal(s, kind);
    let is_checked = signal.read().contains(&key);
    let label = i18n.item_label(key).to_string();
    let hint = i18n.item_hint(key).to_string();

    rsx! {
        label { style: "display: flex; align-items: flex-start; gap: 8px; padding: 6px 0; cursor: pointer;",
            input {
                r#type: "checkbox",
                checked: is_checked,
                style: "margin-top: 3px; accent-color: #667eea;",
                onchange: move |_| {
                    let mut sig = signal;
                    let mut current = sig.read().clone();
                    if current.contains(&key) {
                        current.retain(|&k| k != key);
                    } else {
                        current.push(key);
                    }
                    sig.set(current);
                },
            }
            div {
                span { style: "font-size: 0.92em; color: #333; font-weight: 500;", "{label}" }
                if !hint.is_empty() {
                    p { style: "margin: 2px 0 0; font-size: 0.78em; color: #888; line-height: 1.3;",
                        "{hint}"
                    }
                }
            }
        }
    }
}
