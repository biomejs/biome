/// Sorted array of core Google Apps Script service globals.
///
/// These are the top-level service objects exposed by the Apps Script runtime.
/// The list intentionally covers the core built-in services only; projects using
/// advanced services or additional globals can extend it through the
/// `javascript.globals` configuration option.
///
/// Source: <https://developers.google.com/apps-script/reference>
pub const GOOGLE_APPS_SCRIPT: &[&str] = &[
    "Browser",
    "CacheService",
    "CalendarApp",
    "Charts",
    "ContactsApp",
    "ContentService",
    "DataStudioApp",
    "DocumentApp",
    "DriveApp",
    "FormApp",
    "GmailApp",
    "GroupsApp",
    "HtmlService",
    "LanguageApp",
    "LinearOptimizationService",
    "LockService",
    "Logger",
    "MailApp",
    "Maps",
    "PropertiesService",
    "ScriptApp",
    "Session",
    "SitesApp",
    "SlidesApp",
    "SpreadsheetApp",
    "UrlFetchApp",
    "Utilities",
    "XmlService",
];

/// Returns `true` if `name` is a built-in Google Apps Script service global.
pub fn is_google_apps_script_global(name: &str) -> bool {
    GOOGLE_APPS_SCRIPT.binary_search(&name).is_ok()
}

#[cfg(test)]
mod tests {
    use super::GOOGLE_APPS_SCRIPT;

    #[test]
    fn list_is_sorted() {
        // `is_google_apps_script_global` relies on a binary search, so the list
        // must stay sorted.
        assert!(
            GOOGLE_APPS_SCRIPT.is_sorted(),
            "GOOGLE_APPS_SCRIPT must be sorted"
        );
    }
}
