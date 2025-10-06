use crate::node_semver::version::{Version, VersionError};
use std::fmt;
use std::str::FromStr;

/// A version range that can match multiple versions.
///
/// Ranges support complex expressions including OR operations, compound ranges,
/// and various comparison operators like tilde (`~`) and caret (`^`).
///
/// The range may contain alternative sets of constraints. A version satisfies
/// the range if it satisfies all constraints in at least one alternative set.
///
/// # Examples
///
/// ```
/// use biome_package::node_semver::{Range, Version};
///
/// let range: Range = "^1.2.3".parse().unwrap();
/// let version: Version = "1.5.0".parse().unwrap();
/// assert!(range.includes(&version));
///
/// let or_range: Range = "1.2.7 || >=1.2.9 <2.0.0".parse().unwrap();
/// let version1: Version = "1.2.7".parse().unwrap();
/// let version2: Version = "1.3.0".parse().unwrap();
/// assert!(or_range.includes(&version1));
/// assert!(or_range.includes(&version2));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Range {
    /// Range represented as a single set of comparators, all of which must be
    /// satisfied.
    ///
    /// An empty set of comparators matches everything.
    Comparators(Vec<Comparator>),

    /// Alternative sets of comparators that can satisfy this range.
    ///
    /// Each inner array represents comparators that must all be satisfied
    /// (AND), while the outer array represents different alternatives (OR).
    ///
    /// At least one alternative must be matched.
    Alternatives(Vec<Vec<Comparator>>),
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Comparators(comparators) if comparators.is_empty() => write!(f, "*"),
            Self::Comparators(comparators) => {
                comparators
                    .iter()
                    .enumerate()
                    .try_for_each(|(i, comparator)| {
                        if i > 0 {
                            write!(f, " ")?;
                        }

                        write!(f, "{comparator}")
                    })
            }
            Self::Alternatives(alternatives) => {
                alternatives
                    .iter()
                    .enumerate()
                    .try_for_each(|(i, comparators)| {
                        if i > 0 {
                            write!(f, "||")?;
                        }

                        for (i, comparator) in comparators.iter().enumerate() {
                            if i > 0 {
                                write!(f, " ")?;
                            }

                            write!(f, "{comparator}")?;
                        }

                        Ok(())
                    })
            }
        }
    }
}

/// A single version comparator within a range.
///
/// Combines an operator with a version to create constraints like `>=1.2.3` or `~2.0.0`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Comparator {
    /// The comparison operator to apply.
    pub operator: ComparatorOperator,
    /// The version to compare against.
    pub version: Version,
}

impl fmt::Display for Comparator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.operator, self.version)
    }
}

/// Operators used in [`Comparator`].
///
/// These are a subset of [`RangeOperator`] to allow for simpler comparator
/// logic.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComparatorOperator {
    /// Exact match (`=1.2.3` or `1.2.3`).
    Exact,
    /// Greater than (`>1.2.3`).
    GreaterThan,
    /// Greater than or equal to (`>=1.2.3`).
    GreaterThanOrEqual,
    /// Less than (`<1.2.3`).
    LessThan,
    /// Less than or equal to (`<=1.2.3`).
    LessThanOrEqual,
}

impl fmt::Display for ComparatorOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Exact => Ok(()),
            Self::GreaterThan => write!(f, ">"),
            Self::GreaterThanOrEqual => write!(f, ">="),
            Self::LessThan => write!(f, "<"),
            Self::LessThanOrEqual => write!(f, "<="),
        }
    }
}

/// Operators used in version range comparisons.
///
/// Each operator defines how versions are matched against a target version.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RangeOperator {
    /// Exact match (`=1.2.3`).
    Exact,
    /// Exact matches, but with wildcards allowed (`1.x`, `1.*`).
    ExactWithWildcards,
    /// Greater than (`>1.2.3`).
    GreaterThan,
    /// Greater than or equal to (`>=1.2.3`).
    GreaterThanOrEqual,
    /// Less than (`<1.2.3`).
    LessThan,
    /// Less than or equal to (`<=1.2.3`).
    LessThanOrEqual,
    /// Tilde range (`~1.2.3`) - allows patch-level changes.
    /// `~1.2.3` := `>=1.2.3 <1.(2+1).0` := `>=1.2.3 <1.3.0`
    Tilde,
    /// Caret range (`^1.2.3`) - allows compatible changes.
    /// `^1.2.3` := `>=1.2.3 <(1+1).0.0` := `>=1.2.3 <2.0.0`
    Caret,
}

