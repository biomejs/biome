use biome_deserialize_macros::{Deserializable, Merge};
use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;
use serde::{Deserialize, Serialize};

/// Options for the `useConsistentBooleanName` rule.
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseConsistentBooleanNameOptions {
    /// Prefixes to enable or disable, merged with the default prefixes.
    ///
    /// A prefix set to `true` is accepted on boolean names, and names that
    /// start with it must be booleans. A prefix set to `false` is disabled.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub prefixes: Option<IndexMap<Box<str>, bool, FxBuildHasher>>,

    /// Names that the rule doesn't check.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub ignore: Option<Box<[Box<str>]>>,
}

impl UseConsistentBooleanNameOptions {
    /// The prefixes that are enabled when [`Self::prefixes`] doesn't disable
    /// them, sorted alphabetically.
    pub const DEFAULT_PREFIXES: &[&str] = &[
        "are", "can", "did", "does", "has", "have", "is", "requires", "should", "was", "were",
        "will",
    ];

    /// Returns the enabled prefixes: the entries of [`Self::DEFAULT_PREFIXES`]
    /// that [`Self::prefixes`] doesn't set to `false`, followed by the
    /// entries of [`Self::prefixes`] set to `true` that aren't defaults.
    pub fn enabled_prefixes(&self) -> impl Iterator<Item = &str> {
        let prefixes = self.prefixes.as_ref();
        let defaults = Self::DEFAULT_PREFIXES
            .iter()
            .copied()
            .filter(move |prefix| prefixes.and_then(|map| map.get(*prefix)) != Some(&false));
        let custom = prefixes
            .into_iter()
            .flatten()
            .filter_map(|(prefix, enabled)| {
                (*enabled
                    && Self::DEFAULT_PREFIXES
                        .binary_search(&prefix.as_ref())
                        .is_err())
                .then_some(prefix.as_ref())
            });
        defaults.chain(custom)
    }

    /// Returns [`Self::ignore`] if it is set.
    /// Otherwise, returns an empty slice.
    pub fn ignore(&self) -> &[Box<str>] {
        self.ignore.as_deref().unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::UseConsistentBooleanNameOptions;

    #[test]
    fn default_prefixes_are_sorted() {
        for window in UseConsistentBooleanNameOptions::DEFAULT_PREFIXES.windows(2) {
            assert!(
                window[0] < window[1],
                "{:?} must come before {:?}",
                window[1],
                window[0]
            );
        }
    }
}
