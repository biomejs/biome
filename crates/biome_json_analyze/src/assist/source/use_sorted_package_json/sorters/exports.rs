use biome_json_factory::make;
use biome_json_syntax::{AnyJsonValue, JsonMember, T};
use biome_rowan::TokenText;

pub fn transform(value: &AnyJsonValue) -> Option<AnyJsonValue> {
    let object = value.as_json_object_value()?;
    let members = object.json_member_list();

    let mut paths: Vec<(TokenText, JsonMember)> = Vec::new();
    let mut conditions: Vec<(TokenText, JsonMember)> = Vec::new();

    for member in &members {
        if let Ok(m) = member
            && let Some(name) = m.name().ok().and_then(|n| n.inner_string_text())
        {
            if name.text().starts_with('.') {
                paths.push((name, m.clone()));
            } else {
                conditions.push((name, m.clone()));
            }
        }
    }

    // Paths keep original order (NOT sorted alphabetically)
    // Only conditions are sorted (types first, default last, others in between)
    let sorted_conditions = sort_conditions(conditions.clone());

    // Check if top-level key order is already correct: paths (original order), then sorted conditions
    let mut expected_order: Vec<&TokenText> = Vec::new();
    expected_order.extend(paths.iter().map(|(k, _)| k));
    expected_order.extend(sorted_conditions.iter().map(|(k, _)| k));

    let current_order: Vec<TokenText> = (&members)
        .into_iter()
        .filter_map(|m| m.ok()?.name().ok()?.inner_string_text())
        .collect();

    let keys_already_sorted = current_order.iter().eq(expected_order.iter().copied());

    // Recursively transform all member values regardless of top-level key order
    let mut all_members = Vec::new();
    all_members.extend(paths);
    all_members.extend(sorted_conditions);

    let mut elements = Vec::new();
    let mut separators = Vec::new();
    let mut any_value_changed = false;

    for (i, (_key, member)) in all_members.iter().enumerate() {
        let value = member.value().ok()?;
        let transformed_value = transform(&value);
        if transformed_value.is_some() {
            any_value_changed = true;
        }
        let final_value = transformed_value.unwrap_or(value);
        let new_member = member.clone().with_value(final_value);

        elements.push(new_member);

        if i < all_members.len() - 1 {
            separators.push(make::token(T![,]));
        }
    }

    // Only return None (no change) if both keys and all values are already correct
    if keys_already_sorted && !any_value_changed {
        return None;
    }

    let new_members = make::json_member_list(elements, separators);
    let new_object = object.clone().with_json_member_list(new_members);

    Some(AnyJsonValue::from(new_object))
}

fn sort_conditions(conditions: Vec<(TokenText, JsonMember)>) -> Vec<(TokenText, JsonMember)> {
    let mut types_conditions = Vec::new();
    let mut default_conditions = Vec::new();
    let mut rest_conditions = Vec::new();

    for item in conditions {
        let key = &item.0;
        if *key == "types" || key.text().starts_with("types@") {
            types_conditions.push(item);
        } else if *key == "default" {
            default_conditions.push(item);
        } else {
            rest_conditions.push(item);
        }
    }

    let mut result = Vec::new();
    result.extend(types_conditions);
    result.extend(rest_conditions);
    result.extend(default_conditions);

    result
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
    fn test_condition_ordering() {
        let obj = parse_object(
            r#"{"default": "./default.js", "unknown": "./unknown.js", "types": "./types.d.ts"}"#,
        );
        let result = transform(&AnyJsonValue::from(obj.clone())).unwrap();
        let result_obj = result.as_json_object_value().unwrap();

        let keys: Vec<TokenText> = result_obj
            .json_member_list()
            .iter()
            .filter_map(|m| m.ok()?.name().ok()?.inner_string_text())
            .collect();

        assert_eq!(keys, vec!["types", "unknown", "default"]);
    }

    #[test]
    fn test_paths_before_conditions() {
        let obj =
            parse_object(r#"{"types": "./types.d.ts", "./path": {}, "default": "./default.js"}"#);
        let result = transform(&AnyJsonValue::from(obj.clone()))
            .expect("Should transform when paths mixed with conditions");
        let result_obj = result.as_json_object_value().unwrap();

        let keys: Vec<TokenText> = result_obj
            .json_member_list()
            .iter()
            .filter_map(|m| m.ok()?.name().ok()?.inner_string_text())
            .collect();

        assert_eq!(keys, vec!["./path", "types", "default"]);
    }
}
