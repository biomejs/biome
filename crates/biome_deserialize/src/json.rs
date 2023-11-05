use crate::{
    Deserializable, DeserializableValue, DeserializationDiagnostic, DeserializationVisitor,
    Deserialized, TokenNumber,
};
use biome_diagnostics::{DiagnosticExt, Error};
use biome_json_parser::{parse_json, JsonParserOptions};
use biome_json_syntax::{AnyJsonValue, JsonMemberName, JsonRoot, T};
use biome_rowan::{AstNode, AstSeparatedList};

/// It attempts to parse and deserialize a source file in JSON. Diagnostics from the parse phase
/// are consumed and joined with the diagnostics emitted during the deserialization.
///
/// The data structures that need to be deserialized have to implement the [Deserializable] trait.
/// To implement [Deserializable], it can need to implement [DeserializationVisitor] that allows
/// visiting a value.
///
/// ## Examples
///
/// ```
/// use biome_deserialize::{DeserializationDiagnostic,  Deserializable, DeserializableValue, DeserializationVisitor, ExpectedType};
/// use biome_deserialize::json::deserialize_from_json_str;
/// use biome_rowan::{TextRange, TokenText};
///
/// #[derive(Default, Debug, Eq, PartialEq)]
/// struct NewConfiguration {
///     lorem: String
/// }
///
/// impl Deserializable for NewConfiguration {
///     fn deserialize(
///         value: impl DeserializableValue,
///         diagnostics: &mut Vec<DeserializationDiagnostic>,
///     ) -> Option<Self> {
///         value.deserialize(Visitor, diagnostics)
///     }
/// }
///
/// struct Visitor;
/// impl DeserializationVisitor for Visitor {
///     type Output = NewConfiguration;
///
///     const EXPECTED_TYPE: ExpectedType = ExpectedType::MAP;
///
///     fn visit_map(
///         self,
///         members: impl Iterator<Item = (impl DeserializableValue, impl DeserializableValue)>,
///         _range: TextRange,
///         diagnostics: &mut Vec<DeserializationDiagnostic>,
///     ) -> Option<Self::Output> {
///         const ALLOWED_KEYS: &[&str] = &["strictCase", "enumMemberCase"];
///         let mut result = NewConfiguration::default();
///         for (key, value) in members {
///             let key_range = key.range();
///             let Some(key) = TokenText::deserialize(key, diagnostics) else {
///                 continue;
///             };
///             match key.text() {
///                 "lorem" => {
///                     if let Some(value) = Deserializable::deserialize(value, diagnostics) {
///                         result.lorem = value;
///                     }
///                 },
///                 _ => diagnostics.push(DeserializationDiagnostic::new_unknown_key(
///                     key.text(),
///                     key_range,
///                     ALLOWED_KEYS,
///                 )),
///             }
///         }
///         Some(result)
///     }
/// }
///
///  use biome_json_parser::JsonParserOptions;
/// let source = r#"{ "lorem": "ipsum" }"#;
/// let deserialized = deserialize_from_json_str::<NewConfiguration>(&source, JsonParserOptions::default());
/// assert!(!deserialized.has_errors());
/// assert_eq!(deserialized.into_deserialized().unwrap(), NewConfiguration { lorem: "ipsum".to_string() });
/// ```
pub fn deserialize_from_json_str<Output: Deserializable>(
    source: &str,
    options: JsonParserOptions,
) -> Deserialized<Output> {
    let parse = parse_json(source, options);
    let Deserialized {
        diagnostics,
        deserialized,
    } = deserialize_from_json_ast::<Output>(&parse.tree());
    let mut errors = parse
        .into_diagnostics()
        .into_iter()
        .map(Error::from)
        .collect::<Vec<_>>();
    errors.extend(
        diagnostics
            .into_iter()
            .map(|diagnostic| diagnostic.with_file_source_code(source))
            .collect::<Vec<_>>(),
    );
    Deserialized {
        diagnostics: errors,
        deserialized,
    }
}

/// Attempts to deserialize a JSON AST, given the `Output`.
pub fn deserialize_from_json_ast<Output: Deserializable>(parse: &JsonRoot) -> Deserialized<Output> {
    let mut diagnostics = vec![];
    let deserialized = parse
        .value()
        .ok()
        .and_then(|value| Output::deserialize(value, &mut diagnostics));
    Deserialized {
        diagnostics: diagnostics.into_iter().map(Error::from).collect::<Vec<_>>(),
        deserialized,
    }
}

impl DeserializableValue for AnyJsonValue {
    fn range(&self) -> biome_rowan::TextRange {
        AstNode::range(self)
    }

    fn deserialize<V: DeserializationVisitor>(
        self,
        visitor: V,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<V::Output> {
        let range = AstNode::range(&self);
        match self {
            AnyJsonValue::JsonArrayValue(array) => {
                let items = array.elements().iter().filter_map(|x| x.ok());
                visitor.visit_array(items, range, diagnostics)
            }
            AnyJsonValue::JsonBogusValue(_) => {
                // The parser should emit an error about this node
                // No need to emit another diagnostic.
                None
            }
            AnyJsonValue::JsonBooleanValue(value) => {
                let value = value.value_token().ok()?;
                visitor.visit_bool(value.kind() == T![true], range, diagnostics)
            }
            AnyJsonValue::JsonNullValue(_) => visitor.visit_null(range, diagnostics),
            AnyJsonValue::JsonNumberValue(value) => {
                let value = value.value_token().ok()?;
                let token_text = value.token_text_trimmed();
                visitor.visit_number(TokenNumber(token_text), range, diagnostics)
            }
            AnyJsonValue::JsonObjectValue(object) => {
                let members = object
                    .json_member_list()
                    .iter()
                    .filter_map(|x| x.ok())
                    .filter_map(|x| Some((x.name().ok()?, x.value().ok()?)));
                visitor.visit_map(members, range, diagnostics)
            }
            AnyJsonValue::JsonStringValue(value) => {
                let value = value.inner_string_text().ok()?;
                visitor.visit_str(value, range, diagnostics)
            }
        }
    }
}

impl DeserializableValue for JsonMemberName {
    fn range(&self) -> biome_rowan::TextRange {
        AstNode::range(self)
    }

    fn deserialize<V: DeserializationVisitor>(
        self,
        visitor: V,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<V::Output> {
        let range = AstNode::range(&self);
        let value = self.inner_string_text().ok()?;
        visitor.visit_str(value, range, diagnostics)
    }
}
