use biome_deserialize::DeserializationContext;
use biome_deserialize_macros::Deserializable;
use biome_rowan::TextRange;
use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;
use serde::{Deserialize, Serialize};

/// The configured blocks, keyed by tag name.
pub type BlockLangMap = IndexMap<Box<str>, BlockLangOptions, FxBuildHasher>;

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseConsistentBlockLangOptions {
    /// The top-level blocks to check in Vue and Svelte components.
    ///
    /// Each key is the tag name of a block, without angle brackets, such as
    /// `script`, `style`, or a Vue custom block like `i18n`. Blocks that aren't
    /// listed aren't checked. When this option is omitted, the rule doesn't
    /// check anything.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub blocks: Option<BlockLangMap>,
}

impl biome_deserialize::Merge for UseConsistentBlockLangOptions {
    fn merge_with(&mut self, other: Self) {
        if let Some(blocks) = other.blocks {
            self.blocks = Some(blocks);
        }
    }
}

/// The checks for one top-level block.
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BlockLangOptions {
    /// The values allowed for the `lang` attribute of the block, such as `["ts"]`.
    ///
    /// When set, it must contain at least one value. Values must match exactly,
    /// so `"ts"` doesn't allow `lang="TS"`. When omitted, the `lang` attribute
    /// of the block isn't checked.
    #[deserializable(bail_on_error, validate = "non_empty_lang")]
    #[cfg_attr(feature = "schema", schemars(length(min = 1)))]
    #[serde(default, skip_serializing_if = "Option::<_>::is_none")]
    pub lang: Option<Box<[Box<str>]>>,

    /// When `true`, the block may also omit the `lang` attribute.
    ///
    /// Only applies when `lang` is set. Defaults to `false`.
    #[serde(default, skip_serializing_if = "Option::<_>::is_none")]
    pub allow_no_lang: Option<bool>,

    /// When `true`, every component must contain at least one block with this
    /// tag name.
    ///
    /// Works with or without `lang`. Defaults to `false`.
    #[serde(default, skip_serializing_if = "Option::<_>::is_none")]
    pub required: Option<bool>,
}

impl BlockLangOptions {
    pub const DEFAULT_ALLOW_NO_LANG: bool = false;
    pub const DEFAULT_REQUIRED: bool = false;

    /// Returns [`Self::allow_no_lang`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_ALLOW_NO_LANG`].
    pub fn allow_no_lang(&self) -> bool {
        self.allow_no_lang.unwrap_or(Self::DEFAULT_ALLOW_NO_LANG)
    }

    /// Returns [`Self::required`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_REQUIRED`].
    pub fn required(&self) -> bool {
        self.required.unwrap_or(Self::DEFAULT_REQUIRED)
    }
}

/// Accepts an omitted `lang`, but rejects an empty list, which would allow no
/// value at all.
fn non_empty_lang(
    ctx: &mut dyn DeserializationContext,
    value: &Option<Box<[Box<str>]>>,
    name: &str,
    range: TextRange,
) -> bool {
    value
        .as_ref()
        .is_none_or(|langs| biome_deserialize::non_empty(ctx, langs, name, range))
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_deserialize::json::deserialize_from_json_str;
    use biome_json_parser::JsonParserOptions;

    fn deserialize(source: &str) -> (Option<BlockLangOptions>, usize) {
        let (options, diagnostics) =
            deserialize_from_json_str::<BlockLangOptions>(source, JsonParserOptions::default(), "")
                .consume();
        (options, diagnostics.len())
    }

    #[test]
    fn accepts_missing_lang() {
        let (options, diagnostics) = deserialize(r#"{ "required": true }"#);
        assert_eq!(diagnostics, 0);
        assert!(options.unwrap().lang.is_none());
    }

    #[test]
    fn rejects_empty_lang() {
        let (options, diagnostics) = deserialize(r#"{ "lang": [] }"#);
        assert!(options.is_none());
        assert_eq!(diagnostics, 1);
    }

    #[test]
    fn accepts_non_empty_lang() {
        let (options, diagnostics) = deserialize(r#"{ "lang": ["ts"] }"#);
        assert_eq!(diagnostics, 0);
        assert_eq!(
            options.unwrap().lang.as_deref(),
            Some([Box::from("ts")].as_slice())
        );
    }
}
