mod deserializable_derive;
mod merge_derive;
mod none_state_derive;

use proc_macro::TokenStream;
use proc_macro_error::*;
use syn::{parse_macro_input, DeriveInput};

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
/// ## Struct field attributes
///
/// When [Deserializable] is derived on a struct, its fields may be adjusted
/// through attributes.
///
/// ### `disallow_empty`
///
/// ```no_test
/// #[derive(Default, Deserializable)]
/// struct StructWithFields {
///     // Deserialization of `foo` is rejected if an empty string is provided.
///     // This is supported on any type that has an `is_empty()` method.
///     #[deserializable(disallow_empty)]
///     foo: String,
/// }
/// ```
///
/// ### `passthrough_name`
///
/// A particularly noteworthy attribute is `passthrough_name`. Every
/// deserializer receives a `name` argument which is used to add context to
/// diagnostics reported when deserializing the value. Typically, when the value
/// of an object property is deserialized, the `name` refers to the name of the
/// property. In some cases however, you may want to pass through the name given
/// to the object's deserializer instead, in order to prevent losing more
/// "distant" context.
///
/// An example is the `RuleWithOptions` struct, where the name of the rule is
/// passed through to the deserializer for `PossibleOptions`. Without this
/// attribute, the name given to the field's deserializer would always be
/// "options".
///
/// ```no_test
/// #[derive(Default, Deserializable)]
/// pub struct RuleWithOptions {
///     pub level: RulePlainConfiguration,
///
///     #[deserializable(passthrough_name)]
///     pub options: Option<PossibleOptions>,
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
/// For structs that also implement Serde's `Serialize` or `Deserialize`, it
/// automatically picks up on Serde's `rename` attribute:
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
/// Using Serde's attributes is supported on enums too.
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

#[proc_macro_derive(NoneState)]
#[proc_macro_error]
pub fn derive_none_state(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let input = none_state_derive::DeriveInput::parse(input);

    let tokens = none_state_derive::generate_none_state(input);

    if false {
        panic!("{tokens}");
    }

    TokenStream::from(tokens)
}
