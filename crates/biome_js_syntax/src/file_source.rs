use biome_rowan::FileSourceError;
use biome_string_case::StrLikeExtension;
use camino::Utf8Path;
use std::borrow::Cow;

/// Enum of the different ECMAScript standard versions.
/// The versions are ordered in increasing order; The newest version comes last.
///
/// Defaults to the latest stable ECMAScript standard.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
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
#[serde(rename_all = "camelCase")]
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
    pub fn jsx() -> Self {
        Self::js_module().with_variant(LanguageVariant::Jsx)
    }

    /// language: TS, variant: Standard, module_kind: Module, version: Latest
    pub fn ts() -> Self {
        Self {
            language: Language::TypeScript {
                definition_file: false,
            },
            ..Self::default()
        }
    }

    /// language: TS, variant: StandardRestricted, module_kind: Module, version: Latest
    pub fn ts_restricted() -> Self {
        Self::ts().with_variant(LanguageVariant::StandardRestricted)
    }

    /// language: TS, variant: JSX, module_kind: Module, version: Latest
    pub fn tsx() -> Self {
        Self::ts().with_variant(LanguageVariant::Jsx)
    }

    /// TypeScript definition file
    /// language: TS, ambient, variant: Standard, module_kind: Module, version: Latest
    pub fn d_ts() -> Self {
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

    pub fn set_module_kind(&mut self, kind: ModuleKind) {
        self.module_kind = kind;
    }

    pub fn set_variant(&mut self, variant: LanguageVariant) {
        self.variant = variant;
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

    pub const fn is_script(&self) -> bool {
        self.module_kind.is_script()
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
                match self.variant {
                    LanguageVariant::Standard => "ts",
                    LanguageVariant::StandardRestricted => {
                        // This could also be `mts`.
                        // We choose `cts` because we expect this extension to be more widely used.
                        // Moreover, it allows more valid syntax such as `import type` with import
                        // attributes (See `noTypeOnlyImportAttributes` syntax rule).
                        "cts"
                    }
                    LanguageVariant::Jsx => "tsx",
                }
            }
        }
    }

    /// Try to return the JS file source corresponding to this file name from well-known files
    pub fn try_from_well_known(path: &Utf8Path) -> Result<Self, FileSourceError> {
        // Be careful with definition files, because `Path::extension()` only
        // returns the extension after the _last_ dot:
        let file_name = path.file_name().ok_or(FileSourceError::MissingFileName)?;
        if file_name.ends_with(".d.ts") {
            return Self::try_from_extension("d.ts");
        } else if file_name.ends_with(".d.mts") {
            return Self::try_from_extension("d.mts");
        } else if file_name.ends_with(".d.cts") {
            return Self::try_from_extension("d.cts");
        }

        match path.extension() {
            Some(extension) => Self::try_from_extension(extension),
            None => Err(FileSourceError::MissingFileExtension),
        }
    }

    /// Try to return the JS file source corresponding to this file extension
    pub fn try_from_extension(extension: &str) -> Result<Self, FileSourceError> {
        // We assume the file extension is normalized to lowercase
        match extension {
            "js" | "mjs" => Ok(Self::js_module()),
            "jsx" => Ok(Self::jsx()),
            "cjs" => Ok(Self::js_script()),
            "ts" => Ok(Self::ts()),
            "mts" | "cts" => Ok(Self::ts_restricted()),
            "tsx" => Ok(Self::tsx()),
            // Note: the extension passed to this function can contain dots,
            // this should be handled properly by the extension provider
            "d.ts" | "d.mts" | "d.cts" => Ok(Self::d_ts()),
            // TODO: Remove once we have full support of astro files
            "astro" => Ok(Self::astro()),
            // TODO: Remove once we have full support of vue files
            "vue" => Ok(Self::vue()),
            // TODO: Remove once we have full support of svelte files
            "svelte" => Ok(Self::svelte()),
            _ => Err(FileSourceError::UnknownExtension),
        }
    }

    /// Try to return the JS file source corresponding to this language ID
    ///
    /// See the [LSP spec] and [VS Code spec] for a list of language identifiers
    ///
    /// [LSP spec]: https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocumentItem
    /// [VS Code spec]: https://code.visualstudio.com/docs/languages/identifiers
    pub fn try_from_language_id(language_id: &str) -> Result<Self, FileSourceError> {
        match language_id {
            // We use Self::jsx() for the javascript language id
            // because `.js` files will be associated with the javascript language id
            // and we already use Self::jsx() for `.js` files in try_from_extension().
            // This will make LSP and CLI work consistently.
            //
            // This also fixes the issue where some plugins like babel plugin will
            // associate `.jsx` files with the javascript language id.
            //
            // TODO: This should be considered as an ad hoc fix.
            // We might want to reconsider the design and variants of the file source.
            "javascript" => Ok(Self::jsx()),
            "typescript" => Ok(Self::ts()),
            "javascriptreact" => Ok(Self::jsx()),
            "typescriptreact" => Ok(Self::tsx()),
            // TODO: Remove once we have full support of astro files
            "astro" => Ok(Self::astro()),
            // TODO: Remove once we have full support of vue files
            "vue" => Ok(Self::vue()),
            // TODO: Remove once we have full support of svelte files
            "svelte" => Ok(Self::svelte()),
            _ => Err(FileSourceError::UnknownLanguageId),
        }
    }
}

impl TryFrom<&Utf8Path> for JsFileSource {
    type Error = FileSourceError;

    fn try_from(path: &Utf8Path) -> Result<Self, Self::Error> {
        if let Ok(file_source) = Self::try_from_well_known(path) {
            return Ok(file_source);
        }

        let filename = path
            .file_name()
            // We assume the file extensions are case-insensitive.
            // Thus, we normalize the filrname to lowercase.
            .map(|filename| filename.to_ascii_lowercase_cow());

        // We assume the file extensions are case-insensitive
        // and we use the lowercase form of them for pattern matching
        // TODO: This should be extracted to a dedicated function, maybe in biome_fs
        // because the same logic is also used in DocumentFileSource::from_path_optional
        // and we may support more and more extensions with more than one dots.
        let extension = &match filename {
            Some(filename) if filename.ends_with(".d.ts") => Cow::Borrowed("d.ts"),
            Some(filename) if filename.ends_with(".d.mts") => Cow::Borrowed("d.mts"),
            Some(filename) if filename.ends_with(".d.cts") => Cow::Borrowed("d.cts"),
            _ => path
                .extension()
                // We assume the file extensions are case-insensitive.
                // Thus, we normalize the extension to lowercase.
                .map(|ext| ext.to_ascii_lowercase_cow())
                .ok_or(FileSourceError::MissingFileExtension)?,
        };

        Self::try_from_extension(extension)
    }
}

impl From<Language> for JsFileSource {
    fn from(value: Language) -> Self {
        match value {
            Language::JavaScript => JsFileSource::js_module(),
            Language::TypeScript { definition_file } => {
                if definition_file {
                    JsFileSource::d_ts()
                } else {
                    JsFileSource::ts()
                }
            }
        }
    }
}
