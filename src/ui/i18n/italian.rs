use super::UiI18n;

pub struct ItalianUi;

impl UiI18n for ItalianUi {
    fn tab_profile(&self) -> &'static str { "Profilo" }
    fn tab_filter(&self) -> &'static str { "Filtro" }
    fn tab_display(&self) -> &'static str { "Visualizzazione" }

    fn profile_name(&self) -> &'static str { "István Finta" }
    fn profile_subtitle(&self) -> &'static str { "Ingegnere meccanico, robotica e informatica" }
    fn profile_title(&self) -> &'static str { "Ingegnere del software" }
    fn profile_location(&self) -> &'static str { "Ungheria" }
    fn profile_email(&self) -> &'static str { "istvan_finta@yahoo.com" }
    fn profile_phone(&self) -> &'static str { "+36 70 343 9517" }

    fn section_skills(&self) -> &'static str { "Competenze tecniche" }
    fn section_countries(&self) -> &'static str { "Paesi" }
    fn section_languages(&self) -> &'static str { "Competenze linguistiche" }
    fn section_companies(&self) -> &'static str { "Aziende" }
    fn section_certificates(&self) -> &'static str { "Certificati" }
    fn filter_select_all(&self) -> &'static str { "Seleziona tutto" }
    fn filter_clear_all(&self) -> &'static str { "Cancella tutto" }

    fn section_job_roles(&self) -> &'static str { "Ruoli lavorativi" }
    fn section_projects(&self) -> &'static str { "Progetti" }
    fn section_tools(&self) -> &'static str { "Strumenti" }
    fn section_project_experience(&self) -> &'static str { "Esperienza nei progetti" }

