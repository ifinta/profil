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
    "leading", "auto_test_dev",
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
    "cert_dotnet", "cert_sql", "cert_canoe", "cert_daf",
    "cert_diploma", "cert_germanistik", "cert_istqb", "cert_js",
    "cert_moderation", "cert_access", "cert_pm", "cert_presentation",
    "cert_ai", "cert_scrum", "cert_driving",
];

// ── Sankey-based groups ──

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

// ── Tools (IDEs, testing tools, compilers, etc.) ──

pub const TOOL_KEYS: &[&str] = &[
    "tl_visual_studio", "tl_ms_office", "tl_clearcase", "tl_clearquest",
    "tl_doors", "tl_isystem", "tl_dspace", "tl_ni_teststand",
    "tl_ni_cvi", "tl_agilent", "tl_opentest", "tl_authors_dream",
    "tl_authorware", "tl_python", "tl_snow", "tl_keil",
    "tl_cosmic", "tl_gnu_cc", "tl_tornado", "tl_java_sdk",
    "tl_structured_text", "tl_node_js",
];

// ── "Főbb jellemzőim" (My Main Characteristics) group ──

pub const MAIN_CHARS_KEYS: &[&str] = &[
    "mc_strengths", "mc_achievements",
    "mc_countries", "mc_languages", "mc_certificates",
    "mc_digital_skills",
];

// ── Project experience (extracted from CV pages 4+) ──

pub struct ProjectEntry {
    pub company_key: &'static str,
    pub date_interval: &'static str,
    pub project_key: &'static str,
    pub job_role_keys: &'static [&'static str],
    pub skill_keys: &'static [&'static str],
    pub tool_keys: &'static [&'static str],
}

