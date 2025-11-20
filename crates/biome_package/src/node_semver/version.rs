use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;

/// A semantic version representation following the SemVer 2.0.0 specification.
///
/// A version consists of MAJOR.MINOR.PATCH, optionally followed by prerelease
/// identifiers and build metadata.
///
/// # Examples
///
/// ```
/// use biome_package::node_semver::Version;
///
/// let version: Version = "1.2.3".parse().unwrap();
/// assert_eq!(version.major, 1);
/// assert_eq!(version.minor, 2);
/// assert_eq!(version.patch, 3);
///
/// let prerelease: Version = "1.2.3-alpha.1".parse().unwrap();
/// assert_eq!(prerelease.prerelease, vec!["alpha", "1"]);
///
/// let with_build: Version = "1.2.3+20130313144700".parse().unwrap();
/// assert_eq!(with_build.build, vec!["20130313144700"]);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Version {
    /// The major version number. Incremented for incompatible API changes.
    pub major: u16,
    /// The minor version number. Incremented for backwards-compatible functionality additions.
    pub minor: u16,
    /// The patch version number. Incremented for backwards-compatible bug fixes.
    pub patch: u16,
    /// Pre-release version identifiers (e.g., ("alpha", "1") for "1.2.3-alpha.1").
    pub prerelease: Vec<String>,
    /// Build metadata identifiers (e.g., ("20130313144700") for "1.2.3+20130313144700").
    pub build: Vec<String>,
}

/// Errors that can occur when parsing or creating a version.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VersionError {
    /// The version string has an invalid format (e.g., missing components, wrong separators).
    InvalidFormat,
    /// A numeric component contains invalid characters or leading zeros.
    InvalidNumber,
    /// A prerelease identifier is empty or contains invalid characters.
    InvalidPrerelease,
    /// A build metadata identifier is empty or contains invalid characters.
    InvalidBuild,
}

impl fmt::Display for VersionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFormat => write!(f, "Invalid version format"),
            Self::InvalidNumber => write!(f, "Invalid number in version"),
            Self::InvalidPrerelease => write!(f, "Invalid prerelease identifier"),
            Self::InvalidBuild => write!(f, "Invalid build metadata"),
        }
    }
}

impl std::error::Error for VersionError {}

