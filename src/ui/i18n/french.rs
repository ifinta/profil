use super::UiI18n;

pub struct FrenchUi;

impl UiI18n for FrenchUi {
    fn tab_profile(&self) -> &'static str { "Profil" }
    fn tab_filter(&self) -> &'static str { "Filtre" }
    fn tab_display(&self) -> &'static str { "Affichage" }

    fn profile_name(&self) -> &'static str { "István Finta" }
    fn profile_subtitle(&self) -> &'static str { "Ingénieur diplômé en génie mécanique, robotique et informatique" }
    fn profile_title(&self) -> &'static str { "Ingénieur logiciel" }
    fn profile_location(&self) -> &'static str { "Hongrie" }
    fn profile_email(&self) -> &'static str { "istvan_finta@yahoo.com" }
    fn profile_phone(&self) -> &'static str { "+36 70 343 9517" }

    fn section_skills(&self) -> &'static str { "Compétences techniques" }
    fn section_countries(&self) -> &'static str { "Pays" }
    fn section_languages(&self) -> &'static str { "Compétences linguistiques" }
    fn section_companies(&self) -> &'static str { "Entreprises" }
    fn section_certificates(&self) -> &'static str { "Certificats" }
    fn filter_select_all(&self) -> &'static str { "Tout sélectionner" }
    fn filter_clear_all(&self) -> &'static str { "Tout effacer" }

    fn section_job_roles(&self) -> &'static str { "Postes" }
    fn section_projects(&self) -> &'static str { "Projets" }
    fn section_tools(&self) -> &'static str { "Outils" }
    fn section_project_experience(&self) -> &'static str { "Expérience projet" }

