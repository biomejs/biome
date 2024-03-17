use biome_rowan::FileSourceError;
use std::path::Path;

/// Enum of the different ECMAScript standard versions.
/// The versions are ordered in increasing order; The newest version comes last.
///
/// Defaults to the latest stable ECMAScript standard.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
pub enum LanguageVersion {
    ES2022,

    /// The next, not yet finalized ECMAScript version
    ESNext,
}

impl LanguageVersion {
    /// Returns the latest finalized ECMAScript version
    pub const fn latest() -> Self {
        LanguageVersion::ES2022
    }
}

impl Default for LanguageVersion {
    fn default() -> Self {
        Self::latest()
    }
}

/// Is the source file an ECMAScript Module or Script.
/// Changes the parsing semantic.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema,))]
#[derive(
    Debug, Clone, Default, Copy, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum ModuleKind {
    /// An ECMAScript [Script](https://tc39.es/ecma262/multipage/ecmascript-language-scripts-and-modules.html#sec-scripts)
    Script,

    /// An ECMAScript [Module](https://tc39.es/ecma262/multipage/ecmascript-language-scripts-and-modules.html#sec-modules)
    #[default]
    Module,
}

impl ModuleKind {
    pub const fn is_script(&self) -> bool {
        matches!(self, ModuleKind::Script)
    }
    pub const fn is_module(&self) -> bool {
        matches!(self, ModuleKind::Module)
    }
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(
    Debug, Copy, Clone, Eq, PartialEq, Hash, Default, serde::Serialize, serde::Deserialize,
)]
pub enum LanguageVariant {
    /// Standard JavaScript or TypeScript syntax without any extensions
    #[default]
    Standard,

    /// Standard JavaScript or TypeScript syntax with some restrictions for future-proof compatibility with JSX
    StandardRestricted,

    /// Allows JSX syntax inside a JavaScript or TypeScript file
    Jsx,
}

