#![deny(clippy::use_self)]

mod deserializable_derive;
mod merge_derive;
mod util;

use proc_macro::TokenStream;
use proc_macro_error2::*;
use syn::{DeriveInput, parse_macro_input};

/// Derives the [biome_deserialize::Deserializable] trait for a custom enum or
/// struct.
///
/// It supports implementing the trait for the following data structures:
/// - Structs with named fields where every field is of type `T` or `Option<T>`
///   and where `T` also implements [biome_deserialize::Deserializable]. The
///   struct must also implement `Default`.
/// - Structs with a single unnamed field (a so-called
///   [newtype](https://doc.rust-lang.org/rust-by-example/generics/new_types.html)).
/// - Enums, so long as the variants don't also have fields.
///
/// ## Examples
///
/// ```no_test
/// #[derive(Default, Deserializable)]
/// struct StructWithNamedFields {
///     foo: String,
///     bar: Option<String>,
/// }
///
/// #[derive(Deserializable)]
/// struct NewType(u16);
///
/// #[derive(Deserializable)]
/// enum EnumWithPlainVariants {
///     Variant1,
///     Variant2
/// }
/// ```
///
/// ## Container attributes
///
/// When [Deserializable] is derived on a struct or  an enum,
/// its behavior may be adjusted through attributes.
///
/// ### `unknown_fields`
///
/// This attribute allows controlling how to handle unknown fields in structs.
/// It takes one of the following values:
///
/// - `"deny"`: emit an error when an unknown struct field is found.
/// - `"warn"`: emit a warning when an unknown struct field is found.
/// - `"allow"`: ignore unknown struct fields.
///
/// The default behavior is `"warn"`, when the attribute is not specified.
///
/// For structs that also implement Serde's [serde::Deserialize],
/// it automatically picks up on Serde's
/// [`deny_unknown_fields` attribute](https://serde.rs/container-attrs.html#from).
/// `serde(deny_unknown_fields)` is mapped to `unknown_fields = "error"`.
/// `deserializable(unknown_fields = _)` takes precedence over `serde(deny_unknown_fields = _)`.
///
/// ```no_test
/// #[derive(Default, Deserializable)]
/// #[deserializable(unknown_fields = "deny")]
/// struct Contact {
///     fullname: String,
/// }
///
/// #[derive(Default, Deserializable)]
/// #[deserializable(unknown_fields = "allow")]
/// struct Person {
///     firstnames: String,
///     lastname: String,
/// }
/// ```
///
/// ### `with_validator`
///
/// When the `with_validator` attribute is present, the deserializable type is
/// expected to implement the `DeserializableValidator` trait. The generated
/// deserializer will call its `validate()` method and reject the instance if
/// it returns `false`.
///
/// ```no_test
/// #[derive(Default, Deserializable)]
/// #[deserializable(with_validator)]
/// struct ValidatedStruct {
///     // In real code, please use the `validate` attribute instead (see below)!
///     non_empty: String,
/// }
///
/// impl DeserializableValidator for ValidatedStruct {
///     fn validate(
///         &self,
///         name: &str,
///         range: biome_rowan::TextRange,
///         diagnostics: &mut Vec<crate::DeserializationDiagnostic>,
///     ) -> bool {
///         if self.non_empty.is_empty() {
///             diagnostics.push(
///                 DeserializationDiagnostic::new(markup! {
///                     <Emphasis>"foo"</Emphasis>" may not be empty"
///                 })
///                 .with_range(range),
///             );
///             false
///         } else {
///             true
///         }
///     }
/// }
/// ```
///
/// ### `from`
///
/// This attribute allows deserializing a type,
/// and then converting it to the current annotated type.
/// The annotated type must implement the standard [From] trait.
///
/// For structs and enums that also implement Serde's [serde::Deserialize],
/// it automatically picks up on Serde's
/// [`from` attribute](https://serde.rs/container-attrs.html#from).
/// `deserializable(from = _)` takes precedence over `serde(from = _)`.
///
/// ```no_test
/// #[derive(Default, Deserializable)]
/// #[deserializable(from = Person)]
/// struct Contact {
///     fullname: String,
/// }
///
/// #[derive(Default, Deserializable)]
/// struct Person {
///     firstnames: String,
///     lastname: String,
/// }
///
/// impl From<Person> for Contact {
///     fn from(value: Person) -> Contact {
///         Contact {
///             fullname: format!("{} {}", value.firstnames, value.lastname),
///         }
///     }
/// }
/// ```
///
/// ### `try_from`
///
/// This attribute allows deserializing a type,
/// and then attempting to convert it to the current annotated type.
/// The annotated type must implement the standard [TryFrom] trait.
///
/// For structs and enums that also implement Serde's [serde::Deserialize],
/// it automatically picks up on Serde's
/// [`try_from` attribute](https://serde.rs/container-attrs.html#try_from).
/// `deserializable(try_from = _)` takes precedence over `serde(try_from = _)`.
///
/// ```no_test
/// #[derive(Default, Deserializable)]
/// #[deserializable(try_from = Contact)]
/// struct Person {
///     firstnames: String,
///     lastname: String,
/// }
///
/// #[derive(Default, Deserializable)]
/// struct Contact {
///     fullname: String,
/// }
///
/// impl TryFrom<Contact> for Person {
///     Error = &'static str;
///
///     fn from(value: Contact) -> Person {
///         let names: Vec<&str> = value.fullname.splitn(' ', 2).collect();
///         if names.len() < 2  {
///             return Err("At least two names separated by a whitespace are required.")
///         }
///         Person {
///             firstnames: names[..names.len()-1].join(' ').to_string(),
///             lastname: names[names.len()-1].to_string(),
///         }
///     }
/// }
/// ```
///
/// ## Struct field attributes
///
/// A struct's fields may also be adjusted through attributes.
///
/// ### `bail_on_error`
///
/// If present, bails on deserializing the entire struct if validation for this
/// field fails.
///
/// Note the struct may still be deserialized if the field is not present in the
/// serialized representation at all. In that case `Default::default()` will be
/// filled in. If this is not what you want, you probably want to use the
/// `required` attribute instead.
///
/// ```no_test
/// #[derive(Default, Deserializable)]
/// pub struct ReactHook {
///     #[deserializable(bail_on_error)]
///     pub name: String,
/// }
/// ```
///
/// ### `deprecated`
///
/// Used to generate diagnostics when a deprecated field is deserialized. Note
/// this does not alter the behavior of the deserializer, so you still need to
/// account for the field's value after deserialization.
///
/// ```no_test
/// #[derive(Default, Deserializable)]
/// pub struct ReactHook {
///     #[deserializable(deprecated(use_instead = "name"))]
///     pub old_name: String,
///
///     #[deserializable(deprecated(message = "nick names are not used anymore"))]
///     pub nick_name: String,
///
///     pub name: String,
/// }
/// ```
///
/// ### `rename`
///
/// ```no_test
/// #[derive(Default, Deserializable)]
/// struct StructWithFields {
///     // Use "$schema" as serialized property, instead of "schema". By
///     // default, field names use "camelCase".
///     #[deserializable(rename = "$schema")]
///     schema: Option<String>,
/// }
/// ```
///
/// For structs that also implement Serde's `Serialize` or `Deserialize`,
/// it automatically picks up on Serde's
/// [`rename` attribute](https://serde.rs/field-attrs.html#rename).
/// `deserializable(rename = _)` takes precedence over `serde(rename = _)`.
///
/// ```no_test
/// #[derive(Default, Deserialize, Deserializable, Serialize)]
/// struct StructWithFields {
///     // This also works:
///     #[serde(rename = "$schema")]
///     schema: Option<String>,
/// }
/// ```
///
/// ### `required`
///
/// If present, presence of this field is required for successful
/// deserialization of the struct.
///
/// Note this does not check whether the value is meaningful. For instance, an
/// empty string would still be accepted. For such use case, you may also want
/// to use the `validate` attribute.
///
/// Implies `bail_on_error`.
///
/// ```no_test
/// #[derive(Default, Deserializable)]
/// pub struct StructWithRequiredName {
///     #[deserializable(required)]
///     pub name: String,
/// }
/// ```
/// ### `rest`
///
/// If present, puts the remaining fields found in the serialized representation
/// into this field. The field must implement `Extend<K, V>` where `K` is the
/// `Text` type and `V` implements `Deserializable`.
///
/// Cannot be used with the container attribute `#[deserializable(unknown_fields = deny)]`
///
/// ```no_test
/// #[derive(Default, Deserializable)]
/// pub struct StructWithRest {
///     pub foo: String,
///     #[deserializable(rest)]
///     pub extra: HashMap<Text, serde_json::Value>,
/// }
/// ```
///
/// ### `validate`
///
/// Like the `with_validator` annotation, this allows the invocation of a
/// validation function that can emit diagnostics and reject instances. But
/// unlike `with_validator`, this attribute takes an argument that allows you
/// to specify the validation function.
///
/// Note the validator is not invoked if the field is not present in the
/// serialized representation at all. To cover this, you may also want to use
/// the `required` attribute.
///
/// ```no_test
/// #[derive(Default, Deserializable)]
/// struct ValidatedStruct {
///     #[deserializable(required, validate = "biome_deserialize::non_empty")]
///     non_empty: String,
/// }
/// ```
///
/// ## Enum variant attributes
///
/// When [Deserializable] is derived on a enum, the deserialization of its
/// variants may also be adjusted through attributes.
///
/// ### `rename`
///
/// ```no_test
/// #[derive(Deserializable)]
/// enum Cases {
///     // Serialized representation defaults to "camelCase", which is what we
///     // want here.
///     CamelCase,
///     // The following attribute provides a custom serialization:
///     #[deserializable(rename = "kebab-case")]
///     KebabCase,
/// }
/// ```
///
/// Using Serde's [rename attribute](https://serde.rs/variant-attrs.html#rename)
/// is supported on enums too.
#[proc_macro_derive(Deserializable, attributes(deserializable))]
#[proc_macro_error]
pub fn derive_deserializable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let input = deserializable_derive::DeriveInput::parse(input);

    let tokens = deserializable_derive::generate_deserializable(input);

    if false {
        panic!("{tokens}");
    }

    TokenStream::from(tokens)
}

/// Derives the [biome_deserialize::Merge] trait for a custom enum or
/// struct.
#[proc_macro_derive(Merge)]
#[proc_macro_error]
pub fn derive_mergeable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let input = merge_derive::DeriveInput::parse(input);

    let tokens = merge_derive::generate_merge(input);

    if false {
        panic!("{tokens}");
    }

    TokenStream::from(tokens)
}
