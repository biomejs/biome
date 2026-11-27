use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoTripleSlashReferenceOptions {
    /// Whether `/// <reference lib="..." />` comments are allowed.
    /// Set it to `"never"` to report all of them. Defaults to `"always"`.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub lib: Option<TripleSlashReferenceUsage>,

    /// Whether `/// <reference path="..." />` comments are allowed.
    /// Set it to `"always"` to allow all of them. Defaults to `"never"`.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub path: Option<TripleSlashReferenceUsage>,

    /// Whether `/// <reference types="..." />` comments are allowed.
    /// With `"preferImport"`, a comment is reported only when the file also imports the same
    /// package. Defaults to `"preferImport"`.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub types: Option<TripleSlashTypesReferenceUsage>,
}

impl NoTripleSlashReferenceOptions {
    pub const DEFAULT_LIB: TripleSlashReferenceUsage = TripleSlashReferenceUsage::Always;
    pub const DEFAULT_PATH: TripleSlashReferenceUsage = TripleSlashReferenceUsage::Never;
    pub const DEFAULT_TYPES: TripleSlashTypesReferenceUsage =
        TripleSlashTypesReferenceUsage::PreferImport;

    /// Returns [`Self::lib`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_LIB`].
    pub fn lib(&self) -> TripleSlashReferenceUsage {
        self.lib.unwrap_or(Self::DEFAULT_LIB)
    }

    /// Returns [`Self::path`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_PATH`].
    pub fn path(&self) -> TripleSlashReferenceUsage {
        self.path.unwrap_or(Self::DEFAULT_PATH)
    }

    /// Returns [`Self::types`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_TYPES`].
    pub fn types(&self) -> TripleSlashTypesReferenceUsage {
        self.types.unwrap_or(Self::DEFAULT_TYPES)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum TripleSlashReferenceUsage {
    /// Allows the reference.
    Always,
    /// Reports the reference.
    Never,
}

#[derive(Clone, Copy, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum TripleSlashTypesReferenceUsage {
    /// Allows the reference.
    Always,
    /// Reports the reference.
    Never,
    /// Reports the reference only when the file also imports the same package.
    PreferImport,
}
