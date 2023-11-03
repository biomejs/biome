use crate::{DeserializationDiagnostic, Deserialized, VisitNode};
use biome_console::markup;
use biome_diagnostics::{DiagnosticExt, Error};
use biome_json_parser::{parse_json, JsonParserOptions};
use biome_json_syntax::{
    AnyJsonValue, JsonArrayValue, JsonBooleanValue, JsonLanguage, JsonMemberName, JsonNumberValue,
    JsonObjectValue, JsonRoot, JsonStringValue, JsonSyntaxNode,
};
use biome_rowan::{AstNode, AstSeparatedList, TextRange};
use indexmap::IndexSet;
use std::num::ParseIntError;

/// Main trait to
///
pub trait JsonDeserialize: Sized {
    /// It accepts a JSON AST and a visitor. The visitor is the [default](Default) implementation of the data
    /// type that implements this trait.
    fn deserialize_from_ast(
        root: &JsonRoot,
        visitor: &mut impl VisitJsonNode,
        deserialize_diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()>;
}

impl JsonDeserialize for () {
    fn deserialize_from_ast(
        _root: &JsonRoot,
        _visitor: &mut impl VisitJsonNode,
        _deserialize_diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        Some(())
    }
}

/// Convenient trait that contains utility functions to work with [JsonLanguage]
pub trait VisitJsonNode: VisitNode<JsonLanguage> {
    /// Convenient function to use inside [visit_map].
    ///
    /// It casts `key` to [JsonMemberName] and the `value` to [AnyJsonValue].
    fn get_key_and_value(
        &mut self,
        key: &JsonSyntaxNode,
        value: &JsonSyntaxNode,
    ) -> Option<(JsonMemberName, AnyJsonValue)> {
        Some((
            JsonMemberName::cast_ref(key)?,
            AnyJsonValue::cast_ref(value)?,
        ))
    }

    /// It attempts to map a [AnyJsonValue] to a string.
    ///
    /// Use this function when you want to map a string to an enum type.
    ///
    /// ## Errors
    ///
    /// The function will emit a generic diagnostic if the `visitor` doesn't implement [visit_value]
    fn map_to_known_string(
        &mut self,
        value: &AnyJsonValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        if JsonStringValue::can_cast(value.syntax().kind()) {
            self.visit_value(value.syntax(), diagnostics)?;
            return Some(());
        }
        diagnostics.push(DeserializationDiagnostic::new_incorrect_type_for_value(
            name,
            "string",
            value.range(),
        ));
        None
    }

