use std::hash::BuildHasherDefault;

use crate::{LanguageRoot, Manifest};
use biome_deserialize::{
    Deserializable, DeserializableValue, DeserializationContext,
    json::{deserialize_from_json_ast, deserialize_from_json_str},
};
use biome_deserialize::{DeserializableType, Deserialized};
use biome_deserialize_macros::Deserializable;
use biome_diagnostics::Error;
use biome_fs::normalize_path;
use biome_json_parser::JsonParserOptions;
use biome_json_syntax::JsonLanguage;
use camino::{Utf8Path, Utf8PathBuf};
use indexmap::IndexMap;
use rustc_hash::FxHasher;

#[derive(Clone, Debug, Default, Deserializable)]
#[deserializable(unknown_fields = "allow")]
pub struct TsConfigJson {
    /// Whether this is the caller tsconfig.
    /// Used for final template variable substitution when all configs are
    /// extended and merged.
    #[deserializable(skip)]
    pub root: bool,

    /// Path to `tsconfig.json`. Contains the `tsconfig.json` filename.
    #[deserializable(skip)]
    pub path: Utf8PathBuf,

    pub extends: Option<ExtendsField>,

    pub compiler_options: CompilerOptions,

    /// Project references.
    pub references: Vec<ProjectReference>,
}

impl Manifest for TsConfigJson {
    type Language = JsonLanguage;

    fn deserialize_manifest(
        root: &LanguageRoot<Self::Language>,
        path: &Utf8Path,
    ) -> Deserialized<Self> {
        let deserialized = deserialize_from_json_ast::<Self>(root, "");
        let (mut tsconfig, errors) = deserialized.consume();
        if let Some(manifest) = tsconfig.as_mut() {
            manifest.initialise_paths(path);
        }

        Deserialized::new(tsconfig, errors)
    }

    fn read_manifest(fs: &dyn biome_fs::FileSystem, path: &Utf8Path) -> Deserialized<Self> {
        match fs.read_file_from_path(path) {
            Ok(content) => {
                let (manifest, errors) = Self::parse(path, &content);
                Deserialized::new(Some(manifest), errors)
            }
            Err(error) => Deserialized::new(None, vec![Error::from(error)]),
        }
    }
}

impl TsConfigJson {
    fn parse(path: &Utf8Path, json: &str) -> (Self, Vec<Error>) {
        let (tsconfig, diagnostics) = deserialize_from_json_str(
            json,
            JsonParserOptions::default()
                .with_allow_comments()
                .with_allow_trailing_commas(),
            path.file_name().unwrap_or("tsconfig.json"),
        )
        .consume();

        let mut tsconfig: Self = tsconfig.unwrap_or_default();
        tsconfig.initialise_paths(path);

        (tsconfig, diagnostics)
    }

    /// Initialises the paths stored in the manifest.
    ///
    /// `path` must be an absolute path to the `tsconfig.json` file itself.
    fn initialise_paths(&mut self, path: &Utf8Path) {
        // Some tests that use UNIX paths are not recognised as absolute on
        // Windows...
        #[cfg(not(target_os = "windows"))]
        debug_assert!(path.is_absolute());

        self.root = true; // For now we only support root configs.

        self.path = path.to_path_buf();
        let directory = path.parent();
        if let Some(base_url) = self.compiler_options.base_url.as_ref() {
            self.compiler_options.base_url =
                directory.map(|dir| normalize_path(&dir.join(base_url)));
        }
        if self.compiler_options.paths.is_some() {
            self.compiler_options.paths_base = self.compiler_options.base_url.as_ref().map_or_else(
                || directory.map_or_else(Default::default, Utf8Path::to_path_buf),
                Clone::clone,
            );
        }
    }

    /// Returns whether the given `path` matches a configured path alias.
    pub fn matches_path_alias(&self, path: &str) -> bool {
        self.compiler_options.paths.as_ref().is_some_and(|paths| {
            paths
                .keys()
                .any(|alias_path| match alias_path.split_once('*') {
                    Some((before, after)) => path.starts_with(before) && path.ends_with(after),
                    None => path == alias_path,
                })
        })
    }

    /// Returns the base identifier from the JSX factory function name.
    pub fn jsx_factory_identifier(&self) -> Option<&str> {
        self.compiler_options.jsx_factory.as_deref()
    }

    /// Returns the base identifier from the JSX fragment factory function name.
    pub fn jsx_fragment_factory_identifier(&self) -> Option<&str> {
        self.compiler_options.jsx_fragment_factory.as_deref()
    }
}

#[derive(Clone, Debug, Default, Deserializable)]
pub struct CompilerOptions {
    /// https://www.typescriptlang.org/tsconfig/#baseUrl
    ///
    /// The base URL is normalised to an absolute path after parsing.
    pub base_url: Option<Utf8PathBuf>,

