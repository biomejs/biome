use crate::node_semver::{Range, VersionError};
use crate::{LanguageRoot, Manifest};
use biome_deserialize::json::deserialize_from_json_str;
use biome_deserialize::{
    Deserializable, DeserializableTypes, DeserializableValue, DeserializationContext,
    DeserializationVisitor, Deserialized, Text, json::deserialize_from_json_ast,
};
use biome_diagnostics::Error;
use biome_json_parser::JsonParserOptions;
use biome_json_syntax::JsonLanguage;
use biome_json_value::JsonValue;
use biome_text_size::TextRange;
use camino::Utf8Path;
use rustc_hash::FxHashMap;
use std::{ops::Deref, str::FromStr};

/// Deserialized `package.json`.
#[derive(Debug, Default, Clone)]
pub struct PackageJson {
    /// The "name" field defines your package's name. The "name" field can be
    /// used in addition to the "exports" field to self-reference a package
    /// using its name.
    ///
    /// <https://nodejs.org/api/packages.html#name>
    pub name: Option<Box<str>>,

    /// The "type" field.
    ///
    /// <https://nodejs.org/api/packages.html#type>
    pub r#type: Option<PackageType>,

    pub version: Option<Box<str>>,
    pub dependencies: Dependencies,
    pub dev_dependencies: Dependencies,
    pub peer_dependencies: Dependencies,
    pub optional_dependencies: Dependencies,
    /// Optional pnpm workspace catalogs (`catalog:` and `catalogs:`) resolved from
    /// a `pnpm-workspace.yaml`. When present, dependency versions declared as
    /// `catalog:` or `catalog:<name>` are looked up via `Catalogs`; when `None`,
    /// no catalog resolution is applied and literal versions are used. This field
    /// is typically populated by parsing the workspace file rather than
    /// directly from `package.json`.
    pub catalog: Option<Catalogs>,
    pub license: Option<(Box<str>, TextRange)>,

    pub author: Option<Box<str>>,
    pub exports: Option<JsonValue>,
    pub imports: Option<JsonValue>,
    pub main: Option<Box<str>>,
    pub types: Option<Box<str>>,
}

static_assertions::assert_impl_all!(PackageJson: Send, Sync);

impl PackageJson {
    pub fn new(name: impl Into<Box<str>>) -> Self {
        Self {
            name: Some(name.into()),
            r#type: Some(PackageType::Module),
            ..Default::default()
        }
    }

    pub fn with_version(self, version: impl Into<Box<str>>) -> Self {
        Self {
            version: Some(version.into()),
            ..self
        }
    }

    pub fn with_exports(self, exports: impl Into<JsonValue>) -> Self {
        Self {
            exports: Some(exports.into()),
            ..self
        }
    }

    pub fn with_dependencies(self, dependencies: Dependencies) -> Self {
        Self {
            dependencies,
            ..self
        }
    }

    /// Checks whether the `specifier` is defined in `dependencies`,
    /// `dev_dependencies` or `peer_dependencies`
    pub fn contains_dependency(&self, specifier: &str) -> bool {
        self.dependencies.contains(specifier)
            || self.dev_dependencies.contains(specifier)
            || self.peer_dependencies.contains(specifier)
    }

    /// Checks whether the `specifier` is defined in `dependencies`,
    /// `dev_dependencies` or `peer_dependencies`, and the `range` of matches
    /// the one of the manifest
    pub fn matches_dependency(&self, specifier: &str, range: &str) -> bool {
        let iter = self
            .dependencies
            .iter()
            .chain(self.dev_dependencies.iter())
            .chain(self.peer_dependencies.iter());
        for (dependency_name, dependency_version) in iter {
            if dependency_name.as_ref() == specifier
                && dependency_satisfies(
                    specifier,
                    dependency_version.as_ref(),
                    self.catalog.as_ref(),
                    range,
                )
            {
                return true;
            }
        }

        false
    }

