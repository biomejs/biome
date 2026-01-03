use biome_json_syntax::{AnyJsonValue, JsonObjectValue};

use super::sorters::sort_object_by_key_order;

/// https://docs.npmjs.com/cli/v7/using-npm/workspaces?v=true#running-commands-in-the-context-of-workspaces
pub fn transform(value: &AnyJsonValue, _root: &JsonObjectValue) -> Option<AnyJsonValue> {
    let object = value.as_json_object_value()?;

    let sorted_base = sort_object_by_key_order(object, &["packages", "catalog"])?;

    Some(AnyJsonValue::from(sorted_base))
}
