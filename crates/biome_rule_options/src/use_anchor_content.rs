use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseAnchorContentOptions {
    /// Additional JSX prop names that indicate the anchor is a render prop
    /// whose children will be injected by the receiving component.
    ///
    /// The prop name `"render"` is always recognised. Use this option to add
    /// further names (e.g. `"as"`, `"component"`) without replacing the default.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub additional_render_props: Vec<String>,
}
