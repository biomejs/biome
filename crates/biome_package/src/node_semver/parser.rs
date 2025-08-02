use crate::node_semver::range::{Range, RangeError};
use crate::node_semver::version::{Version, VersionError};

/// Parses a version string into a `Version` struct.
///
/// This is a convenience wrapper around the `FromStr` implementation for `Version`.
///
/// # Examples
///
/// ```
/// use biome_package::node_semver::parse_version;
///
/// let version = parse_version("1.2.3").unwrap();
/// assert_eq!(version.major, 1);
/// assert_eq!(version.minor, 2);
/// assert_eq!(version.patch, 3);
///
/// let prerelease = parse_version("1.2.3-alpha.1").unwrap();
/// assert_eq!(prerelease.prerelease, vec!["alpha", "1"]);
/// ```
///
/// # Errors
///
/// Returns a `VersionError` if the input string is not a valid semantic version.
pub fn parse_version(input: &str) -> Result<Version, VersionError> {
    input.parse()
}

/// Parses a range string into a `Range` struct.
///
/// This is a convenience wrapper around the `FromStr` implementation for `Range`.
///
/// # Examples
///
/// ```
/// use biome_package::node_semver::{parse_range, parse_version};
///
/// let range = parse_range("^1.2.3").unwrap();
/// let version = parse_version("1.5.0").unwrap();
/// assert!(range.satisfies(&version));
///
/// let complex_range = parse_range(">=1.2.0 <2.0.0 || ^2.1.0").unwrap();
/// ```
///
/// # Errors
///
/// Returns a `RangeError` if the input string is not a valid version range.
pub fn parse_range(input: &str) -> Result<Range, RangeError> {
    input.parse()
}

/// Tests whether a version satisfies a range.
///
/// This is a high-level convenience function that parses both strings and
/// tests the relationship.
///
/// # Examples
///
/// ```
/// use biome_package::node_semver::satisfies;
///
/// assert!(satisfies("1.2.3", "^1.2.0").unwrap());
/// assert!(!satisfies("2.0.0", "^1.2.0").unwrap());
/// assert!(satisfies("1.2.4", "~1.2.3").unwrap());
/// ```
///
/// # Errors
///
/// Returns an error if either the version or range string is invalid.
pub fn satisfies(version: &str, range: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let version = parse_version(version)?;
    let range = parse_range(range)?;
    Ok(range.satisfies(&version))
}

/// Finds the maximum version from a list that satisfies a range.
///
/// Invalid version strings in the input list are ignored.
///
/// # Examples
///
/// ```
/// use biome_package::node_semver::max_satisfying;
///
/// let versions = vec!["1.2.3", "1.2.4", "1.3.0", "2.0.0"];
/// let max = max_satisfying(&versions, "~1.2.3").unwrap().unwrap();
/// assert_eq!(max.to_string(), "1.2.4");
///
/// let max = max_satisfying(&versions, "^1.2.3").unwrap().unwrap();
/// assert_eq!(max.to_string(), "1.3.0");
/// ```
///
/// # Returns
///
/// Returns `Ok(Some(version))` if at least one version satisfies the range,
/// `Ok(None)` if no versions satisfy the range, or an error if the range is invalid.
pub fn max_satisfying(
    versions: &[&str],
    range: &str,
) -> Result<Option<Version>, Box<dyn std::error::Error>> {
    let range = parse_range(range)?;
    let mut max_version: Option<Version> = None;

    for version_str in versions {
        if let Ok(version) = parse_version(version_str) {
            if range.satisfies(&version) {
                match &max_version {
                    None => max_version = Some(version),
                    Some(current_max) => {
                        if version > *current_max {
                            max_version = Some(version);
                        }
                    }
                }
            }
        }
    }

    Ok(max_version)
}

/// Finds the minimum version from a list that satisfies a range.
///
/// Invalid version strings in the input list are ignored.
///
/// # Examples
///
/// ```
/// use biome_package::node_semver::min_satisfying;
///
/// let versions = vec!["1.2.3", "1.2.4", "1.3.0", "2.0.0"];
/// let min = min_satisfying(&versions, ">=1.2.4").unwrap().unwrap();
/// assert_eq!(min.to_string(), "1.2.4");
/// ```
///
/// # Returns
///
/// Returns `Ok(Some(version))` if at least one version satisfies the range,
/// `Ok(None)` if no versions satisfy the range, or an error if the range is invalid.
pub fn min_satisfying(
    versions: &[&str],
    range: &str,
) -> Result<Option<Version>, Box<dyn std::error::Error>> {
    let range = parse_range(range)?;
    let mut min_version: Option<Version> = None;

    for version_str in versions {
        if let Ok(version) = parse_version(version_str) {
            if range.satisfies(&version) {
                match &min_version {
                    None => min_version = Some(version),
                    Some(current_min) => {
                        if version < *current_min {
                            min_version = Some(version);
                        }
                    }
                }
            }
        }
    }

    Ok(min_version)
}

