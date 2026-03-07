use super::UiI18n;

pub struct FinnishUi;

impl UiI18n for FinnishUi {
    fn tab_profile(&self) -> &'static str { "Profiili" }
    fn tab_filter(&self) -> &'static str { "Suodatin" }
    fn tab_display(&self) -> &'static str { "Näyttö" }

    fn profile_name(&self) -> &'static str { "István Finta" }
    fn profile_subtitle(&self) -> &'static str { "Konetekniikan diplomi-insinööri, robotiikka ja tietotekniikka" }
    fn profile_title(&self) -> &'static str { "Ohjelmistoinsinööri" }
    fn profile_location(&self) -> &'static str { "Unkari" }
    fn profile_email(&self) -> &'static str { "istvan_finta@yahoo.com" }
    fn profile_phone(&self) -> &'static str { "+36 70 343 9517" }

    fn section_skills(&self) -> &'static str { "Tekniset taidot" }
    fn section_countries(&self) -> &'static str { "Maat" }
    fn section_languages(&self) -> &'static str { "Kielitaito" }
    fn section_companies(&self) -> &'static str { "Yritykset" }
    fn section_certificates(&self) -> &'static str { "Sertifikaatit" }
    fn filter_select_all(&self) -> &'static str { "Valitse kaikki" }
    fn filter_clear_all(&self) -> &'static str { "Tyhjennä kaikki" }

    fn section_job_roles(&self) -> &'static str { "Työtehtävät" }
    fn section_projects(&self) -> &'static str { "Projektit" }
    fn section_tools(&self) -> &'static str { "Työkalut" }
    fn section_project_experience(&self) -> &'static str { "Projektikokemus" }