pub const PROJECT_EXPERIENCE: &[ProjectEntry] = &[
    // ── Deutsche Telekom ──
    ProjectEntry { company_key: "telekom", date_interval: "2022.03–2024.03", project_key: "pj_georoute",
        job_role_keys: &["jr_admin"], skill_keys: &["sql", "administration"], tool_keys: &["tl_snow"] },

    // ── Bitnök ──
    ProjectEntry { company_key: "bitnok", date_interval: "2016.10–2022.02", project_key: "pj_vortexledger",
        job_role_keys: &["jr_sw_dev", "jr_project_mgr"], skill_keys: &["project_management"], tool_keys: &["tl_node_js"] },

    // ── Sigmatek ──
    ProjectEntry { company_key: "sigmatek", date_interval: "2016.02–2016.09", project_key: "pj_prufloader",
        job_role_keys: &["jr_sw_dev"], skill_keys: &["csharp", "sql"], tool_keys: &["tl_visual_studio", "tl_structured_text"] },

    // ── Porsche Informatik ──
    ProjectEntry { company_key: "porsche", date_interval: "2010.11–2015.09", project_key: "pj_contract",
        job_role_keys: &["jr_sw_dev", "jr_tester"], skill_keys: &["cpp_win_gui", "cpp_linux_server", "sql"], tool_keys: &[] },
    ProjectEntry { company_key: "porsche", date_interval: "2010.11–2015.09", project_key: "pj_asanet",
        job_role_keys: &["jr_sw_dev", "jr_tester"], skill_keys: &["cpp_win_gui", "cpp_linux_server", "sql"], tool_keys: &[] },
    ProjectEntry { company_key: "porsche", date_interval: "2013.02–2015.09", project_key: "pj_lynx",
        job_role_keys: &["jr_sw_dev", "jr_tester"], skill_keys: &["cpp_linux_server", "sql"], tool_keys: &[] },
    ProjectEntry { company_key: "porsche", date_interval: "2010.11–2015.09", project_key: "pj_sms_email",
        job_role_keys: &["jr_sw_dev", "jr_tester"], skill_keys: &["cpp_win_gui", "cpp_linux_server", "sql"], tool_keys: &[] },
    ProjectEntry { company_key: "porsche", date_interval: "2014.04–2015.09", project_key: "pj_customer_cards",
        job_role_keys: &["jr_sw_dev", "jr_tester"], skill_keys: &["cpp_win_gui", "cpp_linux_server", "sql"], tool_keys: &[] },
    ProjectEntry { company_key: "porsche", date_interval: "2012.01–2015.09", project_key: "pj_e_billing",
        job_role_keys: &["jr_sw_dev", "jr_tester"], skill_keys: &["cpp_win_gui", "cpp_linux_server", "sql"], tool_keys: &[] },
    ProjectEntry { company_key: "porsche", date_interval: "2011.05–2015.09", project_key: "pj_mein_auto",
        job_role_keys: &["jr_sw_dev", "jr_tester"], skill_keys: &["cpp_linux_server", "sql"], tool_keys: &[] },
    ProjectEntry { company_key: "porsche", date_interval: "2012.07–2015.09", project_key: "pj_mobil_car",
        job_role_keys: &["jr_sw_dev", "jr_tester"], skill_keys: &["cpp_linux_server", "sql"], tool_keys: &[] },
    ProjectEntry { company_key: "porsche", date_interval: "2012.07–2014.10", project_key: "pj_supplier_inv",
        job_role_keys: &["jr_sw_dev", "jr_tester"], skill_keys: &["cpp_win_gui", "cpp_linux_server", "sql"], tool_keys: &[] },

    // ── Robert Bosch ──
    ProjectEntry { company_key: "bosch", date_interval: "2009.11–2010.09", project_key: "pj_car_body",
        job_role_keys: &["jr_sw_dev"], skill_keys: &["c_embedded"],
        tool_keys: &["tl_cosmic", "tl_isystem", "tl_visual_studio", "tl_clearcase", "tl_clearquest", "tl_doors"] },
    ProjectEntry { company_key: "bosch", date_interval: "2009.09–2009.10", project_key: "pj_dcdc",
        job_role_keys: &["jr_sw_dev"], skill_keys: &["c_embedded"],
        tool_keys: &["tl_visual_studio", "tl_clearcase"] },
    ProjectEntry { company_key: "bosch", date_interval: "2009.03–2009.08", project_key: "pj_erp_bosch",
        job_role_keys: &["jr_sw_dev"], skill_keys: &["csharp", "sql", "erp"],
        tool_keys: &["tl_visual_studio", "tl_ms_office"] },
    ProjectEntry { company_key: "bosch", date_interval: "2008.03–2009.03", project_key: "pj_test_designer",
        job_role_keys: &["jr_test_mgr", "jr_sw_dev"], skill_keys: &["csharp", "test_management"],
        tool_keys: &["tl_visual_studio", "tl_python", "tl_dspace", "tl_ms_office"] },
    ProjectEntry { company_key: "bosch", date_interval: "2007.06–2008.03", project_key: "pj_test_tools",
        job_role_keys: &["jr_test_dev"], skill_keys: &["automated_testing"],
        tool_keys: &["tl_opentest", "tl_ms_office"] },
    ProjectEntry { company_key: "bosch", date_interval: "2005.10–2008.03", project_key: "pj_window_lifter",
        job_role_keys: &["jr_tester", "jr_test_dev", "jr_sw_dev"], skill_keys: &["csharp", "automated_testing", "manual_testing"],
        tool_keys: &["tl_ni_teststand", "tl_agilent", "tl_visual_studio", "tl_ms_office", "tl_opentest", "tl_ni_cvi"] },
    ProjectEntry { company_key: "bosch", date_interval: "2007.01–2008.03", project_key: "pj_test_net",
        job_role_keys: &["jr_sw_dev"], skill_keys: &["csharp"],
        tool_keys: &["tl_visual_studio", "tl_ms_office"] },
    ProjectEntry { company_key: "bosch", date_interval: "2006.06–2007.06", project_key: "pj_truck_body",
        job_role_keys: &["jr_test_mgr"], skill_keys: &["test_management"],
        tool_keys: &["tl_ms_office"] },

    // ── Mediso ──
    ProjectEntry { company_key: "mediso", date_interval: "2005.08–2005.09", project_key: "pj_medical",
        job_role_keys: &["jr_sw_dev"], skill_keys: &["cpp_win_gui", "c_win_gui"],
        tool_keys: &["tl_visual_studio"] },

    // ── Vilati ──
    ProjectEntry { company_key: "vilati", date_interval: "2003.06–2005.07", project_key: "pj_ocit",
        job_role_keys: &["jr_dept_head", "jr_sw_dev"], skill_keys: &["c_embedded", "system_design", "leading"],
        tool_keys: &["tl_gnu_cc"] },
    ProjectEntry { company_key: "vilati", date_interval: "2003.06–2003.11", project_key: "pj_debrecen",
        job_role_keys: &["jr_dept_head"], skill_keys: &["system_design", "leading"],
        tool_keys: &[] },
    ProjectEntry { company_key: "vilati", date_interval: "2002.09–2002.12", project_key: "pj_junction",
        job_role_keys: &["jr_sw_dev"], skill_keys: &["cpp_win_gui", "c_win_gui"],
        tool_keys: &["tl_visual_studio", "tl_java_sdk"] },
    ProjectEntry { company_key: "vilati", date_interval: "2002.03–2003.05", project_key: "pj_btc5000",
        job_role_keys: &["jr_sw_dev"], skill_keys: &["c_embedded"],
        tool_keys: &["tl_tornado", "tl_java_sdk"] },
    ProjectEntry { company_key: "vilati", date_interval: "2001.04–2002.02", project_key: "pj_sk24",
        job_role_keys: &["jr_sw_dev"], skill_keys: &["c_embedded"],
        tool_keys: &["tl_keil"] },

    // ── Teamcom ──
    ProjectEntry { company_key: "teamcom", date_interval: "1998.06–2001.03", project_key: "pj_db_finance",
        job_role_keys: &["jr_sw_dev"], skill_keys: &["c_win_gui", "multimedia"],
        tool_keys: &["tl_authors_dream", "tl_visual_studio"] },
    ProjectEntry { company_key: "teamcom", date_interval: "1997.01–1998.10", project_key: "pj_cwl_kwg",
        job_role_keys: &["jr_sw_dev"], skill_keys: &["c_win_gui", "multimedia"],
        tool_keys: &["tl_authors_dream"] },
    ProjectEntry { company_key: "teamcom", date_interval: "1995.07–2001.03", project_key: "pj_authors_dream",
        job_role_keys: &["jr_sw_dev"], skill_keys: &["c_win_gui", "multimedia"],
        tool_keys: &["tl_visual_studio"] },
    ProjectEntry { company_key: "teamcom", date_interval: "1995.02–1995.06", project_key: "pj_citibank",
        job_role_keys: &["jr_sw_dev"], skill_keys: &["c_win_gui"],
        tool_keys: &["tl_authorware", "tl_visual_studio"] },

    // ── MOL ──
    ProjectEntry { company_key: "mol", date_interval: "1993.03–1994.10", project_key: "pj_labor",
        job_role_keys: &["jr_admin"], skill_keys: &["administration"],
        tool_keys: &[] },

    // ── Bäko-Hungaria ──
    ProjectEntry { company_key: "bako", date_interval: "1993.03–1994.10", project_key: "pj_erp_system",
        job_role_keys: &["jr_admin"], skill_keys: &["administration", "erp"],
        tool_keys: &[] },
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
    selected_companies: Vec<String>,
    #[serde(default)]
    selected_job_roles: Vec<String>,
    #[serde(default)]
    selected_projects: Vec<String>,
    #[serde(default)]
    selected_main_chars: Vec<String>,
    #[serde(default)]
    selected_tools: Vec<String>,
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
    pub selected_companies: Signal<Vec<&'static str>>,
    pub selected_job_roles: Signal<Vec<&'static str>>,
    pub selected_projects: Signal<Vec<&'static str>>,
    pub selected_main_chars: Signal<Vec<&'static str>>,
    pub selected_tools: Signal<Vec<&'static str>>,
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
        selected_companies: use_signal(|| {
            load_persisted()
                .map(|p| resolve_keys(&p.selected_companies, COMPANY_KEYS))
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
        selected_main_chars: use_signal(|| {
            load_persisted()
                .map(|p| resolve_keys(&p.selected_main_chars, MAIN_CHARS_KEYS))
                .unwrap_or_default()
        }),
        selected_tools: use_signal(|| {
            load_persisted()
                .map(|p| resolve_keys(&p.selected_tools, TOOL_KEYS))
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
        selected_companies: s.selected_companies.read().iter().map(|s| s.to_string()).collect(),
        selected_job_roles: s.selected_job_roles.read().iter().map(|s| s.to_string()).collect(),
        selected_projects: s.selected_projects.read().iter().map(|s| s.to_string()).collect(),
        selected_main_chars: s.selected_main_chars.read().iter().map(|s| s.to_string()).collect(),
        selected_tools: s.selected_tools.read().iter().map(|s| s.to_string()).collect(),
    };
    save_persisted(&snapshot);
}
