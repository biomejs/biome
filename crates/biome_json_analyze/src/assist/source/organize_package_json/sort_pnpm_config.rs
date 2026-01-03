use biome_json_syntax::AnyJsonValue;

use super::sorters::sort_object_by_key_order;

const PNPM_BASE_CONFIG_PROPERTIES: &[&str] = &[
    "peerDependencyRules",
    "neverBuiltDependencies",
    "onlyBuiltDependencies",
    "onlyBuiltDependenciesFile",
    "allowedDeprecatedVersions",
    "allowNonAppliedPatches",
    "updateConfig",
    "auditConfig",
    "requiredScripts",
    "supportedArchitectures",
    "overrides",
    "patchedDependencies",
    "packageExtensions",
];

pub fn transform(value: &AnyJsonValue) -> Option<AnyJsonValue> {
    let object = value.as_json_object_value()?;
    let sorted = sort_object_by_key_order(object, PNPM_BASE_CONFIG_PROPERTIES)?;
    Some(AnyJsonValue::from(sorted))
}
