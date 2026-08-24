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

#![deny(clippy::use_self)]

mod diagnostics;
mod impls;
pub mod json;
mod merge;
mod validator;

use biome_diagnostics::{Error, Severity};
pub use biome_rowan::{Text, TextRange};
pub use diagnostics::{
    DeserializableType, DeserializableTypes, DeserializationAdvice, DeserializationDiagnostic,
};
pub use impls::*;
pub use merge::Merge;
use std::fmt::Debug;
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
    /// Any diagnostics emitted during deserialization are reported via `ctx`.
    /// `name` corresponds to the name used in a diagnostic to designate the deserialized value.
    fn deserialize(
        ctx: &mut dyn DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self>;
}

/// Context used during deserialization.
///
/// We provide a default implementation [DefaultDeserializationContext].
/// Creating a own context implementation allows you to customize how diagnostics are reported.
pub trait DeserializationContext {
    fn id(&self) -> Option<&str>;
    fn report(&mut self, diagnostic: DeserializationDiagnostic);

    /// Enters a named property before its value is deserialized.
    fn enter_property(&mut self, _name: &str, _key_range: TextRange, _value_range: TextRange) {}

    /// Exits the property most recently passed to [`Self::enter_property`].
    fn exit_property(&mut self) {}

    /// Enters an array element before its value is deserialized.
    fn enter_index(&mut self, _index: usize, _range: TextRange) {}

    /// Exits the array element most recently passed to [`Self::enter_index`].
    fn exit_index(&mut self) {}
}

/// Default implementation for [DeserializationContext].
///
/// This implementation stores all reporetd diagnostics inside a vector.
#[derive(Debug, Default)]
pub struct DefaultDeserializationContext<'a> {
    pub diagnostics: Vec<Error>,
    pub id: Option<&'a str>,
}
impl<'a> DefaultDeserializationContext<'a> {
    fn new(id: &'a str) -> Self {
        Self {
            diagnostics: Default::default(),
            id: Some(id),
        }
    }
}
impl DeserializationContext for DefaultDeserializationContext<'_> {
    // Identifier of the deserialized root value.
    fn id(&self) -> Option<&str> {
        self.id
    }

    /// Report `diagnostc` to the user.
    fn report(&mut self, diagnostc: DeserializationDiagnostic) {
        self.diagnostics.push(Error::from(diagnostc));
    }
}

/// Implemented by data structure that can be deserialized.
///
/// This trait should only be implemented when adding the support for a new data format.
/// See [biome_deserialize::json] for an example of an implementation.
pub trait DeserializableValue {
    /// Range in the source content of this value
    fn range(&self) -> TextRange;

    /// Visits this value with a type-erased visitor.
    ///
    /// This is the method data formats implement: because it is not generic,
    /// the traversal of the format's AST is compiled once instead of once per
    /// deserialized type. Use [Self::deserialize] to deserialize a value.
    fn deserialize_erased(
        &self,
        ctx: &mut dyn DeserializationContext,
        visitor: &mut dyn ErasedDeserializationVisitor,
        name: &str,
    );

    /// Returns the deserialized form of this value using `visitor`.
    /// Any diagnostics emitted during deserialization are reported via `ctx`.
    /// `name` corresponds to the name used in a diagnostic to designate the value.
    fn deserialize<V: DeserializationVisitor>(
        &self,
        ctx: &mut dyn DeserializationContext,
        visitor: V,
        name: &str,
    ) -> Option<V::Output>
    where
        Self: Sized,
    {
        let mut erased = ErasedVisitor {
            visitor: Some(visitor),
            output: None,
        };
        self.deserialize_erased(ctx, &mut erased, name);
        erased.output
    }

    /// Returns the type of this value.
    fn visitable_type(&self) -> Option<DeserializableType>;
}

impl DeserializableValue for Box<dyn DeserializableValue> {
    fn range(&self) -> TextRange {
        (**self).range()
    }

    fn deserialize_erased(
        &self,
        ctx: &mut dyn DeserializationContext,
        visitor: &mut dyn ErasedDeserializationVisitor,
        name: &str,
    ) {
        (**self).deserialize_erased(ctx, visitor, name)
    }

    fn visitable_type(&self) -> Option<DeserializableType> {
        (**self).visitable_type()
    }
}

/// Iterator over the key-value pairs of a map, as passed to
/// [DeserializationVisitor::visit_map].
pub type MapMembers<'a> = dyn ExactSizeIterator<Item = Option<(Box<dyn DeserializableValue>, Box<dyn DeserializableValue>)>>
    + 'a;

