/// Configuration related to the
/// [Svelte ESLint plugin](https://sveltejs.github.io/eslint-plugin-svelte/).
///
/// Also, the module includes implementation to convert rule options to Biome's rule options.
use super::eslint_eslint::OneOrMany;
use super::eslint_to_biome::RestrictedElements;
use biome_deserialize::{
    Deserializable, DeserializableType, DeserializableValue, DeserializationContext,
};
use biome_deserialize_macros::Deserializable;
use biome_rule_options::use_consistent_block_lang::{
    BlockLangMap, BlockLangOptions as BlockLangBlockOptions, UseConsistentBlockLangOptions,
};

/// An option of the [svelte/no-restricted-html-elements](https://sveltejs.github.io/eslint-plugin-svelte/rules/no-restricted-html-elements/) rule.
///
/// The rule takes any number of these options.
#[derive(Debug)]
pub(crate) enum RestrictedHtmlElement {
    Plain(Box<str>),
    WithMessage(RestrictedHtmlElementsWithMessage),
}
impl RestrictedHtmlElement {
    /// Returns the restricted elements of `options` with their optional messages.
    ///
    /// The plugin reports an element once for every option that lists it.
    /// The first message of an element is kept, and replaces a missing message.
    pub(crate) fn into_restricted_elements(options: Vec<Self>) -> RestrictedElements {
        let mut elements = RestrictedElements::default();
        for option in options {
            let (names, message) = match option {
                Self::Plain(name) => (vec![name], None),
                Self::WithMessage(RestrictedHtmlElementsWithMessage { elements, message }) => {
                    (elements.into_vec(), message)
                }
            };
            for name in names {
                let existing = elements.entry(name).or_insert(None);
                if existing.is_none() {
                    existing.clone_from(&message);
                }
            }
        }
        elements
    }
}
impl Deserializable for RestrictedHtmlElement {
    fn deserialize(
        ctx: &mut dyn DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        if value.visitable_type()? == DeserializableType::Str {
            Deserializable::deserialize(ctx, value, name).map(Self::Plain)
        } else {
            Deserializable::deserialize(ctx, value, name).map(Self::WithMessage)
        }
    }
}

#[derive(Debug, Default, Deserializable)]
pub(crate) struct RestrictedHtmlElementsWithMessage {
    elements: Box<[Box<str>]>,
    message: Option<Box<str>>,
}

/// A string that can be `null`.
///
/// `Option<Box<str>>` fails to deserialize `null`, which would silently drop
/// `null` items from an array such as `["ts", null]`.
#[derive(Debug)]
pub(crate) struct NullableString(Option<Box<str>>);
impl Deserializable for NullableString {
    fn deserialize(
        ctx: &mut dyn DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        if value.visitable_type()? == DeserializableType::Null {
            Some(Self(None))
        } else {
            Deserializable::deserialize(ctx, value, name).map(|text| Self(Some(text)))
        }
    }
}

/// Options of [`svelte/block-lang`](https://sveltejs.github.io/eslint-plugin-svelte/rules/block-lang/).
#[derive(Debug, Default, Deserializable)]
pub(crate) struct BlockLangOptions {
    enforce_script_present: bool,
    enforce_style_present: bool,
    /// Allowed languages, where `null` allows omitting `lang`. Defaults to `null`.
    script: Option<OneOrMany<NullableString>>,
    /// Allowed languages, where `null` allows omitting `lang`. Defaults to `null`.
    style: Option<OneOrMany<NullableString>>,
}
impl BlockLangOptions {
    /// Returns `None` when no block allows a language or is required to be
    /// present, because Biome doesn't enforce omitting `lang`.
    pub(crate) fn into_biome_options(self) -> Option<UseConsistentBlockLangOptions> {
        fn to_block(
            langs: Option<OneOrMany<NullableString>>,
            required: bool,
        ) -> Option<BlockLangBlockOptions> {
            let langs = langs.map_or_else(|| vec![NullableString(None)], |langs| langs.0);
            let allow_no_lang = langs.iter().any(|lang| lang.0.is_none());
            let langs: Box<[Box<str>]> = langs.into_iter().filter_map(|lang| lang.0).collect();
            // Biome doesn't enforce omitting `lang`, so a block that only allows
            // omitting it keeps just its presence check.
            let lang = (!langs.is_empty()).then_some(langs);
            (lang.is_some() || required).then(|| BlockLangBlockOptions {
                allow_no_lang: (allow_no_lang && lang.is_some()).then_some(true),
                lang,
                required: required.then_some(true),
            })
        }
        let blocks: BlockLangMap = [
            ("script", to_block(self.script, self.enforce_script_present)),
            ("style", to_block(self.style, self.enforce_style_present)),
        ]
        .into_iter()
        .filter_map(|(name, block)| Some((name.into(), block?)))
        .collect();
        (!blocks.is_empty()).then_some(UseConsistentBlockLangOptions {
            blocks: Some(blocks),
        })
    }
}
