#![deny(clippy::use_self)]

use biome_console::markup;
use biome_deserialize::json::deserialize_from_json_str;
use biome_deserialize::{
    Deserializable, DeserializableType, DeserializableTypes, DeserializableValidator,
    DeserializableValue, DeserializationContext, DeserializationDiagnostic, DeserializationVisitor,
    MapMembers,
};
use biome_deserialize_macros::Deserializable;
use biome_diagnostics::{Diagnostic, Error, MessageAndDescription};
use biome_fs::{FileSystem, FileSystemDiagnostic, ManifestName};
use biome_json_parser::JsonParserOptions;
use biome_resolver::package_specifier_parts;
use biome_rowan::TextRange;
use camino::Utf8Path;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::ops::Deref;

/// Declares the rules and configurations provided by a Biome package.
#[derive(Clone, Debug, Default, Deserializable, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[deserializable(unknown_fields = "deny", with_validator)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BiomeManifest {
    /// An optional JSON Schema reference used by editors.
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,

    /// The manifest format version.
    #[deserializable(required, validate = "supported_version")]
    pub version: u8,

    /// Named plugin rules and presets provided by the package.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugins: Option<ManifestPlugins>,

    /// Named configurations and selected configurations from other packages.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub configs: Vec<ManifestEntry>,
}

/// Named lint rules and groups of rules provided by a Biome package.
#[derive(Clone, Debug, Default, Deserializable, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[deserializable(unknown_fields = "deny")]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ManifestPlugins {
    /// Named rules and selected rules or presets from other packages.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<ManifestEntry>,

    /// Named groups of rules that can be loaded together.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presets: Option<ManifestPresets>,
}

/// Named groups of rules that can be loaded together.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(transparent)]
pub struct ManifestPresets(BTreeMap<String, Vec<String>>);

impl Deref for ManifestPresets {
    type Target = BTreeMap<String, Vec<String>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(feature = "schema")]
impl schemars::JsonSchema for ManifestPresets {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("ManifestPresets")
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let mut schema =
            <BTreeMap<String, Vec<String>> as schemars::JsonSchema>::json_schema(generator);
        if let Some(preset) = schema
            .get_mut("additionalProperties")
            .and_then(|value| value.as_object_mut())
        {
            preset.insert("minItems".into(), 1.into());
        }
        schema.insert("minProperties".into(), 1.into());
        schema
    }
}

impl Deserializable for ManifestPresets {
    fn deserialize(
        ctx: &mut dyn DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        struct PresetsVisitor;
        impl DeserializationVisitor for PresetsVisitor {
            type Output = ManifestPresets;
            const EXPECTED_TYPE: DeserializableTypes = DeserializableTypes::MAP;

            fn visit_map(
                self,
                ctx: &mut dyn DeserializationContext,
                members: &mut MapMembers<'_>,
                _range: TextRange,
                _name: &str,
            ) -> Option<Self::Output> {
                let mut presets = BTreeMap::new();
                for (key, value) in members.flatten() {
                    let key_range = key.range();
                    let key: Option<String> = Deserializable::deserialize(ctx, &key, "");
                    let value: Option<Vec<String>> = Deserializable::deserialize(ctx, &value, "");
                    if let Some(key) = &key
                        && !is_valid_export_name(key)
                    {
                        report_invalid_export_name(ctx, key, "preset", key_range);
                    }
                    if let (Some(key), Some(value)) = (key, value)
                        && presets.insert(key.clone(), value).is_some()
                    {
                        ctx.report(
                            DeserializationDiagnostic::new(markup! {
                                "The key "<Emphasis>{key}</Emphasis>" is specified more than once."
                            })
                            .with_range(key_range),
                        );
                    }
                }
                Some(ManifestPresets(presets))
            }
        }

        value.deserialize(ctx, PresetsVisitor, name)
    }
}

