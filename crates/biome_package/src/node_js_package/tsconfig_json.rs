use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::{LanguageRoot, Manifest};
use biome_deserialize::{
    json::{deserialize_from_json_ast, deserialize_from_json_str},
    Deserializable, DeserializableValue, DeserializationContext,
};
use biome_deserialize::{DeserializableType, Deserialized};
use biome_deserialize_macros::Deserializable;
use biome_diagnostics::Error;
use biome_json_parser::JsonParserOptions;
use biome_json_syntax::JsonLanguage;
use camino::{Utf8Path, Utf8PathBuf};
use oxc_resolver::{CompilerOptionsPathsMap, PathUtil, TsConfig, TsconfigReferences};

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

    /// Bubbled up project references with a reference to their tsconfig.
    pub references: Vec<ProjectReference>,
}

impl Manifest for TsConfigJson {
    type Language = JsonLanguage;

    fn deserialize_manifest(root: &LanguageRoot<Self::Language>) -> Deserialized<Self> {
        deserialize_from_json_ast::<TsConfigJson>(root, "")
    }
}

#[expect(refining_impl_trait)]
impl TsConfig for TsConfigJson {
    type Co = CompilerOptions;

    fn root(&self) -> bool {
        self.root
    }

    fn path(&self) -> &Path {
        self.path.as_std_path()
    }

    fn directory(&self) -> &Path {
        debug_assert!(self.path.file_name().is_some());
        self.path.parent().unwrap().as_std_path()
    }

    fn compiler_options(&self) -> &Self::Co {
        &self.compiler_options
    }

    fn compiler_options_mut(&mut self) -> &mut Self::Co {
        &mut self.compiler_options
    }

    fn extends(&self) -> impl Iterator<Item = &str> {
        let specifiers = match &self.extends {
            Some(ExtendsField::Single(specifier)) => {
                vec![specifier.as_str()]
            }
            Some(ExtendsField::Multiple(specifiers)) => {
                specifiers.iter().map(String::as_str).collect()
            }
            None => Vec::new(),
        };
        specifiers.into_iter()
    }

    fn load_references(&mut self, references: &TsconfigReferences) -> bool {
        match references {
            TsconfigReferences::Disabled => {
                self.references.drain(..);
            }
            TsconfigReferences::Auto => {}
            TsconfigReferences::Paths(paths) => {
                self.references = paths
                    .iter()
                    .filter_map(|path| path.clone().try_into().ok())
                    .map(|path| ProjectReference {
                        path,
                        tsconfig: None,
                    })
                    .collect();
            }
        }

        !self.references.is_empty()
    }

    fn references(&self) -> impl Iterator<Item = &ProjectReference> {
        self.references.iter()
    }

    fn references_mut(&mut self) -> impl Iterator<Item = &mut ProjectReference> {
        self.references.iter_mut()
    }
}

impl TsConfigJson {
    pub fn parse(root: bool, path: &Utf8Path, json: &mut str) -> (Self, Vec<Error>) {
        let (tsconfig, diagnostics) = deserialize_from_json_str(
            json,
            JsonParserOptions::default()
                .with_allow_comments()
                .with_allow_trailing_commas(),
            path.file_name().unwrap_or("tsconfig.json"),
        )
        .consume();

        let mut tsconfig: TsConfigJson = tsconfig.unwrap_or_default();
        tsconfig.root = root;
        tsconfig.path = path.to_path_buf();
        let directory = tsconfig.directory().to_path_buf();
        if let Some(base_url) = tsconfig.compiler_options.base_url {
            tsconfig.compiler_options.base_url = directory.normalize_with(base_url).try_into().ok();
        }
        if tsconfig.compiler_options.paths.is_some() {
            tsconfig.compiler_options.paths_base =
                tsconfig.compiler_options.base_url.as_ref().map_or_else(
                    || directory.try_into().expect("Non UTF-8 character in path"),
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
}

impl oxc_resolver::CompilerOptions for CompilerOptions {
    fn base_url(&self) -> Option<&Path> {
        self.base_url.as_deref().map(Utf8Path::as_std_path)
    }

    fn set_base_url(&mut self, base_url: PathBuf) {
        self.base_url = base_url.try_into().ok();
    }

    fn paths(&self) -> Option<&CompilerOptionsPathsMap> {
        self.paths.as_ref()
    }

    fn paths_mut(&mut self) -> Option<&mut CompilerOptionsPathsMap> {
        self.paths.as_mut()
    }

    fn set_paths(&mut self, paths: Option<CompilerOptionsPathsMap>) {
        self.paths = paths;
    }

    fn paths_base(&self) -> &Path {
        self.paths_base.as_std_path()
    }

    fn set_paths_base(&mut self, paths_base: PathBuf) {
        self.paths_base = paths_base.try_into().expect("non UTF-8 character in path");
    }
}

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

    #[deserializable(skip)]
    pub tsconfig: Option<Arc<TsConfigJson>>,
}

impl oxc_resolver::ProjectReference for ProjectReference {
    type Tc = TsConfigJson;

    fn path(&self) -> &Path {
        self.path.as_std_path()
    }

    fn tsconfig(&self) -> Option<Arc<Self::Tc>> {
        self.tsconfig.clone()
    }

    fn set_tsconfig(&mut self, tsconfig: Arc<Self::Tc>) {
        self.tsconfig.replace(tsconfig);
    }
}
