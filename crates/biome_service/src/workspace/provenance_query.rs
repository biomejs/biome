//! Query parsing and matching for configuration provenance tracking.
//!
//! This module provides zero-allocation query parsing for configuration field paths like
//! "formatter.indentWidth" or "overrides[0].linter", and matching logic to find provenance
//! entries that correspond to those queries.

use biome_configuration::provenance::{FieldQuery, FieldQuerySegment, ProvenanceEntry};
use biome_console::markup;
use biome_diagnostics::{Diagnostic, Severity, category};
use biome_json_syntax::JsonRoot;
use biome_rowan::AstNode;
use serde::{Deserialize, Serialize};

/// A segment in a parsed query string (zero-copy, uses string slices)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParsedSegment<'a> {
    /// A named field like "formatter" or "indentWidth"
    Field(&'a str),
    /// An array index like [0] or [2]
    Index(usize),
}

/// A parsed query path (zero-copy, uses string slices)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedQuery<'a> {
    segments: Vec<ParsedSegment<'a>>,
}

impl<'a> ParsedQuery<'a> {
    /// Get the segments of this query
    pub fn segments(&self) -> &[ParsedSegment<'a>] {
        &self.segments
    }
}

/// Diagnostic for query parsing failures
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueryParseError {
    /// Human-readable error message
    pub message: String,
    /// Position in the input string where the error occurred (byte offset)
    pub position: usize,
    /// The query string that failed to parse
    pub query: String,
}

impl QueryParseError {
    fn new(message: String, position: usize, query: &str) -> Self {
        Self {
            message,
            position,
            query: query.to_string(),
        }
    }
}

impl Diagnostic for QueryParseError {
    fn category(&self) -> Option<&'static biome_diagnostics::Category> {
        Some(category!("provenance/queryParse"))
    }

    fn severity(&self) -> Severity {
        Severity::Error
    }

    fn message(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_markup(markup! {
            "Failed to parse configuration query"
        })
    }

    fn description(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            fmt,
            "{} at position {} in query: '{}'",
            self.message, self.position, self.query
        )
    }
}

/// Parse a query string like "formatter.indentWidth" or "overrides[0].linter" into segments.
///
/// Uses zero-copy string slices for efficiency.
///
/// # Examples
///
/// ```ignore
/// let query = parse_query_segments("formatter.indentWidth")?;
/// assert_eq!(query.segments().len(), 2);
/// ```
///
/// # Errors
///
/// Returns `QueryParseError` if the query string is malformed (e.g., unclosed brackets,
/// invalid array indices, empty field names).
pub fn parse_query_segments(query: &str) -> Result<ParsedQuery<'_>, QueryParseError> {
    let mut segments = Vec::new();
    let mut chars = query.char_indices().peekable();

    while let Some((start_pos, ch)) = chars.next() {
        match ch {
            // Field name
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut end_pos = start_pos;
                while let Some(&(pos, ch)) = chars.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        end_pos = pos;
                        chars.next();
                    } else {
                        break;
                    }
                }
                // Include the last character
                end_pos += 1;
                segments.push(ParsedSegment::Field(&query[start_pos..end_pos]));

                // Expect . or [ or end
                if let Some(&(_, next_ch)) = chars.peek() {
                    if next_ch == '.' {
                        chars.next(); // consume the dot
                    } else if next_ch != '[' {
                        return Err(QueryParseError::new(
                            format!("Expected '.' or '[' after field name, found '{}'", next_ch),
                            end_pos,
                            query,
                        ));
                    }
                }
            }
            // Array index
            '[' => {
                // Find the closing bracket
                let mut index_str = String::new();
                let mut found_close = false;
                let index_start = start_pos + 1;

                for (pos, ch) in chars.by_ref() {
                    if ch == ']' {
                        found_close = true;
                        break;
                    } else if ch.is_ascii_digit() {
                        index_str.push(ch);
                    } else {
                        return Err(QueryParseError::new(
                            format!("Invalid character '{}' in array index", ch),
                            pos,
                            query,
                        ));
                    }
                }

                if !found_close {
                    return Err(QueryParseError::new(
                        "Unclosed array index bracket".to_string(),
                        index_start,
                        query,
                    ));
                }

                if index_str.is_empty() {
                    return Err(QueryParseError::new(
                        "Empty array index".to_string(),
                        index_start,
                        query,
                    ));
                }

                let index = index_str.parse::<usize>().map_err(|_| {
                    QueryParseError::new(
                        format!("Array index '{}' is too large", index_str),
                        index_start,
                        query,
                    )
                })?;

                segments.push(ParsedSegment::Index(index));

                // Expect . or [ or end
                if let Some(&(pos, next_ch)) = chars.peek() {
                    if next_ch == '.' {
                        chars.next(); // consume the dot
                    } else if next_ch != '[' {
                        return Err(QueryParseError::new(
                            format!("Expected '.' or '[' after array index, found '{}'", next_ch),
                            pos,
                            query,
                        ));
                    }
                }
            }
            '.' => {
                return Err(QueryParseError::new(
                    if start_pos == 0 {
                        "Query cannot start with '.'".to_string()
                    } else {
                        "Empty field name (double dot)".to_string()
                    },
                    start_pos,
                    query,
                ));
            }
            _ => {
                return Err(QueryParseError::new(
                    format!("Unexpected character '{}'", ch),
                    start_pos,
                    query,
                ));
            }
        }
    }

    if segments.is_empty() {
        return Err(QueryParseError::new(
            "Query cannot be empty".to_string(),
            0,
            query,
        ));
    }

    Ok(ParsedQuery { segments })
}