    /// Path aliases.
    pub paths: Option<CompilerOptionsPathsMap>,

    /// The actual base from where path aliases are resolved.
    ///
    /// The base URL is normalised to an absolute path.
    #[deserializable(skip)]
    pub paths_base: Utf8PathBuf,

    /// See: https://www.typescriptlang.org/tsconfig/#typeRoots
    #[deserializable(rename = "typeRoots")]
    pub type_roots: Option<Vec<String>>,

    /// See: https://www.typescriptlang.org/tsconfig/#jsxFactory
    /// Specifies the JSX factory function to use when targeting react JSX emit.
    /// The value is normalized to the base identifier during deserialization.
    /// For example, "React.createElement" becomes "React", "h" stays "h".
    #[deserializable(rename = "jsxFactory")]
    pub jsx_factory: Option<JsxFactoryIdentifier>,

    /// See: https://www.typescriptlang.org/tsconfig/#jsxFragmentFactory
    /// Specifies the JSX fragment factory function to use when targeting react JSX emit.
    /// The value is normalized to the base identifier during deserialization.
    /// For example, "React.Fragment" becomes "React", "Fragment" stays "Fragment".
    #[deserializable(rename = "jsxFragmentFactory")]
    pub jsx_fragment_factory: Option<JsxFactoryIdentifier>,
}

pub type CompilerOptionsPathsMap = IndexMap<String, Vec<String>, BuildHasherDefault<FxHasher>>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExtendsField {
    Single(String),
    Multiple(Vec<String>),
}

impl Deserializable for ExtendsField {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        if value.visitable_type()? == DeserializableType::Array {
            Vec::<String>::deserialize(ctx, value, name).map(Self::Multiple)
        } else {
            String::deserialize(ctx, value, name).map(Self::Single)
        }
    }
}

#[derive(Clone, Debug, Default, Deserializable)]
pub struct ProjectReference {
    pub path: Utf8PathBuf,
}

/// A JSX factory identifier that is normalized during deserialization.
///
/// When deserializing from JSON, if the value contains a dot (e.g., "React.createElement"),
/// only the base identifier before the first dot is kept (e.g., "React").
/// This normalization happens once during deserialization for efficiency.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct JsxFactoryIdentifier(String);

impl JsxFactoryIdentifier {
    /// Returns the normalized identifier as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::ops::Deref for JsxFactoryIdentifier {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deserializable for JsxFactoryIdentifier {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        let full_name = String::deserialize(ctx, value, name)?;
        // Extract the base identifier (everything before the first dot)
        let base_identifier = full_name.split('.').next().unwrap().trim();

        // Return None if the identifier is empty or whitespace-only
        // to avoid "configured but unusable" states downstream
        if base_identifier.is_empty() {
            return None;
        }

        Some(Self(base_identifier.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jsx_factory_normalization() {
        let json = r#"{
            "compilerOptions": {
                "jsxFactory": "React.createElement",
                "jsxFragmentFactory": "React.Fragment"
            }
        }"#;

        let (tsconfig, _) = TsConfigJson::parse(Utf8Path::new("/test/tsconfig.json"), json);

        assert_eq!(
            tsconfig.jsx_factory_identifier(),
            Some("React"),
            "React.createElement should be normalized to React"
        );
        assert_eq!(
            tsconfig.jsx_fragment_factory_identifier(),
            Some("React"),
            "React.Fragment should be normalized to React"
        );
    }

    #[test]
    fn test_jsx_factory_simple_identifier() {
        let json = r#"{
            "compilerOptions": {
                "jsxFactory": "h",
                "jsxFragmentFactory": "Fragment"
            }
        }"#;

        let (tsconfig, _) = TsConfigJson::parse(Utf8Path::new("/test/tsconfig.json"), json);

        assert_eq!(
            tsconfig.jsx_factory_identifier(),
            Some("h"),
            "h should remain as h"
        );
        assert_eq!(
            tsconfig.jsx_fragment_factory_identifier(),
            Some("Fragment"),
            "Fragment should remain as Fragment"
        );
    }

    #[test]
    fn test_jsx_factory_namespaced() {
        let json = r#"{
            "compilerOptions": {
                "jsxFactory": "MyLib.createElement",
                "jsxFragmentFactory": "MyLib.Fragment"
            }
        }"#;

        let (tsconfig, _) = TsConfigJson::parse(Utf8Path::new("/test/tsconfig.json"), json);

