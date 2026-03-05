use dioxus::prelude::*;
use crate::i18n::Language;
use super::tabs::Tab;

/// All checkbox item keys used across filters.
/// Each key is a &'static str that the i18n trait can translate.
pub const SKILL_KEYS: &[&str] = &[
    "c_embedded", "c_win_gui", "cpp_win_gui", "cpp_linux_server",
    "csharp", "sql", "rust", "multimedia",
    "system_design", "project_management", "test_management",
    "automated_testing", "manual_testing", "erp", "administration",
];

pub const COUNTRY_KEYS: &[&str] = &[
    "hungary", "germany", "austria",
];

pub const LANGUAGE_KEYS: &[&str] = &[
    "lang_hungarian", "lang_german", "lang_english",
];

pub const COMPANY_KEYS: &[&str] = &[
    "mol", "bako", "teamcom", "vilati", "mediso",
    "bosch", "porsche", "sigmatek", "bitnok", "telekom",
];

pub const CERTIFICATE_KEYS: &[&str] = &[
    "cert_diploma", "cert_pm", "cert_sql", "cert_js", "cert_ai", "cert_driving",
];

#[derive(Clone, Copy)]
pub struct AppState {
    pub language: Signal<Language>,
    pub active_tab: Signal<Tab>,
    pub selected_skills: Signal<Vec<&'static str>>,
    pub selected_countries: Signal<Vec<&'static str>>,
    pub selected_languages: Signal<Vec<&'static str>>,
    pub selected_companies: Signal<Vec<&'static str>>,
    pub selected_certificates: Signal<Vec<&'static str>>,
}

pub fn use_app_state() -> AppState {
    AppState {
        language: use_signal(Language::default),
        active_tab: use_signal(Tab::default),
        selected_skills: use_signal(Vec::new),
        selected_countries: use_signal(Vec::new),
        selected_languages: use_signal(Vec::new),
        selected_companies: use_signal(Vec::new),
        selected_certificates: use_signal(Vec::new),
    }
}
