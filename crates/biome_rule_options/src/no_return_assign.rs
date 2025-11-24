use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoReturnAssignOptions {
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    except_parenthesis: Option<bool>,
}

impl NoReturnAssignOptions {
    pub const DEFAULT_EXCEPT_PARENTHESIS: bool = false;

    /// Returns [`Self::ignore_null`] if it is set
    /// Otherwise, returns [`Self::DEFAULT_IGNORE_NULL`].
    pub fn except_parenthesis(&self) -> bool {
        self.except_parenthesis
            .unwrap_or(Self::DEFAULT_EXCEPT_PARENTHESIS)
    }
}
