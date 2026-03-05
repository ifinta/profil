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
            _ => "",
        }
    }

    fn toast_update_available(&self) -> &'static str { "Új verzió érhető el!" }
    fn btn_update_now(&self) -> &'static str { "Frissítés" }
}