impl Version {
    /// Creates a new version with the specified major, minor, and patch numbers.
    ///
    /// Prerelease identifiers and build metadata are initially empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use biome_package::node_semver::Version;
    ///
    /// let version = Version::new(1, 2, 3);
    /// assert_eq!(version.major, 1);
    /// assert_eq!(version.minor, 2);
    /// assert_eq!(version.patch, 3);
    /// assert!(version.prerelease.is_empty());
    /// assert!(version.build.is_empty());
    /// ```
    pub fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self {
            major,
            minor,
            patch,
            prerelease: Vec::new(),
            build: Vec::new(),
        }
    }

    /// Adds prerelease identifiers to this version.
    ///
    /// Prerelease identifiers must be non-empty and contain only ASCII alphanumeric
    /// characters and hyphens.
    ///
    /// # Errors
    ///
    /// Returns `VersionError::InvalidPrerelease` if any identifier is empty or contains
    /// invalid characters.
    ///
    /// # Examples
    ///
    /// ```
    /// use biome_package::node_semver::Version;
    ///
    /// let version = Version::new(1, 2, 3)
    ///     .with_prerelease(vec!["alpha".to_string(), "1".to_string()])
    ///     .unwrap();
    /// assert_eq!(version.prerelease, vec!["alpha", "1"]);
    /// ```
    pub fn with_prerelease(mut self, prerelease: Vec<String>) -> Result<Self, VersionError> {
        if prerelease
            .iter()
            .any(|p| p.is_empty() || !is_valid_identifier(p))
        {
            return Err(VersionError::InvalidPrerelease);
        }
        self.prerelease = prerelease;
        Ok(self)
    }

    /// Adds a minimum prerelease to the version.
    ///
    /// A minimum prerelease can be added to a version to avoid any possible
    /// prerelease from being considered less than the version.
    #[inline]
    pub fn with_minimum_prerelease(mut self) -> Self {
        self.prerelease = vec!["0".to_string()];
        self
    }

    /// Adds build metadata identifiers to this version.
    ///
    /// Build metadata identifiers must be non-empty and contain only ASCII alphanumeric
    /// characters and hyphens.
    ///
    /// # Errors
    ///
    /// Returns `VersionError::InvalidBuild` if any identifier is empty or contains
    /// invalid characters.
    ///
    /// # Examples
    ///
    /// ```
    /// use biome_package::node_semver::Version;
    ///
    /// let version = Version::new(1, 2, 3)
    ///     .with_build(vec!["20130313144700".to_string()])
    ///     .unwrap();
    /// assert_eq!(version.build, vec!["20130313144700"]);
    /// ```
    pub fn with_build(mut self, build: Vec<String>) -> Result<Self, VersionError> {
        if build
            .iter()
            .any(|b| b.is_empty() || !is_valid_identifier(b))
        {
            return Err(VersionError::InvalidBuild);
        }
        self.build = build;
        Ok(self)
    }

    /// Returns `true` if this version has prerelease identifiers.
    ///
    /// According to SemVer, a prerelease version has lower precedence than a normal version.
    ///
    /// # Examples
    ///
    /// ```
    /// use biome_package::node_semver::Version;
    ///
    /// let stable: Version = "1.2.3".parse().unwrap();
    /// let prerelease: Version = "1.2.3-alpha.1".parse().unwrap();
    ///
    /// assert!(!stable.is_prerelease());
    /// assert!(prerelease.is_prerelease());
    /// ```
    pub fn is_prerelease(&self) -> bool {
        !self.prerelease.is_empty()
    }

    /// Compares two versions according to SemVer precedence rules.
    ///
    /// This comparison ignores build metadata, as specified in SemVer 2.0.0.
    /// Prerelease versions have lower precedence than normal versions.
    ///
    /// # Examples
    ///
    /// ```
    /// use biome_package::node_semver::Version;
    /// use std::cmp::Ordering;
    ///
    /// let v1: Version = "1.0.0-alpha".parse().unwrap();
    /// let v2: Version = "1.0.0".parse().unwrap();
    ///
    /// assert_eq!(v1.compare_precedence(&v2), Ordering::Less);
    /// ```
    pub fn compare_precedence(&self, other: &Self) -> Ordering {
        match (self.major, self.minor, self.patch).cmp(&(other.major, other.minor, other.patch)) {
            Ordering::Equal => match (self.prerelease.is_empty(), other.prerelease.is_empty()) {
                (true, true) => Ordering::Equal,
                (true, false) => Ordering::Greater,
                (false, true) => Ordering::Less,
                (false, false) => compare_prerelease(&self.prerelease, &other.prerelease),
            },
            other => other,
        }
    }

    /// Increments the version to the next possible major version.
    ///
    /// In case of overflow, nothing will happen.
    pub fn inc_major(&self) -> Self {
        Self::new(self.major.saturating_add(1), 0, 0)
    }

    /// Increments the version to the next possible minor version.
    ///
    /// In case of overflow, it will increment the major version instead.
    pub fn inc_minor(&self) -> Self {
        if let Some(incremented_minor) = self.minor.checked_add(1) {
            Self::new(self.major, incremented_minor, 0)
        } else {
            self.inc_major()
        }
    }

    /// Increments the version to the next possible patch version.
    ///
    /// In case of overflow, it will increment the minor version instead.
    pub fn inc_patch(&self) -> Self {
        if let Some(incremented_patch) = self.patch.checked_add(1) {
            Self::new(self.major, self.minor, incremented_patch)
        } else {
            self.inc_minor()
        }
    }

    /// Decrements the version to the previous possible major version.
    ///
    /// In case of underflow, nothing will happen.
    pub fn dec_major(&self) -> Self {
        Self::new(self.major.saturating_sub(1), 0, 0)
    }

    /// Decrements the version to the previous possible minor version.
    ///
    /// In case of underflow, it will decrement the major version instead.
    pub fn dec_minor(&self) -> Self {
        if let Some(decremented_minor) = self.minor.checked_sub(1) {
            Self::new(self.major, decremented_minor, 0)
        } else {
            self.dec_major()
        }
    }

    /// Decrements the version to the previous possible patch version.
    ///
    /// In case of underflow, it will decrement the minor version instead.
    pub fn dec_patch(&self) -> Self {
        if let Some(decremented_patch) = self.patch.checked_sub(1) {
            Self::new(self.major, self.minor, decremented_patch)
        } else {
            self.dec_minor()
        }
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        self.compare_precedence(other)
    }
}

