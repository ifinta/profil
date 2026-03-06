use dioxus::prelude::*;
use crate::ui::state::{AppState, COUNTRY_KEYS, LANGUAGE_KEYS, CERTIFICATE_KEYS, COMPANY_KEYS, PROJECT_EXPERIENCE};
use crate::ui::i18n::UiI18n;
use crate::ui::pdf;

pub fn render_display_tab(s: AppState, i18n: &dyn UiI18n) -> Element {
    let skills = s.selected_skills.read().clone();
    let companies = s.selected_companies.read().clone();
    let main_chars = s.selected_main_chars.read().clone();
    let tools = s.selected_tools.read().clone();

    let has_anything = !skills.is_empty()
        || !companies.is_empty()
        || !main_chars.is_empty()
        || !tools.is_empty();

    if !has_anything {
        return rsx! {
            div { style: "text-align: center; margin-top: 60px; color: #888;",
                p { style: "font-size: 2.5em; margin-bottom: 10px;", "\u{1F4CB}" }
                p { style: "font-size: 0.95em;", "{i18n.display_nothing_selected()}" }
            }
        };
    }

    let pdf_label = i18n.btn_generate_pdf().to_string();
    let role_key = s.selected_role.read().key();
    let show_strengths = main_chars.contains(&"mc_strengths");
    let show_achievements = main_chars.contains(&"mc_achievements");
    let show_countries = main_chars.contains(&"mc_countries");
    let show_languages = main_chars.contains(&"mc_languages");
    let show_certificates = main_chars.contains(&"mc_certificates");

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

        // ── "Főbb jellemzőim" items ──
        if show_strengths {
            {render_strengths_section(i18n, role_key)}
        }
        if show_achievements {
            {render_achievements_section(i18n)}
        }
        if show_countries {
            {render_display_section(i18n, i18n.section_countries(), COUNTRY_KEYS, "#28a745")}
        }
        if show_languages {
            {render_display_section(i18n, i18n.section_languages(), LANGUAGE_KEYS, "#fd7e14")}
        }
        if show_certificates {
            {render_display_section(i18n, i18n.section_certificates(), CERTIFICATE_KEYS, "#6f42c1")}
        }

        if !skills.is_empty() {
            {render_display_section(i18n, i18n.section_skills(), &skills, "#667eea")}
        }
        if !companies.is_empty() {
            {render_display_section(i18n, i18n.section_companies(), &companies, "#17a2b8")}
        }
        if !tools.is_empty() {
            {render_display_section(i18n, i18n.section_tools(), &tools, "#20c997")}
        }

        // ── Project Experience (pages 4+ of the PDF, rendered as compact components) ──
        if !companies.is_empty() {
            {render_project_experience(i18n, &companies, &s.selected_job_roles.read(), &s.selected_projects.read(), &skills, &tools)}
        }
    }
}

fn render_strengths_section(i18n: &dyn UiI18n, role_key: &str) -> Element {
    let title = i18n.role_strengths_title().to_string();
    let items: Vec<(String, String)> = i18n
        .role_strengths(role_key)
        .iter()
        .map(|(t, d)| (t.to_string(), d.to_string()))
        .collect();

    rsx! {
        div { style: "margin-bottom: 18px; border-left: 4px solid #764ba2; padding: 12px 16px; background: #fafbfc; border-radius: 0 8px 8px 0;",
            h4 { style: "margin: 0 0 8px; color: #764ba2; font-size: 1em;", "{title}" }
            for (t, d) in items.iter() {
                div { style: "margin-bottom: 10px; padding-left: 10px; border-left: 2px solid #e9ecef;",
                    p { style: "margin: 0 0 2px; color: #333; font-weight: 600; font-size: 0.92em;", "{t}" }
                    p { style: "margin: 0; color: #666; font-size: 0.85em; line-height: 1.5;", "{d}" }
                }
            }
        }
    }
}

