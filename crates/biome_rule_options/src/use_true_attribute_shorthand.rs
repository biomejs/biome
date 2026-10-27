use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseTrueAttributeShorthandOptions {
    /// How a component prop set to `true` should be written: `"shorthand"` writes only the
    /// prop name, and `"longhand"` writes the value `true` explicitly.
    /// Default: `"shorthand"`.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub style: Option<TrueAttributeStyle>,

    /// A list of prop names that must use the opposite of `style`.
    /// Names must match exactly, as written in the template.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub except: Option<Box<[Box<str>]>>,
}

#[derive(
    Clone, Copy, Default, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum TrueAttributeStyle {
    /// Write only the prop name, such as `<MyComponent disabled />`.
    #[default]
    Shorthand,
    /// Write the value `true` explicitly, such as `<MyComponent :disabled="true" />`.
    Longhand,
}

impl UseTrueAttributeShorthandOptions {
    pub const DEFAULT_STYLE: TrueAttributeStyle = TrueAttributeStyle::Shorthand;

    /// Returns [`Self::style`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_STYLE`].
    pub fn style(&self) -> TrueAttributeStyle {
        self.style.unwrap_or(Self::DEFAULT_STYLE)
    }

    /// Returns [`Self::except`] if it is set.
    /// Otherwise, returns an empty slice.
    pub fn except(&self) -> &[Box<str>] {
        self.except.as_deref().unwrap_or_default()
    }
}
