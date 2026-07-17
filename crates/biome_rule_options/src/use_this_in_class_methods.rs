use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

/// Options for the `useThisInClassMethods` rule.
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseThisInClassMethodsOptions {
    /// Method names that should be ignored by the rule.
    ///
    /// Defaults to `[]`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_methods: Option<Box<[Box<str>]>>,

    /// Whether methods marked with `override` should be ignored.
    ///
    /// Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_override_methods: Option<bool>,

    /// Whether members of classes with an `implements` clause should be ignored.
    ///
    /// Defaults to `"none"`, which means implemented classes are checked like any other class.
    /// Use `"all"` to ignore every eligible member in such classes, or `"public-fields"`
    /// to ignore only public members in them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_classes_with_implements: Option<IgnoreClassesWithImplements>,
}

impl UseThisInClassMethodsOptions {
    pub fn ignore_classes_with_implements(&self) -> IgnoreClassesWithImplements {
        self.ignore_classes_with_implements.unwrap_or_default()
    }
}

/// Controls how `useThisInClassMethods` treats classes that implement interfaces.
#[derive(
    Clone, Copy, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum IgnoreClassesWithImplements {
    /// Check members in classes that implement interfaces the same way as any other class.
    #[serde(rename = "none")]
    #[default]
    None,

    /// Ignore every eligible instance member in classes that implement at least one interface.
    #[serde(rename = "all")]
    All,

    /// Ignore only public eligible members in classes that implement at least one interface.
    ///
    /// Protected and private members in those classes are still checked.
    #[serde(rename = "public-fields")]
    PublicFields,
}
