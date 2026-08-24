//! Resolves retained configuration inputs and traces effective values to their declarations.

use super::{display_path, key::ConfigurationKey};
use biome_configuration::{
    BiomeDiagnostic, Configuration, ConfigurationSource, ConfigurationSourceEntry, OverridePattern,
};
use biome_deserialize::{
    DeserializationContext, DeserializationDiagnostic, Merge, TextRange,
    json::deserialize_from_json_ast_with_context,
};
use biome_diagnostics::{DiagnosticExt, Error, Severity};
use biome_json_parser::{JsonParserOptions, parse_json};
use biome_service::WorkspaceError;
use camino::Utf8Path;
use serde_json::{Map, Value};
use std::collections::BTreeMap;

/// Owns the resolved view and parsed source metadata for one retained configuration graph.
///
/// The inspector borrows source text and typed configurations from `ConfigurationSource`. This
/// keeps paths, code frames, ranges, and merge inputs tied to the same load operation.
pub(super) struct ConfigurationInspector<'source> {
    files: InspectedConfigurationFiles<'source>,
    configuration: Configuration,
    serialized_configuration: Value,
}

impl<'source> ConfigurationInspector<'source> {
    pub(super) fn new(source: &'source ConfigurationSource) -> Result<Self, Vec<Error>> {
        let files = InspectedConfigurationFiles::new(source)?;
        let mut configuration = source.resolve();
        configuration.extends = None;
        if !files.root_explicitly_set() {
            configuration.root = None;
        }
        let serialized_configuration = serde_json::to_value(&configuration)
            .map_err(|_| vec![BiomeDiagnostic::new_serialization_error().into()])?;

        Ok(Self {
            files,
            configuration,
            serialized_configuration,
        })
    }

    pub(super) fn serialized_configuration(&self) -> &Value {
        &self.serialized_configuration
    }

    pub(super) fn configuration_paths(&self) -> impl Iterator<Item = &Utf8Path> {
        self.files.files.iter().map(|file| file.path)
    }

    pub(super) fn has_overrides(&self) -> bool {
        self.configuration.overrides.is_some()
    }

    pub(super) fn inspect_key<'inspection>(
        &'inspection self,
        key: &ConfigurationKey,
        matching_overrides: &[usize],
        matched_path: Option<&str>,
    ) -> Result<KeyInspection<'inspection>, WorkspaceError> {
        let mut value = key.value_in_configuration(&self.configuration)?;
        let mut sources = if value.is_some() {
            self.files.base_sources(&self.configuration, key)?
        } else {
            Vec::new()
        };
        let override_patterns = self
            .configuration
            .overrides
            .as_ref()
            .map(|overrides| overrides.0.as_slice())
            .unwrap_or_default();
        let mut effective_configuration = self.configuration.clone();

        for &override_index in matching_overrides {
            let Some(pattern) = override_patterns.get(override_index) else {
                continue;
            };
            let previous_value = value.clone();
            pattern.apply_to_configuration(&mut effective_configuration, &self.configuration);
            value = key.value_in_configuration(&effective_configuration)?;
            let Some(source) = self.files.override_source(
                &self.serialized_configuration,
                key,
                override_index,
                matched_path,
            ) else {
                continue;
            };
            if previous_value != value && !source.declares_key {
                sources = if value.is_some() {
                    self.files.base_sources(&self.configuration, key)?
                } else {
                    Vec::new()
                };
            } else if previous_value != value || source.declares_key {
                if value.as_ref().is_some_and(JsonValueExt::is_scalar) {
                    sources = vec![source];
                } else {
                    sources.push(source);
                }
            }
        }

        if value
            .as_ref()
            .is_some_and(|value| !JsonValueExt::is_scalar(value))
        {
            sources = self.files.composite_sources(
                override_patterns,
                matching_overrides,
                &self.serialized_configuration,
                key,
                matched_path,
            )?;
        }

        Ok(KeyInspection { value, sources })
    }
}

