use super::{FieldQuery, ProvenanceSource};
use biome_json_syntax::AnyJsonValue;
use biome_rowan::AstPtr;

/// A single record of a field being set
#[derive(Debug, Clone)]
pub struct ProvenanceEntry {
    /// Path to the field that was set
    pub field_query: FieldQuery,

    /// Where this value came from
    pub source: ProvenanceSource,

    /// Pointer to the JSON value node in the parsed tree
    /// AstPtr is thread-safe (Send + Sync) and can be resolved later
    /// by calling workspace.get_parse() to retrieve the JsonRoot
    pub value_ptr: AstPtr<AnyJsonValue>,

    /// Merge order: lower = earlier, higher = later (wins)
    pub merge_order: u64,
}

impl ProvenanceEntry {
    /// Create a new provenance entry
    pub fn new(
        field_query: FieldQuery,
        source: ProvenanceSource,
        value_ptr: AstPtr<AnyJsonValue>,
        merge_order: u64,
    ) -> Self {
        Self {
            field_query,
            source,
            value_ptr,
            merge_order,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provenance::FieldQuery;

    #[test]
    fn test_provenance_entry_creation() {
        let _query = FieldQuery::new();
        let _source = ProvenanceSource::default();

        // We can't create a real AstPtr without parsing JSON,
        // but we can test the structure compiles
        // This test mainly ensures the types are correct

        // Just verify the types work together
        fn _type_check(
            query: FieldQuery,
            source: ProvenanceSource,
            ptr: AstPtr<AnyJsonValue>,
            order: u64,
        ) -> ProvenanceEntry {
            ProvenanceEntry::new(query, source, ptr, order)
        }
    }
}