    /// Extract catalog entries from a pnpm workspace file, supporting both the
    /// default `catalog:` and named catalogs under `catalogs:`.
    pub fn parse_pnpm_workspace_catalog(source: &str) -> Option<Catalogs> {
        let mut catalogs = Catalogs::default();

        let mut in_default = false;
        let mut default_indent = 0usize;
        let mut default_entries: Vec<(Box<str>, Box<str>)> = Vec::new();

        let mut catalogs_indent = None;
        let mut current_catalog: Option<(Box<str>, Vec<(Box<str>, Box<str>)>)> = None;
        let mut current_catalog_indent = 0usize;

        for line in source.lines() {
            let trimmed_start = line.trim_start();

            if trimmed_start.is_empty() || trimmed_start.starts_with('#') {
                continue;
            }

            let indent = line.len() - trimmed_start.len();

            if in_default {
                if indent <= default_indent {
                    in_default = false;
                } else if let Some(entry) = parse_pnpm_catalog_entry(trimmed_start) {
                    default_entries.push(entry);
                }

                if in_default {
                    continue;
                }
            }

            if !in_default && catalogs_indent.is_none() && trimmed_start.starts_with("catalog:") {
                in_default = true;
                default_indent = indent;
                continue;
            }

            if catalogs_indent.is_none() && trimmed_start.starts_with("catalogs:") {
                catalogs_indent = Some(indent);
                continue;
            }

            if let Some(cat_indent) = catalogs_indent {
                if indent <= cat_indent {
                    if let Some((name, entries)) = current_catalog.take() {
                        catalogs
                            .named
                            .insert(name, Dependencies(entries.into_boxed_slice()));
                    }
                    continue;
                }

                if let Some((name, entries)) = &mut current_catalog {
                    if indent <= current_catalog_indent {
                        let entries = std::mem::take(entries);
                        catalogs
                            .named
                            .insert(name.clone(), Dependencies(entries.into_boxed_slice()));
                        current_catalog = None;
                    }
                }

                if current_catalog.is_none() {
                    if let Some((raw_name, _)) = trimmed_start.split_once(':') {
                        let name = raw_name.trim().trim_matches(['\'', '"']);
                        if !name.is_empty() {
                            current_catalog = Some((name.into(), Vec::new()));
                            current_catalog_indent = indent;
                        }
                    }
                    continue;
                }

                if let Some((_, entries)) = &mut current_catalog {
                    if indent > current_catalog_indent {
                        if let Some(entry) = parse_pnpm_catalog_entry(trimmed_start) {
                            entries.push(entry);
                        }
                    }
                }
            }
        }

        if let Some((name, entries)) = current_catalog.take() {
            catalogs
                .named
                .insert(name, Dependencies(entries.into_boxed_slice()));
        }

        if !default_entries.is_empty() {
            catalogs.default = Some(Dependencies(default_entries.into_boxed_slice()));
        }

        if catalogs.is_empty() {
            None
        } else {
            Some(catalogs)
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Catalogs {
    /// Dependencies listed under the top-level `catalog:` key in pnpm-workspace.yaml.
    pub default: Option<Dependencies>,
    /// Named catalogs listed under `catalogs:` in pnpm-workspace.yaml, keyed by catalog name.
    pub named: FxHashMap<Box<str>, Dependencies>,
}

impl Catalogs {
    fn is_empty(&self) -> bool {
        self.default.is_none() && self.named.is_empty()
    }

    fn lookup<'a>(&'a self, specifier: &str, catalog_name: Option<&str>) -> Option<&'a str> {
        if let Some(name) = catalog_name {
            return self.named.get(name).and_then(|deps| deps.get(specifier));
        }

        self.default.as_ref().and_then(|deps| deps.get(specifier))
    }
}

/// Parses a single `key: value` line from a pnpm catalog section, stripping
/// surrounding quotes and whitespace. Returns `None` for malformed pairs.
fn parse_pnpm_catalog_entry(value: &str) -> Option<(Box<str>, Box<str>)> {
    let (raw_key, raw_value) = value.split_once(':')?;
    let key = raw_key.trim().trim_matches(['\'', '"']);
    let mut val = raw_value.trim();

    if val.starts_with('"') && val.ends_with('"') && val.len() >= 2 {
        val = &val[1..val.len() - 1];
    } else if val.starts_with('\'') && val.ends_with('\'') && val.len() >= 2 {
        val = &val[1..val.len() - 1];
    }

    if key.is_empty() || val.is_empty() {
        return None;
    }

    Some((key.into(), val.into()))
}

/// Checks if a manifest dependency satisfies a semver range, resolving pnpm
/// `catalog:` specifiers (default or named) through the provided `Catalogs`.
fn dependency_satisfies(
    specifier: &str,
    version: &str,
    catalog: Option<&Catalogs>,
    range: &str,
) -> bool {
    let resolved_version = resolve_dependency_version(specifier, version, catalog);

    Version::from(resolved_version).satisfies(range)
}

/// Resolves a dependency version, expanding pnpm `catalog:` references (default
/// or named) using the provided `Catalogs`. Falls back to the literal version
/// string if no catalog match is found.
fn resolve_dependency_version<'a>(
    specifier: &str,
    version: &'a str,
    catalog: Option<&'a Catalogs>,
) -> &'a str {
    if let (Some(catalogs), Some(rest)) = (catalog, version.strip_prefix("catalog:")) {
        let (catalog_name, package_name) = if rest.is_empty() {
            (None, specifier)
        } else {
            (Some(rest), specifier)
        };

        if let Some(mapped_version) = catalogs.lookup(package_name, catalog_name) {
            return mapped_version;
        }
    }

    version
}

