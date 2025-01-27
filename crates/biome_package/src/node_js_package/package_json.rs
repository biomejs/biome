use std::{ops::Deref, path::Path, str::FromStr};

use biome_deserialize::{
    json::deserialize_from_json_ast, Deserializable, DeserializableTypes, DeserializableValue,
    DeserializationContext, DeserializationVisitor, Deserialized, Text,
};
use biome_json_syntax::JsonLanguage;
use biome_json_value::{JsonObject, JsonString, JsonValue};
use biome_text_size::TextRange;
use camino::{Utf8Path, Utf8PathBuf};
use node_semver::Range;
use oxc_resolver::{ImportsExportsEntry, ImportsExportsMap, PathUtil, ResolveError};
use rustc_hash::{FxBuildHasher, FxHashMap};

use crate::{LanguageRoot, Manifest};

/// Deserialized `package.json`.
#[derive(Debug, Default, Clone)]
pub struct PackageJson {
    /// Path to `package.json`. Contains the `package.json` filename.
    pub path: Utf8PathBuf,

    /// Canonicalized version of [Self::path], where all symbolic links are
    /// resolved.
    pub canonicalized_path: Utf8PathBuf,

    /// The "name" field defines your package's name.
    /// The "name" field can be used in addition to the "exports" field to self-reference a package using its name.
    ///
    /// <https://nodejs.org/api/packages.html#name>
    pub name: Option<String>,

    /// The "type" field.
    ///
    /// <https://nodejs.org/api/packages.html#type>
    pub r#type: Option<PackageType>,

    pub version: Option<Version>,
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

    /// Returns [self] with both `path` and `canonicalized_path` set to the
    /// given `path`.
    ///
    /// Use [Self::with_path_and_canonicalized_path()] if you want to set a
    /// different canonicalized path.
    pub fn with_path(self, path: impl Into<Utf8PathBuf>) -> Self {
        let path: Utf8PathBuf = path.into();
        Self {
            path: path.clone(),
            canonicalized_path: path,
            ..self
        }
    }

    /// Returns [self] with updated `path` and `canonicalized_path`.
    pub fn with_path_and_canonicalized_path(
        self,
        path: impl Into<Utf8PathBuf>,
        canonicalized_path: impl Into<Utf8PathBuf>,
    ) -> Self {
        Self {
            path: path.into(),
            canonicalized_path: canonicalized_path.into(),
            ..self
        }
    }

    pub fn with_version(self, version: Version) -> Self {
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
            if dependency_name == specifier && dependency_version.satisfies(range) {
                return true;
            }
        }

        false
    }

    pub(crate) fn alias_value<'a>(
        key: &Path,
        value: &'a JsonValue,
    ) -> Result<Option<&'a str>, ResolveError> {
        match value {
            JsonValue::String(value) => Ok(Some(value.as_ref())),
            JsonValue::Bool(b) if !b => Err(ResolveError::Ignored(key.to_path_buf())),
            _ => Ok(None),
        }
    }

    /// The "browser" field is provided by a module author as a hint to javascript bundlers or component tools when packaging modules for client side use.
    /// Multiple values are configured by [ResolveOptions::alias_fields].
    ///
    /// <https://github.com/defunctzombie/package-browser-field-spec>
    pub(crate) fn browser_fields<'a>(
        &'a self,
        alias_fields: &'a [Vec<String>],
    ) -> impl Iterator<Item = &'a JsonObject> + 'a {
        alias_fields.iter().filter_map(|object_path| {
            Self::get_value_by_path(&self.raw_json, object_path)
                // Only object is valid, all other types are invalid
                // https://github.com/webpack/enhanced-resolve/blob/3a28f47788de794d9da4d1702a3a583d8422cd48/lib/AliasFieldPlugin.js#L44-L52
                .and_then(|value| value.as_map())
        })
    }

    pub(crate) fn get_value_by_path<'a>(
        fields: &'a JsonObject,
        path: &[String],
    ) -> Option<&'a JsonValue> {
        if path.is_empty() {
            return None;
        }

        let mut value = fields.get(path[0].as_str())?;
        for key in path.iter().skip(1) {
            if let Some(inner_value) = value.as_map().and_then(|o| o.get(key.as_str())) {
                value = inner_value;
            } else {
                return None;
            }
        }
        Some(value)
    }
}

