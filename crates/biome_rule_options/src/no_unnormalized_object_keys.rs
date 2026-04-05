use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoUnnormalizedObjectKeysOptions {
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub form: Option<NormalizationForm>,
}

impl NoUnnormalizedObjectKeysOptions {
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
    #[default]
    #[serde(rename = "NFC")]
    NFC,
    #[serde(rename = "NFD")]
    NFD,
    #[serde(rename = "NFKC")]
    NFKC,
    #[serde(rename = "NFKD")]
    NFKD,
}
