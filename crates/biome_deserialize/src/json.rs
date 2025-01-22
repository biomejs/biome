//! Implementation of [DeserializableValue] for the JSON data format.
use crate::{
    diagnostics::DeserializableType, DefaultDeserializationContext, Deserializable,
    DeserializableValue, DeserializationContext, DeserializationVisitor, Deserialized, Text,
    TextNumber,
};
use biome_diagnostics::{DiagnosticExt, Error};
use biome_json_parser::{parse_json, JsonParserOptions};
use biome_json_syntax::{AnyJsonValue, JsonMemberName, JsonRoot, T};
use biome_rowan::{AstNode, AstSeparatedList, TokenText};

/// It attempts to parse and deserialize a source file in JSON. Diagnostics from the parse phase
/// are consumed and joined with the diagnostics emitted during the deserialization.
///
/// The data structures that need to be deserialized have to implement the [Deserializable] trait.
/// For most data structures, this can be achieved using the
/// [biome_deserialize_macros::Deserializable] derive.
///
/// `name` corresponds to the name used in a diagnostic to designate the deserialized value.
///
/// ## Examples
///
/// ```
/// use biome_deserialize::json::deserialize_from_json_str;
/// use biome_deserialize_macros::Deserializable;
/// use biome_json_parser::JsonParserOptions;
///
/// #[derive(Debug, Default, Deserializable, Eq, PartialEq)]
/// struct NewConfiguration {
///     lorem: String
/// }
///
/// let source = r#"{ "lorem": "ipsum" }"#;
/// let deserialized = deserialize_from_json_str::<NewConfiguration>(&source, JsonParserOptions::default(), "");
/// assert!(!deserialized.has_errors());
/// assert_eq!(deserialized.into_deserialized().unwrap(), NewConfiguration { lorem: "ipsum".to_string() });
/// ```
pub fn deserialize_from_json_str<Output: Deserializable>(
    source: &str,
    options: JsonParserOptions,
    id: &str,
) -> Deserialized<Output> {
    let parse = parse_json(source, options);
    let Deserialized {
        diagnostics,
        deserialized,
    } = deserialize_from_json_ast::<Output>(&parse.tree(), id);
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

pub fn deserialize_from_str<Output: Deserializable>(source: &str) -> Deserialized<Output> {
    deserialize_from_json_str(source, JsonParserOptions::default(), "")
}

/// Attempts to deserialize a JSON AST, given the `Output`.
///
/// `id` corresponds to the identifier of the deserialized value.
pub fn deserialize_from_json_ast<Output: Deserializable>(
    parse: &JsonRoot,
    id: &str,
) -> Deserialized<Output> {
    let mut ctx = DefaultDeserializationContext::new(id);
    let deserialized = parse
        .value()
        .ok()
        .and_then(|value| Output::deserialize(&mut ctx, &value, ""));
    Deserialized {
        diagnostics: ctx.diagnostics,
        deserialized,
    }
}

impl DeserializableValue for AnyJsonValue {
    fn range(&self) -> biome_rowan::TextRange {
        AstNode::range(self)
    }

    fn deserialize<V: DeserializationVisitor>(
        &self,
        ctx: &mut impl DeserializationContext,
        visitor: V,
        name: &str,
    ) -> Option<V::Output> {
        let range = AstNode::range(self);
        match self {
            AnyJsonValue::JsonArrayValue(array) => {
                let items = array.elements().iter().map(|x| x.ok());
                visitor.visit_array(ctx, items, range, name)
            }
            AnyJsonValue::JsonBogusValue(_) => {
                // The parser should emit an error about this node
                // No need to emit another diagnostic.
                None
            }
            AnyJsonValue::JsonBooleanValue(value) => {
                let value = value.value_token().ok()?;
                visitor.visit_bool(ctx, value.kind() == T![true], range, name)
            }
            AnyJsonValue::JsonNullValue(_) => visitor.visit_null(ctx, range, name),
            AnyJsonValue::JsonNumberValue(value) => {
                let value = value.value_token().ok()?;
                let token_text = value.token_text_trimmed();
                visitor.visit_number(ctx, TextNumber(token_text), range, name)
            }
            AnyJsonValue::JsonObjectValue(object) => {
                let members = object.json_member_list().iter().map(|member| {
                    let member = member.ok()?;
                    Some((member.name().ok()?, member.value().ok()?))
                });
                visitor.visit_map(ctx, members, range, name)
            }
            AnyJsonValue::JsonStringValue(value) => {
                let value = unescape_json_string(value.inner_string_text().ok()?);
                visitor.visit_str(ctx, value, range, name)
            }
        }
    }

    fn visitable_type(&self) -> Option<DeserializableType> {
        match self {
            AnyJsonValue::JsonArrayValue(_) => Some(DeserializableType::Array),
            AnyJsonValue::JsonBogusValue(_) => None,
            AnyJsonValue::JsonBooleanValue(_) => Some(DeserializableType::Bool),
            AnyJsonValue::JsonNullValue(_) => Some(DeserializableType::Null),
            AnyJsonValue::JsonNumberValue(_) => Some(DeserializableType::Number),
            AnyJsonValue::JsonObjectValue(_) => Some(DeserializableType::Map),
            AnyJsonValue::JsonStringValue(_) => Some(DeserializableType::Str),
        }
    }
}

#[cfg(feature = "serde")]
impl Deserializable for serde_json::Value {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        struct Visitor;
        impl DeserializationVisitor for Visitor {
            type Output = serde_json::Value;
            const EXPECTED_TYPE: crate::DeserializableTypes = crate::DeserializableTypes::all();
            fn visit_null(
                self,
                _ctx: &mut impl DeserializationContext,
                _range: biome_rowan::TextRange,
                _name: &str,
            ) -> Option<Self::Output> {
                Some(serde_json::Value::Null)
            }

            fn visit_bool(
                self,
                _ctx: &mut impl DeserializationContext,
                value: bool,
                _range: biome_rowan::TextRange,
                _name: &str,
            ) -> Option<Self::Output> {
                Some(serde_json::Value::Bool(value))
            }

            fn visit_number(
                self,
                ctx: &mut impl DeserializationContext,
                value: TextNumber,
                _range: biome_rowan::TextRange,
                _name: &str,
            ) -> Option<Self::Output> {
                match serde_json::from_str(value.text()) {
                    Ok(num) => Some(serde_json::Value::Number(num)),
                    Err(err) => {
                        ctx.report(crate::DeserializationDiagnostic::new(err.to_string()));
                        None
                    }
                }
            }

            fn visit_str(
                self,
                _ctx: &mut impl DeserializationContext,
                value: Text,
                _range: biome_rowan::TextRange,
                _name: &str,
            ) -> Option<Self::Output> {
                Some(serde_json::Value::String(value.text().to_string()))
            }

            fn visit_array(
                self,
                ctx: &mut impl DeserializationContext,
                values: impl Iterator<Item = Option<impl DeserializableValue>>,
                _range: biome_rowan::TextRange,
                _name: &str,
            ) -> Option<Self::Output> {
                Some(serde_json::Value::Array(
                    values
                        .filter_map(|value| Deserializable::deserialize(ctx, &value?, ""))
                        .collect(),
                ))
            }

            fn visit_map(
                self,
                ctx: &mut impl DeserializationContext,
                members: impl Iterator<
                    Item = Option<(impl DeserializableValue, impl DeserializableValue)>,
                >,
                _range: biome_rowan::TextRange,
                _name: &str,
            ) -> Option<Self::Output> {
                Some(serde_json::Value::Object(
                    members
                        .filter_map(|entry| {
                            let (key, value) = entry?;
                            let key = Deserializable::deserialize(ctx, &key, "")?;
                            let value = value.deserialize(ctx, Visitor, "")?;
                            Some((key, value))
                        })
                        .collect(),
                ))
            }
        }

        value.deserialize(ctx, Visitor, name)
    }
}