impl oxc_resolver::PackageJson for PackageJson {
    fn path(&self) -> &Path {
        self.path.as_std_path()
    }

    fn realpath(&self) -> &Path {
        self.canonicalized_path.as_std_path()
    }

    fn directory(&self) -> &Path {
        debug_assert!(self
            .canonicalized_path
            .file_name()
            .is_some_and(|x| x == "package.json"));
        self.canonicalized_path
            .parent()
            .map(Utf8Path::as_std_path)
            .unwrap()
    }

    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    fn r#type(&self) -> Option<oxc_resolver::PackageType> {
        self.r#type.map(Into::into)
    }

    fn main_fields<'a>(&'a self, main_fields: &'a [String]) -> impl Iterator<Item = &'a str> + 'a {
        main_fields
            .iter()
            .filter_map(|main_field| self.raw_json.get(main_field.as_str()))
            .filter_map(|value| value.as_string())
            .map(JsonString::as_str)
    }

    fn exports_fields<'a>(
        &'a self,
        exports_fields: &'a [Vec<String>],
    ) -> impl Iterator<Item = impl ImportsExportsEntry<'a>> + 'a {
        exports_fields
            .iter()
            .filter_map(|object_path| Self::get_value_by_path(&self.raw_json, object_path))
    }

    fn imports_fields<'a>(
        &'a self,
        imports_fields: &'a [Vec<String>],
    ) -> impl Iterator<Item = impl ImportsExportsMap<'a>> + 'a {
        imports_fields
            .iter()
            .filter_map(|object_path| Self::get_value_by_path(&self.raw_json, object_path))
            .filter_map(|value| value.as_map())
    }

    fn resolve_browser_field<'a>(
        &'a self,
        path: &Path,
        request: Option<&str>,
        alias_fields: &'a [Vec<String>],
    ) -> Result<Option<&'a str>, ResolveError> {
        for object in self.browser_fields(alias_fields) {
            if let Some(request) = request {
                if let Some(value) = object.get(request) {
                    return Self::alias_value(path, value);
                }
            } else {
                let dir = self.path.parent().unwrap();
                for (key, value) in object.iter() {
                    let joined = dir.as_std_path().normalize_with(Utf8Path::new(key));
                    if joined == path {
                        return Self::alias_value(path, value);
                    }
                }
            }
        }
        Ok(None)
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

impl<const N: usize> From<[(String, Version); N]> for Dependencies {
    fn from(dependencies: [(String, Version); N]) -> Self {
        let mut map = FxHashMap::with_capacity_and_hasher(N, FxBuildHasher);
        for (dependency, version) in dependencies {
            map.insert(dependency, version);
        }
        Self(map)
    }
}

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
    pub fn satisfies(&self, other_range: &str) -> bool {
        let range = Range::from_str(other_range);
        if let Ok(other_range) = range {
            match self {
                Version::SemVer(range) => range.allows_any(&other_range),
                Version::Literal(_) => false,
            }
        } else {
            false
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
        let value = Text::deserialize(ctx, value, name)?;
        match node_semver::Range::parse(value.text()) {
            Ok(version) => Some(Version::SemVer(version)),
            Err(_) => Some(Version::Literal(value.text().to_string())),
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

impl From<PackageType> for oxc_resolver::PackageType {
    fn from(value: PackageType) -> Self {
        match value {
            PackageType::Module => Self::Module,
            PackageType::CommonJs => Self::CommonJs,
        }
    }
}
