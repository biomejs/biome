use std::fmt::{Display, Formatter};

/// Represents a value from the source configuration file
///
/// This enum captures the original representation of values as they appear
/// in the source (JSON, TOML, YAML, etc.), not the deserialized Rust values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProvenanceValue {
    /// A string value: "space", "hello"
    String(String),

    /// A number value: "2", "2.0", "1e10"
    /// Stored as string to preserve exact formatting from source
    Number(String),

    /// A boolean value: true or false
    Boolean(bool),

    /// A null value
    Null,

    /// A composite value (object/array)
    /// We don't capture the full structure, only track that it exists
    Composite,
}

impl Display for ProvenanceValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(s) => write!(f, "\"{}\"", s),
            Self::Number(n) => write!(f, "{}", n),
            Self::Boolean(b) => write!(f, "{}", b),
            Self::Null => write!(f, "null"),
            Self::Composite => write!(f, "<composite>"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_string() {
        let val = ProvenanceValue::String("hello".to_string());
        assert_eq!(val.to_string(), "\"hello\"");
    }

    #[test]
    fn test_display_number() {
        let val = ProvenanceValue::Number("42".to_string());
        assert_eq!(val.to_string(), "42");
    }

    #[test]
    fn test_display_boolean() {
        let val = ProvenanceValue::Boolean(true);
        assert_eq!(val.to_string(), "true");
    }

    #[test]
    fn test_display_null() {
        let val = ProvenanceValue::Null;
        assert_eq!(val.to_string(), "null");
    }

    #[test]
    fn test_display_composite() {
        let val = ProvenanceValue::Composite;
        assert_eq!(val.to_string(), "<composite>");
    }
}
