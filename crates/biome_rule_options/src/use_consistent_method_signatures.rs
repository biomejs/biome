use biome_console::fmt::{Display, Formatter};
use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

/// Options type for `useConsistentMethodSignatures`.
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseConsistentMethodSignaturesOptions {
    /// The style of method signatures whose usage will be enforced.
    ///
    /// Default: "property"
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub style: Option<MethodSignatureStyle>,
}

#[derive(
    Clone, Copy, Debug, Default, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub enum MethodSignatureStyle {
    /// Enforce use of property-style signatures (`methodName: (args) => returnType`).
    #[default]
    Property,
    /// Enforce use of method-style signatures (`methodName(args): returnType`).
    Method,
}

impl Display for MethodSignatureStyle {
    fn fmt(&self, fmt: &mut Formatter) -> std::io::Result<()> {
        match self {
            Self::Property => fmt.write_str("property"),
            Self::Method => fmt.write_str("method"),
        }
    }
}
