use biome_json_syntax::{AnyJsonValue, JsonMemberList, JsonObjectValue};
use biome_string_case::StrOnlyExtension;

use super::sorters::{sort_alphabetically, sort_object_by_comparator};

/// Sort dependencies alphabetically, detecting package manager to use the appropriate comparison.
/// npm uses locale-aware comparison, yarn and pnpm use simple string comparison.
///
/// npm sorting uses English locale-aware comparison (similar to JavaScript's localeCompare).
/// This is approximated using case-insensitive comparison.
pub fn transform(
    value: &AnyJsonValue,
    package_json_root: &JsonObjectValue,
) -> Option<AnyJsonValue> {
    let object = value.as_json_object_value()?;

    // Sort like npm CLI does (via @npmcli/package-json)
    // https://github.com/npm/package-json/blob/b6465f44c727d6513db6898c7cbe41dd355cebe8/lib/update-dependencies.js#L8-L21
    // npm uses a.localeCompare(b, 'en') which is locale-aware (case-insensitive)
    let sorted = if should_sort_dependencies_like_npm(package_json_root) {
        sort_object_by_comparator(object, |a, b| {
            a.to_lowercase_cow().cmp(&b.to_lowercase_cow())
        })?
    } else {
        // yarn/pnpm use simple string comparison
        sort_alphabetically(object)?
    };

    Some(AnyJsonValue::from(sorted))
}

/// Detects the package manager from package.json fields.
/// Based on https://github.com/nodejs/corepack for packageManager field.
fn should_sort_dependencies_like_npm(package_json: &JsonObjectValue) -> bool {
    let members = package_json.json_member_list();

    if let Some(pm_value) = find_member_value(&members, "packageManager")
        && let Some(s) = pm_value.as_json_string_value()
        && let Ok(text) = s.inner_string_text()
    {
        return text.text().starts_with("npm@");
    }

    if let Some(dev_engines) = find_member_value(&members, "devEngines")
        && let Some(de_obj) = dev_engines.as_json_object_value()
        && let Some(pm_obj) = find_member_value(&de_obj.json_member_list(), "packageManager")
        && let Some(pm_obj_value) = pm_obj.as_json_object_value()
        && let Some(name_value) = find_member_value(&pm_obj_value.json_member_list(), "name")
        && let Some(name_str) = name_value.as_json_string_value()
        && let Ok(text) = name_str.inner_string_text()
    {
        return text.text() == "npm";
    }

    if find_member_value(&members, "pnpm").is_some() {
        return false;
    }

    // Optimization: Check if npm is explicit before checking filesystem (which we skip in Biome)
    if let Some(engines) = find_member_value(&members, "engines")
        && let Some(engines_obj) = engines.as_json_object_value()
        && find_member_value(&engines_obj.json_member_list(), "npm").is_some()
    {
        return true;
    }

    // In sort-package-json, this would check for yarn.lock/pnpm-lock.yaml
    // We skip filesystem checks in Biome and default to npm
    true
}

fn find_member_value(members: &JsonMemberList, key: &str) -> Option<AnyJsonValue> {
    for member in members {
        let member = member.ok()?;
        let name = member.name().ok()?.inner_string_text().ok()?;
        if name.text() == key {
            return member.value().ok();
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_json_parser::{JsonParserOptions, parse_json};

    fn parse_root(source: &str) -> JsonObjectValue {
        let parsed = parse_json(source, JsonParserOptions::default());
        parsed
            .tree()
            .value()
            .ok()
            .and_then(|v| v.as_json_object_value().cloned())
            .unwrap()
    }

    #[test]
    fn test_detect_npm_via_package_manager_field() {
        let root = parse_root(r#"{"packageManager": "npm@10.0.0"}"#);
        assert!(should_sort_dependencies_like_npm(&root));
    }

    #[test]
    fn test_detect_yarn_via_package_manager_field() {
        let root = parse_root(r#"{"packageManager": "yarn@1.22.0"}"#);
        assert!(!should_sort_dependencies_like_npm(&root));
    }

    #[test]
    fn test_detect_pnpm_via_package_manager_field() {
        let root = parse_root(r#"{"packageManager": "pnpm@8.0.0"}"#);
        assert!(!should_sort_dependencies_like_npm(&root));
    }

    #[test]
    fn test_detect_pnpm_via_pnpm_field() {
        let root = parse_root(r#"{"pnpm": {}}"#);
        assert!(!should_sort_dependencies_like_npm(&root));
    }

    #[test]
    fn test_detect_npm_via_engines() {
        let root = parse_root(r#"{"engines": {"npm": ">=8.0.0"}}"#);
        assert!(should_sort_dependencies_like_npm(&root));
    }

    #[test]
    fn test_default_to_npm() {
        let root = parse_root(r#"{}"#);
        assert!(should_sort_dependencies_like_npm(&root));
    }
}
