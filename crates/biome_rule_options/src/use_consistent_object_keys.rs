use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseConsistentObjectKeysOptions {
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub form: Option<NormalizationForm>,
}

impl UseConsistentObjectKeysOptions {
    pub const DEFAULT_FORM: NormalizationForm = NormalizationForm::NFC;

    /// Returns [`Self::form`] if it is set
    /// Otherwise, returns [`Self::DEFAULT_FORM`].
    pub fn form(&self) -> NormalizationForm {
        self.form.unwrap_or(Self::DEFAULT_FORM)
    }
}

#[derive(
    Clone, Copy, Debug, Default, Deserialize, Deserializable, Merge, Eq, Hash, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub enum NormalizationForm {
    /// Canonical Decomposition followed by Canonical Composition
    #[default]
    #[serde(rename = "NFC")]
    NFC,
    /// Canonical Decomposition
    #[serde(rename = "NFD")]
    NFD,
    /// Compatibility Decomposition followed by Canonical Composition
    #[serde(rename = "NFKC")]
    NFKC,
    /// Compatibility Decomposition
    #[serde(rename = "NFKD")]
    NFKD,
}
