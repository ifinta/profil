use super::UiI18n;

pub struct GermanUi;

impl UiI18n for GermanUi {
    fn tab_profile(&self) -> &'static str { "Profil" }
    fn tab_filter(&self) -> &'static str { "Filter" }
    fn tab_display(&self) -> &'static str { "Anzeige" }

    fn profile_name(&self) -> &'static str { "István Finta" }
    fn profile_title(&self) -> &'static str { "Software-Entwickler" }
    fn profile_location(&self) -> &'static str { "Ungarn" }
    fn profile_email(&self) -> &'static str { "istvan_finta@yahoo.com" }
    fn profile_phone(&self) -> &'static str { "+36 70 343 9517" }
    fn profile_about(&self) -> &'static str { "Über mich" }
    fn profile_about_text(&self) -> &'static str {
        "Erfahrener Software-Entwickler mit über 30 Jahren Branchenerfahrung. \
         Expertise in eingebetteten Systemen, Desktop- und Webanwendungen, \
         Testautomatisierung und Projektmanagement. Tätig in Ungarn, \
         Deutschland und Österreich in den Bereichen Automotive, Verkehrssteuerung, \
         medizinische Bildgebung, Finanzsoftware und Blockchain."
    }

    fn section_skills(&self) -> &'static str { "Technische Kenntnisse" }
    fn section_countries(&self) -> &'static str { "Länder" }
    fn section_languages(&self) -> &'static str { "Sprachkenntnisse" }
    fn section_companies(&self) -> &'static str { "Firmen" }
    fn section_certificates(&self) -> &'static str { "Zertifikate" }
    fn filter_select_all(&self) -> &'static str { "Alle auswählen" }
    fn filter_clear_all(&self) -> &'static str { "Alle löschen" }

    fn display_nothing_selected(&self) -> &'static str { "Wählen Sie Elemente im Filter-Tab aus, um Details hier zu sehen." }

    fn item_label(&self, key: &str) -> &'static str {
        match key {
            "c_embedded" => "C (embedded)",
            "c_win_gui" => "C (Windows/GUI)",
            "cpp_win_gui" => "C++ (Windows/GUI)",
            "cpp_linux_server" => "C++ (Linux/Server)",
            "csharp" => "C#",
            "sql" => "SQL",
            "rust" => "Rust / WebAssembly",
            "multimedia" => "Multimedia-Programmierung",
            "system_design" => "System-Design",
            "project_management" => "Projektmanagement",
            "test_management" => "Test Management",
            "automated_testing" => "Automatisiertes Testen",
            "manual_testing" => "Manuelles Testen",
            "erp" => "ERP-Systeme",
            "administration" => "Systemadministration",
            "hungary" => "Ungarn",
            "germany" => "Deutschland",
            "austria" => "Österreich",
            "lang_hungarian" => "Ungarisch (Muttersprache)",
            "lang_german" => "Deutsch (fließend)",
            "lang_english" => "Englisch (fließend)",
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
            "cert_diploma" => "Universitätsdiplom",
            "cert_pm" => "PM-Zertifikat (PMCC)",
            "cert_sql" => "Datenbank & SQL Grundlagen",
            "cert_js" => "JavaScript-Zertifikat",
            "cert_ai" => "Programmierung mit KI-Agenten",
            "cert_driving" => "Fahrtechnik-Zertifikat",
            _ => "",
        }
    }

    fn item_hint(&self, key: &str) -> &'static str {
        match key {
            "c_embedded" => "Mikrocontroller-Firmware für Verkehrssysteme & Automotive-ECUs",
            "c_win_gui" => "Windows-Desktop-Anwendungen in C",
            "cpp_win_gui" => "Windows-GUI-Anwendungen in C++",
            "cpp_linux_server" => "Linux-Server-Entwicklung in C++",
            "csharp" => "C# (.NET) — Testtools, ERP, Prüfloader",
            "sql" => "Datenbankabfragen, gespeicherte Prozeduren",
            "rust" => "Rust + Dioxus + WebAssembly für Webanwendungen",
            "multimedia" => "Multimedia & interaktive Präsentationen",
            "system_design" => "Architektur & Systemdesign für Verkehrssteuerung",
            "project_management" => "Teamführung und Projektmanagement",
            "test_management" => "Teststrategie, Planung und Berichterstattung",
            "automated_testing" => "Automatisierte HW/SW-Testentwicklung",
            "manual_testing" => "Funktionale Tests & Regressionstests",
            "erp" => "Unternehmensressourcenplanung",
            "administration" => "System- und Netzwerkadministration",
            "hungary" => "Vilati, Mediso, MOL, Bäko-Hungaria, Bitnök, Telekom",
            "germany" => "Teamcom, Bosch",
            "austria" => "Porsche Informatik, Sigmatek",
            "lang_hungarian" => "Muttersprache",
            "lang_german" => "Arbeitssprache seit 1995",
            "lang_english" => "Berufliche Kompetenz",
            "mol" => "Administration des Laborsystems",
            "bako" => "ERP-Systemadministration",
            "teamcom" => "Finanzmultimedia-Software — Deutsche Bank, Citibank, C&L",
            "vilati" => "Verkehrssteuerung, eingebettetes C, Abteilungsleitung",
            "mediso" => "Medizinische Bildverarbeitung",
            "bosch" => "Automotive-ECU-Entwicklung & Test — Body Computer, Fensterheber",
            "porsche" => "Händlermanagement-Systeme — C++ Linux/Windows, SQL",
            "sigmatek" => "Prüfloader.NET — industrielles Testtool in C#",
            "bitnok" => "Blockchain-Projektmanagement — VortexLedger",
            "telekom" => "GIS / GeoRoute Administration",
            "cert_diploma" => "Ingenieurstudium-Abschluss",
            "cert_pm" => "Projektmanagement-Zertifikat — PMCC",
            "cert_sql" => "Datenbank- und SQL-Grundlagenkurs",
            "cert_js" => "JavaScript-Entwicklungszertifikat",
            "cert_ai" => "Programmierung mit KI-Agenten Kurs",
            "cert_driving" => "Fahrtechnik für Sicherheit",
            _ => "",
        }
    }

    fn btn_generate_pdf(&self) -> &'static str { "PDF erstellen" }
    fn pdf_filter_choices(&self) -> &'static str { "Ausgewählte Filter" }
}
