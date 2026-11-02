//! Access to environment variables through the [Environment] trait.
//!
//! Code that reads variables through this trait, instead of [std::env], can be
//! tested with a fixed set of variables supplied by [MemoryEnvironment]. Tests
//! can't safely change the variables of the test process, because other tests
//! run in parallel and read them too.

use std::collections::BTreeMap;
use std::env;

/// A source of environment variables.
pub trait Environment: Send + Sync {
    /// Returns the value of the variable `name`, or `None` when the variable
    /// isn't set or its value isn't valid Unicode.
    fn var(&self, name: &str) -> Option<String>;

    /// Returns whether the variable `name` is set to a value that turns a
    /// feature on.
    ///
    /// Any value counts, except an empty value, `0`, and `false`. The
    /// comparison ignores surrounding whitespace and letter case, so
    /// `FALSE` and ` 0 ` are also treated as off.
    fn is_enabled(&self, name: &str) -> bool {
        self.var(name).is_some_and(|value| {
            let value = value.trim();
            !value.is_empty() && value != "0" && !value.eq_ignore_ascii_case("false")
        })
    }
}

/// Reads variables from the environment of the current process.
#[derive(Debug, Default, Clone, Copy)]
pub struct OsEnvironment;

impl Environment for OsEnvironment {
    fn var(&self, name: &str) -> Option<String> {
        env::var(name).ok()
    }
}

/// An environment that contains only the variables it was built with.
#[derive(Debug, Default, Clone)]
pub struct MemoryEnvironment {
    variables: BTreeMap<String, String>,
}

impl MemoryEnvironment {
    /// Returns this environment with the variable `name` set to `value`.
    pub fn with_var(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.variables.insert(name.into(), value.into());
        self
    }
}

impl Environment for MemoryEnvironment {
    fn var(&self, name: &str) -> Option<String> {
        self.variables.get(name).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_environment_returns_only_its_variables() {
        let environment = MemoryEnvironment::default().with_var("BIOME_TEST", "value");

        assert_eq!(environment.var("BIOME_TEST").as_deref(), Some("value"));
        assert_eq!(environment.var("PATH"), None);
    }

    #[test]
    fn is_enabled_rejects_off_values() {
        for value in ["", "  ", "0", " 0 ", "false", "FALSE", "False"] {
            let environment = MemoryEnvironment::default().with_var("FLAG", value);
            assert!(!environment.is_enabled("FLAG"), "{value:?} should be off");
        }

        assert!(!MemoryEnvironment::default().is_enabled("FLAG"));
    }

    #[test]
    fn is_enabled_accepts_other_values() {
        for value in ["1", "true", "TRUE", "yes", "anything"] {
            let environment = MemoryEnvironment::default().with_var("FLAG", value);
            assert!(environment.is_enabled("FLAG"), "{value:?} should be on");
        }
    }
}