/// Validates and normalizes a range string.
///
/// Returns the normalized range string if valid, or `None` if invalid.
///
/// # Examples
///
/// ```
/// use biome_package::node_semver::valid_range;
///
/// assert!(valid_range("^1.2.3").is_some());
/// assert!(valid_range(">=1.2.0 <2.0.0").is_some());
/// assert!(valid_range("invalid-range").is_none());
/// ```
pub fn valid_range(range: &str) -> Option<String> {
    match parse_range(range) {
        Ok(_) => Some(range.to_string()),
        Err(_) => None,
    }
}

/// Tests whether two ranges intersect.
///
/// Two ranges intersect if there exists at least one version that
/// satisfies both ranges.
///
/// # Examples
///
/// ```
/// use biome_package::node_semver::intersects;
///
/// assert!(intersects("^1.2.3", "~1.5.0").unwrap());   // Both match 1.5.x
/// assert!(!intersects("^1.2.3", "^2.0.0").unwrap());  // No overlap
/// assert!(intersects(">=1.2.0", "<=1.3.0").unwrap()); // Clear overlap
/// ```
///
/// # Errors
///
/// Returns an error if either range string is invalid.
pub fn intersects(range1: &str, range2: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let r1 = parse_range(range1)?;
    let r2 = parse_range(range2)?;
    Ok(r1.intersects(&r2))
}

/// Tests whether the first range has any overlap with the second range.
///
/// This is equivalent to `intersects()` but matches the node-semver-rs API.
/// Two ranges have overlap if there exists at least one version that
/// satisfies both ranges.
///
/// # Examples
///
/// ```
/// use biome_package::node_semver::allows_any;
///
/// assert!(allows_any("^1.2.3", "~1.5.0").unwrap());   // Both match 1.5.x
/// assert!(!allows_any("^1.2.3", "^2.0.0").unwrap());  // No overlap
/// ```
///
/// # Errors
///
/// Returns an error if either range string is invalid.
pub fn allows_any(range1: &str, range2: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let r1 = parse_range(range1)?;
    let r2 = parse_range(range2)?;
    Ok(r1.allows_any(&r2))
}

