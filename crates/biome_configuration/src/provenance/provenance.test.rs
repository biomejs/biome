//! Integration tests demonstrating that the provenance infrastructure is ready.
//!
//! These tests verify that `ProvenanceAwareDeserializationContext` can be created
//! and used. Actual provenance capture requires Deserializable types to call
//! provenance methods during deserialization, which will be added in a future phase.

#[cfg(test)]
mod tests {
    use crate::provenance::{ProvenanceAwareDeserializationContext, ProvenanceSource};
    use biome_deserialize::{DefaultDeserializationContext, DeserializationContext};
    use biome_json_parser::{JsonParserOptions, parse_json};
    use camino::Utf8PathBuf;

    #[test]
    fn test_provenance_aware_context_creation() {
        // Parse JSON
        let json = r#"{"formatter": {"indentWidth": 4}}"#;
        let parsed = parse_json(json, JsonParserOptions::default());
        let root = parsed.tree();

        // Create provenance-aware context
        let source = ProvenanceSource::base_config(Utf8PathBuf::from("/test/biome.json"));
        let base_context = DefaultDeserializationContext::new(&root, "test");
        let context = ProvenanceAwareDeserializationContext::new(base_context, source, 0);

        // Verify context wrapping works
        assert_eq!(context.id(), Some("test"));
        assert!(context.root().value().is_ok());

        // Take entries (will be empty until Deserializable types call provenance methods)
        let entries = context.take_entries();
        assert_eq!(entries.len(), 0, "No provenance calls made yet");
    }

    #[test]
    fn test_provenance_context_with_different_sources() {
        let json = r#"{"lineWidth": 120}"#;
        let parsed = parse_json(json, JsonParserOptions::default());
        let root = parsed.tree();

        // Test BaseConfig source
        let source1 = ProvenanceSource::base_config(Utf8PathBuf::from("/test/biome.json"));
        let ctx1 = DefaultDeserializationContext::new(&root, "base");
        let mut prov_ctx1 = ProvenanceAwareDeserializationContext::new(ctx1, source1, 0);
        assert!(prov_ctx1.provenance().is_some());

        // Test ExtendedConfig source
        let source2 = ProvenanceSource::extended_config(
            Utf8PathBuf::from("/test/base.json"),
            vec![
                Utf8PathBuf::from("/test/biome.json"),
                Utf8PathBuf::from("/test/base.json"),
            ],
        );
        let ctx2 = DefaultDeserializationContext::new(&root, "extended");
        let mut prov_ctx2 = ProvenanceAwareDeserializationContext::new(ctx2, source2, 1);
        assert!(prov_ctx2.provenance().is_some());

        // Both contexts should work correctly
        let entries1 = prov_ctx1.take_entries();
        let entries2 = prov_ctx2.take_entries();
        assert_eq!(entries1.len(), 0);
        assert_eq!(entries2.len(), 0);
    }

    #[test]
    fn test_merge_order_tracking() {
        // Test that we can create contexts with different merge orders
        // In a real extends chain: base.json (order=0), extended.json (order=1), biome.json (order=2)
        let json = r#"{"test": true}"#;
        let parsed = parse_json(json, JsonParserOptions::default());
        let root = parsed.tree();

        for merge_order in 0..3 {
            let source = ProvenanceSource::base_config(Utf8PathBuf::from("/test/config.json"));
            let ctx = DefaultDeserializationContext::new(&root, "test");
            let prov_ctx = ProvenanceAwareDeserializationContext::new(ctx, source, merge_order);

            // Verify the context was created successfully
            assert_eq!(prov_ctx.id(), Some("test"));

            // When actual deserialization happens, entries will have the correct merge_order
            let entries = prov_ctx.take_entries();
            assert_eq!(entries.len(), 0);
        }
    }

    #[test]
    fn test_context_delegates_to_inner() {
        let json = r#"{"nested": {"field": "value"}}"#;
        let parsed = parse_json(json, JsonParserOptions::default());
        let root = parsed.tree();

        let source = ProvenanceSource::base_config(Utf8PathBuf::from("/test/biome.json"));
        let base_context = DefaultDeserializationContext::new(&root, "delegation_test");
        let context = ProvenanceAwareDeserializationContext::new(base_context, source, 0);

        // Test that context properly delegates to inner context
        assert_eq!(context.id(), Some("delegation_test"));

        // Access root through context and verify it works
        let root_from_context = context.root();
        assert!(
            root_from_context.value().is_ok(),
            "Root should have a value"
        );
    }
}