    fn section_main_chars(&self) -> &'static str { "Mes principales caractéristiques" }
    fn role_achievements_title(&self) -> &'static str { "Réalisations clés" }
    fn digital_skills_title(&self) -> &'static str { "Compétences numériques" }
    fn digital_skills_text(&self) -> &'static str {
        "Complètes.\n\n\
         OS : Windows, Debian Linux, iOS, Android, OSX, VxWorks, ...\n\
         Outils de développement, makefiles, compilateurs (C, C#, C++, un peu de Rust, ...), \
         interpréteurs (Python, ...), éditeurs de texte (nano, kate, MS Code, ...)\n\
         MS Office, Word, Excel, ... (Libre Office et similaires également), Navigateurs, \
         Applications web, Fournisseurs cloud (Vultr, Oracle, ...)\n\
         Applications graphiques, OpenSCAD (conception 3D), Eagle (conception de circuits), \
         Gimp, outils de capture d'écran, outils de retouche photo, ...\n\
         Connaissances de base en IA, connaissances avancées en DLT (Distributed Ledger \
         Technology), Stellar Network, smart contracts, ... Développement web de base, \
         HTML5, CSS, JavaScript, un peu de React, un peu de NodeJS, ...\n\n\
         Je maîtrise rapidement les applications, langages de programmation et systèmes d'exploitation."
    }
    fn role_achievements(&self) -> &'static [(&'static str, &'static str)] {
        &[
            ("Framework de développement de tests Test.NET",
             "Vendu ultérieurement par Bosch à Vector."),
            ("Synchronisation de la carte de commutation de lampes SK24",
             "Synchronisation en temps réel des processus internes de la carte de commutation de lampes de contrôle du trafic avec les ondes sinusoïdales du réseau électrique."),
            ("Concept Vortexledger",
             "Un concept de registre ouvert et d'économie basé sur le troc."),
        ]
    }

    fn display_nothing_selected(&self) -> &'static str { "Sélectionnez des éléments dans l'onglet Filtre pour voir les détails ici." }

    fn item_label(&self, key: &str) -> &'static str {
        match key {
            // Skills
            "c_embedded" => "C (embarqué)",
            "c_win_gui" => "C (Windows/GUI)",
            "cpp_win_gui" => "C++ (Windows/GUI)",
            "cpp_linux_server" => "C++ (Linux/Serveur)",
            "csharp" => "C#",
            "sql" => "SQL",
            "rust" => "Rust / WebAssembly",
            "multimedia" => "Programmation multimédia",
            "system_design" => "Conception de systèmes",
            "project_management" => "Gestion de projet",
            "test_management" => "Gestion de tests",
            "automated_testing" => "Tests automatisés",
            "manual_testing" => "Tests manuels",
            "erp" => "Systèmes ERP",
            "administration" => "Administration système",
            "leading" => "Direction",
            "auto_test_dev" => "Développement de tests automatisés",
            // Countries
            "hungary" => "Hongrie",
            "germany" => "Allemagne",
            "austria" => "Autriche",
            // Languages
            "lang_hungarian" => "Hongrois",
            "lang_german" => "Allemand",
            "lang_english" => "Anglais",
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
            "cert_diploma" => "Diplôme universitaire",
            "cert_pm" => "Certificat PM (PMCC)",
            "cert_sql" => "Bases de données et SQL — Fondamentaux",
            "cert_js" => "Certificat JavaScript",
            "cert_ai" => "Programmation avec des agents IA",
            "cert_driving" => "Technique de conduite avancée",
            "cert_scrum" => "Scrum",
            "cert_istqb" => "Séminaire ISTQB",
            "cert_presentation" => "Techniques de présentation",
            "cert_access" => "MS Access 2003 (intermédiaire)",
            "cert_canoe" => "Bases de CANoe",
            "cert_dotnet" => "Cours .NET, C#",
            "cert_moderation" => "Modération — bases",
            "cert_daf" => "Allemand langue étrangère",
            "cert_germanistik" => "Semestre international d'été en études germaniques",
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
            "jr_admin" => "Administrateur",
            "jr_sw_dev" => "Développeur logiciel",
            "jr_dept_head" => "Chef de département",
            "jr_tester" => "Testeur",
            "jr_test_mgr" => "Responsable de tests",
            "jr_test_dev" => "Développeur de tests",
            "jr_project_mgr" => "Chef de projet",
            // Projects (Sankey)
            "pj_labor" => "Laboratoire",
            "pj_erp_system" => "Système ERP",
            "pj_db_finance" => "Deutsche Bank Financement immobilier",
            "pj_cwl_kwg" => "C&L Deutsche Revision KWG Novelle",
            "pj_authors_dream" => "Author's Dream",
            "pj_citibank" => "Citibank Anti-blanchiment",
            "pj_junction" => "Outil de contrôle de carrefour",
            "pj_btc5000" => "BTC5000",
            "pj_sk24" => "SK24",
            "pj_ocit" => "OCIT",
            "pj_debrecen" => "Debrecen",
            "pj_medical" => "Imagerie médicale",
            "pj_car_body" => "Ordinateur de bord voiture",
            "pj_dcdc" => "Convertisseur DC/DC",
            "pj_erp_bosch" => "ERP / Planification des ressources",
            "pj_test_designer" => "TestDesigner",
            "pj_test_net" => "Test.NET",
            "pj_truck_body" => "Ordinateur de bord camion",
            "pj_test_tools" => "Amélioration des outils de test",
            "pj_window_lifter" => "Lève-vitre",
            "pj_contract" => "Contrat (Auftrag)",
            "pj_asanet" => "ASA-Net",
            "pj_lynx" => "Lynx",
            "pj_sms_email" => "SMS-eMail",
            "pj_customer_cards" => "Cartes clients",
            "pj_e_billing" => "Facturation électronique",
            "pj_mein_auto" => "Mein-Auto App",
            "pj_mobil_car" => "Prise en charge mobile de véhicule",
            "pj_supplier_inv" => "Contrôle des factures fournisseurs",
            "pj_prufloader" => "Prüfloader.NET",
            "pj_vortexledger" => "Vortexledger",
            "pj_georoute" => "GIS GeoRoute",
            // Expertise (Sankey)
            "ex_admin" => "Administration",
            "ex_c_win" => "C win/GUI",
            "ex_c_embedded" => "C embarqué",
            "ex_cpp_win" => "C++ win/GUI",
            "ex_cpp_linux" => "C++ linux/serveur",
            "ex_csharp" => "C#",
            "ex_sql" => "SQL",
            "ex_multimedia" => "Programmation multimédia",
            "ex_system_design" => "Conception de systèmes",
            "ex_project_mgmt" => "Gestion de projet",
            "ex_leading" => "Direction",
            "ex_manual_test" => "Tests manuels",
            "ex_auto_testing" => "Tests automatisés",
            "ex_auto_test_dev" => "Développement de tests automatisés",
            "ex_test_mgmt" => "Gestion de tests",
            // Main Characteristics
            "mc_strengths" => "Points forts",
            "mc_achievements" => "Réalisations clés",
            "mc_countries" => "Pays",
            "mc_languages" => "Compétences linguistiques",
            "mc_certificates" => "Certificats",
            "mc_digital_skills" => "Compétences numériques",
            // Tools
            "tl_visual_studio" => "Visual Studio",
            "tl_ms_office" => "MS Office (Excel, Word, Access)",
            "tl_clearcase" => "IBM ClearCase",
            "tl_clearquest" => "IBM ClearQuest",
            "tl_doors" => "IBM Doors",
            "tl_isystem" => "iSYSTEM Debugger",
            "tl_dspace" => "dSPACE Tour de test",
            "tl_ni_teststand" => "NI TestStand",
            "tl_ni_cvi" => "NI LabWindows/CVI",
            "tl_agilent" => "Instruments Agilent",
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
            "c_embedded" => "Firmware microcontrôleur pour systèmes de trafic et ECU automobiles",
            "c_win_gui" => "Applications de bureau Windows en C",
            "cpp_win_gui" => "Applications GUI Windows en C++",
            "cpp_linux_server" => "Développement serveur Linux en C++",
            "csharp" => "C# (.NET) — outils de test, ERP, Prüfloader",
            "sql" => "Requêtes de bases de données, procédures stockées",
            "rust" => "Rust + Dioxus + WebAssembly pour applications web",
            "multimedia" => "Multimédia et présentations interactives",
            "system_design" => "Architecture et conception de systèmes pour le contrôle du trafic",
            "project_management" => "Direction d'équipe et gestion de projet",
            "test_management" => "Stratégie de test, planification et reporting",
            "automated_testing" => "Développement de tests HW/SW automatisés",
            "manual_testing" => "Tests fonctionnels et de régression",
            "erp" => "Systèmes de planification des ressources d'entreprise",
            "administration" => "Administration système et réseau",
            "leading" => "Vilati — direction du département développement logiciel",
            "auto_test_dev" => "Bosch — développement d'outils de test automatisés",
            "hungary" => "Vilati, Mediso, MOL, Bäko-Hungaria, Bitnök, Telekom",
            "germany" => "Teamcom, Bosch",
            "austria" => "Porsche Informatik, Sigmatek",
            "lang_hungarian" => "Langue maternelle",
            "lang_german" => "Était courant, longtemps inutilisé",
            "lang_english" => "Conversation de base, littérature technique, e-mail, chat",
            "mol" => "Administration du système de laboratoire",
            "bako" => "Administration du système ERP",
            "teamcom" => "Logiciel multimédia financier — Deutsche Bank, Citibank, C&L",
            "vilati" => "Contrôle de carrefour, C embarqué, direction de département",
            "mediso" => "Traitement d'images médicales",
            "bosch" => "Développement et test ECU automobile — Body Computers, lève-vitres",
            "porsche" => "Systèmes de gestion de concessionnaires — C++ Linux/Windows, SQL",
            "sigmatek" => "Prüfloader.NET — outil de test industriel en C#",
            "bitnok" => "Gestion de projet blockchain — VortexLedger",
            "telekom" => "GIS / GeoRoute administration",
            "cert_diploma" => "Diplôme universitaire d'ingénieur",
            "cert_pm" => "Certificat de gestion de projet — PMCC",
            "cert_sql" => "Cours de fondamentaux bases de données et SQL",
            "cert_js" => "Certificat de développement JavaScript",
            "cert_ai" => "Cours de programmation avec des agents IA",
            "cert_driving" => "Technique de conduite avancée pour la sécurité",
            "cert_scrum" => "Scrum — cours interne PORSCHE",
            "cert_istqb" => "Séminaire ISTQB — cours interne Bosch",
            "cert_presentation" => "Techniques de présentation — cours Develor",
            "cert_access" => "MS Access 2003 (intermédiaire) — Controll Training",
            "cert_canoe" => "Bases de CANoe — VECTOR",
            "cert_dotnet" => "Cours .NET, C# — BME",
            "cert_moderation" => "Modération — bases — Bosch interne",
            "cert_daf" => "Allemand langue étrangère — Université Friedrich Schiller Iéna (bourse TEMPUS)",
            "cert_germanistik" => "Semestre international d'été en études germaniques — FSU Iéna",
            // Workplace hints
            "wk_mol" => "Administration du système de laboratoire",
            "wk_bako" => "Administration du système ERP",
            "wk_teamcom" => "Logiciel multimédia financier — Deutsche Bank, Citibank, C&L",
            "wk_vilati" => "Contrôle de carrefour, C embarqué, direction de département",
            "wk_mediso" => "Traitement d'images médicales",
            "wk_bosch" => "Développement et test ECU automobile — Body Computers, lève-vitres",
            "wk_porsche" => "Systèmes de gestion de concessionnaires — C++ Linux/Windows, SQL",
            "wk_sigmatek" => "Prüfloader.NET — outil de test industriel en C#",
            "wk_bitnok" => "Gestion de projet blockchain — VortexLedger",
            "wk_telekom" => "GIS / GeoRoute administration",
            // Job role hints
            "jr_admin" => "MOL, Bäko-Hungaria, Telekom",
            "jr_sw_dev" => "Teamcom, Vilati, Mediso, Bosch, Porsche, Sigmatek",
            "jr_dept_head" => "Vilati — OCIT, Debrecen",
            "jr_tester" => "Bosch, Porsche Informatik",
            "jr_test_mgr" => "Bosch — Ordinateur de bord camion, outils de test",
            "jr_test_dev" => "Bosch — Lève-vitre, Ordinateur de bord camion",
            "jr_project_mgr" => "Bitnök — Vortexledger",
            // Project hints
            "pj_labor" => "MOL AG — système de laboratoire",
            "pj_erp_system" => "Bäko-Hungaria — planification des ressources d'entreprise",
            "pj_db_finance" => "Teamcom — Deutsche Bank financement immobilier",
            "pj_cwl_kwg" => "Teamcom — C&L Deutsche Revision",
            "pj_authors_dream" => "Teamcom — éditeur multimédia",
            "pj_citibank" => "Teamcom — anti-blanchiment d'argent",
            "pj_junction" => "Vilati — contrôle de carrefour",
            "pj_btc5000" => "Vilati — contrôleur de trafic",
            "pj_sk24" => "Vilati — carte de commutation de lampes, synchronisation temps réel",
            "pj_ocit" => "Vilati — direction de département, standard OCIT",
            "pj_debrecen" => "Vilati — direction de département, projet Debrecen",
            "pj_medical" => "Mediso — traitement d'images médicales",
            "pj_car_body" => "Bosch — ordinateur de bord voiture, C embarqué",
            "pj_dcdc" => "Bosch — convertisseur DC/DC, C embarqué",
            "pj_erp_bosch" => "Bosch — planification des ressources, C#",
            "pj_test_designer" => "Bosch — outil de conception de tests, C#",
            "pj_test_net" => "Bosch — framework de développement de tests, C#",
            "pj_truck_body" => "Bosch — ordinateur de bord camion, gestion de tests",
            "pj_test_tools" => "Bosch — amélioration des outils de test",
            "pj_window_lifter" => "Bosch — lève-vitre, tests automatisés",
            "pj_contract" => "Porsche — gestion de contrats, C++ Linux/Windows",
            "pj_asanet" => "Porsche — ASA-Net, C++ Linux/Windows",
            "pj_lynx" => "Porsche — Lynx, C++ linux/serveur",
            "pj_sms_email" => "Porsche — SMS-eMail, C++ Linux/Windows",
            "pj_customer_cards" => "Porsche — cartes clients, C++ Linux/Windows",
            "pj_e_billing" => "Porsche — facturation électronique, C++ Linux/Windows",
            "pj_mein_auto" => "Porsche — Mein-Auto App, C++ linux/serveur",
            "pj_mobil_car" => "Porsche — prise en charge mobile de véhicule, C++ linux/serveur",
            "pj_supplier_inv" => "Porsche — contrôle factures fournisseurs, C++ Linux/Windows",
            "pj_prufloader" => "Sigmatek — outil de test industriel, C#",
            "pj_vortexledger" => "Bitnök — registre ouvert blockchain",
            "pj_georoute" => "Telekom — administration GIS/GeoRoute",
            // Expertise hints
            "ex_admin" => "MOL, Bäko-Hungaria, Telekom",
            "ex_c_win" => "Teamcom, Mediso — applications de bureau Windows",
            "ex_c_embedded" => "Vilati, Bosch — firmware microcontrôleur",
            "ex_cpp_win" => "Mediso, Porsche — C++ Windows GUI",
            "ex_cpp_linux" => "Porsche — C++ serveur Linux",
            "ex_csharp" => "Bosch, Sigmatek — applications .NET",
            "ex_sql" => "Porsche — bases de données, procédures stockées",
            "ex_multimedia" => "Teamcom — présentations financières interactives",
            "ex_system_design" => "Vilati — conception de systèmes de contrôle du trafic",
            "ex_project_mgmt" => "Vilati, Bitnök — gestion de projet",
            "ex_leading" => "Vilati — direction du département développement logiciel",
            "ex_manual_test" => "Bosch, Porsche — tests fonctionnels et de régression",
            "ex_auto_testing" => "Bosch — tests HW/SW automatisés",
            "ex_auto_test_dev" => "Bosch — développement d'outils de test automatisés",
            "ex_test_mgmt" => "Bosch — stratégie de test, planification et reporting",
            // Main Characteristics hints
            "mc_strengths" => "Expertise, leadership et résolution de problèmes",
            "mc_achievements" => "Test.NET, synchronisation SK24, Vortexledger",
            "mc_countries" => "Hongrie, Allemagne, Autriche",
            "mc_languages" => "Hongrois, allemand, anglais",
            "mc_certificates" => "Diplôme, PM, SQL, JavaScript, IA, conduite",
            "mc_digital_skills" => "Compétences numériques complètes",
            // Tool hints
            "tl_visual_studio" => "Bosch, Vilati, Teamcom, Mediso",
            "tl_ms_office" => "Bosch — documentation, planification",
            "tl_clearcase" => "Bosch — contrôle de version",
            "tl_clearquest" => "Bosch — suivi de bogues",
            "tl_doors" => "Bosch — gestion des exigences",
            "tl_isystem" => "Bosch — débogage microcontrôleur",
            "tl_dspace" => "Bosch — tests HiL",
            "tl_ni_teststand" => "Bosch — tests automatisés",
            "tl_ni_cvi" => "Bosch — tests semi-automatiques",
            "tl_agilent" => "Bosch — instruments pour test de lève-vitre",
            "tl_opentest" => "Bosch — outil de test automatisé (maintenant Vector)",
            "tl_authors_dream" => "Teamcom — éditeur multimédia",
            "tl_authorware" => "Teamcom — Citibank multimédia",
            "tl_python" => "Bosch — scripts TestDesigner",
            "tl_snow" => "Telekom — gestion de tickets ITSM",
            "tl_keil" => "Vilati — compilateur microcontrôleur C167",
            "tl_cosmic" => "Bosch — compilateur microcontrôleur automobile",
            "tl_gnu_cc" => "Vilati — firmware OCIT sous Linux",
            "tl_tornado" => "Vilati — développement temps réel VxWorks",
            "tl_java_sdk" => "Vilati — Java Native Interface",
            "tl_structured_text" => "Sigmatek — programmation de contrôleurs industriels",
            "tl_node_js" => "Bitnök — outils de registre ouvert",
            _ => "",
        }
    }

    fn btn_generate_pdf(&self) -> &'static str { "Générer le PDF" }

    fn role_section_label(&self) -> &'static str { "Rôle" }
    fn role_label(&self, key: &str) -> &'static str {
        match key {
            "po" => "Product Owner",
            "szm" => "Ingénieur logiciel",
            "tm" => "Responsable de tests",
            _ => "",
        }
    }
    fn role_title(&self, key: &str) -> &'static str {
        match key {
            "po" => "Product Owner",
            "szm" => "Ingénieur logiciel",
            "tm" => "Responsable de tests",
            _ => "Ingénieur logiciel",
        }
    }
    fn role_target_title(&self) -> &'static str { "Objectif" }
    fn role_target_text(&self, key: &str) -> &'static str {
        match key {
            "po" => "En tant qu'ingénieur logiciel expérimenté, mon objectif est de mettre à \
                profit ma vaste expérience en gestion de produits et de processus dans un rôle \
                de Product Owner. Je vise à améliorer l'expérience utilisateur par l'analyse \
                des retours clients, les pratiques de développement agile et la pensée systémique.",
            "szm" => "En tant qu'ingénieur logiciel expérimenté avec plus d'une décennie \
                d'expérience industrielle (notamment en C, C#, C++, SQL, systèmes embarqués), \
                je souhaite appliquer mes connaissances et mes compétences innovantes en \
                résolution de problèmes dans une organisation dynamique et en croissance. \
                Mon objectif est de contribuer au succès de l'organisation en développant \
                des solutions logicielles de haute qualité tout en élargissant continuellement \
                mon expertise dans les nouvelles technologies.",
            "tm" => "Avec des décennies d'expérience en développement logiciel, je possède \
                une vaste expérience en gestion de tests dans des systèmes complexes. Mon \
                expertise couvre la planification, la gestion et l'automatisation des processus \
                de test, ainsi que l'assurance qualité. Je maîtrise les méthodologies de test \
                manuelles et automatisées.",
            _ => "",
        }
    }
    fn role_strengths_title(&self) -> &'static str { "Points forts" }
    fn role_strengths(&self, key: &str) -> &'static [(&'static str, &'static str)] {
        match key {
            "po" => &[
                ("Gestion de produits et de processus",
                 "Expérience dans le développement, le support et l'optimisation de divers systèmes bancaires et automobiles."),
                ("Pensée analytique",
                 "Résolution de tâches de programmation logicielle et matérielle complexes."),
                ("Précision et attention au détail",
                 "Travail méticuleux et documentation précise."),
                ("Collaboration et communication",
                 "Expérience dans la direction d'équipes de développement et la coopération avec des départements transversaux."),
                ("Environnement agile",
                 "Connaissance des canaux numériques et des environnements de développement agile."),
            ],
            "szm" => &[
                ("Développement et test logiciel",
                 "Large expérience avec C, C++, C# incluant la synchronisation de systèmes temps réel et le développement de frameworks de test automatisés."),
                ("Pensée systémique",
                 "Formation en génie mécanique soutenant la programmation au niveau matériel et la compréhension de systèmes mécatroniques complexes."),
                ("Gestion de projet et leadership",
                 "Direction de départements de développement logiciel, coordination de tests système, méthodologies Scrum et ISTQB."),
                ("Diversité technologique",
                 "Maîtrise de Linux, Windows, SQL, développement web et concepts DLT."),
                ("Résolution de problèmes",
                 "Apprentissage rapide avec d'excellentes compétences en débogage et dépannage."),
            ],
            "tm" => &[
                ("Approche orientée qualité",
                 "Engagement envers les plus hauts standards professionnels (ISTQB, standards internes Bosch)."),
                ("Résolution proactive de problèmes",
                 "Analyse systématique et persistante des causes profondes, recherche de solutions optimales ou de compromis."),
                ("Prototypes et frameworks",
                 "Conception et implémentation de frameworks de test internes (ex. Test.NET) pour l'automatisation des tests."),
                ("Stratégie de test réaliste",
                 "Alignement de la validation de l'environnement de test avec les délais de production des résultats de test lors de la planification."),
            ],
            _ => &[],
        }
    }
    fn pwa_hint(&self) -> &'static str {
        "Ceci est une application PWA \u{2014} vous pouvez l\u{2019}enregistrer depuis votre navigateur sur l\u{2019}écran d\u{2019}accueil de votre téléphone comme une application native"
    }
}