/// Object-safe counterpart of [DeserializationVisitor], used by
/// [DeserializableValue::deserialize_erased].
///
/// You should never need to implement this trait: every
/// [DeserializationVisitor] is adapted to it automatically by
/// [DeserializableValue::deserialize].
pub trait ErasedDeserializationVisitor {
    /// The visited value is `null`.
    fn visit_null(&mut self, ctx: &mut dyn DeserializationContext, range: TextRange, name: &str);

    /// The visited value is a `bool`.
    fn visit_bool(
        &mut self,
        ctx: &mut dyn DeserializationContext,
        value: bool,
        range: TextRange,
        name: &str,
    );

    /// The visited value is a number (integer or float), represented by a string.
    fn visit_number(
        &mut self,
        ctx: &mut dyn DeserializationContext,
        value: TextNumber,
        range: TextRange,
        name: &str,
    );

    /// The visited value is a `string`.
    fn visit_str(
        &mut self,
        ctx: &mut dyn DeserializationContext,
        value: Text,
        range: TextRange,
        name: &str,
    );

    /// The visited value is an array-like (array, list, vector) structure.
    fn visit_array(
        &mut self,
        ctx: &mut dyn DeserializationContext,
        items: &mut dyn ExactSizeIterator<Item = Option<Box<dyn DeserializableValue>>>,
        range: TextRange,
        name: &str,
    );

    /// The visited value is a `map` (key-value pairs).
    fn visit_map(
        &mut self,
        ctx: &mut dyn DeserializationContext,
        members: &mut MapMembers<'_>,
        range: TextRange,
        name: &str,
    );
}

/// Adapts a [DeserializationVisitor] to [ErasedDeserializationVisitor],
/// carrying the visitor in and its output out of the type-erased traversal.
struct ErasedVisitor<V: DeserializationVisitor> {
    visitor: Option<V>,
    output: Option<V::Output>,
}

impl<V: DeserializationVisitor> ErasedDeserializationVisitor for ErasedVisitor<V> {
    fn visit_null(&mut self, ctx: &mut dyn DeserializationContext, range: TextRange, name: &str) {
        if let Some(visitor) = self.visitor.take() {
            self.output = visitor.visit_null(ctx, range, name);
        }
    }

    fn visit_bool(
        &mut self,
        ctx: &mut dyn DeserializationContext,
        value: bool,
        range: TextRange,
        name: &str,
    ) {
        if let Some(visitor) = self.visitor.take() {
            self.output = visitor.visit_bool(ctx, value, range, name);
        }
    }

    fn visit_number(
        &mut self,
        ctx: &mut dyn DeserializationContext,
        value: TextNumber,
        range: TextRange,
        name: &str,
    ) {
        if let Some(visitor) = self.visitor.take() {
            self.output = visitor.visit_number(ctx, value, range, name);
        }
    }

    fn visit_str(
        &mut self,
        ctx: &mut dyn DeserializationContext,
        value: Text,
        range: TextRange,
        name: &str,
    ) {
        if let Some(visitor) = self.visitor.take() {
            self.output = visitor.visit_str(ctx, value, range, name);
        }
    }

    fn visit_array(
        &mut self,
        ctx: &mut dyn DeserializationContext,
        items: &mut dyn ExactSizeIterator<Item = Option<Box<dyn DeserializableValue>>>,
        range: TextRange,
        name: &str,
    ) {
        if let Some(visitor) = self.visitor.take() {
            self.output = visitor.visit_array(ctx, items, range, name);
        }
    }

