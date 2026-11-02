use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
use std::num::NonZeroU16;

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoVueExcessivePropsOptions {
    /// The maximum number of props a component can declare.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub max_props: Option<NonZeroU16>,
}

impl NoVueExcessivePropsOptions {
    pub const DEFAULT_MAX_PROPS: NonZeroU16 = NonZeroU16::new(10).unwrap();

    /// Returns [`Self::max_props`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_MAX_PROPS`].
    pub fn max_props(&self) -> NonZeroU16 {
        self.max_props.unwrap_or(Self::DEFAULT_MAX_PROPS)
    }
}
