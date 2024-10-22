use crate::{DeserializationContext, DeserializationDiagnostic};
use biome_console::markup;
use biome_rowan::TextRange;

/// Trait that should be implemented on types that use the
/// `#[deserializable(with_validator)]` annotation with the
/// [biome_deserialize_macros::Deserializable] derive macro.
pub trait DeserializableValidator {
    /// Validates the deserialized instance.
    ///
    /// May generate any kind of diagnostics.
    ///
    /// Returns `true` if the instance passes validation and `false` when it
    /// should be rejected.
    fn validate(
        &mut self,
        ctx: &mut impl DeserializationContext,
        name: &str,
        range: TextRange,
    ) -> bool;
}

/// Validates whether the given value is non-empty.
pub fn non_empty<T: IsEmpty>(
    ctx: &mut impl DeserializationContext,
    value: &T,
    name: &str,
    range: TextRange,
) -> bool {
    if value.is_empty() {
        ctx.report(
            DeserializationDiagnostic::new(markup! {
                <Emphasis>{name}</Emphasis>" may not be empty"
            })
            .with_range(range),
        );
        false
    } else {
        true
    }
}

pub trait IsEmpty {
    fn is_empty(&self) -> bool;
}

impl IsEmpty for String {
    fn is_empty(&self) -> bool {
        String::is_empty(self)
    }
}

impl IsEmpty for Box<str> {
    fn is_empty(&self) -> bool {
        str::is_empty(self)
    }
}

impl<T> IsEmpty for Vec<T> {
    fn is_empty(&self) -> bool {
        Vec::is_empty(self)
    }
}

impl<T> IsEmpty for Box<[T]> {
    fn is_empty(&self) -> bool {
        <[_]>::is_empty(self)
    }
}