/// A selected package export or a map from public names to package-relative files.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum ManifestEntry {
    /// A named export selected from an installed package.
    Package(String),

    /// Public names mapped to package-relative files.
    Paths(BTreeMap<String, String>),
}

impl Deserializable for ManifestEntry {
    fn deserialize(
        ctx: &mut dyn DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        if value.visitable_type()? == DeserializableType::Str {
            Deserializable::deserialize(ctx, value, name).map(Self::Package)
        } else {
            struct NamedEntriesVisitor;
            impl DeserializationVisitor for NamedEntriesVisitor {
                type Output = BTreeMap<String, String>;
                const EXPECTED_TYPE: DeserializableTypes = DeserializableTypes::MAP;

                fn visit_map(
                    self,
                    ctx: &mut dyn DeserializationContext,
                    members: &mut MapMembers<'_>,
                    _range: TextRange,
                    _name: &str,
                ) -> Option<Self::Output> {
                    let mut entries = BTreeMap::new();
                    for (key, value) in members.flatten() {
                        let key_range = key.range();
                        let key: Option<String> = Deserializable::deserialize(ctx, &key, "");
                        let value: Option<String> = Deserializable::deserialize(ctx, &value, "");
                        if let Some(key) = &key
                            && !is_valid_export_name(key)
                        {
                            report_invalid_export_name(ctx, key, "entry", key_range);
                        }
                        if let (Some(key), Some(value)) = (key, value)
                            && entries.insert(key.clone(), value).is_some()
                        {
                            ctx.report(
                                DeserializationDiagnostic::new(markup! {
                                    "The key "<Emphasis>{key}</Emphasis>" is specified more than once."
                                })
                                .with_range(key_range),
                            );
                        }
                    }
                    Some(entries)
                }
            }

            value
                .deserialize(ctx, NamedEntriesVisitor, name)
                .map(Self::Paths)
        }
    }
}

impl DeserializableValidator for BiomeManifest {
    fn validate(
        &mut self,
        ctx: &mut dyn DeserializationContext,
        _name: &str,
        range: TextRange,
    ) -> bool {
        let mut valid = true;
        for entry in self.plugins.iter().flat_map(|plugins| &plugins.rules) {
            match entry {
                ManifestEntry::Package(specifier) if !is_rule_package_specifier(specifier) => {
                    ctx.report(
                        DeserializationDiagnostic::new(markup! {
                            <Emphasis>{specifier}</Emphasis>
                            " must select a named rule or preset from a package."
                        })
                        .with_range(range),
                    );
                    valid = false;
                }
                ManifestEntry::Paths(paths) if paths.is_empty() => {
                    ctx.report(
                        DeserializationDiagnostic::new(
                            markup! { "Named manifest entries must not be empty." },
                        )
                        .with_range(range),
                    );
                    valid = false;
                }
                ManifestEntry::Package(_) | ManifestEntry::Paths(_) => {}
            }
        }
        for entry in &self.configs {
            match entry {
                ManifestEntry::Package(specifier) if !is_config_package_specifier(specifier) => {
                    ctx.report(
                        DeserializationDiagnostic::new(markup! {
                            <Emphasis>{specifier}</Emphasis>
                            " must select a named configuration from a package."
                        })
                        .with_range(range),
                    );
                    valid = false;
                }
                ManifestEntry::Paths(paths) if paths.is_empty() => {
                    ctx.report(
                        DeserializationDiagnostic::new(
                            markup! { "Named manifest entries must not be empty." },
                        )
                        .with_range(range),
                    );
                    valid = false;
                }
                ManifestEntry::Package(_) | ManifestEntry::Paths(_) => {}
            }
        }

        if let Some(plugins) = &self.plugins
            && let Some(presets) = &plugins.presets
        {
            if presets.is_empty() {
                ctx.report(
                    DeserializationDiagnostic::new(
                        markup! { "The plugin presets must contain at least one preset." },
                    )
                    .with_range(range),
                );
                valid = false;
            }
            for (preset, rules) in presets.iter() {
                if rules.is_empty() {
                    ctx.report(
                        DeserializationDiagnostic::new(markup! {
                            "Preset "<Emphasis>{preset}</Emphasis>" must contain at least one rule."
                        })
                        .with_range(range),
                    );
                    valid = false;
                }
            }
        }

        let rule_entries = self
            .plugins
            .as_ref()
            .map_or(&[][..], |plugins| plugins.rules.as_slice());
        for (kind, entries) in [
            ("rule", rule_entries),
            ("configuration", self.configs.as_slice()),
        ] {
            let mut names = BTreeSet::new();
            for entry in entries {
                let ManifestEntry::Paths(paths) = entry else {
                    continue;
                };
                for name in paths.keys() {
                    if !names.insert(name) {
                        ctx.report(
                            DeserializationDiagnostic::new(markup! {
                                "The "{kind}" name "<Emphasis>{name}</Emphasis>" is declared more than once."
                            })
                            .with_range(range),
                        );
                        valid = false;
                    }
                }
            }
        }

        if rule_entries.is_empty() && self.configs.is_empty() {
            ctx.report(
                DeserializationDiagnostic::new(markup! {
                    "The Biome manifest must contain at least one rule or configuration."
                })
                .with_range(range),
            );
            valid = false;
        }
        valid
    }
}

