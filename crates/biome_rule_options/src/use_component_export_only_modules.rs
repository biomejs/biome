use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseComponentExportOnlyModulesOptions {
    /// Allows the export of constants. This option is for environments that support it, such as [Vite](https://vitejs.dev/)
    #[serde(default)]
    pub allow_constant_export: Option<bool>,
    /// A list of names that can be additionally exported from the module This option is for exports that do not hinder [React Fast Refresh](https://github.com/facebook/react/tree/main/packages/react-refresh), such as [`meta` in Remix](https://remix.run/docs/en/main/route/meta)
    #[serde(default, skip_serializing_if = "Option::<_>::is_none")]
    pub allow_export_names: Option<Box<[Box<str>]>>,
}

impl biome_deserialize::Merge for UseComponentExportOnlyModulesOptions {
    fn merge_with(&mut self, other: Self) {
        self.allow_constant_export
            .merge_with(other.allow_constant_export);
        if let Some(allow_export_names) = other.allow_export_names {
            self.allow_export_names = Some(allow_export_names);
        }
    }
}

impl UseComponentExportOnlyModulesOptions {
    pub const DEFAULT_ALLOW_CONSTANT_EXPORT: bool = false;

    /// Returns [`Self::allow_constant_export`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_IGNORE_REST_SIBLINGS`].
    pub fn allow_constant_export(&self) -> bool {
        self.allow_constant_export
            .unwrap_or(Self::DEFAULT_ALLOW_CONSTANT_EXPORT)
    }
}
