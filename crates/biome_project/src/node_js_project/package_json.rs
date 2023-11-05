use crate::{LanguageRoot, Manifest};
use biome_deserialize::json::deserialize_from_json_ast;
use biome_deserialize::{
    Deserializable, DeserializableValue, DeserializationDiagnostic, DeserializationVisitor,
    Deserialized, ExpectedType,
};
use biome_json_syntax::JsonLanguage;
use biome_rowan::TokenText;
use biome_text_size::{TextRange, TextSize};
use rustc_hash::FxHashMap;
use std::ops::Add;

#[derive(Debug, Default)]
pub struct PackageJson {
    pub version: Option<Version>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub dependencies: Dependencies,
    pub dev_dependencies: Dependencies,
    pub optional_dependencies: Dependencies,
    pub license: Option<(String, TextRange)>,
}

impl Manifest for PackageJson {
    type Language = JsonLanguage;

    fn deserialize_manifest(root: &LanguageRoot<Self::Language>) -> Deserialized<Self> {
        deserialize_from_json_ast::<PackageJson>(root)
    }
}

#[derive(Debug, Default)]
pub struct Dependencies(FxHashMap<String, Version>);

#[derive(Debug)]
pub struct Version(node_semver::Version);

impl Deserializable for PackageJson {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        value.deserialize(PackageJsonVisitor, diagnostics)
    }
}

struct PackageJsonVisitor;
impl DeserializationVisitor for PackageJsonVisitor {
    type Output = PackageJson;

    const EXPECTED_TYPE: ExpectedType = ExpectedType::MAP;

    fn visit_map(
        self,
        members: impl Iterator<Item = (impl DeserializableValue, impl DeserializableValue)>,
        _range: TextRange,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        let mut result = Self::Output::default();
        for (key, value) in members {
            let Some(key) = TokenText::deserialize(key, diagnostics) else {
                continue;
            };
            match key.text() {
                "version" => {
                    result.version = Deserializable::deserialize(value, diagnostics);
                }
                "name" => {
                    result.name = Deserializable::deserialize(value, diagnostics);
                }
                "license" => {
                    let license_range = value.range();
                    // TODO: add proper parsing of license, e.g. support for AND keywords
                    result.license = Deserializable::deserialize(value, diagnostics)
                        .map(|license| (license, license_range));
                }
                "description" => {
                    result.description = Deserializable::deserialize(value, diagnostics);
                }
                "dependencies" => {
                    if let Some(deps) = Deserializable::deserialize(value, diagnostics) {
                        result.dependencies = deps;
                    }
                }
                "devDependencies" => {
                    if let Some(deps) = Deserializable::deserialize(value, diagnostics) {
                        result.dev_dependencies = deps;
                    }
                }
                "optionalDependencies" => {
                    if let Some(deps) = Deserializable::deserialize(value, diagnostics) {
                        result.optional_dependencies = deps;
                    }
                }
                _ => {
                    // each package can add their own field, so we should ignore any extraneous key
                    // and only deserialize the ones that Rome deems important
                }
            }
        }
        Some(result)
    }
}

impl Deserializable for Dependencies {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        Some(Dependencies(Deserializable::deserialize(
            value,
            diagnostics,
        )?))
    }
}

impl Deserializable for Version {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        let range = value.range();
        let value = TokenText::deserialize(value, diagnostics)?;
        match value.parse() {
            Ok(version) => Some(Version(version)),
            Err(err) => {
                let (start, end) = err.location();
                let start_range = range.start();
                let end_range = range.end();
                let range = TextRange::new(
                    start_range.add(TextSize::from(start as u32)),
                    end_range.add(TextSize::from(end as u32)),
                );
                diagnostics
                    .push(DeserializationDiagnostic::new(err.kind().to_string()).with_range(range));
                None
            }
        }
    }
}
