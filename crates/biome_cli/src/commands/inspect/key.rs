//! Parses configuration keys supplied to `biome inspect config` and resolves them against typed
//! and serialized configurations.

use crate::CliDiagnostic;
use biome_configuration::{BiomeDiagnostic, Configuration};
use biome_deserialize::Merge;
use biome_service::WorkspaceError;
use serde_json::{Map, Value};

/// A validated dot-separated path into a serialized configuration.
///
/// A numeric segment indexes an array only when the preceding value is an array. The same segment
/// remains an object property when the preceding value is an object.
#[derive(Debug)]
pub(super) struct ConfigurationKey {
    text: String,
    segments: Vec<String>,
}

impl ConfigurationKey {
    pub(super) fn parse(text: String) -> Result<Self, CliDiagnostic> {
        let segments = text.split('.').map(str::to_string).collect::<Vec<_>>();
        if segments.is_empty() || segments.iter().any(String::is_empty) {
            return Err(CliDiagnostic::incompatible_end_configuration(
                "Configuration keys must be non-empty dot-separated property names. For example formatter.lineWidth",
            ));
        }
        Ok(Self { text, segments })
    }

    pub(super) fn as_str(&self) -> &str {
        &self.text
    }

    pub(super) fn value_in<'a>(&self, value: &'a Value) -> Option<&'a Value> {
        self.value_in_prefix(value, self.segments.len())
    }

    /// Returns the value after applying the typed configuration's shorthand normalization.
    ///
    /// The normalization step allows a key below a scalar group shorthand to resolve even when the
    /// serialized configuration does not contain the requested leaf.
    pub(super) fn value_in_configuration(
        &self,
        configuration: &Configuration,
    ) -> Result<Option<Value>, WorkspaceError> {
        let value = serde_json::to_value(configuration)
            .map_err(|_| BiomeDiagnostic::new_serialization_error())?;
        if let Some(value) = self.value_in(&value) {
            return Ok(Some(value.clone()));
        }

        for parent_len in (1..self.segments.len()).rev() {
            let probe = self
                .segments
                .iter()
                .take(parent_len)
                .rev()
                .fold(Value::Object(Map::new()), |value, segment| {
                    Value::Object(Map::from_iter([(segment.clone(), value)]))
                });
            let Ok(probe) = serde_json::from_value::<Configuration>(probe) else {
                continue;
            };
            let mut normalized = configuration.clone();
            normalized.merge_with(probe);
            let normalized = serde_json::to_value(normalized)
                .map_err(|_| BiomeDiagnostic::new_serialization_error())?;
            if let Some(value) = self.value_in(&normalized) {
                return Ok(Some(value.clone()));
            }
        }

        Ok(None)
    }

    pub(super) fn with_prefix(&self, prefix: Option<&str>) -> Self {
        let Some(prefix) = prefix else {
            return Self {
                text: self.text.clone(),
                segments: self.segments.clone(),
            };
        };
        let text = format!("{prefix}.{}", self.text);
        let segments = prefix
            .split('.')
            .map(str::to_string)
            .chain(self.segments.iter().cloned())
            .collect();
        Self { text, segments }
    }

    pub(super) fn has_non_null_scalar_ancestor_in(&self, value: &Value) -> bool {
        (1..self.segments.len()).rev().any(|segment_count| {
            self.value_in_prefix(value, segment_count)
                .is_some_and(|value| {
                    !matches!(value, Value::Array(_) | Value::Object(_)) && !value.is_null()
                })
        })
    }

    fn value_in_prefix<'a>(&self, value: &'a Value, segment_count: usize) -> Option<&'a Value> {
        self.segments
            .iter()
            .take(segment_count)
            .try_fold(value, |value, segment| match value {
                Value::Object(object) => object.get(segment),
                Value::Array(array) => segment
                    .parse::<usize>()
                    .ok()
                    .and_then(|index| array.get(index)),
                _ => None,
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn owns_dotted_value_traversal() {
        let key = ConfigurationKey::parse("overrides.0.formatter.lineWidth".to_string())
            .expect("valid configuration key");
        let value = serde_json::json!({
            "overrides": [{ "formatter": { "lineWidth": 100 } }]
        });

        assert_eq!(key.value_in(&value), Some(&Value::from(100)));
    }
}