impl LanguageVariant {
    pub const fn is_standard(&self) -> bool {
        matches!(self, LanguageVariant::Standard)
    }
    pub const fn is_standard_restricted(&self) -> bool {
        matches!(self, LanguageVariant::StandardRestricted)
    }
    pub const fn is_jsx(&self) -> bool {
        matches!(self, LanguageVariant::Jsx)
    }
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(
    Debug, Copy, Clone, Eq, PartialEq, Default, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum Language {
    #[default]
    JavaScript,

    /// TypeScript source with or without JSX.
    /// `definition_file` must be true for `d.ts` files.
    TypeScript { definition_file: bool },
}

impl Language {
    pub const fn is_javascript(&self) -> bool {
        matches!(self, Language::JavaScript)
    }
    pub const fn is_typescript(&self) -> bool {
        matches!(self, Language::TypeScript { .. })
    }

    pub const fn is_definition_file(&self) -> bool {
        matches!(
            self,
            Language::TypeScript {
                definition_file: true
            }
        )
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(
    Debug, Clone, Default, Copy, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum EmbeddingKind {
    Astro,
    Vue,
    Svelte,
    #[default]
    None,
}

impl EmbeddingKind {
    pub const fn is_astro(&self) -> bool {
        matches!(self, EmbeddingKind::Astro)
    }
    pub const fn is_vue(&self) -> bool {
        matches!(self, EmbeddingKind::Vue)
    }
    pub const fn is_svelte(&self) -> bool {
        matches!(self, EmbeddingKind::Svelte)
    }
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(
    Debug, Clone, Default, Copy, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct JsFileSource {
    language: Language,
    variant: LanguageVariant,
    module_kind: ModuleKind,
    version: LanguageVersion,
    /// Used to mark if the source is being used for an Astro, Svelte or Vue file
    embedding_kind: EmbeddingKind,
}

impl JsFileSource {
    /// language: JS, variant: Standard, module_kind: Module, version: Latest
    pub fn js_module() -> Self {
        Self::default()
    }

    /// language: JS, variant: Standard, module_kind: Script, version: Latest
    pub fn js_script() -> Self {
        Self::default().with_module_kind(ModuleKind::Script)
    }

    /// language: JS, variant: JSX, module_kind: Module, version: Latest
    pub fn jsx() -> JsFileSource {
        Self::js_module().with_variant(LanguageVariant::Jsx)
    }

    /// language: TS, variant: Standard, module_kind: Module, version: Latest
    pub fn ts() -> JsFileSource {
        Self {
            language: Language::TypeScript {
                definition_file: false,
            },
            ..Self::default()
        }
    }

    /// language: TS, variant: StandardRestricted, module_kind: Module, version: Latest
    pub fn ts_restricted() -> JsFileSource {
        Self::ts().with_variant(LanguageVariant::StandardRestricted)
    }

    /// language: TS, variant: JSX, module_kind: Module, version: Latest
    pub fn tsx() -> JsFileSource {
        Self::ts().with_variant(LanguageVariant::Jsx)
    }

    /// TypeScript definition file
    /// language: TS, ambient, variant: Standard, module_kind: Module, version: Latest
    pub fn d_ts() -> JsFileSource {
        Self {
            language: Language::TypeScript {
                definition_file: true,
            },
            ..Self::default()
        }
    }

    /// Astro file definition
    pub fn astro() -> Self {
        Self::ts().with_embedding_kind(EmbeddingKind::Astro)
    }

    /// Vue file definition
    pub fn vue() -> Self {
        Self::js_module().with_embedding_kind(EmbeddingKind::Vue)
    }

    /// Svelte file definition
    pub fn svelte() -> Self {
        Self::js_module().with_embedding_kind(EmbeddingKind::Svelte)
    }

    pub const fn with_module_kind(mut self, kind: ModuleKind) -> Self {
        self.module_kind = kind;
        self
    }

    pub const fn with_version(mut self, version: LanguageVersion) -> Self {
        self.version = version;
        self
    }

    pub const fn with_variant(mut self, variant: LanguageVariant) -> Self {
        self.variant = variant;
        self
    }

    pub const fn with_embedding_kind(mut self, kind: EmbeddingKind) -> Self {
        self.embedding_kind = kind;
        self
    }

    pub const fn language(&self) -> Language {
        self.language
    }

    pub const fn variant(&self) -> LanguageVariant {
        self.variant
    }

    pub const fn version(&self) -> LanguageVersion {
        self.version
    }

    pub const fn module_kind(&self) -> ModuleKind {
        self.module_kind
    }

    pub const fn is_module(&self) -> bool {
        self.module_kind.is_module()
    }

    pub const fn is_typescript(&self) -> bool {
        self.language.is_typescript()
    }

    pub const fn is_jsx(&self) -> bool {
        self.variant.is_jsx()
    }

    pub const fn as_embedding_kind(&self) -> &EmbeddingKind {
        &self.embedding_kind
    }

    pub fn file_extension(&self) -> &str {
        match self.language {
            Language::JavaScript => {
                if matches!(self.variant, LanguageVariant::Jsx) {
                    return "jsx";
                }
                match self.module_kind {
                    ModuleKind::Script => "cjs",
                    ModuleKind::Module => "js",
                }
            }
            Language::TypeScript { .. } => {
                if matches!(self.variant, LanguageVariant::Jsx) {
                    "tsx"
                } else {
                    "ts"
                }
            }
        }
    }
}

impl TryFrom<&Path> for JsFileSource {
    type Error = FileSourceError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let file_name = path
            .file_name()
            .ok_or_else(|| FileSourceError::MissingFileName(path.into()))?
            .to_str()
            .ok_or_else(|| FileSourceError::MissingFileName(path.into()))?;

        let extension = path
            .extension()
            .ok_or_else(|| FileSourceError::MissingFileExtension(path.into()))?
            .to_str()
            .ok_or_else(|| FileSourceError::MissingFileExtension(path.into()))?;

        compute_source_type_from_path_or_extension(file_name, extension)
    }
}

/// It deduce the [JsFileSource] from the file name and its extension
fn compute_source_type_from_path_or_extension(
    file_name: &str,
    extension: &str,
) -> Result<JsFileSource, FileSourceError> {
    let source_type = if file_name.ends_with(".d.ts")
        || file_name.ends_with(".d.mts")
        || file_name.ends_with(".d.cts")
    {
        JsFileSource::d_ts()
    } else {
        match extension {
            "js" | "mjs" | "jsx" => JsFileSource::jsx(),
            "cjs" => JsFileSource::js_script(),
            "ts" => JsFileSource::ts(),
            "mts" | "cts" => JsFileSource::ts_restricted(),
            "tsx" => JsFileSource::tsx(),
            // TODO: Remove once we have full support of astro files
            "astro" => JsFileSource::astro(),
            "vue" => JsFileSource::vue(),
            "svelte" => JsFileSource::svelte(),
            _ => {
                return Err(FileSourceError::UnknownExtension(
                    file_name.into(),
                    extension.into(),
                ));
            }
        }
    };
    Ok(source_type)
}
