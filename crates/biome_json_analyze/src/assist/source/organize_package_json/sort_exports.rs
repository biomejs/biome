use biome_json_factory::make;
use biome_json_syntax::{AnyJsonValue, JsonMember, T};
use biome_rowan::AstSeparatedList;

pub fn transform(value: &AnyJsonValue) -> Option<AnyJsonValue> {
    let object = value.as_json_object_value()?;
    let members = object.json_member_list();

    let mut paths = Vec::new();
    let mut conditions = Vec::new();

    for member in &members {
        if let Ok(m) = member
            && let Ok(name) = m.name().and_then(|n| n.inner_string_text())
        {
            let key = name.text();
            if key.starts_with('.') {
                paths.push((key.to_string(), m.clone()));
            } else {
                conditions.push((key.to_string(), m.clone()));
            }
        }
    }

    let sorted_conditions = sort_conditions(conditions);

    let mut all_members = Vec::new();
    all_members.extend(paths);
    all_members.extend(sorted_conditions);

    let mut elements = Vec::new();
    let mut separators = Vec::new();

    for (i, (_key, member)) in all_members.iter().enumerate() {
        let value = member.value().ok()?;
        let transformed_value = transform(&value).unwrap_or(value);
        let new_member = member.clone().with_value(transformed_value);

        elements.push(new_member);

        if i < all_members.len() - 1 {
            separators.push(make::token(T![,]));
        }
    }

    let new_members = make::json_member_list(elements, separators);
    let new_object = object.clone().with_json_member_list(new_members);

    Some(AnyJsonValue::from(new_object))
}

fn sort_conditions(conditions: Vec<(String, JsonMember)>) -> Vec<(String, JsonMember)> {
    let mut types_conditions = Vec::new();
    let mut default_conditions = Vec::new();
    let mut rest_conditions = Vec::new();

    for item in conditions {
        let key = &item.0;
        if key == "types" || key.starts_with("types@") {
            types_conditions.push(item);
        } else if key == "default" {
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

        let keys: Vec<String> = result_obj
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

        assert_eq!(keys, vec!["types", "unknown", "default"]);
    }

    #[test]
    fn test_paths_before_conditions() {
        let obj =
            parse_object(r#"{"types": "./types.d.ts", "./path": {}, "default": "./default.js"}"#);
        let result = transform(&AnyJsonValue::from(obj.clone())).unwrap();
        let result_obj = result.as_json_object_value().unwrap();

        let keys: Vec<String> = result_obj
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

        assert_eq!(keys, vec!["./path", "types", "default"]);
    }
}
