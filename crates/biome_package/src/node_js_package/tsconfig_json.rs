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
