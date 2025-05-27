use crate::{LanguageRoot, Manifest};
use biome_deserialize::json::deserialize_from_json_str;
use biome_deserialize::{
    Deserializable, DeserializableTypes, DeserializableValue, DeserializationContext,
    DeserializationVisitor, Deserialized, Text, json::deserialize_from_json_ast,
};
use biome_diagnostics::Error;
use biome_json_parser::JsonParserOptions;
use biome_json_syntax::JsonLanguage;
use biome_json_value::{JsonObject, JsonValue};
use biome_text_size::TextRange;
use camino::Utf8Path;
use node_semver::{Range, SemverError};
use rustc_hash::{FxBuildHasher, FxHashMap};
use std::panic::catch_unwind;
use std::{ops::Deref, str::FromStr};

/// Deserialized `package.json`.
#[derive(Debug, Default, Clone)]
pub struct PackageJson {
    /// The "name" field defines your package's name.
    /// The "name" field can be used in addition to the "exports" field to self-reference a package using its name.
    ///
    /// <https://nodejs.org/api/packages.html#name>
    pub name: Option<String>,

    /// The "type" field.
    ///
    /// <https://nodejs.org/api/packages.html#type>
    pub r#type: Option<PackageType>,

    pub version: Option<String>,
    pub description: Option<String>,
    pub dependencies: Dependencies,
    pub dev_dependencies: Dependencies,
    pub peer_dependencies: Dependencies,
    pub optional_dependencies: Dependencies,
    pub license: Option<(String, TextRange)>,

    pub(crate) raw_json: JsonObject,
}

static_assertions::assert_impl_all!(PackageJson: Send, Sync);

impl PackageJson {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: Some(name.into()),
            r#type: Some(PackageType::Module),
            ..Default::default()
        }
    }

    pub fn with_version(self, version: String) -> Self {
        Self {
            version: Some(version),
            ..self
        }
    }

    pub fn with_exports(self, exports: impl Into<JsonValue>) -> Self {
        let mut raw_json = self.raw_json;
        raw_json.insert("exports".into(), exports.into());
        Self { raw_json, ..self }
    }

    pub fn with_dependencies(self, dependencies: Dependencies) -> Self {
        Self {
            dependencies,
            ..self
        }
    }

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
        for (dependency_name, dependency_version) in iter {
            if dependency_name == specifier
                && Version::from(dependency_version.as_str()).satisfies(range)
            {
                return true;
            }
        }

        false
    }

    pub fn get_value_by_path(&self, path: &[&str]) -> Option<&JsonValue> {
        if path.is_empty() {
            return None;
        }

        let mut value = self.raw_json.get(path[0])?;
        for key in path.iter().skip(1) {
            if let Some(inner_value) = value.as_object().and_then(|object| object.get(*key)) {
                value = inner_value;
            } else {
                return None;
            }
        }
        Some(value)
    }
}

impl Manifest for PackageJson {
    type Language = JsonLanguage;

    fn deserialize_manifest(root: &LanguageRoot<Self::Language>) -> Deserialized<Self> {
        deserialize_from_json_ast::<Self>(root, "")
    }

    fn read_manifest(fs: &dyn biome_fs::FileSystem, path: &Utf8Path) -> Deserialized<Self> {
        match fs.read_file_from_path(path) {
            Ok(content) => deserialize_from_json_str(&content, JsonParserOptions::default(), ""),
            Err(error) => Deserialized::new(None, vec![Error::from(error)]),
        }
    }
}

#[derive(Debug, Default, Clone, biome_deserialize_macros::Deserializable)]
pub struct Dependencies(FxHashMap<String, String>);

impl<const N: usize> From<[(String, String); N]> for Dependencies {
    fn from(dependencies: [(String, String); N]) -> Self {
        let mut map = FxHashMap::with_capacity_and_hasher(N, FxBuildHasher);
        for (dependency, version) in dependencies {
            map.insert(dependency, version);
        }
        Self(map)
    }
}

impl Deref for Dependencies {
    type Target = FxHashMap<String, String>;

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

    pub fn add(&mut self, dependency: impl Into<String>, version: impl Into<String>) {
        self.0.insert(dependency.into(), version.into());
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Version {
    SemVer(node_semver::Range),
    Literal(String),
}

impl Version {
    pub fn satisfies(&self, other_range: &str) -> bool {
        let range = Range::from_str(other_range);
        if let Ok(other_range) = range {
            match self {
                Self::SemVer(range) => range.allows_any(&other_range),
                Self::Literal(_) => false,
            }
        } else {
            false
        }
    }
}

impl From<&str> for Version {
    fn from(value: &str) -> Self {
        parse_range(value)
            .ok()
            .unwrap_or(Self::Literal(value.to_string()))
    }
}

impl From<String> for Version {
    fn from(value: String) -> Self {
        parse_range(&value).ok().unwrap_or(Self::Literal(value))
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
                key => {
                    if let Some(value) = JsonValue::deserialize(ctx, &value, &key_text) {
                        result.raw_json.insert(key.into(), value);
                    }
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
        let version = Text::deserialize(ctx, value, name)?;
        match parse_range(version.text()) {
            Ok(result) => Some(result),
            Err(_) => Some(Self::Literal(version.text().to_string())),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, biome_deserialize_macros::Deserializable)]
pub enum PackageType {
    #[default]
    #[deserializable(rename = "module")]
    Module,
    #[deserializable(rename = "commonjs")]
    CommonJs,
}

impl PackageType {
    pub const fn is_commonjs(&self) -> bool {
        matches!(self, Self::CommonJs)
    }

    pub const fn is_module(&self) -> bool {
        matches!(self, Self::Module)
    }
}

fn parse_range(range: &str) -> Result<Version, SemverError> {
    match catch_unwind(|| Range::parse(range).map(Version::SemVer)) {
        Ok(result) => result,
        Err(_) => Ok(Version::Literal(range.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_not_panic_on_invalid_semver_range() {
        let result = parse_range("~0.x.0");

        assert_eq!(result, Ok(Version::Literal("~0.x.0".to_string())));
    }
}
