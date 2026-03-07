use super::UiI18n;

pub struct SpanishUi;

impl UiI18n for SpanishUi {
    fn tab_profile(&self) -> &'static str { "Perfil" }
    fn tab_filter(&self) -> &'static str { "Filtro" }
    fn tab_display(&self) -> &'static str { "Visualización" }

    fn profile_name(&self) -> &'static str { "István Finta" }
    fn profile_subtitle(&self) -> &'static str { "Ingeniero mecánico, robótica e informática" }
    fn profile_title(&self) -> &'static str { "Ingeniero de software" }
    fn profile_location(&self) -> &'static str { "Hungría" }
    fn profile_email(&self) -> &'static str { "istvan_finta@yahoo.com" }
    fn profile_phone(&self) -> &'static str { "+36 70 343 9517" }

    fn section_skills(&self) -> &'static str { "Habilidades técnicas" }
    fn section_countries(&self) -> &'static str { "Países" }
    fn section_languages(&self) -> &'static str { "Idiomas" }
    fn section_companies(&self) -> &'static str { "Empresas" }
    fn section_certificates(&self) -> &'static str { "Certificados" }
    fn filter_select_all(&self) -> &'static str { "Seleccionar todo" }
    fn filter_clear_all(&self) -> &'static str { "Borrar todo" }

    fn section_job_roles(&self) -> &'static str { "Puestos" }
    fn section_projects(&self) -> &'static str { "Proyectos" }
    fn section_tools(&self) -> &'static str { "Herramientas" }
    fn section_project_experience(&self) -> &'static str { "Experiencia en proyectos" }

