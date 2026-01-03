use biome_json_factory::make;
use biome_json_syntax::{AnyJsonValue, JsonMember, JsonObjectValue, T};

use super::sorters::{sort_alphabetically, sort_object_by_key_order};

/// https://github.com/eslint/eslint/blob/acc0e47572a9390292b4e313b4a4bf360d236358/conf/config-schema.js
const ESLINT_BASE_CONFIG_PROPERTIES: &[&str] = &[
    "files",
    "excludedFiles",
    "env",
    "parser",
    "parserOptions",
    "settings",
    "plugins",
    "extends",
    "rules",
    "overrides",
    "globals",
    "processor",
    "noInlineConfig",
    "reportUnusedDisableDirectives",
];

pub fn transform(value: &AnyJsonValue) -> Option<AnyJsonValue> {
    let object = value.as_json_object_value()?;

    let sorted_base = sort_object_by_key_order(object, ESLINT_BASE_CONFIG_PROPERTIES)?;

    let with_env = transform_field(&sorted_base, "env", |obj| {
        sort_alphabetically(obj).map(AnyJsonValue::from)
    });
    let with_globals = transform_field(&with_env, "globals", |obj| {
        sort_alphabetically(obj).map(AnyJsonValue::from)
    });
    let with_rules = transform_field(&with_globals, "rules", |obj| {
        sort_eslint_rules(obj).map(AnyJsonValue::from)
    });
    let with_parser_options = transform_field(&with_rules, "parserOptions", |obj| {
        sort_alphabetically(obj).map(AnyJsonValue::from)
    });
    let with_settings = transform_field(&with_parser_options, "settings", |obj| {
        sort_alphabetically(obj).map(AnyJsonValue::from)
    });

    Some(AnyJsonValue::from(with_settings))
}

fn transform_field<F>(
    object: &JsonObjectValue,
    field_name: &str,
    transform_fn: F,
) -> JsonObjectValue
where
    F: Fn(&JsonObjectValue) -> Option<AnyJsonValue>,
{
    let members = object.json_member_list();
    let mut elements = Vec::new();

    for m in (&members).into_iter().flatten() {
        let transformed_member = if let Ok(name) = m.name().and_then(|n| n.inner_string_text())
            && name.text() == field_name
            && let Ok(value) = m.value()
            && let Some(obj) = value.as_json_object_value()
            && let Some(new_value) = transform_fn(obj)
        {
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
    object.clone().with_json_member_list(new_members)
}

fn sort_eslint_rules(object: &JsonObjectValue) -> Option<JsonObjectValue> {
    let members = object.json_member_list();
    let mut member_vec: Vec<JsonMember> = Vec::new();

    for m in (&members).into_iter().flatten() {
        member_vec.push(m.clone());
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