impl FromStr for Version {
    type Err = VersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input = s.trim_start_matches(['v', '=']);

        let (version_part, build_part) = if let Some(pos) = input.find('+') {
            (&input[..pos], Some(&input[pos + 1..]))
        } else {
            (input, None)
        };

        let (core_part, prerelease_part) = if let Some(pos) = version_part.find('-') {
            (&version_part[..pos], Some(&version_part[pos + 1..]))
        } else {
            (version_part, None)
        };

        let parts: Vec<&str> = core_part.split('.').collect();
        if parts.is_empty() || parts.len() > 3 {
            return Err(VersionError::InvalidFormat);
        }

        let major = parse_number(parts[0])?;
        let minor = match parts.get(1) {
            Some(minor) => parse_number(minor)?,
            None => 0,
        };
        let patch = match parts.get(2) {
            Some(patch) => parse_number(patch)?,
            None => 0,
        };

        let mut version = Self::new(major, minor, patch);

        if let Some(prerelease) = prerelease_part {
            let prerelease_parts = prerelease.split('.').map(|s| s.to_string()).collect();
            version = version.with_prerelease(prerelease_parts)?;
        }

        if let Some(build) = build_part {
            let build_parts = build.split('.').map(|s| s.to_string()).collect();
            version = version.with_build(build_parts)?;
        }

        Ok(version)
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)?;

        if !self.prerelease.is_empty() {
            write!(f, "-{}", self.prerelease.join("."))?;
        }

        if !self.build.is_empty() {
            write!(f, "+{}", self.build.join("."))?;
        }

        Ok(())
    }
}

/// Parses a numeric component of a version string.
///
/// According to SemVer, numeric identifiers must not have leading zeros
/// (except for the single digit "0").
///
/// # Errors
///
/// Returns `VersionError::InvalidNumber` if the string is empty, contains
/// non-numeric characters, or has leading zeros.
fn parse_number(s: &str) -> Result<u16, VersionError> {
    if s.is_empty() {
        return Err(VersionError::InvalidNumber);
    }

    if s.len() > 1 && s.starts_with('0') {
        return Err(VersionError::InvalidNumber);
    }

    s.parse().map_err(|_| VersionError::InvalidNumber)
}

/// Validates that an identifier contains only allowed characters.
///
/// Valid identifiers must be non-empty and contain only ASCII alphanumeric
/// characters and hyphens, as specified in SemVer 2.0.0.
fn is_valid_identifier(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    s.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
}