impl Manifest for PackageJson {
    type Language = JsonLanguage;

    fn deserialize_manifest(
        root: &LanguageRoot<Self::Language>,
        _path: &Utf8Path,
    ) -> Deserialized<Self> {
        deserialize_from_json_ast::<Self>(root, "")
    }

    fn read_manifest(fs: &dyn biome_fs::FileSystem, path: &Utf8Path) -> Deserialized<Self> {
        match fs.read_file_from_path(path) {
            Ok(content) => deserialize_from_json_str(&content, JsonParserOptions::default(), ""),
            Err(error) => Deserialized::new(None, vec![Error::from(error)]),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Dependencies(pub Box<[(Box<str>, Box<str>)]>);

impl Deserializable for Dependencies {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        struct Visitor;
        impl DeserializationVisitor for Visitor {
            type Output = Dependencies;
            const EXPECTED_TYPE: DeserializableTypes = DeserializableTypes::MAP;

            fn visit_map(
                self,
                ctx: &mut impl DeserializationContext,
                members: impl Iterator<
                    Item = Option<(impl DeserializableValue, impl DeserializableValue)>,
                >,
                _range: TextRange,
                name: &str,
            ) -> Option<Self::Output> {
                let result = members
                    .filter_map(|value| {
                        if let Some((key, value)) = value {
                            let key: Box<str> = Deserializable::deserialize(ctx, &key, name)?;
                            let value: Box<str> = Deserializable::deserialize(ctx, &value, name)?;
                            Some((key, value))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();

                Some(Dependencies(result.into_boxed_slice()))
            }
        }

        value.deserialize(ctx, Visitor, name)
        // let result = Vec::<(Box<str>, Box<str>)>::deserialize(ctx, value, name)
        //     .map(|v| v.into_boxed_slice())

        // .map(Self)
    }
}

// impl<const N: usize> From<[(Box<str>, Box<str>); N]> for Dependencies {
//     fn from(dependencies: [(Box<str>, Box<str>); N]) -> Self {
//         for (dependency, version) in dependencies {
//             map.insert(dependency.as_str().into(), version.as_str().into());
//         }
//         Self(dependencies)
//     }
// }

impl Deref for Dependencies {
    type Target = Box<[(Box<str>, Box<str>)]>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Dependencies {
    pub fn contains(&self, specifier: &str) -> bool {
        self.0.iter().any(|(k, _)| k.as_ref() == specifier)
    }

    /// Returns the version string for a dependency if present.
    pub fn get(&self, specifier: &str) -> Option<&str> {
        self.0
            .iter()
            .find(|(k, _)| k.as_ref() == specifier)
            .map(|(_, v)| v.as_ref())
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Version {
    SemVer(Range),
    Literal(String),
}

impl Version {
    pub fn satisfies(&self, other_range: &str) -> bool {
        let range = Range::from_str(other_range);
        if let Ok(other_range) = range {
            match self {
                Self::SemVer(range) => range.intersects(&other_range),
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
                "author" => {
                    if let Some(value) = Deserializable::deserialize(ctx, &value, &key_text) {
                        result.author = Some(value);
                    }
                }
                "exports" => {
                    if let Some(value) = JsonValue::deserialize(ctx, &value, &key_text) {
                        result.exports = Some(value);
                    }
                }

                "imports" => {
                    if let Some(value) = JsonValue::deserialize(ctx, &value, &key_text) {
                        result.imports = Some(value);
                    }
                }
                "types" => {
                    if let Some(value) = Deserializable::deserialize(ctx, &value, &key_text) {
                        result.types = Some(value);
                    }
                }
                "main" => {
                    if let Some(value) = Deserializable::deserialize(ctx, &value, &key_text) {
                        result.main = Some(value);
                    }
                }
                _ => {}
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

fn parse_range(range: &str) -> Result<Version, VersionError> {
    match Range::from_str(range).map(Version::SemVer) {
        Ok(result) => Ok(result),
        Err(_) => Ok(Version::Literal(range.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_package_json_author_field() {
        let deserialized = deserialize_from_json_str::<PackageJson>(
            r#"{
    "name": "@shared/format",
    "author": "Biome Team",
    "exports": {
        "./biome": "./biome.json"
    }
}"#,
            JsonParserOptions::default(),
            "",
        );
        let (package_json, errors) = deserialized.consume();
        assert!(errors.is_empty());

        let package_json = package_json.expect("parsing must have succeeded");
        assert_eq!(package_json.author, Some("Biome Team".into()));
    }

    #[test]
    fn should_not_panic_on_invalid_semver_range() {
        let result = parse_range("~0.x.0");

        assert_eq!(result, Ok(Version::Literal("~0.x.0".to_string())));
    }

    #[test]
    fn matches_dependency_with_pnpm_catalog() {
        let package_json = PackageJson {
            dependencies: Dependencies(Box::new([("react".into(), "catalog:".into())])),
            catalog: Some(Catalogs {
                default: Some(Dependencies(Box::new([("react".into(), "19.0.0".into())]))),
                named: FxHashMap::default(),
            }),
            ..Default::default()
        };

        assert!(package_json.matches_dependency("react", ">=19.0.0"));
    }

    #[test]
    fn matches_dependency_with_named_pnpm_catalog() {
        let package_json = PackageJson {
            dependencies: Dependencies(Box::new([("react".into(), "catalog:react19".into())])),
            catalog: Some(Catalogs {
                default: None,
                named: FxHashMap::from_iter([(
                    "react19".into(),
                    Dependencies(Box::new([("react".into(), "19.0.0".into())])),
                )]),
            }),
            ..Default::default()
        };

        assert!(package_json.matches_dependency("react", ">=19.0.0"));
    }

    #[test]
    fn parse_pnpm_workspace_catalog_minimal() {
        let yaml = r#"
packages:
  - "packages/*"
catalog:
  react: 19.0.0
  "react-dom": "^19.0.0"
"#;

        let catalog =
            PackageJson::parse_pnpm_workspace_catalog(yaml).expect("catalog should be parsed");

        let default = catalog.default.expect("default catalog");
        assert_eq!(default.get("react"), Some("19.0.0"));
        assert_eq!(default.get("react-dom"), Some("^19.0.0"));
        assert!(catalog.named.is_empty());
    }

    #[test]
    fn parse_pnpm_workspace_catalog_named() {
        let yaml = r#"
packages:
  - "packages/*"
catalogs:
  react19:
    react: 19.0.0
    "react-dom": "^19.0.0"
"#;

        let catalog =
            PackageJson::parse_pnpm_workspace_catalog(yaml).expect("catalog should be parsed");

        let named = catalog.named.get("react19").expect("react19 catalog");
        assert_eq!(named.get("react"), Some("19.0.0"));
        assert_eq!(named.get("react-dom"), Some("^19.0.0"));
    }

    #[test]
    fn parse_pnpm_workspace_catalog_default_and_named() {
        let yaml = r#"
packages:
  - "packages/*"
catalog:
  react: 19.0.0
catalogs:
  legacy:
    react: 18.3.1
"#;

        let catalog =
            PackageJson::parse_pnpm_workspace_catalog(yaml).expect("catalog should be parsed");

        let default = catalog.default.expect("default catalog");
        assert_eq!(default.get("react"), Some("19.0.0"));

        let legacy = catalog.named.get("legacy").expect("legacy catalog");
        assert_eq!(legacy.get("react"), Some("18.3.1"));
    }

    #[test]
    fn resolve_dependency_version_prefers_named_catalog() {
        let catalog = Catalogs {
            default: Some(Dependencies(Box::new([("react".into(), "19.0.0".into())]))),
            named: FxHashMap::from_iter([(
                "legacy".into(),
                Dependencies(Box::new([("react".into(), "18.3.1".into())])),
            )]),
        };

        let resolved_default =
            super::resolve_dependency_version("react", "catalog:", Some(&catalog));
        assert_eq!(resolved_default, "19.0.0");

        let resolved_named =
            super::resolve_dependency_version("react", "catalog:legacy", Some(&catalog));
        assert_eq!(resolved_named, "18.3.1");
    }

    #[test]
    fn parse_pnpm_catalog_entry_accepts_variants() {
        assert_eq!(
            super::parse_pnpm_catalog_entry("react: 19.0.0"),
            Some(("react".into(), "19.0.0".into()))
        );
        assert_eq!(
            super::parse_pnpm_catalog_entry("\"react-dom\": \"^19.0.0\""),
            Some(("react-dom".into(), "^19.0.0".into()))
        );
        assert_eq!(
            super::parse_pnpm_catalog_entry("react : '19.0.0'"),
            Some(("react".into(), "19.0.0".into()))
        );
    }

    #[test]
    fn parse_pnpm_catalog_entry_rejects_invalid() {
        assert!(super::parse_pnpm_catalog_entry("react").is_none());
        assert!(super::parse_pnpm_catalog_entry(":").is_none());
        assert!(super::parse_pnpm_catalog_entry("react: ").is_none());
        assert!(super::parse_pnpm_catalog_entry(": 19.0.0").is_none());
    }
}
