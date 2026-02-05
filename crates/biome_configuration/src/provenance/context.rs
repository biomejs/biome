//! Provenance context implementation for tracking configuration values during deserialization.

use super::{FieldQuery, FieldQuerySegment, ProvenanceEntry, ProvenanceSource};
use biome_deserialize::{DeserializationContext, Provenance};
use biome_json_syntax::{AnyJsonValue, JsonMemberName, JsonRoot};
use biome_rowan::{AstNode, AstPtr};

/// Implementation of `Provenance` trait that captures field paths and creates provenance entries.
pub struct ProvenanceImpl {
    /// The current field path being deserialized (stack of segments)
    path_stack: Vec<FieldQuerySegment>,

    /// Captured provenance entries
    entries: Vec<ProvenanceEntry>,

    /// Current merge order (indicates priority in config merging)
    merge_order: u64,

    /// Current source (e.g., BaseConfig, Extends(1))
    source: ProvenanceSource,
}

impl ProvenanceImpl {
    /// Create a new provenance tracker
    pub fn new(source: ProvenanceSource, merge_order: u64) -> Self {
        Self {
            path_stack: Vec::new(),
            entries: Vec::new(),
            merge_order,
            source,
        }
    }

    /// Take the captured entries, consuming the tracker
    pub fn into_entries(self) -> Vec<ProvenanceEntry> {
        self.entries
    }

    /// Convert the current path stack to FieldQuery
    ///
    /// Simply clones the path stack segments.
    fn build_field_query(&self) -> FieldQuery {
        FieldQuery::from_segments(self.path_stack.clone())
    }
}

impl Provenance for ProvenanceImpl {
    fn push_field(&mut self, name: &JsonMemberName) {
        let ptr = AstPtr::new(name);
        self.path_stack.push(FieldQuerySegment::Field(ptr));
    }

    fn push_index(&mut self, index: usize) {
        self.path_stack.push(FieldQuerySegment::Index(index));
    }

    fn pop(&mut self) {
        self.path_stack.pop();
    }

    fn current_path(&self, root: &JsonRoot) -> String {
        let mut path = String::new();
        for segment in &self.path_stack {
            match segment {
                FieldQuerySegment::Field(ptr) => {
                    if !path.is_empty() {
                        path.push('.');
                    }
                    // Resolve the field name from the AstPtr using the JsonRoot
                    if let Some(node) = ptr.try_to_node(root.syntax())
                        && let Ok(text) = node.inner_string_text()
                    {
                        path.push_str(text.text());
                    }
                }
                FieldQuerySegment::Index(idx) => {
                    path.push('[');
                    path.push_str(&idx.to_string());
                    path.push(']');
                }
            }
        }
        path
    }

    fn capture_value(&mut self, value: &AnyJsonValue) {
        let field_query = self.build_field_query();
        let value_ptr = AstPtr::new(value);
        let range = value.range();

        let entry = ProvenanceEntry::new(
            field_query,
            self.source.clone(),
            value_ptr,
            range,
            self.merge_order,
        );
        self.entries.push(entry);
    }
}

/// Wrapper that adds provenance tracking to any `DeserializationContext`.
pub struct ProvenanceAwareDeserializationContext<C: DeserializationContext> {
    inner: C,
    provenance: ProvenanceImpl,
}

impl<C: DeserializationContext> ProvenanceAwareDeserializationContext<C> {
    /// Create a new provenance-aware context wrapping an existing context
    pub fn new(inner: C, source: ProvenanceSource, merge_order: u64) -> Self {
        Self {
            inner,
            provenance: ProvenanceImpl::new(source, merge_order),
        }
    }

    /// Take the captured provenance entries
    pub fn take_entries(self) -> Vec<ProvenanceEntry> {
        self.provenance.into_entries()
    }
}

impl<C: DeserializationContext> DeserializationContext
    for ProvenanceAwareDeserializationContext<C>
{
    fn id(&self) -> Option<&str> {
        self.inner.id()
    }

    fn report(&mut self, diagnostic: biome_deserialize::DeserializationDiagnostic) {
        self.inner.report(diagnostic);
    }

    fn root(&self) -> &JsonRoot {
        self.inner.root()
    }

    fn provenance(&mut self) -> Option<&mut dyn Provenance> {
        Some(&mut self.provenance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_json_parser::{JsonParserOptions, parse_json};

    use biome_rowan::AstSeparatedList;

    #[test]
    fn test_path_tracking() {
        let json_src = r#"{"formatter": {"indentWidth": 4}}"#;
        let parse = parse_json(json_src, JsonParserOptions::default());
        let json_root = parse.tree();

        let mut provenance = ProvenanceImpl::new(
            ProvenanceSource::BaseConfig {
                path: "test.json".into(),
            },
            0,
        );

        // Test basic path tracking
        assert_eq!(provenance.current_path(&json_root), "");

        // Extract the actual JsonMemberName nodes from the parsed tree
        let root_value = json_root.value().expect("should have root value");
        let obj = root_value
            .as_json_object_value()
            .expect("Expected JsonObjectValue");

        // Get "formatter" member
        let formatter_member = obj
            .json_member_list()
            .iter()
            .find_map(|m| m.ok())
            .expect("should have formatter member");
        let formatter_member_name = formatter_member.name().expect("should have name");
        let formatter_name = formatter_member_name
            .as_json_member_name()
            .expect("Expected JsonMemberName");

        provenance.push_field(formatter_name);
        assert_eq!(provenance.current_path(&json_root), "formatter");

        // Get "indentWidth" member from nested object
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

        provenance.push_field(indent_name);
        assert_eq!(provenance.current_path(&json_root), "formatter.indentWidth");

        provenance.pop();
        assert_eq!(provenance.current_path(&json_root), "formatter");

        provenance.push_index(0);
        assert_eq!(provenance.current_path(&json_root), "formatter[0]");

        provenance.pop();
        provenance.pop();
        assert_eq!(provenance.current_path(&json_root), "");
    }

    #[test]
    fn test_capture_creates_entry() {
        let json_src = r#"{"formatter": {"indentWidth": 4}}"#;
        let parse = parse_json(json_src, JsonParserOptions::default());
        let json_root = parse.tree();

        let mut provenance = ProvenanceImpl::new(
            ProvenanceSource::BaseConfig {
                path: "test.json".into(),
            },
            0,
        );

        // Extract the actual JsonMemberName node from the parsed tree
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

        provenance.push_field(formatter_name);
        provenance.capture_value(&root_value);

        let entries = provenance.into_entries();
        assert_eq!(entries.len(), 1);

        // Verify the entry has the correct range and field path
        assert_eq!(entries[0].range, root_value.range());
        assert_eq!(entries[0].field_query.len(), 1);
    }
}
