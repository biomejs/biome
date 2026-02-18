use biome_json_factory::make;
use biome_json_syntax::{AnyJsonValue, JsonMember, JsonObjectValue, T};
use biome_rowan::TokenText;

use super::constants::ESLINT_BASE_CONFIG_PROPERTIES;
use super::helpers::{extract_member_names, sort_alphabetically, sort_object_by_key_order};

/// https://github.com/eslint/eslint/blob/acc0e47572a9390292b4e313b4a4bf360d236358/conf/config-schema.js
pub fn transform(value: &AnyJsonValue) -> Option<AnyJsonValue> {
    let object = value.as_json_object_value()?;

    // Sort base config and track if any changes occur
    let mut changed = false;
    let base = if let Some(sorted) = sort_object_by_key_order(object, ESLINT_BASE_CONFIG_PROPERTIES)
    {
        changed = true;
        sorted
    } else {
        object.clone()
    };

    // Process nested fields - track changes
    let (with_env, env_changed) = transform_field_tracked(&base, "env", |obj| {
        sort_alphabetically(obj).map(AnyJsonValue::from)
    });
    changed |= env_changed;

    let (with_globals, globals_changed) = transform_field_tracked(&with_env, "globals", |obj| {
        sort_alphabetically(obj).map(AnyJsonValue::from)
    });
    changed |= globals_changed;

    let (with_rules, rules_changed) = transform_field_tracked(&with_globals, "rules", |obj| {
        sort_eslint_rules(obj).map(AnyJsonValue::from)
    });
    changed |= rules_changed;

    let (with_parser_options, parser_changed) =
        transform_field_tracked(&with_rules, "parserOptions", |obj| {
            sort_alphabetically(obj).map(AnyJsonValue::from)
        });
    changed |= parser_changed;

    let (with_settings, settings_changed) =
        transform_field_tracked(&with_parser_options, "settings", |obj| {
            sort_alphabetically(obj).map(AnyJsonValue::from)
        });
    changed |= settings_changed;

    // Return None if no changes, Some if any transformation occurred
    if changed {
        Some(AnyJsonValue::from(with_settings))
    } else {
        None
    }
}

fn transform_field_tracked<F>(
    object: &JsonObjectValue,
    field_name: &str,
    transform_fn: F,
) -> (JsonObjectValue, bool)
where
    F: Fn(&JsonObjectValue) -> Option<AnyJsonValue>,
{
    let members = object.json_member_list();
    let mut elements = Vec::new();
    let mut changed = false;

    for m in (&members).into_iter().flatten() {
        let transformed_member = if let Some(name) =
            m.name().ok().and_then(|n| n.inner_string_text())
            && name == field_name
            && let Ok(value) = m.value()
            && let Some(obj) = value.as_json_object_value()
            && let Some(new_value) = transform_fn(obj)
        {
            changed = true;
            m.clone().with_value(new_value)
        } else {
            m.clone()
        };

        elements.push(transformed_member);
    }

    let mut separators = Vec::new();
    for _ in 0..(elements.len().saturating_sub(1)) {
        separators.push(make::token(T![,]));
    }

    let new_members = make::json_member_list(elements, separators);
    let result = object.clone().with_json_member_list(new_members);

    (result, changed)
}

