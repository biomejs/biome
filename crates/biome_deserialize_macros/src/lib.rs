mod deserializable_derive;

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
/// ```
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
/// through attributes:
///
/// ```
/// #[derive(Default, Deserializable)]
/// struct StructWithFields {
///     // Deserialization of `foo` is rejected if an empty string is provided.
///     // This is supported on any type that has an `is_empty()` method.
///     #[deserializable(disallow_empty)]
///     foo: String,
/// }
///
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
/// ```
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
/// variants may also be adjusted through attributes:
///
/// ```
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
/// ```
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
