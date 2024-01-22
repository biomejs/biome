//! `biome_deserialize` consists of data structures that know how to deserialize themselves
//! along with data formats that know how to deserialize data.
//! It provides a framework by which these two groups interact with each other,
//! allowing any supported data structure to be deserialized using any supported data format.
//!
//! This crate inspired by [serde](https://serde.rs/).
//! 0ne of the main difference is the fault-tolerant behavior of `biome_deserialize`.
//! _Serde_ uses a fast-fail strategy, while `biome_deserialize` deserialize as much as possible
//! and report several diagnostics (errors, warning, deprecation messages, ...).
//! Also, `biome_deserialize` is intended to deserialize textual data formats.
//!
//! `biome_deserialize` assumes that every supported data formats supports the following types:
//!
//! - null-like values;
//! - boolean;
//! - number -- integers and floats;
//! - string;
//! - array;
//! - maps of key-value pairs (covers objects).
//!
//! It currently supports the JSON data format. See [biome_deserialize::json] for more details.
//!
//! The two most important traits are [Deserializable] and [DeserializableValue].
//!
//! - A type that implements `Deserializable` is a data structure that can be
//!   deserialized from any supported data format;
//! - A type that implements `DeserializableValue` is a data format that can
//!   deserialize any supported data structure.
//!
//! You can find a guide and more examples in the README.
//!
mod diagnostics;
mod impls;
pub mod json;
mod merge;
pub mod string_set;
mod validator;

use biome_diagnostics::{Error, Severity};
pub use biome_rowan::TextRange;
pub use diagnostics::{DeserializationAdvice, DeserializationDiagnostic, VisitableType};
pub use impls::*;
pub use merge::Merge;
use std::fmt::Debug;
pub use string_set::StringSet;
pub use validator::*;

/// Implemented by data structures that can deserialize any [DeserializableValue].
///
/// `biome_deserialize` provides [Deserializable] implementations for common Rust types.
/// To implement [Deserializable], you can reuse a type that implements [Deserializable] and
/// turn the obtained value into what you want.
///
/// When deserializing more complex types, such as a `struct` or `enum`, you can use the
/// [Deserializable] derive macro.
///
/// ## Example
///
/// ```
/// use biome_deserialize_macros::Deserializable;
/// use biome_rowan::TextRange;
///
/// #[derive(Deserializable)]
/// pub enum Variant {
///     A,
///     B,
/// }
///
/// use biome_deserialize::json::deserialize_from_json_str;
/// use biome_json_parser::JsonParserOptions;
///
/// let source = r#""a""#;
/// let deserialized = deserialize_from_json_str::<Variant>(&source, JsonParserOptions::default(), "");
/// assert!(!deserialized.has_errors());
/// assert!(matches!(deserialized.into_deserialized(), Some(Variant::A)));
/// ```
pub trait Deserializable: Sized {
    /// Returns the deserialized form of `value`, or `None` if it failed.
    /// Any diagnostics emitted during deserialization are appended to `diagnostics`.
    /// `name` corresponds to the name used in a diagnostic to designate the deserialized value.
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self>;
}

/// Implemented by data structure that can be deserialized.
///
/// This trait should only be implemented when adding the support for a new data format.
/// See [biome_deserialize::json] for an example of an implementation.
pub trait DeserializableValue: Sized {
    /// Range in the source content of this value
    fn range(&self) -> TextRange;