/// The effective value of a key and the declarations that contribute to it.
pub(super) struct KeyInspection<'source> {
    pub(super) value: Option<Value>,
    pub(super) sources: Vec<SourceReference<'source>>,
}

impl KeyInspection<'_> {
    pub(super) fn source_json(&self) -> Option<Value> {
        match self.sources.as_slice() {
            [] => None,
            [source] => Some(source.to_json()),
            sources => Some(Value::Object(Map::from_iter([
                (
                    "configurationKind".to_string(),
                    Value::String("merged".to_string()),
                ),
                (
                    "contributors".to_string(),
                    Value::Array(sources.iter().map(|source| source.to_json()).collect()),
                ),
            ]))),
        }
    }
}

/// Identifies whether a declaration belongs to the root file or an extended file.
pub(super) enum ConfigurationKind {
    Root,
    Extend { specifier: Option<String> },
}

struct InspectedConfigurationFile<'source> {
    path: &'source Utf8Path,
    source: &'source str,
    value: Value,
    configuration: &'source Configuration,
    ranges: BTreeMap<String, TextRange>,
    kind: ConfigurationKind,
}

/// Parsed retained files in merge order: extended configurations followed by the root.
struct InspectedConfigurationFiles<'source> {
    files: Vec<InspectedConfigurationFile<'source>>,
}

impl<'source> InspectedConfigurationFiles<'source> {
    fn new(configuration_source: &'source ConfigurationSource) -> Result<Self, Vec<Error>> {
        let mut files = Vec::new();
        for extended in configuration_source.extended_configurations.as_slice() {
            files.push(InspectedConfigurationFile::new(
                &extended.source,
                ConfigurationKind::Extend {
                    specifier: extended.specifier.clone(),
                },
            )?);
        }
        if let Some(root) = &configuration_source.root {
            files.push(InspectedConfigurationFile::new(
                root,
                ConfigurationKind::Root,
            )?);
        }
        Ok(Self { files })
    }

    fn root_explicitly_set(&self) -> bool {
        self.files
            .iter()
            .find(|file| matches!(file.kind, ConfigurationKind::Root))
            .is_some_and(|file| file.value.get("root").is_some())
    }