fn sort_eslint_rules(object: &JsonObjectValue) -> Option<JsonObjectValue> {
    let members = object.json_member_list();
    let mut member_vec: Vec<JsonMember> = Vec::new();

    for m in (&members).into_iter().flatten() {
        member_vec.push(m.clone());
    }

    if member_vec.len() < 2 {
        return None;
    }

    // Clone original order for comparison
    let original = member_vec.clone();

    member_vec.sort_by(|a, b| {
        let a_name: Option<TokenText> = a.name().ok().and_then(|n| n.inner_string_text());
        let b_name: Option<TokenText> = b.name().ok().and_then(|n| n.inner_string_text());

        match (a_name, b_name) {
            (Some(a), Some(b)) => {
                let a_slash_count = a.matches('/').count();
                let b_slash_count = b.matches('/').count();

                match a_slash_count.cmp(&b_slash_count) {
                    std::cmp::Ordering::Equal => a.cmp(&b),
                    other => other,
                }
            }
            _ => std::cmp::Ordering::Equal,
        }
    });

    // Check if order changed by comparing member names
    let original_keys = extract_member_names(&original);
    let sorted_keys = extract_member_names(&member_vec);

    if original_keys == sorted_keys {
        return None;
    }

    let mut elements = Vec::new();
    let mut separators = Vec::new();

    for (i, member) in member_vec.iter().enumerate() {
        elements.push(member.clone());
        if i < member_vec.len() - 1 {
            separators.push(make::token(T![,]));
        }
    }

    let new_members = make::json_member_list(elements, separators);
    Some(object.clone().with_json_member_list(new_members))
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_json_parser::{JsonParserOptions, parse_json};
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
    fn test_eslint_base_config_field_order() {
        let obj = parse_object(r#"{"rules": {}, "extends": [], "plugins": [], "env": {}}"#);
        let result = transform(&AnyJsonValue::from(obj)).expect("Should reorder fields");
        let result_obj = result.as_json_object_value().unwrap();

        let keys: Vec<TokenText> = result_obj
            .json_member_list()
            .iter()
            .filter_map(|m| m.ok()?.name().ok()?.inner_string_text())
            .collect();

        assert_eq!(keys, vec!["env", "plugins", "extends", "rules"]);
    }

    #[test]
    fn test_eslint_rules_builtin_first() {
        let obj = parse_object(
            r#"{"rules": {"react/jsx-key": "error", "no-console": "warn", "prettier/prettier": "error"}}"#,
        );
        let result = transform(&AnyJsonValue::from(obj)).expect("Should sort rules");
        let result_obj = result.as_json_object_value().unwrap();

        // Get rules object
        let rules_member = result_obj
            .json_member_list()
            .into_iter()
            .find_map(|m| {
                let member = m.ok()?;
                let name = member.name().ok()?.inner_string_text()?;
                if name == "rules" {
                    Some(member.value().ok()?.as_json_object_value()?.clone())
                } else {
                    None
                }
            })
            .unwrap();

        let rule_keys: Vec<TokenText> = rules_member
            .json_member_list()
            .iter()
            .filter_map(|m| m.ok()?.name().ok()?.inner_string_text())
            .collect();

        // Builtin rules first, then plugins (sorted by slash count then alphabetically)
        assert_eq!(
            rule_keys,
            vec!["no-console", "prettier/prettier", "react/jsx-key"]
        );
    }

    #[test]
    fn test_eslint_already_sorted_returns_none() {
        let obj =
            parse_object(r#"{"env": {}, "parser": "espree", "rules": {"no-console": "warn"}}"#);
        let result = transform(&AnyJsonValue::from(obj));
        assert!(result.is_none(), "Should return None when already sorted");
    }

    #[test]
    fn test_eslint_rules_already_sorted_returns_none() {
        let obj = parse_object(
            r#"{"rules": {"no-console": "warn", "prettier/prettier": "error", "react/jsx-key": "error"}}"#,
        );
        let result = transform(&AnyJsonValue::from(obj));
        assert!(
            result.is_none(),
            "Should return None when rules are already sorted"
        );
    }

    #[test]
    fn test_eslint_nested_env_sorted() {
        let obj = parse_object(r#"{"env": {"node": true, "browser": true}}"#);
        let result = transform(&AnyJsonValue::from(obj)).expect("Should sort nested env");
        let result_obj = result.as_json_object_value().unwrap();

        let env_member = result_obj
            .json_member_list()
            .into_iter()
            .find_map(|m| {
                let member = m.ok()?;
                let name = member.name().ok()?.inner_string_text()?;
                if name == "env" {
                    Some(member.value().ok()?.as_json_object_value()?.clone())
                } else {
                    None
                }
            })
            .unwrap();

        let env_keys: Vec<TokenText> = env_member
            .json_member_list()
            .iter()
            .filter_map(|m| m.ok()?.name().ok()?.inner_string_text())
            .collect();

        assert_eq!(env_keys, vec!["browser", "node"]);
    }
}
