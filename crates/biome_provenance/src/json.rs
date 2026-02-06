//! JSON implementation of ProvenanceSourceNode

use crate::{ProvenanceSourceNode, ProvenanceValue};
use biome_json_syntax::*;
use biome_rowan::{AstNode, AstSeparatedList, TextRange};

impl ProvenanceSourceNode for AnyJsonValue {
    fn source_value(&self) -> Option<ProvenanceValue> {
        match self {
            Self::JsonStringValue(s) => {
                let text = s.inner_string_text().ok()?;
                Some(ProvenanceValue::String(text.to_string()))
            }
            Self::JsonNumberValue(n) => {
                let token = n.value_token().ok()?;
                Some(ProvenanceValue::Number(token.text_trimmed().to_string()))
            }
            Self::JsonBooleanValue(b) => {
                let token = b.value_token().ok()?;
                let is_true = token.kind() == T![true];
                Some(ProvenanceValue::Boolean(is_true))
            }
            Self::JsonNullValue(_) => Some(ProvenanceValue::Null),
            Self::JsonObjectValue(_) | Self::JsonArrayValue(_) => Some(ProvenanceValue::Composite),
            // Bogus/invalid nodes
            _ => None,
        }
    }

    fn range(&self) -> TextRange {
        AstNode::range(self)
    }

    fn is_object(&self) -> bool {
        matches!(self, Self::JsonObjectValue(_))
    }

    fn is_array(&self) -> bool {
        matches!(self, Self::JsonArrayValue(_))
    }

    fn traverse_fields(&self, visitor: &mut dyn FnMut(&str, &Self)) {
        if let Self::JsonObjectValue(obj) = self {
            for member in obj.json_member_list().iter().filter_map(|m| m.ok()) {
                if let (Ok(name), Ok(value)) = (member.name(), member.value()) {
                    if let Some(Ok(key_text)) = name.inner_string_text() {
                        let key_str: &str = key_text.text();
                        visitor(key_str, &value);
                    }
                }
            }
        }
    }

    fn traverse_array(&self, visitor: &mut dyn FnMut(usize, &Self)) {
        if let Self::JsonArrayValue(arr) = self {
            for (index, element) in arr.elements().iter().enumerate() {
                if let Ok(value) = element {
                    visitor(index, &value);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_json_parser::{JsonParserOptions, parse_json};

    #[test]
    fn test_json_string_value() {
        let json = r#""hello""#;
        let parsed = parse_json(json, JsonParserOptions::default());
        let root = parsed.tree();
        let value = root.value().unwrap();

        assert_eq!(
            value.source_value(),
            Some(ProvenanceValue::String("hello".to_string()))
        );
        assert!(!value.is_object());
        assert!(!value.is_array());
    }

    #[test]
    fn test_json_number_value() {
        let json = r#"42"#;
        let parsed = parse_json(json, JsonParserOptions::default());
        let root = parsed.tree();
        let value = root.value().unwrap();

        assert_eq!(
            value.source_value(),
            Some(ProvenanceValue::Number("42".to_string()))
        );
    }

    #[test]
    fn test_json_number_preserves_formatting() {
        let json = r#"2.0"#;
        let parsed = parse_json(json, JsonParserOptions::default());
        let root = parsed.tree();
        let value = root.value().unwrap();

        // Should preserve "2.0" not "2"
        assert_eq!(
            value.source_value(),
            Some(ProvenanceValue::Number("2.0".to_string()))
        );
    }

    #[test]
    fn test_json_boolean_true() {
        let json = r#"true"#;
        let parsed = parse_json(json, JsonParserOptions::default());
        let root = parsed.tree();
        let value = root.value().unwrap();

        assert_eq!(value.source_value(), Some(ProvenanceValue::Boolean(true)));
    }

    #[test]
    fn test_json_boolean_false() {
        let json = r#"false"#;
        let parsed = parse_json(json, JsonParserOptions::default());
        let root = parsed.tree();
        let value = root.value().unwrap();

        assert_eq!(value.source_value(), Some(ProvenanceValue::Boolean(false)));
    }

    #[test]
    fn test_json_null_value() {
        let json = r#"null"#;
        let parsed = parse_json(json, JsonParserOptions::default());
        let root = parsed.tree();
        let value = root.value().unwrap();

        assert_eq!(value.source_value(), Some(ProvenanceValue::Null));
    }

    #[test]
    fn test_json_object_value() {
        let json = r#"{"key": "value"}"#;
        let parsed = parse_json(json, JsonParserOptions::default());
        let root = parsed.tree();
        let value = root.value().unwrap();

        assert_eq!(value.source_value(), Some(ProvenanceValue::Composite));
        assert!(value.is_object());
        assert!(!value.is_array());
    }

    #[test]
    fn test_json_array_value() {
        let json = r#"[1, 2, 3]"#;
        let parsed = parse_json(json, JsonParserOptions::default());
        let root = parsed.tree();
        let value = root.value().unwrap();

        assert_eq!(value.source_value(), Some(ProvenanceValue::Composite));
        assert!(!value.is_object());
        assert!(value.is_array());
    }

    #[test]
    fn test_traverse_fields() {
        let json = r#"{"name": "John", "age": 30}"#;
        let parsed = parse_json(json, JsonParserOptions::default());
        let root = parsed.tree();
        let value = root.value().unwrap();

        let mut fields = Vec::new();
        value.traverse_fields(&mut |key, val| {
            fields.push((key.to_string(), val.source_value()));
        });

        assert_eq!(fields.len(), 2);
        assert_eq!(fields[0].0, "name");
        assert_eq!(
            fields[0].1,
            Some(ProvenanceValue::String("John".to_string()))
        );
        assert_eq!(fields[1].0, "age");
        assert_eq!(fields[1].1, Some(ProvenanceValue::Number("30".to_string())));
    }

    #[test]
    fn test_traverse_array() {
        let json = r#"["apple", "banana", "cherry"]"#;
        let parsed = parse_json(json, JsonParserOptions::default());
        let root = parsed.tree();
        let value = root.value().unwrap();

        let mut elements = Vec::new();
        value.traverse_array(&mut |idx, val| {
            elements.push((idx, val.source_value()));
        });

        assert_eq!(elements.len(), 3);
        assert_eq!(elements[0].0, 0);
        assert_eq!(
            elements[0].1,
            Some(ProvenanceValue::String("apple".to_string()))
        );
        assert_eq!(elements[1].0, 1);
        assert_eq!(
            elements[1].1,
            Some(ProvenanceValue::String("banana".to_string()))
        );
        assert_eq!(elements[2].0, 2);
        assert_eq!(
            elements[2].1,
            Some(ProvenanceValue::String("cherry".to_string()))
        );
    }

    #[test]
    fn test_range() {
        let json = r#"{"key": "value"}"#;
        let parsed = parse_json(json, JsonParserOptions::default());
        let root = parsed.tree();
        let value = root.value().unwrap();

        let range = ProvenanceSourceNode::range(&value);
        assert!(range.start() < range.end());
    }
}