/// Errors that can occur when parsing or working with version ranges.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RangeError {
    /// The range string has an invalid format.
    InvalidFormat,
    /// A version within the range is invalid.
    InvalidVersion(VersionError),
    /// An unrecognized range operator was encountered.
    InvalidOperator,
}

impl fmt::Display for RangeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFormat => write!(f, "Invalid range format"),
            Self::InvalidVersion(err) => write!(f, "Invalid version: {err}"),
            Self::InvalidOperator => write!(f, "Invalid range operator"),
        }
    }
}

impl std::error::Error for RangeError {}

impl From<VersionError> for RangeError {
    fn from(err: VersionError) -> Self {
        Self::InvalidVersion(err)
    }
}

impl Range {
    /// Creates a new range from alternative sets of comparators.
    ///
    /// Each inner vector represents comparators that must all be satisfied (AND),
    /// while the outer vector represents alternatives (OR).
    ///
    /// # Examples
    ///
    /// ```
    /// use biome_package::node_semver::{Comparator, ComparatorOperator, Range, Version};
    ///
    /// let comp1 = Comparator::new(ComparatorOperator::GreaterThanOrEqual, Version::new(1, 2, 3));
    /// let comp2 = Comparator::new(ComparatorOperator::LessThan, Version::new(2, 0, 0));
    /// let range = Range::new(vec![vec![comp1, comp2]]);  // >=1.2.3 <2.0.0
    /// ```
    pub fn new(alternatives: Vec<Vec<Comparator>>) -> Self {
        Self::Alternatives(alternatives)
    }

    /// Tests whether a version satisfies this range.
    ///
    /// A version satisfies a range if it satisfies at least one alternative,
    /// where satisfying an alternative means satisfying all comparators in that
    /// set.
    ///
    /// # Examples
    ///
    /// ```
    /// use biome_package::node_semver::{Range, Version};
    ///
    /// let range: Range = ">=1.2.0 <2.0.0".parse().unwrap();
    /// let version: Version = "1.5.0".parse().unwrap();
    /// assert!(range.includes(&version));
    ///
    /// let version2: Version = "2.0.0".parse().unwrap();
    /// assert!(!range.includes(&version2));
    /// ```
    pub fn includes(&self, version: &Version) -> bool {
        match self {
            Self::Comparators(comparators) => comparators
                .iter()
                .all(|comparator| comparator.matches(version)),
            Self::Alternatives(alternatives) => alternatives.iter().any(|alternative| {
                alternative
                    .iter()
                    .all(|comparator| comparator.matches(version))
            }),
        }
    }

    /// Tests whether this range intersects with another range.
    ///
    /// Two ranges intersect if there exists at least one version that
    /// satisfies both ranges.
    ///
    /// # Examples
    ///
    /// ```
    /// use biome_package::node_semver::Range;
    ///
    /// let range1: Range = "^1.2.3".parse().unwrap();
    /// let range2: Range = "~1.5.0".parse().unwrap();
    /// assert!(range1.intersects(&range2));  // Both can match 1.5.x
    ///
    /// let range3: Range = "^2.0.0".parse().unwrap();
    /// assert!(!range1.intersects(&range3)); // No overlap between 1.x and 2.x
    /// ```
    pub fn intersects(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Comparators(comparators1), Self::Comparators(comparators2)) => {
                sets_intersect(comparators1, comparators2)
            }
            (Self::Comparators(comparators1), Self::Alternatives(alternatives2)) => alternatives2
                .iter()
                .any(|alternative2| sets_intersect(comparators1, alternative2)),
            (Self::Alternatives(alternatives1), Self::Comparators(comparators2)) => alternatives1
                .iter()
                .any(|alternative1| sets_intersect(alternative1, comparators2)),
            (Self::Alternatives(alternatives1), Self::Alternatives(alternatives2)) => {
                alternatives1.iter().any(|alternative1| {
                    alternatives2
                        .iter()
                        .any(|alternative2| sets_intersect(alternative1, alternative2))
                })
            }
        }
    }
}

impl Comparator {
    /// Creates a new comparator with the given operator and version.
    ///
    /// # Examples
    ///
    /// ```
    /// use biome_package::node_semver::{Comparator, ComparatorOperator, Version};
    ///
    /// let version = Version::new(1, 2, 3);
    /// let comp = Comparator::new(ComparatorOperator::GreaterThanOrEqual, version);
    ///
    /// let test_version = Version::new(1, 5, 0);
    /// assert!(comp.matches(&test_version));
    /// ```
    pub fn new(operator: ComparatorOperator, version: Version) -> Self {
        Self { operator, version }
    }

