use dioxus::prelude::*;
use crate::ui::state::AppState;
use crate::ui::i18n::UiI18n;

/// Build a styled HTML document and open the browser's Print dialog
/// so the user can save it as PDF (no external libraries needed).
pub fn generate_pdf(s: &AppState, i18n: &dyn UiI18n) {
    let body = build_pdf_html(s, i18n);
    let escaped = escape_for_js(&body);

    let js = format!(
        r#"(function() {{
            var w = window.open('', '_blank');
            if (!w) {{ alert('Please allow pop-ups to generate PDF.'); return; }}
            w.document.write('<!DOCTYPE html><html><head>' +
                '<meta charset="UTF-8">' +
                '<title>Profile</title>' +
                '<style>' +
                '@media print {{' +
                '  body {{ margin: 0; padding: 12mm 14mm; }}' +
                '  .no-print {{ display: none !important; }}' +
                '}}' +
                'body {{ font-family: sans-serif; margin: 0; padding: 20px 30px; color: #333; }}' +
                '</style></head><body>' +
                "{escaped}" +
                '<div class="no-print" style="text-align:center; margin-top:20px;">' +
                '<button onclick="window.print()" style="padding:10px 30px; font-size:1em; font-weight:600; color:#fff; background:linear-gradient(135deg,#667eea,#764ba2); border:none; border-radius:8px; cursor:pointer;">\u{{1F5A8}} Print / Save PDF</button>' +
                '</div></body></html>');
            w.document.close();
        }})()"#
    );

    document::eval(&js);
}

fn escape_for_js(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}

fn build_pdf_html(s: &AppState, i18n: &dyn UiI18n) -> String {
    let mut html = String::with_capacity(4096);

    // ── Profile section ──
    html.push_str("<div style=\"padding: 10px 20px;\">");

    // Name + title header
    html.push_str(&format!(
        "<h1 style=\"margin: 0 0 2px; color: #333; font-size: 22px;\">{}</h1>",
        esc(i18n.profile_name())
    ));
    html.push_str(&format!(
        "<p style=\"margin: 0 0 2px; color: #667eea; font-size: 14px; font-weight: 600;\">{}</p>",
        esc(i18n.profile_title())
    ));
    html.push_str(&format!(
        "<p style=\"margin: 0 0 10px; color: #888; font-size: 12px;\">{}</p>",
        esc(i18n.profile_location())
    ));

    // Contact
    html.push_str(&format!(
        "<p style=\"margin: 0 0 4px; font-size: 12px; color: #555;\">✉ {} &nbsp; ☎ {}</p>",
        esc(i18n.profile_email()),
        esc(i18n.profile_phone())
    ));

    // About
    html.push_str(&format!(
        "<h3 style=\"margin: 14px 0 4px; color: #333; font-size: 14px;\">{}</h3>",
        esc(i18n.profile_about())
    ));
    html.push_str(&format!(
        "<p style=\"margin: 0 0 14px; color: #555; font-size: 11px; line-height: 1.5;\">{}</p>",
        esc(i18n.profile_about_text())
    ));

    // ── Filter choices (small text) ──
    html.push_str("<hr style=\"border: none; border-top: 1px solid #ddd; margin: 10px 0;\">");
    html.push_str(&format!(
        "<p style=\"margin: 0 0 6px; font-size: 10px; color: #999; font-weight: 600;\">{}</p>",
        esc(i18n.pdf_filter_choices())
    ));

    let filter_sections: Vec<(&str, Vec<&str>)> = vec![
        (i18n.section_skills(), s.selected_skills.read().clone()),
        (i18n.section_countries(), s.selected_countries.read().clone()),
        (i18n.section_languages(), s.selected_languages.read().clone()),
        (i18n.section_companies(), s.selected_companies.read().clone()),
        (i18n.section_certificates(), s.selected_certificates.read().clone()),
    ];

    for (section_name, keys) in &filter_sections {
        if !keys.is_empty() {
            let labels: Vec<String> = keys.iter().map(|k| esc(i18n.item_label(k))).collect();
            html.push_str(&format!(
                "<p style=\"margin: 0 0 2px; font-size: 9px; color: #aaa;\"><b>{}:</b> {}</p>",
                esc(section_name),
                labels.join(", ")
            ));
        }
    }

    // ── Display content (detailed sections) ──
    html.push_str("<hr style=\"border: none; border-top: 1px solid #ddd; margin: 10px 0;\">");

    let sk = s.selected_skills.read();
    let co = s.selected_countries.read();
    let la = s.selected_languages.read();
    let cm = s.selected_companies.read();
    let ce = s.selected_certificates.read();

    let display_sections: Vec<(&str, &[&str], &str)> = vec![
        (i18n.section_skills(), &*sk, "#667eea"),
        (i18n.section_countries(), &*co, "#28a745"),
        (i18n.section_languages(), &*la, "#fd7e14"),
        (i18n.section_companies(), &*cm, "#17a2b8"),
        (i18n.section_certificates(), &*ce, "#6f42c1"),
    ];

    for (title, keys, color) in &display_sections {
        if keys.is_empty() {
            continue;
        }
        html.push_str(&format!(
            "<div style=\"margin-bottom: 12px; border-left: 3px solid {}; padding: 8px 12px; background: #fafbfc; border-radius: 0 6px 6px 0;\">",
            color
        ));
        html.push_str(&format!(
            "<h4 style=\"margin: 0 0 6px; color: {}; font-size: 13px;\">{}</h4>",
            color,
            esc(title)
        ));

        for key in *keys {
            let label = esc(i18n.item_label(key));
            let hint = i18n.item_hint(key);
            html.push_str(&format!(
                "<div style=\"padding: 3px 0; border-bottom: 1px solid #eee;\"><span style=\"font-size: 11px; color: #333; font-weight: 600;\">{}</span>",
                label
            ));
            if !hint.is_empty() {
                html.push_str(&format!(
                    " <span style=\"font-size: 9px; color: #888;\">— {}</span>",
                    esc(hint)
                ));
            }
            html.push_str("</div>");
        }

        html.push_str("</div>");
    }

    html.push_str("</div>");
    html
}

/// Minimal HTML escaping for text content.
fn esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}
