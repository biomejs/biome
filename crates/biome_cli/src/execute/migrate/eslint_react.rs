/// Configuration related to the
/// [React ESLint plugin](https://github.com/jsx-eslint/eslint-plugin-react).
///
/// Also, the module includes implementation to convert rule options to Biome's rule options.
use super::eslint_to_biome::RestrictedElements;
use biome_deserialize::{
    Deserializable, DeserializableType, DeserializableValue, DeserializationContext,
};
use biome_deserialize_macros::Deserializable;

/// Options for the [react/forbid-elements](https://github.com/jsx-eslint/eslint-plugin-react/blob/master/docs/rules/forbid-elements.md) rule.
#[derive(Debug, Default, Deserializable)]
pub(crate) struct ForbidElementsOptions {
    forbid: Box<[ForbiddenElement]>,
}
impl ForbidElementsOptions {
    /// Returns the forbidden elements with their optional messages.
    ///
    /// When an element is listed more than once, the plugin uses the last entry.
    pub(crate) fn into_restricted_elements(self) -> RestrictedElements {
        let mut elements = RestrictedElements::default();
        for forbidden in self.forbid {
            let (element, message) = match forbidden {
                ForbiddenElement::Plain(element) => (element, None),
                ForbiddenElement::WithMessage(ElementWithMessage { element, message }) => {
                    (element, message)
                }
            };
            elements.insert(element, message);
        }
        elements
    }
}

#[derive(Debug)]
pub(crate) enum ForbiddenElement {
    Plain(Box<str>),
    WithMessage(ElementWithMessage),
}
impl Deserializable for ForbiddenElement {
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
pub(crate) struct ElementWithMessage {
    #[deserializable(required)]
    element: Box<str>,
    message: Option<Box<str>>,
}
