use biome_json_factory::make;
use biome_json_syntax::{AnyJsonValue, JsonMember, T};

pub fn transform(value: &AnyJsonValue) -> Option<AnyJsonValue> {
    let object = value.as_json_object_value()?;
    let members = object.json_member_list();

    let mut keys_with_members: Vec<(String, JsonMember)> = Vec::new();
    let mut overrides_member: Option<JsonMember> = None;

    for member in &members {
        if let Ok(m) = member
            && let Ok(name) = m.name().and_then(|n| n.inner_string_text())
        {
            let key = name.text().to_string();
            if key == "overrides" {
                overrides_member = Some(m.clone());
            } else {
                keys_with_members.push((key, m.clone()));
            }
        }
    }

    if keys_with_members.len() < 2 && overrides_member.is_none() {
        return None;
    }

    // Check if already sorted (alphabetical + overrides last if present)
    let current_keys: Vec<String> = keys_with_members.iter().map(|(k, _)| k.clone()).collect();
    let mut sorted_keys = current_keys.clone();
    sorted_keys.sort();

    let overrides_is_last = if overrides_member.is_some() {
        (&members)
            .into_iter()
            .filter_map(|m| m.ok())
            .last()
            .and_then(|m| m.name().ok()?.inner_string_text().ok())
            .map(|name| name.text() == "overrides")
            .unwrap_or(false)
    } else {
        true
    };

    if current_keys == sorted_keys && overrides_is_last {
        return None;
    }

    keys_with_members.sort_by(|a, b| a.0.cmp(&b.0));

    let mut elements = Vec::new();
    let mut separators = Vec::new();

    for (_, member) in &keys_with_members {
        elements.push(member.clone());
        separators.push(make::token(T![,]));
    }

    if let Some(overrides) = overrides_member {
        elements.push(overrides);
    } else if !separators.is_empty() {
        separators.pop();
    }

    let new_members = make::json_member_list(elements, separators);
    let sorted_object = object.clone().with_json_member_list(new_members);

    Some(AnyJsonValue::from(sorted_object))
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

        let keys: Vec<String> = (&result_obj.json_member_list())
            .into_iter()
            .filter_map(|m| {
                let member = m.ok()?;
                let text = member.name().ok()?.inner_string_text().ok()?;
                Some(text.text().to_string())
            })
            .collect();

        assert_eq!(keys, vec!["semi", "tabWidth", "overrides"]);
    }

    #[test]
    fn test_prettier_alphabetical_without_overrides() {
        let obj = parse_object(r#"{"tabWidth": 2, "semi": false, "singleQuote": true}"#);
        let result = transform(&AnyJsonValue::from(obj)).expect("Should sort alphabetically");
        let result_obj = result.as_json_object_value().unwrap();

        let keys: Vec<String> = (&result_obj.json_member_list())
            .into_iter()
            .filter_map(|m| {
                let member = m.ok()?;
                let text = member.name().ok()?.inner_string_text().ok()?;
                Some(text.text().to_string())
            })
            .collect();

        assert_eq!(keys, vec!["semi", "singleQuote", "tabWidth"]);
    }

    #[test]
    fn test_prettier_already_sorted_returns_none() {
        let obj = parse_object(r#"{"semi": false, "tabWidth": 2, "overrides": []}"#);
        let result = transform(&AnyJsonValue::from(obj));
        assert!(result.is_none(), "Should return None when already sorted");
    }
}
