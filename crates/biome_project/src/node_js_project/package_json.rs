use crate::{LanguageRoot, Manifest};
use biome_deserialize::json::deserialize_from_json_ast;
use biome_deserialize::{
    Deserializable, DeserializableTypes, DeserializableValue, DeserializationContext,
    DeserializationVisitor, Deserialized, Text,
};
use biome_json_syntax::JsonLanguage;
use biome_text_size::TextRange;
use node_semver::Range;
use rustc_hash::FxHashMap;
use std::ops::Deref;
use std::str::FromStr;

#[derive(Debug, Default, Clone)]
pub struct PackageJson {
    pub version: Option<Version>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub dependencies: Dependencies,
    pub dev_dependencies: Dependencies,
    pub peer_dependencies: Dependencies,
    pub optional_dependencies: Dependencies,
    pub license: Option<(String, TextRange)>,
    pub r#type: Option<PackageType>,
}

impl PackageJson {
    /// Checks whether the `specifier` is defined in `dependencies`, `dev_dependencies` or `peer_dependencies`
    pub fn contains_dependency(&self, specifier: &str) -> bool {
        self.dependencies.contains(specifier)
            || self.dev_dependencies.contains(specifier)
            || self.peer_dependencies.contains(specifier)
    }

    /// Checks whether the `specifier` is defined in `dependencies`, `dev_dependencies` or `peer_dependencies`, and the `range`
    /// of matches the one of the manifest
    pub fn matches_dependency(&self, specifier: &str, range: &str) -> bool {
        let iter = self
            .dependencies
            .iter()
            .chain(self.dev_dependencies.iter())
            .chain(self.peer_dependencies.iter());
        let Ok(range) = Range::from_str(range) else {
            return false;
        };
        for (dependency_name, dependency_version) in iter {
            if dependency_name == specifier && dependency_version.satisfies(&range) {
                return true;
            }
        }

        false
    }
}

impl Manifest for PackageJson {
    type Language = JsonLanguage;

    fn deserialize_manifest(root: &LanguageRoot<Self::Language>) -> Deserialized<Self> {
        deserialize_from_json_ast::<PackageJson>(root, "")
    }
}

#[derive(Debug, Default, Clone, biome_deserialize_macros::Deserializable)]
pub struct Dependencies(FxHashMap<String, Version>);

impl Deref for Dependencies {
    type Target = FxHashMap<String, Version>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Dependencies {
    pub fn to_keys(&self) -> Vec<String> {
        self.0.keys().cloned().collect()
    }

    pub fn contains(&self, specifier: &str) -> bool {
        self.0.contains_key(specifier)
    }

    pub fn add(&mut self, dependency: impl Into<String>, version: impl Into<Version>) {
        self.0.insert(dependency.into(), version.into());
    }
}

#[derive(Debug, Clone)]
pub enum Version {
    SemVer(node_semver::Range),
    Literal(String),
}

impl Version {
    pub fn satisfies(&self, other_range: &Range) -> bool {
        match self {
            Version::SemVer(range) => range.allows_any(other_range),
            Version::Literal(_) => false,
        }
    }
}

impl From<&str> for Version {
    fn from(value: &str) -> Self {
        node_semver::Range::parse(value)
            .ok()
            .map_or_else(|| Self::Literal(value.into()), Self::SemVer)
    }
}

impl From<String> for Version {
    fn from(value: String) -> Self {
        node_semver::Range::parse(value.as_str())
            .ok()
            .map_or_else(|| Self::Literal(value), Self::SemVer)
    }
}

impl Deserializable for PackageJson {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        value.deserialize(ctx, PackageJsonVisitor, name)
    }
}

struct PackageJsonVisitor;
impl DeserializationVisitor for PackageJsonVisitor {
    type Output = PackageJson;

    const EXPECTED_TYPE: DeserializableTypes = DeserializableTypes::MAP;

    fn visit_map(
        self,
        ctx: &mut impl DeserializationContext,
        members: impl Iterator<Item = Option<(impl DeserializableValue, impl DeserializableValue)>>,
        _range: TextRange,
        _name: &str,
    ) -> Option<Self::Output> {
        let mut result = Self::Output::default();
        for (key, value) in members.flatten() {
            let Some(key_text) = Text::deserialize(ctx, &key, "") else {
                continue;
            };
            match key_text.text() {
                "version" => {
                    result.version = Deserializable::deserialize(ctx, &value, &key_text);
                }
                "name" => {
                    result.name = Deserializable::deserialize(ctx, &value, &key_text);
                }
                "license" => {
                    let license_range = value.range();
                    // TODO: add proper parsing of license, e.g. support for AND keywords
                    result.license = Deserializable::deserialize(ctx, &value, &key_text)
                        .map(|license| (license, license_range));
                }
                "description" => {
                    result.description = Deserializable::deserialize(ctx, &value, &key_text);
                }
                "dependencies" => {
                    if let Some(deps) = Deserializable::deserialize(ctx, &value, &key_text) {
                        result.dependencies = deps;
                    }
                }
                "devDependencies" => {
                    if let Some(deps) = Deserializable::deserialize(ctx, &value, &key_text) {
                        result.dev_dependencies = deps;
                    }
                }
                "peerDependencies" => {
                    if let Some(deps) = Deserializable::deserialize(ctx, &value, &key_text) {
                        result.peer_dependencies = deps;
                    }
                }
                "optionalDependencies" => {
                    if let Some(deps) = Deserializable::deserialize(ctx, &value, &key_text) {
                        result.optional_dependencies = deps;
                    }
                }
                "type" => {
                    result.r#type = Deserializable::deserialize(ctx, &value, &key_text);
                }
                _ => {
                    // each package can add their own field, so we should ignore any extraneous key
                    // and only deserialize the ones that Biome deems important
                }
            }
        }
        Some(result)
    }
}

impl Deserializable for Version {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        let value = Text::deserialize(ctx, value, name)?;
        match node_semver::Range::parse(value.text()) {
            Ok(version) => Some(Version::SemVer(version)),
            Err(_) => Some(Version::Literal(value.text().to_string())),
        }
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq, biome_deserialize_macros::Deserializable)]
pub enum PackageType {
    #[default]
    Module,
    Commonjs,
}

impl PackageType {
    pub const fn is_commonjs(&self) -> bool {
        matches!(self, Self::Commonjs)
    }

    pub const fn is_module(&self) -> bool {
        matches!(self, Self::Module)
    }
}
