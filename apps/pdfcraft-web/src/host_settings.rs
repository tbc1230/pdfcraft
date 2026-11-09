//! The hosting page's interface language, theme and author name: the checks shared by the setters
//! (`window.pdfcraft.setLanguage`, `setTheme`, `setAuthor`) and by the `data-language`, `data-theme` and
//! `data-author` attributes of the canvas, which are read once when the editor starts. A value that
//! fails its check is `None`: the setter returns false, and the editor stays as it was.

use pdfcraft_ui_egui::theme::ThemePreference;

/// The author name: trimmed, and refused when blank. (`PdfCraftApp::set_author` cuts it to its limit.)
pub fn author(name: &str) -> Option<String> {
    Some(name.trim().to_string()).filter(|n| !n.is_empty())
}

/// The interface language: `auto` or a language PdfCraft has a translation for, any case.
pub fn language(code: &str) -> Option<String> {
    pdfcraft_ui_egui::i18n::normalize_pref(code.trim()).map(str::to_string)
}

/// The theme: exactly `light`, `dark` or `system`.
pub fn theme(theme: &str) -> Option<ThemePreference> {
    match theme {
        "light" => Some(ThemePreference::Light),
        "dark" => Some(ThemePreference::Dark),
        "system" => Some(ThemePreference::System),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn author_is_trimmed_and_a_blank_one_is_refused() {
        assert_eq!(author("  Ada Lovelace "), Some("Ada Lovelace".to_string()));
        assert_eq!(author("   "), None);
        assert_eq!(author(""), None);
    }

    #[test]
    fn language_accepts_auto_in_any_case_and_refuses_an_unknown_code() {
        assert_eq!(language(" AUTO "), Some("auto".to_string()));
        assert_eq!(language("zz-nope"), None);
        assert_eq!(language(""), None);
    }

    #[test]
    fn theme_accepts_the_three_values_only() {
        assert_eq!(theme("light"), Some(ThemePreference::Light));
        assert_eq!(theme("dark"), Some(ThemePreference::Dark));
        assert_eq!(theme("system"), Some(ThemePreference::System));
        assert_eq!(theme("Dark"), None);
        assert_eq!(theme(""), None);
    }
}
