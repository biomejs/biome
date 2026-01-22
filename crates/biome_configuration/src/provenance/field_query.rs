use biome_json_syntax::JsonMemberName;
use biome_rowan::AstPtr;

/// A segment in a configuration field query
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldQuerySegment {
    /// A named field: stores a pointer to the JsonMemberName node
    /// AstPtr is thread-safe (Send + Sync) and only 8 bytes
    Field(AstPtr<JsonMemberName>),

    /// An array index: [2]
    Index(usize),
}

/// Represents a captured field path during configuration deserialization
/// Examples: "formatter.indentWidth", "extends[2]", "overrides[1].includes[0]"
///
/// **Important**: This structure is used ONLY during the capture phase when we have
/// access to the JsonRoot and can create proper AstPtr references. User queries
/// (from CLI/LSP) remain as strings and are processed using the QueryVisitor pattern.
///
/// Thread-safe structure using AstPtr (Send + Sync) for minimal memory usage.
/// Field names are resolved on-demand using JsonRoot when displaying or comparing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldQuery {
    pub(crate) segments: Vec<FieldQuerySegment>,
}

impl FieldQuery {
    /// Create a new empty field query
    pub fn new() -> Self {
        Self {
            segments: Vec::new(),
        }
    }

    /// Create a field query from segments
    pub fn from_segments(segments: Vec<FieldQuerySegment>) -> Self {
        Self { segments }
    }

    /// Add a field segment
    pub fn push_field(&mut self, ptr: AstPtr<JsonMemberName>) {
        self.segments.push(FieldQuerySegment::Field(ptr));
    }

    /// Add an index segment
    pub fn push_index(&mut self, index: usize) {
        self.segments.push(FieldQuerySegment::Index(index));
    }

    /// Remove the last segment
    pub fn pop(&mut self) -> Option<FieldQuerySegment> {
        self.segments.pop()
    }

    /// Get the segments
    pub fn segments(&self) -> &[FieldQuerySegment] {
        &self.segments
    }

    /// Check if this query starts with another query (for prefix matching during capture)
    pub fn starts_with(&self, prefix: &FieldQuery) -> bool {
        if prefix.segments.len() > self.segments.len() {
            return false;
        }
        self.segments[..prefix.segments.len()] == prefix.segments
    }

    /// Check if the query is empty
    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }

    /// Get the length of the query (number of segments)
    pub fn len(&self) -> usize {
        self.segments.len()
    }
}

impl Default for FieldQuery {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_query_new() {
        let query = FieldQuery::new();
        assert!(query.is_empty());
        assert_eq!(query.len(), 0);
    }

    #[test]
    fn test_field_query_push_index() {
        let mut query = FieldQuery::new();
        query.push_index(0);
        query.push_index(1);

        assert_eq!(query.len(), 2);
        assert!(!query.is_empty());

        match &query.segments[0] {
            FieldQuerySegment::Index(idx) => assert_eq!(*idx, 0),
            _ => panic!("Expected index segment"),
        }

        match &query.segments[1] {
            FieldQuerySegment::Index(idx) => assert_eq!(*idx, 1),
            _ => panic!("Expected index segment"),
        }
    }

    #[test]
    fn test_field_query_pop() {
        let mut query = FieldQuery::new();
        query.push_index(0);
        query.push_index(1);

        let popped = query.pop();
        assert!(popped.is_some());
        assert_eq!(query.len(), 1);

        let popped = query.pop();
        assert!(popped.is_some());
        assert_eq!(query.len(), 0);

        let popped = query.pop();
        assert!(popped.is_none());
    }

    #[test]
    fn test_field_query_starts_with() {
        let mut prefix = FieldQuery::new();
        prefix.push_index(0);

        let mut full = FieldQuery::new();
        full.push_index(0);
        full.push_index(1);

        assert!(full.starts_with(&prefix));
        assert!(!prefix.starts_with(&full));

        let empty = FieldQuery::new();
        assert!(full.starts_with(&empty));
        assert!(prefix.starts_with(&empty));
    }
}
