use biome_json_factory::make;
use biome_json_syntax::{AnyJsonValue, JsonMember, T};
use biome_rowan::{AstSeparatedList, TokenText};

use super::helpers::sort_alphabetically;

pub fn transform(value: &AnyJsonValue) -> Option<AnyJsonValue> {
    let object = value.as_json_object_value()?;
    let members = object.json_member_list();

    let mut keys_with_members: Vec<(TokenText, JsonMember)> = Vec::new();
    let mut overrides_member: Option<JsonMember> = None;

    for member in &members {
        if let Ok(m) = member
            && let Some(name) = m.name().ok().and_then(|n| n.inner_string_text())
        {
            if name == "overrides" {
                overrides_member = Some(m.clone());
            } else {
                keys_with_members.push((name, m.clone()));
            }
        }
    }

    if keys_with_members.len() < 2 && overrides_member.is_none() {
        return None;
    }

    // Check if top-level keys are already sorted (alphabetical + overrides last)
    let current_keys: Vec<&TokenText> = keys_with_members.iter().map(|(k, _)| k).collect();
    let mut sorted_keys = current_keys.clone();
    sorted_keys.sort();

    let overrides_is_last = if overrides_member.is_some() {
        (&members)
            .into_iter()
            .filter_map(|m| m.ok())
            .last()
            .and_then(|m| m.name().ok()?.inner_string_text())
            .is_some_and(|name| name == "overrides")
    } else {
        true
    };

    // Sort the overrides array entries (each is an object; also sort their `options`)
    let (sorted_overrides_member, overrides_changed) =
        sort_overrides_array(overrides_member.as_ref());

    let keys_changed = current_keys != sorted_keys || !overrides_is_last;

    if !keys_changed && !overrides_changed {
        return None;
    }

    keys_with_members.sort_by(|a, b| a.0.cmp(&b.0));

    let mut elements = Vec::new();
    let mut separators = Vec::new();

    for (_, member) in &keys_with_members {
        elements.push(member.clone());
        separators.push(make::token(T![,]));
    }

    let overrides_to_push = sorted_overrides_member.or(overrides_member);
    if let Some(overrides) = overrides_to_push {
        elements.push(overrides);
    } else if !separators.is_empty() {
        separators.pop();
    }

    let new_members = make::json_member_list(elements, separators);
    let sorted_object = object.clone().with_json_member_list(new_members);

    Some(AnyJsonValue::from(sorted_object))
}

/// Sort each object in the `overrides` array alphabetically, also sorting any
/// nested `options` object. Returns `(Option<sorted_member>, changed)`.
fn sort_overrides_array(member: Option<&JsonMember>) -> (Option<JsonMember>, bool) {
    let Some(member) = member else {
        return (None, false);
    };
    let Ok(value) = member.value() else {
        return (None, false);
    };
    let Some(array) = value.as_json_array_value() else {
        return (None, false);
    };

    let mut new_elements: Vec<AnyJsonValue> = Vec::new();
    let mut any_changed = false;

    for elem in array.elements().iter().filter_map(|e| e.ok()) {
        let Some(obj) = elem.as_json_object_value() else {
            new_elements.push(elem.clone());
            continue;
        };

        // Sort each override entry alphabetically, but also sort its `options` sub-object
        let (sorted_obj, changed) = sort_override_entry(obj);
        any_changed |= changed;
        new_elements.push(AnyJsonValue::from(sorted_obj));
    }

    if !any_changed {
        return (None, false);
    }

    let sep_count = new_elements.len().saturating_sub(1);
    let separators: Vec<_> = (0..sep_count).map(|_| make::token(T![,])).collect();
    let new_array_elems = make::json_array_element_list(new_elements, separators);
    let new_array = array.clone().with_elements(new_array_elems);
    let new_member = member.clone().with_value(AnyJsonValue::from(new_array));

    (Some(new_member), true)
}