    fn base_sources<'inspection>(
        &'inspection self,
        configuration: &Configuration,
        key: &ConfigurationKey,
    ) -> Result<Vec<SourceReference<'inspection>>, WorkspaceError> {
        let final_value = key.value_in_configuration(configuration)?;
        if !final_value.as_ref().is_some_and(JsonValueExt::is_scalar) {
            let merged = self.merge_base_files(None, key);
            let merged_value = key.value_in_configuration(&merged)?;
            let mut sources = Vec::new();
            for (file_index, file) in self.files.iter().enumerate() {
                let without = self.merge_base_files(Some(file_index), key);
                if key.value_in_configuration(&without)? != merged_value {
                    sources.push(file.source_reference(
                        None,
                        key,
                        SourceScope::Base,
                        None,
                        None,
                        None,
                    ));
                }
            }
            return Ok(sources);
        }

        let mut sources = Vec::new();
        let mut merged = Configuration::default();
        let mut previous_value = key.value_in_configuration(&merged)?;
        for file in &self.files {
            file.merge_into(&mut merged, key);
            let value = key.value_in_configuration(&merged)?;
            if previous_value != value || file.declares_key(None, key) {
                sources =
                    vec![file.source_reference(None, key, SourceScope::Base, None, None, None)];
            }
            previous_value = value;
        }
        Ok(sources)
    }

    fn merge_base_files(
        &self,
        skipped_file: Option<usize>,
        key: &ConfigurationKey,
    ) -> Configuration {
        let mut merged = Configuration::default();
        for (file_index, file) in self.files.iter().enumerate() {
            if Some(file_index) != skipped_file {
                file.merge_into(&mut merged, key);
            }
        }
        merged
    }

    fn override_source<'inspection>(
        &'inspection self,
        configuration: &Value,
        key: &ConfigurationKey,
        override_index: usize,
        matched_path: Option<&str>,
    ) -> Option<SourceReference<'inspection>> {
        let mut first_index = 0;
        for file in &self.files {
            let override_count = file
                .value
                .get("overrides")
                .and_then(Value::as_array)
                .map_or(0, Vec::len);
            if override_index < first_index + override_count {
                let local_index = override_index - first_index;
                let key_prefix = format!("overrides.{local_index}");
                let includes = configuration
                    .get("overrides")
                    .and_then(Value::as_array)
                    .and_then(|overrides| overrides.get(override_index))
                    .and_then(|pattern| pattern.get("includes"))
                    .and_then(Value::as_array)
                    .map(|patterns| {
                        patterns
                            .iter()
                            .filter_map(Value::as_str)
                            .map(str::to_string)
                            .collect()
                    });
                return Some(file.source_reference(
                    Some(&key_prefix),
                    key,
                    SourceScope::Override,
                    Some(override_index),
                    includes,
                    matched_path.map(str::to_string),
                ));
            }
            first_index += override_count;
        }
        None
    }

    fn composite_sources<'inspection>(
        &'inspection self,
        override_patterns: &[OverridePattern],
        matching_overrides: &[usize],
        configuration: &Value,
        key: &ConfigurationKey,
        matched_path: Option<&str>,
    ) -> Result<Vec<SourceReference<'inspection>>, WorkspaceError> {
        let merged = self.replay_effective_configuration(
            override_patterns,
            matching_overrides,
            None,
            None,
            key,
        );
        let merged_value = key.value_in_configuration(&merged)?;
        let mut sources = Vec::new();

        for (file_index, file) in self.files.iter().enumerate() {
            let without = self.replay_effective_configuration(
                override_patterns,
                matching_overrides,
                Some(file_index),
                None,
                key,
            );
            if key.value_in_configuration(&without)? != merged_value {
                sources.push(file.source_reference(None, key, SourceScope::Base, None, None, None));
            }
        }

        for &override_index in matching_overrides {
            let without = self.replay_effective_configuration(
                override_patterns,
                matching_overrides,
                None,
                Some(override_index),
                key,
            );
            if key.value_in_configuration(&without)? != merged_value
                && let Some(source) =
                    self.override_source(configuration, key, override_index, matched_path)
            {
                sources.push(source);
            }
        }

        if sources.is_empty() {
            if let Some(&override_index) = matching_overrides.iter().rev().find(|&&index| {
                self.override_source(configuration, key, index, matched_path)
                    .is_some_and(|source| source.declares_key)
            }) {
                if let Some(source) =
                    self.override_source(configuration, key, override_index, matched_path)
                {
                    sources.push(source);
                }
            } else if let Some(file) = self
                .files
                .iter()
                .rev()
                .find(|file| file.declares_key(None, key))
            {
                sources.push(file.source_reference(None, key, SourceScope::Base, None, None, None));
            }
        }

        Ok(sources)
    }

    fn replay_effective_configuration(
        &self,
        override_patterns: &[OverridePattern],
        matching_overrides: &[usize],
        skipped_file: Option<usize>,
        skipped_override: Option<usize>,
        key: &ConfigurationKey,
    ) -> Configuration {
        let mut configuration = self.merge_base_files(skipped_file, key);
        let base_configuration = configuration.clone();
        for &override_index in matching_overrides {
            if Some(override_index) == skipped_override {
                continue;
            }
            if let Some(pattern) = override_patterns.get(override_index) {
                pattern.apply_to_configuration(&mut configuration, &base_configuration);
            }
        }
        configuration
    }
}