impl BiomeManifest {
    /// Loads and validates a Biome manifest from `path`.
    ///
    /// # Errors
    ///
    /// Returns an error when the file cannot be read or contains an invalid manifest.
    pub fn load(fs: &dyn FileSystem, path: &Utf8Path) -> Result<Self, BiomeManifestError> {
        let parser_options = if path.file_name() == Some(ManifestName::biome_manifest_jsonc()) {
            JsonParserOptions::default()
                .with_allow_comments()
                .with_allow_trailing_commas()
        } else {
            JsonParserOptions::default()
        };
        let content = fs.read_file_from_path(path)?;
        let (manifest, errors) =
            deserialize_from_json_str::<Self>(&content, parser_options, "").consume();
        manifest.filter(|_| errors.is_empty()).ok_or_else(|| {
            InvalidBiomeManifest::new(path.to_string(), errors.into_iter().next()).into()
        })
    }
}

/// An error emitted while loading a Biome manifest.
#[derive(Debug, Deserialize, Diagnostic, Serialize)]
pub enum BiomeManifestError {
    /// The manifest could not be read from the file system.
    FileSystem(FileSystemDiagnostic),

    /// The manifest contents are invalid.
    Invalid(InvalidBiomeManifest),
}

impl From<FileSystemDiagnostic> for BiomeManifestError {
    fn from(value: FileSystemDiagnostic) -> Self {
        Self::FileSystem(value)
    }
}

impl From<InvalidBiomeManifest> for BiomeManifestError {
    fn from(value: InvalidBiomeManifest) -> Self {
        Self::Invalid(value)
    }
}

/// A Biome manifest that could be read but not deserialized or validated.
#[derive(Debug, Deserialize, Diagnostic, Serialize)]
#[diagnostic(category = "configuration", severity = Error)]
pub struct InvalidBiomeManifest {
    #[message]
    #[description]
    message: MessageAndDescription,

    path: String,

    #[serde(skip)]
    #[source]
    source: Option<Error>,
}

impl InvalidBiomeManifest {
    fn new(path: String, source: Option<Error>) -> Self {
        Self {
            message: MessageAndDescription::from(
                markup!("Cannot load Biome manifest "<Emphasis>{path}</Emphasis>".").to_owned(),
            ),
            path,
            source,
        }
    }

    /// Returns the manifest path and the diagnostic that caused validation to fail.
    pub fn into_parts(self) -> (String, Option<Error>) {
        (self.path, self.source)
    }
}

fn supported_version(
    ctx: &mut dyn DeserializationContext,
    value: &u8,
    name: &str,
    range: TextRange,
) -> bool {
    if *value == 1 {
        true
    } else {
        ctx.report(
            DeserializationDiagnostic::new(markup! {
                <Emphasis>{name}</Emphasis>" must be 1."
            })
            .with_range(range),
        );
        false
    }
}

