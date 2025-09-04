use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoJsxPropsBindOptions {
    pub ignore_dom_components: bool,
    pub ignore_refs: bool,
    pub allow_arrow_functions: bool,
    pub allow_functions: bool,
    pub allow_bind: bool,
}