impl<'source> InspectedConfigurationFile<'source> {
    fn new(
        document: &'source ConfigurationSourceEntry,
        kind: ConfigurationKind,
    ) -> Result<Self, Vec<Error>> {
        let Some(path) = document.file_path.as_ref() else {
            return Err(vec![
                BiomeDiagnostic::invalid_configuration(
                    "The configuration source does not have a file path.",
                )
                .into(),
            ]);
        };
        let Some(source) = document.file_source.as_ref() else {
            return Err(vec![
                BiomeDiagnostic::invalid_configuration(
                    "The configuration source does not have file contents.",
                )
                .into(),
            ]);
        };
        let options = match path.extension() {
            Some("json") => JsonParserOptions::default(),
            _ => JsonParserOptions::default()
                .with_allow_comments()
                .with_allow_trailing_commas(),
        };
        let parse = parse_json(source, options);
        let mut context = SourceRangeCollector::default();
        let value = deserialize_from_json_ast_with_context::<Value>(&parse.tree(), &mut context);
        let mut diagnostics = parse
            .into_diagnostics()
            .into_iter()
            .map(Error::from)
            .chain(context.diagnostics)
            .map(|diagnostic| {
                diagnostic
                    .with_file_path(path.to_string())
                    .with_file_source_code(source.to_string())
            })
            .collect::<Vec<_>>();
        let is_invalid = diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity() >= Severity::Error)
            || value.is_none();
        if is_invalid {
            if diagnostics.is_empty() {
                diagnostics.push(
                    Error::from(BiomeDiagnostic::invalid_configuration(
                        "The retained configuration source could not be inspected.",
                    ))
                    .with_file_path(path.to_string())
                    .with_file_source_code(source.to_string()),
                );
            }
            return Err(diagnostics);
        }

        let (Some(value), Some(configuration)) = (value, document.configuration.as_ref()) else {
            return Err(diagnostics);
        };
        Ok(Self {
            path,
            source,
            value,
            configuration,
            ranges: context.entries,
            kind,
        })
    }

    fn merge_into(&self, merged: &mut Configuration, key: &ConfigurationKey) {
        let mut configuration = self.configuration.clone();
        configuration.extends = None;
        if !matches!(&self.kind, ConfigurationKind::Root) {
            configuration.root = None;
        }
        if key.as_str() != "overrides" && !key.as_str().starts_with("overrides.") {
            configuration.overrides = None;
        }
        merged.merge_with(configuration);
    }

    fn range(&self, prefix: Option<&str>, key: &ConfigurationKey) -> Option<TextRange> {
        let mut range_key = key.with_prefix(prefix).as_str().to_string();
        loop {
            if let Some(range) = self.ranges.get(&range_key) {
                return Some(*range);
            }
            let (parent, _) = range_key.rsplit_once('.')?;
            range_key.truncate(parent.len());
        }
    }

    fn declares_key(&self, prefix: Option<&str>, key: &ConfigurationKey) -> bool {
        let full_key = key.with_prefix(prefix);
        if let Some(value) = full_key.value_in(&self.value) {
            return !value.is_null();
        }
        full_key.has_non_null_scalar_ancestor_in(&self.value)
    }

    fn source_reference<'inspection>(
        &'inspection self,
        prefix: Option<&str>,
        key: &ConfigurationKey,
        scope: SourceScope,
        override_index: Option<usize>,
        includes: Option<Vec<String>>,
        matched_path: Option<String>,
    ) -> SourceReference<'inspection> {
        SourceReference {
            path: self.path,
            source: self.source,
            range: self.range(prefix, key),
            kind: &self.kind,
            declares_key: self.declares_key(prefix, key),
            scope,
            override_index,
            includes,
            matched_path,
        }
    }
}

enum PathSegment {
    Property(String),
    Index(usize),
}

/// Collects value ranges while the retained JSON syntax tree is deserialized.
#[derive(Default)]
struct SourceRangeCollector {
    entries: BTreeMap<String, TextRange>,
    diagnostics: Vec<Error>,
    path: Vec<PathSegment>,
}

