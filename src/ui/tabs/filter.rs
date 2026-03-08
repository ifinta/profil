use dioxus::prelude::*;
use qrcode::QrCode;
use qrcode::render::svg;
use crate::ui::state::{
    AppState, SKILL_KEYS, COMPANY_KEYS,
    JOB_ROLE_KEYS, PROJECT_KEYS, MAIN_CHARS_KEYS, TOOL_KEYS,
    available_keys_for_companies,
};
use crate::ui::i18n::UiI18n;

const APP_URL: &str = "https://ifinta.github.io/profil/";

fn render_qr_section(i18n: &dyn UiI18n) -> Element {
    let qr = QrCode::new(APP_URL.as_bytes()).unwrap();
    let svg_string = qr.render::<svg::Color>()
        .min_dimensions(180, 180)
        .max_dimensions(220, 220)
        .quiet_zone(true)
        .build();
    let hint = i18n.pwa_hint().to_string();

    rsx! {
        div { style: "margin-bottom: 18px; background: #f8f9fa; border-radius: 10px; padding: 18px; border: 1px solid #e9ecef; text-align: center;",
            // static version string
            p { style: "margin: 0; color: #888; font-size: 0.82em; line-height: 1.5;",
                "CACHE_NAME" // it will be replaced through build.sh to the version string
            }
            // QR code
            div { style: "display: inline-block; background: white; padding: 10px; border-radius: 8px; margin-bottom: 12px;",
                div { dangerous_inner_html: "{svg_string}" }
            }
            // Clickable link
            div { style: "margin-bottom: 14px;",
                a {
                    href: "{APP_URL}",
                    target: "_blank",
                    style: "color: #667eea; font-size: 0.95em; font-weight: 600; text-decoration: underline;",
                    "{APP_URL}"
                }
            }
            // PWA install hint
            p { style: "margin: 0; color: #888; font-size: 0.82em; line-height: 1.5; font-style: italic;",
                "{hint}"
            }
        }
    }
}

pub fn render_filter_tab(s: AppState, i18n: &dyn UiI18n) -> Element {
    // Compute which items are reachable from the selected companies
    let selected_companies = s.selected_companies.read();
    let available = available_keys_for_companies(&selected_companies);
    drop(selected_companies);

    // Companies: reversed (newest first)
    let mut companies_display: Vec<&'static str> = COMPANY_KEYS.to_vec();
    companies_display.reverse();

    // Projects: reversed (newest first)
    let mut projects_display: Vec<&'static str> = PROJECT_KEYS.to_vec();
    projects_display.reverse();

    // Job Roles: alphabetical by label
    let mut job_roles_display: Vec<&'static str> = JOB_ROLE_KEYS.to_vec();
    job_roles_display.sort_by_key(|k| i18n.item_label(k));

    // Technical Skills: alphabetical by label
    let mut skills_display: Vec<&'static str> = SKILL_KEYS.to_vec();
    skills_display.sort_by_key(|k| i18n.item_label(k));

    // Tools: alphabetical by label
    let mut tools_display: Vec<&'static str> = TOOL_KEYS.to_vec();
    tools_display.sort_by_key(|k| i18n.item_label(k));

    rsx! {
        // QR code, link, and PWA install hint
        {render_qr_section(i18n)}

        // "Főbb jellemzőim" (My Main Characteristics) group — not company-dependent
        {render_section(s, i18n, i18n.section_main_chars(), MAIN_CHARS_KEYS, MAIN_CHARS_KEYS.to_vec(), SectionKind::MainChars, None)}

        // Companies (reversed) — never restricted
        {render_section(s, i18n, i18n.section_companies(), COMPANY_KEYS, companies_display, SectionKind::Companies, None)}

        // Job Roles (alphabetical) — restricted by selected companies
        {render_section(s, i18n, i18n.section_job_roles(), JOB_ROLE_KEYS, job_roles_display, SectionKind::JobRoles, Some(&available.job_roles))}

        // Projects (reversed) — restricted by selected companies
        {render_section(s, i18n, i18n.section_projects(), PROJECT_KEYS, projects_display, SectionKind::Projects, Some(&available.projects))}

        // Technical Skills (alphabetical) — restricted by selected companies
        {render_section(s, i18n, i18n.section_skills(), SKILL_KEYS, skills_display, SectionKind::Skills, Some(&available.skills))}

        // Tools (alphabetical) — restricted by selected companies
        {render_section(s, i18n, i18n.section_tools(), TOOL_KEYS, tools_display, SectionKind::Tools, Some(&available.tools))}
    }
}

#[derive(Clone, Copy)]
enum SectionKind {
    Skills,
    Companies,
    JobRoles,
    Projects,
    MainChars,
    Tools,
}

fn get_signal(s: AppState, kind: SectionKind) -> Signal<Vec<&'static str>> {
    match kind {
        SectionKind::Skills => s.selected_skills,
        SectionKind::Companies => s.selected_companies,
        SectionKind::JobRoles => s.selected_job_roles,
        SectionKind::Projects => s.selected_projects,
        SectionKind::MainChars => s.selected_main_chars,
        SectionKind::Tools => s.selected_tools,
    }
}

fn render_section(
    s: AppState,
    i18n: &dyn UiI18n,
    section_title: &str,
    all_keys: &'static [&'static str],
    display_keys: Vec<&'static str>,
    kind: SectionKind,
    available_keys: Option<&Vec<&'static str>>,
) -> Element {
    let signal = get_signal(s, kind);
    let selected = signal.read();
    let all_selected = all_keys.iter().all(|k| selected.contains(k));

    let title = section_title.to_string();
    let select_all_label = i18n.filter_select_all().to_string();
    let clear_all_label = i18n.filter_clear_all().to_string();

    // Pre-compute availability for each key so we can pass owned data into rsx
    let items_with_availability: Vec<(&'static str, bool)> = display_keys
        .iter()
        .map(|&key| {
            let is_available = available_keys
                .map(|avail| avail.contains(&key))
                .unwrap_or(true);
            (key, is_available)
        })
        .collect();

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
                            sig.set(all_keys.to_vec());
                        }
                    },
                    if all_selected { "{clear_all_label}" } else { "{select_all_label}" }
                }
            }

            // Checkboxes in display order
            for (key, is_available) in items_with_availability.iter() {
                {render_checkbox_item(s, i18n, key, kind, *is_available)}
            }
        }
    }
}

fn render_checkbox_item(
    s: AppState,
    i18n: &dyn UiI18n,
    key: &'static str,
    kind: SectionKind,
    is_available: bool,
) -> Element {
    let signal = get_signal(s, kind);
    let is_checked = signal.read().contains(&key);
    let label = i18n.item_label(key).to_string();
    let hint = i18n.item_hint(key).to_string();

    let label_style = if is_available {
        "display: flex; align-items: flex-start; gap: 8px; padding: 6px 0; cursor: pointer;"
    } else {
        "display: flex; align-items: flex-start; gap: 8px; padding: 6px 0; cursor: default; opacity: 0.4;"
    };

    rsx! {
        label { style: "{label_style}",
            input {
                r#type: "checkbox",
                checked: is_checked && is_available,
                disabled: !is_available,
                style: "margin-top: 3px; accent-color: #667eea;",
                onchange: move |_| {
                    if !is_available { return; }
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