fn is_valid_export_name(name: &str) -> bool {
    !name.is_empty()
        && !name.contains('/')
        && !name
            .chars()
            .any(|character| character == ':' || character == '(' || character.is_whitespace())
}

fn is_rule_package_specifier(specifier: &str) -> bool {
    let Ok((_, selection)) = package_specifier_parts(specifier) else {
        return false;
    };
    selection
        .strip_prefix("presets/")
        .map_or_else(|| is_valid_export_name(selection), is_valid_export_name)
}

fn is_config_package_specifier(specifier: &str) -> bool {
    package_specifier_parts(specifier)
        .ok()
        .and_then(|(_, selection)| selection.strip_prefix("configs/"))
        .is_some_and(is_valid_export_name)
}

fn report_invalid_export_name(
    ctx: &mut dyn DeserializationContext,
    name: &str,
    kind: &str,
    range: TextRange,
) {
    ctx.report(
        DeserializationDiagnostic::new(markup! {
            "The manifest "{kind}" name "<Emphasis>{name}</Emphasis>
            " must not contain slashes, whitespace, "<Emphasis>":"</Emphasis>
            ", or "<Emphasis>"("</Emphasis>"."
        })
        .with_range(range),
    );
}

#[cfg(test)]
mod tests {
    use super::{BiomeManifest, ManifestEntry};
    use biome_deserialize::json::deserialize_from_json_str;
    use biome_json_parser::JsonParserOptions;

    fn assert_manifest_snapshot(name: &str, source: &str) {
        let diagnostics =
            deserialize_from_json_str::<BiomeManifest>(source, JsonParserOptions::default(), "")
                .into_diagnostics()
                .into_iter()
                .map(|diagnostic| biome_diagnostics::print_diagnostic_to_string(&diagnostic))
                .collect::<Vec<_>>()
                .join("\n");

        insta::with_settings!({ prepend_module_to_snapshot => false }, {
            insta::assert_snapshot!(name, diagnostics);
        });
    }

    #[test]
    fn accepts_configuration_only_manifests() {
        let manifest = deserialize_from_json_str::<BiomeManifest>(
            r#"{
                "version": 1,
                "configs": [
                    "@org/shared-configs/configs/recommended",
                    { "recommended": "./biome.json" }
                ]
            }"#,
            JsonParserOptions::default(),
            "",
        );