    /// Returns the deserialized form of this value using `visitor`.
    /// Any diagnostics emitted during deserialization are appended to `diagnostics`.
    /// `name` corresponds to the name used in a diagnostic to designate the value.
    fn deserialize<V: DeserializationVisitor>(
        &self,
        visitor: V,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<V::Output>;

    /// Returns whether the value is of the given type.
    fn is_type(&self, ty: VisitableType) -> bool;
}

/// This trait represents a visitor that walks through a [DeserializableValue].
///
/// We assume that a deserializable value has one of the following type:
///
/// - null / none;
/// - boolean;
/// - number -- an integer or a float;
/// - string;
/// - array;
/// - map (key-value pairs with value's type that can depend on its key).
///
/// Every type is associated to a `visit_` method.
/// [DeserializableValue::deserialize] calls the `viist_` method that matches the type of the value.
///
/// Most of the time you should implement [Deserializable] and rely on existing deserializable types.
/// You should use a visitor only when you deserialize a `struct` or a union of several types.
///
/// ## Examples
///
/// ```
/// use biome_deserialize::{DeserializationDiagnostic, Deserializable, DeserializableValue, DeserializationVisitor, Text, VisitableType};
/// use biome_rowan::TextRange;
///
/// #[derive(Debug, Eq, PartialEq)]
/// struct Person {
///     name: String
/// }
///
/// impl Deserializable for Person {
///     fn deserialize(
///         value: &impl DeserializableValue,
///         name: &str,
///         diagnostics: &mut Vec<DeserializationDiagnostic>,
///     ) -> Option<Self> {
///         value.deserialize(PersonVisitor, name, diagnostics)
///     }
/// }
///
/// struct PersonVisitor;
/// impl DeserializationVisitor for PersonVisitor {
///     type Output = Person;
///     const EXPECTED_TYPE: VisitableType = VisitableType::MAP;
///
///     fn visit_map(
///         self,
///         members: impl Iterator<Item = Option<(impl DeserializableValue, impl DeserializableValue)>>,
///         range: TextRange,
///         _name: &str,
///         diagnostics: &mut Vec<DeserializationDiagnostic>,
///     ) -> Option<Self::Output> {
///         let mut name = None;
///         for (key, value) in members.flatten() {
///             let Some(key_text) = Text::deserialize(&key, "", diagnostics) else {
///                 continue;
///             };
///             match key_text.text() {
///                 "name" => {
///                     name = Deserializable::deserialize(&value, &key_text, diagnostics);
///                 },
///                 unknown_key => {
///                     const ALLOWED_KEYS: &[&str] = &["name"];
///                     diagnostics.push(DeserializationDiagnostic::new_unknown_key(
///                         unknown_key,
///                         key.range(),
///                         ALLOWED_KEYS,
///                     ));
///                 }
///             }
///         }
///         Some(Person { name: name? })
///     }
/// }
///
/// use biome_deserialize::json::deserialize_from_json_str;
/// use biome_json_parser::JsonParserOptions;
///
/// let source = r#"{ "name": "Isaac Asimov" }"#;
/// let deserialized = deserialize_from_json_str::<Person>(&source, JsonParserOptions::default(), "");
/// assert!(!deserialized.has_errors());
/// assert_eq!(deserialized.into_deserialized(), Some(Person { name: "Isaac Asimov".to_string() }));
/// ```
///
/// ```
/// use biome_deserialize::{DeserializationDiagnostic, Deserializable, DeserializableValue, DeserializationVisitor, Text, VisitableType};
/// use biome_rowan::TextRange;
///
/// #[derive(Debug, Eq, PartialEq)]
/// enum Union {
///     Bool(bool),
///     Str(String),
/// }
///
/// impl Deserializable for Union {
///     fn deserialize(
///         value: &impl DeserializableValue,
///         name: &str,
///         diagnostics: &mut Vec<DeserializationDiagnostic>,
///     ) -> Option<Self> {
///         value.deserialize(UnionVisitor, name, diagnostics)
///     }
/// }
///
/// struct UnionVisitor;
/// impl DeserializationVisitor for UnionVisitor {
///     type Output = Union;
///     const EXPECTED_TYPE: VisitableType = VisitableType::BOOL.union(VisitableType::STR);
///
///     fn visit_bool(
///         self,
///         value: bool,
///         range: TextRange,
///         _name: &str,
///         diagnostics: &mut Vec<DeserializationDiagnostic>,
///     ) -> Option<Self::Output> {
///         Some(Union::Bool(value))
///     }
///
///     fn visit_str(
///         self,
///         value: Text,
///         range: TextRange,
///         _name: &str,
///         diagnostics: &mut Vec<DeserializationDiagnostic>,
///     ) -> Option<Self::Output> {
///         Some(Union::Str(value.text().to_string()))
///     }
/// }
///
/// use biome_deserialize::json::deserialize_from_json_str;
/// use biome_json_parser::JsonParserOptions;
///
/// let source = r#" "string" "#;
/// let deserialized = deserialize_from_json_str::<Union>(&source, JsonParserOptions::default(), "");
/// assert!(!deserialized.has_errors());
/// assert_eq!(deserialized.into_deserialized(), Some(Union::Str("string".to_string())));
///
/// let source = "true";
/// let deserialized = deserialize_from_json_str::<Union>(&source, JsonParserOptions::default(), "");
/// assert!(!deserialized.has_errors());
/// assert_eq!(deserialized.into_deserialized(), Some(Union::Bool(true)));
/// ```
pub trait DeserializationVisitor: Sized {
    /// The type of the deserialized form of the visited value.
    type Output;

