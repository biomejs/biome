//! `biome_deserialize` provides a framework to deserialize textual data format.
//!
//! This is inspired by serde.
//! 0ne of the main difference is the fault-tolerant behavior of `biome_deserialize`.
//! Serde uses a fast-fail strategy, while `biome_deserialize` deserialize as much as possible
//! and report several diagnostics (errors, warning, deprecation messages, ...).
//!
//! The two most important traits are [Deserializable] and [DeserializableValue].
//!
//!  - A type that implements `Deserializable` is a data structure that can be
//!    deserialized from any supported data format
//!  - A type that implements `DeserializableValue` is a data format that can
//!    deserialize any supported data structure.
mod diagnostics;
mod impls;
pub mod json;
pub mod string_set;
use biome_diagnostics::{Error, Severity};
use biome_rowan::{TextRange, TokenText};
pub use diagnostics::{DeserializationAdvice, DeserializationDiagnostic, ExpectedType};
pub use impls::*;
use std::fmt::Debug;
pub use string_set::StringSet;

/// Implemented by data structures that can deserialize any [DeserializableValue].
///
/// `biome_deserialize` provides [Deserializable] implementations for common Rust types.
/// To implement [Deserializable], you can reuse a type that implements [Deserializable] and
/// turn the obtained value into what you want.
///
/// When deserializing more complex types, such as `struct`,
/// you have to use a type that implements [DeserializationVisitor].
///
/// ### Example
///
/// ```
/// use biome_deserialize::{DeserializationDiagnostic,  Deserializable, DeserializableValue, DeserializationVisitor, ExpectedType};
/// use biome_deserialize::json::deserialize_from_json_str;
/// use biome_rowan::{TextRange, TokenText};
///
/// pub enum Variant {
///     A,
///     B,
/// }
///
/// impl Deserializable for Variant {
///     fn deserialize(
///         value: impl DeserializableValue,
///         diagnostics: &mut Vec<DeserializationDiagnostic>,
///     ) -> Option<Self> {
///         const ALLOWED_VARIANTS: &[&str] = &["A", "B", "C"];
///         let range = value.range();
///         let value = TokenText::deserialize(value, diagnostics)?;
///         match value.text() {
///             "A" => Some(Variant::A),
///             "B" => Some(Variant::B),
///             _ => {
///                 diagnostics.push(DeserializationDiagnostic::new_unknown_value(
///                     value.text(),
///                     range,
///                     ALLOWED_VARIANTS,
///                 ));
///                 None
///             }
///         }
///     }
/// }
/// ```
pub trait Deserializable: Sized {
    /// Returns the deserialized form of `value`, or `None` if it failed.
    /// Any diagnostics emitted during deserialization are appended to `diagnostics`.
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self>;
}

/// Implemented by data structure that can be deserialized.
///
/// This trait should only be implemented when adding the support for a new data format.
/// See [biome_deserialize::json] for an example of implementation.
pub trait DeserializableValue: Sized {
    /// Range in the source content of this value
    fn range(&self) -> TextRange;

    /// Returns the deserialized form of this value using `visitor`.
    /// Any diagnostics emitted during deserialization are appended to `diagnostics`.
    fn deserialize<V: DeserializationVisitor>(
        self,
        visitor: V,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<V::Output>;
}

/// This trait represents a visitor that walks through a [DeserializableValue].
///
/// We assume that a deserializable value has one of the following type:
/// - null / none
/// - boolean
/// - number (integer, floats)
/// - string
/// - array
/// - map (key-value pairs with value's type that can depend on its key)
///
/// Every type is associated to a `visit_` method.
/// [DeserializableValue::deserialize] calls the `viist_` method that matches the type of the value.
///
/// Most of the time you should implement [Deserializable] and rely on existing deserializable types.
/// You should use a visitor only when you deserialize a map or a union of several types.
///
/// ### Examples
///
/// ```
/// use biome_deserialize::{DeserializationDiagnostic,  Deserializable, DeserializableValue, DeserializationVisitor, ExpectedType};
/// use biome_deserialize::json::deserialize_from_json_str;
/// use biome_rowan::{TextRange, TokenText};
///
/// struct Person {
///     name: String
/// }
///
/// impl Deserializable for Person {
///     fn deserialize(
///         value: impl DeserializableValue,
///         diagnostics: &mut Vec<DeserializationDiagnostic>,
///     ) -> Option<Self> {
///         value.deserialize(PersonVisitor, diagnostics)
///     }
/// }
///
/// struct PersonVisitor;
/// impl DeserializationVisitor for PersonVisitor {
///     type Output = Person;
///     const EXPECTED_TYPE: ExpectedType = ExpectedType::MAP;
///
///     fn visit_map(
///         self,
///         members: impl Iterator<Item = (impl DeserializableValue, impl DeserializableValue)>,
///         range: TextRange,
///         diagnostics: &mut Vec<DeserializationDiagnostic>,
///     ) -> Option<Self::Output> {
///         const ALLOWED_KEYS: &[&str] = &["name"];
///         let mut name = None;
///         for (key, value) in members {
///             let key_range = key.range();
///             let Some(key) = TokenText::deserialize(key, diagnostics) else {
///                 continue;
///             };
///             match key.text() {
///                 "name" => {
///                     name = Deserializable::deserialize(value, diagnostics);
///                 },
///                 _ => diagnostics.push(DeserializationDiagnostic::new_unknown_key(
///                     key.text(),
///                     key_range,
///                     ALLOWED_KEYS,
///                 )),
///             }
///         }
///         Some(Person { name: name? })
///     }
/// }
///
/// enum Union {
///     Bool(bool),
///     Str(String),
/// }
///
/// impl Deserializable for Union {
///     fn deserialize(
///         value: impl DeserializableValue,
///         diagnostics: &mut Vec<DeserializationDiagnostic>,
///     ) -> Option<Self> {
///         value.deserialize(UnionVisitor, diagnostics)
///     }
/// }
///
/// struct UnionVisitor;
/// impl DeserializationVisitor for UnionVisitor {
///     type Output = Union;
///     const EXPECTED_TYPE: ExpectedType = ExpectedType::BOOL.union(ExpectedType::STR);
///
///     fn visit_bool(
///         self,
///         value: bool,
///         range: TextRange,
///         diagnostics: &mut Vec<DeserializationDiagnostic>,
///     ) -> Option<Self::Output> {
///         Some(Union::Bool(value))
///     }
///
///     fn visit_str(
///         self,
///         value: TokenText,
///         range: TextRange,
///         diagnostics: &mut Vec<DeserializationDiagnostic>,
///     ) -> Option<Self::Output> {
///         Some(Union::Str(value.text().to_string()))
///     }
/// }
/// ```
pub trait DeserializationVisitor: Sized {
    /// The type of the deserialized form of the visited value.
    type Output;