        assert_eq!(
            tsconfig.jsx_factory_identifier(),
            Some("MyLib"),
            "MyLib.createElement should be normalized to MyLib"
        );
        assert_eq!(
            tsconfig.jsx_fragment_factory_identifier(),
            Some("MyLib"),
            "MyLib.Fragment should be normalized to MyLib"
        );
    }

    #[test]
    fn test_jsx_factory_missing() {
        let json = r#"{
            "compilerOptions": {}
        }"#;

        let (tsconfig, _) = TsConfigJson::parse(Utf8Path::new("/test/tsconfig.json"), json);

        assert_eq!(
            tsconfig.jsx_factory_identifier(),
            None,
            "Missing jsxFactory should return None"
        );
        assert_eq!(
            tsconfig.jsx_fragment_factory_identifier(),
            None,
            "Missing jsxFragmentFactory should return None"
        );
    }

    #[test]
    fn test_jsx_factory_deeply_nested() {
        let json = r#"{
            "compilerOptions": {
                "jsxFactory": "Deeply.Nested.Function",
                "jsxFragmentFactory": "Another.Nested.Fragment"
            }
        }"#;

        let (tsconfig, _) = TsConfigJson::parse(Utf8Path::new("/test/tsconfig.json"), json);

        assert_eq!(
            tsconfig.jsx_factory_identifier(),
            Some("Deeply"),
            "Deeply.Nested.Function should be normalized to Deeply"
        );
        assert_eq!(
            tsconfig.jsx_fragment_factory_identifier(),
            Some("Another"),
            "Another.Nested.Fragment should be normalized to Another"
        );
    }

    #[test]
    fn test_jsx_factory_efficiency() {
        // This test verifies that normalization happens during deserialization,
        // not on every access
        let json = r#"{
            "compilerOptions": {
                "jsxFactory": "React.createElement"
            }
        }"#;

        let (tsconfig, _) = TsConfigJson::parse(Utf8Path::new("/test/tsconfig.json"), json);

        assert_eq!(tsconfig.jsx_factory_identifier(), Some("React"));
        assert_eq!(tsconfig.jsx_factory_identifier(), Some("React")); // consistency
    }

    #[test]
    fn test_jsx_factory_empty_string() {
        // Empty strings should be treated as None to avoid "configured but unusable" states
        let json = r#"{
            "compilerOptions": {
                "jsxFactory": "",
                "jsxFragmentFactory": ""
            }
        }"#;

        let (tsconfig, _) = TsConfigJson::parse(Utf8Path::new("/test/tsconfig.json"), json);

        assert_eq!(
            tsconfig.jsx_factory_identifier(),
            None,
            "Empty jsxFactory should be None"
        );
        assert_eq!(
            tsconfig.jsx_fragment_factory_identifier(),
            None,
            "Empty jsxFragmentFactory should be None"
        );
    }

    #[test]
    fn test_jsx_factory_whitespace_only() {
        // Whitespace-only strings should be treated as None
        let json = r#"{
            "compilerOptions": {
                "jsxFactory": "   ",
                "jsxFragmentFactory": "\t\n"
            }
        }"#;

        let (tsconfig, _) = TsConfigJson::parse(Utf8Path::new("/test/tsconfig.json"), json);

        assert_eq!(
            tsconfig.jsx_factory_identifier(),
            None,
            "Whitespace-only jsxFactory should be None"
        );
        assert_eq!(
            tsconfig.jsx_fragment_factory_identifier(),
            None,
            "Whitespace-only jsxFragmentFactory should be None"
        );
    }

    #[test]
    fn test_jsx_factory_dot_only() {
        // A string with only dots should result in None (empty base identifier)
        let json = r#"{
            "compilerOptions": {
                "jsxFactory": ".",
                "jsxFragmentFactory": "..."
            }
        }"#;

        let (tsconfig, _) = TsConfigJson::parse(Utf8Path::new("/test/tsconfig.json"), json);

        assert_eq!(
            tsconfig.jsx_factory_identifier(),
            None,
            "Dot-only jsxFactory should be None"
        );
        assert_eq!(
            tsconfig.jsx_fragment_factory_identifier(),
            None,
            "Dot-only jsxFragmentFactory should be None"
        );
    }

    #[test]
    fn test_jsx_factory_with_surrounding_whitespace() {
        // Whitespace should be trimmed
        let json = r#"{
            "compilerOptions": {
                "jsxFactory": "  React.createElement  ",
                "jsxFragmentFactory": "\tFragment\n"
            }
        }"#;

        let (tsconfig, _) = TsConfigJson::parse(Utf8Path::new("/test/tsconfig.json"), json);

        assert_eq!(
            tsconfig.jsx_factory_identifier(),
            Some("React"),
            "Whitespace should be trimmed from jsxFactory"
        );
        assert_eq!(
            tsconfig.jsx_fragment_factory_identifier(),
            Some("Fragment"),
            "Whitespace should be trimmed from jsxFragmentFactory"
        );
    }
}
