use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseComponentExportOnlyModulesOptions {
    /// Allows the export of constants. This option is for environments that support it, such as [Vite](https://vitejs.dev/)
    #[serde(default)]
    pub allow_constant_export: bool,
    /// A list of names that can be additionally exported from the module This option is for exports that do not hinder [React Fast Refresh](https://github.com/facebook/react/tree/main/packages/react-refresh), such as [`meta` in Remix](https://remix.run/docs/en/main/route/meta)
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub allow_export_names: Box<[Box<str>]>,
}
