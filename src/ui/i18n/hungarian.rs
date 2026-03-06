use super::UiI18n;

pub struct HungarianUi;

impl UiI18n for HungarianUi {
    fn tab_profile(&self) -> &'static str { "Profil" }
    fn tab_filter(&self) -> &'static str { "Szűrő" }
    fn tab_display(&self) -> &'static str { "Megjelenítés" }

    fn profile_name(&self) -> &'static str { "Finta István" }
    fn profile_title(&self) -> &'static str { "Szoftvermérnök" }
    fn profile_location(&self) -> &'static str { "Magyarország" }
    fn profile_email(&self) -> &'static str { "istvan_finta@yahoo.com" }
    fn profile_phone(&self) -> &'static str { "+36 70 343 9517" }
    fn profile_about(&self) -> &'static str { "Rólam" }
    fn profile_about_text(&self) -> &'static str {
        "Tapasztalt szoftvermérnök, több mint 30 éves iparági tapasztalattal. \
         Szakterületeim: beágyazott rendszerek, asztali és webes alkalmazások, \
         tesztautomatizálás és projektmenedzsment. Magyarországon, \
         Németországban és Ausztriában dolgoztam az autóipar, forgalomirányítás, \
         orvosi képalkotás, pénzügyi szoftverek és blockchain területein."
    }

    fn section_skills(&self) -> &'static str { "Technikai ismeretek" }
    fn section_countries(&self) -> &'static str { "Országok" }
    fn section_languages(&self) -> &'static str { "Nyelvtudás" }
    fn section_companies(&self) -> &'static str { "Cégek" }
    fn section_certificates(&self) -> &'static str { "Bizonyítványok" }
    fn filter_select_all(&self) -> &'static str { "Összes kijelölése" }
    fn filter_clear_all(&self) -> &'static str { "Összes törlése" }

    fn section_job_roles(&self) -> &'static str { "Munkakörök" }
    fn section_projects(&self) -> &'static str { "Projektek" }
    fn section_tools(&self) -> &'static str { "Eszközök" }
    fn section_project_experience(&self) -> &'static str { "Projektek – részletezve" }

    fn section_main_chars(&self) -> &'static str { "Főbb jellemzőim" }
    fn role_achievements_title(&self) -> &'static str { "Főbb eredmények" }
    fn role_achievements(&self) -> &'static [(&'static str, &'static str)] {
        &[
            ("Test.NET tesztfejlesztő-framework",
             "Azóta a Bosch értékesítette a Vector-nak."),
            ("SK24 lámpakapcsolókártya szinkronizációja",
             "A közlekedésirányítási lámpakapcsolókártya belső folyamatainak real time szinkronizációja az elektromos hálózat szinusz-hullámaihoz."),
            ("Vortexledger koncepció",
             "Cserekereskedelemre alapuló open ledger és gazdaság koncepciója."),
        ]
    }

    fn display_nothing_selected(&self) -> &'static str { "Válassz elemeket a Szűrő fülön a részletek megtekintéséhez." }

    fn item_label(&self, key: &str) -> &'static str {
        match key {
            "c_embedded" => "C (beágyazott)",
            "c_win_gui" => "C (Windows/GUI)",
            "cpp_win_gui" => "C++ (Windows/GUI)",
            "cpp_linux_server" => "C++ (Linux/Szerver)",
            "csharp" => "C#",
            "sql" => "SQL",
            "rust" => "Rust / WebAssembly",
            "multimedia" => "Multimédia-programozás",
            "system_design" => "Rendszertervezés",
            "project_management" => "Projektmenedzsment",
            "test_management" => "Tesztmenedzsment",
            "automated_testing" => "Automatizált tesztelés",
            "manual_testing" => "Kézi tesztelés",
            "erp" => "ERP rendszerek",
            "administration" => "Rendszeradminisztráció",
            "leading" => "Vezetés",
            "auto_test_dev" => "Automatizált-teszt fejlesztés",
            "hungary" => "Magyarország",
            "germany" => "Németország",
            "austria" => "Ausztria",
            "lang_hungarian" => "Magyar (anyanyelv)",
            "lang_german" => "Német (folyékony)",
            "lang_english" => "Angol (folyékony)",
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
            "cert_diploma" => "Egyetemi diploma",
            "cert_pm" => "PM bizonyítvány (PMCC)",
            "cert_sql" => "Adatbázis és SQL alapismeretek",
            "cert_js" => "JavaScript bizonyítvány",
            "cert_ai" => "Programozás AI ügynökökkel",
            "cert_driving" => "Vezetéstechnikai tanfolyam",
            "cert_scrum" => "Scrum",
            "cert_istqb" => "ISTQB szeminárium",
            "cert_presentation" => "Prezentációs technikák",
            "cert_access" => "MS Access 2003 (középhaladó)",
            "cert_canoe" => "CANoe alapok",
            "cert_dotnet" => ".NET, C# tanfolyam",
            "cert_moderation" => "Moderáció – alapok",
            "cert_daf" => "Deutsch als Fremdsprache",
            "cert_germanistik" => "Internat. Sommersemester für Germanistik",
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
            "jr_admin" => "Adminisztrátor",
            "jr_sw_dev" => "Szoftverfejlesztő",
            "jr_dept_head" => "Részlegvezető",
            "jr_tester" => "Teszter",
            "jr_test_mgr" => "Tesztmenedzser",
            "jr_test_dev" => "Tesztfejlesztő",
            "jr_project_mgr" => "Projektmenedzser",
            // Projects (Sankey)
            "pj_labor" => "Labor",
            "pj_erp_system" => "ERP-Rendszer",
            "pj_db_finance" => "Deutsche Bank Építésfinanszírozás",
            "pj_cwl_kwg" => "C&L Deutsche Revision KWG Novelle",
            "pj_authors_dream" => "Author's Dream",
            "pj_citibank" => "Citibank Pénzmosás elleni védekezés",
            "pj_junction" => "Csomópont ellenőrzés",
            "pj_btc5000" => "BTC5000",
            "pj_sk24" => "SK24",
            "pj_ocit" => "OCIT",
            "pj_debrecen" => "Debrecen",
            "pj_medical" => "Orvosi képfeldolgozás",
            "pj_car_body" => "Személyautó Body Computer",
            "pj_dcdc" => "DC/DC átalakító",
            "pj_erp_bosch" => "Erőforrásgazdálkodás",
            "pj_test_designer" => "TestDesigner",
            "pj_test_net" => "Test.NET",
            "pj_truck_body" => "Kamion Body Computer",
            "pj_test_tools" => "Teszt eszközök feljavítása",
            "pj_window_lifter" => "Ablakemelő",
            "pj_contract" => "Szerződés (Auftrag)",
            "pj_asanet" => "ASA-Net",
            "pj_lynx" => "Lynx",
            "pj_sms_email" => "SMS-eMail",
            "pj_customer_cards" => "Ügyfélkártyák",
            "pj_e_billing" => "Elektronikus számlázás",
            "pj_mein_auto" => "Mein-Auto App",
            "pj_mobil_car" => "Mobil-Autóátvétel",
            "pj_supplier_inv" => "Szállító számlák ellenőrzése",
            "pj_prufloader" => "Prüfloader.NET",
            "pj_vortexledger" => "Vortexledger",
            "pj_georoute" => "GIS GeoRoute",
            // Expertise (Sankey)
            "ex_admin" => "Adminisztrálás",
            "ex_c_win" => "C win/GUI",
            "ex_c_embedded" => "C embedded",
            "ex_cpp_win" => "C++ win/GUI",
            "ex_cpp_linux" => "C++ linux/server",
            "ex_csharp" => "C#",
            "ex_sql" => "SQL",
            "ex_multimedia" => "Multimédia-programozás",
            "ex_system_design" => "Rendszertervezés (System-Design)",
            "ex_project_mgmt" => "Projektmenedzsment",
            "ex_leading" => "Vezetés",
            "ex_manual_test" => "Kézi tesztelés",
            "ex_auto_testing" => "Automatizált tesztelés",
            "ex_auto_test_dev" => "Automatizált-teszt fejlesztés",
            "ex_test_mgmt" => "Tesztmenedzsment",
            // Main Characteristics
            "mc_strengths" => "Erősségek",
            "mc_achievements" => "Főbb eredmények",
            "mc_countries" => "Országok",
            "mc_languages" => "Nyelvtudás",
            "mc_certificates" => "Bizonyítványok",
            // Tools
            "tl_visual_studio" => "Visual Studio",
            "tl_ms_office" => "MS Office (Excel, Word, Access)",
            "tl_clearcase" => "IBM ClearCase",
            "tl_clearquest" => "IBM ClearQuest",
            "tl_doors" => "IBM Doors",
            "tl_isystem" => "iSYSTEM debugger",
            "tl_dspace" => "dSPACE teszttorony",
            "tl_ni_teststand" => "NI TestStand",
            "tl_ni_cvi" => "NI LabWindows/CVI",
            "tl_agilent" => "Agilent mérőműszerek",
            "tl_opentest" => "OpenTest (Bosch/Vector)",
            "tl_authors_dream" => "Author's Dream",
            "tl_authorware" => "Macromedia Authorware",
            "tl_python" => "Python",
            "tl_snow" => "ServiceNow (SNOW)",
            "tl_keil" => "Keil fordító",
            "tl_cosmic" => "Cosmic C fordító",
            "tl_gnu_cc" => "GNU C/C++ eszközkészlet",
            "tl_tornado" => "Wind River Tornado IDE",
            "tl_java_sdk" => "Sun Java SDK",
            "tl_structured_text" => "Structured Text / LASAL",
            "tl_node_js" => "Node.js",
            _ => "",
        }
    }

    fn item_hint(&self, key: &str) -> &'static str {
        match key {
            "c_embedded" => "Mikrokontroller firmware forgalmi rendszerekhez és autóipari ECU-khoz",
            "c_win_gui" => "Windows asztali alkalmazások C-ben",
            "cpp_win_gui" => "Windows GUI alkalmazások C++-ban",
            "cpp_linux_server" => "Linux szerver-oldali fejlesztés C++-ban",
            "csharp" => "C# (.NET) — teszteszközök, ERP, Prüfloader",
            "sql" => "Adatbázis-lekérdezések, tárolt eljárások",
            "rust" => "Rust + Dioxus + WebAssembly webes alkalmazásokhoz",
            "multimedia" => "Multimédia és interaktív prezentációk",
            "system_design" => "Architektúra és rendszertervezés forgalomirányításhoz",
            "project_management" => "Csapatok vezetése és projektek menedzselése",
            "test_management" => "Tesztstratégia, tervezés és jelentéskészítés",
            "automated_testing" => "Automatizált HW/SW tesztfejlesztés",
            "manual_testing" => "Funkcionális és regressziós tesztelés",
            "erp" => "Vállalatirányítási rendszerek",
            "administration" => "Rendszer- és hálózat-adminisztráció",
            "leading" => "Vilati — szoftverfejlesztő részleg vezetése",
            "auto_test_dev" => "Bosch — automatizált teszteszközök fejlesztése",
            "hungary" => "Vilati, Mediso, MOL, Bäko-Hungaria, Bitnök, Telekom",
            "germany" => "Teamcom, Bosch",
            "austria" => "Porsche Informatik, Sigmatek",
            "lang_hungarian" => "Anyanyelv",
            "lang_german" => "Munkanyelv 1995 óta",
            "lang_english" => "Szakmai szintű tudás",
            "mol" => "Laborrendszer adminisztrálása",
            "bako" => "ERP rendszer adminisztrálása",
            "teamcom" => "Pénzügyi multimédia szoftver — Deutsche Bank, Citibank, C&L",
            "vilati" => "Forgalmi csomópontvezérlés, beágyazott C, részlegvezetés",
            "mediso" => "Orvosi képfeldolgozás",
            "bosch" => "Autóipari ECU fejlesztés és teszt — Body Computer, ablakemelő",
            "porsche" => "Kereskedésmenedzsment rendszerek — C++ Linux/Windows, SQL",
            "sigmatek" => "Prüfloader.NET — ipari teszteszköz C#-ban",
            "bitnok" => "Blockchain projektmenedzsment — VortexLedger",
            "telekom" => "GIS / GeoRoute adminisztráció",
            "cert_diploma" => "Mérnöki egyetemi diploma",
            "cert_pm" => "Projektmenedzsment bizonyítvány — PMCC",
            "cert_sql" => "Adatbázis és SQL alapismeretek tanfolyam",
            "cert_js" => "JavaScript fejlesztői bizonyítvány",
            "cert_ai" => "Programozás AI ügynökökkel tanfolyam",
            "cert_driving" => "Haladó vezetéstechnika a biztonságért",
            "cert_scrum" => "Scrum — PORSCHE belső tanfolyam",
            "cert_istqb" => "ISTQB szeminárium — Bosch belső tanfolyam",
            "cert_presentation" => "Prezentációs technikák — Develor tanfolyam",
            "cert_access" => "MS Access 2003 (középhaladó) — Controll Training",
            "cert_canoe" => "CANoe alapok — VECTOR",
            "cert_dotnet" => ".NET, C# tanfolyam — BME",
            "cert_moderation" => "Moderáció – alapok — Bosch belső",
            "cert_daf" => "Deutsch als Fremdsprache — Friedrich Schiller Universität Jena (TEMPUS ösztöndíj)",
            "cert_germanistik" => "Internat. Sommersemester für Germanistik — FSU Jena (Nemzetközi németnyelvi nyári tábor)",
            // Workplace hints
            "wk_mol" => "Laborrendszer adminisztrálása",
            "wk_bako" => "ERP rendszer adminisztrálása",
            "wk_teamcom" => "Pénzügyi multimédia szoftver — Deutsche Bank, Citibank, C&L",
            "wk_vilati" => "Forgalmi csomópontvezérlés, beágyazott C, részlegvezetés",
            "wk_mediso" => "Orvosi képfeldolgozás",
            "wk_bosch" => "Autóipari ECU fejlesztés és teszt — Body Computer, ablakemelő",
            "wk_porsche" => "Kereskedésmenedzsment rendszerek — C++ Linux/Windows, SQL",
            "wk_sigmatek" => "Prüfloader.NET — ipari teszteszköz C#-ban",
            "wk_bitnok" => "Blockchain projektmenedzsment — VortexLedger",
            "wk_telekom" => "GIS / GeoRoute adminisztráció",
            // Job role hints
            "jr_admin" => "MOL, Bäko-Hungaria, Telekom",
            "jr_sw_dev" => "Teamcom, Vilati, Mediso, Bosch, Porsche, Sigmatek",
            "jr_dept_head" => "Vilati — OCIT, Debrecen",
            "jr_tester" => "Bosch, Porsche Informatik",
            "jr_test_mgr" => "Bosch — Kamion Body Computer, teszt eszközök",
            "jr_test_dev" => "Bosch — Ablakemelő, Kamion Body Computer",
            "jr_project_mgr" => "Bitnök — Vortexledger",
            // Project hints
            "pj_labor" => "MOL AG — laborrendszer",
            "pj_erp_system" => "Bäko-Hungaria — vállalatirányítási rendszer",
            "pj_db_finance" => "Teamcom — Deutsche Bank építésfinanszírozás",
            "pj_cwl_kwg" => "Teamcom — C&L Deutsche Revision",
            "pj_authors_dream" => "Teamcom — multimédia szerkesztő",
            "pj_citibank" => "Teamcom — pénzmosás elleni védekezés",
            "pj_junction" => "Vilati — forgalmi csomópontvezérlés",
            "pj_btc5000" => "Vilati — forgalomirányító vezérlő",
            "pj_sk24" => "Vilati — lámpakapcsoló kártya, valós idejű szinkron",
            "pj_ocit" => "Vilati — részlegvezetés, OCIT szabvány",
            "pj_debrecen" => "Vilati — részlegvezetés, Debrecen projekt",
            "pj_medical" => "Mediso — orvosi képfeldolgozás",
            "pj_car_body" => "Bosch — személyautó Body Computer, C embedded",
            "pj_dcdc" => "Bosch — DC/DC átalakító, C embedded",
            "pj_erp_bosch" => "Bosch — erőforrásgazdálkodás, C#",
            "pj_test_designer" => "Bosch — teszttervező eszköz, C#",
            "pj_test_net" => "Bosch — tesztfejlesztő-framework, C#",
            "pj_truck_body" => "Bosch — kamion Body Computer, tesztmenedzsment",
            "pj_test_tools" => "Bosch — teszt eszközök feljavítása",
            "pj_window_lifter" => "Bosch — ablakemelő, automatizált tesztelés",
            "pj_contract" => "Porsche — szerződéskezelés, C++ Linux/Windows",
            "pj_asanet" => "Porsche — ASA-Net, C++ Linux/Windows",
            "pj_lynx" => "Porsche — Lynx, C++ linux/server",
            "pj_sms_email" => "Porsche — SMS-eMail, C++ Linux/Windows",
            "pj_customer_cards" => "Porsche — ügyfélkártyák, C++ Linux/Windows",
            "pj_e_billing" => "Porsche — elektronikus számlázás, C++ Linux/Windows",
            "pj_mein_auto" => "Porsche — Mein-Auto App, C++ linux/server",
            "pj_mobil_car" => "Porsche — mobil-autóátvétel, C++ linux/server",
            "pj_supplier_inv" => "Porsche — szállító számlák, C++ Linux/Windows",
            "pj_prufloader" => "Sigmatek — ipari teszteszköz, C#",
            "pj_vortexledger" => "Bitnök — blockchain open ledger",
            "pj_georoute" => "Telekom — GIS/GeoRoute adminisztráció",
            // Expertise hints
            "ex_admin" => "MOL, Bäko-Hungaria, Telekom",
            "ex_c_win" => "Teamcom, Mediso — Windows asztali alkalmazások",
            "ex_c_embedded" => "Vilati, Bosch — mikrokontroller firmware",
            "ex_cpp_win" => "Mediso, Porsche — C++ Windows GUI",
            "ex_cpp_linux" => "Porsche — C++ Linux szerver",
            "ex_csharp" => "Bosch, Sigmatek — .NET alkalmazások",
            "ex_sql" => "Porsche — adatbázisok, tárolt eljárások",
            "ex_multimedia" => "Teamcom — interaktív pénzügyi prezentációk",
            "ex_system_design" => "Vilati — forgalomirányítási rendszertervezés",
            "ex_project_mgmt" => "Vilati, Bitnök — projektmenedzsment",
            "ex_leading" => "Vilati — szoftverfejlesztő részleg vezetése",
            "ex_manual_test" => "Bosch, Porsche — funkcionális és regressziós tesztelés",
            "ex_auto_testing" => "Bosch — automatizált HW/SW tesztelés",
            "ex_auto_test_dev" => "Bosch — automatizált teszteszközök fejlesztése",
            "ex_test_mgmt" => "Bosch — tesztstratégia, tervezés, jelentéskészítés",
            // Main Characteristics hints
            "mc_strengths" => "Szakértelem, vezetés és problémamegoldás",
            "mc_achievements" => "Test.NET, SK24 szinkronizáció, Vortexledger",
            "mc_countries" => "Magyarország, Németország, Ausztria",
            "mc_languages" => "Magyar, német, angol",
            "mc_certificates" => "Diploma, PM, SQL, JavaScript, AI, vezetéstechnika",
            // Tool hints
            "tl_visual_studio" => "Bosch, Vilati, Teamcom, Mediso",
            "tl_ms_office" => "Bosch — dokumentáció, tervezés",
            "tl_clearcase" => "Bosch — verziókezelés",
            "tl_clearquest" => "Bosch — hibakövetés",
            "tl_doors" => "Bosch — követelménymenedzsment",
            "tl_isystem" => "Bosch — mikrokontroller debuggolás",
            "tl_dspace" => "Bosch — HiL tesztelés",
            "tl_ni_teststand" => "Bosch — automatizált tesztelés",
            "tl_ni_cvi" => "Bosch — félautomatikus tesztelés",
            "tl_agilent" => "Bosch — mérőműszerek ablakemelő teszthez",
            "tl_opentest" => "Bosch — automatizált teszteszköz (most Vector)",
            "tl_authors_dream" => "Teamcom — multimédia szerkesztő",
            "tl_authorware" => "Teamcom — Citibank multimédia",
            "tl_python" => "Bosch — TestDesigner szkriptek",
            "tl_snow" => "Telekom — ITSM jegykezelés",
            "tl_keil" => "Vilati — C167 mikrokontroller fordító",
            "tl_cosmic" => "Bosch — autóipari mikrokontroller fordító",
            "tl_gnu_cc" => "Vilati — OCIT firmware Linux alatt",
            "tl_tornado" => "Vilati — VxWorks valós idejű fejlesztés",
            "tl_java_sdk" => "Vilati — Java Native Interface",
            "tl_structured_text" => "Sigmatek — ipari vezérlőprogramozás",
            "tl_node_js" => "Bitnök — open ledger eszközök",
            _ => "",
        }
    }

    fn toast_update_available(&self) -> &'static str { "Új verzió érhető el!" }
    fn btn_update_now(&self) -> &'static str { "Frissítés" }

    fn pdf_filter_choices(&self) -> &'static str { "Szűrő választások" }
    fn btn_generate_pdf(&self) -> &'static str { "PDF generálás" }

    fn role_section_label(&self) -> &'static str { "Pozíció" }
    fn role_label(&self, key: &str) -> &'static str {
        match key {
            "po" => "Product Owner",
            "szm" => "Szoftvermérnök",
            "tm" => "Tesztmenedzser",
            _ => "",
        }
    }
    fn role_title(&self, key: &str) -> &'static str {
        match key {
            "po" => "Product Owner",
            "szm" => "Szoftvermérnök",
            "tm" => "Tesztmenedzser",
            _ => "Szoftvermérnök",
        }
    }
    fn role_target_title(&self) -> &'static str { "Célkitűzés" }
    fn role_target_text(&self, key: &str) -> &'static str {
        match key {
            "po" => "Tapasztalt szoftvermérnökként célom, hogy product owneri szerepkörben \
                kamatoztassam kiterjedt termék- és folyamatmenedzsment tapasztalataimat. \
                Célom a felhasználói élmény javítása ügyfél-visszajelzések elemzésén, \
                agilis fejlesztési környezetben szerzett gyakorlatommal, valamint \
                rendszerszintű gondolkodásommal.",
            "szm" => "Tapasztalt szoftvermérnökként, több mint egy évtizedes iparági \
                tapasztalattal (különösen C, C#, C++, SQL, beágyazott rendszerek területén), \
                szeretném tudásomat és innovatív problémamegoldó készségemet egy dinamikus, \
                növekvő szervezetben kamatoztatni. Célom, hogy magas minőségű \
                szoftvermegoldások fejlesztésével hozzájáruljak a szervezet sikeréhez, \
                miközben folyamatosan fejlesztem szakmai ismereteimet az új technológiák terén.",
            "tm" => "Több évtizedes szoftverfejlesztési háttérrel rendelkezem, kiterjedt \
                tesztmenedzseri tapasztalattal komplex rendszerek területén. Szakértelmem \
                kiterjed a tesztelési folyamatok megtervezésére, menedzselésére és \
                automatizálására, valamint a minőségbiztosításra. Jártas vagyok a manuális \
                és automatizált tesztelés módszertanaiban.",
            _ => "",
        }
    }
    fn role_strengths_title(&self) -> &'static str { "Erősségek" }
    fn role_strengths(&self, key: &str) -> &'static [(&'static str, &'static str)] {
        match key {
            "po" => &[
                ("Termék- és folyamatmenedzsment",
                 "Különböző banki és autóipari rendszerek fejlesztésében, támogatásában és optimalizálásában szerzett gyakorlat."),
                ("Analitikus szemlélet",
                 "Komplex szoftveres és HW közeli programozási feladatok megoldása."),
                ("Precíz munkavégzés",
                 "Részletekre való odafigyelés és pontos dokumentáció."),
                ("Együttműködés és kommunikáció",
                 "Fejlesztői csapatok vezetésében és társterületekkel való együttműködésben szerzett tapasztalat."),
                ("Agilis környezet",
                 "Digitális csatornák és agilis fejlesztési környezet ismerete."),
            ],
            "szm" => &[
                ("Szoftverfejlesztés és tesztelés",
                 "Széleskörű tapasztalat C, C++, C# nyelvekkel, beleértve valós idejű rendszerek szinkronizációját és automatizált tesztkeretrendszerek kidolgozását."),
                ("Rendszerszintű gondolkodás",
                 "Gépészmérnöki háttér, amely segíti a hardver-közeli programozást és komplex mechatronikai rendszerek megértését."),
                ("Projektmenedzsment és vezetés",
                 "Szoftverfejlesztő részleg vezetése, rendszerteszt-koordináció, Scrum és ISTQB módszertanok ismerete."),
                ("Technológiai sokszínűség",
                 "Linux, Windows, SQL, webfejlesztés és DLT koncepciók ismerete."),
                ("Problémamegoldás",
                 "Gyors tanulási készség, kiváló hibakeresési és javítási képesség."),
            ],
            "tm" => &[
                ("Minőségorientált megközelítés",
                 "Elkötelezettség a legmagasabb szakmai előírásoknak való megfelelés iránt (ISTQB, Bosch belső standardok)."),
                ("Proaktív problémamegoldás",
                 "Módszeres és kitartó problémagyökér-keresés, optimális megoldások vagy kompromisszumok megtalálása."),
                ("Prototípusok és frameworkök",
                 "Belső teszt-frameworkök (pl. Test.NET) tervezése és kivitelezése a tesztautomatizálás megvalósításához."),
                ("Realisztikus tesztelési stratégia",
                 "A tesztelési környezet validációjának és a teszteredmények produkálásának összehangolása a tervezés során."),
            ],
            _ => &[],
        }
    }
}
