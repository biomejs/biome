use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Default, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseDeprecatedDateOptions {
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub argument_name: Option<Box<str>>,
}

impl UseDeprecatedDateOptions {
    pub const DEFAULT_ARGUMENT_NAME: &str = "deletionDate";

    /// Returns [`Self::argument_name`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_ARGUMENT_NAME`]
    pub fn argument_name_or_default(&self) -> &str {
        self.argument_name
            .as_ref()
            .map_or(Self::DEFAULT_ARGUMENT_NAME, |name| name.as_ref())
    }
}
