pub mod constants;
pub mod dependencies;
pub mod dependencies_meta;
pub mod eslint_config;
pub mod exports;
pub mod helpers;
pub mod prettier_config;
pub mod scripts;

use biome_json_syntax::{AnyJsonValue, JsonObjectValue};

use super::field_order::FieldTransformer;
use constants::*;
use helpers::*;

/// Single source of truth for all field transformations.
/// Returns Some(transformed_value) if transformation applied, None if already sorted.
///
/// This function is called by both:
/// - `needs_transformation()` - checks `.is_some()` to determine if diagnostic needed
/// - `apply_field_transformer()` - unwraps result to apply the transformation
pub fn try_transform_field(
    value: &AnyJsonValue,
    transformer: FieldTransformer,
    root_object: &JsonObjectValue,
) -> Option<AnyJsonValue> {
    match transformer {
        FieldTransformer::None => None,

        // Simple object sorters - direct calls with constants
        FieldTransformer::SortObject => value
            .as_json_object_value()
            .and_then(sort_alphabetically)
            .map(AnyJsonValue::from),

        FieldTransformer::SortPeopleObject => value
            .as_json_object_value()
            .and_then(|obj| sort_object_by_key_order(obj, PEOPLE_FIELD_ORDER))
            .map(AnyJsonValue::from),

        FieldTransformer::SortURLObject => value
            .as_json_object_value()
            .and_then(|obj| sort_object_by_key_order(obj, URL_FIELD_ORDER))
            .map(AnyJsonValue::from),

        FieldTransformer::SortBugsObject => value
            .as_json_object_value()
            .and_then(|obj| sort_object_by_key_order(obj, BUGS_FIELD_ORDER))
            .map(AnyJsonValue::from),

        FieldTransformer::SortDirectories => value
            .as_json_object_value()
            .and_then(|obj| sort_object_by_key_order(obj, DIRECTORIES_FIELD_ORDER))
            .map(AnyJsonValue::from),

        FieldTransformer::SortVolta => value
            .as_json_object_value()
            .and_then(|obj| sort_object_by_key_order(obj, VOLTA_FIELD_ORDER))
            .map(AnyJsonValue::from),

        FieldTransformer::SortBinary => value
            .as_json_object_value()
            .and_then(|obj| sort_object_by_key_order(obj, BINARY_FIELD_ORDER))
            .map(AnyJsonValue::from),

        FieldTransformer::SortGitHooks => value
            .as_json_object_value()
            .and_then(|obj| sort_object_by_key_order(obj, GIT_HOOKS_ORDER))
            .map(AnyJsonValue::from),

        // Complex object sorters
        FieldTransformer::SortDependencies => dependencies::transform(value, root_object),

        FieldTransformer::SortDependenciesMeta => dependencies_meta::transform(value),

        FieldTransformer::SortScripts => scripts::transform(value, root_object),

        FieldTransformer::SortExports => exports::transform(value),

        FieldTransformer::SortEslintConfig => eslint_config::transform(value),

        FieldTransformer::SortPrettierConfig => prettier_config::transform(value),

        FieldTransformer::SortWorkspaces => value
            .as_json_object_value()
            .and_then(|obj| sort_object_by_key_order(obj, WORKSPACES_FIELD_ORDER))
            .map(AnyJsonValue::from),

        FieldTransformer::SortPnpmConfig => value
            .as_json_object_value()
            .and_then(|obj| sort_object_by_key_order(obj, PNPM_BASE_CONFIG_PROPERTIES))
            .map(AnyJsonValue::from),

        FieldTransformer::SortObjectDeep => value
            .as_json_object_value()
            .and_then(sort_alphabetically_deep)
            .map(AnyJsonValue::from),

        // Nested property transformers
        FieldTransformer::SortHusky => value
            .as_json_object_value()
            .and_then(|obj| {
                transform_nested_property(obj, "hooks", |hooks_value| {
                    hooks_value
                        .as_json_object_value()
                        .and_then(|hooks| sort_object_by_key_order(hooks, GIT_HOOKS_ORDER))
                        .map(AnyJsonValue::from)
                })
            })
            .map(AnyJsonValue::from),

        FieldTransformer::SortDevEngines => value
            .as_json_object_value()
            .and_then(|obj| {
                transform_nested_property(obj, "packageManager", |pm_value| {
                    pm_value
                        .as_json_object_value()
                        .and_then(|pm_obj| sort_object_by_key_order(pm_obj, DEV_ENGINES_PM_ORDER))
                        .map(AnyJsonValue::from)
                })
            })
            .map(AnyJsonValue::from),

        // Array transformers - inline transformations for array elements
        FieldTransformer::SortPeopleArray => transform_array_with(value, |obj| {
            sort_object_by_key_order(obj, PEOPLE_FIELD_ORDER).map(AnyJsonValue::from)
        }),

        FieldTransformer::SortBadgesArray => transform_array_with(value, |obj| {
            sort_object_by_key_order(obj, VSCODE_BADGE_FIELD_ORDER).map(AnyJsonValue::from)
        }),

        FieldTransformer::UniqArray => uniq_array(value),

        FieldTransformer::UniqAndSortArray => uniq_and_sort_array(value),
    }
}
