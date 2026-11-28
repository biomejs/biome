use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

/// Configures the required declaration style for default-exported functions and classes.
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseConsistentDefaultExportStyleOptions {
    /// The declaration style for default-exported functions. Default: `"inline"`.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub functions: Option<DefaultExportStyle>,
    /// The declaration style for default-exported classes. Default: `"inline"`.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub classes: Option<DefaultExportStyle>,
}

impl UseConsistentDefaultExportStyleOptions {
    /// Returns [`Self::functions`] if it is set.
    /// Otherwise, returns [`DefaultExportStyle::default()`].
    pub fn functions(&self) -> DefaultExportStyle {
        self.functions.unwrap_or_default()
    }

    /// Returns [`Self::classes`] if it is set.
    /// Otherwise, returns [`DefaultExportStyle::default()`].
    pub fn classes(&self) -> DefaultExportStyle {
        self.classes.unwrap_or_default()
    }
}

/// Where a default-exported function or class is declared.
#[derive(
    Clone, Copy, Debug, Default, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum DefaultExportStyle {
    /// Require the declaration inside the `export default` statement. This is the default.
    #[default]
    Inline,
    /// Require a separate declaration, followed by `export default` with its name.
    /// Functions must be declared as `const` arrow functions.
    Separate,
    /// Don't check this kind of declaration.
    Ignore,
}
