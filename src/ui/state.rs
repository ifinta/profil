use dioxus::prelude::*;
use serde::{Serialize, Deserialize};
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

const STORAGE_KEY: &str = "profil_app_state";

/// Serializable snapshot of the entire app state for localStorage.
#[derive(Serialize, Deserialize)]
struct PersistedState {
    language: Language,
    active_tab: Tab,
    selected_skills: Vec<String>,
    selected_countries: Vec<String>,
    selected_languages: Vec<String>,
    selected_companies: Vec<String>,
    selected_certificates: Vec<String>,
}

/// Convert a `Vec<String>` back to `Vec<&'static str>` by matching against
/// the known key slices.  Unknown values are silently dropped.
fn resolve_keys(saved: &[String], known: &[&'static str]) -> Vec<&'static str> {
    saved
        .iter()
        .filter_map(|s| known.iter().find(|&&k| k == s.as_str()).copied())
        .collect()
}

fn local_storage() -> Option<web_sys::Storage> {
    web_sys::window()?.local_storage().ok()?
}

fn load_persisted() -> Option<PersistedState> {
    let storage = local_storage()?;
    let json = storage.get_item(STORAGE_KEY).ok()??;
    serde_json::from_str(&json).ok()
}

fn save_persisted(state: &PersistedState) {
    if let Some(storage) = local_storage() {
        if let Ok(json) = serde_json::to_string(state) {
            let _ = storage.set_item(STORAGE_KEY, &json);
        }
    }
}

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
    // Try to restore from localStorage; fall back to defaults.
    let state = AppState {
        language: use_signal(|| {
            load_persisted().map(|p| p.language).unwrap_or_default()
        }),
        active_tab: use_signal(|| {
            load_persisted().map(|p| p.active_tab).unwrap_or_default()
        }),
        selected_skills: use_signal(|| {
            load_persisted()
                .map(|p| resolve_keys(&p.selected_skills, SKILL_KEYS))
                .unwrap_or_default()
        }),
        selected_countries: use_signal(|| {
            load_persisted()
                .map(|p| resolve_keys(&p.selected_countries, COUNTRY_KEYS))
                .unwrap_or_default()
        }),
        selected_languages: use_signal(|| {
            load_persisted()
                .map(|p| resolve_keys(&p.selected_languages, LANGUAGE_KEYS))
                .unwrap_or_default()
        }),
        selected_companies: use_signal(|| {
            load_persisted()
                .map(|p| resolve_keys(&p.selected_companies, COMPANY_KEYS))
                .unwrap_or_default()
        }),
        selected_certificates: use_signal(|| {
            load_persisted()
                .map(|p| resolve_keys(&p.selected_certificates, CERTIFICATE_KEYS))
                .unwrap_or_default()
        }),
    };

    state
}

/// Call this inside a `use_effect` to auto-save whenever any signal changes.
pub fn persist(s: &AppState) {
    let snapshot = PersistedState {
        language: *s.language.read(),
        active_tab: *s.active_tab.read(),
        selected_skills: s.selected_skills.read().iter().map(|s| s.to_string()).collect(),
        selected_countries: s.selected_countries.read().iter().map(|s| s.to_string()).collect(),
        selected_languages: s.selected_languages.read().iter().map(|s| s.to_string()).collect(),
        selected_companies: s.selected_companies.read().iter().map(|s| s.to_string()).collect(),
        selected_certificates: s.selected_certificates.read().iter().map(|s| s.to_string()).collect(),
    };
    save_persisted(&snapshot);
}
