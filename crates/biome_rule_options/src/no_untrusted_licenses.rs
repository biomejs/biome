use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoUntrustedLicensesOptions {
    /// Additional license identifiers to trust, beyond valid SPDX identifiers.
    ///
    /// Useful for custom or proprietary licenses that are not part of the SPDX
    /// standard but are acceptable in your project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow: Option<Box<[Box<str>]>>,

    /// License identifiers to explicitly deny, even if they are valid SPDX identifiers.
    ///
    /// Use this to block specific licenses that your project or organization can't use (e.g.,
    /// copyleft licenses in a proprietary project).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deny: Option<Box<[Box<str>]>>,

    /// When `true`, only licenses approved by the Open Source Initiative (OSI)
    /// are trusted. Licenses in the `allow` list bypass this check.
    /// Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_osi_approved: Option<bool>,

    /// When `true`, only licenses recognized as free/libre by the Free Software
    /// Foundation (FSF) are trusted. Licenses in the `allow` list bypass this check.
    /// Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_fsf_libre: Option<bool>,

    /// When `true`, deprecated SPDX license identifiers are accepted.
    /// When `false`, deprecated licenses are flagged as untrusted.
    /// Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_deprecated: Option<bool>,
}

impl biome_deserialize::Merge for NoUntrustedLicensesOptions {
    fn merge_with(&mut self, other: Self) {
        if let Some(allow) = other.allow {
            self.allow = Some(allow);
        }
        if let Some(deny) = other.deny {
            self.deny = Some(deny);
        }
        if let Some(require_osi_approved) = other.require_osi_approved {
            self.require_osi_approved = Some(require_osi_approved);
        }
        if let Some(require_fsf_libre) = other.require_fsf_libre {
            self.require_fsf_libre = Some(require_fsf_libre);
        }
        if let Some(ignore_deprecated) = other.ignore_deprecated {
            self.ignore_deprecated = Some(ignore_deprecated);
        }
    }
}
