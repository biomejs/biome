use crate::{ProvenanceEntry, ProvenanceValue};
use biome_rowan::TextRange;
use camino::Utf8PathBuf;

/// Represents a segment in a field path
#[derive(Debug, Clone)]
enum PathSegment {
    /// A field name: "formatter", "indentWidth"
    Field(String),
    /// An array index: [0], [1]
    Index(usize),
}

/// Context for tracking provenance during deserialization
///
/// This is passed through the deserialization process and accumulates
/// provenance entries as values are extracted from the source.
pub struct ProvenanceContext {
    /// The source file path
    source_path: Utf8PathBuf,

    /// Merge order (for extends chains)
    merge_order: u64,

    /// Current field path being processed (stack of segments)
    path_stack: Vec<PathSegment>,

    /// Collected provenance entries
    entries: Vec<ProvenanceEntry>,
}

impl ProvenanceContext {
    /// Create a new provenance context
    pub fn new(source_path: Utf8PathBuf, merge_order: u64) -> Self {
        Self {
            source_path,
            merge_order,
            path_stack: Vec::new(),
            entries: Vec::new(),
        }
    }

    /// Push a field name onto the current path
    pub fn push_field(&mut self, name: &str) {
        self.path_stack.push(PathSegment::Field(name.to_string()));
    }

    /// Push an array index onto the current path
    pub fn push_index(&mut self, index: usize) {
        self.path_stack.push(PathSegment::Index(index));
    }

    /// Pop the last segment from the current path
    pub fn pop(&mut self) {
        self.path_stack.pop();
    }

    /// Get the current field path as a string
    ///
    /// Examples:
    /// - "formatter.indentWidth"
    /// - "linter.rules[0].name"
    pub fn current_path(&self) -> String {
        let mut path = String::new();
        for segment in &self.path_stack {
            match segment {
                PathSegment::Field(name) => {
                    if !path.is_empty() {
                        path.push('.');
                    }
                    path.push_str(name);
                }
                PathSegment::Index(idx) => {
                    path.push('[');
                    path.push_str(&idx.to_string());
                    path.push(']');
                }
            }
        }
        path
    }

    /// Capture a value at the current path
    pub fn capture(&mut self, value: ProvenanceValue, range: TextRange) {
        let entry = ProvenanceEntry::new(
            self.current_path(),
            value,
            range,
            self.source_path.clone(),
            self.merge_order,
        );
        self.entries.push(entry);
    }

    /// Consume the context and return all collected entries
    pub fn into_entries(self) -> Vec<ProvenanceEntry> {
        self.entries
    }

    /// Get a reference to the entries (for testing)
    #[cfg(test)]
    pub fn entries(&self) -> &[ProvenanceEntry] {
        &self.entries
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_tracking() {
        let mut ctx = ProvenanceContext::new(Utf8PathBuf::from("/test.json"), 0);

        assert_eq!(ctx.current_path(), "");

        ctx.push_field("formatter");
        assert_eq!(ctx.current_path(), "formatter");

        ctx.push_field("indentWidth");
        assert_eq!(ctx.current_path(), "formatter.indentWidth");

        ctx.pop();
        assert_eq!(ctx.current_path(), "formatter");

        ctx.push_index(0);
        assert_eq!(ctx.current_path(), "formatter[0]");

        ctx.pop();
        ctx.pop();
        assert_eq!(ctx.current_path(), "");
    }

    #[test]
    fn test_nested_array_path() {
        let mut ctx = ProvenanceContext::new(Utf8PathBuf::from("/test.json"), 0);

        ctx.push_field("items");
        ctx.push_index(0);
        ctx.push_field("name");
        assert_eq!(ctx.current_path(), "items[0].name");
    }

    #[test]
    fn test_capture() {
        let mut ctx = ProvenanceContext::new(Utf8PathBuf::from("/test.json"), 0);

        ctx.push_field("indentWidth");
        ctx.capture(
            ProvenanceValue::Number("4".to_string()),
            TextRange::new(0.into(), 1.into()),
        );

        let entries = ctx.entries();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].field_path, "indentWidth");
        assert_eq!(
            entries[0].source_value,
            ProvenanceValue::Number("4".to_string())
        );
    }

    #[test]
    fn test_multiple_captures() {
        let mut ctx = ProvenanceContext::new(Utf8PathBuf::from("/test.json"), 0);

        ctx.push_field("field1");
        ctx.capture(
            ProvenanceValue::String("value1".to_string()),
            TextRange::new(0.into(), 10.into()),
        );
        ctx.pop();

        ctx.push_field("field2");
        ctx.capture(
            ProvenanceValue::Number("42".to_string()),
            TextRange::new(10.into(), 20.into()),
        );
        ctx.pop();

        let entries = ctx.into_entries();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].field_path, "field1");
        assert_eq!(entries[1].field_path, "field2");
    }
}
