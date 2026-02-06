//! Provenance index for tracking configuration source information.
//!
//! This module provides storage and querying capabilities for configuration
//! provenance data. The `ProvenanceIndex` holds all provenance entries and
//! override metadata, allowing efficient lookup of where configuration values
//! originated.

use biome_configuration::provenance::{OverrideProvenanceMetadata, ProvenanceEntry};
use camino::Utf8PathBuf;

/// Index storing all provenance information for a loaded configuration.
///
/// This type is designed to be stored in `Arc` for cheap cloning across
/// threads. Once built, it is immutable.
#[derive(Debug, Clone)]
pub struct ProvenanceIndex {
    /// All provenance entries, sorted by merge_order (ascending).
    ///
    /// Entries with higher merge_order win when multiple entries match the same field.
    entries: Vec<ProvenanceEntry>,

    /// Metadata about overrides blocks in the configuration chain.
    ///
    /// Each entry corresponds to an override block, with information about
    /// which patterns it matches and where it came from in the config chain.
    override_metadata: Vec<OverrideProvenanceMetadata>,

    /// Path to the base configuration file that started the configuration chain.
    ///
    /// This is typically the `biome.json` file in the project root or a parent directory.
    base_config_path: Option<Utf8PathBuf>,
}

impl ProvenanceIndex {
    /// Creates a new `ProvenanceIndex` and sorts entries by merge order.
    ///
    /// # Arguments
    ///
    /// * `entries` - All provenance entries to store
    /// * `override_metadata` - Metadata about override blocks
    /// * `base_config_path` - Path to the base configuration file
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use biome_service::workspace::provenance::ProvenanceIndex;
    ///
    /// let index = ProvenanceIndex::build(
    ///     vec![entry1, entry2, entry3],
    ///     vec![override_meta],
    ///     Some("/path/to/biome.json".into())
    /// );
    /// ```
    pub fn build(
        mut entries: Vec<ProvenanceEntry>,
        override_metadata: Vec<OverrideProvenanceMetadata>,
        base_config_path: Option<Utf8PathBuf>,
    ) -> Self {
        // Sort entries by merge_order (ascending)
        // This ensures that when querying, we can iterate in order and the
        // last matching entry (highest merge_order) wins
        entries.sort_by_key(|entry| entry.merge_order);

        Self {
            entries,
            override_metadata,
            base_config_path,
        }
    }

    /// Returns a reference to all provenance entries, sorted by merge order.
    pub fn entries(&self) -> &[ProvenanceEntry] {
        &self.entries
    }

    /// Returns a reference to the override metadata.
    pub fn override_metadata(&self) -> &[OverrideProvenanceMetadata] {
        &self.override_metadata
    }

    /// Returns the path to the base configuration file, if available.
    pub fn base_config_path(&self) -> Option<&Utf8PathBuf> {
        self.base_config_path.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_configuration::provenance::{FieldQuery, ProvenanceSource};
    use biome_json_parser::JsonParserOptions;
    use biome_json_syntax::JsonRoot;
    use biome_rowan::{AstPtr, TextRange, TextSize};
    use camino::Utf8PathBuf;

    fn create_test_entry(merge_order: u64, field_name: &str) -> ProvenanceEntry {
        // Create a minimal JSON document for testing
        let json = format!(r#"{{"{}": "value"}}"#, field_name);
        let parsed = biome_json_parser::parse_json(&json, JsonParserOptions::default());
        let root: JsonRoot = parsed.tree();

        // Get the value node to create AstPtr
        let value = root.value().expect("test JSON should have a value");

        let value_ptr = AstPtr::new(&value);

        // Create a simple text range for testing
        let range = TextRange::new(TextSize::from(0), TextSize::from(10));

        let field_query = FieldQuery::new();
        let source = ProvenanceSource::base_config(Utf8PathBuf::from("/test/biome.json"));

        ProvenanceEntry::new(field_query, source, value_ptr, range, merge_order)
    }

    #[test]
    fn test_build_sorts_entries_by_merge_order() {
        // Create entries with different merge orders (not in order)
        let entry1 = create_test_entry(10, "field1");
        let entry2 = create_test_entry(5, "field2");
        let entry3 = create_test_entry(15, "field3");

        let index = ProvenanceIndex::build(vec![entry1, entry2, entry3], vec![], None);

        // Verify entries are sorted by merge_order
        let entries = index.entries();
        assert_eq!(entries.len(), 3);
        assert_eq!(entries[0].merge_order, 5);
        assert_eq!(entries[1].merge_order, 10);
        assert_eq!(entries[2].merge_order, 15);
    }

    #[test]
    fn test_entries_accessor() {
        let entry1 = create_test_entry(1, "field1");
        let entry2 = create_test_entry(2, "field2");

        let index = ProvenanceIndex::build(vec![entry1, entry2], vec![], None);

        let entries = index.entries();
        assert_eq!(entries.len(), 2);
    }

    #[test]
    fn test_base_config_path() {
        let index = ProvenanceIndex::build(
            vec![],
            vec![],
            Some(Utf8PathBuf::from("/path/to/biome.json")),
        );

        assert_eq!(
            index.base_config_path(),
            Some(&Utf8PathBuf::from("/path/to/biome.json"))
        );
    }

    #[test]
    fn test_base_config_path_none() {
        let index = ProvenanceIndex::build(vec![], vec![], None);
        assert_eq!(index.base_config_path(), None);
    }

    #[test]
    fn test_override_metadata() {
        let source = ProvenanceSource::base_config(Utf8PathBuf::from("/test/biome.json"));
        let metadata = OverrideProvenanceMetadata::new(source, 0, vec![], 1);

        let index = ProvenanceIndex::build(vec![], vec![metadata], None);

        assert_eq!(index.override_metadata().len(), 1);
        assert_eq!(index.override_metadata()[0].index, 0);
    }

    #[test]
    fn test_empty_index() {
        let index = ProvenanceIndex::build(vec![], vec![], None);

        assert_eq!(index.entries().len(), 0);
        assert_eq!(index.override_metadata().len(), 0);
        assert_eq!(index.base_config_path(), None);
    }
}
