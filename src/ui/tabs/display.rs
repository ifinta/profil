use dioxus::prelude::*;
use crate::ui::state::AppState;
use crate::ui::i18n::UiI18n;
use crate::ui::pdf;

pub fn render_display_tab(s: AppState, i18n: &dyn UiI18n) -> Element {
    let skills = s.selected_skills.read().clone();
    let countries = s.selected_countries.read().clone();
    let languages = s.selected_languages.read().clone();
    let companies = s.selected_companies.read().clone();
    let certificates = s.selected_certificates.read().clone();

    let has_anything = !skills.is_empty()
        || !countries.is_empty()
        || !languages.is_empty()
        || !companies.is_empty()
        || !certificates.is_empty();

    if !has_anything {
        return rsx! {
            div { style: "text-align: center; margin-top: 60px; color: #888;",
                p { style: "font-size: 2.5em; margin-bottom: 10px;", "\u{1F4CB}" }
                p { style: "font-size: 0.95em;", "{i18n.display_nothing_selected()}" }
            }
        };
    }

    let pdf_label = i18n.btn_generate_pdf().to_string();

    rsx! {
        // Generate PDF button
        div { style: "text-align: right; margin-bottom: 14px;",
            button {
                style: "padding: 8px 20px; font-size: 0.9em; font-weight: 600; color: white; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); border: none; border-radius: 8px; cursor: pointer; box-shadow: 0 2px 8px rgba(102,126,234,0.3);",
                onclick: move |_| {
                    pdf::generate_pdf(&s, crate::ui::i18n::ui_i18n(*s.language.read()).as_ref());
                },
                "\u{1F4C4} {pdf_label}"
            }
        }

        if !skills.is_empty() {
            {render_display_section(i18n, i18n.section_skills(), &skills, "#667eea")}
        }
        if !countries.is_empty() {
            {render_display_section(i18n, i18n.section_countries(), &countries, "#28a745")}
        }
        if !languages.is_empty() {
            {render_display_section(i18n, i18n.section_languages(), &languages, "#fd7e14")}
        }
        if !companies.is_empty() {
            {render_display_section(i18n, i18n.section_companies(), &companies, "#17a2b8")}
        }
        if !certificates.is_empty() {
            {render_display_section(i18n, i18n.section_certificates(), &certificates, "#6f42c1")}
        }
    }
}

fn render_display_section(
    i18n: &dyn UiI18n,
    title: &str,
    keys: &[&str],
    accent: &str,
) -> Element {
    let title = title.to_string();
    let accent = accent.to_string();
    let border_style = format!(
        "margin-bottom: 18px; border-left: 4px solid {}; padding: 12px 16px; background: #fafbfc; border-radius: 0 8px 8px 0;",
        accent
    );
    let title_style = format!("margin: 0 0 8px; color: {}; font-size: 1em;", accent);

    // Collect items into owned strings so they live long enough
    let items: Vec<(String, String)> = keys
        .iter()
        .map(|k| (i18n.item_label(k).to_string(), i18n.item_hint(k).to_string()))
        .collect();

    rsx! {
        div { style: "{border_style}",
            h4 { style: "{title_style}", "{title}" }
            for (label, hint) in items.iter() {
                div { style: "padding: 6px 0; border-bottom: 1px solid #eee;",
                    span { style: "font-size: 0.92em; color: #333; font-weight: 600;",
                        "{label}"
                    }
                    if !hint.is_empty() {
                        span { style: "font-size: 0.8em; color: #888; margin-left: 8px;",
                            "— {hint}"
                        }
                    }
                }
            }
        }
    }
}