    /// Convenience method for creating a new comparator with
    /// [`ComparatorOperator::Exact`].
    #[inline]
    pub fn new_exact(version: Version) -> Self {
        Self {
            operator: ComparatorOperator::Exact,
            version,
        }
    }

    /// Convenience method for creating a new comparator with
    /// [`ComparatorOperator::GreaterThan`].
    #[inline]
    pub fn new_gt(version: Version) -> Self {
        Self {
            operator: ComparatorOperator::GreaterThan,
            version,
        }
    }

    /// Convenience method for creating a new comparator with
    /// [`ComparatorOperator::GreaterThanOrEqual`].
    #[inline]
    pub fn new_gte(version: Version) -> Self {
        Self {
            operator: ComparatorOperator::GreaterThanOrEqual,
            version,
        }
    }

    /// Convenience method for creating a new comparator with
    /// [`ComparatorOperator::LessThan`].
    #[inline]
    pub fn new_lt(version: Version) -> Self {
        Self {
            operator: ComparatorOperator::LessThan,
            version,
        }
    }

    /// Convenience method for creating a new comparator with
    /// [`ComparatorOperator::LessThanOrEqual`].
    #[inline]
    pub fn new_lte(version: Version) -> Self {
        Self {
            operator: ComparatorOperator::LessThanOrEqual,
            version,
        }
    }

    /// Tests whether a version is matched by this comparator.
    ///
    /// The behavior depends on the operator:
    /// - `Exact`: version must exactly match
    /// - `GreaterThan`/`GreaterThanOrEqual`/`LessThan`/`LessThanOrEqual`: basic comparison
    /// - `Tilde`: allows patch-level changes if the minor version matches
    /// - `Caret`: allows compatible changes according to SemVer rules
    pub fn matches(&self, version: &Version) -> bool {
        match self.operator {
            ComparatorOperator::Exact => version == &self.version,
            ComparatorOperator::GreaterThan => version > &self.version,
            ComparatorOperator::GreaterThanOrEqual => version >= &self.version,
            ComparatorOperator::LessThan => version < &self.version,
            ComparatorOperator::LessThanOrEqual => version <= &self.version,
        }
    }

    /// Tests whether this comparator intersects with another comparator.
    ///
    /// Two comparators intersect if there exists at least one version that
    /// satisfies both comparators.
    pub fn intersects(&self, other: &Self) -> bool {
        match (self.operator, other.operator) {
            (ComparatorOperator::Exact, ComparatorOperator::Exact) => self.version == other.version,
            (ComparatorOperator::Exact, ComparatorOperator::GreaterThan) => {
                self.version > other.version
            }
            (ComparatorOperator::Exact, ComparatorOperator::GreaterThanOrEqual) => {
                self.version >= other.version
            }
            (ComparatorOperator::Exact, ComparatorOperator::LessThan) => {
                self.version < other.version
            }
            (ComparatorOperator::Exact, ComparatorOperator::LessThanOrEqual) => {
                self.version <= other.version
            }
            (ComparatorOperator::GreaterThan, ComparatorOperator::Exact) => {
                self.version < other.version
            }
            (ComparatorOperator::GreaterThan, ComparatorOperator::LessThan) => {
                self.version.inc_patch() < other.version
            }
            (ComparatorOperator::GreaterThan, ComparatorOperator::LessThanOrEqual) => {
                self.version < other.version
            }
            (ComparatorOperator::GreaterThanOrEqual, ComparatorOperator::Exact) => {
                self.version <= other.version
            }
            (ComparatorOperator::GreaterThanOrEqual, ComparatorOperator::LessThan) => {
                self.version < other.version
            }
            (ComparatorOperator::GreaterThanOrEqual, ComparatorOperator::LessThanOrEqual) => {
                self.version <= other.version
            }
            (ComparatorOperator::LessThan, ComparatorOperator::Exact) => {
                self.version > other.version
            }
            (ComparatorOperator::LessThan, ComparatorOperator::GreaterThan) => {
                self.version > other.version.inc_patch()
            }
            (ComparatorOperator::LessThan, ComparatorOperator::GreaterThanOrEqual) => {
                self.version > other.version
            }
            (ComparatorOperator::LessThanOrEqual, ComparatorOperator::Exact) => {
                self.version >= other.version
            }
            (ComparatorOperator::LessThanOrEqual, ComparatorOperator::GreaterThan) => {
                self.version > other.version
            }
            (ComparatorOperator::LessThanOrEqual, ComparatorOperator::GreaterThanOrEqual) => {
                self.version >= other.version
            }
            _ => true,
        }
    }
}

