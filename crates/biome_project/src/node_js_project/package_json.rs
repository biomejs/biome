use crate::{LanguageRoot, Manifest};
use biome_deserialize::json::deserialize_from_json_ast;
use biome_deserialize::{
    Deserializable, DeserializableContext, DeserializableTypes, DeserializableValue,
    DeserializationVisitor, Deserialized, Text,
};
use biome_json_syntax::JsonLanguage;
use biome_text_size::TextRange;
use rustc_hash::FxHashMap;

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

impl Manifest for PackageJson {
    type Language = JsonLanguage;

    fn deserialize_manifest(root: &LanguageRoot<Self::Language>) -> Deserialized<Self> {
        deserialize_from_json_ast::<PackageJson>(root, "")
    }
}

#[derive(Debug, Default, Clone, biome_deserialize_macros::Deserializable)]
pub struct Dependencies(FxHashMap<String, Version>);

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
    SemVer(node_semver::Version),
    Literal(String),
}

impl From<&str> for Version {
    fn from(value: &str) -> Self {
        Self::Literal(value.to_string())
    }
}

impl From<String> for Version {
    fn from(value: String) -> Self {
        Self::Literal(value)
    }
}

impl Deserializable for PackageJson {
    fn deserialize(
        ctx: &mut impl DeserializableContext,
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
        ctx: &mut impl DeserializableContext,
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
        ctx: &mut impl DeserializableContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        let value = Text::deserialize(ctx, value, name)?;
        match value.text().parse() {
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
