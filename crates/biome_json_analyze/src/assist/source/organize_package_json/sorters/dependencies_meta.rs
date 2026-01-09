use biome_json_factory::make;
use biome_json_syntax::{AnyJsonValue, JsonObjectValue, T};

use super::helpers::sort_object_by_comparator;

/// Extracts package name from dependency identifiers.
///
/// For identifiers like `"@scope/package@1.0.0"` or `"package@1.0.0"`,
/// returns just the package name part (`"@scope/package"` or `"package"`).
///
/// Based on sort-package-json index.js:105-109
/// https://github.com/keithamus/sort-package-json/blob/main/index.js#L105-L109
fn get_package_name(ident: &str) -> &str {
    let start_idx = if ident.starts_with('@') { 1 } else { 0 };

    if let Some(at_pos) = ident[start_idx..].find('@') {
        &ident[..start_idx + at_pos]
    } else {
        ident
    }
}

/// Sorts dependenciesMeta keys by package name.
///
/// This is used for the `dependenciesMeta` field which can contain keys like:
/// - `"package@1.0.0"`
/// - `"@scope/package@^2.0.0"`
///
/// Keys are sorted by extracting the package name portion (before the `@version`),
/// so `"package@1.0.0"` and `"package@2.0.0"` are both treated as `"package"`.
///
/// After sorting top-level keys, nested objects are recursively sorted alphabetically.
///
/// Based on sort-package-json index.js:111-118, 541
/// https://github.com/keithamus/sort-package-json/blob/main/index.js#L111-L118
/// https://github.com/keithamus/sort-package-json/blob/main/index.js#L541
pub fn transform(value: &AnyJsonValue) -> Option<AnyJsonValue> {
    let object = value.as_json_object_value()?;

    let sorted_top_level = sort_object_by_comparator(object, |a, b| {
        let pkg_a = get_package_name(a);
        let pkg_b = get_package_name(b);
        pkg_a.cmp(pkg_b)
    })?;

    // Note: sort_object_by_comparator already returns None if no change needed at top level
    // deep_sort_nested_objects also returns None if no nested changes needed
    deep_sort_nested_objects(&sorted_top_level).map(AnyJsonValue::from)
}

/// Recursively sorts all nested objects alphabetically.
fn deep_sort_nested_objects(object: &JsonObjectValue) -> Option<JsonObjectValue> {
    let members = object.json_member_list();
    let mut elements = Vec::new();
    let mut has_changes = false;

    for m in (&members).into_iter().flatten() {
        let transformed_member = if let Ok(value) = m.value()
            && let Some(obj) = value.as_json_object_value()
            && let Some(deep_sorted) = super::helpers::sort_alphabetically_deep(obj)
        {
            has_changes = true;
            m.clone().with_value(AnyJsonValue::from(deep_sorted))
        } else {
            m.clone()
        };
        elements.push(transformed_member);
    }

    if !has_changes {
        return Some(object.clone());
    }

    let mut separators = Vec::new();
    for _ in 0..(elements.len().saturating_sub(1)) {
        separators.push(make::token(T![,]));
    }

    let new_members = make::json_member_list(elements, separators);
    Some(object.clone().with_json_member_list(new_members))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_package_name() {
        assert_eq!(get_package_name("package"), "package");
        assert_eq!(get_package_name("package@1.0.0"), "package");
        assert_eq!(get_package_name("package@^2.0.0"), "package");
        assert_eq!(get_package_name("@scope/package"), "@scope/package");
        assert_eq!(get_package_name("@scope/package@1.0.0"), "@scope/package");
        assert_eq!(get_package_name("@scope/package@^2.0.0"), "@scope/package");
        assert_eq!(get_package_name("@scope/package@>=1.0.0"), "@scope/package");
    }

    #[test]
    fn test_get_package_name_edge_cases() {
        assert_eq!(get_package_name("@"), "@");
        assert_eq!(get_package_name("@@"), "@");
        assert_eq!(get_package_name("no-version"), "no-version");
        assert_eq!(get_package_name("@scoped"), "@scoped");
    }
}
