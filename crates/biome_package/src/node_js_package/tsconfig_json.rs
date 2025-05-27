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

    fn deserialize_manifest(root: &LanguageRoot<Self::Language>) -> Deserialized<Self> {
        deserialize_from_json_ast::<Self>(root, "")
    }

    fn read_manifest(fs: &dyn biome_fs::FileSystem, path: &Utf8Path) -> Deserialized<Self> {
        match fs.read_file_from_path(path) {
            Ok(content) => {
                let (manifest, errors) = Self::parse(true, path, &content);
                Deserialized::new(Some(manifest), errors)
            }
            Err(error) => Deserialized::new(None, vec![Error::from(error)]),
        }
    }
}

impl TsConfigJson {
    fn parse(root: bool, path: &Utf8Path, json: &str) -> (Self, Vec<Error>) {
        let (tsconfig, diagnostics) = deserialize_from_json_str(
            json,
            JsonParserOptions::default()
                .with_allow_comments()
                .with_allow_trailing_commas(),
            path.file_name().unwrap_or("tsconfig.json"),
        )
        .consume();

        let mut tsconfig: Self = tsconfig.unwrap_or_default();
        tsconfig.root = root;
        tsconfig.path = path.to_path_buf();
        let directory = path.parent();
        if let Some(base_url) = tsconfig.compiler_options.base_url {
            tsconfig.compiler_options.base_url =
                directory.map(|dir| normalize_path(&dir.join(base_url)));
        }
        if tsconfig.compiler_options.paths.is_some() {
            tsconfig.compiler_options.paths_base =
                tsconfig.compiler_options.base_url.as_ref().map_or_else(
                    || directory.map_or_else(Default::default, Utf8Path::to_path_buf),
                    Clone::clone,
                );
        }
        (tsconfig, diagnostics)
    }
}

#[derive(Clone, Debug, Default, Deserializable)]
pub struct CompilerOptions {
    pub base_url: Option<Utf8PathBuf>,

    /// Path aliases.
    pub paths: Option<CompilerOptionsPathsMap>,

    /// The actual base from where path aliases are resolved.
    #[deserializable(skip)]
    paths_base: Utf8PathBuf,

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
