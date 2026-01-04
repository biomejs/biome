use biome_json_factory::make;
use biome_json_syntax::{AnyJsonValue, JsonMember, JsonObjectValue, T};
use biome_rowan::AstSeparatedList;
use std::cmp::Ordering;

pub fn sort_alphabetically(object: &JsonObjectValue) -> Option<JsonObjectValue> {
    sort_object_by_comparator(object, |a, b| a.cmp(b))
}

pub fn sort_alphabetically_deep(object: &JsonObjectValue) -> Option<JsonObjectValue> {
    let sorted = sort_object_by_comparator(object, |a, b| a.cmp(b))?;
    deep_sort_all_nested_objects(&sorted)
}

fn deep_sort_all_nested_objects(object: &JsonObjectValue) -> Option<JsonObjectValue> {
    let members = object.json_member_list();
    let mut elements = Vec::new();
    let mut has_changes = false;

    for m in (&members).into_iter().flatten() {
        let transformed_member = if let Ok(value) = m.value()
            && let Some(obj) = value.as_json_object_value()
            && let Some(deep_sorted) = sort_alphabetically_deep(obj)
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

/// Generic object sorter that accepts a custom comparator function
pub fn sort_object_by_comparator<F>(
    object: &JsonObjectValue,
    comparator: F,
) -> Option<JsonObjectValue>
where
    F: Fn(&String, &String) -> Ordering,
{
    let members = object.json_member_list();
    let mut member_vec: Vec<JsonMember> = members.iter().filter_map(|m| m.ok()).collect();

    if member_vec.len() < 2 {
        return None;
    }

    member_vec.sort_by(|a, b| {
        let a_name = a
            .name()
            .ok()
            .and_then(|n| n.inner_string_text().ok())
            .map(|t| t.text().to_string());
        let b_name = b
            .name()
            .ok()
            .and_then(|n| n.inner_string_text().ok())
            .map(|t| t.text().to_string());

        match (a_name, b_name) {
            (Some(ref a), Some(ref b)) => comparator(a, b),
            _ => Ordering::Equal,
        }
    });

    rebuild_object_from_members(object, member_vec)
}

pub fn sort_object_by_key_order(
    object: &JsonObjectValue,
    key_order: &[&str],
) -> Option<JsonObjectValue> {
    let members = object.json_member_list();
    let mut member_vec: Vec<JsonMember> = members.iter().filter_map(|m| m.ok()).collect();

    if member_vec.len() < 2 {
        return None;
    }

    member_vec.sort_by(|a, b| {
        let a_name = a
            .name()
            .ok()
            .and_then(|n| n.inner_string_text().ok())
            .map(|t| t.text().to_string());
        let b_name = b
            .name()
            .ok()
            .and_then(|n| n.inner_string_text().ok())
            .map(|t| t.text().to_string());

        match (a_name, b_name) {
            (Some(a), Some(b)) => {
                let a_idx = key_order.iter().position(|&k| k == a);
                let b_idx = key_order.iter().position(|&k| k == b);

                match (a_idx, b_idx) {
                    (Some(a_i), Some(b_i)) => a_i.cmp(&b_i),
                    (Some(_), None) => Ordering::Less,
                    (None, Some(_)) => Ordering::Greater,
                    (None, None) => a.cmp(&b),
                }
            }
            _ => Ordering::Equal,
        }
    });

    rebuild_object_from_members(object, member_vec)
}

/// https://docs.npmjs.com/cli/v10/configuring-npm/package-json#people-fields-author-contributors
pub fn sort_people_object(object: &JsonObjectValue) -> Option<JsonObjectValue> {
    sort_object_by_key_order(object, &["name", "email", "url"])
}

pub fn sort_url_object(object: &JsonObjectValue) -> Option<JsonObjectValue> {
    sort_object_by_key_order(object, &["type", "url"])
}

pub fn sort_bugs_object(object: &JsonObjectValue) -> Option<JsonObjectValue> {
    sort_object_by_key_order(object, &["url", "email"])
}

pub fn sort_directories(object: &JsonObjectValue) -> Option<JsonObjectValue> {
    sort_object_by_key_order(object, &["lib", "bin", "man", "doc", "example", "test"])
}

pub fn sort_volta(object: &JsonObjectValue) -> Option<JsonObjectValue> {
    sort_object_by_key_order(object, &["node", "npm", "yarn"])
}

pub fn sort_binary(object: &JsonObjectValue) -> Option<JsonObjectValue> {
    sort_object_by_key_order(
        object,
        &[
            "module_name",
            "module_path",
            "remote_path",
            "package_name",
            "host",
        ],
    )
}

pub fn sort_vscode_badge_object(object: &JsonObjectValue) -> Option<JsonObjectValue> {
    sort_object_by_key_order(object, &["description", "url", "href"])
}

/// Git hooks in execution order, based on git-hooks-list package
pub(super) const GIT_HOOKS_ORDER: &[&str] = &[
    "applypatch-msg",
    "pre-applypatch",
    "post-applypatch",
    "pre-commit",
    "pre-merge-commit",
    "prepare-commit-msg",
    "commit-msg",
    "post-commit",
    "pre-rebase",
    "post-checkout",
    "post-merge",
    "pre-push",
    "pre-receive",
    "update",
    "proc-receive",
    "post-receive",
    "post-update",
    "reference-transaction",
    "push-to-checkout",
    "pre-auto-gc",
    "post-rewrite",
    "sendemail-validate",
    "fsmonitor-watchman",
    "p4-changelist",
    "p4-prepare-changelist",
    "p4-post-changelist",
    "p4-pre-submit",
    "post-index-change",
];

/// Based on git-hooks-list package
pub fn sort_git_hooks(object: &JsonObjectValue) -> Option<JsonObjectValue> {
    sort_object_by_key_order(object, GIT_HOOKS_ORDER)
}

fn rebuild_object_from_members(
    original: &JsonObjectValue,
    members: Vec<JsonMember>,
) -> Option<JsonObjectValue> {
    if members.is_empty() {
        return Some(original.clone());
    }

    let mut elements = Vec::new();
    let mut separators = Vec::new();

    for (i, member) in members.iter().enumerate() {
        elements.push(member.clone());

        if i < members.len() - 1 {
            separators.push(make::token(T![,]));
        }
    }

    let new_members = make::json_member_list(elements, separators);

    Some(original.clone().with_json_member_list(new_members))
}

pub fn transform_people_array(array: &AnyJsonValue) -> Option<AnyJsonValue> {
    transform_array_with(array, |obj| sort_people_object(obj).map(AnyJsonValue::from))
}

pub fn transform_badges_array(array: &AnyJsonValue) -> Option<AnyJsonValue> {
    transform_array_with(array, |obj| {
        sort_vscode_badge_object(obj).map(AnyJsonValue::from)
    })
}

fn transform_array_with<F>(array: &AnyJsonValue, transform_fn: F) -> Option<AnyJsonValue>
where
    F: Fn(&JsonObjectValue) -> Option<AnyJsonValue>,
{
    let array_value = array.as_json_array_value()?;
    let elements = array_value.elements();

    let mut transformed_elements = Vec::new();
    let mut has_changes = false;

    for element in (&elements).into_iter().flatten() {
        if let Some(obj) = element.as_json_object_value()
            && let Some(transformed) = transform_fn(obj)
        {
            transformed_elements.push(transformed);
            has_changes = true;
        } else {
            transformed_elements.push(element.clone());
        }
    }

    if !has_changes {
        return None;
    }

    let mut array_elements = Vec::new();
    let mut separators = Vec::new();

    for (i, elem) in transformed_elements.iter().enumerate() {
        array_elements.push(elem.clone());
        if i < transformed_elements.len() - 1 {
            separators.push(make::token(T![,]));
        }
    }

    let new_elements = make::json_array_element_list(array_elements, separators);
    let new_array = array_value.clone().with_elements(new_elements);

    Some(AnyJsonValue::from(new_array))
}

pub fn transform_nested_property<F>(
    object: &JsonObjectValue,
    property_name: &str,
    transform_fn: F,
) -> Option<JsonObjectValue>
where
    F: Fn(&AnyJsonValue) -> Option<AnyJsonValue>,
{
    let members = object.json_member_list();
    let mut elements = Vec::new();
    let mut has_changes = false;

    for m in (&members).into_iter().flatten() {
        if let Ok(name) = m.name().and_then(|n| n.inner_string_text())
            && name.text() == property_name
            && let Ok(value) = m.value()
            && let Some(transformed_value) = transform_fn(&value)
        {
            elements.push(m.clone().with_value(transformed_value));
            has_changes = true;
            continue;
        }
        elements.push(m.clone());
    }

    if !has_changes {
        return None;
    }

    let mut separators = Vec::new();
    for _ in 0..(elements.len().saturating_sub(1)) {
        separators.push(make::token(T![,]));
    }

    let new_members = make::json_member_list(elements, separators);
    Some(object.clone().with_json_member_list(new_members))
}

/// Remove duplicate string values from an array
pub fn uniq_array(array: &AnyJsonValue) -> Option<AnyJsonValue> {
    let array_value = array.as_json_array_value()?;
    let elements = array_value.elements();

    let mut seen = std::collections::HashSet::new();
    let mut unique_elements = Vec::new();
    let mut has_changes = false;

    for element in (&elements).into_iter().flatten() {
        if let Some(string_val) = element.as_json_string_value()
            && let Ok(text) = string_val.inner_string_text()
        {
            let content = text.text().to_string();
            if seen.insert(content) {
                unique_elements.push(element.clone());
            } else {
                has_changes = true;
            }
        } else {
            unique_elements.push(element.clone());
        }
    }

    if !has_changes {
        return None;
    }

    rebuild_array(array_value, unique_elements)
}

/// Remove duplicates and sort array alphabetically
pub fn uniq_and_sort_array(array: &AnyJsonValue) -> Option<AnyJsonValue> {
    let array_value = array.as_json_array_value()?;
    let elements = array_value.elements();

    let mut string_values = Vec::new();
    let mut non_string_elements = Vec::new();

    for element in (&elements).into_iter().flatten() {
        if let Some(string_val) = element.as_json_string_value()
            && let Ok(text) = string_val.inner_string_text()
        {
            string_values.push((text.text().to_string(), element.clone()));
        } else {
            non_string_elements.push(element.clone());
        }
    }

    // Remove duplicates and sort
    let mut seen = std::collections::HashSet::new();
    let mut unique_sorted: Vec<_> = string_values
        .into_iter()
        .filter(|(s, _)| seen.insert(s.clone()))
        .collect();
    unique_sorted.sort_by(|a, b| a.0.cmp(&b.0));

    let original_count = elements.iter().count();
    let new_count = unique_sorted.len() + non_string_elements.len();

    // Check if order changed
    let current_strings: Vec<String> = elements
        .iter()
        .filter_map(|e| {
            e.ok()?
                .as_json_string_value()?
                .inner_string_text()
                .ok()
                .map(|t| t.text().to_string())
        })
        .collect();
    let sorted_strings: Vec<String> = unique_sorted.iter().map(|(s, _)| s.clone()).collect();

    if original_count == new_count && current_strings == sorted_strings {
        return None;
    }

    let mut final_elements = Vec::new();
    for (_, elem) in unique_sorted {
        final_elements.push(elem);
    }
    final_elements.extend(non_string_elements);

    rebuild_array(array_value, final_elements)
}

fn rebuild_array(
    array_value: &biome_json_syntax::JsonArrayValue,
    elements: Vec<AnyJsonValue>,
) -> Option<AnyJsonValue> {
    let mut separators = Vec::new();
    for _ in 0..(elements.len().saturating_sub(1)) {
        separators.push(make::token(T![,]));
    }

    let new_elements = make::json_array_element_list(elements, separators);
    let new_array = array_value.clone().with_elements(new_elements);

    Some(AnyJsonValue::from(new_array))
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_json_parser::{JsonParserOptions, parse_json};

    fn parse_json_object(source: &str) -> JsonObjectValue {
        let parsed = parse_json(source, JsonParserOptions::default());
        parsed
            .tree()
            .value()
            .ok()
            .and_then(|v| v.as_json_object_value().cloned())
            .expect("Failed to parse JSON object")
    }

    #[test]
    fn test_sort_alphabetically() {
        let obj = parse_json_object(r#"{"z": 1, "a": 2, "m": 3}"#);
        let sorted = sort_alphabetically(&obj).unwrap();

        let keys: Vec<String> = sorted
            .json_member_list()
            .iter()
            .filter_map(|m| {
                m.ok()?
                    .name()
                    .ok()?
                    .inner_string_text()
                    .ok()
                    .map(|t| t.text().to_string())
            })
            .collect();

        assert_eq!(keys, vec!["a", "m", "z"]);
    }

    #[test]
    fn test_sort_people_object() {
        let obj = parse_json_object(
            r#"{"url": "https://example.com", "name": "John", "email": "john@example.com"}"#,
        );
        let sorted = sort_people_object(&obj).unwrap();

        let keys: Vec<String> = sorted
            .json_member_list()
            .iter()
            .filter_map(|m| {
                m.ok()?
                    .name()
                    .ok()?
                    .inner_string_text()
                    .ok()
                    .map(|t| t.text().to_string())
            })
            .collect();

        assert_eq!(keys, vec!["name", "email", "url"]);
    }

    #[test]
    fn test_sort_url_object() {
        let obj = parse_json_object(r#"{"url": "https://github.com", "type": "git"}"#);
        let sorted = sort_url_object(&obj).unwrap();

        let keys: Vec<String> = sorted
            .json_member_list()
            .iter()
            .filter_map(|m| {
                m.ok()?
                    .name()
                    .ok()?
                    .inner_string_text()
                    .ok()
                    .map(|t| t.text().to_string())
            })
            .collect();

        assert_eq!(keys, vec!["type", "url"]);
    }

    #[test]
    fn test_sort_volta() {
        let obj = parse_json_object(r#"{"yarn": "1.22.0", "node": "18.0.0", "npm": "9.0.0"}"#);
        let sorted = sort_volta(&obj).unwrap();

        let keys: Vec<String> = sorted
            .json_member_list()
            .iter()
            .filter_map(|m| {
                m.ok()?
                    .name()
                    .ok()?
                    .inner_string_text()
                    .ok()
                    .map(|t| t.text().to_string())
            })
            .collect();

        assert_eq!(keys, vec!["node", "npm", "yarn"]);
    }
}