/// Sort a single prettier override object: alphabetically, with `options` also sorted.
/// Returns `(result_object, changed)`.
fn sort_override_entry(
    obj: &biome_json_syntax::JsonObjectValue,
) -> (biome_json_syntax::JsonObjectValue, bool) {
    // First sort `options` sub-object if present
    let members = obj.json_member_list();
    let mut any_nested_changed = false;
    let mut patched_elements: Vec<biome_json_syntax::JsonMember> = Vec::new();

    for m in members.iter().filter_map(|m| m.ok()) {
        let is_options = m
            .name()
            .ok()
            .and_then(|n| n.inner_string_text())
            .is_some_and(|t| t == "options");

        if is_options {
            if let Ok(opt_value) = m.value()
                && let Some(opt_obj) = opt_value.as_json_object_value()
                && let Some(sorted_opts) = sort_alphabetically(opt_obj)
            {
                any_nested_changed = true;
                patched_elements.push(m.clone().with_value(AnyJsonValue::from(sorted_opts)));
                continue;
            }
        }
        patched_elements.push(m.clone());
    }

    // Now sort the override object itself alphabetically
    let obj_to_sort = if any_nested_changed {
        let sep_count = patched_elements.len().saturating_sub(1);
        let separators: Vec<_> = (0..sep_count).map(|_| make::token(T![,])).collect();
        let new_members = make::json_member_list(patched_elements, separators);
        obj.clone().with_json_member_list(new_members)
    } else {
        obj.clone()
    };

    match sort_alphabetically(&obj_to_sort) {
        Some(sorted) => (sorted, true),
        None if any_nested_changed => (obj_to_sort, true),
        None => (obj.clone(), false),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_json_parser::{JsonParserOptions, parse_json};
    use biome_json_syntax::JsonObjectValue;
    use biome_rowan::AstSeparatedList;

    fn parse_object(source: &str) -> JsonObjectValue {
        let parsed = parse_json(source, JsonParserOptions::default());
        parsed
            .tree()
            .value()
            .ok()
            .and_then(|v| v.as_json_object_value().cloned())
            .unwrap()
    }

    #[test]
    fn test_prettier_overrides_last() {
        let obj = parse_object(r#"{"overrides": [], "semi": false, "tabWidth": 2}"#);
        let result = transform(&AnyJsonValue::from(obj)).expect("Should move overrides last");
        let result_obj = result.as_json_object_value().unwrap();

        let keys: Vec<TokenText> = result_obj
            .json_member_list()
            .iter()
            .filter_map(|m| m.ok()?.name().ok()?.inner_string_text())
            .collect();

        assert_eq!(keys, vec!["semi", "tabWidth", "overrides"]);
    }

    #[test]
    fn test_prettier_alphabetical_without_overrides() {
        let obj = parse_object(r#"{"tabWidth": 2, "semi": false, "singleQuote": true}"#);
        let result = transform(&AnyJsonValue::from(obj)).expect("Should sort alphabetically");
        let result_obj = result.as_json_object_value().unwrap();

        let keys: Vec<TokenText> = result_obj
            .json_member_list()
            .iter()
            .filter_map(|m| m.ok()?.name().ok()?.inner_string_text())
            .collect();

        assert_eq!(keys, vec!["semi", "singleQuote", "tabWidth"]);
    }

    #[test]
    fn test_prettier_already_sorted_returns_none() {
        let obj = parse_object(r#"{"semi": false, "tabWidth": 2, "overrides": []}"#);
        let result = transform(&AnyJsonValue::from(obj));
        assert!(result.is_none(), "Should return None when already sorted");
    }

    #[test]
    fn test_prettier_overrides_entries_sorted_alphabetically() {
        // Override entry has keys in reverse order: "files" > "options" alphabetically but it's after
        // Actually, test that keys within an override entry are sorted
        let obj = parse_object(
            r#"{"overrides": [{"options": {"tabWidth": 2, "semi": false}, "files": "*.ts"}]}"#,
        );
        let result = transform(&AnyJsonValue::from(obj)).expect("Should sort override entry keys");
        let result_obj = result.as_json_object_value().unwrap();

        // Get the first override entry
        let override_entry = result_obj
            .json_member_list()
            .iter()
            .find_map(|m| {
                let m = m.ok()?;
                if m.name().ok()?.inner_string_text()? == "overrides" {
                    m.value()
                        .ok()?
                        .as_json_array_value()
                        .and_then(|a| a.elements().iter().next()?.ok())
                        .and_then(|e| e.as_json_object_value().cloned())
                } else {
                    None
                }
            })
            .unwrap();

        // Keys of the override entry should be sorted: "files" before "options"
        let entry_keys: Vec<TokenText> = override_entry
            .json_member_list()
            .iter()
            .filter_map(|m| m.ok()?.name().ok()?.inner_string_text())
            .collect();
        assert_eq!(entry_keys, vec!["files", "options"]);

        // Options within the override entry should also be sorted: "semi" before "tabWidth"
        let options_obj = override_entry
            .json_member_list()
            .iter()
            .find_map(|m| {
                let m = m.ok()?;
                if m.name().ok()?.inner_string_text()? == "options" {
                    m.value().ok()?.as_json_object_value().cloned()
                } else {
                    None
                }
            })
            .unwrap();

        let option_keys: Vec<TokenText> = options_obj
            .json_member_list()
            .iter()
            .filter_map(|m| m.ok()?.name().ok()?.inner_string_text())
            .collect();
        assert_eq!(option_keys, vec!["semi", "tabWidth"]);
    }

    #[test]
    fn test_prettier_overrides_already_sorted_returns_none() {
        // Everything already sorted: keys alphabetical, overrides last, override entries sorted
        let obj = parse_object(
            r#"{"semi": false, "overrides": [{"files": "*.ts", "options": {"semi": false, "tabWidth": 2}}]}"#,
        );
        let result = transform(&AnyJsonValue::from(obj));
        assert!(
            result.is_none(),
            "Should return None when overrides entries are already sorted"
        );
    }
}
