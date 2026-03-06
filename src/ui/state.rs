use dioxus::prelude::*;
use serde::{Serialize, Deserialize};
use crate::i18n::Language;
use super::tabs::Tab;

#[derive(Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub enum Role {
    #[default]
    SoftwareEngineer,
    ProjectOwner,
    TestManager,
}

impl Role {
    pub fn key(&self) -> &'static str {
        match self {
            Role::SoftwareEngineer => "szm",
            Role::ProjectOwner => "po",
            Role::TestManager => "tm",
        }
    }

    pub fn from_key(key: &str) -> Self {
        match key {
            "po" => Role::ProjectOwner,
            "tm" => Role::TestManager,
            _ => Role::SoftwareEngineer,
        }
    }
}

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

// ── Sankey-based groups ──

pub const WORKPLACE_KEYS: &[&str] = &[
    "wk_mol", "wk_bako", "wk_teamcom", "wk_vilati", "wk_mediso",
    "wk_bosch", "wk_porsche", "wk_sigmatek", "wk_bitnok", "wk_telekom",
];

pub const JOB_ROLE_KEYS: &[&str] = &[
    "jr_admin", "jr_sw_dev", "jr_dept_head", "jr_tester",
    "jr_test_mgr", "jr_test_dev", "jr_project_mgr",
];

pub const PROJECT_KEYS: &[&str] = &[
    "pj_labor", "pj_erp_system",
    "pj_db_finance", "pj_cwl_kwg", "pj_authors_dream", "pj_citibank",
    "pj_junction", "pj_btc5000", "pj_sk24", "pj_ocit", "pj_debrecen",
    "pj_medical",
    "pj_car_body", "pj_dcdc", "pj_erp_bosch", "pj_test_designer",
    "pj_test_net", "pj_truck_body", "pj_test_tools", "pj_window_lifter",
    "pj_contract", "pj_asanet", "pj_lynx", "pj_sms_email",
    "pj_customer_cards", "pj_e_billing", "pj_mein_auto",
    "pj_mobil_car", "pj_supplier_inv",
    "pj_prufloader", "pj_vortexledger", "pj_georoute",
];

pub const EXPERTISE_KEYS: &[&str] = &[
    "ex_admin", "ex_c_win", "ex_c_embedded", "ex_cpp_win", "ex_cpp_linux",
    "ex_csharp", "ex_sql", "ex_multimedia", "ex_system_design",
    "ex_project_mgmt", "ex_leading", "ex_manual_test",
    "ex_auto_testing", "ex_auto_test_dev", "ex_test_mgmt",
];

const STORAGE_KEY: &str = "profil_app_state";

/// Serializable snapshot of the entire app state for localStorage.
#[derive(Serialize, Deserialize)]
struct PersistedState {
    language: Language,
    active_tab: Tab,
    #[serde(default)]
    selected_role: Role,
    selected_skills: Vec<String>,
    selected_countries: Vec<String>,
    selected_languages: Vec<String>,
    selected_companies: Vec<String>,
    selected_certificates: Vec<String>,
    #[serde(default)]
    selected_workplaces: Vec<String>,
    #[serde(default)]
    selected_job_roles: Vec<String>,
    #[serde(default)]
    selected_projects: Vec<String>,
    #[serde(default)]
    selected_expertise: Vec<String>,
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
    pub selected_role: Signal<Role>,
    pub selected_skills: Signal<Vec<&'static str>>,
    pub selected_countries: Signal<Vec<&'static str>>,
    pub selected_languages: Signal<Vec<&'static str>>,
    pub selected_companies: Signal<Vec<&'static str>>,
    pub selected_certificates: Signal<Vec<&'static str>>,
    pub selected_workplaces: Signal<Vec<&'static str>>,
    pub selected_job_roles: Signal<Vec<&'static str>>,
    pub selected_projects: Signal<Vec<&'static str>>,
    pub selected_expertise: Signal<Vec<&'static str>>,
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
        selected_role: use_signal(|| {
            load_persisted().map(|p| p.selected_role).unwrap_or_default()
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
        selected_workplaces: use_signal(|| {
            load_persisted()
                .map(|p| resolve_keys(&p.selected_workplaces, WORKPLACE_KEYS))
                .unwrap_or_default()
        }),
        selected_job_roles: use_signal(|| {
            load_persisted()
                .map(|p| resolve_keys(&p.selected_job_roles, JOB_ROLE_KEYS))
                .unwrap_or_default()
        }),
        selected_projects: use_signal(|| {
            load_persisted()
                .map(|p| resolve_keys(&p.selected_projects, PROJECT_KEYS))
                .unwrap_or_default()
        }),
        selected_expertise: use_signal(|| {
            load_persisted()
                .map(|p| resolve_keys(&p.selected_expertise, EXPERTISE_KEYS))
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
        selected_role: *s.selected_role.read(),
        selected_skills: s.selected_skills.read().iter().map(|s| s.to_string()).collect(),
        selected_countries: s.selected_countries.read().iter().map(|s| s.to_string()).collect(),
        selected_languages: s.selected_languages.read().iter().map(|s| s.to_string()).collect(),
        selected_companies: s.selected_companies.read().iter().map(|s| s.to_string()).collect(),
        selected_certificates: s.selected_certificates.read().iter().map(|s| s.to_string()).collect(),
        selected_workplaces: s.selected_workplaces.read().iter().map(|s| s.to_string()).collect(),
        selected_job_roles: s.selected_job_roles.read().iter().map(|s| s.to_string()).collect(),
        selected_projects: s.selected_projects.read().iter().map(|s| s.to_string()).collect(),
        selected_expertise: s.selected_expertise.read().iter().map(|s| s.to_string()).collect(),
    };
    save_persisted(&snapshot);
}
