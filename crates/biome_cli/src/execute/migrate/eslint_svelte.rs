/// Configuration related to the
/// [Svelte ESLint plugin](https://sveltejs.github.io/eslint-plugin-svelte/).
///
/// Also, the module includes implementation to convert rule options to Biome's rule options.
use super::eslint_to_biome::RestrictedElements;
use biome_deserialize::{
    Deserializable, DeserializableType, DeserializableValue, DeserializationContext,
};
use biome_deserialize_macros::Deserializable;

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
