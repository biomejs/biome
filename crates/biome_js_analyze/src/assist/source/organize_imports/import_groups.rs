use biome_deserialize::{Deserializable, DeserializationContext, Text};
use biome_deserialize_macros::Deserializable;
use biome_glob::{CandidatePath, Glob};

use crate::globals::is_node_builtin_module;

use super::{
    comparable_token::ComparableToken,
    import_key::ImportInfo,
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
    pub fn index(&self, info: &ImportInfo) -> u16 {
        let candidate = ImportSourceCandidate::new(&info.source);
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
                groups
                    .iter()
                    .any(|group| matches!(group, ImportGroup::BlankLine,))
            })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum ImportGroup {
    #[serde(rename = ":BLANK_LINE:")]
    BlankLine,
    Matcher(GroupMatcher),
    MatcherList(Box<[GroupMatcher]>),
}
impl ImportGroup {
    pub fn contains(&self, candidate: &ImportSourceCandidate) -> bool {
        match self {
            ImportGroup::BlankLine => false,
            ImportGroup::Matcher(glob) => glob.is_match(candidate),
            ImportGroup::MatcherList(matchers) => {
                candidate.matches_with_exceptions(matchers.iter())
            }
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
            if value_text.text() == ":BLANK_LINE:" {
                Some(ImportGroup::BlankLine)
            } else {
                Deserializable::deserialize(ctx, value, name).map(ImportGroup::Matcher)
            }
        } else {
            Deserializable::deserialize(ctx, value, name).map(ImportGroup::MatcherList)
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum GroupMatcher {
    Predefined(NegatablePredefinedGroupMatcher),
    Glob(Box<ImportSourceGlob>),
}
impl GroupMatcher {
    pub fn is_negated(&self) -> bool {
        match self {
            Self::Predefined(matcher) => matcher.is_negated(),
            Self::Glob(glob) => glob.is_negated(),
        }
    }

    /// Tests whether the given `candidate` matches this matcher.
    pub fn is_match(&self, candidate: &ImportSourceCandidate) -> bool {
        match self {
            Self::Predefined(matcher) => matcher.is_match(candidate),
            Self::Glob(glob) => glob.is_match(candidate),
        }
    }

    /// Tests whether the given `candidate` matches this matcher, ignoring the negation.
    pub fn is_raw_match(&self, candidate: &ImportSourceCandidate) -> bool {
        match self {
            Self::Predefined(matcher) => matcher.is_raw_match(candidate),
            Self::Glob(glob) => glob.is_raw_match(candidate),
        }
    }
}
impl biome_deserialize::Deserializable for GroupMatcher {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl biome_deserialize::DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        let text = biome_deserialize::Text::deserialize(ctx, value, name)?;
        if text.ends_with(':') && (text.starts_with(':') || text.starts_with("!:")) {
            text.parse().ok().map(GroupMatcher::Predefined)
        } else {
            Deserializable::deserialize(ctx, value, name).map(GroupMatcher::Glob)
        }
    }
}

#[derive(Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(try_from = "String", into = "String")]
pub struct NegatablePredefinedGroupMatcher {
    is_negated: bool,
    matcher: PredefinedGroupMatcher,
}
impl NegatablePredefinedGroupMatcher {
    pub fn is_negated(&self) -> bool {
        self.is_negated
    }

    /// Tests whether the given `candidate` matches this matcher.
    pub fn is_match(&self, candidate: &ImportSourceCandidate) -> bool {
        self.is_raw_match(candidate) != self.is_negated
    }

    /// Tests whether the given `candidate` matches this matcher, ignoring the negation.
    pub fn is_raw_match(&self, candidate: &ImportSourceCandidate) -> bool {
        self.matcher.is_match(candidate)
    }
}
impl std::fmt::Display for NegatablePredefinedGroupMatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let negation = if self.is_negated { "!" } else { "" };
        let matcher_repr = &self.matcher;
        write!(f, "{negation}{matcher_repr}")
    }
}
impl std::fmt::Debug for NegatablePredefinedGroupMatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
impl From<NegatablePredefinedGroupMatcher> for String {
    fn from(value: NegatablePredefinedGroupMatcher) -> Self {
        value.to_string()
    }
}
impl std::str::FromStr for NegatablePredefinedGroupMatcher {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (is_negated, value) = if let Some(stripped) = value.strip_prefix('!') {
            (true, stripped)
        } else {
            (false, value)
        };
        let matcher = PredefinedGroupMatcher::from_str(value)?;
        Ok(Self {
            is_negated,
            matcher,
        })
    }
}
impl TryFrom<String> for NegatablePredefinedGroupMatcher {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}
#[cfg(feature = "schema")]
impl schemars::JsonSchema for NegatablePredefinedGroupMatcher {
    fn schema_name() -> String {
        "PredefinedGroupMatcher".to_string()
    }
    fn json_schema(generator: &mut schemars::r#gen::SchemaGenerator) -> schemars::schema::Schema {
        String::json_schema(generator)
    }
}

