use biome_json_syntax::AnyJsonValue;

use super::sorters::sort_object_by_key_order;

/// Sorts workspaces object by key order: packages, catalog (then alphabetically for others).
/// Arrays inside are NOT sorted to preserve workspace execution order.
///
/// https://docs.npmjs.com/cli/v7/using-npm/workspaces?v=true#running-commands-in-the-context-of-workspaces
pub fn transform(value: &AnyJsonValue) -> Option<AnyJsonValue> {
    let object = value.as_json_object_value()?;
    let sorted = sort_object_by_key_order(object, &["packages", "catalog"])?;
    Some(AnyJsonValue::from(sorted))
}
