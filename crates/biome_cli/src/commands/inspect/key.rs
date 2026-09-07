//! Parses configuration keys supplied to `biome inspect config` and resolves them against typed
//! and serialized configurations.

use crate::CliDiagnostic;
use biome_configuration::{BiomeDiagnostic, Configuration};
use biome_service::WorkspaceError;
use serde_json::Value;

/// A validated dot-separated path into a serialized configuration.
///
/// A numeric segment indexes an array only when the preceding value is an array. The same segment
/// remains an object property when the preceding value is an object.
#[derive(Clone)]
pub(super) struct ConfigurationKey {
    text: String,
    segments: Vec<String>,
}

impl ConfigurationKey {
    pub(super) fn parse(text: String) -> Result<Self, CliDiagnostic> {
        let segments = text.split('.').map(str::to_string).collect::<Vec<_>>();
        if segments.is_empty() || segments.iter().any(String::is_empty) {
            return Err(CliDiagnostic::parse_error(
                "Configuration keys must be non-empty dot-separated property names. For example, formatter.lineWidth.",
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

    /// Returns the value after accounting for lint-rule group shorthand.
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

        Ok(self.rule_group_shorthand_in(&value).cloned())
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

    pub(super) fn is_declared_in(&self, value: &Value) -> bool {
        self.value_in(value)
            .or_else(|| self.rule_group_shorthand_in(value))
            .is_some_and(|value| !value.is_null())
    }

    pub(super) fn source_in_override(&self, value: &Value) -> Option<Self> {
        if self.is_declared_in(value) {
            return Some(self.clone());
        }
        let [language, section, property] = self.segments.as_slice() else {
            return None;
        };
        if !matches!(
            language.as_str(),
            "javascript" | "json" | "css" | "graphql" | "grit" | "html"
        ) || !matches!(
            (section.as_str(), property.as_str()),
            ("formatter", _) | ("linter" | "assist", "enabled")
        ) {
            return None;
        }
        let source = Self {
            text: format!("{section}.{property}"),
            segments: vec![section.clone(), property.clone()],
        };
        source.is_declared_in(value).then_some(source)
    }

    pub(super) fn append_array_index_in(&self, value: &Value) -> Option<(usize, usize)> {
        let mut value = value;
        for (position, segment) in self.segments.iter().enumerate() {
            match value {
                Value::Object(object) => value = object.get(segment)?,
                Value::Array(array) => {
                    let index = segment.parse::<usize>().ok()?;
                    if !self.is_append_array(position) || array.get(index).is_none() {
                        return None;
                    }
                    return Some((position, index));
                }
                _ => return None,
            }
        }
        None
    }

    pub(super) fn array_before_index<'a>(
        &self,
        value: &'a Value,
        index_position: usize,
    ) -> Option<&'a [Value]> {
        self.value_in_prefix(value, index_position)
            .and_then(Value::as_array)
            .map(Vec::as_slice)
    }

    pub(super) fn with_index(&self, position: usize, index: usize) -> Self {
        let mut segments = self.segments.clone();
        *segments
            .get_mut(position)
            .expect("the indexed segment should exist") = index.to_string();
        Self {
            text: segments.join("."),
            segments,
        }
    }

    fn is_append_array(&self, index_position: usize) -> bool {
        let path = &self.segments[..index_position];
        matches!(path, [property] if matches!(property.as_str(), "overrides" | "plugins"))
            || matches!(
                path,
                [section, property]
                    if property == "includes"
                        && matches!(
                            section.as_str(),
                            "files" | "formatter" | "linter" | "assist"
                        )
            )
            || matches!(path, [section, property] if section == "files" && property == "experimentalScannerIgnores")
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

    fn rule_group_shorthand_in<'a>(&self, value: &'a Value) -> Option<&'a Value> {
        match self.segments.as_slice() {
            [linter, rules, _, _] if linter == "linter" && rules == "rules" => self
                .value_in_prefix(value, 3)
                .filter(|value| value.is_string()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_configuration::{
        LinterConfiguration, Rules,
        analyzer::{GroupPlainConfiguration, SeverityOrGroup},
    };

    #[test]
    fn owns_dotted_value_traversal() {
        let key = ConfigurationKey::parse("overrides.0.formatter.lineWidth".to_string())
            .expect("valid configuration key");
        let value = serde_json::json!({
            "overrides": [{ "formatter": { "lineWidth": 100 } }]
        });

        assert_eq!(key.value_in(&value), Some(&Value::from(100)));
    }

    #[test]
    fn resolves_rule_group_shorthand_without_deserializing_configuration() {
        let configuration = Configuration {
            linter: Some(LinterConfiguration {
                rules: Some(Rules {
                    suspicious: Some(SeverityOrGroup::Plain(GroupPlainConfiguration::Error)),
                    ..Rules::default()
                }),
                ..LinterConfiguration::default()
            }),
            ..Configuration::default()
        };
        let key = ConfigurationKey::parse("linter.rules.suspicious.noConsole".to_string())
            .expect("valid configuration key");

        assert_eq!(
            key.value_in_configuration(&configuration)
                .expect("serializable configuration"),
            Some(Value::String("error".to_string()))
        );
    }

    #[test]
    fn only_rule_group_shorthand_uses_a_scalar_ancestor() {
        let value = serde_json::json!({
            "formatter": { "lineWidth": 100 },
            "linter": { "rules": { "suspicious": "error" } }
        });
        let unknown_formatter_key =
            ConfigurationKey::parse("formatter.lineWidth.extra".to_string())
                .expect("valid configuration key");
        let shorthand_rule =
            ConfigurationKey::parse("linter.rules.suspicious.noDebugger".to_string())
                .expect("valid configuration key");

        assert!(!unknown_formatter_key.is_declared_in(&value));
        assert!(shorthand_rule.is_declared_in(&value));
    }

    #[test]
    fn maps_language_fallbacks_to_the_global_override_key() {
        let key = ConfigurationKey::parse("javascript.formatter.lineWidth".to_string())
            .expect("valid configuration key");
        let value = serde_json::json!({ "formatter": { "lineWidth": 100 } });

        assert_eq!(
            key.source_in_override(&value).map(|source| source.text),
            Some("formatter.lineWidth".to_string())
        );
    }
}
