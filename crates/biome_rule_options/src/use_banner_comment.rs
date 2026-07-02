use biome_deserialize::{
    Deserializable, DeserializableType, DeserializableValue, DeserializationContext,
};
use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseBannerCommentOptions {
    /// The expected banner content.
    ///
    /// Accepts either a single string (one canonical banner) or an array of
    /// strings (any one of which is an acceptable banner).
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub content: Option<BannerContent>,
}

impl biome_deserialize::Merge for UseBannerCommentOptions {
    fn merge_with(&mut self, other: Self) {
        if other.content.is_some() {
            self.content = other.content;
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum BannerContent {
    Single(Box<str>),
    Multiple(Box<[Box<str>]>),
}

#[cfg(feature = "schema")]
impl schemars::JsonSchema for BannerContent {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("BannerContent")
    }

    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({
            "anyOf": [
                {
                    "type": "string",
                    "description": "A single canonical banner."
                },
                {
                    "type": "array",
                    "items": { "type": "string" },
                    "minItems": 1,
                    "description": "A list of acceptable banners. The first entry is used when auto-fixing."
                }
            ]
        })
    }
}

impl Deserializable for BannerContent {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        if value.visitable_type()? == DeserializableType::Str {
            biome_deserialize::Deserializable::deserialize(ctx, value, name).map(Self::Single)
        } else {
            biome_deserialize::Deserializable::deserialize(ctx, value, name).map(Self::Multiple)
        }
    }
}

impl UseBannerCommentOptions {
    /// Returns the list of acceptable banner bodies (text expected between
    /// `/*` and `*/`). The first entry is treated as the canonical form used
    /// when auto-fixing. Returns an empty slice when no content is configured.
    pub fn accepted_contents(&self) -> &[Box<str>] {
        match self.content.as_ref() {
            None => &[],
            Some(BannerContent::Single(s)) => std::slice::from_ref(s),
            Some(BannerContent::Multiple(many)) => many.as_ref(),
        }
    }
}
