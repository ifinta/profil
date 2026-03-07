mod english;
mod german;
mod hungarian;

use crate::i18n::Language;
use english::EnglishUi;
use german::GermanUi;
use hungarian::HungarianUi;

/// Trait for UI internationalized strings
pub trait UiI18n {
    // Tab labels
    fn tab_profile(&self) -> &'static str;
    fn tab_filter(&self) -> &'static str;
    fn tab_display(&self) -> &'static str;

    // Profile tab
    fn profile_name(&self) -> &'static str;
    fn profile_subtitle(&self) -> &'static str;
    fn profile_title(&self) -> &'static str;
    fn profile_location(&self) -> &'static str;
    fn profile_email(&self) -> &'static str;
    fn profile_phone(&self) -> &'static str;

    // Filter tab section headers
    fn section_skills(&self) -> &'static str;
    fn section_countries(&self) -> &'static str;
    fn section_languages(&self) -> &'static str;
    fn section_companies(&self) -> &'static str;
    fn section_certificates(&self) -> &'static str;
    fn filter_select_all(&self) -> &'static str;
    fn filter_clear_all(&self) -> &'static str;

    // Sankey-based filter sections
    fn section_job_roles(&self) -> &'static str;
    fn section_projects(&self) -> &'static str;

    // Tools section
    fn section_tools(&self) -> &'static str;

    // Project experience section header
    fn section_project_experience(&self) -> &'static str;

    // "Főbb jellemzőim" (My Main Characteristics) group
    fn section_main_chars(&self) -> &'static str;
    fn role_achievements_title(&self) -> &'static str;
    fn role_achievements(&self) -> &'static [(&'static str, &'static str)];
    fn digital_skills_title(&self) -> &'static str;
    fn digital_skills_text(&self) -> &'static str;

    // Display tab
    fn display_nothing_selected(&self) -> &'static str;

    // Translate an item key to localized text
    fn item_label(&self, key: &str) -> &'static str;

    // Hints for filter items
    fn item_hint(&self, key: &str) -> &'static str;

    // PDF
    fn btn_generate_pdf(&self) -> &'static str;

    // Role selector (Profile tab)
    fn role_section_label(&self) -> &'static str;
    fn role_label(&self, key: &str) -> &'static str;
    fn role_title(&self, key: &str) -> &'static str;
    fn role_target_title(&self) -> &'static str;
    fn role_target_text(&self, key: &str) -> &'static str;
    fn role_strengths_title(&self) -> &'static str;
    fn role_strengths(&self, key: &str) -> &'static [(&'static str, &'static str)];

    // PWA install hint
    fn pwa_hint(&self) -> &'static str;
}

/// Factory function
pub fn ui_i18n(lang: Language) -> Box<dyn UiI18n> {
    match lang {
        Language::English => Box::new(EnglishUi),
        Language::German => Box::new(GermanUi),
        Language::Hungarian => Box::new(HungarianUi),
    }
}
