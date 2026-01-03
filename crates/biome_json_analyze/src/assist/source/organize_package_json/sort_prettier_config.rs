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