/// Tests whether the second range is a strict superset of the first range.
///
/// A range is a superset of another if every version that satisfies
/// the first range also satisfies the second range.
///
/// # Examples
///
/// ```
/// use biome_package::node_semver::allows_all;
///
/// assert!(allows_all("~1.2.3", "^1.0.0").unwrap());   // ^1.0.0 encompasses ~1.2.3
/// assert!(!allows_all("^1.0.0", "~1.2.3").unwrap());  // ~1.2.3 doesn't encompass ^1.0.0
/// assert!(allows_all("1.2.3", "~1.2.3").unwrap());    // ~1.2.3 encompasses exact 1.2.3
/// ```
///
/// # Errors
///
/// Returns an error if either range string is invalid.
pub fn allows_all(range1: &str, range2: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let r1 = parse_range(range1)?;
    let r2 = parse_range(range2)?;
    Ok(r1.allows_all(&r2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_satisfies() {
        assert!(satisfies("1.2.3", "1.2.3").unwrap());
        assert!(satisfies("1.2.4", "~1.2.3").unwrap());
        assert!(!satisfies("1.3.0", "~1.2.3").unwrap());
        assert!(satisfies("1.3.0", "^1.2.3").unwrap());
        assert!(!satisfies("2.0.0", "^1.2.3").unwrap());
    }

    #[test]
    fn test_max_satisfying() {
        let versions = vec!["1.2.3", "1.2.4", "1.3.0", "2.0.0"];
        let result = max_satisfying(&versions, "~1.2.3").unwrap();
        assert_eq!(result.unwrap().to_string(), "1.2.4");

        let result = max_satisfying(&versions, "^1.2.3").unwrap();
        assert_eq!(result.unwrap().to_string(), "1.3.0");
    }

    #[test]
    fn test_min_satisfying() {
        let versions = vec!["1.2.3", "1.2.4", "1.3.0", "2.0.0"];
        let result = min_satisfying(&versions, ">=1.2.4").unwrap();
        assert_eq!(result.unwrap().to_string(), "1.2.4");
    }

    #[test]
    fn test_valid_range() {
        assert!(valid_range("1.2.3").is_some());
        assert!(valid_range("~1.2.3").is_some());
        assert!(valid_range("^1.2.3").is_some());
        assert!(valid_range(">=1.2.3 <2.0.0").is_some());
        assert!(valid_range("invalid").is_none());
    }

    #[test]
    fn test_intersects() {
        assert!(intersects("^1.2.3", "~1.2.5").unwrap());
        assert!(!intersects("^1.2.3", "^2.0.0").unwrap());
        assert!(intersects(">=1.2.0", "<=1.3.0").unwrap());
    }

    #[test]
    fn test_allows_any() {
        assert!(allows_any("^1.2.3", "~1.5.0").unwrap());
        assert!(!allows_any("^1.2.3", "^2.0.0").unwrap());
        assert!(allows_any(">=1.2.0", "<=1.3.0").unwrap());

        // Test identical ranges
        assert!(allows_any("^1.2.3", "^1.2.3").unwrap());

        // Test error cases
        assert!(allows_any("invalid", "^1.2.3").is_err());
        assert!(allows_any("^1.2.3", "invalid").is_err());
    }

    #[test]
    fn test_allows_all() {
        // Test superset relationships
        assert!(allows_all("~1.2.3", "^1.0.0").unwrap());
        assert!(!allows_all("^1.0.0", "~1.2.3").unwrap());
        assert!(allows_all("1.2.3", "~1.2.3").unwrap());

        // Test identical ranges
        assert!(allows_all("^1.2.3", "^1.2.3").unwrap());

        // Test non-overlapping ranges
        assert!(!allows_all("^1.2.3", "^2.0.0").unwrap());
        assert!(!allows_all("^2.0.0", "^1.2.3").unwrap());

        // Test error cases
        assert!(allows_all("invalid", "^1.2.3").is_err());
        assert!(allows_all("^1.2.3", "invalid").is_err());
    }

    #[test]
    fn test_comprehensive_satisfies() {
        // Test cases covering various range and version combinations
        let satisfies_cases = vec![
            // Basic exact matches
            ("1.0.0", "1.0.0", true),
            ("1.0.0", "1.0.1", false),
            ("1.2.3", "v1.2.3", true),
            ("=1.2.3", "1.2.3", true),
            // Comparison operators
            (">=1.0.0", "1.0.0", true),
            (">=1.0.0", "1.0.1", true),
            (">=1.0.0", "0.9.9", false),
            (">1.0.0", "1.0.1", true),
            (">1.0.0", "1.0.0", false),
            ("<=2.0.0", "2.0.0", true),
            ("<=2.0.0", "1.9.9", true),
            ("<=2.0.0", "2.0.1", false),
            ("<2.0.0", "1.9.9", true),
            ("<2.0.0", "2.0.0", false),
            // Tilde ranges
            ("~1.2.3", "1.2.3", true),
            ("~1.2.3", "1.2.4", true),
            ("~1.2.3", "1.3.0", false),
            ("~1.2.0", "1.2.0", true),
            ("~1.2.0", "1.2.5", true),
            ("~1.2.0", "1.3.0", false),
            ("~1.0.0", "1.0.0", true),
            ("~1.0.0", "1.0.9", true),
            ("~1.0.0", "1.1.0", false),
            // Caret ranges
            ("^1.2.3", "1.2.3", true),
            ("^1.2.3", "1.8.1", true),
            ("^1.2.3", "2.0.0", false),
            ("^0.2.3", "0.2.3", true),
            ("^0.2.3", "0.2.4", true),
            ("^0.2.3", "0.3.0", false),
            ("^0.0.3", "0.0.3", true),
            ("^0.0.3", "0.0.4", false),
            // OR ranges
            ("1.2.7 || >=1.2.9 <2.0.0", "1.2.7", true),
            ("1.2.7 || >=1.2.9 <2.0.0", "1.2.8", false),
            ("1.2.7 || >=1.2.9 <2.0.0", "1.2.9", true),
            ("1.2.7 || >=1.2.9 <2.0.0", "1.5.0", true),
            ("1.2.7 || >=1.2.9 <2.0.0", "2.0.0", false),
            // Compound ranges
            (">=1.2.1 1.2.3", "1.2.3", true),
            (">=1.2.1 1.2.3", "1.2.1", false),
            (">=1.2.1 1.2.3", "1.2.4", false),
            // Prerelease handling
            ("1.2.3-alpha.3", "1.2.3-alpha.3", true),
            ("1.2.3-alpha.3", "1.2.3-alpha.2", false),
            ("1.2.3-alpha.3", "1.2.3-alpha.4", false),
            (">=1.2.3-alpha", "1.2.3-alpha.1", true),
            (">=1.2.3-alpha", "1.2.3-beta", true),
            (">=1.2.3-alpha", "1.2.3", true),
        ];

        for (range_str, version_str, expected) in satisfies_cases {
            let result = satisfies(version_str, range_str);
            match result {
                Ok(actual) => {
                    assert_eq!(
                        actual, expected,
                        "satisfies('{version_str}', '{range_str}') expected {expected} but got {actual}"
                    );
                }
                Err(e) => {
                    panic!("Failed to parse range '{range_str}' or version '{version_str}': {e}");
                }
            }
        }
    }

    #[test]
    fn test_comprehensive_max_min_satisfying() {
        let versions = vec!["1.2.3", "1.2.4", "1.3.0", "2.0.0", "2.1.0", "3.0.0"];

        // Test max_satisfying
        let max_cases = vec![
            ("~1.2.3", Some("1.2.4")),
            ("^1.2.3", Some("1.3.0")),
            ("^2.0.0", Some("2.1.0")),
            (">=1.2.4", Some("3.0.0")),
            (">=4.0.0", None), // No version satisfies
            ("<1.2.4", Some("1.2.3")),
        ];

        for (range_str, expected_opt) in max_cases {
            let result = max_satisfying(&versions, range_str).unwrap();
            match (result, expected_opt) {
                (Some(actual), Some(expected)) => {
                    assert_eq!(
                        actual.to_string(),
                        expected,
                        "max_satisfying({versions:?}, '{range_str}') expected {expected} but got {actual}"
                    );
                }
                (None, None) => {
                    // Both None, this is correct
                }
                (actual, expected) => {
                    panic!(
                        "max_satisfying({versions:?}, '{range_str}') expected {expected:?} but got {actual:?}"
                    );
                }
            }
        }

        // Test min_satisfying
        let min_cases = vec![
            ("~1.2.3", Some("1.2.3")),
            ("^1.2.3", Some("1.2.3")),
            (">=1.2.4", Some("1.2.4")),
            (">=4.0.0", None), // No version satisfies
            (">1.2.3", Some("1.2.4")),
        ];

        for (range_str, expected_opt) in min_cases {
            let result = min_satisfying(&versions, range_str).unwrap();
            match (result, expected_opt) {
                (Some(actual), Some(expected)) => {
                    assert_eq!(
                        actual.to_string(),
                        expected,
                        "min_satisfying({versions:?}, '{range_str}') expected {expected} but got {actual}"
                    );
                }
                (None, None) => {
                    // Both None, this is correct
                }
                (actual, expected) => {
                    panic!(
                        "min_satisfying({versions:?}, '{range_str}') expected {expected:?} but got {actual:?}"
                    );
                }
            }
        }
    }

    #[test]
    fn test_invalid_range_handling() {
        // Test various invalid ranges
        let invalid_ranges = vec![
            "blerg",
            "git+https://user:password0123@github.com/foo",
            "1.2.3.4.5",
            "1.2.3-",
            "1.2.3+",
            ">=1.2.3 <",
            "~1.2.3 ^",
            // Note: Our implementation currently allows "1.2.3 || " and " || 1.2.3"
            // These would be caught by more strict validation
        ];

        for invalid_range in invalid_ranges {
            let result = valid_range(invalid_range);
            assert!(
                result.is_none(),
                "Range '{invalid_range}' should be invalid but was accepted"
            );

            // Test that other functions also handle these gracefully
            assert!(satisfies("1.2.3", invalid_range).is_err());
            assert!(intersects(invalid_range, "^1.0.0").is_err());
            assert!(allows_any(invalid_range, "^1.0.0").is_err());
            assert!(allows_all(invalid_range, "^1.0.0").is_err());
        }
    }
}
