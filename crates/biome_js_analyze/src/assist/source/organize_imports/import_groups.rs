use biome_deserialize::{Deserializable, DeserializationContext, Text};
use biome_deserialize_macros::Deserializable;
use biome_glob::{CandidatePath, Glob};

use crate::globals::is_node_builtin_module;

use super::{
    comparable_token::ComparableToken,
    import_source::{ImportSource, ImportSourceKind},
};

#[derive(
    Clone, Debug, Default, Deserializable, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ImportGroups(Box<[ImportGroup]>);
impl ImportGroups {
    /// Returns `true` if no explicit group is set.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the index of the first group containing `candidate`.
    /// If no group contains `candidate`, then the returned value corresponds to the index of the implicit group.
    /// The index of the implicit group correspond to the number of groups.
    pub fn index(&self, source: &ImportSource<ComparableToken>) -> u16 {
        let candidate = ImportSourceCandidate::new(source);
        self.0
            .iter()
            .position(|group| group.contains(&candidate))
            .unwrap_or(self.0.len()) as u16
    }

    /// Returns how many blank lines must separate `first_group` and `second_group`.
    pub fn separated_by_blank_line(&self, first_group: u16, second_group: u16) -> bool {
        self.0
            .get((first_group as usize)..(second_group as usize))
            .is_some_and(|groups| {
                groups.iter().any(|group| {
                    matches!(
                        group,
                        ImportGroup::Predefined(PredefinedImportGroup::BlankLine)
                    )
                })
            })
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
    #[serde(rename = ":BLANK_LINE:")]
    BlankLine,
    #[serde(rename = ":ALIAS:")]
    Alias,
    #[serde(rename = ":BUN:")]
    Bun,
    #[serde(rename = ":NODE:")]
    Node,
    #[serde(rename = ":PACKAGE:")]
    Package,
    #[serde(rename = ":PACKAGE_WITH_PROTOCOL:")]
    ProtocolPackage,
    #[serde(rename = ":PATH:")]
    Path,
    #[serde(rename = ":URL:")]
    Url,
}
impl PredefinedImportGroup {
    fn contains(&self, candidate: &ImportSourceCandidate) -> bool {
        let source_kind = candidate.source_kund();
        let source = candidate.as_str();
        match self {
            Self::BlankLine => false,
            Self::Alias => source_kind == ImportSourceKind::Alias,
            Self::Bun => {
                (source_kind == ImportSourceKind::Package && source == "bun")
                    || (source_kind == ImportSourceKind::ProtocolPackage
                        && source.starts_with("bun:"))
            }
            Self::Node => {
                (source_kind == ImportSourceKind::ProtocolPackage && source.starts_with("node:"))
                    || (source_kind == ImportSourceKind::Package && is_node_builtin_module(source))
            }
            Self::Package => source_kind == ImportSourceKind::Package,
            Self::Path => source_kind == ImportSourceKind::Path,
            Self::ProtocolPackage => source_kind == ImportSourceKind::ProtocolPackage,
            Self::Url => source_kind == ImportSourceKind::Url,
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
    source: &'a ImportSource<ComparableToken>,
    path_candidate: CandidatePath<'a>,
}
impl<'a> ImportSourceCandidate<'a> {
    /// Create a new candidate for matching from the given path.
    pub fn new(source: &'a ImportSource<ComparableToken>) -> Self {
        Self {
            source,
            path_candidate: CandidatePath::new(source.inner().0.text()),
        }
    }

    pub fn source_kund(&self) -> ImportSourceKind {
        self.source.kind()
    }

    /// Returns the original string of this import source.
    pub fn as_str(&self) -> &'a str {
        self.source.inner().0.text()
    }
}
