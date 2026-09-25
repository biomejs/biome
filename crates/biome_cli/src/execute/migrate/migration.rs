use std::ops::{Deref, DerefMut};

use super::ignorefile;
use biome_analyze::RuleSource;
use biome_console::{fmt::Display, markup};
use biome_deserialize::{
    Deserializable, DeserializableType, DeserializableValue, DeserializationContext, Merge,
};
use std::vec;

#[derive(Clone, Debug, Default)]
pub(crate) struct MigrationOptions {
    /// Migrate inspired rules from eslint and its plugins?
    pub(crate) include_inspired: bool,
    /// Migrate nursery rules from eslint and its plugins?
    pub(crate) include_nursery: bool,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum RuleMigrationResult {
    /// A rule that has been migrated.
    Migrated,
    /// A rule that could be migrated if `--include-inspired` was passed
    Inspired,
    /// A rule that could be migrated if `--include-nursery` was passed
    Nursery,
    /// An unsupported rule
    Unsupported,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) struct UnsupportedRule(pub RuleSource<'static>, pub UnsupportedRuleReason);

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) enum UnsupportedRuleReason {
    /// The rule is stylistic and is fundamentally incompatible with the formatter, and there's no formatter option to adjust its behavior.
    ///
    /// This is for rules that enforce formatting that are at odds with Biome's formatting decisions.
    Stylistic,
    /// The formatter completely covers the functionality that the rule is meant to enforce (assuming default rule options).
    ///
    /// The rule is therefore redundant when using the formatter, and losing the rule does not reduce code quality.
    FormatterCovers,
    /// The functionality is covered by a Biome formatter option.
    FormatterOption(&'static str),
    /// The rule belongs to a known source, but it is not yet implemented in Biome.
    KnownSourceNotImplemented,
    /// The rule belongs to an unknown source, and is therefore not implemented in Biome.
    UnknownSource,
    /// The rule is covered by a different rule, and is therefore not implemented as its own rule in Biome.
    CoveredByRule(&'static str),
}

impl Display for UnsupportedRuleReason {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter) -> std::io::Result<()> {
        match self {
            Self::Stylistic => {
                fmt.write_markup(markup! { "Stylistic, incompatible with formatter." })
            }
            Self::FormatterCovers => {
                fmt.write_markup(markup! { "Redundant, completely covered by Biome's formatter." })
            }
            Self::FormatterOption(option) => fmt.write_markup(
                markup! { "Covered by Biome's "<Emphasis>{option}</Emphasis>" formatter option." },
            ),
            Self::KnownSourceNotImplemented => {
                fmt.write_markup(markup! { "Known source, not yet implemented." })
            }
            Self::UnknownSource => fmt.write_markup(markup! {
                "These rules originate from an eslint plugin or other tool that Biome doesn't know about."
            }),
            Self::CoveredByRule(rule) => fmt.write_markup(markup! {
                "Covered by the "<Emphasis>{rule}</Emphasis>" rule."
            }),
        }
    }
}

pub(crate) fn to_biome_includes(
    files: &[impl AsRef<str>],
    ignores: &[impl AsRef<str>],
) -> Vec<biome_glob::NormalizedGlob> {
    let mut includes: Vec<biome_glob::NormalizedGlob> = Vec::new();
    if !files.is_empty() {
        includes.extend(files.iter().filter_map(|glob| glob.as_ref().parse().ok()));
    }
    if !ignores.is_empty() {
        if includes.is_empty()
            && let Ok(glob) = "**".parse()
        {
            includes.push(glob);
        }
        includes.extend(ignores.iter().filter_map(|glob| {
            // ESLint supports negation: https://eslint.org/docs/latest/use/configure/ignore#unignoring-files-and-directories
            if let Some(rest) = glob.as_ref().strip_prefix('!') {
                rest.parse()
            } else {
                glob.as_ref()
                    .parse()
                    .map(|glob: biome_glob::NormalizedGlob| glob.negated())
            }
            .ok()
        }));
    }
    includes
}

#[derive(Debug, Default)]
pub(crate) struct ShorthandVec<T>(Vec<T>);
impl<T> Merge for ShorthandVec<T> {
    fn merge_with(&mut self, mut other: Self) {
        self.0.append(&mut other.0);
    }
}
impl<T> From<T> for ShorthandVec<T> {
    fn from(value: T) -> Self {
        Self(vec![value])
    }
}
impl<T> Deref for ShorthandVec<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for ShorthandVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<T> IntoIterator for ShorthandVec<T> {
    type Item = T;
    type IntoIter = vec::IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
impl<T: Deserializable> Deserializable for ShorthandVec<T> {
    fn deserialize(
        ctx: &mut dyn DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        Some(Self(
            if value.visitable_type()? == DeserializableType::Array {
                Deserializable::deserialize(ctx, value, name)?
            } else {
                Vec::from_iter([Deserializable::deserialize(ctx, value, name)?])
            },
        ))
    }
}

#[derive(Debug, Default)]
pub(crate) struct IgnorePattern(pub(crate) Box<str>);
impl Deref for IgnorePattern {
    type Target = Box<str>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl AsRef<str> for IgnorePattern {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}
impl biome_deserialize::Deserializable for IgnorePattern {
    fn deserialize(
        ctx: &mut dyn DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        let s = biome_deserialize::Text::deserialize(ctx, value, name)?;
        Some(Self(ignorefile::convert_pattern(s.text()).into_boxed_str()))
    }
}