        assert!(!manifest.has_errors());
    }

    #[test]
    fn accepts_named_rules_and_presets() {
        let manifest = deserialize_from_json_str::<BiomeManifest>(
            r#"{
                "version": 1,
                "plugins": {
                    "rules": [
                        "@org/shared-rules/noDeprecatedApi",
                        "@org/strict-rules/presets/recommended",
                        { "useCompanyLogger": "./rules/useCompanyLogger.grit" }
                    ],
                    "presets": {
                        "recommended": ["useCompanyLogger"]
                    }
                }
            }"#,
            JsonParserOptions::default(),
            "",
        )
        .into_deserialized()
        .expect("valid manifest");

        let plugins = manifest.plugins.expect("plugins");
        assert_eq!(plugins.rules.len(), 3);
        assert!(matches!(
            &plugins.rules[0],
            ManifestEntry::Package(package) if package == "@org/shared-rules/noDeprecatedApi"
        ));
        assert_eq!(
            plugins.presets.expect("presets")["recommended"],
            ["useCompanyLogger"]
        );
    }

    #[test]
    fn rejects_file_paths_in_string_entries() {
        let manifest = deserialize_from_json_str::<BiomeManifest>(
            r#"{ "version": 1, "configs": ["./biome.json"] }"#,
            JsonParserOptions::default(),
            "",
        );

        assert!(manifest.has_errors());
    }

    #[test]
    fn rejects_bare_package_rule_imports() {
        let manifest = deserialize_from_json_str::<BiomeManifest>(
            r#"{
                "version": 1,
                "plugins": {
                    "rules": ["@org/shared-rules"]
                }
            }"#,
            JsonParserOptions::default(),
            "",
        );

        assert!(manifest.has_errors());
    }

    #[test]
    fn rejects_nested_package_rule_imports() {
        for specifier in ["middle/leaf/rule", "middle/presets/team/recommended"] {
            let manifest = deserialize_from_json_str::<BiomeManifest>(
                &format!(
                    r#"{{
                        "version": 1,
                        "plugins": {{ "rules": ["{specifier}"] }}
                    }}"#
                ),
                JsonParserOptions::default(),
                "",
            );

            assert!(manifest.has_errors(), "{specifier} should be rejected");
        }
    }

    #[test]
    fn rejects_bare_package_config_imports() {
        let manifest = deserialize_from_json_str::<BiomeManifest>(
            r#"{ "version": 1, "configs": ["@org/shared-configs"] }"#,
            JsonParserOptions::default(),
            "",
        );

        assert!(manifest.has_errors());
    }

    #[test]
    fn rejects_presets_without_rules() {
        assert_manifest_snapshot(
            "preset_without_rules",
            r#"{
                "version": 1,
                "plugins": {
                    "rules": [{ "useMyLogger": "./useMyLogger.grit" }],
                    "presets": { "recommended": [] }
                }
            }"#,
        );
    }

    #[test]
    fn rejects_empty_presets() {
        assert_manifest_snapshot(
            "empty_presets",
            r#"{
                "version": 1,
                "plugins": {
                    "rules": [{ "useMyLogger": "./useMyLogger.grit" }],
                    "presets": {}
                }
            }"#,
        );
    }

    #[test]
    fn accepts_missing_presets() {
        let manifest = deserialize_from_json_str::<BiomeManifest>(
            r#"{
                "version": 1,
                "plugins": {
                    "rules": [{ "useMyLogger": "./useMyLogger.grit" }]
                }
            }"#,
            JsonParserOptions::default(),
            "",
        );

        assert!(!manifest.has_errors());
    }

    #[test]
    fn rejects_duplicate_rule_keys() {
        assert_manifest_snapshot(
            "duplicate_rule_keys",
            r#"{
                "version": 1,
                "plugins": {
                    "rules": [{
                        "useMyLogger": "./first.grit",
                        "useMyLogger": "./second.grit"
                    }],
                    "presets": { "recommended": ["useMyLogger"] }
                }
            }"#,
        );
    }

    #[test]
    fn rejects_duplicate_config_keys() {
        assert_manifest_snapshot(
            "duplicate_config_keys",
            r#"{
                "version": 1,
                "configs": [{
                    "recommended": "./first.json",
                    "recommended": "./second.json"
                }]
            }"#,
        );
    }

    #[test]
    fn rejects_duplicate_preset_keys() {
        assert_manifest_snapshot(
            "duplicate_preset_keys",
            r#"{
                "version": 1,
                "plugins": {
                    "rules": [{ "useMyLogger": "./useMyLogger.grit" }],
                    "presets": {
                        "recommended": ["useMyLogger"],
                        "recommended": ["useMyLogger"]
                    }
                }
            }"#,
        );
    }

    #[test]
    fn rejects_invalid_preset_names() {
        assert_manifest_snapshot(
            "invalid_preset_name",
            r#"{
                "version": 1,
                "plugins": {
                    "rules": [{ "useMyLogger": "./useMyLogger.grit" }],
                    "presets": { "company/recommended": ["useMyLogger"] }
                }
            }"#,
        );
    }

    #[test]
    fn rejects_empty_manifests() {
        let manifest = deserialize_from_json_str::<BiomeManifest>(
            r#"{ "version": 1 }"#,
            JsonParserOptions::default(),
            "",
        );

        assert!(manifest.has_errors());
    }
}
