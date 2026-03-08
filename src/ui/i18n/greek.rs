use super::UiI18n;

pub struct GreekUi;

impl UiI18n for GreekUi {
    fn tab_profile(&self) -> &'static str { "Προφίλ" }
    fn tab_filter(&self) -> &'static str { "Φίλτρο" }
    fn tab_display(&self) -> &'static str { "Εμφάνιση" }

    fn profile_name(&self) -> &'static str { "István Finta" }
    fn profile_subtitle(&self) -> &'static str { "Διπλ. Μηχανολόγος Μηχανικός, Ρομποτική & Πληροφορική" }
    fn profile_title(&self) -> &'static str { "Μηχανικός Λογισμικού" }
    fn profile_location(&self) -> &'static str { "Ουγγαρία" }
    fn profile_email(&self) -> &'static str { "istvan_finta@yahoo.com" }
    fn profile_phone(&self) -> &'static str { "+36 70 343 9517" }

    fn section_skills(&self) -> &'static str { "Τεχνικές δεξιότητες" }
    fn section_countries(&self) -> &'static str { "Χώρες" }
    fn section_languages(&self) -> &'static str { "Γλωσσικές δεξιότητες" }
    fn section_companies(&self) -> &'static str { "Εταιρείες" }
    fn section_certificates(&self) -> &'static str { "Πιστοποιητικά" }
    fn filter_select_all(&self) -> &'static str { "Επιλογή όλων" }
    fn filter_clear_all(&self) -> &'static str { "Εκκαθάριση όλων" }

    fn section_job_roles(&self) -> &'static str { "Θέσεις εργασίας" }
    fn section_projects(&self) -> &'static str { "Έργα" }
    fn section_tools(&self) -> &'static str { "Εργαλεία" }
    fn section_project_experience(&self) -> &'static str { "Εμπειρία σε έργα" }