impl DeserializableValue for JsonMemberName {
    fn range(&self) -> biome_rowan::TextRange {
        AstNode::range(self)
    }

    fn deserialize<V: DeserializationVisitor>(
        &self,
        ctx: &mut impl DeserializationContext,
        visitor: V,
        name: &str,
    ) -> Option<V::Output> {
        let value = unescape_json_string(self.inner_string_text().ok()?);
        visitor.visit_str(ctx, value, AstNode::range(self), name)
    }

    fn visitable_type(&self) -> Option<DeserializableType> {
        Some(DeserializableType::Str)
    }
}

/// Returns `text` with escape sequences processed.
///
/// If nothing is escaped, `text` is returned without any allocation. If at
/// least one character is escaped, then a string is allocated and holds the
/// unescaped string.
pub fn unescape_json_string(text: TokenText) -> Text {
    enum State {
        Normal,
        Escaped,
        EscapedUnicode(u8, char),
    }

    match text.find('\\') {
        Some(index) => {
            let mut state = State::Escaped;
            let mut string = text[..index].to_string();
            string.reserve(usize::from(text.len()) - string.len());
            for c in text[(index + 1)..].chars() {
                match state {
                    State::Escaped => {
                        let escaped = match c {
                            '"' => '"',
                            '\\' => '\\',
                            '/' => '/',
                            'b' => '\u{0008}',
                            'f' => '\u{000c}',
                            'n' => '\n',
                            'r' => '\r',
                            't' => '\t',
                            'u' => {
                                state = State::EscapedUnicode(0, '\0');
                                continue;
                            }
                            c => c,
                        };
                        string.push(escaped);
                        state = State::Normal;
                    }
                    State::EscapedUnicode(digit, char) => {
                        let value = if c.is_ascii_digit() {
                            c as u32 - '0' as u32
                        } else if ('a'..='f').contains(&c) {
                            c as u32 - 'a' as u32 + 10
                        } else if ('A'..='F').contains(&c) {
                            c as u32 - 'A' as u32 + 10
                        } else {
                            string.push(c);
                            state = State::Normal;
                            continue;
                        };

                        let char = match (16 * char as u32 + value).try_into() {
                            Ok(char) => char,
                            Err(_) => {
                                state = State::Normal;
                                continue;
                            }
                        };
                        if digit == 3 {
                            string.push(char);
                            state = State::Normal;
                        } else {
                            state = State::EscapedUnicode(digit + 1, char);
                        }
                    }
                    State::Normal if c == '\\' => state = State::Escaped,
                    State::Normal => string.push(c),
                }
            }
            Text::Owned(string)
        }
        None => Text::Borrowed(text),
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::{BTreeMap, HashMap, HashSet},
        num::{NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize},
    };

    use super::*;
    use biome_json_parser::JsonParserOptions;

    #[test]
    fn test_unit() {
        let source = "0";
        let Deserialized {
            deserialized,
            diagnostics,
        } = deserialize_from_json_str::<()>(source, JsonParserOptions::default(), "");
        assert!(!diagnostics.is_empty());
        assert!(deserialized.is_none());
    }

    #[test]
    fn test_name() {
        #[derive(Debug, Eq, PartialEq)]
        struct Name {
            name: String,
        }
        impl Deserializable for Name {
            fn deserialize(
                ctx: &mut impl DeserializationContext,
                _value: &impl DeserializableValue,
                name: &str,
            ) -> Option<Self> {
                Some(Name {
                    name: ctx.id().unwrap_or(name).to_string(),
                })
            }
        }
        let source = "0";
        let Deserialized { deserialized, .. } =
            deserialize_from_json_str::<Name>(source, JsonParserOptions::default(), "root");
        assert_eq!(
            deserialized,
            Some(Name {
                name: "root".to_string()
            })
        )
    }

    #[test]
    fn test_bool() {
        let source = "true";
        let Deserialized {
            deserialized,
            diagnostics,
        } = deserialize_from_json_str::<bool>(source, JsonParserOptions::default(), "");
        assert!(diagnostics.is_empty());
        assert!(deserialized.unwrap());

        let source = "0";
        let Deserialized {
            deserialized,
            diagnostics,
        } = deserialize_from_json_str::<bool>(source, JsonParserOptions::default(), "");
        assert!(!diagnostics.is_empty());
        assert!(deserialized.is_none());
    }

    #[test]
    fn test_f32() {
        let source = "0.5";
        let Deserialized {
            deserialized,
            diagnostics,
        } = deserialize_from_json_str::<f32>(source, JsonParserOptions::default(), "");
        assert!(diagnostics.is_empty());
        assert_eq!(deserialized, Some(0.5));
    }

    #[test]
    fn test_f64() {
        let source = "0.5";
        let Deserialized {
            deserialized,
            diagnostics,
        } = deserialize_from_json_str::<f64>(source, JsonParserOptions::default(), "");
        assert!(diagnostics.is_empty());
        assert_eq!(deserialized, Some(0.5));
    }

    #[test]
    fn test_i8() {
        let source = "-1";
        let Deserialized {
            deserialized,
            diagnostics,
        } = deserialize_from_json_str::<i8>(source, JsonParserOptions::default(), "");
        assert!(diagnostics.is_empty());
        assert_eq!(deserialized, Some(-1));

        let source = u8::MAX.to_string();
        let Deserialized {
            deserialized,
            diagnostics,
        } = deserialize_from_json_str::<i8>(&source, JsonParserOptions::default(), "");
        assert!(!diagnostics.is_empty());
        assert!(deserialized.is_none());
    }

    #[test]
    fn test_i16() {
        let source = "-1";
        let Deserialized {
            deserialized,
            diagnostics,
        } = deserialize_from_json_str::<i16>(source, JsonParserOptions::default(), "");
        assert!(diagnostics.is_empty());
        assert_eq!(deserialized, Some(-1));

        let source = u16::MAX.to_string();
        let Deserialized {
            deserialized,
            diagnostics,
        } = deserialize_from_json_str::<i16>(&source, JsonParserOptions::default(), "");
        assert!(!diagnostics.is_empty());
        assert!(deserialized.is_none());
    }

    #[test]
    fn test_i32() {
        let source = "-1";
        let Deserialized {
            deserialized,
            diagnostics,
        } = deserialize_from_json_str::<i32>(source, JsonParserOptions::default(), "");
        assert!(diagnostics.is_empty());
        assert_eq!(deserialized, Some(-1));

        let source = u32::MAX.to_string();
        let Deserialized {
            deserialized,
            diagnostics,
        } = deserialize_from_json_str::<i32>(&source, JsonParserOptions::default(), "");
        assert!(!diagnostics.is_empty());
        assert!(deserialized.is_none());
    }

    #[test]
    fn test_i64() {
        let source = "-1";
        let Deserialized {
            deserialized,
            diagnostics,
        } = deserialize_from_json_str::<i64>(source, JsonParserOptions::default(), "");
        assert!(diagnostics.is_empty());
        assert_eq!(deserialized, Some(-1));

        let source = u64::MAX.to_string();
        let Deserialized {
            deserialized,
            diagnostics,
        } = deserialize_from_json_str::<i64>(&source, JsonParserOptions::default(), "");
        assert!(!diagnostics.is_empty());
        assert!(deserialized.is_none());
    }

    #[test]
    fn test_isize() {
        let source = "-1";
        let Deserialized {
            deserialized,
            diagnostics,
        } = deserialize_from_json_str::<isize>(source, JsonParserOptions::default(), "");
        assert!(diagnostics.is_empty());
        assert_eq!(deserialized, Some(-1));

        let source = usize::MAX.to_string();
        let Deserialized {
            deserialized,
            diagnostics,
        } = deserialize_from_json_str::<isize>(&source, JsonParserOptions::default(), "");
        assert!(!diagnostics.is_empty());
        assert!(deserialized.is_none());
    }

    #[test]
    fn test_u8() {
        let source = "0";
        let Deserialized {
            deserialized,
            diagnostics,
        } = deserialize_from_json_str::<u8>(source, JsonParserOptions::default(), "");
        assert!(diagnostics.is_empty());
        assert_eq!(deserialized, Some(0));

        let source = "256";
        let Deserialized {
            deserialized,
            diagnostics,
        } = deserialize_from_json_str::<u8>(source, JsonParserOptions::default(), "");
        assert!(!diagnostics.is_empty());
        assert!(deserialized.is_none());
    }

    #[test]
    fn test_u16() {
        let source = "0";
        let Deserialized {
            deserialized,
            diagnostics,
        } = deserialize_from_json_str::<u16>(source, JsonParserOptions::default(), "");
        assert!(diagnostics.is_empty());
        assert_eq!(deserialized, Some(0));

        let source = "-1";
        let Deserialized {
            deserialized,
            diagnostics,
        } = deserialize_from_json_str::<u16>(source, JsonParserOptions::default(), "");
        assert!(!diagnostics.is_empty());
        assert!(deserialized.is_none());
    }

    #[test]
    fn test_u32() {
        let source = "0";
        let Deserialized {
            deserialized,
            diagnostics,
        } = deserialize_from_json_str::<u32>(source, JsonParserOptions::default(), "");
        assert!(diagnostics.is_empty());
        assert_eq!(deserialized, Some(0));

        let source = "-1";
        let Deserialized {
            deserialized,
            diagnostics,
        } = deserialize_from_json_str::<u32>(source, JsonParserOptions::default(), "");
        assert!(!diagnostics.is_empty());
        assert!(deserialized.is_none());
    }

    #[test]
    fn test_u64() {
        let source = "0";
        let Deserialized {
            deserialized,
            diagnostics,
        } = deserialize_from_json_str::<u64>(source, JsonParserOptions::default(), "");
        assert!(diagnostics.is_empty());
        assert_eq!(deserialized, Some(0));

        let source = "-1";
        let Deserialized {
            deserialized,
            diagnostics,
        } = deserialize_from_json_str::<u64>(source, JsonParserOptions::default(), "");
        assert!(!diagnostics.is_empty());
        assert!(deserialized.is_none());
    }

    #[test]
    fn test_usize() {
        let source = "0";
        let Deserialized {
            deserialized,
            diagnostics,
        } = deserialize_from_json_str::<usize>(source, JsonParserOptions::default(), "");
        assert!(diagnostics.is_empty());
        assert_eq!(deserialized, Some(0));

        let source = "-1";
        let Deserialized {
            deserialized,
            diagnostics,
        } = deserialize_from_json_str::<usize>(source, JsonParserOptions::default(), "");
        assert!(!diagnostics.is_empty());
        assert!(deserialized.is_none());
    }

    #[test]
    fn test_non_zero_u8() {
        let source = "1";
        let Deserialized {
            deserialized,
            diagnostics,
        } = deserialize_from_json_str::<NonZeroU8>(source, JsonParserOptions::default(), "");
        assert!(diagnostics.is_empty());
        assert_eq!(deserialized, NonZeroU8::new(1));

        let source = "0";
        let Deserialized {
            deserialized,
            diagnostics,
        } = deserialize_from_json_str::<NonZeroU8>(source, JsonParserOptions::default(), "");
        assert!(!diagnostics.is_empty());
        assert!(deserialized.is_none());
    }

    #[test]
    fn test_non_zero_u16() {
        let source = "1";
        let Deserialized {
            deserialized,
            diagnostics,
        } = deserialize_from_json_str::<NonZeroU16>(source, JsonParserOptions::default(), "");
        assert!(diagnostics.is_empty());
        assert_eq!(deserialized, NonZeroU16::new(1));

        let source = "0";
        let Deserialized {
            deserialized,
            diagnostics,
        } = deserialize_from_json_str::<NonZeroU16>(source, JsonParserOptions::default(), "");
        assert!(!diagnostics.is_empty());
        assert!(deserialized.is_none());
    }

    #[test]
    fn test_non_zero_u32() {
        let source = "1";
        let Deserialized {
            deserialized,
            diagnostics,
        } = deserialize_from_json_str::<NonZeroU32>(source, JsonParserOptions::default(), "");
        assert!(diagnostics.is_empty());
        assert_eq!(deserialized, NonZeroU32::new(1));

        let source = "0";
        let Deserialized {
            deserialized,
            diagnostics,
        } = deserialize_from_json_str::<NonZeroU32>(source, JsonParserOptions::default(), "");
        assert!(!diagnostics.is_empty());
        assert!(deserialized.is_none());
    }

    #[test]
    fn test_non_zero_u64() {
        let source = "1";
        let Deserialized {
            deserialized,
            diagnostics,
        } = deserialize_from_json_str::<NonZeroU64>(source, JsonParserOptions::default(), "");
        assert!(diagnostics.is_empty());
        assert_eq!(deserialized, NonZeroU64::new(1));

        let source = "0";
        let Deserialized {
            deserialized,
            diagnostics,
        } = deserialize_from_json_str::<NonZeroU64>(source, JsonParserOptions::default(), "");
        assert!(!diagnostics.is_empty());
        assert!(deserialized.is_none());
    }

    #[test]
    fn test_non_zero_usize() {
        let source = "1";
        let Deserialized {
            deserialized,
            diagnostics,
        } = deserialize_from_json_str::<NonZeroUsize>(source, JsonParserOptions::default(), "");
        assert!(diagnostics.is_empty());
        assert_eq!(deserialized, NonZeroUsize::new(1));

        let source = "0";
        let Deserialized {
            deserialized,
            diagnostics,
        } = deserialize_from_json_str::<NonZeroUsize>(source, JsonParserOptions::default(), "");
        assert!(!diagnostics.is_empty());
        assert!(deserialized.is_none());
    }

    #[test]
    fn test_number() {
        let source = u128::MAX.to_string();
        let Deserialized {
            deserialized,
            diagnostics,
        } = deserialize_from_json_str::<TextNumber>(&source, JsonParserOptions::default(), "");
        assert!(diagnostics.is_empty());
        assert_eq!(deserialized.unwrap().text(), u128::MAX.to_string());

        let source = "true";
        let Deserialized {
            deserialized,
            diagnostics,
        } = deserialize_from_json_str::<TextNumber>(source, JsonParserOptions::default(), "");
        assert!(!diagnostics.is_empty());
        assert!(deserialized.is_none());
    }

    #[test]
    fn test_string() {
        let source = r#""string""#;
        let Deserialized {
            deserialized,
            diagnostics,
        } = deserialize_from_json_str::<String>(source, JsonParserOptions::default(), "");
        assert!(diagnostics.is_empty());
        assert_eq!(deserialized.unwrap(), "string");

        let source = "0";
        let Deserialized {
            deserialized,
            diagnostics,
        } = deserialize_from_json_str::<String>(source, JsonParserOptions::default(), "");
        assert!(!diagnostics.is_empty());
        assert!(deserialized.is_none());
    }

    #[test]
    fn test_vec() {
        let source = r#"[0, 1]"#;
        let Deserialized {
            deserialized,
            diagnostics,
        } = deserialize_from_json_str::<Vec<u8>>(source, JsonParserOptions::default(), "");
        assert!(diagnostics.is_empty());
        assert_eq!(deserialized.unwrap(), vec![0, 1]);

        let source = "0";
        let Deserialized {
            deserialized,
            diagnostics,
        } = deserialize_from_json_str::<Vec<u8>>(source, JsonParserOptions::default(), "");
        assert!(!diagnostics.is_empty());
        assert!(deserialized.is_none());
    }

    #[test]
    fn test_hash_set() {
        let source = r#"[0, 1]"#;
        let Deserialized {
            deserialized,
            diagnostics,
        } = deserialize_from_json_str::<HashSet<u8>>(source, JsonParserOptions::default(), "");
        assert!(diagnostics.is_empty());
        assert_eq!(deserialized.unwrap(), HashSet::from([0, 1]));

        let source = "0";
        let Deserialized {
            deserialized,
            diagnostics,
        } = deserialize_from_json_str::<HashSet<u8>>(source, JsonParserOptions::default(), "");
        assert!(!diagnostics.is_empty());
        assert!(deserialized.is_none());
    }

    #[test]
    #[cfg(feature = "indexmap")]
    fn test_index_set() {
        let source = r#"[0, 1]"#;
        let Deserialized {
            deserialized,
            diagnostics,
        } = deserialize_from_json_str::<indexmap::IndexSet<u8>>(
            source,
            JsonParserOptions::default(),
            "",
        );
        assert!(diagnostics.is_empty());
        assert_eq!(deserialized.unwrap(), indexmap::IndexSet::from([0, 1]));

        let source = "0";
        let Deserialized {
            deserialized,
            diagnostics,
        } = deserialize_from_json_str::<indexmap::IndexSet<u8>>(
            source,
            JsonParserOptions::default(),
            "",
        );
        assert!(!diagnostics.is_empty());
        assert!(deserialized.is_none());
    }

    #[test]
    fn test_hash_map() {
        let source = r#"{ "a": 0, "b": 1 }"#;
        let Deserialized {
            deserialized,
            diagnostics,
        } = deserialize_from_json_str::<HashMap<String, u8>>(
            source,
            JsonParserOptions::default(),
            "",
        );
        assert!(diagnostics.is_empty());
        assert_eq!(
            deserialized.unwrap(),
            HashMap::from([("a".to_string(), 0), ("b".to_string(), 1)])
        );

        let source = "0";
        let Deserialized {
            deserialized,
            diagnostics,
        } = deserialize_from_json_str::<HashMap<String, u8>>(
            source,
            JsonParserOptions::default(),
            "",
        );
        assert!(!diagnostics.is_empty());
        assert!(deserialized.is_none());
    }

    #[test]
    fn test_b_tree_map_map() {
        let source = r#"{ "a": 0, "b": 1 }"#;
        let Deserialized {
            deserialized,
            diagnostics,
        } = deserialize_from_json_str::<BTreeMap<String, u8>>(
            source,
            JsonParserOptions::default(),
            "",
        );
        assert!(diagnostics.is_empty());
        assert_eq!(
            deserialized.unwrap(),
            BTreeMap::from([("a".to_string(), 0), ("b".to_string(), 1)])
        );

        let source = "0";
        let Deserialized {
            deserialized,
            diagnostics,
        } = deserialize_from_json_str::<BTreeMap<String, u8>>(
            source,
            JsonParserOptions::default(),
            "",
        );
        assert!(!diagnostics.is_empty());
        assert!(deserialized.is_none());
    }

    #[test]
    #[cfg(feature = "indexmap")]
    fn test_index_map() {
        let source = r#"{ "a": 0, "b": 1 }"#;
        let Deserialized {
            deserialized,
            diagnostics,
        } = deserialize_from_json_str::<indexmap::IndexMap<String, u8>>(
            source,
            JsonParserOptions::default(),
            "",
        );
        assert!(diagnostics.is_empty());
        assert_eq!(
            deserialized.unwrap(),
            indexmap::IndexMap::from([("a".to_string(), 0), ("b".to_string(), 1)])
        );

        let source = "0";
        let Deserialized {
            deserialized,
            diagnostics,
        } = deserialize_from_json_str::<indexmap::IndexMap<String, u8>>(
            source,
            JsonParserOptions::default(),
            "",
        );
        assert!(!diagnostics.is_empty());
        assert!(deserialized.is_none());
    }
}