impl SourceRangeCollector {
    fn record(&mut self, range: TextRange) {
        let key = self
            .path
            .iter()
            .map(|segment| match segment {
                PathSegment::Property(name) => name.clone(),
                PathSegment::Index(index) => index.to_string(),
            })
            .collect::<Vec<_>>()
            .join(".");
        self.entries.insert(key, range);
    }
}

impl DeserializationContext for SourceRangeCollector {
    fn id(&self) -> Option<&str> {
        None
    }

    fn report(&mut self, diagnostic: DeserializationDiagnostic) {
        self.diagnostics.push(Error::from(diagnostic));
    }

    fn enter_property(&mut self, name: &str, _key_range: TextRange, value_range: TextRange) {
        self.path.push(PathSegment::Property(name.to_string()));
        self.record(value_range);
    }

    fn exit_property(&mut self) {
        self.path.pop();
    }

    fn enter_index(&mut self, index: usize, range: TextRange) {
        self.path.push(PathSegment::Index(index));
        self.record(range);
    }

    fn exit_index(&mut self) {
        self.path.pop();
    }
}

trait JsonValueExt {
    fn is_scalar(&self) -> bool;
}

impl JsonValueExt for Value {
    fn is_scalar(&self) -> bool {
        !matches!(self, Self::Array(_) | Self::Object(_))
    }
}

/// Identifies whether a value comes from a base configuration or a matching override.
#[derive(Clone, Copy)]
pub(super) enum SourceScope {
    Base,
    Override,
}

/// A declaration that contributes to an inspected value and its diagnostic metadata.
pub(super) struct SourceReference<'source> {
    pub(super) path: &'source Utf8Path,
    pub(super) source: &'source str,
    pub(super) range: Option<TextRange>,
    pub(super) kind: &'source ConfigurationKind,
    declares_key: bool,
    pub(super) scope: SourceScope,
    pub(super) override_index: Option<usize>,
    pub(super) includes: Option<Vec<String>>,
    pub(super) matched_path: Option<String>,
}

impl SourceReference<'_> {
    fn to_json(&self) -> Value {
        let mut result = Map::new();
        let configuration_kind = match self.kind {
            ConfigurationKind::Root => "root",
            ConfigurationKind::Extend { .. } => "extend",
        };
        result.insert(
            "configurationKind".to_string(),
            Value::String(configuration_kind.to_string()),
        );
        result.insert(
            "scope".to_string(),
            Value::String(
                match self.scope {
                    SourceScope::Base => "base",
                    SourceScope::Override => "override",
                }
                .to_string(),
            ),
        );
        result.insert(
            "path".to_string(),
            Value::String(display_path(self.path).into_owned()),
        );
        if let Some(range) = self.range {
            result.insert(
                "range".to_string(),
                serde_json::json!({
                    "start": u32::from(range.start()),
                    "end": u32::from(range.end()),
                }),
            );
        }
        if let ConfigurationKind::Extend {
            specifier: Some(specifier),
        } = self.kind
        {
            result.insert("specifier".to_string(), Value::String(specifier.clone()));
        }
        if let Some(index) = self.override_index {
            result.insert("overrideIndex".to_string(), Value::from(index));
        }
        if let Some(includes) = &self.includes {
            result.insert(
                "includes".to_string(),
                Value::Array(includes.iter().cloned().map(Value::String).collect()),
            );
        }
        if let Some(path) = &self.matched_path {
            result.insert("matchedPath".to_string(), Value::String(path.clone()));
        }
        Value::Object(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use camino::Utf8PathBuf;

    #[test]
    fn provenance_parse_uses_the_retained_configuration() {
        let source = ConfigurationSourceEntry {
            configuration: Some(Configuration::default()),
            file_path: Some(Utf8PathBuf::from("biome.json")),
            file_source: Some(r#"{ "formatter": { "lineWidth": "wide" } }"#.into()),
        };
        let result = InspectedConfigurationFile::new(&source, ConfigurationKind::Root)
            .expect("valid JSON source");

        assert_eq!(result.configuration, &Configuration::default());
    }
}
