use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoTypeofUndefinedOptions {
    /// If `true`, the rule also checks unresolved or global identifiers.
    ///
    /// This is disabled by default because `typeof missingGlobal === "undefined"`
    /// is runtime-safe, while `missingGlobal === undefined` can throw.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub check_global_variables: Option<bool>,
}

impl NoTypeofUndefinedOptions {
    pub const DEFAULT_CHECK_GLOBAL_VARIABLES: bool = false;

    /// Returns [`Self::check_global_variables`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_CHECK_GLOBAL_VARIABLES`].
    pub fn check_global_variables(&self) -> bool {
        self.check_global_variables
            .unwrap_or(Self::DEFAULT_CHECK_GLOBAL_VARIABLES)
    }
}