    /// The expected type of the visited value.
    const EXPECTED_TYPE: VisitableType;

    /// The visited value is `null`.
    ///
    /// The default implementation appends an incorrect type diagnostic to `diagnostics`.
    /// The expected type is retrieved from [Self::EXPECTED_TYPE].
    fn visit_null(
        self,
        range: TextRange,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        debug_assert!(
            !Self::EXPECTED_TYPE.contains(VisitableType::NULL),
            "This method should be implemented because the expected type is null."
        );
        diagnostics.push(DeserializationDiagnostic::new_incorrect_type_with_name(
            VisitableType::NULL,
            Self::EXPECTED_TYPE,
            name,
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
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        debug_assert!(
            !Self::EXPECTED_TYPE.contains(VisitableType::BOOL),
            "This method should be implemented because the expected type is bool."
        );
        diagnostics.push(DeserializationDiagnostic::new_incorrect_type_with_name(
            VisitableType::BOOL,
            Self::EXPECTED_TYPE,
            name,
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
        _value: TextNumber,
        range: TextRange,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        debug_assert!(
            !Self::EXPECTED_TYPE.contains(VisitableType::NUMBER),
            "This method should be implemented because the expected type is number."
        );
        diagnostics.push(DeserializationDiagnostic::new_incorrect_type_with_name(
            VisitableType::NUMBER,
            Self::EXPECTED_TYPE,
            name,
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
        _value: Text,
        range: TextRange,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        debug_assert!(
            !Self::EXPECTED_TYPE.contains(VisitableType::STR),
            "This method should be implemented because the expected type is str."
        );
        diagnostics.push(DeserializationDiagnostic::new_incorrect_type_with_name(
            VisitableType::STR,
            Self::EXPECTED_TYPE,
            name,
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
        _items: impl Iterator<Item = Option<impl DeserializableValue>>,
        range: TextRange,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        debug_assert!(
            !Self::EXPECTED_TYPE.contains(VisitableType::ARRAY),
            "This method should be implemented because the expected type is array."
        );
        diagnostics.push(DeserializationDiagnostic::new_incorrect_type_with_name(
            VisitableType::ARRAY,
            Self::EXPECTED_TYPE,
            name,
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
        _members: impl Iterator<Item = Option<(impl DeserializableValue, impl DeserializableValue)>>,
        range: TextRange,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        debug_assert!(
            !Self::EXPECTED_TYPE.contains(VisitableType::MAP),
            "This method should be implemented because the expected type is map."
        );
        diagnostics.push(DeserializationDiagnostic::new_incorrect_type_with_name(
            VisitableType::MAP,
            Self::EXPECTED_TYPE,
            name,
            range,
        ));
        None
    }
}

/// A small type to interrogate the result of a JSON deserialization
#[derive(Debug, Default)]
pub struct Deserialized<T> {
    /// Diagnostics emitted during the parsing and deserialization phase
    diagnostics: Vec<Error>,
    /// The deserialized result, or `None` if the deserialization failed
    deserialized: Option<T>,
}

impl<T> Deserialized<T> {
    /// Consumes self to return the diagnostics
    pub fn into_diagnostics(self) -> Vec<Error> {
        self.diagnostics
    }

    pub fn diagnostics(&self) -> &[Error] {
        self.diagnostics.as_slice()
    }

    /// The deserialized result, or `None` if the deserialization failed.
    pub fn into_deserialized(self) -> Option<T> {
        self.deserialized
    }

    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| d.severity() == Severity::Error)
    }

    pub fn has_warnings(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| d.severity() == Severity::Warning)
    }

    /// Consume itself to return the deserialized result and its diagnostics.
    pub fn consume(self) -> (Option<T>, Vec<Error>) {
        (self.deserialized, self.diagnostics)
    }
}