/// Check if a captured FieldQuery matches a user's ParsedQuery.
///
/// This compares the segments of both queries, resolving field names from the FieldQuery's
/// AstPtr references using the provided JsonRoot.
///
/// Returns `true` if all segments match, `false` otherwise.
pub fn field_path_matches(
    field_query: &FieldQuery,
    parsed_query: &ParsedQuery,
    json_root: &JsonRoot,
) -> bool {
    let field_segments = field_query.segments();
    let parsed_segments = parsed_query.segments();

    if field_segments.len() != parsed_segments.len() {
        return false;
    }

    for (field_seg, parsed_seg) in field_segments.iter().zip(parsed_segments.iter()) {
        match (field_seg, parsed_seg) {
            (FieldQuerySegment::Index(f_idx), ParsedSegment::Index(p_idx)) => {
                if f_idx != p_idx {
                    return false;
                }
            }
            (FieldQuerySegment::Field(ptr), ParsedSegment::Field(name)) => {
                // Resolve the field name from the AstPtr
                if let Some(node) = ptr.try_to_node(json_root.syntax()) {
                    if let Ok(text) = node.inner_string_text() {
                        if text.text() != *name {
                            return false;
                        }
                    } else {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            // Mismatch: one is field, other is index
            _ => return false,
        }
    }

    true
}

/// Query provenance entries to find which one sets a specific configuration field.
///
/// Returns the provenance entry with the highest merge_order that matches the query,
/// or `None` if no entries match.
///
/// The entry with the highest `merge_order` wins, as it was the last one to set the value.
pub fn query_provenance<'a>(
    entries: &'a [ProvenanceEntry],
    query: &str,
    json_root: &JsonRoot,
) -> Result<Option<&'a ProvenanceEntry>, QueryParseError> {
    let parsed_query = parse_query_segments(query)?;

    let winner = entries
        .iter()
        .filter(|entry| field_path_matches(&entry.field_query, &parsed_query, json_root))
        .max_by_key(|entry| entry.merge_order);

    Ok(winner)
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_configuration::provenance::{FieldQuery, ProvenanceEntry, ProvenanceSource};
    use biome_json_parser::{JsonParserOptions, parse_json};

    use biome_rowan::{AstNode, AstPtr, AstSeparatedList};

    #[test]
    fn test_parse_simple_query() {
        let query = parse_query_segments("formatter").expect("should parse");
        assert_eq!(query.segments().len(), 1);
        assert_eq!(query.segments()[0], ParsedSegment::Field("formatter"));
    }

    #[test]
    fn test_parse_nested_query() {
        let query = parse_query_segments("formatter.indentWidth").expect("should parse");
        assert_eq!(query.segments().len(), 2);
        assert_eq!(query.segments()[0], ParsedSegment::Field("formatter"));
        assert_eq!(query.segments()[1], ParsedSegment::Field("indentWidth"));
    }

    #[test]
    fn test_parse_query_with_array_index() {
        let query = parse_query_segments("overrides[0].linter").expect("should parse");
        assert_eq!(query.segments().len(), 3);
        assert_eq!(query.segments()[0], ParsedSegment::Field("overrides"));
        assert_eq!(query.segments()[1], ParsedSegment::Index(0));
        assert_eq!(query.segments()[2], ParsedSegment::Field("linter"));
    }

    #[test]
    fn test_parse_query_with_multiple_indices() {
        let query = parse_query_segments("a[0].b[1].c").expect("should parse");
        assert_eq!(query.segments().len(), 5);
        assert_eq!(query.segments()[0], ParsedSegment::Field("a"));
        assert_eq!(query.segments()[1], ParsedSegment::Index(0));
        assert_eq!(query.segments()[2], ParsedSegment::Field("b"));
        assert_eq!(query.segments()[3], ParsedSegment::Index(1));
        assert_eq!(query.segments()[4], ParsedSegment::Field("c"));
    }

    #[test]
    fn test_parse_empty_query() {
        let err = parse_query_segments("").expect_err("should fail");
        assert_eq!(err.position, 0);
        assert!(err.message.contains("empty"));
    }

    #[test]
    fn test_parse_query_starting_with_dot() {
        let err = parse_query_segments(".formatter").expect_err("should fail");
        assert_eq!(err.position, 0);
        assert!(err.message.contains("start with"));
    }

    #[test]
    fn test_parse_unclosed_bracket() {
        let err = parse_query_segments("overrides[0").expect_err("should fail");
        assert!(err.message.contains("Unclosed"));
    }

    #[test]
    fn test_parse_empty_array_index() {
        let err = parse_query_segments("overrides[]").expect_err("should fail");
        assert!(err.message.contains("Empty"));
    }

    #[test]
    fn test_parse_invalid_character_in_index() {
        let err = parse_query_segments("overrides[a]").expect_err("should fail");
        assert!(err.message.contains("Invalid character"));
    }

    #[test]
    fn test_parse_double_dot() {
        let err = parse_query_segments("formatter..indentWidth").expect_err("should fail");
        // After "formatter.", we expect a field name but get another dot
        // This should trigger "Empty field name (double dot)" error
        assert!(err.message.contains("Empty") || err.message.contains("double"));
    }

    #[test]
    fn test_field_path_matches_simple() {
        // Create a JSON document with a simple field
        let json_src = r#"{"formatter": {"indentWidth": 4}}"#;
        let parse = parse_json(json_src, JsonParserOptions::default());
        let json_root = parse.tree();

        // Get the "formatter" member name
        let root_value = json_root.value().expect("should have root value");
        let obj = root_value
            .as_json_object_value()
            .expect("Expected JsonObjectValue");
        let formatter_member = obj
            .json_member_list()
            .iter()
            .find_map(|m| m.ok())
            .expect("should have formatter member");
        let formatter_member_name = formatter_member.name().expect("should have name");
        let formatter_name = formatter_member_name
            .as_json_member_name()
            .expect("Expected JsonMemberName");

        // Create a FieldQuery with just "formatter"
        let mut field_query = FieldQuery::new();
        field_query.push_field(AstPtr::new(formatter_name));

        // Create a ParsedQuery
        let parsed_query = parse_query_segments("formatter").expect("should parse");

        // They should match
        assert!(field_path_matches(&field_query, &parsed_query, &json_root));

        // Different query should not match
        let different_query = parse_query_segments("linter").expect("should parse");
        assert!(!field_path_matches(
            &field_query,
            &different_query,
            &json_root
        ));
    }

    #[test]
    fn test_field_path_matches_nested() {
        let json_src = r#"{"formatter": {"indentWidth": 4}}"#;
        let parse = parse_json(json_src, JsonParserOptions::default());
        let json_root = parse.tree();

        // Get both "formatter" and "indentWidth" member names
        let root_value = json_root.value().expect("should have root value");
        let obj = root_value
            .as_json_object_value()
            .expect("Expected JsonObjectValue");
        let formatter_member = obj
            .json_member_list()
            .iter()
            .find_map(|m| m.ok())
            .expect("should have formatter member");
        let formatter_member_name = formatter_member.name().expect("should have name");
        let formatter_name = formatter_member_name
            .as_json_member_name()
            .expect("Expected JsonMemberName");

        let formatter_value = formatter_member.value().expect("should have value");
        let formatter_obj = formatter_value
            .as_json_object_value()
            .expect("Expected JsonObjectValue");
        let indent_member = formatter_obj
            .json_member_list()
            .iter()
            .find_map(|m| m.ok())
            .expect("should have indentWidth member");
        let indent_member_name = indent_member.name().expect("should have name");
        let indent_name = indent_member_name
            .as_json_member_name()
            .expect("Expected JsonMemberName");

        // Create a FieldQuery with "formatter.indentWidth"
        let mut field_query = FieldQuery::new();
        field_query.push_field(AstPtr::new(formatter_name));
        field_query.push_field(AstPtr::new(indent_name));

        // Should match
        let parsed_query = parse_query_segments("formatter.indentWidth").expect("should parse");
        assert!(field_path_matches(&field_query, &parsed_query, &json_root));

        // Partial match should fail
        let partial_query = parse_query_segments("formatter").expect("should parse");
        assert!(!field_path_matches(
            &field_query,
            &partial_query,
            &json_root
        ));
    }

    #[test]
    fn test_field_path_matches_with_index() {
        let json_src = r#"{"overrides": [{"linter": {}}]}"#;
        let parse = parse_json(json_src, JsonParserOptions::default());
        let json_root = parse.tree();

        // Get "overrides" member name
        let root_value = json_root.value().expect("should have root value");
        let obj = root_value
            .as_json_object_value()
            .expect("Expected JsonObjectValue");
        let overrides_member = obj
            .json_member_list()
            .iter()
            .find_map(|m| m.ok())
            .expect("should have overrides member");
        let overrides_member_name = overrides_member.name().expect("should have name");
        let overrides_name = overrides_member_name
            .as_json_member_name()
            .expect("Expected JsonMemberName");

        // Create a FieldQuery with "overrides[0]"
        let mut field_query = FieldQuery::new();
        field_query.push_field(AstPtr::new(overrides_name));
        field_query.push_index(0);

        // Should match
        let parsed_query = parse_query_segments("overrides[0]").expect("should parse");
        assert!(field_path_matches(&field_query, &parsed_query, &json_root));

        // Different index should not match
        let different_index = parse_query_segments("overrides[1]").expect("should parse");
        assert!(!field_path_matches(
            &field_query,
            &different_index,
            &json_root
        ));
    }

    #[test]
    fn test_query_provenance_winner_selection() {
        let json_src = r#"{"formatter": {"indentWidth": 4}}"#;
        let parse = parse_json(json_src, JsonParserOptions::default());
        let json_root = parse.tree();

        // Get the "formatter" member name
        let root_value = json_root.value().expect("should have root value");
        let obj = root_value
            .as_json_object_value()
            .expect("Expected JsonObjectValue");
        let formatter_member = obj
            .json_member_list()
            .iter()
            .find_map(|m| m.ok())
            .expect("should have formatter member");
        let formatter_member_name = formatter_member.name().expect("should have name");
        let formatter_name = formatter_member_name
            .as_json_member_name()
            .expect("Expected JsonMemberName");

        // Create multiple entries with different merge_order
        let mut field_query1 = FieldQuery::new();
        field_query1.push_field(AstPtr::new(formatter_name));

        let mut field_query2 = FieldQuery::new();
        field_query2.push_field(AstPtr::new(formatter_name));

        let entry1 = ProvenanceEntry::new(
            field_query1,
            ProvenanceSource::BaseConfig {
                path: "base.json".into(),
            },
            AstPtr::new(&root_value),
            root_value.range(),
            0, // Lower merge_order
        );

        let entry2 = ProvenanceEntry::new(
            field_query2,
            ProvenanceSource::BaseConfig {
                path: "override.json".into(),
            },
            AstPtr::new(&root_value),
            root_value.range(),
            10, // Higher merge_order
        );

        let entries = vec![entry1, entry2];

        // The entry with the highest merge_order should win
        let winner = query_provenance(&entries, "formatter", &json_root)
            .expect("should query successfully")
            .expect("should have a winner");

        assert_eq!(winner.merge_order, 10);
    }

    #[test]
    fn test_query_provenance_no_match() {
        let json_src = r#"{"formatter": {"indentWidth": 4}}"#;
        let parse = parse_json(json_src, JsonParserOptions::default());
        let json_root = parse.tree();

        // Query for a field that doesn't exist in entries
        let entries = vec![];
        let result =
            query_provenance(&entries, "linter", &json_root).expect("should query successfully");

        assert!(result.is_none());
    }
}