    fn section_main_chars(&self) -> &'static str { "Mis principales características" }
    fn role_achievements_title(&self) -> &'static str { "Logros clave" }
    fn digital_skills_title(&self) -> &'static str { "Competencias digitales" }
    fn digital_skills_text(&self) -> &'static str {
        "Completas.\n\n\
         SO: Windows, Debian Linux, iOS, Android, OSX, VxWorks, ...\n\
         Herramientas de desarrollo, makefiles, compiladores (C, C#, C++, algo de Rust, ...), \
         intérpretes (Python, ...), editores de texto (nano, kate, MS Code, ...)\n\
         MS Office, Word, Excel, ... (Libre Office y similares también), Navegadores, \
         Aplicaciones web, Proveedores de nube (Vultr, Oracle, ...)\n\
         Aplicaciones gráficas, OpenSCAD (diseño 3D), Eagle (diseño de circuitos), Gimp, \
         herramientas de captura de pantalla, herramientas de edición de fotos, ...\n\
         Conocimientos básicos de IA, conocimientos avanzados de DLT (Distributed Ledger \
         Technology), Stellar Network, contratos inteligentes, ... Desarrollo web básico, \
         HTML5, CSS, JavaScript, algo de React, algo de NodeJS, ...\n\n\
         Domino rápidamente aplicaciones, lenguajes de programación y sistemas operativos."
    }
    fn role_achievements(&self) -> &'static [(&'static str, &'static str)] {
        &[
            ("Framework de desarrollo de pruebas Test.NET",
             "Posteriormente vendido por Bosch a Vector."),
            ("Sincronización de la tarjeta de conmutación de lámparas SK24",
             "Sincronización en tiempo real de los procesos internos de la tarjeta de conmutación de lámparas de control de tráfico con las ondas sinusoidales de la red eléctrica."),
            ("Concepto Vortexledger",
             "Un concepto de libro mayor abierto y economía basado en el trueque."),
        ]
    }

    fn display_nothing_selected(&self) -> &'static str { "Seleccione elementos en la pestaña Filtro para ver los detalles aquí." }

    fn item_label(&self, key: &str) -> &'static str {
        match key {
            // Skills
            "c_embedded" => "C (embebido)",
            "c_win_gui" => "C (Windows/GUI)",
            "cpp_win_gui" => "C++ (Windows/GUI)",
            "cpp_linux_server" => "C++ (Linux/Servidor)",
            "csharp" => "C#",
            "sql" => "SQL",
            "rust" => "Rust / WebAssembly",
            "multimedia" => "Programación multimedia",
            "system_design" => "Diseño de sistemas",
            "project_management" => "Gestión de proyectos",
            "test_management" => "Gestión de pruebas",
            "automated_testing" => "Pruebas automatizadas",
            "manual_testing" => "Pruebas manuales",
            "erp" => "Sistemas ERP",
            "administration" => "Administración de sistemas",
            "leading" => "Liderazgo",
            "auto_test_dev" => "Desarrollo de pruebas automatizadas",
            // Countries
            "hungary" => "Hungría",
            "germany" => "Alemania",
            "austria" => "Austria",
            // Languages
            "lang_hungarian" => "Húngaro",
            "lang_german" => "Alemán",
            "lang_english" => "Inglés",
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
            "cert_pm" => "Certificado PM (PMCC)",
            "cert_sql" => "Fundamentos de bases de datos y SQL",
            "cert_js" => "Certificado JavaScript",
            "cert_ai" => "Programación con agentes de IA",
            "cert_driving" => "Técnica de conducción avanzada",
            "cert_scrum" => "Scrum",
            "cert_istqb" => "Seminario ISTQB",
            "cert_presentation" => "Técnicas de presentación",
            "cert_access" => "MS Access 2003 (intermedio)",
            "cert_canoe" => "Fundamentos de CANoe",
            "cert_dotnet" => "Curso .NET, C#",
            "cert_moderation" => "Moderación — fundamentos",
            "cert_daf" => "Alemán como lengua extranjera",
            "cert_germanistik" => "Semestre internacional de verano en estudios germánicos",
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
            "jr_admin" => "Administrador",
            "jr_sw_dev" => "Desarrollador de software",
            "jr_dept_head" => "Jefe de departamento",
            "jr_tester" => "Probador",
            "jr_test_mgr" => "Responsable de pruebas",
            "jr_test_dev" => "Desarrollador de pruebas",
            "jr_project_mgr" => "Director de proyecto",
            // Projects (Sankey)
            "pj_labor" => "Laboratorio",
            "pj_erp_system" => "Sistema ERP",
            "pj_db_finance" => "Deutsche Bank Financiación de edificios",
            "pj_cwl_kwg" => "C&L Deutsche Revision KWG Novelle",
            "pj_authors_dream" => "Author's Dream",
            "pj_citibank" => "Citibank Antilavado de dinero",
            "pj_junction" => "Herramienta de control de cruce",
            "pj_btc5000" => "BTC5000",
            "pj_sk24" => "SK24",
            "pj_ocit" => "OCIT",
            "pj_debrecen" => "Debrecen",
            "pj_medical" => "Imagen médica",
            "pj_car_body" => "Computadora de carrocería de coche",
            "pj_dcdc" => "Convertidor DC/DC",
            "pj_erp_bosch" => "ERP / Planificación de recursos",
            "pj_test_designer" => "TestDesigner",
            "pj_test_net" => "Test.NET",
            "pj_truck_body" => "Computadora de carrocería de camión",
            "pj_test_tools" => "Mejora de herramientas de prueba",
            "pj_window_lifter" => "Elevalunas",
            "pj_contract" => "Contrato (Auftrag)",
            "pj_asanet" => "ASA-Net",
            "pj_lynx" => "Lynx",
            "pj_sms_email" => "SMS-eMail",
            "pj_customer_cards" => "Tarjetas de clientes",
            "pj_e_billing" => "Facturación electrónica",
            "pj_mein_auto" => "Mein-Auto App",
            "pj_mobil_car" => "Recepción móvil de vehículos",
            "pj_supplier_inv" => "Control de facturas de proveedores",
            "pj_prufloader" => "Prüfloader.NET",
            "pj_vortexledger" => "Vortexledger",
            "pj_georoute" => "GIS GeoRoute",
            // Expertise (Sankey)
            "ex_admin" => "Administración",
            "ex_c_win" => "C win/GUI",
            "ex_c_embedded" => "C embebido",
            "ex_cpp_win" => "C++ win/GUI",
            "ex_cpp_linux" => "C++ linux/servidor",
            "ex_csharp" => "C#",
            "ex_sql" => "SQL",
            "ex_multimedia" => "Programación multimedia",
            "ex_system_design" => "Diseño de sistemas",
            "ex_project_mgmt" => "Gestión de proyectos",
            "ex_leading" => "Liderazgo",
            "ex_manual_test" => "Pruebas manuales",
            "ex_auto_testing" => "Pruebas automatizadas",
            "ex_auto_test_dev" => "Desarrollo de pruebas automatizadas",
            "ex_test_mgmt" => "Gestión de pruebas",
            // Main Characteristics
            "mc_strengths" => "Fortalezas",
            "mc_achievements" => "Logros clave",
            "mc_countries" => "Países",
            "mc_languages" => "Idiomas",
            "mc_certificates" => "Certificados",
            "mc_digital_skills" => "Competencias digitales",
            // Tools
            "tl_visual_studio" => "Visual Studio",
            "tl_ms_office" => "MS Office (Excel, Word, Access)",
            "tl_clearcase" => "IBM ClearCase",
            "tl_clearquest" => "IBM ClearQuest",
            "tl_doors" => "IBM Doors",
            "tl_isystem" => "iSYSTEM Debugger",
            "tl_dspace" => "dSPACE Torre de pruebas",
            "tl_ni_teststand" => "NI TestStand",
            "tl_ni_cvi" => "NI LabWindows/CVI",
            "tl_agilent" => "Instrumentos Agilent",
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
            "c_embedded" => "Firmware de microcontrolador para sistemas de tráfico y ECUs automotrices",
            "c_win_gui" => "Aplicaciones de escritorio Windows en C",
            "cpp_win_gui" => "Aplicaciones GUI Windows en C++",
            "cpp_linux_server" => "Desarrollo de servidor Linux en C++",
            "csharp" => "C# (.NET) — herramientas de prueba, ERP, Prüfloader",
            "sql" => "Consultas de bases de datos, procedimientos almacenados",
            "rust" => "Rust + Dioxus + WebAssembly para aplicaciones web",
            "multimedia" => "Multimedia y presentaciones interactivas",
            "system_design" => "Arquitectura y diseño de sistemas para control de tráfico",
            "project_management" => "Dirección de equipos y gestión de proyectos",
            "test_management" => "Estrategia de pruebas, planificación e informes",
            "automated_testing" => "Desarrollo de pruebas HW/SW automatizadas",
            "manual_testing" => "Pruebas funcionales y de regresión",
            "erp" => "Sistemas de planificación de recursos empresariales",
            "administration" => "Administración de sistemas y redes",
            "leading" => "Vilati — dirección del departamento de desarrollo de software",
            "auto_test_dev" => "Bosch — desarrollo de herramientas de prueba automatizadas",
            "hungary" => "Vilati, Mediso, MOL, Bäko-Hungaria, Bitnök, Telekom",
            "germany" => "Teamcom, Bosch",
            "austria" => "Porsche Informatik, Sigmatek",
            "lang_hungarian" => "Lengua materna",
            "lang_german" => "Era fluido, sin uso durante mucho tiempo",
            "lang_english" => "Conversación básica, literatura técnica, correo electrónico, chat",
            "mol" => "Administración del sistema de laboratorio",
            "bako" => "Administración del sistema ERP",
            "teamcom" => "Software multimedia financiero — Deutsche Bank, Citibank, C&L",
            "vilati" => "Control de cruces de tráfico, C embebido, dirección de departamento",
            "mediso" => "Procesamiento de imágenes médicas",
            "bosch" => "Desarrollo y prueba de ECU automotriz — Body Computers, elevalunas",
            "porsche" => "Sistemas de gestión de concesionarios — C++ Linux/Windows, SQL",
            "sigmatek" => "Prüfloader.NET — herramienta de prueba industrial en C#",
            "bitnok" => "Gestión de proyecto blockchain — VortexLedger",
            "telekom" => "GIS / GeoRoute administración",
            "cert_diploma" => "Título universitario de ingeniería",
            "cert_pm" => "Certificado de gestión de proyectos — PMCC",
            "cert_sql" => "Curso de fundamentos de bases de datos y SQL",
            "cert_js" => "Certificado de desarrollo JavaScript",
            "cert_ai" => "Curso de programación con agentes de IA",
            "cert_driving" => "Técnica de conducción avanzada para la seguridad",
            "cert_scrum" => "Scrum — curso interno PORSCHE",
            "cert_istqb" => "Seminario ISTQB — curso interno Bosch",
            "cert_presentation" => "Técnicas de presentación — curso Develor",
            "cert_access" => "MS Access 2003 (intermedio) — Controll Training",
            "cert_canoe" => "Fundamentos de CANoe — VECTOR",
            "cert_dotnet" => "Curso .NET, C# — BME",
            "cert_moderation" => "Moderación — fundamentos — Bosch interno",
            "cert_daf" => "Alemán como lengua extranjera — Universidad Friedrich Schiller Jena (beca TEMPUS)",
            "cert_germanistik" => "Semestre internacional de verano en estudios germánicos — FSU Jena",
            // Workplace hints
            "wk_mol" => "Administración del sistema de laboratorio",
            "wk_bako" => "Administración del sistema ERP",
            "wk_teamcom" => "Software multimedia financiero — Deutsche Bank, Citibank, C&L",
            "wk_vilati" => "Control de cruces de tráfico, C embebido, dirección de departamento",
            "wk_mediso" => "Procesamiento de imágenes médicas",
            "wk_bosch" => "Desarrollo y prueba de ECU automotriz — Body Computers, elevalunas",
            "wk_porsche" => "Sistemas de gestión de concesionarios — C++ Linux/Windows, SQL",
            "wk_sigmatek" => "Prüfloader.NET — herramienta de prueba industrial en C#",
            "wk_bitnok" => "Gestión de proyecto blockchain — VortexLedger",
            "wk_telekom" => "GIS / GeoRoute administración",
            // Job role hints
            "jr_admin" => "MOL, Bäko-Hungaria, Telekom",
            "jr_sw_dev" => "Teamcom, Vilati, Mediso, Bosch, Porsche, Sigmatek",
            "jr_dept_head" => "Vilati — OCIT, Debrecen",
            "jr_tester" => "Bosch, Porsche Informatik",
            "jr_test_mgr" => "Bosch — Computadora de carrocería de camión, herramientas de prueba",
            "jr_test_dev" => "Bosch — Elevalunas, Computadora de carrocería de camión",
            "jr_project_mgr" => "Bitnök — Vortexledger",
            // Project hints
            "pj_labor" => "MOL AG — sistema de laboratorio",
            "pj_erp_system" => "Bäko-Hungaria — planificación de recursos empresariales",
            "pj_db_finance" => "Teamcom — Deutsche Bank financiación de edificios",
            "pj_cwl_kwg" => "Teamcom — C&L Deutsche Revision",
            "pj_authors_dream" => "Teamcom — editor multimedia",
            "pj_citibank" => "Teamcom — antilavado de dinero",
            "pj_junction" => "Vilati — control de cruce de tráfico",
            "pj_btc5000" => "Vilati — controlador de tráfico",
            "pj_sk24" => "Vilati — tarjeta de conmutación de lámparas, sincronización en tiempo real",
            "pj_ocit" => "Vilati — dirección de departamento, estándar OCIT",
            "pj_debrecen" => "Vilati — dirección de departamento, proyecto Debrecen",
            "pj_medical" => "Mediso — procesamiento de imágenes médicas",
            "pj_car_body" => "Bosch — computadora de carrocería de coche, C embebido",
            "pj_dcdc" => "Bosch — convertidor DC/DC, C embebido",
            "pj_erp_bosch" => "Bosch — planificación de recursos, C#",
            "pj_test_designer" => "Bosch — herramienta de diseño de pruebas, C#",
            "pj_test_net" => "Bosch — framework de desarrollo de pruebas, C#",
            "pj_truck_body" => "Bosch — computadora de carrocería de camión, gestión de pruebas",
            "pj_test_tools" => "Bosch — mejora de herramientas de prueba",
            "pj_window_lifter" => "Bosch — elevalunas, pruebas automatizadas",
            "pj_contract" => "Porsche — gestión de contratos, C++ Linux/Windows",
            "pj_asanet" => "Porsche — ASA-Net, C++ Linux/Windows",
            "pj_lynx" => "Porsche — Lynx, C++ linux/servidor",
            "pj_sms_email" => "Porsche — SMS-eMail, C++ Linux/Windows",
            "pj_customer_cards" => "Porsche — tarjetas de clientes, C++ Linux/Windows",
            "pj_e_billing" => "Porsche — facturación electrónica, C++ Linux/Windows",
            "pj_mein_auto" => "Porsche — Mein-Auto App, C++ linux/servidor",
            "pj_mobil_car" => "Porsche — recepción móvil de vehículos, C++ linux/servidor",
            "pj_supplier_inv" => "Porsche — facturas de proveedores, C++ Linux/Windows",
            "pj_prufloader" => "Sigmatek — herramienta de prueba industrial, C#",
            "pj_vortexledger" => "Bitnök — libro mayor abierto blockchain",
            "pj_georoute" => "Telekom — administración GIS/GeoRoute",
            // Expertise hints
            "ex_admin" => "MOL, Bäko-Hungaria, Telekom",
            "ex_c_win" => "Teamcom, Mediso — aplicaciones de escritorio Windows",
            "ex_c_embedded" => "Vilati, Bosch — firmware de microcontrolador",
            "ex_cpp_win" => "Mediso, Porsche — C++ Windows GUI",
            "ex_cpp_linux" => "Porsche — C++ servidor Linux",
            "ex_csharp" => "Bosch, Sigmatek — aplicaciones .NET",
            "ex_sql" => "Porsche — bases de datos, procedimientos almacenados",
            "ex_multimedia" => "Teamcom — presentaciones financieras interactivas",
            "ex_system_design" => "Vilati — diseño de sistemas de control de tráfico",
            "ex_project_mgmt" => "Vilati, Bitnök — gestión de proyectos",
            "ex_leading" => "Vilati — dirección del departamento de desarrollo de software",
            "ex_manual_test" => "Bosch, Porsche — pruebas funcionales y de regresión",
            "ex_auto_testing" => "Bosch — pruebas HW/SW automatizadas",
            "ex_auto_test_dev" => "Bosch — desarrollo de herramientas de prueba automatizadas",
            "ex_test_mgmt" => "Bosch — estrategia de pruebas, planificación e informes",
            // Main Characteristics hints
            "mc_strengths" => "Experiencia, liderazgo y resolución de problemas",
            "mc_achievements" => "Test.NET, sincronización SK24, Vortexledger",
            "mc_countries" => "Hungría, Alemania, Austria",
            "mc_languages" => "Húngaro, alemán, inglés",
            "mc_certificates" => "Diploma, PM, SQL, JavaScript, IA, conducción",
            "mc_digital_skills" => "Competencias digitales completas",
            // Tool hints
            "tl_visual_studio" => "Bosch, Vilati, Teamcom, Mediso",
            "tl_ms_office" => "Bosch — documentación, planificación",
            "tl_clearcase" => "Bosch — control de versiones",
            "tl_clearquest" => "Bosch — seguimiento de errores",
            "tl_doors" => "Bosch — gestión de requisitos",
            "tl_isystem" => "Bosch — depuración de microcontrolador",
            "tl_dspace" => "Bosch — pruebas HiL",
            "tl_ni_teststand" => "Bosch — pruebas automatizadas",
            "tl_ni_cvi" => "Bosch — pruebas semiautomáticas",
            "tl_agilent" => "Bosch — instrumentos para prueba de elevalunas",
            "tl_opentest" => "Bosch — herramienta de prueba automatizada (ahora Vector)",
            "tl_authors_dream" => "Teamcom — editor multimedia",
            "tl_authorware" => "Teamcom — Citibank multimedia",
            "tl_python" => "Bosch — scripts TestDesigner",
            "tl_snow" => "Telekom — gestión de tickets ITSM",
            "tl_keil" => "Vilati — compilador de microcontrolador C167",
            "tl_cosmic" => "Bosch — compilador de microcontrolador automotriz",
            "tl_gnu_cc" => "Vilati — firmware OCIT bajo Linux",
            "tl_tornado" => "Vilati — desarrollo en tiempo real VxWorks",
            "tl_java_sdk" => "Vilati — Java Native Interface",
            "tl_structured_text" => "Sigmatek — programación de controladores industriales",
            "tl_node_js" => "Bitnök — herramientas de libro mayor abierto",
            _ => "",
        }
    }

    fn btn_generate_pdf(&self) -> &'static str { "Generar PDF" }

    fn role_section_label(&self) -> &'static str { "Rol" }
    fn role_label(&self, key: &str) -> &'static str {
        match key {
            "po" => "Product Owner",
            "szm" => "Ingeniero de software",
            "tm" => "Responsable de pruebas",
            _ => "",
        }
    }
    fn role_title(&self, key: &str) -> &'static str {
        match key {
            "po" => "Product Owner",
            "szm" => "Ingeniero de software",
            "tm" => "Responsable de pruebas",
            _ => "Ingeniero de software",
        }
    }
    fn role_target_title(&self) -> &'static str { "Objetivo" }
    fn role_target_text(&self, key: &str) -> &'static str {
        match key {
            "po" => "Como ingeniero de software experimentado, mi objetivo es aprovechar mi \
                amplia experiencia en gestión de productos y procesos en un rol de Product \
                Owner. Busco mejorar la experiencia del usuario mediante el análisis de \
                comentarios de clientes, prácticas de desarrollo ágil y pensamiento sistémico.",
            "szm" => "Como ingeniero de software experimentado con más de una década de \
                experiencia industrial (especialmente en C, C#, C++, SQL, sistemas embebidos), \
                quiero aplicar mis conocimientos y habilidades innovadoras de resolución de \
                problemas en una organización dinámica y en crecimiento. Mi objetivo es \
                contribuir al éxito de la organización desarrollando soluciones de software \
                de alta calidad mientras amplío continuamente mi experiencia en nuevas \
                tecnologías.",
            "tm" => "Con décadas de experiencia en desarrollo de software, tengo una amplia \
                experiencia en gestión de pruebas en sistemas complejos. Mi experiencia \
                abarca la planificación, gestión y automatización de procesos de prueba, \
                así como el aseguramiento de la calidad. Domino las metodologías de pruebas \
                manuales y automatizadas.",
            _ => "",
        }
    }
    fn role_strengths_title(&self) -> &'static str { "Fortalezas" }
    fn role_strengths(&self, key: &str) -> &'static [(&'static str, &'static str)] {
        match key {
            "po" => &[
                ("Gestión de productos y procesos",
                 "Experiencia en el desarrollo, soporte y optimización de diversos sistemas bancarios y automotrices."),
                ("Pensamiento analítico",
                 "Resolución de tareas complejas de programación a nivel de software y hardware."),
                ("Precisión y atención al detalle",
                 "Trabajo meticuloso y documentación precisa."),
                ("Colaboración y comunicación",
                 "Experiencia en la dirección de equipos de desarrollo y cooperación con departamentos transversales."),
                ("Entorno ágil",
                 "Conocimiento de canales digitales y entornos de desarrollo ágil."),
            ],
            "szm" => &[
                ("Desarrollo y pruebas de software",
                 "Amplia experiencia con C, C++, C# incluyendo sincronización de sistemas en tiempo real y desarrollo de frameworks de pruebas automatizadas."),
                ("Pensamiento sistémico",
                 "Formación en ingeniería mecánica que apoya la programación a nivel de hardware y la comprensión de sistemas mecatrónicos complejos."),
                ("Gestión de proyectos y liderazgo",
                 "Dirección de departamentos de desarrollo de software, coordinación de pruebas de sistema, metodologías Scrum e ISTQB."),
                ("Diversidad tecnológica",
                 "Dominio de Linux, Windows, SQL, desarrollo web y conceptos DLT."),
                ("Resolución de problemas",
                 "Aprendizaje rápido con excelentes habilidades de depuración y solución de problemas."),
            ],
            "tm" => &[
                ("Enfoque orientado a la calidad",
                 "Compromiso con los más altos estándares profesionales (ISTQB, estándares internos de Bosch)."),
                ("Resolución proactiva de problemas",
                 "Análisis sistemático y persistente de causas raíz, búsqueda de soluciones óptimas o compromisos."),
                ("Prototipos y frameworks",
                 "Diseño e implementación de frameworks de prueba internos (p. ej. Test.NET) para la automatización de pruebas."),
                ("Estrategia de pruebas realista",
                 "Alineación de la validación del entorno de pruebas con los plazos de producción de resultados de prueba durante la planificación."),
            ],
            _ => &[],
        }
    }
    fn pwa_hint(&self) -> &'static str {
        "Esta es una aplicación PWA \u{2014} puede guardarla desde su navegador en la pantalla de inicio de su teléfono como si fuera una aplicación nativa"
    }
}