impl FromStr for Range {
    type Err = RangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input = s.trim();
        if input.is_empty() || input == "*" {
            return Ok(Self::Comparators(Vec::new()));
        }

        if input.contains("||") {
            let alternatives: Vec<_> = input
                .split("||")
                .map(parse_comparator_set)
                .collect::<Result<_, Self::Err>>()?;
            return if alternatives.contains(&Vec::new()) {
                Ok(Self::Comparators(Vec::new()))
            } else {
                Ok(Self::Alternatives(alternatives))
            };
        }

        parse_comparator_set(input).map(Self::Comparators)
    }
}

fn parse_comparator_set(mut input: &str) -> Result<Vec<Comparator>, RangeError> {
    if let Some((before, after)) = input.split_once(" - ") {
        return parse_hyphen_range(before, after);
    }

    // We start with a capacity of 2, since almost all sets have either one or
    // two comparators, so we should rarely have to reallocate.
    let mut set = Vec::with_capacity(2);

    // Some errors are intentionally ignored if there is another version string
    // in the set that _can_ be parsed. But if there is an error _and_ the set
    // remains empty, we want to report the error that occurred.
    let mut err = None;

    loop {
        input = input.trim_start();

        let (operator, remainder) = parse_operator(input);
        let version_end = remainder
            .find(char::is_whitespace)
            .unwrap_or(remainder.len());
        if version_end == 0 {
            break match err {
                Some(err) if set.is_empty() => Err(err),
                _ => Ok(set),
            };
        }

        match parse_version_string(&mut set, operator, &remainder[..version_end]) {
            Ok(()) => {}
            Err(version_err) => err = Some(version_err),
        }

        input = &remainder[version_end..];
    }
}

/// Parses the given range operator, followed by the given version string
/// into a set of comparators.
fn parse_version_string(
    set: &mut Vec<Comparator>,
    operator: RangeOperator,
    version_str: &str,
) -> Result<(), RangeError> {
    match operator {
        RangeOperator::Exact => parse_exact(set, version_str.parse()?, version_str),
        RangeOperator::ExactWithWildcards => match version_str.parse() {
            Ok(version) => parse_exact(set, version, version_str),
            Err(err) => match split_component_or_wildcard(version_str) {
                ComponentOrWildcardResult::Wildcard => {}
                ComponentOrWildcardResult::ComponentWithRest(major, rest) => {
                    match split_component_or_wildcard(rest) {
                        ComponentOrWildcardResult::Wildcard => push_range_from_version_to(
                            set,
                            Version::new(major, 0, 0),
                            Version::inc_major,
                        ),
                        ComponentOrWildcardResult::ComponentWithRest(minor, rest) => {
                            match split_component_or_wildcard(rest) {
                                ComponentOrWildcardResult::Wildcard => push_range_from_version_to(
                                    set,
                                    Version::new(major, minor, 0),
                                    Version::inc_minor,
                                ),
                                _ => return Err(err.into()),
                            }
                        }
                        ComponentOrWildcardResult::Neither => return Err(err.into()),
                    }
                }
                ComponentOrWildcardResult::Neither => return Err(err.into()),
            },
        },
        RangeOperator::GreaterThan => {
            let version: Version = version_str.parse()?;
            match count_dots(version_str) {
                0 => push_unique_comparator(set, Comparator::new_gte(version.inc_major())),
                1 => push_unique_comparator(set, Comparator::new_gte(version.inc_minor())),
                _ => push_unique_comparator(set, Comparator::new_gt(version)),
            }
        }
        RangeOperator::GreaterThanOrEqual => {
            push_unique_comparator(set, Comparator::new_gte(version_str.parse()?));
        }
        RangeOperator::LessThan => {
            let version: Version = version_str.parse()?;
            let version = if count_dots(version_str) < 2 {
                version.with_minimum_prerelease()
            } else {
                version
            };
            push_unique_comparator(set, Comparator::new_lt(version));
        }
        RangeOperator::LessThanOrEqual => {
            let version: Version = version_str.parse()?;
            let version = if count_dots(version_str) < 2 {
                version.with_minimum_prerelease()
            } else {
                version
            };
            push_unique_comparator(set, Comparator::new_lte(version));
        }
        RangeOperator::Tilde => {
            push_range_from_version_to(set, version_str.parse()?, |version| {
                if count_dots(version_str) >= 1 {
                    version.inc_minor()
                } else {
                    version.inc_major()
                }
            });
        }
        RangeOperator::Caret => {
            let version: Version = version_str.parse()?;
            if version.major > 0 {
                push_range_from_version_to(set, version, Version::inc_major);
            } else if version.minor > 0 {
                push_range_from_version_to(set, version, Version::inc_minor);
            } else if version.patch > 0 {
                push_range_from_version_to(set, version, Version::inc_patch);
            } else {
                push_unique_comparator(
                    set,
                    Comparator::new_lt(version.inc_major().with_minimum_prerelease()),
                );
            }
        }
    }

    Ok(())
}

