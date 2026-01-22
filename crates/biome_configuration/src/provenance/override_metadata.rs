use super::ProvenanceSource;
use biome_glob::Glob;
use camino::Utf8Path;
use std::str::FromStr;

/// Metadata about an override pattern for lazy evaluation
/// Stored in ProvenanceIndex to enable file-specific queries
#[derive(Debug, Clone)]
pub struct OverrideProvenanceMetadata {
    /// Source of this override (which config file)
    pub source: ProvenanceSource,

    /// The index of this override in the overrides array
    /// Used by QueryVisitor to match entries like "overrides[N].field"
    pub index: usize,

    /// The compiled glob patterns for matching files
    pub matchers: Vec<GlobMatcher>,

    /// Merge order: when this override was encountered during config loading
    pub merge_order: u64,
}

impl OverrideProvenanceMetadata {
    /// Create new override metadata
    pub fn new(
        source: ProvenanceSource,
        index: usize,
        matchers: Vec<GlobMatcher>,
        merge_order: u64,
    ) -> Self {
        Self {
            source,
            index,
            matchers,
            merge_order,
        }
    }

    /// Check if this override applies to the given file path
    pub fn matches_file(&self, path: &Utf8Path) -> bool {
        self.matchers.iter().any(|m| m.matches(path))
    }
}

/// A glob matcher for file paths
#[derive(Debug, Clone)]
pub struct GlobMatcher {
    glob: Glob,
}

impl GlobMatcher {
    /// Create a new glob matcher
    pub fn new(glob: Glob) -> Self {
        Self { glob }
    }

    /// Check if the glob matches the given path
    pub fn matches(&self, path: &Utf8Path) -> bool {
        self.glob.is_match(path)
    }

    /// Get the underlying glob
    pub fn glob(&self) -> &Glob {
        &self.glob
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use camino::Utf8PathBuf;

    #[test]
    fn test_override_metadata_creation() {
        let source = ProvenanceSource::default();
        let matchers = vec![];

        let metadata = OverrideProvenanceMetadata::new(source, 0, matchers, 1);

        assert_eq!(metadata.index, 0);
        assert_eq!(metadata.merge_order, 1);
    }

    #[test]
    fn test_glob_matcher_basic() {
        // Test basic glob pattern matching
        let glob = Glob::from_str("*.test.js").expect("valid glob");
        let matcher = GlobMatcher::new(glob);

        let test_file = Utf8PathBuf::from("example.test.js");
        let normal_file = Utf8PathBuf::from("example.js");

        assert!(matcher.matches(&test_file));
        assert!(!matcher.matches(&normal_file));
    }

    #[test]
    fn test_override_metadata_matches_file() {
        let glob = Glob::from_str("*.test.js").expect("valid glob");
        let matchers = vec![GlobMatcher::new(glob)];

        let source = ProvenanceSource::default();
        let metadata = OverrideProvenanceMetadata::new(source, 0, matchers, 1);

        let test_file = Utf8PathBuf::from("example.test.js");
        let normal_file = Utf8PathBuf::from("example.js");

        assert!(metadata.matches_file(&test_file));
        assert!(!metadata.matches_file(&normal_file));
    }

    #[test]
    fn test_override_metadata_multiple_patterns() {
        let glob1 = Glob::from_str("*.test.js").expect("valid glob");
        let glob2 = Glob::from_str("*.spec.ts").expect("valid glob");
        let matchers = vec![GlobMatcher::new(glob1), GlobMatcher::new(glob2)];

        let source = ProvenanceSource::default();
        let metadata = OverrideProvenanceMetadata::new(source, 0, matchers, 1);

        let test_js = Utf8PathBuf::from("example.test.js");
        let spec_ts = Utf8PathBuf::from("example.spec.ts");
        let normal = Utf8PathBuf::from("example.js");

        assert!(metadata.matches_file(&test_js));
        assert!(metadata.matches_file(&spec_ts));
        assert!(!metadata.matches_file(&normal));
    }
}