    /// The expected type of the visited value.
    const EXPECTED_TYPE: ExpectedType;

    /// The visited value is `null`.
    ///
    /// The default implementation appends an incorrect type diagnostic to `diagnostics`.
    /// The expected type is retrieved from [Self::EXPECTED_TYPE].
    fn visit_null(
        self,
        range: TextRange,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        debug_assert!(
            !Self::EXPECTED_TYPE.contains(ExpectedType::NULL),
            "This method should be implemented because the type is expected."
        );
        diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
            Self::EXPECTED_TYPE,
            range,
        ));
        None
    }

    /// The visited value is a `bool`.
    ///
    /// The default implementation appends an incorrect type diagnostic to `diagnostics`.
    /// The expected type is retrieved from [Self::EXPECTED_TYPE].
    fn visit_bool(
        self,
        _value: bool,
        range: TextRange,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        debug_assert!(
            !Self::EXPECTED_TYPE.contains(ExpectedType::BOOL),
            "This method should be implemented because the type is expected."
        );
        diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
            Self::EXPECTED_TYPE,
            range,
        ));
        None
    }

    /// The visited value is a number (integer or float).
    /// The number is represented by a string.
    ///
    /// The default implementation appends an incorrect type diagnostic to `diagnostics`.
    /// The expected type is retrieved from [Self::EXPECTED_TYPE].
    fn visit_number(
        self,
        _value: TokenNumber,
        range: TextRange,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        debug_assert!(
            !Self::EXPECTED_TYPE.contains(ExpectedType::NUMBER),
            "This method should be implemented because the type is expected."
        );
        diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
            Self::EXPECTED_TYPE,
            range,
        ));
        None
    }

    /// The visited value is a `string`.
    ///
    /// The default implementation appends an incorrect type diagnostic to `diagnostics`.
    /// The expected type is retrieved from [Self::EXPECTED_TYPE].
    fn visit_str(
        self,
        _value: TokenText,
        range: TextRange,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        debug_assert!(
            !Self::EXPECTED_TYPE.contains(ExpectedType::STR),
            "This method should be implemented because the type is expected."
        );
        diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
            Self::EXPECTED_TYPE,
            range,
        ));
        None
    }

    /// The visited value is an array-like (array, list, vector) structure.
    ///
    /// The default implementation appends an incorrect type diagnostic to `diagnostics`.
    /// The expected type is retrieved from [Self::EXPECTED_TYPE].
    fn visit_array(
        self,
        _items: impl Iterator<Item = impl DeserializableValue>,
        range: TextRange,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        debug_assert!(
            !Self::EXPECTED_TYPE.contains(ExpectedType::ARRAY),
            "This method should be implemented because the type is expected."
        );
        diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
            Self::EXPECTED_TYPE,
            range,
        ));
        None
    }

    /// The visited value is a `map` (key-value pairs).
    ///
    /// The default implementation appends an incorrect type diagnostic to `diagnostics`.
    /// The expected type is retrieved from [Self::EXPECTED_TYPE].
    fn visit_map(
        self,
        _members: impl Iterator<Item = (impl DeserializableValue, impl DeserializableValue)>,
        range: TextRange,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        debug_assert!(
            !Self::EXPECTED_TYPE.contains(ExpectedType::MAP),
            "This method should be implemented because the type is expected."
        );
        diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
            Self::EXPECTED_TYPE,
            range,
        ));
        None
    }
}

/// A small type to interrogate the result of a JSON deserialization
#[derive(Debug, Default)]
pub struct Deserialized<P> {
    diagnostics: Vec<Error>,
    deserialized: Option<P>,
}

impl<P> Deserialized<P> {
    /// Consumes self to return the diagnostics
    pub fn into_diagnostics(self) -> Vec<Error> {
        self.diagnostics
    }

    pub fn diagnostics(&self) -> &[Error] {
        self.diagnostics.as_slice()
    }

    /// Consumes self and returns the deserialized result
    pub fn into_deserialized(self) -> Option<P> {
        self.deserialized
    }

    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| d.severity() == Severity::Error)
    }

    /// Consume itself to return the parsed result and its diagnostics
    pub fn consume(self) -> (Option<P>, Vec<Error>) {
        (self.deserialized, self.diagnostics)
    }
}