    /// It attempts to map a [AnyJsonValue] to a [String].
    ///
    /// ## Errors
    ///
    /// It emits an error if `value` can't be cast to a [JsonStringValue]
    fn map_to_string(
        &self,
        value: &AnyJsonValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<String> {
        JsonStringValue::cast_ref(value.syntax())
            .and_then(|node| Some(node.inner_string_text().ok()?.to_string()))
            .or_else(|| {
                diagnostics.push(DeserializationDiagnostic::new_incorrect_type_for_value(
                    name,
                    "string",
                    value.range(),
                ));
                None
            })
    }

    /// It attempts to map a [AnyJsonValue] to a [u8].
    ///
    /// ## Errors
    ///
    /// It will fail if:
    /// - `value` can't be cast to [JsonNumberValue]
    /// - the value of the node can't be parsed to [u8]
    fn map_to_u8(
        &self,
        value: &AnyJsonValue,
        name: &str,
        maximum: u8,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<u8> {
        let value = JsonNumberValue::cast_ref(value.syntax()).or_else(|| {
            diagnostics.push(DeserializationDiagnostic::new_incorrect_type_for_value(
                name,
                "number",
                value.range(),
            ));
            None
        })?;
        let value = value.value_token().ok()?;
        let result = value.text_trimmed().parse::<u8>().map_err(|err| {
            emit_diagnostic_form_number(
                err,
                value.text_trimmed(),
                value.text_trimmed_range(),
                maximum,
            )
        });
        match result {
            Ok(number) => Some(number),
            Err(err) => {
                diagnostics.push(err);
                None
            }
        }
    }

    /// It attempts to map a [AnyJsonValue] to a [usize].
    ///
    /// ## Errors
    ///
    /// It will fail if:
    /// - `value` can't be cast to [JsonNumberValue]
    /// - the value of the node can't be parsed to [usize]
    fn map_to_usize(
        &self,
        value: &AnyJsonValue,
        name: &str,
        maximum: usize,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<usize> {
        let value = JsonNumberValue::cast_ref(value.syntax()).or_else(|| {
            diagnostics.push(DeserializationDiagnostic::new_incorrect_type_for_value(
                name,
                "number",
                value.range(),
            ));
            None
        })?;
        let value = value.value_token().ok()?;
        let result = value.text_trimmed().parse::<usize>().map_err(|err| {
            emit_diagnostic_form_number(
                err,
                value.text_trimmed(),
                value.text_trimmed_range(),
                maximum,
            )
        });
        match result {
            Ok(number) => Some(number),
            Err(err) => {
                diagnostics.push(err);
                None
            }
        }
    }

    /// It attempts to map a [AnyJsonValue] to a [u16].
    ///
    /// ## Errors
    ///
    /// It will fail if:
    /// - `value` can't be cast to [JsonNumberValue]
    /// - the value of the node can't be parsed to [u16]
    fn map_to_u16(
        &self,
        value: &AnyJsonValue,
        name: &str,
        maximum: u16,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<u16> {
        let value = JsonNumberValue::cast_ref(value.syntax())
            .ok_or_else(|| {
                DeserializationDiagnostic::new_incorrect_type_for_value(
                    name,
                    "number",
                    value.range(),
                )
            })
            .ok()?;
        let value = value.value_token().ok()?;

        let result = value.text_trimmed().parse::<u16>().map_err(|err| {
            emit_diagnostic_form_number(
                err,
                value.text_trimmed(),
                value.text_trimmed_range(),
                maximum,
            )
        });
        match result {
            Ok(number) => Some(number),
            Err(err) => {
                diagnostics.push(err);
                None
            }
        }
    }

    /// It attempts to map a [AnyJsonValue] to a [u64].
    ///
    /// ## Errors
    ///
    /// It will fail if:
    /// - `value` can't be cast to [JsonNumberValue]
    /// - the value of the node can't be parsed to [u64]
    fn map_to_u64(
        &self,
        value: &AnyJsonValue,
        name: &str,
        maximum: u64,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<u64> {
        let value = JsonNumberValue::cast_ref(value.syntax()).or_else(|| {
            diagnostics.push(DeserializationDiagnostic::new_incorrect_type_for_value(
                name,
                "number",
                value.range(),
            ));
            None
        })?;
        let value = value.value_token().ok()?;

        let result = value.text_trimmed().parse::<u64>().map_err(|err| {
            emit_diagnostic_form_number(
                err,
                value.text_trimmed(),
                value.text_trimmed_range(),
                maximum,
            )
        });

        match result {
            Ok(number) => Some(number),
            Err(err) => {
                diagnostics.push(err);
                None
            }
        }
    }

    /// It attempts to cast [AnyJsonValue] to a [bool]
    ///
    /// ## Errors
    ///
    /// The function emits a diagnostic if `value` can't be cast to [JsonBooleanValue]
    fn map_to_boolean(
        &self,
        value: &AnyJsonValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<bool> {
        JsonBooleanValue::cast_ref(value.syntax())
            .and_then(|value| Some(value.value_token().ok()?.text_trimmed() == "true"))
            .or_else(|| {
                diagnostics.push(DeserializationDiagnostic::new_incorrect_type_for_value(
                    name,
                    "boolean",
                    value.range(),
                ));

                None
            })
    }

    /// It attempts to map a [AnyJsonValue] to a [IndexSet] of [String].
    ///
    /// ## Errors
    ///
    /// The function emit diagnostics if:
    /// - `value` can't be cast to [JsonArrayValue]
    /// - any element of the of the array can't be cast to [JsonStringValue]
    fn map_to_index_set_string(
        &self,
        value: &AnyJsonValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<IndexSet<String>> {
        let array = JsonArrayValue::cast_ref(value.syntax()).or_else(|| {
            diagnostics.push(DeserializationDiagnostic::new_incorrect_type_for_value(
                name,
                "array",
                value.range(),
            ));
            None
        })?;
        let mut elements = IndexSet::new();
        if array.elements().is_empty() {
            return None;
        }
        for element in array.elements() {
            let element = element.ok()?;
            match element {
                AnyJsonValue::JsonStringValue(value) => {
                    elements.insert(value.inner_string_text().ok()?.to_string());
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "string",
                        element.range(),
                    ));
                }
            }
        }

        Some(elements)
    }

    /// It attempts to map a [AnyJsonValue] to a [Vec] of [String].
    ///
    /// ## Errors
    ///
    /// The function emit diagnostics if:
    /// - `value` can't be cast to [JsonArrayValue]
    /// - any element of the of the array can't be cast to [JsonStringValue]
    fn map_to_array_of_strings(
        &self,
        value: &AnyJsonValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Vec<String>> {
        let array = JsonArrayValue::cast_ref(value.syntax()).or_else(|| {
            diagnostics.push(DeserializationDiagnostic::new_incorrect_type_for_value(
                name,
                "array",
                value.range(),
            ));
            None
        })?;
        let mut elements = Vec::new();
        if array.elements().is_empty() {
            return None;
        }
        for element in array.elements() {
            let element = element.ok()?;
            match element {
                AnyJsonValue::JsonStringValue(value) => {
                    elements.push(value.inner_string_text().ok()?.to_string());
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_incorrect_type(
                        "string",
                        element.range(),
                    ));
                }
            }
        }

        Some(elements)
    }

    /// It attempts to map [AnyJsonValue] to a generic map.
    ///
    /// Use this function when the value of your member is another object, and this object
    /// needs to be mapped to another type.
    ///
    /// This function will loop though the list of elements and call [visit_map] on each pair
    /// of `name` and `value`.
    ///
    /// ## Errors
    /// This function will emit diagnostics if:
    /// - the `value` can't be cast to [JsonObjectValue]
    fn map_to_object(
        &mut self,
        value: &AnyJsonValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let value = JsonObjectValue::cast_ref(value.syntax()).or_else(|| {
            diagnostics.push(DeserializationDiagnostic::new_incorrect_type_for_value(
                name,
                "object",
                value.range(),
            ));
            None
        })?;
        for element in value.json_member_list() {
            let element = element.ok()?;
            self.visit_map(
                element.name().ok()?.syntax(),
                element.value().ok()?.syntax(),
                diagnostics,
            )?;
        }
        Some(())
    }

    fn map_to_array(
        &mut self,
        value: &AnyJsonValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let array = JsonArrayValue::cast_ref(value.syntax()).or_else(|| {
            diagnostics.push(DeserializationDiagnostic::new_incorrect_type_for_value(
                name,
                "array",
                value.range(),
            ));
            None
        })?;
        if array.elements().is_empty() {
            return None;
        }
        for element in array.elements() {
            let element = element.ok()?;
            self.visit_array_member(element.syntax(), diagnostics);
        }

        Some(())
    }
}

impl<V: VisitNode<JsonLanguage>> VisitJsonNode for V {}

fn emit_diagnostic_form_number(
    parse_error: ParseIntError,
    value_text: &str,
    value_range: TextRange,
    maximum: impl biome_console::fmt::Display,
) -> DeserializationDiagnostic {
    let diagnostic =
        DeserializationDiagnostic::new(parse_error.to_string()).with_range(value_range);
    if value_text.starts_with('-') {
        diagnostic.with_note(markup! {"Value can't be negative"})
    } else {
        diagnostic.with_note(markup! {"Maximum value accepted is "{{maximum}}})
    }
}

/// Convenient function to report that `key` is not allowed and
/// to suggest a key in `allowed_keys`.
pub fn report_unknown_map_key(
    key: &JsonMemberName,
    allowed_keys: &[&str],
    diagnostics: &mut Vec<DeserializationDiagnostic>,
) {
    if let Ok(name) = key.inner_string_text() {
        diagnostics.push(DeserializationDiagnostic::new_unknown_key(
            name.text(),
            key.range(),
            allowed_keys,
        ))
    }
}

/// Convenient function to report that `variant` is not allowed and
/// to suggest a variant in `allowed_variant`.
pub fn report_unknown_variant(
    variant: &JsonStringValue,
    allowed_variants: &[&str],
    diagnostics: &mut Vec<DeserializationDiagnostic>,
) {
    if let Ok(value) = variant.inner_string_text() {
        diagnostics.push(DeserializationDiagnostic::new_unknown_value(
            value.text(),
            variant.range(),
            allowed_variants,
        ));
    }
}

/// It attempts to parse and deserialize a source file in JSON. Diagnostics from the parse phase
/// are consumed and joined with the diagnostics emitted during the deserialization.
///
/// The data structure that needs to be deserialized needs to implement three important traits:
/// - [Default], to create a first instance of the data structure;
/// - [JsonDeserialize], a trait to begin the deserialization from JSON AST;
/// - [VisitNode], to visit values inside a JSON file;
/// - [VisitJsonNode], to inherit a series of useful functions to handle specifically
/// JSON values;
///
/// ## Examples
///
/// ```
/// use biome_deserialize::{DeserializationDiagnostic,  VisitNode, Deserialized};
/// use biome_deserialize::json::deserialize_from_json_str;
/// use biome_deserialize::json::{report_unknown_map_key, JsonDeserialize, VisitJsonNode};
/// use biome_json_syntax::{JsonLanguage, JsonSyntaxNode};
/// use biome_json_syntax::JsonRoot;
/// use biome_rowan::AstNode;
///
/// #[derive(Default, Debug, Eq, PartialEq)]
/// struct NewConfiguration {
///     lorem: bool
/// }
///
/// impl VisitNode<JsonLanguage> for NewConfiguration {
///     fn visit_map(&mut self, key: &JsonSyntaxNode, value: &JsonSyntaxNode, diagnostics: &mut Vec<DeserializationDiagnostic>) -> Option<()> {
///         let (key, value) = self.get_key_and_value(key, value)?;
///         let name_text = key.inner_string_text().ok()?;
///         let name_text = name_text.text();
///         match name_text {
///             "lorem" => {
///                 self.lorem = self.map_to_boolean(&value, name_text, diagnostics)?
///             }
///             _ => {
///                 report_unknown_map_key(&key, &["lorem"], diagnostics);
///             }
///         }
///         Some(())
///     }
/// }
///
/// impl NewConfiguration {
///     fn parse(root: JsonRoot) -> Deserialized<Self> {
///         use biome_deserialize::Deserialized;
///         let mut output = Self::default();
///         let mut diagnostics = vec![];
///         NewConfiguration::deserialize_from_ast(&root, &mut output, &mut diagnostics);
///         Deserialized::new(output, diagnostics)
///     }
/// }
///
///
/// impl JsonDeserialize for NewConfiguration {
///     fn deserialize_from_ast(root: &JsonRoot, visitor: &mut impl VisitJsonNode, diagnostics: &mut Vec<DeserializationDiagnostic>) -> Option<()> {
///         let object = root.value().ok()?;
///         let object = object.as_json_object_value()?;
///         for member in object.json_member_list() {
///             let member = member.ok()?;
///             visitor.visit_map(member.name().ok()?.syntax(), member.value().ok()?.syntax(), diagnostics)?;
///         }
///         Some(())
///     }
/// }
///
///  use biome_json_parser::JsonParserOptions;
/// # fn main() -> Result<(), DeserializationDiagnostic> {
/// let source = r#"{ "lorem": true }"#;
///  let deserialized = deserialize_from_json_str::<NewConfiguration>(&source, JsonParserOptions::default());
///  assert!(!deserialized.has_errors());
///  assert_eq!(deserialized.into_deserialized(), NewConfiguration { lorem: true });
/// # Ok(())
/// # }
///
///
/// ```
pub fn deserialize_from_json_str<Output>(
    source: &str,
    options: JsonParserOptions,
) -> Deserialized<Output>
where
    Output: Default + VisitJsonNode + JsonDeserialize,
{
    let mut output = Output::default();
    let mut diagnostics = vec![];
    let parse = parse_json(source, options);

    Output::deserialize_from_ast(&parse.tree(), &mut output, &mut diagnostics);
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
        deserialized: output,
    }
}

/// Attempts to deserialize a JSON AST, given the `Output`.
pub fn deserialize_from_json_ast<Output>(parse: &JsonRoot) -> Deserialized<Output>
where
    Output: Default + VisitJsonNode + JsonDeserialize,
{
    let mut output = Output::default();
    let mut diagnostics = vec![];
    Output::deserialize_from_ast(parse, &mut output, &mut diagnostics);
    Deserialized {
        diagnostics: diagnostics.into_iter().map(Error::from).collect::<Vec<_>>(),
        deserialized: output,
    }
}

/// Attempts to deserialize a JSON AST, given the `Output`.
pub fn deserialize_from_json_root<Output>(parse: &JsonRoot) -> Deserialized<Output>
where
    Output: Default + VisitJsonNode + JsonDeserialize,
{
    let mut output = Output::default();
    let mut diagnostics = vec![];
    Output::deserialize_from_ast(parse, &mut output, &mut diagnostics);
    Deserialized {
        diagnostics: diagnostics.into_iter().map(Error::from).collect::<Vec<_>>(),
        deserialized: output,
    }
}