    fn section_main_chars(&self) -> &'static str { "Le mie caratteristiche principali" }
    fn role_achievements_title(&self) -> &'static str { "Risultati chiave" }
    fn digital_skills_title(&self) -> &'static str { "Competenze digitali" }
    fn digital_skills_text(&self) -> &'static str {
        "Complete.\n\n\
         SO: Windows, Debian Linux, iOS, Android, OSX, VxWorks, ...\n\
         Strumenti di sviluppo, makefile, compilatori (C, C#, C++, un po' di Rust, ...), \
         interpreti (Python, ...), editor di testo (nano, kate, MS Code, ...)\n\
         MS Office, Word, Excel, ... (Libre Office e simili), Browser, Applicazioni web, \
         Provider cloud (Vultr, Oracle, ...)\n\
         Applicazioni grafiche, OpenSCAD (progettazione 3D), Eagle (progettazione circuiti), \
         Gimp, strumenti di cattura schermo, strumenti di elaborazione foto, ...\n\
         Conoscenze di base sull'IA, conoscenze avanzate DLT (Distributed Ledger Technology), \
         Stellar Network, smart contract, ... Sviluppo web di base, HTML5, CSS, JavaScript, \
         un po' di React, un po' di NodeJS, ...\n\n\
         Padroneggio rapidamente applicazioni, linguaggi di programmazione e sistemi operativi."
    }
    fn role_achievements(&self) -> &'static [(&'static str, &'static str)] {
        &[
            ("Framework di sviluppo test Test.NET",
             "Successivamente venduto da Bosch a Vector."),
            ("Sincronizzazione della scheda di commutazione lampade SK24",
             "Sincronizzazione in tempo reale dei processi interni della scheda di commutazione lampade del controllo del traffico con le onde sinusoidali della rete elettrica."),
            ("Concetto Vortexledger",
             "Un concetto di registro aperto ed economia basato sul baratto."),
        ]
    }

    fn display_nothing_selected(&self) -> &'static str { "Seleziona gli elementi nella scheda Filtro per vedere i dettagli qui." }

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
            "multimedia" => "Programmazione multimediale",
            "system_design" => "Progettazione di sistemi",
            "project_management" => "Gestione progetti",
            "test_management" => "Gestione test",
            "automated_testing" => "Test automatizzati",
            "manual_testing" => "Test manuali",
            "erp" => "Sistemi ERP",
            "administration" => "Amministrazione di sistema",
            "leading" => "Leadership",
            "auto_test_dev" => "Sviluppo test automatizzati",
            // Countries
            "hungary" => "Ungheria",
            "germany" => "Germania",
            "austria" => "Austria",
            // Languages
            "lang_hungarian" => "Ungherese",
            "lang_german" => "Tedesco",
            "lang_english" => "Inglese",
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
            "cert_diploma" => "Diploma universitario",
            "cert_pm" => "Certificato PM (PMCC)",
            "cert_sql" => "Fondamenti di database e SQL",
            "cert_js" => "Certificato JavaScript",
            "cert_ai" => "Programmazione con agenti IA",
            "cert_driving" => "Tecnica di guida avanzata",
            "cert_scrum" => "Scrum",
            "cert_istqb" => "Seminario ISTQB",
            "cert_presentation" => "Tecniche di presentazione",
            "cert_access" => "MS Access 2003 (intermedio)",
            "cert_canoe" => "Fondamenti di CANoe",
            "cert_dotnet" => "Corso .NET, C#",
            "cert_moderation" => "Moderazione — fondamenti",
            "cert_daf" => "Tedesco come lingua straniera",
            "cert_germanistik" => "Semestre estivo internazionale di studi germanici",
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
            "jr_admin" => "Amministratore",
            "jr_sw_dev" => "Sviluppatore software",
            "jr_dept_head" => "Capo dipartimento",
            "jr_tester" => "Tester",
            "jr_test_mgr" => "Responsabile test",
            "jr_test_dev" => "Sviluppatore test",
            "jr_project_mgr" => "Responsabile progetto",
            // Projects (Sankey)
            "pj_labor" => "Laboratorio",
            "pj_erp_system" => "Sistema ERP",
            "pj_db_finance" => "Deutsche Bank Finanziamento edilizio",
            "pj_cwl_kwg" => "C&L Deutsche Revision KWG Novelle",
            "pj_authors_dream" => "Author's Dream",
            "pj_citibank" => "Citibank Antiriciclaggio",
            "pj_junction" => "Strumento di controllo incroci",
            "pj_btc5000" => "BTC5000",
            "pj_sk24" => "SK24",
            "pj_ocit" => "OCIT",
            "pj_debrecen" => "Debrecen",
            "pj_medical" => "Imaging medicale",
            "pj_car_body" => "Computer di bordo auto",
            "pj_dcdc" => "Convertitore DC/DC",
            "pj_erp_bosch" => "ERP / Pianificazione risorse",
            "pj_test_designer" => "TestDesigner",
            "pj_test_net" => "Test.NET",
            "pj_truck_body" => "Computer di bordo camion",
            "pj_test_tools" => "Miglioramento strumenti di test",
            "pj_window_lifter" => "Alzacristalli",
            "pj_contract" => "Contratto (Auftrag)",
            "pj_asanet" => "ASA-Net",
            "pj_lynx" => "Lynx",
            "pj_sms_email" => "SMS-eMail",
            "pj_customer_cards" => "Carte clienti",
            "pj_e_billing" => "Fatturazione elettronica",
            "pj_mein_auto" => "Mein-Auto App",
            "pj_mobil_car" => "Accettazione mobile veicolo",
            "pj_supplier_inv" => "Controllo fatture fornitori",
            "pj_prufloader" => "Prüfloader.NET",
            "pj_vortexledger" => "Vortexledger",
            "pj_georoute" => "GIS GeoRoute",
            // Expertise (Sankey)
            "ex_admin" => "Amministrazione",
            "ex_c_win" => "C win/GUI",
            "ex_c_embedded" => "C embedded",
            "ex_cpp_win" => "C++ win/GUI",
            "ex_cpp_linux" => "C++ linux/server",
            "ex_csharp" => "C#",
            "ex_sql" => "SQL",
            "ex_multimedia" => "Programmazione multimediale",
            "ex_system_design" => "Progettazione di sistemi",
            "ex_project_mgmt" => "Gestione progetti",
            "ex_leading" => "Leadership",
            "ex_manual_test" => "Test manuali",
            "ex_auto_testing" => "Test automatizzati",
            "ex_auto_test_dev" => "Sviluppo test automatizzati",
            "ex_test_mgmt" => "Gestione test",
            // Main Characteristics
            "mc_strengths" => "Punti di forza",
            "mc_achievements" => "Risultati chiave",
            "mc_countries" => "Paesi",
            "mc_languages" => "Competenze linguistiche",
            "mc_certificates" => "Certificati",
            "mc_digital_skills" => "Competenze digitali",
            // Tools
            "tl_visual_studio" => "Visual Studio",
            "tl_ms_office" => "MS Office (Excel, Word, Access)",
            "tl_clearcase" => "IBM ClearCase",
            "tl_clearquest" => "IBM ClearQuest",
            "tl_doors" => "IBM Doors",
            "tl_isystem" => "iSYSTEM Debugger",
            "tl_dspace" => "dSPACE Torre di test",
            "tl_ni_teststand" => "NI TestStand",
            "tl_ni_cvi" => "NI LabWindows/CVI",
            "tl_agilent" => "Strumenti Agilent",
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
            "c_embedded" => "Firmware microcontrollore per sistemi di traffico ed ECU automotive",
            "c_win_gui" => "Applicazioni desktop Windows in C",
            "cpp_win_gui" => "Applicazioni GUI Windows in C++",
            "cpp_linux_server" => "Sviluppo server Linux in C++",
            "csharp" => "C# (.NET) — strumenti di test, ERP, Prüfloader",
            "sql" => "Query su database, procedure memorizzate",
            "rust" => "Rust + Dioxus + WebAssembly per applicazioni web",
            "multimedia" => "Multimedia e presentazioni interattive",
            "system_design" => "Architettura e progettazione di sistemi per il controllo del traffico",
            "project_management" => "Direzione di team e gestione progetti",
            "test_management" => "Strategia di test, pianificazione e reporting",
            "automated_testing" => "Sviluppo test HW/SW automatizzati",
            "manual_testing" => "Test funzionali e di regressione",
            "erp" => "Sistemi di pianificazione delle risorse aziendali",
            "administration" => "Amministrazione di sistema e di rete",
            "leading" => "Vilati — direzione del dipartimento di sviluppo software",
            "auto_test_dev" => "Bosch — sviluppo di strumenti di test automatizzati",
            "hungary" => "Vilati, Mediso, MOL, Bäko-Hungaria, Bitnök, Telekom",
            "germany" => "Teamcom, Bosch",
            "austria" => "Porsche Informatik, Sigmatek",
            "lang_hungarian" => "Lingua madre",
            "lang_german" => "Era fluente, non utilizzato da tempo",
            "lang_english" => "Conversazione di base, letteratura tecnica, e-mail, chat",
            "mol" => "Amministrazione del sistema di laboratorio",
            "bako" => "Amministrazione del sistema ERP",
            "teamcom" => "Software multimediale finanziario — Deutsche Bank, Citibank, C&L",
            "vilati" => "Controllo incroci stradali, C embedded, direzione dipartimento",
            "mediso" => "Elaborazione di immagini mediche",
            "bosch" => "Sviluppo e test ECU automotive — Body Computer, alzacristalli",
            "porsche" => "Sistemi di gestione concessionari — C++ Linux/Windows, SQL",
            "sigmatek" => "Prüfloader.NET — strumento di test industriale in C#",
            "bitnok" => "Gestione progetto blockchain — VortexLedger",
            "telekom" => "GIS / GeoRoute amministrazione",
            "cert_diploma" => "Laurea in ingegneria",
            "cert_pm" => "Certificato di gestione progetti — PMCC",
            "cert_sql" => "Corso fondamenti di database e SQL",
            "cert_js" => "Certificato di sviluppo JavaScript",
            "cert_ai" => "Corso di programmazione con agenti IA",
            "cert_driving" => "Tecnica di guida avanzata per la sicurezza",
            "cert_scrum" => "Scrum — corso interno PORSCHE",
            "cert_istqb" => "Seminario ISTQB — corso interno Bosch",
            "cert_presentation" => "Tecniche di presentazione — corso Develor",
            "cert_access" => "MS Access 2003 (intermedio) — Controll Training",
            "cert_canoe" => "Fondamenti di CANoe — VECTOR",
            "cert_dotnet" => "Corso .NET, C# — BME",
            "cert_moderation" => "Moderazione — fondamenti — Bosch interno",
            "cert_daf" => "Tedesco come lingua straniera — Università Friedrich Schiller Jena (borsa di studio TEMPUS)",
            "cert_germanistik" => "Semestre estivo internazionale di studi germanici — FSU Jena",
            // Workplace hints
            "wk_mol" => "Amministrazione del sistema di laboratorio",
            "wk_bako" => "Amministrazione del sistema ERP",
            "wk_teamcom" => "Software multimediale finanziario — Deutsche Bank, Citibank, C&L",
            "wk_vilati" => "Controllo incroci stradali, C embedded, direzione dipartimento",
            "wk_mediso" => "Elaborazione di immagini mediche",
            "wk_bosch" => "Sviluppo e test ECU automotive — Body Computer, alzacristalli",
            "wk_porsche" => "Sistemi di gestione concessionari — C++ Linux/Windows, SQL",
            "wk_sigmatek" => "Prüfloader.NET — strumento di test industriale in C#",
            "wk_bitnok" => "Gestione progetto blockchain — VortexLedger",
            "wk_telekom" => "GIS / GeoRoute amministrazione",
            // Job role hints
            "jr_admin" => "MOL, Bäko-Hungaria, Telekom",
            "jr_sw_dev" => "Teamcom, Vilati, Mediso, Bosch, Porsche, Sigmatek",
            "jr_dept_head" => "Vilati — OCIT, Debrecen",
            "jr_tester" => "Bosch, Porsche Informatik",
            "jr_test_mgr" => "Bosch — Computer di bordo camion, strumenti di test",
            "jr_test_dev" => "Bosch — Alzacristalli, Computer di bordo camion",
            "jr_project_mgr" => "Bitnök — Vortexledger",
            // Project hints
            "pj_labor" => "MOL AG — sistema di laboratorio",
            "pj_erp_system" => "Bäko-Hungaria — pianificazione risorse aziendali",
            "pj_db_finance" => "Teamcom — Deutsche Bank finanziamento edilizio",
            "pj_cwl_kwg" => "Teamcom — C&L Deutsche Revision",
            "pj_authors_dream" => "Teamcom — editor multimediale",
            "pj_citibank" => "Teamcom — antiriciclaggio",
            "pj_junction" => "Vilati — controllo incroci stradali",
            "pj_btc5000" => "Vilati — controllore del traffico",
            "pj_sk24" => "Vilati — scheda commutazione lampade, sincronizzazione in tempo reale",
            "pj_ocit" => "Vilati — direzione dipartimento, standard OCIT",
            "pj_debrecen" => "Vilati — direzione dipartimento, progetto Debrecen",
            "pj_medical" => "Mediso — elaborazione di immagini mediche",
            "pj_car_body" => "Bosch — computer di bordo auto, C embedded",
            "pj_dcdc" => "Bosch — convertitore DC/DC, C embedded",
            "pj_erp_bosch" => "Bosch — pianificazione risorse, C#",
            "pj_test_designer" => "Bosch — strumento di progettazione test, C#",
            "pj_test_net" => "Bosch — framework di sviluppo test, C#",
            "pj_truck_body" => "Bosch — computer di bordo camion, gestione test",
            "pj_test_tools" => "Bosch — miglioramento strumenti di test",
            "pj_window_lifter" => "Bosch — alzacristalli, test automatizzati",
            "pj_contract" => "Porsche — gestione contratti, C++ Linux/Windows",
            "pj_asanet" => "Porsche — ASA-Net, C++ Linux/Windows",
            "pj_lynx" => "Porsche — Lynx, C++ linux/server",
            "pj_sms_email" => "Porsche — SMS-eMail, C++ Linux/Windows",
            "pj_customer_cards" => "Porsche — carte clienti, C++ Linux/Windows",
            "pj_e_billing" => "Porsche — fatturazione elettronica, C++ Linux/Windows",
            "pj_mein_auto" => "Porsche — Mein-Auto App, C++ linux/server",
            "pj_mobil_car" => "Porsche — accettazione mobile veicolo, C++ linux/server",
            "pj_supplier_inv" => "Porsche — fatture fornitori, C++ Linux/Windows",
            "pj_prufloader" => "Sigmatek — strumento di test industriale, C#",
            "pj_vortexledger" => "Bitnök — registro aperto blockchain",
            "pj_georoute" => "Telekom — amministrazione GIS/GeoRoute",
            // Expertise hints
            "ex_admin" => "MOL, Bäko-Hungaria, Telekom",
            "ex_c_win" => "Teamcom, Mediso — applicazioni desktop Windows",
            "ex_c_embedded" => "Vilati, Bosch — firmware microcontrollore",
            "ex_cpp_win" => "Mediso, Porsche — C++ Windows GUI",
            "ex_cpp_linux" => "Porsche — C++ server Linux",
            "ex_csharp" => "Bosch, Sigmatek — applicazioni .NET",
            "ex_sql" => "Porsche — database, procedure memorizzate",
            "ex_multimedia" => "Teamcom — presentazioni finanziarie interattive",
            "ex_system_design" => "Vilati — progettazione di sistemi di controllo del traffico",
            "ex_project_mgmt" => "Vilati, Bitnök — gestione progetti",
            "ex_leading" => "Vilati — direzione del dipartimento di sviluppo software",
            "ex_manual_test" => "Bosch, Porsche — test funzionali e di regressione",
            "ex_auto_testing" => "Bosch — test HW/SW automatizzati",
            "ex_auto_test_dev" => "Bosch — sviluppo strumenti di test automatizzati",
            "ex_test_mgmt" => "Bosch — strategia di test, pianificazione e reporting",
            // Main Characteristics hints
            "mc_strengths" => "Competenza, leadership e risoluzione dei problemi",
            "mc_achievements" => "Test.NET, sincronizzazione SK24, Vortexledger",
            "mc_countries" => "Ungheria, Germania, Austria",
            "mc_languages" => "Ungherese, tedesco, inglese",
            "mc_certificates" => "Diploma, PM, SQL, JavaScript, IA, guida",
            "mc_digital_skills" => "Competenze digitali complete",
            // Tool hints
            "tl_visual_studio" => "Bosch, Vilati, Teamcom, Mediso",
            "tl_ms_office" => "Bosch — documentazione, pianificazione",
            "tl_clearcase" => "Bosch — controllo versioni",
            "tl_clearquest" => "Bosch — tracciamento bug",
            "tl_doors" => "Bosch — gestione requisiti",
            "tl_isystem" => "Bosch — debug microcontrollore",
            "tl_dspace" => "Bosch — test HiL",
            "tl_ni_teststand" => "Bosch — test automatizzati",
            "tl_ni_cvi" => "Bosch — test semi-automatici",
            "tl_agilent" => "Bosch — strumenti per test alzacristalli",
            "tl_opentest" => "Bosch — strumento di test automatizzato (ora Vector)",
            "tl_authors_dream" => "Teamcom — editor multimediale",
            "tl_authorware" => "Teamcom — Citibank multimedia",
            "tl_python" => "Bosch — script TestDesigner",
            "tl_snow" => "Telekom — gestione ticket ITSM",
            "tl_keil" => "Vilati — compilatore microcontrollore C167",
            "tl_cosmic" => "Bosch — compilatore microcontrollore automotive",
            "tl_gnu_cc" => "Vilati — firmware OCIT su Linux",
            "tl_tornado" => "Vilati — sviluppo in tempo reale VxWorks",
            "tl_java_sdk" => "Vilati — Java Native Interface",
            "tl_structured_text" => "Sigmatek — programmazione controllori industriali",
            "tl_node_js" => "Bitnök — strumenti registro aperto",
            _ => "",
        }
    }

    fn btn_generate_pdf(&self) -> &'static str { "Genera PDF" }

    fn role_section_label(&self) -> &'static str { "Ruolo" }
    fn role_label(&self, key: &str) -> &'static str {
        match key {
            "po" => "Product Owner",
            "szm" => "Ingegnere del software",
            "tm" => "Responsabile test",
            _ => "",
        }
    }
    fn role_title(&self, key: &str) -> &'static str {
        match key {
            "po" => "Product Owner",
            "szm" => "Ingegnere del software",
            "tm" => "Responsabile test",
            _ => "Ingegnere del software",
        }
    }
    fn role_target_title(&self) -> &'static str { "Obiettivo" }
    fn role_target_text(&self, key: &str) -> &'static str {
        match key {
            "po" => "Come ingegnere del software esperto, il mio obiettivo è sfruttare la mia \
                vasta esperienza nella gestione di prodotti e processi in un ruolo di Product \
                Owner. Miro a migliorare l'esperienza utente attraverso l'analisi del feedback \
                dei clienti, le pratiche di sviluppo agile e il pensiero sistemico.",
            "szm" => "Come ingegnere del software esperto con oltre un decennio di esperienza \
                industriale (in particolare in C, C#, C++, SQL, sistemi embedded), desidero \
                applicare le mie conoscenze e le mie capacità innovative di risoluzione dei \
                problemi in un'organizzazione dinamica e in crescita. Il mio obiettivo è \
                contribuire al successo dell'organizzazione sviluppando soluzioni software \
                di alta qualità, espandendo continuamente la mia competenza in nuove \
                tecnologie.",
            "tm" => "Con decenni di esperienza nello sviluppo software, possiedo un'ampia \
                esperienza nella gestione dei test in sistemi complessi. La mia competenza \
                copre la pianificazione, la gestione e l'automazione dei processi di test, \
                nonché la garanzia della qualità. Sono esperto in metodologie di test \
                manuali e automatizzati.",
            _ => "",
        }
    }
    fn role_strengths_title(&self) -> &'static str { "Punti di forza" }
    fn role_strengths(&self, key: &str) -> &'static [(&'static str, &'static str)] {
        match key {
            "po" => &[
                ("Gestione prodotti e processi",
                 "Esperienza nello sviluppo, supporto e ottimizzazione di vari sistemi bancari e automotive."),
                ("Pensiero analitico",
                 "Risoluzione di compiti complessi di programmazione a livello software e hardware."),
                ("Precisione e attenzione ai dettagli",
                 "Lavoro meticoloso e documentazione accurata."),
                ("Collaborazione e comunicazione",
                 "Esperienza nella direzione di team di sviluppo e cooperazione con dipartimenti trasversali."),
                ("Ambiente agile",
                 "Conoscenza dei canali digitali e degli ambienti di sviluppo agile."),
            ],
            "szm" => &[
                ("Sviluppo e test software",
                 "Ampia esperienza con C, C++, C# inclusa la sincronizzazione di sistemi in tempo reale e lo sviluppo di framework di test automatizzati."),
                ("Pensiero sistemico",
                 "Formazione in ingegneria meccanica che supporta la programmazione a livello hardware e la comprensione di sistemi meccatronici complessi."),
                ("Gestione progetti e leadership",
                 "Direzione di dipartimenti di sviluppo software, coordinamento test di sistema, metodologie Scrum e ISTQB."),
                ("Diversità tecnologica",
                 "Competenza in Linux, Windows, SQL, sviluppo web e concetti DLT."),
                ("Risoluzione dei problemi",
                 "Apprendimento rapido con eccellenti capacità di debug e troubleshooting."),
            ],
            "tm" => &[
                ("Approccio orientato alla qualità",
                 "Impegno per i più alti standard professionali (ISTQB, standard interni Bosch)."),
                ("Risoluzione proattiva dei problemi",
                 "Analisi sistematica e persistente delle cause profonde, ricerca di soluzioni ottimali o compromessi."),
                ("Prototipi e framework",
                 "Progettazione e implementazione di framework di test interni (es. Test.NET) per l'automazione dei test."),
                ("Strategia di test realistica",
                 "Allineamento della validazione dell'ambiente di test con le tempistiche di produzione dei risultati dei test durante la pianificazione."),
            ],
            _ => &[],
        }
    }
    fn pwa_hint(&self) -> &'static str {
        "Questa è un\u{2019}app PWA \u{2014} puoi salvarla dal browser sulla schermata principale del telefono come un\u{2019}app nativa"
    }
}
