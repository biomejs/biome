//! A Rust implementation of the node-semver specification.
//!
//! This module provides semantic version parsing, comparison, and range matching
//! functionality compatible with the Node.js semver package.
//!
//! # Overview
//!
//! The module consists of three main parts:
//!
//! - **Version parsing and comparison**: Parse semantic versions and compare them
//!   according to SemVer 2.0.0 precedence rules
//! - **Range parsing and matching**: Parse version ranges and test if versions satisfy them
//! - **Utility functions**: High-level functions for common semver operations
//!
//! # Examples
//!
//! ## Basic version parsing and comparison
//!
//! ```
//! use biome_package::node_semver::{Version, parse_version};
//!
//! let v1: Version = "1.2.3".parse().unwrap();
//! let v2 = parse_version("1.2.4").unwrap();
//!
//! assert!(v1 < v2);
//! assert_eq!(v1.major, 1);
//! assert_eq!(v1.minor, 2);
//! assert_eq!(v1.patch, 3);
//! ```
//!
//! ## Range matching
//!
//! ```
//! use biome_package::node_semver::{Range, Version};
//!
//! let range: Range = "^1.2.3".parse().unwrap();
//! let version: Version = "1.5.0".parse().unwrap();
//!
//! assert!(range.satisfies(&version));
//! ```
//!
//! ## High-level utility functions
//!
//! ```
//! use biome_package::node_semver::{satisfies, max_satisfying, allows_any, allows_all};
//!
//! // Test if a version satisfies a range
//! assert!(satisfies("1.2.4", "~1.2.3").unwrap());
//!
//! // Find the maximum satisfying version from a list
//! let versions = vec!["1.2.3", "1.2.4", "1.3.0", "2.0.0"];
//! let max = max_satisfying(&versions, "^1.2.0").unwrap().unwrap();
//! assert_eq!(max.to_string(), "1.3.0");
//!
//! // Test if ranges have overlap
//! assert!(allows_any("^1.2.3", "~1.5.0").unwrap());    // Both match 1.5.x
//! assert!(!allows_any("^1.2.3", "^2.0.0").unwrap());   // No overlap
//!
//! // Test if one range encompasses another
//! assert!(allows_all("~1.2.3", "^1.0.0").unwrap());    // ^1.0.0 encompasses ~1.2.3
//! assert!(!allows_all("^1.0.0", "~1.2.3").unwrap());   // ~1.2.3 doesn't encompass ^1.0.0
//! ```
//!
//! # Supported Range Operators
//!
//! - **Exact**: `1.2.3` or `=1.2.3`
//! - **Greater than**: `>1.2.3`
//! - **Greater than or equal**: `>=1.2.3`
//! - **Less than**: `<1.2.3`
//! - **Less than or equal**: `<=1.2.3`
//! - **Tilde ranges**: `~1.2.3` (allows patch-level changes)
//! - **Caret ranges**: `^1.2.3` (allows compatible changes)
//! - **Hyphen ranges**: `1.2.3 - 2.3.4`
//! - **OR operations**: `1.2.7 || >=1.2.9 <2.0.0`
//! - **Compound ranges**: `>=1.2.7 <1.3.0`
//!
//! # Error Handling
//!
//! All parsing operations return `Result` types with descriptive error information.
//! The implementation is designed to be panic-free and handle all edge cases gracefully.

mod parser;
mod range;
mod version;

pub use parser::{
    allows_all, allows_any, intersects, max_satisfying, min_satisfying, parse_range, parse_version,
    satisfies, valid_range,
};
pub use range::{Comparator, Range, RangeError, RangeOperator};
pub use version::{Version, VersionError};
