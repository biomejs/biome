use biome_deserialize::{Deserializable, DeserializationContext, Text};
use biome_deserialize_macros::Deserializable;
use biome_glob::{CandidatePath, Glob};
use biome_resolver::is_builtin_node_module;
use biome_string_case::comparable_token::ComparableToken;

use super::import_source::{ImportSource, ImportSourceKind};

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
    pub fn index(&self, candidate: &ImportCandidate<'_>) -> u16 {
        candidate.source.as_str();
        self.0
            .iter()
            .position(|group| group.contains(candidate))
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

pub struct ImportCandidate<'a> {
    pub has_type_token: bool,
    pub source: ImportSourceCandidate<'a>,
}
impl ImportCandidate<'_> {
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

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
enum ImportGroup {
    #[serde(rename = ":BLANK_LINE:")]
    BlankLine,
    Matcher(GroupMatcher),
    MatcherList(Box<[GroupMatcher]>),
}
impl ImportGroup {
    fn contains(&self, candidate: &ImportCandidate<'_>) -> bool {
        match self {
            Self::BlankLine => false,
            Self::Matcher(matcher) => matcher.is_match(candidate),
            Self::MatcherList(matchers) => candidate.matches_with_exceptions(matchers.iter()),
        }
    }
}
impl Deserializable for ImportGroup {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl biome_deserialize::DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        match value.visitable_type() {
            Some(biome_deserialize::DeserializableType::Array) => {
                Deserializable::deserialize(ctx, value, name).map(Self::MatcherList)
            }
            Some(biome_deserialize::DeserializableType::Map) => {
                Deserializable::deserialize(ctx, value, name)
                    .map(GroupMatcher::Import)
                    .map(Self::Matcher)
            }
            _ => {
                let value_text = Text::deserialize(ctx, value, name)?;
                if value_text.text() == ":BLANK_LINE:" {
                    Some(Self::BlankLine)
                } else {
                    Deserializable::deserialize(ctx, value, name)
                        .map(GroupMatcher::Source)
                        .map(Self::Matcher)
                }
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum GroupMatcher {
    Import(ImportMatcher),
    Source(SourceMatcher),
}
impl GroupMatcher {
    pub fn is_negated(&self) -> bool {
        match self {
            Self::Import(_) => false,
            Self::Source(matcher) => matcher.is_negated(),
        }
    }

    pub fn is_match(&self, candidate: &ImportCandidate<'_>) -> bool {
        match self {
            Self::Import(matcher) => matcher.is_match(candidate),
            Self::Source(matcher) => matcher.is_match(&candidate.source),
        }
    }

    pub fn is_raw_match(&self, candidate: &ImportCandidate<'_>) -> bool {
        match self {
            Self::Import(matcher) => matcher.is_match(candidate),
            Self::Source(matcher) => matcher.is_raw_match(&candidate.source),
        }
    }
}
impl Deserializable for GroupMatcher {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl biome_deserialize::DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        if value.visitable_type() == Some(biome_deserialize::DeserializableType::Map) {
            Deserializable::deserialize(ctx, value, name).map(Self::Import)
        } else {
            Deserializable::deserialize(ctx, value, name).map(Self::Source)
        }
    }
}

#[derive(
    Clone, Debug, Default, Eq, PartialEq, Deserializable, serde::Deserialize, serde::Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ImportMatcher {
    r#type: Option<bool>,
    source: Option<SourcesMatcher>,
}
impl ImportMatcher {
    pub fn is_match(&self, candidate: &ImportCandidate<'_>) -> bool {
        let matches_type = self
            .r#type
            .is_none_or(|r#type| candidate.has_type_token == r#type);
        matches_type
            && self
                .source
                .as_ref()
                .is_none_or(|src| src.is_match(&candidate.source))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum SourcesMatcher {
    Matcher(SourceMatcher),
    MatcherList(Box<[SourceMatcher]>),
}
impl Deserializable for SourcesMatcher {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl biome_deserialize::DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        if value.visitable_type() == Some(biome_deserialize::DeserializableType::Array) {
            Deserializable::deserialize(ctx, value, name).map(Self::MatcherList)
        } else {
            Deserializable::deserialize(ctx, value, name).map(Self::Matcher)
        }
    }
}
impl SourcesMatcher {
    /// Tests whether the given `candidate` matches this matcher.
    pub fn is_match(&self, candidate: &ImportSourceCandidate) -> bool {
        match self {
            Self::Matcher(matcher) => candidate.matches(matcher),
            Self::MatcherList(matchers) => candidate.matches_with_exceptions(matchers),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum SourceMatcher {
    Predefined(NegatablePredefinedSourceMatcher),
    Glob(Box<ImportSourceGlob>),
}
impl SourceMatcher {
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
impl biome_deserialize::Deserializable for SourceMatcher {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl biome_deserialize::DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        let text = biome_deserialize::Text::deserialize(ctx, value, name)?;
        if text.ends_with(':') && (text.starts_with(':') || text.starts_with("!:")) {
            text.parse().ok().map(SourceMatcher::Predefined)
        } else {
            Deserializable::deserialize(ctx, value, name).map(SourceMatcher::Glob)
        }
    }
}

#[derive(Clone, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(try_from = "String", into = "String")]
pub struct NegatablePredefinedSourceMatcher {
    is_negated: bool,
    matcher: PredefinedSourceMatcher,
}
impl NegatablePredefinedSourceMatcher {
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
impl std::fmt::Display for NegatablePredefinedSourceMatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let negation = if self.is_negated { "!" } else { "" };
        let matcher_repr = &self.matcher;
        write!(f, "{negation}{matcher_repr}")
    }
}
impl std::fmt::Debug for NegatablePredefinedSourceMatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
impl From<NegatablePredefinedSourceMatcher> for String {
    fn from(value: NegatablePredefinedSourceMatcher) -> Self {
        value.to_string()
    }
}
impl std::str::FromStr for NegatablePredefinedSourceMatcher {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (is_negated, value) = if let Some(stripped) = value.strip_prefix('!') {
            (true, stripped)
        } else {
            (false, value)
        };
        let matcher = PredefinedSourceMatcher::from_str(value)?;
        Ok(Self {
            is_negated,
            matcher,
        })
    }
}
impl TryFrom<String> for NegatablePredefinedSourceMatcher {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}
#[cfg(feature = "schema")]
impl schemars::JsonSchema for NegatablePredefinedSourceMatcher {
    fn schema_name() -> String {
        "NegatablePredefinedSourceMatcher".to_string()
    }
    fn json_schema(generator: &mut schemars::r#gen::SchemaGenerator) -> schemars::schema::Schema {
        let schema = PredefinedSourceMatcher::json_schema(generator);
        let mut schema_object = schema.into_object();
        // Add negated variants
        if let Some(enum_values) = &mut schema_object.enum_values {
            for index in 0..enum_values.len() {
                if let Some(val) = enum_values[index].as_str() {
                    enum_values.push(format!("!{val}").into());
                }
            }
        }
        schema_object.into()
    }
}

#[derive(Clone, Debug, Deserializable, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum PredefinedSourceMatcher {
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
impl PredefinedSourceMatcher {
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
                    || (source_kind == ImportSourceKind::Package && is_builtin_node_module(source))
            }
            Self::Package => source_kind == ImportSourceKind::Package,
            Self::Path => source_kind == ImportSourceKind::Path,
            Self::ProtocolPackage => source_kind == ImportSourceKind::ProtocolPackage,
            Self::Url => source_kind == ImportSourceKind::Url,
        }
    }
}
impl std::fmt::Display for PredefinedSourceMatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = match self {
            // Don't forget to update `impl std::str::FromStr for PredefinedSourceMatcher`
            Self::Alias => ":ALIAS:",
            Self::Bun => ":BUN:",
            Self::Node => ":NODE:",
            Self::Package => ":PACKAGE:",
            Self::ProtocolPackage => ":PACKAGE_WITH_PROTOCOL:",
            Self::Path => ":PATH:",
            Self::Url => ":URL:",
        };
        f.write_str(repr)
    }
}
impl std::str::FromStr for PredefinedSourceMatcher {
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
    fn is_negated(&self) -> bool {
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
            path_candidate: CandidatePath::new(source.inner().token.text()),
        }
    }

    pub fn source_kund(&self) -> ImportSourceKind {
        self.source.kind()
    }

    /// Returns the original string of this import source.
    pub fn as_str(&self) -> &'a str {
        self.source.inner().token.text()
    }

    /// Tests whether the current path matches `matcher`.
    pub fn matches(&self, matcher: &SourceMatcher) -> bool {
        matcher.is_match(self)
    }

    /// Match against a list of matchers where negated matchers are handled as exceptions.
    /// Returns `default` if there is no matchers that match.
    pub fn matches_with_exceptions<'b, I>(&self, matchers: I) -> bool
    where
        I: IntoIterator<Item = &'b SourceMatcher>,
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