fn parse_exact(set: &mut Vec<Comparator>, version: Version, version_str: &str) {
    match count_dots(version_str) {
        0 => push_range_from_version_to(set, version, Version::inc_major),
        1 => push_range_from_version_to(set, version, Version::inc_minor),
        _ => push_unique_comparator(set, Comparator::new_exact(version)),
    }
}

fn push_range_from_version_to(
    set: &mut Vec<Comparator>,
    version: Version,
    upper_bound: impl Fn(&Version) -> Version,
) {
    let upper_bound = upper_bound(&version).with_minimum_prerelease();
    push_unique_comparator(set, Comparator::new_gte(version));
    push_unique_comparator(set, Comparator::new_lt(upper_bound));
}

fn push_unique_comparator(set: &mut Vec<Comparator>, comparator: Comparator) {
    match set
        .iter_mut()
        .find(|existing| existing.operator == comparator.operator)
    {
        Some(existing) => match comparator.operator {
            ComparatorOperator::Exact => {}
            ComparatorOperator::GreaterThan | ComparatorOperator::GreaterThanOrEqual => {
                existing.version = existing.version.clone().max(comparator.version);
            }
            ComparatorOperator::LessThan | ComparatorOperator::LessThanOrEqual => {
                existing.version = existing.version.clone().min(comparator.version);
            }
        },
        None => set.push(comparator),
    }
}

fn parse_hyphen_range(before: &str, after: &str) -> Result<Vec<Comparator>, RangeError> {
    if after.contains(" - ") {
        return Err(RangeError::InvalidFormat);
    }

    let lower_bound: Version = match before.parse() {
        Ok(version) => version,
        Err(err) => match split_component_or_wildcard(before) {
            ComponentOrWildcardResult::Wildcard => Version::new(0, 0, 0),
            ComponentOrWildcardResult::ComponentWithRest(major, rest) => {
                match split_component_or_wildcard(rest) {
                    ComponentOrWildcardResult::Wildcard => Version::new(major, 0, 0),
                    ComponentOrWildcardResult::ComponentWithRest(minor, rest) => {
                        match split_component_or_wildcard(rest) {
                            ComponentOrWildcardResult::Wildcard => Version::new(major, minor, 0),
                            _ => return Err(err.into()),
                        }
                    }
                    ComponentOrWildcardResult::Neither => return Err(err.into()),
                }
            }
            ComponentOrWildcardResult::Neither => return Err(err.into()),
        },
    };
    let (upper_bound, upper_operator): (Version, ComparatorOperator) =
        match after.parse::<Version>() {
            Ok(version) => match count_dots(after) {
                0 => (
                    version.inc_major().with_minimum_prerelease(),
                    ComparatorOperator::LessThan,
                ),
                1 => (
                    version.inc_minor().with_minimum_prerelease(),
                    ComparatorOperator::LessThan,
                ),
                _ => (version, ComparatorOperator::LessThanOrEqual),
            },
            Err(err) => match split_component_or_wildcard(after) {
                ComponentOrWildcardResult::Wildcard => {
                    return Ok(vec![Comparator::new(
                        ComparatorOperator::GreaterThanOrEqual,
                        lower_bound,
                    )]);
                }
                ComponentOrWildcardResult::ComponentWithRest(major, rest) => {
                    match split_component_or_wildcard(rest) {
                        ComponentOrWildcardResult::Wildcard => (
                            Version::new(major.saturating_add(1), 0, 0).with_minimum_prerelease(),
                            ComparatorOperator::LessThan,
                        ),
                        ComponentOrWildcardResult::ComponentWithRest(minor, rest) => {
                            match split_component_or_wildcard(rest) {
                                ComponentOrWildcardResult::Wildcard => (
                                    Version::new(major, minor.saturating_add(1), 0)
                                        .with_minimum_prerelease(),
                                    ComparatorOperator::LessThan,
                                ),
                                _ => return Err(err.into()),
                            }
                        }
                        ComponentOrWildcardResult::Neither => return Err(err.into()),
                    }
                }
                ComponentOrWildcardResult::Neither => return Err(err.into()),
            },
        };

    Ok(vec![
        Comparator::new(ComparatorOperator::GreaterThanOrEqual, lower_bound),
        Comparator::new(upper_operator, upper_bound),
    ])
}