/// Compares two prerelease identifier arrays according to SemVer precedence rules.
///
/// Prerelease precedence is determined by comparing each dot-separated identifier
/// from left to right:
/// - Identifiers consisting of only digits are compared numerically
/// - Identifiers with letters or hyphens are compared lexically in ASCII sort order
/// - Numeric identifiers always have lower precedence than non-numeric identifiers
/// - A larger set of prerelease fields has higher precedence than a smaller set,
///   when all preceding identifiers are equal
fn compare_prerelease(a: &[String], b: &[String]) -> Ordering {
    for (part_a, part_b) in a.iter().zip(b.iter()) {
        match (part_a.parse::<u16>(), part_b.parse::<u16>()) {
            (Ok(num_a), Ok(num_b)) => match num_a.cmp(&num_b) {
                Ordering::Equal => {}
                other => return other,
            },
            (Ok(_), Err(_)) => return Ordering::Less,
            (Err(_), Ok(_)) => return Ordering::Greater,
            (Err(_), Err(_)) => match part_a.cmp(part_b) {
                Ordering::Equal => {}
                other => return other,
            },
        }
    }

    a.len().cmp(&b.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_version_parsing() {
        let version = "1.2.3".parse::<Version>().unwrap();
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 2);
        assert_eq!(version.patch, 3);
        assert!(version.prerelease.is_empty());
        assert!(version.build.is_empty());
    }

    #[test]
    fn test_missing_patch() {
        let version = "1.2".parse::<Version>().unwrap();
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 2);
        assert_eq!(version.patch, 0);
        assert!(version.prerelease.is_empty());
        assert!(version.build.is_empty());
    }

    #[test]
    fn test_missing_minor() {
        let version = "1".parse::<Version>().unwrap();
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 0);
        assert_eq!(version.patch, 0);
        assert!(version.prerelease.is_empty());
        assert!(version.build.is_empty());
    }

    #[test]
    fn test_version_with_prerelease() {
        let version = "1.2.3-alpha.1".parse::<Version>().unwrap();
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 2);
        assert_eq!(version.patch, 3);
        assert_eq!(version.prerelease, &["alpha", "1"]);
        assert!(version.build.is_empty());
    }

    #[test]
    fn test_version_with_build() {
        let version = "1.2.3+20130313144700".parse::<Version>().unwrap();
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 2);
        assert_eq!(version.patch, 3);
        assert!(version.prerelease.is_empty());
        assert_eq!(version.build, &["20130313144700"]);
    }

    #[test]
    fn test_version_with_prerelease_and_build() {
        let version = "1.2.3-beta+exp.sha.5114f85".parse::<Version>().unwrap();
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 2);
        assert_eq!(version.patch, 3);
        assert_eq!(version.prerelease, &["beta"]);
        assert_eq!(version.build, &["exp", "sha", "5114f85"]);
    }

    #[test]
    fn test_version_prefixes() {
        let version1 = "v1.2.3".parse::<Version>().unwrap();
        let version2 = "=1.2.3".parse::<Version>().unwrap();
        let version3 = "1.2.3".parse::<Version>().unwrap();

        assert_eq!(version1, version2);
        assert_eq!(version2, version3);
    }

    #[test]
    fn test_version_comparison() {
        let v1 = "1.0.0".parse::<Version>().unwrap();
        let v2 = "2.0.0".parse::<Version>().unwrap();
        let v3 = "2.1.0".parse::<Version>().unwrap();
        let v4 = "2.1.1".parse::<Version>().unwrap();

        assert!(v1 < v2);
        assert!(v2 < v3);
        assert!(v3 < v4);
    }

    #[test]
    fn test_prerelease_comparison() {
        let v1 = "1.0.0-alpha".parse::<Version>().unwrap();
        let v2 = "1.0.0".parse::<Version>().unwrap();
        let v3 = "1.0.0-alpha.1".parse::<Version>().unwrap();
        let v4 = "1.0.0-alpha.beta".parse::<Version>().unwrap();
        let v5 = "1.0.0-beta".parse::<Version>().unwrap();
        let v6 = "1.0.0-beta.2".parse::<Version>().unwrap();
        let v7 = "1.0.0-beta.11".parse::<Version>().unwrap();
        let v8 = "1.0.0-rc.1".parse::<Version>().unwrap();

        assert!(v1 < v2);
        assert!(v1 < v3);
        assert!(v3 < v4);
        assert!(v4 < v5);
        assert!(v5 < v6);
        assert!(v6 < v7);
        assert!(v7 < v8);
        assert!(v8 < v2);
    }

    #[test]
    fn test_invalid_versions() {
        assert!("1.2.3.4".parse::<Version>().is_err());
        assert!("01.2.3".parse::<Version>().is_err());
        assert!("1.02.3".parse::<Version>().is_err());
        assert!("1.2.03".parse::<Version>().is_err());
        assert!("1.2.3-".parse::<Version>().is_err());
        assert!("1.2.3+".parse::<Version>().is_err());

        // Additional invalid cases from node-semver tests
        assert!("".parse::<Version>().is_err());
        assert!("hello, world".parse::<Version>().is_err());
        assert!("xyz".parse::<Version>().is_err());
        assert!("1.2.3-+".parse::<Version>().is_err());
        assert!("1.2.3.DEV.SNAPSHOT".parse::<Version>().is_err());
        assert!(
            "1.2.31.2.3----RC-SNAPSHOT.12.09.1--..12+788"
                .parse::<Version>()
                .is_err()
        );
        assert!("1.0.0+".parse::<Version>().is_err());
        assert!("1.0.0-".parse::<Version>().is_err());
        assert!("1.0.0-+".parse::<Version>().is_err());
        assert!("1.0.0-+123.456".parse::<Version>().is_err());
        assert!("1.0.0-123.456+".parse::<Version>().is_err());
        assert!("1.0.0-123.456+123.456+".parse::<Version>().is_err());
        assert!("1.0.0-123..456".parse::<Version>().is_err());
        assert!("1.0.0+123..456".parse::<Version>().is_err());
        assert!("not-a-version".parse::<Version>().is_err());
    }

    #[test]
    fn test_extensive_valid_versions() {
        // Basic versions
        let cases = vec![
            "1.0.0",
            "10.20.30",
            "1.1.2-prerelease+meta",
            "1.1.2+meta",
            "1.1.2+meta-valid",
            "1.2-SNAPSHOT-123",
            "1.2-RC-SNAPSHOT",
            "1.2-3",
            "1.0.0-alpha",
            "1.0.0-beta",
            "1.0.0-alpha.beta",
            "1.0.0-alpha.1",
            "1.0.0-alpha0.beta",
            "1.0.0-alpha.0beta",
            "1.0.0-alpha-a.b-c-somethinglong",
            "1.0.0-rc.1+build.123",
            "2.0.0-rc.1+build.456",
            "1.2.3-beta",
            "10.2.3-DEV-SNAPSHOT",
            "1.2.3-SNAPSHOT-123",
            "2.0.0+build.1848",
            "2.0.1-alpha.1",
            "2.0.1-alpha.beta",
            "2.0.1-alpha.beta.1",
            "2.0.1-alpha0",
            "2.0.1-alpha0.beta",
            "2.0.1-alpha.0beta",
            "2.0.1-alpha.0beta.0",
            "2.0.1-alpha-a.b-c-somethinglong",
            // With prefixes
            "v1.0.0",
            "v2.0.0",
            "v1.2.3-beta",
            "v10.2.3-DEV-SNAPSHOT",
            "=1.0.0",
            "=2.0.0",
        ];

        for case in cases {
            assert!(case.parse::<Version>().is_ok(), "Failed to parse: {case}");
        }
    }

    #[test]
    fn test_complex_prerelease_comparison() {
        // Test cases based on node-semver comparisons fixture
        let comparison_cases = vec![
            ("1.0.0-alpha", "1.0.0-alpha.1", true),      // alpha < alpha.1
            ("1.0.0-alpha.1", "1.0.0-alpha.beta", true), // alpha.1 < alpha.beta
            ("1.0.0-alpha.beta", "1.0.0-beta", true),    // alpha.beta < beta
            ("1.0.0-beta", "1.0.0-beta.2", true),        // beta < beta.2
            ("1.0.0-beta.2", "1.0.0-beta.11", true),     // beta.2 < beta.11
            ("1.0.0-beta.11", "1.0.0-rc.1", true),       // beta.11 < rc.1
            ("1.0.0-rc.1", "1.0.0", true),               // rc.1 < 1.0.0
            ("1.0.0-1", "1.0.0-2", true),                // 1 < 2 (numeric comparison)
            ("1.0.0-2", "1.0.0-10", true),               // 2 < 10 (numeric comparison)
            ("1.0.0-10", "1.0.0-a", true),               // 10 < a (numeric < alpha)
            ("1.0.0-a", "1.0.0-b", true),                // a < b (lexical)
        ];

        for (lower, higher, should_be_less) in comparison_cases {
            let v1 = lower.parse::<Version>().unwrap();
            let v2 = higher.parse::<Version>().unwrap();

            if should_be_less {
                assert!(v1 < v2, "{lower} should be < {higher}");
            } else {
                assert!(v1 >= v2, "{lower} should be >= {higher}");
            }
        }
    }
}
