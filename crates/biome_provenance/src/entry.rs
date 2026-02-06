use crate::ProvenanceValue;
use biome_rowan::TextRange;
use camino::Utf8PathBuf;

/// A single record of where a configuration field's value came from
#[derive(Debug, Clone)]
pub struct ProvenanceEntry {
    /// Field path like "formatter.indentWidth" or "linter.rules[0]"
    pub field_path: String,

    /// The source value as it appeared in the configuration file
    pub source_value: ProvenanceValue,

    /// Text range in the source file
    pub range: TextRange,

    /// Source file path
    pub source_path: Utf8PathBuf,

    /// Merge order: lower = earlier, higher = later (wins in extends chains)
    pub merge_order: u64,
}

impl ProvenanceEntry {
    /// Create a new provenance entry
    pub fn new(
        field_path: String,
        source_value: ProvenanceValue,
        range: TextRange,
        source_path: Utf8PathBuf,
        merge_order: u64,
    ) -> Self {
        Self {
            field_path,
            source_value,
            range,
            source_path,
            merge_order,
        }
    }
}

/// Index of all provenance entries for a configuration
///
/// Allows querying where specific fields came from
#[derive(Debug, Clone)]
pub struct ProvenanceIndex {
    entries: Vec<ProvenanceEntry>,
}

impl ProvenanceIndex {
    /// Create a new provenance index from entries
    pub fn new(entries: Vec<ProvenanceEntry>) -> Self {
        Self { entries }
    }

    /// Query provenance for a specific field path
    ///
    /// If multiple entries exist for the same field (from extends chains),
    /// returns the entry with the highest merge_order (most recent).
    pub fn query(&self, field_path: &str) -> Option<&ProvenanceEntry> {
        self.entries
            .iter()
            .filter(|e| e.field_path == field_path)
            .max_by_key(|e| e.merge_order)
    }

    /// Get all provenance entries
    pub fn all_entries(&self) -> &[ProvenanceEntry] {
        &self.entries
    }

    /// Check if the index is empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Get the number of entries
    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provenance_entry_creation() {
        let entry = ProvenanceEntry::new(
            "formatter.indentWidth".to_string(),
            ProvenanceValue::Number("4".to_string()),
            TextRange::new(0.into(), 1.into()),
            Utf8PathBuf::from("/test/biome.json"),
            0,
        );

        assert_eq!(entry.field_path, "formatter.indentWidth");
        assert_eq!(entry.source_value, ProvenanceValue::Number("4".to_string()));
        assert_eq!(entry.merge_order, 0);
    }

    #[test]
    fn test_provenance_index_query() {
        let entries = vec![
            ProvenanceEntry::new(
                "field1".to_string(),
                ProvenanceValue::String("value1".to_string()),
                TextRange::new(0.into(), 10.into()),
                Utf8PathBuf::from("/test.json"),
                0,
            ),
            ProvenanceEntry::new(
                "field2".to_string(),
                ProvenanceValue::Number("42".to_string()),
                TextRange::new(10.into(), 20.into()),
                Utf8PathBuf::from("/test.json"),
                0,
            ),
        ];

        let index = ProvenanceIndex::new(entries);

        assert_eq!(index.len(), 2);
        assert!(!index.is_empty());

        let entry1 = index.query("field1").unwrap();
        assert_eq!(entry1.field_path, "field1");

        let entry2 = index.query("field2").unwrap();
        assert_eq!(entry2.field_path, "field2");

        assert!(index.query("nonexistent").is_none());
    }

    #[test]
    fn test_provenance_index_merge_order() {
        // Simulate an extends chain where same field is set multiple times
        let entries = vec![
            ProvenanceEntry::new(
                "indentWidth".to_string(),
                ProvenanceValue::Number("2".to_string()),
                TextRange::new(0.into(), 1.into()),
                Utf8PathBuf::from("/base.json"),
                0, // Earlier
            ),
            ProvenanceEntry::new(
                "indentWidth".to_string(),
                ProvenanceValue::Number("4".to_string()),
                TextRange::new(0.into(), 1.into()),
                Utf8PathBuf::from("/biome.json"),
                1, // Later (wins)
            ),
        ];

        let index = ProvenanceIndex::new(entries);

        // Should return the entry with highest merge_order
        let entry = index.query("indentWidth").unwrap();
        assert_eq!(entry.source_value, ProvenanceValue::Number("4".to_string()));
        assert_eq!(entry.merge_order, 1);
        assert_eq!(entry.source_path, Utf8PathBuf::from("/biome.json"));
    }
}
