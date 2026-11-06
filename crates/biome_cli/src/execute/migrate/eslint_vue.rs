/// Configuration related to the
/// [Vue ESLint plugin](https://eslint.vuejs.org/).
///
/// Also, the module includes implementation to convert rule options to Biome's rule options.
use super::eslint_eslint::OneOrMany;
use super::eslint_to_biome::RestrictedElements;
use biome_deserialize::{
    Deserializable, DeserializableType, DeserializableValue, DeserializationContext,
};
use biome_deserialize_macros::Deserializable;
use biome_rule_options::no_vue_undeclared_properties::NoVueUndeclaredPropertiesOptions;
use biome_rule_options::use_consistent_block_lang::{
    BlockLangMap, BlockLangOptions as BlockLangBlockOptions, UseConsistentBlockLangOptions,
};
use std::collections::BTreeMap;

/// An option of the [vue/no-restricted-html-elements](https://eslint.vuejs.org/rules/no-restricted-html-elements) rule.
///
/// The rule takes any number of these options.
#[derive(Debug)]
pub(crate) enum RestrictedHtmlElement {
    Plain(Box<str>),
    WithMessage(RestrictedHtmlElementWithMessage),
}
impl RestrictedHtmlElement {
    /// Returns the restricted elements of `options` with their optional messages.
    ///
    /// When an element is listed more than once, the plugin uses the first entry.
    pub(crate) fn into_restricted_elements(options: Vec<Self>) -> RestrictedElements {
        let mut elements = RestrictedElements::default();
        for option in options {
            match option {
                Self::Plain(element) => {
                    elements.entry(element).or_insert(None);
                }
                Self::WithMessage(RestrictedHtmlElementWithMessage { element, message }) => {
                    let names = match element {
                        ElementNames::One(name) => vec![name],
                        ElementNames::Many(names) => names.into_vec(),
                    };
                    for name in names {
                        elements.entry(name).or_insert_with(|| message.clone());
                    }
                }
            }
        }
        elements
    }
}
impl Deserializable for RestrictedHtmlElement {
    fn deserialize<V: DeserializableValue>(
        ctx: &mut dyn DeserializationContext<State = V::State>,
        value: &V,
        name: &str,
    ) -> Option<Self> {
        if value.visitable_type(ctx)? == DeserializableType::Str {
            Deserializable::deserialize(ctx, value, name).map(Self::Plain)
        } else {
            Deserializable::deserialize(ctx, value, name).map(Self::WithMessage)
        }
    }
}

#[derive(Debug, Default, Deserializable)]
pub(crate) struct RestrictedHtmlElementWithMessage {
    #[deserializable(required)]
    element: ElementNames,
    message: Option<Box<str>>,
}

/// A single element name or a list of element names.
#[derive(Debug)]
pub(crate) enum ElementNames {
    One(Box<str>),
    Many(Box<[Box<str>]>),
}
impl Default for ElementNames {
    fn default() -> Self {
        Self::Many(Box::default())
    }
}
impl Deserializable for ElementNames {
    fn deserialize<V: DeserializableValue>(
        ctx: &mut dyn DeserializationContext<State = V::State>,
        value: &V,
        name: &str,
    ) -> Option<Self> {
        if value.visitable_type(ctx)? == DeserializableType::Str {
            Deserializable::deserialize(ctx, value, name).map(Self::One)
        } else {
            Deserializable::deserialize(ctx, value, name).map(Self::Many)
        }
    }
}

/// Options of [`vue/block-lang`](https://eslint.vuejs.org/rules/block-lang), keyed by block name.
#[derive(Debug, Deserializable)]
pub(crate) struct BlockLangOptions(BTreeMap<Box<str>, BlockOptions>);
#[derive(Debug, Default, Deserializable)]
pub(crate) struct BlockOptions {
    lang: Option<OneOrMany<Box<str>>>,
    allow_no_lang: Option<bool>,
}
impl BlockLangOptions {
    /// Returns `None` when every block only allows omitting `lang`, which
    /// Biome doesn't enforce.
    pub(crate) fn into_biome_options(self) -> Option<UseConsistentBlockLangOptions> {
        let blocks: BlockLangMap = self
            .0
            .into_iter()
            .filter_map(|(name, block)| {
                let default_langs: &[&str] = match name.as_ref() {
                    "template" => &["html"],
                    "style" => &["css"],
                    "script" => &["js", "javascript"],
                    _ => &[],
                };
                // eslint-plugin-vue reads a block's default language, such as
                // `js` for `<script>`, as "omit `lang`", and reports it when it's
                // written explicitly.
                let (defaults, langs): (Vec<_>, Vec<_>) = block
                    .lang
                    .map(|langs| langs.0)
                    .unwrap_or_default()
                    .into_iter()
                    .partition(|lang| default_langs.contains(&lang.as_ref()));
                let options = if !langs.is_empty() {
                    let allow_no_lang = !defaults.is_empty() || block.allow_no_lang == Some(true);
                    BlockLangBlockOptions {
                        lang: Some(langs.into()),
                        allow_no_lang: allow_no_lang.then_some(true),
                        required: None,
                    }
                } else if !defaults.is_empty() {
                    // Only default languages are listed. Biome can't enforce omitting
                    // `lang`, so it allows the default languages as well as no `lang`,
                    // which stops reporting an explicit `lang="js"`.
                    BlockLangBlockOptions {
                        lang: Some(defaults.into()),
                        allow_no_lang: Some(true),
                        required: None,
                    }
                } else {
                    // Biome doesn't enforce omitting `lang`, so the block isn't migrated.
                    return None;
                };
                Some((name, options))
            })
            .collect();
        (!blocks.is_empty()).then_some(UseConsistentBlockLangOptions {
            blocks: Some(blocks),
        })
    }
}

/// Options of the [vue/no-undef-properties](https://eslint.vuejs.org/rules/no-undef-properties) rule.
#[derive(Debug, Default, Deserializable)]
pub(crate) struct NoUndefPropertiesOptions {
    /// Property names, or regular expressions written as `/pattern/flags`.
    ignores: Box<[Box<str>]>,
}
impl NoUndefPropertiesOptions {
    /// Returns the Biome options that keep the plain names of `ignores`.
    ///
    /// Biome doesn't support regular expressions, so they are dropped. The
    /// plugin's default, `/^\$/`, needs no migration, because Biome never
    /// reports names that start with `$`.
    pub(crate) fn into_biome_options(self) -> Option<NoVueUndeclaredPropertiesOptions> {
        let names: Box<[Box<str>]> = self
            .ignores
            .into_iter()
            .filter(|name| !is_regex_pattern(name))
            .collect();
        (!names.is_empty()).then_some(NoVueUndeclaredPropertiesOptions {
            ignore: Some(names),
        })
    }
}

/// Returns `true` when eslint-plugin-vue reads `pattern` as a regular
/// expression: text between slashes, followed by optional flags.
fn is_regex_pattern(pattern: &str) -> bool {
    pattern
        .strip_prefix('/')
        .and_then(|rest| rest.rsplit_once('/'))
        .is_some_and(|(_, flags)| flags.bytes().all(|flag| flag.is_ascii_alphabetic()))
}