    fn section_main_chars(&self) -> &'static str { "Tärkeimmät ominaisuuteni" }
    fn role_achievements_title(&self) -> &'static str { "Keskeiset saavutukset" }
    fn digital_skills_title(&self) -> &'static str { "Digitaaliset taidot" }
    fn digital_skills_text(&self) -> &'static str {
        "Kattavat.\n\n\
         Käyttöjärjestelmät: Windows, Debian Linux, iOS, Android, OSX, VxWorks, ...\n\
         Kehitystyökalut, makefilet, kääntäjät (C, C#, C++, jonkin verran Rustia, ...), \
         tulkit (Python, ...), tekstieditorit (nano, kate, MS Code, ...)\n\
         MS Office, Word, Excel, ... (Libre Office ja vastaavat myös), Selaimet, Web-\
         sovellukset, Pilvipalveluntarjoajat (Vultr, Oracle, ...)\n\
         Grafiikkasovellukset, OpenSCAD (3D-suunnittelu), Eagle (piirisuunnittelu), Gimp, \
         kuvakaappaustyökalut, kuvankäsittelytyökalut, ...\n\
         AI:n perusteet, edistynyt DLT (Distributed Ledger Technology) -osaaminen, \
         Stellar Network, älykkäät sopimukset, ... Web-kehityksen perusteet, HTML5, CSS, \
         JavaScript, jonkin verran Reactia, jonkin verran NodeJS:ää, ...\n\n\
         Omaksun nopeasti sovelluksia, ohjelmointikieliä ja käyttöjärjestelmiä."
    }
    fn role_achievements(&self) -> &'static [(&'static str, &'static str)] {
        &[
            ("Test.NET testinkehityskehys",
             "Myöhemmin Bosch myi sen Vectorille."),
            ("SK24 lampunkytkentäkortin synkronointi",
             "Liikenteenohjauskäyttöön tarkoitetun lampunkytkentäkortin sisäisten prosessien reaaliaikainen synkronointi sähköverkon siniaaltoon."),
            ("Vortexledger-konsepti",
             "Avoin tilikirja ja talouskonsepti, joka perustuu vaihtokauppaan."),
        ]
    }

    fn display_nothing_selected(&self) -> &'static str { "Valitse kohteita Suodatin-välilehdeltä nähdäksesi tiedot täällä." }

    fn item_label(&self, key: &str) -> &'static str {
        match key {
            // Skills
            "c_embedded" => "C (sulautettu)",
            "c_win_gui" => "C (Windows/GUI)",
            "cpp_win_gui" => "C++ (Windows/GUI)",
            "cpp_linux_server" => "C++ (Linux/Palvelin)",
            "csharp" => "C#",
            "sql" => "SQL",
            "rust" => "Rust / WebAssembly",
            "multimedia" => "Multimediaohjelmointi",
            "system_design" => "Järjestelmäsuunnittelu",
            "project_management" => "Projektinhallinta",
            "test_management" => "Testauksen hallinta",
            "automated_testing" => "Automatisoitu testaus",
            "manual_testing" => "Manuaalinen testaus",
            "erp" => "ERP-järjestelmät",
            "administration" => "Järjestelmähallinnointi",
            "leading" => "Johtaminen",
            "auto_test_dev" => "Automaattitestien kehitys",
            // Countries
            "hungary" => "Unkari",
            "germany" => "Saksa",
            "austria" => "Itävalta",
            // Languages
            "lang_hungarian" => "Unkari",
            "lang_german" => "Saksa",
            "lang_english" => "Englanti",
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
            "cert_diploma" => "Yliopiston tutkinto",
            "cert_pm" => "PM-sertifikaatti (PMCC)",
            "cert_sql" => "Tietokanta & SQL perusteet",
            "cert_js" => "JavaScript-sertifikaatti",
            "cert_ai" => "Ohjelmointi tekoälyagenteilla",
            "cert_driving" => "Edistynyt ajotekniikka",
            "cert_scrum" => "Scrum",
            "cert_istqb" => "ISTQB-seminaari",
            "cert_presentation" => "Esitystekniikat",
            "cert_access" => "MS Access 2003 (keskitaso)",
            "cert_canoe" => "CANoe perusteet",
            "cert_dotnet" => ".NET, C# kurssi",
            "cert_moderation" => "Moderointi — perusteet",
            "cert_daf" => "Saksa vieraana kielenä",
            "cert_germanistik" => "Kansainvälinen kesälukukausi germanistiikassa",
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
            "jr_admin" => "Järjestelmänvalvoja",
            "jr_sw_dev" => "Ohjelmistokehittäjä",
            "jr_dept_head" => "Osastopäällikkö",
            "jr_tester" => "Testaaja",
            "jr_test_mgr" => "Testauspäällikkö",
            "jr_test_dev" => "Testikehittäjä",
            "jr_project_mgr" => "Projektipäällikkö",
            // Projects (Sankey)
            "pj_labor" => "Laboratorio",
            "pj_erp_system" => "ERP-järjestelmä",
            "pj_db_finance" => "Deutsche Bank Rakennusrahoitus",
            "pj_cwl_kwg" => "C&L Deutsche Revision KWG Novelle",
            "pj_authors_dream" => "Author's Dream",
            "pj_citibank" => "Citibank Rahanpesun esto",
            "pj_junction" => "Risteyksen ohjaustyökalu",
            "pj_btc5000" => "BTC5000",
            "pj_sk24" => "SK24",
            "pj_ocit" => "OCIT",
            "pj_debrecen" => "Debrecen",
            "pj_medical" => "Lääketieteellinen kuvantaminen",
            "pj_car_body" => "Auton korisähkötietokone",
            "pj_dcdc" => "DC/DC-muunnin",
            "pj_erp_bosch" => "ERP / Resurssisuunnittelu",
            "pj_test_designer" => "TestDesigner",
            "pj_test_net" => "Test.NET",
            "pj_truck_body" => "Kuorma-auton korisähkötietokone",
            "pj_test_tools" => "Testaustyökalujen parantaminen",
            "pj_window_lifter" => "Ikkunannostin",
            "pj_contract" => "Sopimus (Auftrag)",
            "pj_asanet" => "ASA-Net",
            "pj_lynx" => "Lynx",
            "pj_sms_email" => "SMS-eMail",
            "pj_customer_cards" => "Asiakaskortit",
            "pj_e_billing" => "Sähköinen laskutus",
            "pj_mein_auto" => "Mein-Auto App",
            "pj_mobil_car" => "Mobiili auton vastaanotto",
            "pj_supplier_inv" => "Toimittajalaskujen valvonta",
            "pj_prufloader" => "Prüfloader.NET",
            "pj_vortexledger" => "Vortexledger",
            "pj_georoute" => "GIS GeoRoute",
            // Expertise (Sankey)
            "ex_admin" => "Hallinnointi",
            "ex_c_win" => "C win/GUI",
            "ex_c_embedded" => "C sulautettu",
            "ex_cpp_win" => "C++ win/GUI",
            "ex_cpp_linux" => "C++ linux/palvelin",
            "ex_csharp" => "C#",
            "ex_sql" => "SQL",
            "ex_multimedia" => "Multimediaohjelmointi",
            "ex_system_design" => "Järjestelmäsuunnittelu",
            "ex_project_mgmt" => "Projektinhallinta",
            "ex_leading" => "Johtaminen",
            "ex_manual_test" => "Manuaalinen testaus",
            "ex_auto_testing" => "Automatisoitu testaus",
            "ex_auto_test_dev" => "Automaattitestien kehitys",
            "ex_test_mgmt" => "Testauksen hallinta",
            // Main Characteristics
            "mc_strengths" => "Vahvuudet",
            "mc_achievements" => "Keskeiset saavutukset",
            "mc_countries" => "Maat",
            "mc_languages" => "Kielitaito",
            "mc_certificates" => "Sertifikaatit",
            "mc_digital_skills" => "Digitaaliset taidot",
            // Tools
            "tl_visual_studio" => "Visual Studio",
            "tl_ms_office" => "MS Office (Excel, Word, Access)",
            "tl_clearcase" => "IBM ClearCase",
            "tl_clearquest" => "IBM ClearQuest",
            "tl_doors" => "IBM Doors",
            "tl_isystem" => "iSYSTEM Debugger",
            "tl_dspace" => "dSPACE Testiturni",
            "tl_ni_teststand" => "NI TestStand",
            "tl_ni_cvi" => "NI LabWindows/CVI",
            "tl_agilent" => "Agilent-instrumentit",
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
            "c_embedded" => "Mikrokontrolleriohjelmisto liikennejärjestelmiin ja autojen ECU:ihin",
            "c_win_gui" => "Windows-työpöytäsovelluksia C-kielellä",
            "cpp_win_gui" => "Windows GUI -sovelluksia C++:lla",
            "cpp_linux_server" => "Linux-palvelinkehitystä C++:lla",
            "csharp" => "C# (.NET) — testityökalut, ERP, Prüfloader",
            "sql" => "Tietokantakyselyt, tallennetut proseduurit",
            "rust" => "Rust + Dioxus + WebAssembly verkkosovelluksiin",
            "multimedia" => "Multimedia ja interaktiiviset esitykset",
            "system_design" => "Arkkitehtuuri ja järjestelmäsuunnittelu liikenteenohjaukseen",
            "project_management" => "Tiimien johtaminen ja projektinhallinta",
            "test_management" => "Testausstrategia, suunnittelu ja raportointi",
            "automated_testing" => "Automatisoitu HW/SW-testikehitys",
            "manual_testing" => "Toiminnallinen testaus ja regressiotestaus",
            "erp" => "Toiminnanohjausjärjestelmät",
            "administration" => "Järjestelmä- ja verkkohallinta",
            "leading" => "Vilati — ohjelmistokehitysosaston johtaminen",
            "auto_test_dev" => "Bosch — automatisoitujen testityökalujen kehitys",
            "hungary" => "Vilati, Mediso, MOL, Bäko-Hungaria, Bitnök, Telekom",
            "germany" => "Teamcom, Bosch",
            "austria" => "Porsche Informatik, Sigmatek",
            "lang_hungarian" => "Äidinkieli",
            "lang_german" => "Oli sujuva, pitkään käyttämättä",
            "lang_english" => "Peruskeskustelu, tekninen kirjallisuus, sähköposti, chat",
            "mol" => "Laboratorion järjestelmähallinta",
            "bako" => "ERP-järjestelmähallinta",
            "teamcom" => "Finanssimultimediaohjelmisto — Deutsche Bank, Citibank, C&L",
            "vilati" => "Liikenteen risteyksenohjaus, sulautettu C, osastonjohtaminen",
            "mediso" => "Lääketieteellinen kuvankäsittely",
            "bosch" => "Auto-ECU-kehitys ja testaus — Body Computers, ikkunannostimet",
            "porsche" => "Jälleenmyyjähallintajärjestelmät — C++ Linux/Windows, SQL",
            "sigmatek" => "Prüfloader.NET — teollinen testityökalu C#:lla",
            "bitnok" => "Lohkoketjuprojektinhallinta — VortexLedger",
            "telekom" => "GIS / GeoRoute hallinta",
            "cert_diploma" => "Yliopiston insinööritutkinto",
            "cert_pm" => "Projektinhallinnan sertifikaatti — PMCC",
            "cert_sql" => "Tietokantojen ja SQL:n perusteiden kurssi",
            "cert_js" => "JavaScript-kehityssertifikaatti",
            "cert_ai" => "Ohjelmointi tekoälyagenteilla -kurssi",
            "cert_driving" => "Edistynyt ajotekniikka turvallisuudeksi",
            "cert_scrum" => "Scrum — PORSCHEn sisäinen kurssi",
            "cert_istqb" => "ISTQB-seminaari — Boschin sisäinen kurssi",
            "cert_presentation" => "Esitystekniikat — Develor-kurssi",
            "cert_access" => "MS Access 2003 (keskitaso) — Controll Training",
            "cert_canoe" => "CANoe perusteet — VECTOR",
            "cert_dotnet" => ".NET, C# kurssi — BME",
            "cert_moderation" => "Moderointi — perusteet — Bosch sisäinen",
            "cert_daf" => "Saksa vieraana kielenä — Friedrich Schiller -yliopisto Jena (TEMPUS-stipendi)",
            "cert_germanistik" => "Kansainvälinen kesälukukausi germanistiikassa — FSU Jena",
            // Workplace hints
            "wk_mol" => "Laboratorion järjestelmähallinta",
            "wk_bako" => "ERP-järjestelmähallinta",
            "wk_teamcom" => "Finanssimultimediaohjelmisto — Deutsche Bank, Citibank, C&L",
            "wk_vilati" => "Liikenteen risteyksenohjaus, sulautettu C, osastonjohtaminen",
            "wk_mediso" => "Lääketieteellinen kuvankäsittely",
            "wk_bosch" => "Auto-ECU-kehitys ja testaus — Body Computers, ikkunannostimet",
            "wk_porsche" => "Jälleenmyyjähallintajärjestelmät — C++ Linux/Windows, SQL",
            "wk_sigmatek" => "Prüfloader.NET — teollinen testityökalu C#:lla",
            "wk_bitnok" => "Lohkoketjuprojektinhallinta — VortexLedger",
            "wk_telekom" => "GIS / GeoRoute hallinta",
            // Job role hints
            "jr_admin" => "MOL, Bäko-Hungaria, Telekom",
            "jr_sw_dev" => "Teamcom, Vilati, Mediso, Bosch, Porsche, Sigmatek",
            "jr_dept_head" => "Vilati — OCIT, Debrecen",
            "jr_tester" => "Bosch, Porsche Informatik",
            "jr_test_mgr" => "Bosch — Kuorma-auton Body Computer, testityökalut",
            "jr_test_dev" => "Bosch — Ikkunannostin, Kuorma-auton Body Computer",
            "jr_project_mgr" => "Bitnök — Vortexledger",
            // Project hints
            "pj_labor" => "MOL AG — laboratoriojärjestelmä",
            "pj_erp_system" => "Bäko-Hungaria — toiminnanohjaus",
            "pj_db_finance" => "Teamcom — Deutsche Bank rakennusrahoitus",
            "pj_cwl_kwg" => "Teamcom — C&L Deutsche Revision",
            "pj_authors_dream" => "Teamcom — multimediaeditori",
            "pj_citibank" => "Teamcom — rahanpesun esto",
            "pj_junction" => "Vilati — liikenteen risteyksenohjaus",
            "pj_btc5000" => "Vilati — liikenneohjain",
            "pj_sk24" => "Vilati — lampunkytkentäkortti, reaaliaikainen synkronointi",
            "pj_ocit" => "Vilati — osastonjohtaminen, OCIT-standardi",
            "pj_debrecen" => "Vilati — osastonjohtaminen, Debrecen-projekti",
            "pj_medical" => "Mediso — lääketieteellinen kuvantaminen",
            "pj_car_body" => "Bosch — auton Body Computer, C sulautettu",
            "pj_dcdc" => "Bosch — DC/DC-muunnin, C sulautettu",
            "pj_erp_bosch" => "Bosch — resurssisuunnittelu, C#",
            "pj_test_designer" => "Bosch — testisuunnittelutyökalu, C#",
            "pj_test_net" => "Bosch — testinkehityskehys, C#",
            "pj_truck_body" => "Bosch — kuorma-auton Body Computer, testauksenhallinta",
            "pj_test_tools" => "Bosch — testityökalujen parantaminen",
            "pj_window_lifter" => "Bosch — ikkunannostin, automatisoitu testaus",
            "pj_contract" => "Porsche — sopimushallinta, C++ Linux/Windows",
            "pj_asanet" => "Porsche — ASA-Net, C++ Linux/Windows",
            "pj_lynx" => "Porsche — Lynx, C++ linux/palvelin",
            "pj_sms_email" => "Porsche — SMS-eMail, C++ Linux/Windows",
            "pj_customer_cards" => "Porsche — asiakaskortit, C++ Linux/Windows",
            "pj_e_billing" => "Porsche — sähköinen laskutus, C++ Linux/Windows",
            "pj_mein_auto" => "Porsche — Mein-Auto App, C++ linux/palvelin",
            "pj_mobil_car" => "Porsche — mobiili auton vastaanotto, C++ linux/palvelin",
            "pj_supplier_inv" => "Porsche — toimittajalaskut, C++ Linux/Windows",
            "pj_prufloader" => "Sigmatek — teollinen testityökalu, C#",
            "pj_vortexledger" => "Bitnök — lohkoketjun avoin tilikirja",
            "pj_georoute" => "Telekom — GIS/GeoRoute hallinta",
            // Expertise hints
            "ex_admin" => "MOL, Bäko-Hungaria, Telekom",
            "ex_c_win" => "Teamcom, Mediso — Windows-työpöytäsovellukset",
            "ex_c_embedded" => "Vilati, Bosch — mikrokontrolleriohjelmisto",
            "ex_cpp_win" => "Mediso, Porsche — C++ Windows GUI",
            "ex_cpp_linux" => "Porsche — C++ Linux-palvelin",
            "ex_csharp" => "Bosch, Sigmatek — .NET-sovellukset",
            "ex_sql" => "Porsche — tietokannat, tallennetut proseduurit",
            "ex_multimedia" => "Teamcom — interaktiiviset finanssiesitykset",
            "ex_system_design" => "Vilati — liikenteenohjauksen järjestelmäsuunnittelu",
            "ex_project_mgmt" => "Vilati, Bitnök — projektinhallinta",
            "ex_leading" => "Vilati — ohjelmistokehitysosaston johtaminen",
            "ex_manual_test" => "Bosch, Porsche — toiminnallinen ja regressiotestaus",
            "ex_auto_testing" => "Bosch — automatisoitu HW/SW-testaus",
            "ex_auto_test_dev" => "Bosch — automatisoitujen testityökalujen kehitys",
            "ex_test_mgmt" => "Bosch — testausstrategia, suunnittelu ja raportointi",
            // Main Characteristics hints
            "mc_strengths" => "Asiantuntemus, johtajuus ja ongelmanratkaisu",
            "mc_achievements" => "Test.NET, SK24-synkronointi, Vortexledger",
            "mc_countries" => "Unkari, Saksa, Itävalta",
            "mc_languages" => "Unkari, saksa, englanti",
            "mc_certificates" => "Tutkinto, PM, SQL, JavaScript, AI, ajotekniikka",
            "mc_digital_skills" => "Kattavat digitaaliset taidot",
            // Tool hints
            "tl_visual_studio" => "Bosch, Vilati, Teamcom, Mediso",
            "tl_ms_office" => "Bosch — dokumentointi, suunnittelu",
            "tl_clearcase" => "Bosch — versionhallinta",
            "tl_clearquest" => "Bosch — virheiden seuranta",
            "tl_doors" => "Bosch — vaatimustenhallinta",
            "tl_isystem" => "Bosch — mikrokontrollerin virheenkorjaus",
            "tl_dspace" => "Bosch — HiL-testaus",
            "tl_ni_teststand" => "Bosch — automatisoitu testaus",
            "tl_ni_cvi" => "Bosch — puoliautomaattinen testaus",
            "tl_agilent" => "Bosch — instrumentit ikkunannostintestiin",
            "tl_opentest" => "Bosch — automatisoitu testityökalu (nyt Vector)",
            "tl_authors_dream" => "Teamcom — multimediaeditori",
            "tl_authorware" => "Teamcom — Citibank multimedia",
            "tl_python" => "Bosch — TestDesigner-skriptit",
            "tl_snow" => "Telekom — ITSM-tikettijärjestelmä",
            "tl_keil" => "Vilati — C167 mikrokontrollerikääntäjä",
            "tl_cosmic" => "Bosch — autoteollisuuden mikrokontrollerikääntäjä",
            "tl_gnu_cc" => "Vilati — OCIT-ohjelmisto Linuxilla",
            "tl_tornado" => "Vilati — VxWorks reaaliaikakehitys",
            "tl_java_sdk" => "Vilati — Java Native Interface",
            "tl_structured_text" => "Sigmatek — teollinen ohjainohjelmointi",
            "tl_node_js" => "Bitnök — avoimen tilikirjan työkalut",
            _ => "",
        }
    }

    fn btn_generate_pdf(&self) -> &'static str { "Luo PDF" }

    fn role_section_label(&self) -> &'static str { "Rooli" }
    fn role_label(&self, key: &str) -> &'static str {
        match key {
            "po" => "Product Owner",
            "szm" => "Ohjelmistoinsinööri",
            "tm" => "Testauspäällikkö",
            _ => "",
        }
    }
    fn role_title(&self, key: &str) -> &'static str {
        match key {
            "po" => "Product Owner",
            "szm" => "Ohjelmistoinsinööri",
            "tm" => "Testauspäällikkö",
            _ => "Ohjelmistoinsinööri",
        }
    }
    fn role_target_title(&self) -> &'static str { "Tavoite" }
    fn role_target_text(&self, key: &str) -> &'static str {
        match key {
            "po" => "Kokeneena ohjelmistoinsinöörinä tavoitteeni on hyödyntää laajaa tuote- \
                ja prosessinhallintakokemustani Product Owner -roolissa. Pyrin parantamaan \
                käyttäjäkokemusta asiakaspalautteen analysoinnin, ketterien \
                kehityskäytäntöjen ja järjestelmätason ajattelun avulla.",
            "szm" => "Kokeneena ohjelmistoinsinöörinä, yli vuosikymmenen teollisuuskokemuksella \
                (erityisesti C, C#, C++, SQL, sulautetut järjestelmät), haluan soveltaa \
                tietotaitoani ja innovatiivista ongelmanratkaisukykyäni dynaamisessa, \
                kasvavassa organisaatiossa. Tavoitteenani on edistää organisaation \
                menestystä kehittämällä korkealaatuisia ohjelmistoratkaisuja samalla kun \
                laajennan jatkuvasti osaamistani uusissa teknologioissa.",
            "tm" => "Vuosikymmenten ohjelmistokehitystaustalla minulla on laaja \
                testauksenhallintakokemus monimutkaisissa järjestelmissä. Osaamiseni kattaa \
                testausprosessien suunnittelun, hallinnan ja automatisoinnin sekä \
                laadunvarmistuksen. Hallitsen manuaalisen ja automatisoidun testauksen \
                menetelmät.",
            _ => "",
        }
    }
    fn role_strengths_title(&self) -> &'static str { "Vahvuudet" }
    fn role_strengths(&self, key: &str) -> &'static [(&'static str, &'static str)] {
        match key {
            "po" => &[
                ("Tuote- ja prosessinhallinta",
                 "Kokemus erilaisten pankki- ja autoteollisuusjärjestelmien kehittämisestä, tukemisesta ja optimoinnista."),
                ("Analyyttinen ajattelu",
                 "Monimutkaisten ohjelmisto- ja laitteistotason ohjelmointitehtävien ratkaisu."),
                ("Tarkkuus ja huolellisuus",
                 "Huolellinen työskentely ja tarkka dokumentointi."),
                ("Yhteistyö ja viestintä",
                 "Kokemus kehitystiimien johtamisesta ja poikkitoiminnallisesta yhteistyöstä."),
                ("Ketterä ympäristö",
                 "Digitaalisten kanavien ja ketterien kehitysympäristöjen tuntemus."),
            ],
            "szm" => &[
                ("Ohjelmistokehitys ja testaus",
                 "Laaja kokemus C, C++, C# -kielistä sisältäen reaaliaikajärjestelmien synkronoinnin ja automatisoitujen testikehysten kehittämisen."),
                ("Järjestelmätason ajattelu",
                 "Konetekniikan tausta tukee laitteistotason ohjelmointia ja monimutkaisten mekatronisten järjestelmien ymmärtämistä."),
                ("Projektinhallinta ja johtaminen",
                 "Ohjelmistokehitysosastojen johtaminen, järjestelmätestauksen koordinointi, Scrum- ja ISTQB-menetelmät."),
                ("Teknologinen monipuolisuus",
                 "Linux, Windows, SQL, web-kehitys ja DLT-konseptien osaaminen."),
                ("Ongelmanratkaisu",
                 "Nopea oppija erinomaisilla virheenkorjaus- ja vianetsintätaidoilla."),
            ],
            "tm" => &[
                ("Laatulähtöinen lähestymistapa",
                 "Sitoutuminen korkeimpiin ammattistandardeihin (ISTQB, Boschin sisäiset standardit)."),
                ("Proaktiivinen ongelmanratkaisu",
                 "Järjestelmällinen ja sinnikäs juurisyyanalyysi, optimaalisten ratkaisujen tai kompromissien löytäminen."),
                ("Prototyypit ja kehykset",
                 "Sisäisten testikehysten (esim. Test.NET) suunnittelu ja toteutus testiautomatisoinnille."),
                ("Realistinen testausstrategia",
                 "Testiympäristön validoinnin ja testitulosten tuotannon aikatauluttaminen suunnittelussa."),
            ],
            _ => &[],
        }
    }
    fn pwa_hint(&self) -> &'static str {
        "Tämä on PWA-sovellus \u{2014} voit tallentaa sen selaimestasi puhelimesi aloitusnäytölle kuin natiivisovelluksen"
    }
}