#[derive(Clone, Debug, Deserializable, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum PredefinedGroupMatcher {
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
impl PredefinedGroupMatcher {
    fn is_match(&self, candidate: &ImportSourceCandidate) -> bool {
        let source_kind = candidate.source_kund();
        let source = candidate.as_str();
        match self {
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
impl std::fmt::Display for PredefinedGroupMatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = match self {
            // Don't forget to update `impl std::str::FromStr for PredefinedImportGroup`
            PredefinedGroupMatcher::Alias => "ALIAS",
            PredefinedGroupMatcher::Bun => "BUN",
            PredefinedGroupMatcher::Node => "NODE",
            PredefinedGroupMatcher::Package => "PACKAGE",
            PredefinedGroupMatcher::ProtocolPackage => "PACKAGE_WITH_PROTOCOL",
            PredefinedGroupMatcher::Path => "PATH",
            PredefinedGroupMatcher::Url => "URL",
        };
        f.write_str(repr)
    }
}
impl std::str::FromStr for PredefinedGroupMatcher {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            ":ALIAS:" => Ok(Self::Alias),
            ":BUN:" => Ok(Self::Bun),
            ":NODE:" => Ok(Self::Node),
            ":PACKAGE:" => Ok(Self::Package),
            ":PACKAGE_WITH_PROTOCOL:" => Ok(Self::ProtocolPackage),
            ":PATH:" => Ok(Self::Path),
            ":URL:" => Ok(Self::Url),
            _ => Err("invalid predefined group"),
        }
    }
}

/// Glob to match against import sources.
#[derive(Clone, Debug, Deserializable, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ImportSourceGlob(Glob);
impl ImportSourceGlob {
    pub fn is_negated(&self) -> bool {
        self.0.is_negated()
    }

    /// Tests whether the given import source matches this pattern or not.
    pub fn is_match(&self, import_source: &ImportSourceCandidate) -> bool {
        import_source.path_candidate.matches(&self.0)
    }

    /// Tests whether the given import source matches this pattern or not.
    pub fn is_raw_match(&self, import_source: &ImportSourceCandidate) -> bool {
        import_source.path_candidate.matches_raw(&self.0)
    }
}

/// A candidate import source for matching.
///
/// Constructing candidates has a very small cost associated with it.
/// The cost is amortized by matching against several import source globs.
#[derive(Debug)]
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

    /// Tests whether the current path matches `matcher`.
    pub fn matches(&self, matcher: &GroupMatcher) -> bool {
        matcher.is_match(self)
    }

    /// Match against a list of matchers where negated matchers are handled as exceptions.
    /// Returns `default` if there is no matchers that match.
    pub fn matches_with_exceptions<'b, I>(&self, matchers: I) -> bool
    where
        I: IntoIterator<Item = &'b GroupMatcher>,
        I::IntoIter: DoubleEndedIterator,
    {
        // Iterate in reverse order to avoid unnecessary glob matching.
        for matcher in matchers.into_iter().rev() {
            if matcher.is_raw_match(self) {
                return !matcher.is_negated();
            }
        }
        false
    }
}
