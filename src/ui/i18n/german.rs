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

    fn section_job_roles(&self) -> &'static str { "Positionen" }
    fn section_projects(&self) -> &'static str { "Projekte" }
    fn section_tools(&self) -> &'static str { "Werkzeuge" }
    fn section_project_experience(&self) -> &'static str { "Projekte – detailliert" }

    fn section_main_chars(&self) -> &'static str { "Meine Hauptmerkmale" }
    fn role_achievements_title(&self) -> &'static str { "Wichtigste Erfolge" }
    fn role_achievements(&self) -> &'static [(&'static str, &'static str)] {
        &[
            ("Test.NET Testentwicklungs-Framework",
             "Später von Bosch an Vector verkauft."),
            ("SK24 Lampenschaltkarte Synchronisation",
             "Echtzeit-Synchronisation der internen Prozesse der Verkehrssteuerungs-Lampenschaltkarte mit den Sinuswellen des Stromnetzes."),
            ("Vortexledger-Konzept",
             "Ein auf Tauschhandel basierendes Open-Ledger- und Wirtschaftskonzept."),
        ]
    }

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
            "leading" => "Leitung",
            "auto_test_dev" => "Automatisierte-Test Entwicklung",
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
            // Workplaces (Sankey)
            "wk_mol" => "MOL AG (1993)",
            "wk_bako" => "Bäko-Hungaria (1993)",
            "wk_teamcom" => "Teamcom (1995–2001)",
            "wk_vilati" => "Vilati (2001–2005)",
            "wk_mediso" => "Mediso (2005)",
            "wk_bosch" => "Bosch (2005–2010)",
            "wk_porsche" => "Porsche Informatik (2010–2016)",
            "wk_sigmatek" => "Sigmatek (2016)",
            "wk_bitnok" => "Bitnök (2016–2022)",
            "wk_telekom" => "Telekom (2022–)",
            // Job roles (Sankey)
            "jr_admin" => "Administrator",
            "jr_sw_dev" => "Software-Entwickler",
            "jr_dept_head" => "Abteilungsleiter",
            "jr_tester" => "Tester",
            "jr_test_mgr" => "Test-Manager",
            "jr_test_dev" => "Test-Entwickler",
            "jr_project_mgr" => "Projekt Manager",
            // Projects (Sankey)
            "pj_labor" => "Labor",
            "pj_erp_system" => "ERP-System",
            "pj_db_finance" => "Deutsche Bank Baufinanzierung",
            "pj_cwl_kwg" => "C&L Deutsche Revision KWG Novelle",
            "pj_authors_dream" => "Author's Dream",
            "pj_citibank" => "Citibank Abwehr von Geldwäsche",
            "pj_junction" => "Prüftool",
            "pj_btc5000" => "BTC5000",
            "pj_sk24" => "SK24",
            "pj_ocit" => "OCIT",
            "pj_debrecen" => "Debrecen",
            "pj_medical" => "Bildverarbeitung",
            "pj_car_body" => "PKW Body Computer",
            "pj_dcdc" => "DC/DC Wandler",
            "pj_erp_bosch" => "Wirtschaftsplanung",
            "pj_test_designer" => "TestDesigner",
            "pj_test_net" => "Test.NET",
            "pj_truck_body" => "LKW Body Computer",
            "pj_test_tools" => "Verbesserung Test Tools",
            "pj_window_lifter" => "Fensterheber",
            "pj_contract" => "Auftrag",
            "pj_asanet" => "ASA-Net",
            "pj_lynx" => "Lynx",
            "pj_sms_email" => "SMS-eMail",
            "pj_customer_cards" => "Kunden-Karten",
            "pj_e_billing" => "Elektronisches Rechnunglegung",
            "pj_mein_auto" => "Mein-Auto App",
            "pj_mobil_car" => "Mobile-Fahrzeugannahme",
            "pj_supplier_inv" => "Eingangsrechnungkontrolle",
            "pj_prufloader" => "Prüfloader",
            "pj_vortexledger" => "Vortexledger",
            "pj_georoute" => "GIS GeoRoute",
            // Expertise (Sankey)
            "ex_admin" => "Administration",
            "ex_c_win" => "C win/GUI",
            "ex_c_embedded" => "C embedded",
            "ex_cpp_win" => "C++ win/GUI",
            "ex_cpp_linux" => "C++ linux/server",
            "ex_csharp" => "C#",
            "ex_sql" => "SQL",
            "ex_multimedia" => "Multimedia-Programmierung",
            "ex_system_design" => "System-Design",
            "ex_project_mgmt" => "Projektmanagement",
            "ex_leading" => "Leitung",
            "ex_manual_test" => "Manuelles Testen",
            "ex_auto_testing" => "Automatisiertes Testen",
            "ex_auto_test_dev" => "Automatisierte-Test Entwicklung",
            "ex_test_mgmt" => "Test Management",
            // Main Characteristics
            "mc_strengths" => "Stärken",
            "mc_achievements" => "Wichtigste Erfolge",
            "mc_countries" => "Länder",
            "mc_languages" => "Sprachkenntnisse",
            "mc_certificates" => "Zertifikate",
            // Tools
            "tl_visual_studio" => "Visual Studio",
            "tl_ms_office" => "MS Office (Excel, Word, Access)",
            "tl_clearcase" => "IBM ClearCase",
            "tl_clearquest" => "IBM ClearQuest",
            "tl_doors" => "IBM Doors",
            "tl_isystem" => "iSYSTEM Debugger",
            "tl_dspace" => "dSPACE Testturm",
            "tl_ni_teststand" => "NI TestStand",
            "tl_ni_cvi" => "NI LabWindows/CVI",
            "tl_agilent" => "Agilent Messgeräte",
            "tl_opentest" => "OpenTest (Bosch/Vector)",
            "tl_authors_dream" => "Author's Dream",
            "tl_authorware" => "Macromedia Authorware",
            "tl_python" => "Python",
            "tl_snow" => "ServiceNow (SNOW)",
            "tl_keil" => "Keil Compiler",
            "tl_cosmic" => "Cosmic C Compiler",
            "tl_gnu_cc" => "GNU C/C++ Toolchain",
            "tl_tornado" => "Wind River Tornado IDE",
            "tl_java_sdk" => "Sun Java SDK",
            "tl_structured_text" => "Structured Text / LASAL",
            "tl_node_js" => "Node.js",
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
            "leading" => "Vilati — Abteilungsleitung Softwareentwicklung",
            "auto_test_dev" => "Bosch — automatisierte Testtool-Entwicklung",
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
            // Workplace hints
            "wk_mol" => "Administration des Laborsystems",
            "wk_bako" => "ERP-Systemadministration",
            "wk_teamcom" => "Finanzmultimedia-Software — Deutsche Bank, Citibank, C&L",
            "wk_vilati" => "Verkehrssteuerung, eingebettetes C, Abteilungsleitung",
            "wk_mediso" => "Medizinische Bildverarbeitung",
            "wk_bosch" => "Automotive-ECU-Entwicklung & Test — Body Computer, Fensterheber",
            "wk_porsche" => "Händlermanagement-Systeme — C++ Linux/Windows, SQL",
            "wk_sigmatek" => "Prüfloader.NET — industrielles Testtool in C#",
            "wk_bitnok" => "Blockchain-Projektmanagement — VortexLedger",
            "wk_telekom" => "GIS / GeoRoute Administration",
            // Job role hints
            "jr_admin" => "MOL, Bäko-Hungaria, Telekom",
            "jr_sw_dev" => "Teamcom, Vilati, Mediso, Bosch, Porsche, Sigmatek",
            "jr_dept_head" => "Vilati — OCIT, Debrecen",
            "jr_tester" => "Bosch, Porsche Informatik",
            "jr_test_mgr" => "Bosch — LKW Body Computer, Test Tools",
            "jr_test_dev" => "Bosch — Fensterheber, LKW Body Computer",
            "jr_project_mgr" => "Bitnök — Vortexledger",
            // Project hints
            "pj_labor" => "MOL AG — Laborsystem",
            "pj_erp_system" => "Bäko-Hungaria — Unternehmensressourcenplanung",
            "pj_db_finance" => "Teamcom — Deutsche Bank Baufinanzierung",
            "pj_cwl_kwg" => "Teamcom — C&L Deutsche Revision",
            "pj_authors_dream" => "Teamcom — Multimedia-Editor",
            "pj_citibank" => "Teamcom — Abwehr von Geldwäsche",
            "pj_junction" => "Vilati — Verkehrssteuerung",
            "pj_btc5000" => "Vilati — Verkehrssteuerung",
            "pj_sk24" => "Vilati — Lampenschaltkarte, Echtzeitsynchronisation",
            "pj_ocit" => "Vilati — Abteilungsleitung, OCIT-Standard",
            "pj_debrecen" => "Vilati — Abteilungsleitung, Debrecen-Projekt",
            "pj_medical" => "Mediso — Medizinische Bildverarbeitung",
            "pj_car_body" => "Bosch — PKW Body Computer, C embedded",
            "pj_dcdc" => "Bosch — DC/DC Wandler, C embedded",
            "pj_erp_bosch" => "Bosch — Wirtschaftsplanung, C#",
            "pj_test_designer" => "Bosch — Testdesigner-Tool, C#",
            "pj_test_net" => "Bosch — Testentwicklungs-Framework, C#",
            "pj_truck_body" => "Bosch — LKW Body Computer, Testmanagement",
            "pj_test_tools" => "Bosch — Verbesserung der Testtools",
            "pj_window_lifter" => "Bosch — Fensterheber, automatisiertes Testen",
            "pj_contract" => "Porsche — Auftragsmanagement, C++ Linux/Windows",
            "pj_asanet" => "Porsche — ASA-Net, C++ Linux/Windows",
            "pj_lynx" => "Porsche — Lynx, C++ linux/server",
            "pj_sms_email" => "Porsche — SMS-eMail, C++ Linux/Windows",
            "pj_customer_cards" => "Porsche — Kunden-Karten, C++ Linux/Windows",
            "pj_e_billing" => "Porsche — Elektronische Rechnunglegung, C++ Linux/Windows",
            "pj_mein_auto" => "Porsche — Mein-Auto App, C++ linux/server",
            "pj_mobil_car" => "Porsche — Mobile-Fahrzeugannahme, C++ linux/server",
            "pj_supplier_inv" => "Porsche — Eingangsrechnungkontrolle, C++ Linux/Windows",
            "pj_prufloader" => "Sigmatek — industrielles Testtool, C#",
            "pj_vortexledger" => "Bitnök — Blockchain Open Ledger",
            "pj_georoute" => "Telekom — GIS/GeoRoute Administration",
            // Expertise hints
            "ex_admin" => "MOL, Bäko-Hungaria, Telekom",
            "ex_c_win" => "Teamcom, Mediso — Windows-Desktop-Anwendungen",
            "ex_c_embedded" => "Vilati, Bosch — Mikrocontroller-Firmware",
            "ex_cpp_win" => "Mediso, Porsche — C++ Windows GUI",
            "ex_cpp_linux" => "Porsche — C++ Linux Server",
            "ex_csharp" => "Bosch, Sigmatek — .NET Anwendungen",
            "ex_sql" => "Porsche — Datenbanken, gespeicherte Prozeduren",
            "ex_multimedia" => "Teamcom — interaktive Finanzpräsentationen",
            "ex_system_design" => "Vilati — Verkehrssteuerung Systemdesign",
            "ex_project_mgmt" => "Vilati, Bitnök — Projektmanagement",
            "ex_leading" => "Vilati — Abteilungsleitung Softwareentwicklung",
            "ex_manual_test" => "Bosch, Porsche — funktionale Tests, Regressionstests",
            "ex_auto_testing" => "Bosch — automatisiertes HW/SW-Testen",
            "ex_auto_test_dev" => "Bosch — automatisierte Testtool-Entwicklung",
            "ex_test_mgmt" => "Bosch — Teststrategie, Planung, Berichterstattung",
            // Main Characteristics hints
            "mc_strengths" => "Fachkompetenz, Führung und Problemlösung",
            "mc_achievements" => "Test.NET, SK24 Synchronisation, Vortexledger",
            "mc_countries" => "Ungarn, Deutschland, Österreich",
            "mc_languages" => "Ungarisch, Deutsch, Englisch",
            "mc_certificates" => "Diplom, PM, SQL, JavaScript, KI, Fahrtechnik",
            // Tool hints
            "tl_visual_studio" => "Bosch, Vilati, Teamcom, Mediso",
            "tl_ms_office" => "Bosch — Dokumentation, Planung",
            "tl_clearcase" => "Bosch — Versionskontrolle",
            "tl_clearquest" => "Bosch — Fehlerverfolgung",
            "tl_doors" => "Bosch — Anforderungsmanagement",
            "tl_isystem" => "Bosch — Mikrocontroller-Debugging",
            "tl_dspace" => "Bosch — HiL-Testen",
            "tl_ni_teststand" => "Bosch — automatisiertes Testen",
            "tl_ni_cvi" => "Bosch — halbautomatisches Testen",
            "tl_agilent" => "Bosch — Messgeräte für Fensterheber-Test",
            "tl_opentest" => "Bosch — automatisiertes Testtool (jetzt Vector)",
            "tl_authors_dream" => "Teamcom — Multimedia-Editor",
            "tl_authorware" => "Teamcom — Citibank Multimedia",
            "tl_python" => "Bosch — TestDesigner-Skripte",
            "tl_snow" => "Telekom — ITSM-Ticketing",
            "tl_keil" => "Vilati — C167 Mikrocontroller-Compiler",
            "tl_cosmic" => "Bosch — Automotive Mikrocontroller-Compiler",
            "tl_gnu_cc" => "Vilati — OCIT-Firmware unter Linux",
            "tl_tornado" => "Vilati — VxWorks Echtzeit-Entwicklung",
            "tl_java_sdk" => "Vilati — Java Native Interface",
            "tl_structured_text" => "Sigmatek — industrielle Steuerungsprogrammierung",
            "tl_node_js" => "Bitnök — Open-Ledger-Tools",
            _ => "",
        }
    }

    fn toast_update_available(&self) -> &'static str { "Eine neue Version ist verfügbar!" }
    fn btn_update_now(&self) -> &'static str { "Jetzt aktualisieren" }

    fn pdf_filter_choices(&self) -> &'static str { "Filterauswahl" }
    fn btn_generate_pdf(&self) -> &'static str { "PDF erstellen" }

    fn role_section_label(&self) -> &'static str { "Rolle" }
    fn role_label(&self, key: &str) -> &'static str {
        match key {
            "po" => "Product Owner",
            "szm" => "Software-Entwickler",
            "tm" => "Testmanager",
            _ => "",
        }
    }
    fn role_title(&self, key: &str) -> &'static str {
        match key {
            "po" => "Product Owner",
            "szm" => "Software-Entwickler",
            "tm" => "Testmanager",
            _ => "Software-Entwickler",
        }
    }
    fn role_target_title(&self) -> &'static str { "Zielsetzung" }
    fn role_target_text(&self, key: &str) -> &'static str {
        match key {
            "po" => "Als erfahrener Software-Entwickler ist es mein Ziel, meine umfangreiche \
                Produkt- und Prozessmanagement-Erfahrung in der Rolle eines Product Owners \
                einzusetzen. Ich möchte die Benutzererfahrung durch Kundenfeedback-Analyse, \
                agile Entwicklungspraxis und systemisches Denken verbessern.",
            "szm" => "Als erfahrener Software-Entwickler mit über einem Jahrzehnt \
                Branchenerfahrung (insbesondere C, C#, C++, SQL, eingebettete Systeme) \
                möchte ich mein Wissen und meine innovativen Problemlösungsfähigkeiten \
                in einer dynamischen, wachsenden Organisation einsetzen. Mein Ziel ist es, \
                durch die Entwicklung hochwertiger Softwarelösungen zum Erfolg der \
                Organisation beizutragen.",
            "tm" => "Mit jahrzehntelanger Softwareentwicklungserfahrung verfüge ich über \
                umfangreiche Testmanagement-Erfahrung in komplexen Systemen. Meine \
                Expertise umfasst die Planung, Verwaltung und Automatisierung von \
                Testprozessen sowie die Qualitätssicherung. Ich bin versiert in manuellen \
                und automatisierten Testmethoden.",
            _ => "",
        }
    }
    fn role_strengths_title(&self) -> &'static str { "Stärken" }
    fn role_strengths(&self, key: &str) -> &'static [(&'static str, &'static str)] {
        match key {
            "po" => &[
                ("Produkt- & Prozessmanagement",
                 "Erfahrung in der Entwicklung, Unterstützung und Optimierung verschiedener Bank- und Automobilsysteme."),
                ("Analytisches Denken",
                 "Lösung komplexer Software- und hardwarenaher Programmieraufgaben."),
                ("Präzision & Detailgenauigkeit",
                 "Sorgfältige Arbeit und genaue Dokumentation."),
                ("Zusammenarbeit & Kommunikation",
                 "Erfahrung in der Leitung von Entwicklerteams und Zusammenarbeit mit Fachabteilungen."),
                ("Agile Umgebung",
                 "Kenntnisse digitaler Kanäle und agiler Entwicklungsumgebungen."),
            ],
            "szm" => &[
                ("Softwareentwicklung & Testen",
                 "Breite Erfahrung mit C, C++, C# einschließlich Echtzeit-Systemsynchronisation und automatisierter Testframework-Entwicklung."),
                ("Systemdenken",
                 "Maschinenbau-Hintergrund, der hardwarenahe Programmierung und das Verständnis komplexer mechatronischer Systeme unterstützt."),
                ("Projektmanagement & Führung",
                 "Leitung von Softwareentwicklungsabteilungen, Systemtest-Koordination, Scrum- und ISTQB-Methoden."),
                ("Technologische Vielfalt",
                 "Kenntnisse in Linux, Windows, SQL, Webentwicklung und DLT-Konzepten."),
                ("Problemlösung",
                 "Schnelle Auffassungsgabe mit ausgezeichneten Debugging- und Fehlerbehebungsfähigkeiten."),
            ],
            "tm" => &[
                ("Qualitätsorientierter Ansatz",
                 "Verpflichtung zu höchsten Fachstandards (ISTQB, Bosch-interne Standards)."),
                ("Proaktive Problemlösung",
                 "Systematische und beharrliche Ursachenanalyse, Finden optimaler Lösungen oder Kompromisse."),
                ("Prototypen & Frameworks",
                 "Entwurf und Implementierung interner Test-Frameworks (z.B. Test.NET) für die Testautomatisierung."),
                ("Realistische Teststrategie",
                 "Abstimmung der Testumgebungsvalidierung mit der Testergebnisproduktion bei der Planung."),
            ],
            _ => &[],
        }
    }
}
