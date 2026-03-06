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

    fn section_workplaces(&self) -> &'static str { "Workplaces" }
    fn section_job_roles(&self) -> &'static str { "Job Roles" }
    fn section_projects(&self) -> &'static str { "Projects" }
    fn section_expertise(&self) -> &'static str { "Professional Skills" }

    fn section_main_chars(&self) -> &'static str { "My Main Characteristics" }
    fn role_achievements_title(&self) -> &'static str { "Key Achievements" }
    fn role_achievements(&self) -> &'static [(&'static str, &'static str)] {
        &[
            ("Test.NET test development framework",
             "Later sold by Bosch to Vector."),
            ("SK24 lamp switch card synchronization",
             "Real-time synchronization of the traffic control lamp switch card's internal processes to the electrical grid's sine waves."),
            ("Vortexledger concept",
             "An open ledger and economy concept based on barter trade."),
        ]
    }

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
            "jr_sw_dev" => "Software Developer",
            "jr_dept_head" => "Head of Department",
            "jr_tester" => "Tester",
            "jr_test_mgr" => "Test Manager",
            "jr_test_dev" => "Test Developer",
            "jr_project_mgr" => "Project Manager",
            // Projects (Sankey)
            "pj_labor" => "Laboratory",
            "pj_erp_system" => "ERP System",
            "pj_db_finance" => "Deutsche Bank Building Financing",
            "pj_cwl_kwg" => "C&L Deutsche Revision KWG Novelle",
            "pj_authors_dream" => "Author's Dream",
            "pj_citibank" => "Citibank Anti-Money Laundering",
            "pj_junction" => "Junction Control Tool",
            "pj_btc5000" => "BTC5000",
            "pj_sk24" => "SK24",
            "pj_ocit" => "OCIT",
            "pj_debrecen" => "Debrecen",
            "pj_medical" => "Medical Imaging",
            "pj_car_body" => "Car Body Computer",
            "pj_dcdc" => "DC/DC Converter",
            "pj_erp_bosch" => "ERP / Resource Planning",
            "pj_test_designer" => "TestDesigner",
            "pj_test_net" => "Test.NET",
            "pj_truck_body" => "Truck Body Computer",
            "pj_test_tools" => "Improvement of Test Tools",
            "pj_window_lifter" => "Window Lifter",
            "pj_contract" => "Contract (Auftrag)",
            "pj_asanet" => "ASA-Net",
            "pj_lynx" => "Lynx",
            "pj_sms_email" => "SMS-eMail",
            "pj_customer_cards" => "Customer Cards",
            "pj_e_billing" => "Electronic Billing",
            "pj_mein_auto" => "Mein-Auto App",
            "pj_mobil_car" => "Mobile Car Takeover",
            "pj_supplier_inv" => "Supplier Invoice Control",
            "pj_prufloader" => "Prüfloader.NET",
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
            "ex_multimedia" => "Multimedia Programming",
            "ex_system_design" => "System Design",
            "ex_project_mgmt" => "Project Management",
            "ex_leading" => "Leadership",
            "ex_manual_test" => "Manual Testing",
            "ex_auto_testing" => "Automated Testing",
            "ex_auto_test_dev" => "Automated Test Development",
            "ex_test_mgmt" => "Test Management",
            // Main Characteristics
            "mc_strengths" => "Strengths",
            "mc_achievements" => "Key Achievements",
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
            // Workplace hints
            "wk_mol" => "Laboratory system administration",
            "wk_bako" => "ERP system administration",
            "wk_teamcom" => "Financial multimedia software — Deutsche Bank, Citibank, C&L",
            "wk_vilati" => "Traffic junction control, embedded C, department leadership",
            "wk_mediso" => "Medical image processing",
            "wk_bosch" => "Automotive ECU development & test — Body Computers, Window Lifters",
            "wk_porsche" => "Dealer management systems — C++ Linux/Windows, SQL",
            "wk_sigmatek" => "Prüfloader.NET — industrial test tool in C#",
            "wk_bitnok" => "Blockchain project management — VortexLedger",
            "wk_telekom" => "GIS / GeoRoute administration",
            // Job role hints
            "jr_admin" => "MOL, Bäko-Hungaria, Telekom",
            "jr_sw_dev" => "Teamcom, Vilati, Mediso, Bosch, Porsche, Sigmatek",
            "jr_dept_head" => "Vilati — OCIT, Debrecen",
            "jr_tester" => "Bosch, Porsche Informatik",
            "jr_test_mgr" => "Bosch — Truck Body Computer, test tools",
            "jr_test_dev" => "Bosch — Window Lifter, Truck Body Computer",
            "jr_project_mgr" => "Bitnök — Vortexledger",
            // Project hints
            "pj_labor" => "MOL AG — laboratory system",
            "pj_erp_system" => "Bäko-Hungaria — enterprise resource planning",
            "pj_db_finance" => "Teamcom — Deutsche Bank building financing",
            "pj_cwl_kwg" => "Teamcom — C&L Deutsche Revision",
            "pj_authors_dream" => "Teamcom — multimedia editor",
            "pj_citibank" => "Teamcom — anti-money laundering",
            "pj_junction" => "Vilati — traffic junction control",
            "pj_btc5000" => "Vilati — traffic controller",
            "pj_sk24" => "Vilati — lamp switch card, real-time sync",
            "pj_ocit" => "Vilati — department leadership, OCIT standard",
            "pj_debrecen" => "Vilati — department leadership, Debrecen project",
            "pj_medical" => "Mediso — medical image processing",
            "pj_car_body" => "Bosch — car Body Computer, C embedded",
            "pj_dcdc" => "Bosch — DC/DC converter, C embedded",
            "pj_erp_bosch" => "Bosch — resource planning, C#",
            "pj_test_designer" => "Bosch — test designer tool, C#",
            "pj_test_net" => "Bosch — test development framework, C#",
            "pj_truck_body" => "Bosch — Truck Body Computer, test management",
            "pj_test_tools" => "Bosch — improvement of test tools",
            "pj_window_lifter" => "Bosch — window lifter, automated testing",
            "pj_contract" => "Porsche — contract management, C++ Linux/Windows",
            "pj_asanet" => "Porsche — ASA-Net, C++ Linux/Windows",
            "pj_lynx" => "Porsche — Lynx, C++ linux/server",
            "pj_sms_email" => "Porsche — SMS-eMail, C++ Linux/Windows",
            "pj_customer_cards" => "Porsche — customer cards, C++ Linux/Windows",
            "pj_e_billing" => "Porsche — electronic billing, C++ Linux/Windows",
            "pj_mein_auto" => "Porsche — Mein-Auto App, C++ linux/server",
            "pj_mobil_car" => "Porsche — mobile car takeover, C++ linux/server",
            "pj_supplier_inv" => "Porsche — supplier invoices, C++ Linux/Windows",
            "pj_prufloader" => "Sigmatek — industrial test tool, C#",
            "pj_vortexledger" => "Bitnök — blockchain open ledger",
            "pj_georoute" => "Telekom — GIS/GeoRoute administration",
            // Expertise hints
            "ex_admin" => "MOL, Bäko-Hungaria, Telekom",
            "ex_c_win" => "Teamcom, Mediso — Windows desktop applications",
            "ex_c_embedded" => "Vilati, Bosch — microcontroller firmware",
            "ex_cpp_win" => "Mediso, Porsche — C++ Windows GUI",
            "ex_cpp_linux" => "Porsche — C++ Linux server",
            "ex_csharp" => "Bosch, Sigmatek — .NET applications",
            "ex_sql" => "Porsche — databases, stored procedures",
            "ex_multimedia" => "Teamcom — interactive financial presentations",
            "ex_system_design" => "Vilati — traffic control system design",
            "ex_project_mgmt" => "Vilati, Bitnök — project management",
            "ex_leading" => "Vilati — software development department leadership",
            "ex_manual_test" => "Bosch, Porsche — functional and regression testing",
            "ex_auto_testing" => "Bosch — automated HW/SW testing",
            "ex_auto_test_dev" => "Bosch — automated test tool development",
            "ex_test_mgmt" => "Bosch — test strategy, planning and reporting",
            // Main Characteristics hints
            "mc_strengths" => "Expertise, leadership and problem solving",
            "mc_achievements" => "Test.NET, SK24 synchronization, Vortexledger",
            _ => "",
        }
    }

    fn toast_update_available(&self) -> &'static str { "A new version is available!" }
    fn btn_update_now(&self) -> &'static str { "Update Now" }

    fn pdf_filter_choices(&self) -> &'static str { "Filter Choices" }
    fn btn_generate_pdf(&self) -> &'static str { "Generate PDF" }

    fn role_section_label(&self) -> &'static str { "Role" }
    fn role_label(&self, key: &str) -> &'static str {
        match key {
            "po" => "Product Owner",
            "szm" => "Software Engineer",
            "tm" => "Test Manager",
            _ => "",
        }
    }
    fn role_title(&self, key: &str) -> &'static str {
        match key {
            "po" => "Product Owner",
            "szm" => "Software Engineer",
            "tm" => "Test Manager",
            _ => "Software Engineer",
        }
    }
    fn role_target_title(&self) -> &'static str { "Target" }
    fn role_target_text(&self, key: &str) -> &'static str {
        match key {
            "po" => "As an experienced software engineer, my goal is to leverage my extensive \
                product and process management experience in a product owner role. \
                I aim to improve user experience through customer feedback analysis, \
                agile development practices, and system-level thinking.",
            "szm" => "As an experienced software engineer with over a decade of industry \
                experience (especially in C, C#, C++, SQL, embedded systems), I want to \
                apply my knowledge and innovative problem-solving skills in a dynamic, \
                growing organization. My goal is to contribute to the organization's \
                success by developing high-quality software solutions while continuously \
                expanding my expertise in new technologies.",
            "tm" => "With decades of software development background, I have extensive \
                test management experience in complex systems. My expertise covers \
                planning, managing and automating testing processes, as well as quality \
                assurance. I am proficient in manual and automated testing methodologies.",
            _ => "",
        }
    }
    fn role_strengths_title(&self) -> &'static str { "Strengths" }
    fn role_strengths(&self, key: &str) -> &'static [(&'static str, &'static str)] {
        match key {
            "po" => &[
                ("Product & Process Management",
                 "Experience in developing, supporting and optimizing various banking and automotive systems."),
                ("Analytical Thinking",
                 "Solving complex software and hardware-level programming tasks."),
                ("Precision & Attention to Detail",
                 "Meticulous work and accurate documentation."),
                ("Collaboration & Communication",
                 "Experience leading development teams and cooperating with cross-functional departments."),
                ("Agile Environment",
                 "Knowledge of digital channels and agile development environments."),
            ],
            "szm" => &[
                ("Software Development & Testing",
                 "Broad experience with C, C++, C# including real-time system synchronization and automated test framework development."),
                ("System-Level Thinking",
                 "Mechanical engineering background supporting hardware-level programming and understanding complex mechatronic systems."),
                ("Project Management & Leadership",
                 "Leading software development departments, system test coordination, Scrum and ISTQB methodologies."),
                ("Technological Diversity",
                 "Proficiency in Linux, Windows, SQL, web development and DLT concepts."),
                ("Problem Solving",
                 "Fast learner with excellent debugging and troubleshooting skills."),
            ],
            "tm" => &[
                ("Quality-Oriented Approach",
                 "Commitment to highest professional standards (ISTQB, Bosch internal standards)."),
                ("Proactive Problem Solving",
                 "Systematic and persistent root cause analysis, finding optimal solutions or compromises."),
                ("Prototypes & Frameworks",
                 "Designing and implementing internal test frameworks (e.g. Test.NET) for test automation."),
                ("Realistic Test Strategy",
                 "Aligning test environment validation with test result production timelines during planning."),
            ],
            _ => &[],
        }
    }
}
