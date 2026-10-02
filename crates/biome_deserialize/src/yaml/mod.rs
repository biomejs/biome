//! Implementation of [DeserializableValue] for the YAML data format.
//!
//! Plain scalars are resolved with the [core schema](https://yaml.org/spec/1.2.2/#103-core-schema)
//! of YAML 1.2: `null`, `~`, and empty nodes are null, `true` and `false` are booleans,
//! integers (including the `0o` and `0x` forms) and floats (including `.inf` and `.nan`) are
//! numbers, and any other scalar is a string. The scalar tags of the core schema, such as `!!str`,
//! override this resolution.
//!
//! Mapping keys that are scalars are always deserialized as strings, like the member names of
//! JSON objects. Aliases are deserialized as the node of their anchor.
//!
//! Only the first document of a stream is deserialized.
mod anchors;
mod block_scalar;
mod flow_scalar;
mod schema;
#[cfg(test)]
mod tests;
mod value;

use crate::{
    DefaultDeserializationContext, Deserializable, DeserializationContext,
    DeserializationDiagnostic, Deserialized,
};
use anchors::Anchors;
use biome_diagnostics::{DiagnosticExt, Error};
use biome_yaml_parser::parse_yaml;
use biome_yaml_syntax::YamlRoot;
use value::YamlValue;

struct YamlDeserializationContext {
    inner: DefaultDeserializationContext,
    anchors: Anchors,
}

impl DeserializationContext for YamlDeserializationContext {
    fn id(&self) -> Option<&str> {
        self.inner.id()
    }

    fn report(&mut self, diagnostic: DeserializationDiagnostic) {
        self.inner.report(diagnostic);
    }
}

/// It attempts to parse and deserialize a source file in YAML. Diagnostics from the parse phase
/// are consumed and joined with the diagnostics emitted during the deserialization.
///
/// The data structures that need to be deserialized have to implement the [Deserializable] trait.
/// For most data structures, this can be achieved using the
/// [biome_deserialize_macros::Deserializable] derive.
///
/// `id` corresponds to the identifier of the deserialized value.
///
/// ## Examples
///
/// ```
/// use biome_deserialize::yaml::deserialize_from_yaml_str;
/// use biome_deserialize_macros::Deserializable;
///
/// #[derive(Debug, Default, Deserializable, Eq, PartialEq)]
/// struct NewConfiguration {
///     lorem: String
/// }
///
/// let source = "lorem: ipsum";
/// let deserialized = deserialize_from_yaml_str::<NewConfiguration>(source, "");
/// assert!(!deserialized.has_errors());
/// assert_eq!(deserialized.into_deserialized().unwrap(), NewConfiguration { lorem: "ipsum".to_string() });
/// ```
pub fn deserialize_from_yaml_str<Output: Deserializable>(
    source: &str,
    id: &str,
) -> Deserialized<Output> {
    let parse = parse_yaml(source);
    let Deserialized {
        diagnostics,
        deserialized,
    } = deserialize_from_yaml_ast::<Output>(&parse.tree(), id);
    let errors = parse
        .into_diagnostics()
        .into_iter()
        .map(Error::from)
        .chain(diagnostics)
        .map(|diagnostic| diagnostic.with_file_source_code(source))
        .collect::<Vec<_>>();
    Deserialized {
        diagnostics: errors,
        deserialized,
    }
}

/// Attempts to deserialize a YAML AST, given the `Output`.
///
/// `id` corresponds to the identifier of the deserialized value.
pub fn deserialize_from_yaml_ast<Output: Deserializable>(
    root: &YamlRoot,
    id: &str,
) -> Deserialized<Output> {
    let Some((value, document)) = YamlValue::root(root) else {
        // The parser already reported the bogus document
        return Deserialized::new(None, Vec::new());
    };
    let mut ctx = YamlDeserializationContext {
        inner: DefaultDeserializationContext::new(id),
        anchors: Anchors::new(document),
    };
    let deserialized = Output::deserialize(&mut ctx, &value, "");
    Deserialized {
        diagnostics: ctx.inner.diagnostics,
        deserialized,
    }
}
