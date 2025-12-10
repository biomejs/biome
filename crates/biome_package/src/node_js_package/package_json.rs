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
use biome_rowan::AstNodeList;
use biome_text_size::TextRange;
use biome_yaml_parser::parse_yaml;
use biome_yaml_syntax::{
    AnyYamlBlockMapEntry, AnyYamlBlockNode, AnyYamlFlowNode, AnyYamlJsonContent,
    AnyYamlMappingImplicitKey, YamlBlockMapping,
};
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
        let parsed = parse_yaml(source);
        if parsed.has_errors() {
            return None;
        }

        let mut catalogs = Catalogs::default();
        let root = parsed.tree();
        let document = root
            .documents()
            .into_iter()
            .find_map(|doc| doc.as_yaml_document().cloned())?;
        let top_node = document.node()?;
        let mapping = as_catalog_block_mapping(&top_node)?;

        for entry in mapping.entries() {
            let Some((key, value_node)) = parse_catalog_mapping_entry(entry) else {
                continue;
            };

            match key.as_ref() {
                "catalog" => {
                    if let Some(deps_map) = as_catalog_block_mapping(&value_node) {
                        let deps = collect_catalog_dependencies(&deps_map);
                        if !deps.is_empty() {
                            catalogs.default = Some(deps);
                        }
                    }
                }
                "catalogs" => {
                    if let Some(named_map) = as_catalog_block_mapping(&value_node) {
                        for catalog_entry in named_map.entries() {
                            let Some((name, catalog_node)) =
                                parse_catalog_mapping_entry(catalog_entry)
                            else {
                                continue;
                            };

                            if let Some(deps_map) = as_catalog_block_mapping(&catalog_node) {
                                let deps = collect_catalog_dependencies(&deps_map);
                                if !deps.is_empty() {
                                    catalogs.named.insert(name, deps);
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
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

/// Parses a catalog mapping entry into a `(key, value node)` pair, keeping only
/// scalar keys and cloning the value node; returns `None` when the key or value
/// is missing or non-scalar.
fn parse_catalog_mapping_entry(
    entry: AnyYamlBlockMapEntry,
) -> Option<(Box<str>, AnyYamlBlockNode)> {
    if let Some(explicit) = entry.as_yaml_block_map_explicit_entry() {
        let key = explicit.key()?;
        let value = explicit.value()?.clone();
        return extract_catalog_scalar_from_block_node(&key).map(|key| (key, value));
    }

    if let Some(implicit) = entry.as_yaml_block_map_implicit_entry() {
        let key = extract_catalog_scalar_from_implicit_key(&implicit.key()?)?;
        let value = implicit.value()?.clone();
        return Some((key, value));
    }

    None
}

/// Narrows a block node to a `YamlBlockMapping` if it represents a mapping.
fn as_catalog_block_mapping(node: &AnyYamlBlockNode) -> Option<YamlBlockMapping> {
    node.as_any_yaml_block_in_block_node()?
        .as_yaml_block_mapping()
        .cloned()
}

/// Builds `Dependencies` from a YAML mapping, reading only scalar
/// `key: value` pairs and skipping any complex structures.
fn collect_catalog_dependencies(mapping: &YamlBlockMapping) -> Dependencies {
    let mut deps = Vec::new();
    for entry in mapping.entries() {
        if let Some((name, version_node)) = parse_catalog_mapping_entry(entry) {
            if let Some(version) = extract_catalog_scalar_from_block_node(&version_node) {
                deps.push((name, version));
            }
        }
    }

    Dependencies(deps.into_boxed_slice())
}

/// Extracts a scalar string from an implicit mapping key (flow YAML/JSON node).
fn extract_catalog_scalar_from_implicit_key(key: &AnyYamlMappingImplicitKey) -> Option<Box<str>> {
    if let Some(flow_yaml) = key.as_yaml_flow_yaml_node() {
        return flow_yaml
            .content()?
            .value_token()
            .ok()
            .and_then(|token| normalize_catalog_scalar_text(token.text()));
    }

    if let Some(flow_json) = key.as_yaml_flow_json_node() {
        if let Some(content) = flow_json.content() {
            return match content {
                AnyYamlJsonContent::YamlDoubleQuotedScalar(scalar) => scalar
                    .value_token()
                    .ok()
                    .and_then(|token| normalize_catalog_scalar_text(token.text())),
                AnyYamlJsonContent::YamlSingleQuotedScalar(scalar) => scalar
                    .value_token()
                    .ok()
                    .and_then(|token| normalize_catalog_scalar_text(token.text())),
                AnyYamlJsonContent::YamlFlowMapping(_)
                | AnyYamlJsonContent::YamlFlowSequence(_) => None,
            };
        }
    }

    None
}

/// Extracts a scalar string from a flow node (plain/double/single quoted).
fn extract_catalog_scalar_from_flow_node(node: &AnyYamlFlowNode) -> Option<Box<str>> {
    if let Some(flow_yaml) = node.as_yaml_flow_yaml_node() {
        return flow_yaml
            .content()?
            .value_token()
            .ok()
            .and_then(|token| normalize_catalog_scalar_text(token.text()));
    }

    if let Some(flow_json) = node.as_yaml_flow_json_node() {
        if let Some(content) = flow_json.content() {
            return match content {
                AnyYamlJsonContent::YamlDoubleQuotedScalar(scalar) => scalar
                    .value_token()
                    .ok()
                    .and_then(|token| normalize_catalog_scalar_text(token.text())),
                AnyYamlJsonContent::YamlSingleQuotedScalar(scalar) => scalar
                    .value_token()
                    .ok()
                    .and_then(|token| normalize_catalog_scalar_text(token.text())),
                AnyYamlJsonContent::YamlFlowMapping(_)
                | AnyYamlJsonContent::YamlFlowSequence(_) => None,
            };
        }
    }

    None
}

/// Extracts a scalar string from a block node when represented as a flow scalar
/// in block context. Returns `None` for mapping/sequence content.
fn extract_catalog_scalar_from_block_node(node: &AnyYamlBlockNode) -> Option<Box<str>> {
    if let Some(flow) = node.as_yaml_flow_in_block_node() {
        let flow_node = flow.flow().ok()?;
        return extract_catalog_scalar_from_flow_node(&flow_node);
    }

    if let Some(block) = node.as_any_yaml_block_in_block_node() {
        if let Some(mapping) = block.as_yaml_block_mapping() {
            if mapping.entries().is_empty() {
                return None;
            }
        }
    }

    None
}

/// Trims whitespace and surrounding quotes from a scalar token, returning `None`
/// for empty values after normalization.
fn normalize_catalog_scalar_text(value: &str) -> Option<Box<str>> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return None;
    }

    let without_quotes = if trimmed.starts_with('"') && trimmed.ends_with('"') && trimmed.len() >= 2
    {
        &trimmed[1..trimmed.len() - 1]
    } else if trimmed.starts_with('\'') && trimmed.ends_with('\'') && trimmed.len() >= 2 {
        &trimmed[1..trimmed.len() - 1]
    } else {
        trimmed
    };

    if without_quotes.is_empty() {
        None
    } else {
        Some(without_quotes.to_owned().into_boxed_str())
    }
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
}