    fn visit_map(
        &mut self,
        ctx: &mut dyn DeserializationContext,
        members: &mut MapMembers<'_>,
        range: TextRange,
        name: &str,
    ) {
        if let Some(visitor) = self.visitor.take() {
            self.output = visitor.visit_map(ctx, members, range, name);
        }
    }
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
/// use biome_deserialize::{DeserializationDiagnostic, Deserializable, DeserializationContext, DeserializableValue, DeserializationVisitor, MapMembers, Text, DeserializableTypes};
/// use biome_rowan::TextRange;
///
/// #[derive(Debug, Eq, PartialEq)]
/// struct Person {
///     name: String
/// }
///
/// impl Deserializable for Person {
///     fn deserialize(
///         ctx: &mut dyn DeserializationContext,
///         value: &impl DeserializableValue,
///         name: &str,
///     ) -> Option<Self> {
///         value.deserialize(ctx, PersonVisitor, name)
///     }
/// }
///
/// struct PersonVisitor;
/// impl DeserializationVisitor for PersonVisitor {
///     type Output = Person;
///     const EXPECTED_TYPE: DeserializableTypes = DeserializableTypes::MAP;
///
///     fn visit_map(
///         self,
///         ctx: &mut dyn DeserializationContext,
///         members: &mut MapMembers<'_>,
///         range: TextRange,
///         _name: &str,
///     ) -> Option<Self::Output> {
///         let mut name = None;
///         for (key, value) in members.flatten() {
///             let Some(key_text) = Text::deserialize(ctx, &key, "") else {
///                 continue;
///             };
///             match key_text.text() {
///                 "name" => {
///                     name = Deserializable::deserialize(ctx, &value, &key_text);
///                 },
///                 unknown_key => {
///                     const ALLOWED_KEYS: &[&str] = &["name"];
///                     ctx.report(DeserializationDiagnostic::new_unknown_key(
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
/// use biome_deserialize::{DeserializationDiagnostic, Deserializable, DeserializationContext, DeserializableValue, DeserializationVisitor, MapMembers, Text, DeserializableTypes};
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
///         ctx: &mut dyn DeserializationContext,
///         value: &impl DeserializableValue,
///         name: &str,
///     ) -> Option<Self> {
///         value.deserialize(ctx, UnionVisitor, name)
///     }
/// }
///
/// struct UnionVisitor;
/// impl DeserializationVisitor for UnionVisitor {
///     type Output = Union;
///     const EXPECTED_TYPE: DeserializableTypes = DeserializableTypes::BOOL.union(DeserializableTypes::STR);
///
///     fn visit_bool(
///         self,
///         ctx: &mut dyn DeserializationContext,
///         value: bool,
///         range: TextRange,
///         _name: &str,
///     ) -> Option<Self::Output> {
///         Some(Union::Bool(value))
///     }
///
///     fn visit_str(
///         self,
///         ctx: &mut dyn DeserializationContext,
///         value: Text,
///         range: TextRange,
///         _name: &str,
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
    const EXPECTED_TYPE: DeserializableTypes;

    /// The visited value is `null`.
    ///
    /// The default implementation appends an incorrect type diagnostic to `diagnostics`.
    /// The expected type is retrieved from [Self::EXPECTED_TYPE].
    fn visit_null(
        self,
        ctx: &mut dyn DeserializationContext,
        range: TextRange,
        name: &str,
    ) -> Option<Self::Output> {
        debug_assert!(
            !Self::EXPECTED_TYPE.contains(DeserializableTypes::NULL),
            "This method should be implemented because the expected type is null."
        );
        ctx.report(DeserializationDiagnostic::new_incorrect_type_with_name(
            DeserializableType::Null,
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
        ctx: &mut dyn DeserializationContext,
        _value: bool,
        range: TextRange,
        name: &str,
    ) -> Option<Self::Output> {
        debug_assert!(
            !Self::EXPECTED_TYPE.contains(DeserializableTypes::BOOL),
            "This method should be implemented because the expected type is bool."
        );
        ctx.report(DeserializationDiagnostic::new_incorrect_type_with_name(
            DeserializableType::Bool,
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
        ctx: &mut dyn DeserializationContext,
        _value: TextNumber,
        range: TextRange,
        name: &str,
    ) -> Option<Self::Output> {
        debug_assert!(
            !Self::EXPECTED_TYPE.contains(DeserializableTypes::NUMBER),
            "This method should be implemented because the expected type is number."
        );
        ctx.report(DeserializationDiagnostic::new_incorrect_type_with_name(
            DeserializableType::Number,
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
        ctx: &mut dyn DeserializationContext,
        _value: Text,
        range: TextRange,
        name: &str,
    ) -> Option<Self::Output> {
        debug_assert!(
            !Self::EXPECTED_TYPE.contains(DeserializableTypes::STR),
            "This method should be implemented because the expected type is str."
        );
        ctx.report(DeserializationDiagnostic::new_incorrect_type_with_name(
            DeserializableType::Str,
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
        ctx: &mut dyn DeserializationContext,
        _items: &mut dyn ExactSizeIterator<Item = Option<Box<dyn DeserializableValue>>>,
        range: TextRange,
        name: &str,
    ) -> Option<Self::Output> {
        debug_assert!(
            !Self::EXPECTED_TYPE.contains(DeserializableTypes::ARRAY),
            "This method should be implemented because the expected type is array."
        );
        ctx.report(DeserializationDiagnostic::new_incorrect_type_with_name(
            DeserializableType::Array,
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
        ctx: &mut dyn DeserializationContext,
        _members: &mut MapMembers<'_>,
        range: TextRange,
        name: &str,
    ) -> Option<Self::Output> {
        debug_assert!(
            !Self::EXPECTED_TYPE.contains(DeserializableTypes::MAP),
            "This method should be implemented because the expected type is map."
        );
        ctx.report(DeserializationDiagnostic::new_incorrect_type_with_name(
            DeserializableType::Map,
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
    pub deserialized: Option<T>,
}

impl<T> Deserialized<T> {
    pub fn new(deserialized: Option<T>, diagnostics: Vec<Error>) -> Self {
        Self {
            deserialized,
            diagnostics,
        }
    }

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
