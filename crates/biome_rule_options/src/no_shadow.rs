use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Default, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoShadowOptions {
    /// Whether to ignore parameter names in function type annotations that
    /// shadow variables in the outer scope.
    ///
    /// Function type parameters (e.g., `callback: (options: unknown) => void`)
    /// define the shape of a callback but do not create runtime bindings in the
    /// enclosing scope. Setting this to `true` (the default) suppresses
    /// shadowing reports for these parameter names.
    ///
    /// Defaults to `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_function_type_parameter_name_value_shadow: Option<bool>,
}
