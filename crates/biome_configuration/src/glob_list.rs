use biome_deserialize::Merge;
use biome_deserialize_macros::Deserializable;
use biome_glob::NormalizedGlob;
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};
use std::ops::DerefMut;

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Bpaf, Deserializable)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
/// Normalized Biome glob pattern that strips `./` from the pattern.
pub struct GlobList(Vec<NormalizedGlob>);

impl GlobList {
    /// Creates an empty list
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Creates a new `Globs` instance from a vector of globs.
    pub fn from_vec(globs: Vec<NormalizedGlob>) -> Self {
        Self(globs)
    }

    pub fn as_slice(&self) -> &[NormalizedGlob] {
        self.0.as_slice()
    }
}

impl From<Vec<NormalizedGlob>> for GlobList {
    fn from(value: Vec<NormalizedGlob>) -> Self {
        Self(value)
    }
}

impl std::ops::Deref for GlobList {
    type Target = Vec<NormalizedGlob>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GlobList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut()
    }
}

impl Merge for GlobList {
    fn merge_with(&mut self, other: Self) {
        self.extend(other.0);
        self.sort();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_with_all_negated_globs_sorts_double_star_first() {
        // When all globs are negated, ** should be sorted to the beginning
        let mut globs1 = GlobList::from_vec(vec![
            "!foo/*.js".parse().unwrap(),
            "!bar/**".parse().unwrap(),
        ]);
        let globs2 = GlobList::from_vec(vec![
            "!baz.txt".parse().unwrap(),
            "**".parse().unwrap(), // Non-negated ** should come first
        ]);

        globs1.merge_with(globs2);

        // ** should be first since all other globs are negated
        assert_eq!(globs1.0.len(), 4);
        assert_eq!(globs1.0[0].to_string(), "**");
    }

    #[test]
    fn test_merge_with_mixed_globs_preserves_insertion_order() {
        // When there's at least one non-negated glob, preserve insertion order
        let mut globs1 = GlobList::from_vec(vec![
            "src/**/*.js".parse().unwrap(),
            "!test/**".parse().unwrap(),
        ]);
        let globs2 = GlobList::from_vec(vec![
            "**".parse().unwrap(),
            "!node_modules/**".parse().unwrap(),
        ]);

        globs1.merge_with(globs2);

        // Order should be preserved (not sorted)
        assert_eq!(globs1.0.len(), 4);
        assert_eq!(globs1.0[0].to_string(), "src/**/*.js");
        assert_eq!(globs1.0[1].to_string(), "!test/**");
        assert_eq!(globs1.0[2].to_string(), "**");
        assert_eq!(globs1.0[3].to_string(), "!node_modules/**");
    }

    #[test]
    fn test_merge_deduplicates_globs() {
        // Duplicates should be removed
        let mut globs1 = GlobList::from_vec(vec![
            "src/**/*.js".parse().unwrap(),
            "!test/**".parse().unwrap(),
        ]);
        let globs2 = GlobList::from_vec(vec![
            "src/**/*.js".parse().unwrap(), // Duplicate
            "dist/**".parse().unwrap(),
        ]);

        globs1.merge_with(globs2);

        // Should have 3 items (duplicate removed)
        assert_eq!(globs1.0.len(), 3);
        assert_eq!(globs1.0[0].to_string(), "src/**/*.js");
        assert_eq!(globs1.0[1].to_string(), "!test/**");
        assert_eq!(globs1.0[2].to_string(), "dist/**");
    }

    #[test]
    fn test_merge_empty_globs() {
        // Merging empty globs should work
        let mut globs1 = GlobList::new();
        let globs2 = GlobList::from_vec(vec!["src/**/*.js".parse().unwrap()]);

        globs1.merge_with(globs2);

        assert_eq!(globs1.0.len(), 1);
        assert_eq!(globs1.0[0].to_string(), "src/**/*.js");
    }

    #[test]
    fn test_merge_with_only_negated_double_star() {
        // When all globs are negated and there's only !**, it should not affect order
        let mut globs1 = GlobList::from_vec(vec![
            "!foo/*.js".parse().unwrap(),
            "!**".parse().unwrap(), // Negated **, not sorted first
        ]);
        let globs2 = GlobList::from_vec(vec!["!bar.txt".parse().unwrap()]);

        globs1.merge_with(globs2);

        // All negated, so should be sorted, but !** doesn't get special treatment
        assert_eq!(globs1.0.len(), 3);
        // Verify all are negated
        assert!(globs1.0.iter().all(|g| g.is_negated()));
    }

    #[test]
    fn test_double_star_ordering() {
        // Test that non-negated ** is ordered before other patterns
        let glob1: biome_glob::NormalizedGlob = "**".parse().unwrap();
        let glob2: biome_glob::NormalizedGlob = "src/**".parse().unwrap();
        let glob3: biome_glob::NormalizedGlob = "!**".parse().unwrap();

        assert!(glob1 < glob2); // ** comes before src/**
        assert!(glob1 < glob3); // ** comes before !**
    }

    #[test]
    fn test_deref_access() {
        // Test that Deref allows easy access to inner Vec
        let globs = GlobList::from_vec(vec![
            "src/**/*.js".parse().unwrap(),
            "!test/**".parse().unwrap(),
        ]);

        // Should be able to use Vec methods via Deref
        assert_eq!(globs.len(), 2);
        assert!(!globs.is_empty());
    }

    #[test]
    fn test_order_is_kept_when_no_all_star_is_present() {
        let mut globs1 = GlobList::from_vec(vec![
            "!test/**".parse().unwrap(),
            "src/**/*.js".parse().unwrap(),
            "foo/**/*.js".parse().unwrap(),
            "bar/**/*.js".parse().unwrap(),
        ]);
        let globs2 = GlobList::from_vec(vec!["dist/**".parse().unwrap()]);

        globs1.merge_with(globs2);

        // Should have 5 items
        assert_eq!(globs1.0.len(), 5);
        assert_eq!(globs1.0[0].to_string(), "!test/**");
        assert_eq!(globs1.0[1].to_string(), "src/**/*.js");
        assert_eq!(globs1.0[2].to_string(), "foo/**/*.js");
        assert_eq!(globs1.0[3].to_string(), "bar/**/*.js");
        assert_eq!(globs1.0[4].to_string(), "dist/**");
    }
}
