use super::UiI18n;

pub struct EnglishUi;

impl UiI18n for EnglishUi {
    fn tab_profile(&self) -> &'static str { "Profile" }
    fn tab_filter(&self) -> &'static str { "Filter" }
    fn tab_display(&self) -> &'static str { "Display" }

    fn profile_name(&self) -> &'static str { "István Finta" }
    fn profile_title(&self) -> &'static str { "Software Engineer" }
    fn profile_location(&self) -> &'static str { "Hungary" }
    fn profile_email(&self) -> &'static str { "istvan_finta@yahoo.com" }
    fn profile_phone(&self) -> &'static str { "+36 70 343 9517" }
    fn profile_about(&self) -> &'static str { "About me" }
    fn profile_about_text(&self) -> &'static str {
        "Experienced software engineer with 30+ years in the industry. \
         Expertise in embedded systems, desktop and web applications, \
         test automation and project management. Worked across Hungary, \
         Germany and Austria in automotive, traffic control, medical imaging, \
         financial software and blockchain domains."
    }

    fn section_skills(&self) -> &'static str { "Technical Skills" }
    fn section_countries(&self) -> &'static str { "Countries" }
    fn section_languages(&self) -> &'static str { "Language Skills" }
    fn section_companies(&self) -> &'static str { "Companies" }
    fn section_certificates(&self) -> &'static str { "Certificates" }
    fn filter_select_all(&self) -> &'static str { "Select all" }
    fn filter_clear_all(&self) -> &'static str { "Clear all" }

    fn display_nothing_selected(&self) -> &'static str { "Select items on the Filter tab to see details here." }

    fn item_label(&self, key: &str) -> &'static str {
        match key {
            // Skills
            "c_embedded" => "C (embedded)",
            "c_win_gui" => "C (Windows/GUI)",
            "cpp_win_gui" => "C++ (Windows/GUI)",
            "cpp_linux_server" => "C++ (Linux/Server)",
            "csharp" => "C#",
            "sql" => "SQL",
            "rust" => "Rust / WebAssembly",
            "multimedia" => "Multimedia Programming",
            "system_design" => "System Design",
            "project_management" => "Project Management",
            "test_management" => "Test Management",
            "automated_testing" => "Automated Testing",
            "manual_testing" => "Manual Testing",
            "erp" => "ERP Systems",
            "administration" => "System Administration",
            // Countries
            "hungary" => "Hungary",
            "germany" => "Germany",
            "austria" => "Austria",
            // Languages
            "lang_hungarian" => "Hungarian (native)",
            "lang_german" => "German (fluent)",
            "lang_english" => "English (fluent)",
            // Companies
            "mol" => "MOL AG (1993)",
            "bako" => "Bäko-Hungaria (1993)",
            "teamcom" => "Teamcom (1995–2001)",
            "vilati" => "Vilati (2001–2005)",
            "mediso" => "Mediso (2005)",
            "bosch" => "Bosch (2005–2010)",
            "porsche" => "Porsche Informatik (2010–2016)",
            "sigmatek" => "Sigmatek (2016)",
            "bitnok" => "Bitnök (2016–2022)",
            "telekom" => "Telekom (2022–)",
            // Certificates
            "cert_diploma" => "University Diploma",
            "cert_pm" => "PM Certificate (PMCC)",
            "cert_sql" => "Database & SQL Fundamentals",
            "cert_js" => "JavaScript Certificate",
            "cert_ai" => "Programming with AI Agents",
            "cert_driving" => "Advanced Driving Technique",
            _ => "",
        }
    }

    fn item_hint(&self, key: &str) -> &'static str {
        match key {
            "c_embedded" => "Microcontroller firmware for traffic systems & automotive ECUs",
            "c_win_gui" => "Windows desktop applications in C",
            "cpp_win_gui" => "Windows GUI applications in C++",
            "cpp_linux_server" => "Linux server-side development in C++",
            "csharp" => "C# (.NET) — test tools, ERP, Prüfloader",
            "sql" => "Database queries, stored procedures",
            "rust" => "Rust + Dioxus + WebAssembly for web apps",
            "multimedia" => "Multimedia & interactive presentations",
            "system_design" => "Architecture & system design for traffic control",
            "project_management" => "Leading teams and managing projects",
            "test_management" => "Test strategy, planning and reporting",
            "automated_testing" => "Automated HW/SW test development",
            "manual_testing" => "Functional & regression testing",
            "erp" => "Enterprise resource planning systems",
            "administration" => "System & network administration",
            "hungary" => "Vilati, Mediso, MOL, Bäko-Hungaria, Bitnök, Telekom",
            "germany" => "Teamcom, Bosch",
            "austria" => "Porsche Informatik, Sigmatek",
            "lang_hungarian" => "Native language",
            "lang_german" => "Working language since 1995",
            "lang_english" => "Professional proficiency",
            "mol" => "Laboratory system administration",
            "bako" => "ERP system administration",
            "teamcom" => "Financial multimedia software — Deutsche Bank, Citibank, C&L",
            "vilati" => "Traffic junction control, embedded C, department leadership",
            "mediso" => "Medical image processing",
            "bosch" => "Automotive ECU development & test — Body Computers, Window Lifters",
            "porsche" => "Dealer management systems — C++ Linux/Windows, SQL",
            "sigmatek" => "Prüfloader.NET — industrial test tool in C#",
            "bitnok" => "Blockchain project management — VortexLedger",
            "telekom" => "GIS / GeoRoute administration",
            "cert_diploma" => "University degree in engineering",
            "cert_pm" => "Project Management Certificate — PMCC",
            "cert_sql" => "Database and SQL fundamentals course",
            "cert_js" => "JavaScript development certificate",
            "cert_ai" => "Programming with AI agents course",
            "cert_driving" => "Advanced driving technique for safety",
            _ => "",
        }
    }

    fn btn_generate_pdf(&self) -> &'static str { "Generate PDF" }
    fn pdf_filter_choices(&self) -> &'static str { "Selected Filters" }

    fn toast_update_available(&self) -> &'static str { "A new version is available!" }
    fn btn_update_now(&self) -> &'static str { "Update Now" }
}
