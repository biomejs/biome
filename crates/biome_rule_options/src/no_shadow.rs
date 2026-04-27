use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoShadowOptions {
    /// Ignore cases where a type and a value share the same name.
    ///
    /// Types and values live in separate namespaces in TypeScript, so a
    /// variable named `Foo` and a `type Foo` cannot collide at runtime.
    ///
    /// Defaults to `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_type_value_shadow: Option<bool>,

    /// Ignore parameter names in function type annotations.
    ///
    /// Function type parameters (e.g. `(x: string) => void`) only create
    /// bindings within the type scope and rarely cause confusion.
    ///
    /// Defaults to `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_function_type_parameter_name_value_shadow: Option<bool>,
}

impl NoShadowOptions {
    pub const DEFAULT_IGNORE_TYPE_VALUE_SHADOW: bool = true;
    pub const DEFAULT_IGNORE_FUNCTION_TYPE_PARAMETER_NAME_VALUE_SHADOW: bool = true;

    pub fn ignore_type_value_shadow(&self) -> bool {
        self.ignore_type_value_shadow
            .unwrap_or(Self::DEFAULT_IGNORE_TYPE_VALUE_SHADOW)
    }

    pub fn ignore_function_type_parameter_name_value_shadow(&self) -> bool {
        self.ignore_function_type_parameter_name_value_shadow
            .unwrap_or(Self::DEFAULT_IGNORE_FUNCTION_TYPE_PARAMETER_NAME_VALUE_SHADOW)
    }
}
