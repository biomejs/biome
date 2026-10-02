/// Configuration related to the
/// [Vue ESLint plugin](https://eslint.vuejs.org/).
///
/// Also, the module includes implementation to convert rule options to Biome's rule options.
use super::eslint_to_biome::RestrictedElements;
use biome_deserialize::{
    Deserializable, DeserializableType, DeserializableValue, DeserializationContext,
};
use biome_deserialize_macros::Deserializable;

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
    fn deserialize(
        ctx: &mut dyn DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        if value.visitable_type()? == DeserializableType::Str {
            Deserializable::deserialize(ctx, value, name).map(Self::One)
        } else {
            Deserializable::deserialize(ctx, value, name).map(Self::Many)
        }
    }
}
