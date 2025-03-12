use biome_deserialize::{Deserializable, DeserializationContext, Text};
use biome_deserialize_macros::Deserializable;
use biome_glob::{CandidatePath, Glob};

use crate::globals::is_node_builtin_module;

#[derive(
    Clone, Debug, Default, Deserializable, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ImportGroups(Box<[ImportGroup]>);
impl ImportGroups {
    /// Returns the index of the first group containing `candidate`.
    /// If no group contains `candidate`, then the returned value corresponds to the index of the implicit group.
    /// The index of the implicit group correspond to the number of groups.
    pub fn index(&self, candidate: &ImportSourceCandidate) -> usize {
        self.0
            .iter()
            .position(|group| group.contains(candidate))
            .unwrap_or(self.0.len())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum ImportGroup {
    Predefined(PredefinedImportGroup),
    Glob(Box<ImportSourceGlob>),
    GlobList(Box<[ImportSourceGlob]>),
}
impl ImportGroup {
    pub fn contains(&self, candidate: &ImportSourceCandidate) -> bool {
        match self {
            ImportGroup::Predefined(predefined) => predefined.contains(candidate),
            ImportGroup::Glob(glob) => glob.is_match(candidate),
            ImportGroup::GlobList(globs) => candidate
                .path_candidate
                .matches_with_exceptions(globs.iter().map(|glob| &glob.0)),
        }
    }
}
impl Deserializable for ImportGroup {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl biome_deserialize::DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        if value.visitable_type() == Some(biome_deserialize::DeserializableType::Str) {
            let value_text = Text::deserialize(ctx, value, name)?;
            if value_text.starts_with(':') && value_text.ends_with(':') {
                Deserializable::deserialize(ctx, value, name).map(ImportGroup::Predefined)
            } else {
                Deserializable::deserialize(ctx, value, name).map(ImportGroup::Glob)
            }
        } else {
            Deserializable::deserialize(ctx, value, name).map(ImportGroup::GlobList)
        }
    }
}

#[derive(Clone, Debug, Deserializable, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum PredefinedImportGroup {
    #[serde(rename = ":BUN:")]
    Bun,
    #[serde(rename = ":NODE:")]
    Node,
}
impl PredefinedImportGroup {
    fn contains(&self, candidate: &ImportSourceCandidate) -> bool {
        let import_source = candidate.as_str();
        match self {
            Self::Bun => import_source == "bun" || import_source.starts_with("bun:"),
            Self::Node => {
                import_source.starts_with("node:") || is_node_builtin_module(import_source)
            }
        }
    }
}

/// Glob to match against import sources.
#[derive(Clone, Debug, Deserializable, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ImportSourceGlob(Glob);
impl ImportSourceGlob {
    /// Tests whether the given import source matches this pattern or not.
    pub fn is_match(&self, import_source: &ImportSourceCandidate) -> bool {
        import_source.path_candidate.matches(&self.0)
    }
}

/// A candidate import source for matching.
///
/// Constructing candidates has a very small cost associated with it.
/// The cost is amortized by matching against several import source globs.
pub struct ImportSourceCandidate<'a> {
    import_source: &'a str,
    path_candidate: CandidatePath<'a>,
}
impl<'a> ImportSourceCandidate<'a> {
    /// Create a new candidate for matching from the given path.
    pub fn new(import_source: &'a str) -> Self {
        Self {
            import_source,
            path_candidate: CandidatePath::new(import_source),
        }
    }

    /// Returns the original string of this import source.
    pub fn as_str(&self) -> &str {
        self.import_source
    }
}