fn render_achievements_section(i18n: &dyn UiI18n) -> Element {
    let title = i18n.role_achievements_title().to_string();
    let items: Vec<(String, String)> = i18n
        .role_achievements()
        .iter()
        .map(|(t, d)| (t.to_string(), d.to_string()))
        .collect();

    rsx! {
        div { style: "margin-bottom: 18px; border-left: 4px solid #e83e8c; padding: 12px 16px; background: #fafbfc; border-radius: 0 8px 8px 0;",
            h4 { style: "margin: 0 0 8px; color: #e83e8c; font-size: 1em;", "{title}" }
            for (t, d) in items.iter() {
                div { style: "margin-bottom: 10px; padding-left: 10px; border-left: 2px solid #e9ecef;",
                    p { style: "margin: 0 0 2px; color: #333; font-weight: 600; font-size: 0.92em;", "{t}" }
                    p { style: "margin: 0; color: #666; font-size: 0.85em; line-height: 1.5;", "{d}" }
                }
            }
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

/// Render project experience as compact components grouped by company.
/// Each company is a card with a header and a table of project rows.
fn render_project_experience(
    i18n: &dyn UiI18n,
    selected_companies: &[&str],
    selected_job_roles: &[&str],
    selected_projects: &[&str],
    selected_skills: &[&str],
    selected_tools: &[&str],
) -> Element {
    // Collect companies that have matching projects, preserving COMPANY_KEYS order
    let company_order: Vec<&str> = COMPANY_KEYS
        .iter()
        .filter(|c| selected_companies.contains(c))
        .copied()
        .collect();

    // Build filtered entries per company
    let mut company_entries: Vec<(&str, Vec<ProjectRow>)> = Vec::new();

    for &company_key in &company_order {
        let rows: Vec<ProjectRow> = PROJECT_EXPERIENCE
            .iter()
            .filter(|e| e.company_key == company_key)
            .filter(|e| {
                // If specific projects selected, filter by them
                if !selected_projects.is_empty() {
                    if !selected_projects.contains(&e.project_key) {
                        return false;
                    }
                }
                // If specific job roles selected, require at least one match
                if !selected_job_roles.is_empty() {
                    if !e.job_role_keys.iter().any(|r| selected_job_roles.contains(r)) {
                        return false;
                    }
                }
                // If specific skills selected, require at least one match
                if !selected_skills.is_empty() {
                    if !e.skill_keys.iter().any(|s| selected_skills.contains(s)) {
                        return false;
                    }
                }
                // If specific tools selected, require at least one match
                if !selected_tools.is_empty() {
                    if !e.tool_keys.iter().any(|t| selected_tools.contains(t)) {
                        return false;
                    }
                }
                true
            })
            .map(|e| ProjectRow {
                date_interval: e.date_interval.to_string(),
                project: i18n.item_label(e.project_key).to_string(),
                job_roles: e.job_role_keys.iter().map(|k| i18n.item_label(k).to_string()).collect::<Vec<_>>().join(", "),
                skills: e.skill_keys.iter().map(|k| i18n.item_label(k).to_string()).collect::<Vec<_>>().join(", "),
                tools: e.tool_keys.iter().map(|k| i18n.item_label(k).to_string()).collect::<Vec<_>>().join(", "),
            })
            .collect();

        if !rows.is_empty() {
            company_entries.push((company_key, rows));
        }
    }

    if company_entries.is_empty() {
        return rsx! {};
    }

    let section_title = i18n.section_project_experience().to_string();

    // Collect owned data so it lives long enough for the RSX
    let entries: Vec<(String, Vec<ProjectRow>)> = company_entries
        .into_iter()
        .map(|(ck, rows)| (i18n.item_label(ck).to_string(), rows))
        .collect();

    rsx! {
        div { style: "margin-top: 24px; margin-bottom: 18px;",
            h3 { style: "margin: 0 0 14px; color: #333; font-size: 1.1em; border-bottom: 2px solid #667eea; padding-bottom: 6px;",
                "{section_title}"
            }
            for (company_label, rows) in entries.iter() {
                div { style: "margin-bottom: 16px; background: #f8f9fa; border-radius: 10px; padding: 14px; border: 1px solid #e9ecef;",
                    // Company header
                    h4 { style: "margin: 0 0 10px; color: #667eea; font-size: 1em; font-weight: 700;",
                        "{company_label}"
                    }
                    // Project rows
                    for row in rows.iter() {
                        div { style: "padding: 8px 0; border-bottom: 1px solid #e9ecef;",
                            // Date interval
                            div { style: "display: flex; flex-wrap: wrap; gap: 6px; align-items: baseline;",
                                span { style: "font-size: 0.82em; color: #764ba2; font-weight: 600; min-width: 130px;",
                                    "{row.date_interval}"
                                }
                                span { style: "font-size: 0.92em; color: #333; font-weight: 600;",
                                    "{row.project}"
                                }
                            }
                            // Job roles
                            if !row.job_roles.is_empty() {
                                div { style: "margin-top: 3px;",
                                    span { style: "font-size: 0.78em; color: #888; font-weight: 600;", "\u{1F464} " }
                                    span { style: "font-size: 0.8em; color: #555;", "{row.job_roles}" }
                                }
                            }
                            // Skills
                            if !row.skills.is_empty() {
                                div { style: "margin-top: 2px;",
                                    span { style: "font-size: 0.78em; color: #888; font-weight: 600;", "\u{2699} " }
                                    span { style: "font-size: 0.8em; color: #555;", "{row.skills}" }
                                }
                            }
                            // Tools
                            if !row.tools.is_empty() {
                                div { style: "margin-top: 2px;",
                                    span { style: "font-size: 0.78em; color: #888; font-weight: 600;", "\u{1F527} " }
                                    span { style: "font-size: 0.8em; color: #555;", "{row.tools}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

struct ProjectRow {
    date_interval: String,
    project: String,
    job_roles: String,
    skills: String,
    tools: String,
}