    fn section_main_chars(&self) -> &'static str { "Κύρια χαρακτηριστικά μου" }
    fn role_achievements_title(&self) -> &'static str { "Βασικά επιτεύγματα" }
    fn digital_skills_title(&self) -> &'static str { "Ψηφιακές δεξιότητες" }
    fn digital_skills_text(&self) -> &'static str {
        "Ολοκληρωμένες.\n\n\
         ΛΣ: Windows, Debian Linux, iOS, Android, OSX, VxWorks, ...\n\
         Εργαλεία ανάπτυξης, makefiles, μεταγλωττιστές (C, C#, C++, λίγο Rust, ...), \
         διερμηνευτές (Python, ...), επεξεργαστές κειμένου (nano, kate, MS Code, ...)\n\
         MS Office, Word, Excel, ... (Libre Office και παρόμοια επίσης), Προγράμματα \
         περιήγησης, Εφαρμογές web, Πάροχοι cloud (Vultr, Oracle, ...)\n\
         Εφαρμογές γραφικών, OpenSCAD (σχεδιασμός 3D), Eagle (σχεδιασμός κυκλωμάτων), \
         Gimp, εργαλεία στιγμιοτύπων οθόνης, εργαλεία επεξεργασίας φωτογραφιών, ...\n\
         Βασικές γνώσεις AI, προχωρημένες γνώσεις DLT (Distributed Ledger Technology), \
         Stellar Network, έξυπνα συμβόλαια, ... Βασική ανάπτυξη web, HTML5, CSS, \
         JavaScript, λίγο React, λίγο NodeJS, ...\n\n\
         Κατακτώ γρήγορα εφαρμογές, γλώσσες προγραμματισμού και λειτουργικά συστήματα."
    }
    fn role_achievements(&self) -> &'static [(&'static str, &'static str)] {
        &[
            ("Πλαίσιο ανάπτυξης δοκιμών Test.NET",
             "Αργότερα πωλήθηκε από τη Bosch στη Vector."),
            ("Συγχρονισμός κάρτας μεταγωγής λαμπτήρων SK24",
             "Συγχρονισμός σε πραγματικό χρόνο των εσωτερικών διεργασιών της κάρτας μεταγωγής λαμπτήρων ελέγχου κυκλοφορίας με τα ημιτονοειδή κύματα του ηλεκτρικού δικτύου."),
            ("Ιδέα Vortexledger",
             "Μια ιδέα ανοικτού βιβλίου και οικονομίας βασισμένη στο αντιπραγματισμό."),
        ]
    }

    fn display_nothing_selected(&self) -> &'static str { "Επιλέξτε στοιχεία στην καρτέλα Φίλτρο για να δείτε λεπτομέρειες εδώ." }

    fn item_label(&self, key: &str) -> &'static str {
        match key {
            // Skills
            "c_embedded" => "C (ενσωματωμένο)",
            "c_win_gui" => "C (Windows/GUI)",
            "cpp_win_gui" => "C++ (Windows/GUI)",
            "cpp_linux_server" => "C++ (Linux/Διακομιστής)",
            "csharp" => "C#",
            "sql" => "SQL",
            "rust" => "Rust / WebAssembly",
            "multimedia" => "Προγραμματισμός πολυμέσων",
            "system_design" => "Σχεδιασμός συστημάτων",
            "project_management" => "Διαχείριση έργων",
            "test_management" => "Διαχείριση δοκιμών",
            "automated_testing" => "Αυτοματοποιημένες δοκιμές",
            "manual_testing" => "Χειροκίνητες δοκιμές",
            "erp" => "Συστήματα ERP",
            "administration" => "Διαχείριση συστημάτων",
            "leading" => "Ηγεσία",
            "auto_test_dev" => "Ανάπτυξη αυτοματοποιημένων δοκιμών",
            // Countries
            "hungary" => "Ουγγαρία",
            "germany" => "Γερμανία",
            "austria" => "Αυστρία",
            // Languages
            "lang_hungarian" => "Ουγγρικά",
            "lang_german" => "Γερμανικά",
            "lang_english" => "Αγγλικά",
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
            "cert_diploma" => "Πανεπιστημιακό δίπλωμα",
            "cert_pm" => "Πιστοποιητικό PM (PMCC)",
            "cert_sql" => "Βάσεις δεδομένων & SQL — Βασικά",
            "cert_js" => "Πιστοποιητικό JavaScript",
            "cert_ai" => "Προγραμματισμός με πράκτορες AI",
            "cert_driving" => "Προηγμένη τεχνική οδήγησης",
            "cert_scrum" => "Scrum",
            "cert_istqb" => "Σεμινάριο ISTQB",
            "cert_presentation" => "Τεχνικές παρουσίασης",
            "cert_access" => "MS Access 2003 (μεσαίο επίπεδο)",
            "cert_canoe" => "Βασικά CANoe",
            "cert_dotnet" => "Μάθημα .NET, C#",
            "cert_moderation" => "Συντονισμός — βασικά",
            "cert_daf" => "Γερμανικά ως ξένη γλώσσα",
            "cert_germanistik" => "Διεθνές θερινό εξάμηνο γερμανικών σπουδών",
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
            "jr_admin" => "Διαχειριστής",
            "jr_sw_dev" => "Προγραμματιστής λογισμικού",
            "jr_dept_head" => "Διευθυντής τμήματος",
            "jr_tester" => "Δοκιμαστής",
            "jr_test_mgr" => "Υπεύθυνος δοκιμών",
            "jr_test_dev" => "Προγραμματιστής δοκιμών",
            "jr_project_mgr" => "Διαχειριστής έργου",
            // Projects (Sankey)
            "pj_labor" => "Εργαστήριο",
            "pj_erp_system" => "Σύστημα ERP",
            "pj_db_finance" => "Deutsche Bank Χρηματοδότηση κτιρίων",
            "pj_cwl_kwg" => "C&L Deutsche Revision KWG Novelle",
            "pj_authors_dream" => "Author's Dream",
            "pj_citibank" => "Citibank Κατά του ξεπλύματος χρήματος",
            "pj_junction" => "Εργαλείο ελέγχου διασταύρωσης",
            "pj_btc5000" => "BTC5000",
            "pj_sk24" => "SK24",
            "pj_ocit" => "OCIT",
            "pj_debrecen" => "Debrecen",
            "pj_medical" => "Ιατρική απεικόνιση",
            "pj_car_body" => "Υπολογιστής αμαξώματος αυτοκινήτου",
            "pj_dcdc" => "Μετατροπέας DC/DC",
            "pj_erp_bosch" => "ERP / Σχεδιασμός πόρων",
            "pj_test_designer" => "TestDesigner",
            "pj_test_net" => "Test.NET",
            "pj_truck_body" => "Υπολογιστής αμαξώματος φορτηγού",
            "pj_test_tools" => "Βελτίωση εργαλείων δοκιμών",
            "pj_window_lifter" => "Ηλεκτρικό παράθυρο",
            "pj_contract" => "Συμβόλαιο (Auftrag)",
            "pj_asanet" => "ASA-Net",
            "pj_lynx" => "Lynx",
            "pj_sms_email" => "SMS-eMail",
            "pj_customer_cards" => "Κάρτες πελατών",
            "pj_e_billing" => "Ηλεκτρονική τιμολόγηση",
            "pj_mein_auto" => "Mein-Auto App",
            "pj_mobil_car" => "Κινητή παραλαβή οχήματος",
            "pj_supplier_inv" => "Έλεγχος τιμολογίων προμηθευτών",
            "pj_prufloader" => "Prüfloader.NET",
            "pj_vortexledger" => "Vortexledger",
            "pj_georoute" => "GIS GeoRoute",
            // Expertise (Sankey)
            "ex_admin" => "Διαχείριση",
            "ex_c_win" => "C win/GUI",
            "ex_c_embedded" => "C ενσωματωμένο",
            "ex_cpp_win" => "C++ win/GUI",
            "ex_cpp_linux" => "C++ linux/διακομιστής",
            "ex_csharp" => "C#",
            "ex_sql" => "SQL",
            "ex_multimedia" => "Προγραμματισμός πολυμέσων",
            "ex_system_design" => "Σχεδιασμός συστημάτων",
            "ex_project_mgmt" => "Διαχείριση έργων",
            "ex_leading" => "Ηγεσία",
            "ex_manual_test" => "Χειροκίνητες δοκιμές",
            "ex_auto_testing" => "Αυτοματοποιημένες δοκιμές",
            "ex_auto_test_dev" => "Ανάπτυξη αυτοματοποιημένων δοκιμών",
            "ex_test_mgmt" => "Διαχείριση δοκιμών",
            // Main Characteristics
            "mc_strengths" => "Δυνατά σημεία",
            "mc_achievements" => "Βασικά επιτεύγματα",
            "mc_countries" => "Χώρες",
            "mc_languages" => "Γλωσσικές δεξιότητες",
            "mc_certificates" => "Πιστοποιητικά",
            "mc_digital_skills" => "Ψηφιακές δεξιότητες",
            // Tools
            "tl_visual_studio" => "Visual Studio",
            "tl_ms_office" => "MS Office (Excel, Word, Access)",
            "tl_clearcase" => "IBM ClearCase",
            "tl_clearquest" => "IBM ClearQuest",
            "tl_doors" => "IBM Doors",
            "tl_isystem" => "iSYSTEM Debugger",
            "tl_dspace" => "dSPACE Πύργος Δοκιμών",
            "tl_ni_teststand" => "NI TestStand",
            "tl_ni_cvi" => "NI LabWindows/CVI",
            "tl_agilent" => "Όργανα Agilent",
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
            "c_embedded" => "Firmware μικροελεγκτή για συστήματα κυκλοφορίας και αυτοκινητοβιομηχανικά ECU",
            "c_win_gui" => "Εφαρμογές επιφάνειας εργασίας Windows σε C",
            "cpp_win_gui" => "Εφαρμογές GUI Windows σε C++",
            "cpp_linux_server" => "Ανάπτυξη διακομιστή Linux σε C++",
            "csharp" => "C# (.NET) — εργαλεία δοκιμών, ERP, Prüfloader",
            "sql" => "Ερωτήματα βάσεων δεδομένων, αποθηκευμένες διαδικασίες",
            "rust" => "Rust + Dioxus + WebAssembly για εφαρμογές web",
            "multimedia" => "Πολυμέσα και διαδραστικές παρουσιάσεις",
            "system_design" => "Αρχιτεκτονική και σχεδιασμός συστημάτων για έλεγχο κυκλοφορίας",
            "project_management" => "Διεύθυνση ομάδων και διαχείριση έργων",
            "test_management" => "Στρατηγική δοκιμών, σχεδιασμός και αναφορές",
            "automated_testing" => "Ανάπτυξη αυτοματοποιημένων δοκιμών HW/SW",
            "manual_testing" => "Λειτουργικές δοκιμές και δοκιμές παλινδρόμησης",
            "erp" => "Συστήματα σχεδιασμού επιχειρηματικών πόρων",
            "administration" => "Διαχείριση συστημάτων και δικτύων",
            "leading" => "Vilati — διεύθυνση τμήματος ανάπτυξης λογισμικού",
            "auto_test_dev" => "Bosch — ανάπτυξη αυτοματοποιημένων εργαλείων δοκιμών",
            "hungary" => "Vilati, Mediso, MOL, Bäko-Hungaria, Bitnök, Telekom",
            "germany" => "Teamcom, Bosch",
            "austria" => "Porsche Informatik, Sigmatek",
            "lang_hungarian" => "Μητρική γλώσσα",
            "lang_german" => "Ήταν σε επίπεδο συνομιλίας, χωρίς χρήση εδώ και καιρό",
            "lang_english" => "Βασική συνομιλία, τεχνική βιβλιογραφία, email, chat",
            "mol" => "Διαχείριση εργαστηριακού συστήματος",
            "bako" => "Διαχείριση συστήματος ERP",
            "teamcom" => "Χρηματοοικονομικό λογισμικό πολυμέσων — Deutsche Bank, Citibank, C&L",
            "vilati" => "Έλεγχος κυκλοφοριακού κόμβου, ενσωματωμένο C, διεύθυνση τμήματος",
            "mediso" => "Επεξεργασία ιατρικής εικόνας",
            "bosch" => "Ανάπτυξη και δοκιμή αυτοκινητοβιομηχανικών ECU — Body Computers, ηλεκτρικά παράθυρα",
            "porsche" => "Συστήματα διαχείρισης αντιπροσωπειών — C++ Linux/Windows, SQL",
            "sigmatek" => "Prüfloader.NET — βιομηχανικό εργαλείο δοκιμών σε C#",
            "bitnok" => "Διαχείριση έργου blockchain — VortexLedger",
            "telekom" => "GIS / GeoRoute διαχείριση",
            "cert_diploma" => "Πανεπιστημιακό πτυχίο μηχανικού",
            "cert_pm" => "Πιστοποιητικό διαχείρισης έργων — PMCC",
            "cert_sql" => "Μάθημα βασικών βάσεων δεδομένων και SQL",
            "cert_js" => "Πιστοποιητικό ανάπτυξης JavaScript",
            "cert_ai" => "Μάθημα προγραμματισμού με πράκτορες AI",
            "cert_driving" => "Προηγμένη τεχνική οδήγησης για ασφάλεια",
            "cert_scrum" => "Scrum — εσωτερικό μάθημα PORSCHE",
            "cert_istqb" => "Σεμινάριο ISTQB — εσωτερικό μάθημα Bosch",
            "cert_presentation" => "Τεχνικές παρουσίασης — μάθημα Develor",
            "cert_access" => "MS Access 2003 (μεσαίο επίπεδο) — Controll Training",
            "cert_canoe" => "Βασικά CANoe — VECTOR",
            "cert_dotnet" => "Μάθημα .NET, C# — BME",
            "cert_moderation" => "Συντονισμός — βασικά — Bosch εσωτερικό",
            "cert_daf" => "Γερμανικά ως ξένη γλώσσα — Πανεπιστήμιο Friedrich Schiller Ιένα (υποτροφία TEMPUS)",
            "cert_germanistik" => "Διεθνές θερινό εξάμηνο γερμανικών σπουδών — FSU Ιένα",
            // Workplace hints
            "wk_mol" => "Διαχείριση εργαστηριακού συστήματος",
            "wk_bako" => "Διαχείριση συστήματος ERP",
            "wk_teamcom" => "Χρηματοοικονομικό λογισμικό πολυμέσων — Deutsche Bank, Citibank, C&L",
            "wk_vilati" => "Έλεγχος κυκλοφοριακού κόμβου, ενσωματωμένο C, διεύθυνση τμήματος",
            "wk_mediso" => "Επεξεργασία ιατρικής εικόνας",
            "wk_bosch" => "Ανάπτυξη και δοκιμή αυτοκινητοβιομηχανικών ECU — Body Computers, ηλεκτρικά παράθυρα",
            "wk_porsche" => "Συστήματα διαχείρισης αντιπροσωπειών — C++ Linux/Windows, SQL",
            "wk_sigmatek" => "Prüfloader.NET — βιομηχανικό εργαλείο δοκιμών σε C#",
            "wk_bitnok" => "Διαχείριση έργου blockchain — VortexLedger",
            "wk_telekom" => "GIS / GeoRoute διαχείριση",
            // Job role hints
            "jr_admin" => "MOL, Bäko-Hungaria, Telekom",
            "jr_sw_dev" => "Teamcom, Vilati, Mediso, Bosch, Porsche, Sigmatek",
            "jr_dept_head" => "Vilati — OCIT, Debrecen",
            "jr_tester" => "Bosch, Porsche Informatik",
            "jr_test_mgr" => "Bosch — Υπολογιστής αμαξώματος φορτηγού, εργαλεία δοκιμών",
            "jr_test_dev" => "Bosch — Ηλεκτρικό παράθυρο, Υπολογιστής αμαξώματος φορτηγού",
            "jr_project_mgr" => "Bitnök — Vortexledger",
            // Project hints
            "pj_labor" => "MOL AG — εργαστηριακό σύστημα",
            "pj_erp_system" => "Bäko-Hungaria — σχεδιασμός επιχειρηματικών πόρων",
            "pj_db_finance" => "Teamcom — Deutsche Bank χρηματοδότηση κτιρίων",
            "pj_cwl_kwg" => "Teamcom — C&L Deutsche Revision",
            "pj_authors_dream" => "Teamcom — επεξεργαστής πολυμέσων",
            "pj_citibank" => "Teamcom — κατά του ξεπλύματος χρήματος",
            "pj_junction" => "Vilati — έλεγχος κυκλοφοριακού κόμβου",
            "pj_btc5000" => "Vilati — ελεγκτής κυκλοφορίας",
            "pj_sk24" => "Vilati — κάρτα μεταγωγής λαμπτήρων, συγχρονισμός πραγματικού χρόνου",
            "pj_ocit" => "Vilati — διεύθυνση τμήματος, πρότυπο OCIT",
            "pj_debrecen" => "Vilati — διεύθυνση τμήματος, έργο Debrecen",
            "pj_medical" => "Mediso — επεξεργασία ιατρικής εικόνας",
            "pj_car_body" => "Bosch — υπολογιστής αμαξώματος αυτοκινήτου, C ενσωματωμένο",
            "pj_dcdc" => "Bosch — μετατροπέας DC/DC, C ενσωματωμένο",
            "pj_erp_bosch" => "Bosch — σχεδιασμός πόρων, C#",
            "pj_test_designer" => "Bosch — εργαλείο σχεδιασμού δοκιμών, C#",
            "pj_test_net" => "Bosch — πλαίσιο ανάπτυξης δοκιμών, C#",
            "pj_truck_body" => "Bosch — υπολογιστής αμαξώματος φορτηγού, διαχείριση δοκιμών",
            "pj_test_tools" => "Bosch — βελτίωση εργαλείων δοκιμών",
            "pj_window_lifter" => "Bosch — ηλεκτρικό παράθυρο, αυτοματοποιημένες δοκιμές",
            "pj_contract" => "Porsche — διαχείριση συμβολαίων, C++ Linux/Windows",
            "pj_asanet" => "Porsche — ASA-Net, C++ Linux/Windows",
            "pj_lynx" => "Porsche — Lynx, C++ linux/διακομιστής",
            "pj_sms_email" => "Porsche — SMS-eMail, C++ Linux/Windows",
            "pj_customer_cards" => "Porsche — κάρτες πελατών, C++ Linux/Windows",
            "pj_e_billing" => "Porsche — ηλεκτρονική τιμολόγηση, C++ Linux/Windows",
            "pj_mein_auto" => "Porsche — Mein-Auto App, C++ linux/διακομιστής",
            "pj_mobil_car" => "Porsche — κινητή παραλαβή οχήματος, C++ linux/διακομιστής",
            "pj_supplier_inv" => "Porsche — τιμολόγια προμηθευτών, C++ Linux/Windows",
            "pj_prufloader" => "Sigmatek — βιομηχανικό εργαλείο δοκιμών, C#",
            "pj_vortexledger" => "Bitnök — ανοικτό βιβλίο blockchain",
            "pj_georoute" => "Telekom — διαχείριση GIS/GeoRoute",
            // Expertise hints
            "ex_admin" => "MOL, Bäko-Hungaria, Telekom",
            "ex_c_win" => "Teamcom, Mediso — εφαρμογές επιφάνειας εργασίας Windows",
            "ex_c_embedded" => "Vilati, Bosch — firmware μικροελεγκτή",
            "ex_cpp_win" => "Mediso, Porsche — C++ Windows GUI",
            "ex_cpp_linux" => "Porsche — C++ διακομιστής Linux",
            "ex_csharp" => "Bosch, Sigmatek — εφαρμογές .NET",
            "ex_sql" => "Porsche — βάσεις δεδομένων, αποθηκευμένες διαδικασίες",
            "ex_multimedia" => "Teamcom — διαδραστικές χρηματοοικονομικές παρουσιάσεις",
            "ex_system_design" => "Vilati — σχεδιασμός συστημάτων ελέγχου κυκλοφορίας",
            "ex_project_mgmt" => "Vilati, Bitnök — διαχείριση έργων",
            "ex_leading" => "Vilati — διεύθυνση τμήματος ανάπτυξης λογισμικού",
            "ex_manual_test" => "Bosch, Porsche — λειτουργικές δοκιμές και δοκιμές παλινδρόμησης",
            "ex_auto_testing" => "Bosch — αυτοματοποιημένες δοκιμές HW/SW",
            "ex_auto_test_dev" => "Bosch — ανάπτυξη αυτοματοποιημένων εργαλείων δοκιμών",
            "ex_test_mgmt" => "Bosch — στρατηγική δοκιμών, σχεδιασμός και αναφορές",
            // Main Characteristics hints
            "mc_strengths" => "Τεχνογνωσία, ηγεσία και επίλυση προβλημάτων",
            "mc_achievements" => "Test.NET, συγχρονισμός SK24, Vortexledger",
            "mc_countries" => "Ουγγαρία, Γερμανία, Αυστρία",
            "mc_languages" => "Ουγγρικά, γερμανικά, αγγλικά",
            "mc_certificates" => "Δίπλωμα, PM, SQL, JavaScript, AI, οδήγηση",
            "mc_digital_skills" => "Ολοκληρωμένες ψηφιακές δεξιότητες",
            // Tool hints
            "tl_visual_studio" => "Bosch, Vilati, Teamcom, Mediso",
            "tl_ms_office" => "Bosch — τεκμηρίωση, σχεδιασμός",
            "tl_clearcase" => "Bosch — έλεγχος εκδόσεων",
            "tl_clearquest" => "Bosch — παρακολούθηση σφαλμάτων",
            "tl_doors" => "Bosch — διαχείριση απαιτήσεων",
            "tl_isystem" => "Bosch — αποσφαλμάτωση μικροελεγκτή",
            "tl_dspace" => "Bosch — δοκιμές HiL",
            "tl_ni_teststand" => "Bosch — αυτοματοποιημένες δοκιμές",
            "tl_ni_cvi" => "Bosch — ημιαυτόματες δοκιμές",
            "tl_agilent" => "Bosch — όργανα για δοκιμή ηλεκτρικού παράθυρου",
            "tl_opentest" => "Bosch — αυτοματοποιημένο εργαλείο δοκιμών (τώρα Vector)",
            "tl_authors_dream" => "Teamcom — επεξεργαστής πολυμέσων",
            "tl_authorware" => "Teamcom — Citibank πολυμέσα",
            "tl_python" => "Bosch — σενάρια TestDesigner",
            "tl_snow" => "Telekom — σύστημα αιτημάτων ITSM",
            "tl_keil" => "Vilati — μεταγλωττιστής μικροελεγκτή C167",
            "tl_cosmic" => "Bosch — μεταγλωττιστής μικροελεγκτή αυτοκινήτου",
            "tl_gnu_cc" => "Vilati — firmware OCIT σε Linux",
            "tl_tornado" => "Vilati — ανάπτυξη πραγματικού χρόνου VxWorks",
            "tl_java_sdk" => "Vilati — Java Native Interface",
            "tl_structured_text" => "Sigmatek — βιομηχανικός προγραμματισμός ελεγκτών",
            "tl_node_js" => "Bitnök — εργαλεία ανοικτού βιβλίου",
            _ => "",
        }
    }

    fn btn_generate_pdf(&self) -> &'static str { "Δημιουργία PDF" }

    fn role_section_label(&self) -> &'static str { "Ρόλος" }
    fn role_label(&self, key: &str) -> &'static str {
        match key {
            "po" => "Υπεύθυνος Προϊόντος",
            "szm" => "Μηχανικός Λογισμικού",
            "tm" => "Υπεύθυνος Δοκιμών",
            _ => "",
        }
    }
    fn role_title(&self, key: &str) -> &'static str {
        match key {
            "po" => "Υπεύθυνος Προϊόντος",
            "szm" => "Μηχανικός Λογισμικού",
            "tm" => "Υπεύθυνος Δοκιμών",
            _ => "Μηχανικός Λογισμικού",
        }
    }
    fn role_target_title(&self) -> &'static str { "Στόχος" }
    fn role_target_text(&self, key: &str) -> &'static str {
        match key {
            "po" => "Ως έμπειρος μηχανικός λογισμικού, στόχος μου είναι να αξιοποιήσω \
                την εκτεταμένη εμπειρία μου στη διαχείριση προϊόντων και διαδικασιών σε \
                ρόλο Product Owner. Στοχεύω στη βελτίωση της εμπειρίας χρήστη μέσω \
                ανάλυσης ανατροφοδότησης πελατών, ευέλικτων πρακτικών ανάπτυξης και \
                συστημικής σκέψης.",
            "szm" => "Ως έμπειρος μηχανικός λογισμικού με πάνω από μια δεκαετία βιομηχανικής \
                εμπειρίας (ειδικά σε C, C#, C++, SQL, ενσωματωμένα συστήματα), θέλω να \
                εφαρμόσω τις γνώσεις μου και τις καινοτόμες ικανότητές μου επίλυσης \
                προβλημάτων σε μια δυναμική, αναπτυσσόμενη οργάνωση. Στόχος μου είναι να \
                συμβάλλω στην επιτυχία του οργανισμού αναπτύσσοντας υψηλής ποιότητας \
                λύσεις λογισμικού ενώ διευρύνω συνεχώς την τεχνογνωσία μου σε νέες \
                τεχνολογίες.",
            "tm" => "Με δεκαετίες εμπειρίας στην ανάπτυξη λογισμικού, διαθέτω εκτεταμένη \
                εμπειρία στη διαχείριση δοκιμών σε σύνθετα συστήματα. Η εμπειρογνωμοσύνη \
                μου καλύπτει τον σχεδιασμό, τη διαχείριση και την αυτοματοποίηση \
                διαδικασιών δοκιμών, καθώς και τη διασφάλιση ποιότητας. Είμαι ικανός \
                σε μεθοδολογίες χειροκίνητων και αυτοματοποιημένων δοκιμών.",
            _ => "",
        }
    }
    fn role_strengths_title(&self) -> &'static str { "Δυνατά σημεία" }
    fn role_strengths(&self, key: &str) -> &'static [(&'static str, &'static str)] {
        match key {
            "po" => &[
                ("Διαχείριση προϊόντων & διαδικασιών",
                 "Εμπειρία στην ανάπτυξη, υποστήριξη και βελτιστοποίηση διαφόρων τραπεζικών και αυτοκινητοβιομηχανικών συστημάτων."),
                ("Αναλυτική σκέψη",
                 "Επίλυση σύνθετων εργασιών προγραμματισμού σε επίπεδο λογισμικού και υλικού."),
                ("Ακρίβεια & προσοχή στη λεπτομέρεια",
                 "Σχολαστική εργασία και ακριβής τεκμηρίωση."),
                ("Συνεργασία & επικοινωνία",
                 "Εμπειρία στην ηγεσία ομάδων ανάπτυξης και συνεργασία με διατμηματικές ομάδες."),
                ("Ευέλικτο περιβάλλον",
                 "Γνώση ψηφιακών καναλιών και ευέλικτων περιβαλλόντων ανάπτυξης."),
            ],
            "szm" => &[
                ("Ανάπτυξη & δοκιμές λογισμικού",
                 "Ευρεία εμπειρία με C, C++, C# συμπεριλαμβανομένου του συγχρονισμού συστημάτων πραγματικού χρόνου και ανάπτυξης πλαισίων αυτοματοποιημένων δοκιμών."),
                ("Συστημική σκέψη",
                 "Υπόβαθρο μηχανολογίας που υποστηρίζει τον προγραμματισμό σε επίπεδο υλικού και την κατανόηση σύνθετων μηχατρονικών συστημάτων."),
                ("Διαχείριση έργων & ηγεσία",
                 "Διεύθυνση τμημάτων ανάπτυξης λογισμικού, συντονισμός δοκιμών συστήματος, μεθοδολογίες Scrum και ISTQB."),
                ("Τεχνολογική ποικιλομορφία",
                 "Επάρκεια σε Linux, Windows, SQL, ανάπτυξη web και έννοιες DLT."),
                ("Επίλυση προβλημάτων",
                 "Γρήγορη μάθηση με εξαιρετικές ικανότητες αποσφαλμάτωσης και αντιμετώπισης προβλημάτων."),
            ],
            "tm" => &[
                ("Προσέγγιση προσανατολισμένη στην ποιότητα",
                 "Δέσμευση στα υψηλότερα επαγγελματικά πρότυπα (ISTQB, εσωτερικά πρότυπα Bosch)."),
                ("Προληπτική επίλυση προβλημάτων",
                 "Συστηματική και επίμονη ανάλυση ριζικών αιτίων, εξεύρεση βέλτιστων λύσεων ή συμβιβασμών."),
                ("Πρωτότυπα & πλαίσια",
                 "Σχεδιασμός και υλοποίηση εσωτερικών πλαισίων δοκιμών (π.χ. Test.NET) για αυτοματοποίηση δοκιμών."),
                ("Ρεαλιστική στρατηγική δοκιμών",
                 "Ευθυγράμμιση της επικύρωσης περιβάλλοντος δοκιμών με τα χρονοδιαγράμματα παραγωγής αποτελεσμάτων δοκιμών κατά τον σχεδιασμό."),
            ],
            _ => &[],
        }
    }
    fn pwa_hint(&self) -> &'static str {
        "Αυτή είναι μια εφαρμογή PWA \u{2014} μπορείτε να την αποθηκεύσετε από τον περιηγητή σας στην αρχική οθόνη του τηλεφώνου σας σαν εγγενή εφαρμογή"
    }
}
