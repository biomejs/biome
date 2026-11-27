use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseCatchErrorNameOptions {
    /// The required name of caught errors.
    /// Defaults to `error`.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub name: Option<Box<str>>,

    /// Names that are always allowed.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub ignore: Option<Box<[Box<str>]>>,
}

impl UseCatchErrorNameOptions {
    pub const DEFAULT_NAME: &'static str = "error";

    /// Returns [`Self::name`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_NAME`].
    pub fn name(&self) -> &str {
        self.name.as_deref().unwrap_or(Self::DEFAULT_NAME)
    }

    /// Returns [`Self::ignore`] if it is set.
    /// Otherwise, returns an empty slice.
    pub fn ignore(&self) -> &[Box<str>] {
        self.ignore.as_deref().unwrap_or_default()
    }
}