fn parse_operator(input: &str) -> (RangeOperator, &str) {
    if let Some(remainder) = input.strip_prefix(">=") {
        (RangeOperator::GreaterThanOrEqual, remainder.trim_start())
    } else if let Some(remainder) = input.strip_prefix("<=") {
        (RangeOperator::LessThanOrEqual, remainder.trim_start())
    } else if let Some(remainder) = input.strip_prefix('>') {
        (RangeOperator::GreaterThan, remainder.trim_start())
    } else if let Some(remainder) = input.strip_prefix('<') {
        (RangeOperator::LessThan, remainder.trim_start())
    } else if let Some(remainder) = input.strip_prefix('~') {
        (
            RangeOperator::Tilde,
            remainder
                .strip_prefix('>') // HACK: Interpret `~>` as `~`
                .unwrap_or(remainder)
                .trim_start(),
        )
    } else if let Some(remainder) = input.strip_prefix('^') {
        (RangeOperator::Caret, remainder.trim_start())
    } else if let Some(remainder) = input.strip_prefix('=') {
        (RangeOperator::Exact, remainder.trim_start())
    } else {
        (RangeOperator::ExactWithWildcards, input)
    }
}

/// Tests whether two comparator sets intersect.
///
/// Two sets intersect if every comparator in the first set intersects
/// with every comparator in the second set (meaning there's a version
/// that satisfies all comparators in both sets).
fn sets_intersect(set1: &[Comparator], set2: &[Comparator]) -> bool {
    set1.iter()
        .all(|comp1| set2.iter().all(|comp2| comp1.intersects(comp2)))
}

enum ComponentOrWildcardResult<'a> {
    ComponentWithRest(u16, &'a str),
    Wildcard,
    Neither,
}

/// Splits the first component from a version string while checking if it is a
/// wildcard.
///
/// Takes the first component before the dot (if there is any) and checks if it
/// is a wildcard ("x" or "*"):
/// - If it is wildcard, [`SplitComponentOrWildcardResult::Wildcard`] is
///   returned.
/// - Otherwise, if it is an ordinary number component,
///   [`SplitComponentOrWildcardResult::ComponentWithRest`] is returned with the
///   component and the remainder of the version string.
/// - [`SplitComponentOrWildcardResult::Neither`] is returned otherwise.
///
/// If an empty string is passed, [`SplitComponentOrWildcardResult::Wildcard`]
/// is returned as well.
fn split_component_or_wildcard<'a>(version: &'a str) -> ComponentOrWildcardResult<'a> {
    if version.is_empty() {
        return ComponentOrWildcardResult::Wildcard;
    }

    match version.split_once('.') {
        Some((number, _rest)) if is_wildcard(number) => ComponentOrWildcardResult::Wildcard,
        Some((number, rest)) => match number.parse() {
            Ok(number) => ComponentOrWildcardResult::ComponentWithRest(number, rest),
            Err(_) => ComponentOrWildcardResult::Neither,
        },
        None if is_wildcard(version) => ComponentOrWildcardResult::Wildcard,
        None => match version.parse() {
            Ok(number) => ComponentOrWildcardResult::ComponentWithRest(number, ""),
            Err(_) => ComponentOrWildcardResult::Neither,
        },
    }
}

/// Work-around to determine the precision in a given version.
fn count_dots(version: &str) -> usize {
    version
        .bytes()
        .take_while(|c| *c != b'-' && *c != b'+')
        .filter(|c| *c == b'.')
        .count()
}

fn is_wildcard(component: &str) -> bool {
    let mut chars = component.bytes();
    chars
        .next()
        .is_some_and(|c| c == b'x' || c == b'X' || c == b'*')
        && chars.next().is_none()
}

#[cfg(test)]
#[path = "range.tests.rs"]
mod tests;
