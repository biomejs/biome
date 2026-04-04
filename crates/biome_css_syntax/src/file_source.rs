use biome_rowan::FileSourceError;
use biome_string_case::StrLikeExtension;
use camino::Utf8Path;

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(
    Debug, Clone, Default, Copy, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum EmbeddingKind {
    /// styled-components or Emotion embedded CSS
    Styled,

    /// The CSS is embedded inside HTML-like files
    Html(EmbeddingHtmlKind),

    #[default]
    None,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(
    Debug, Clone, Default, Copy, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum EmbeddingHtmlKind {
    #[default]
    None,
    /// `.html` files
    Html,
    /// `.vue` files
    Vue,
    /// `.astro` files
    Astro,
    /// `.svelte` files
    Svelte,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(
    Debug, Clone, Default, Copy, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct CssFileSource {
    language: CssFileLanguage,
    variant: CssVariant,

    /// Used to mark if the CSS is embedded inside some particular files. This affects the parsing.
    /// For example, if inside a styled`` literal, a top-level declaration is allowed.
    embedding_kind: EmbeddingKind,
}

/// The language of the stylesheet.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(
    Debug, Clone, Default, Copy, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub enum CssFileLanguage {
    #[default]
    Css,
    Scss,
}

impl CssFileLanguage {
    pub const fn is_css(&self) -> bool {
        matches!(self, Self::Css)
    }

    pub const fn is_scss(&self) -> bool {
        matches!(self, Self::Scss)
    }
}

/// Extra CSS features enabled for the file.
///
/// Currently, Biome aims to be compatible with
/// the latest Recommendation level standards.
///
/// It also supports Tailwind CSS syntax additions, when the parser option is enabled.
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(
    Debug, Clone, Default, Copy, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub enum CssVariant {
    #[default]
    Standard,
    /// The file is a CSS module
    CssModules,
    /// The file belongs to tailwind
    TailwindCss,
}

impl CssFileSource {
    pub fn css() -> Self {
        Self {
            language: CssFileLanguage::Css,
            variant: CssVariant::Standard,
            embedding_kind: EmbeddingKind::None,
        }
    }

    pub fn scss() -> Self {
        Self {
            language: CssFileLanguage::Scss,
            variant: CssVariant::Standard,
            embedding_kind: EmbeddingKind::None,
        }
    }

    pub fn tailwind_css() -> Self {
        Self {
            language: CssFileLanguage::Css,
            variant: CssVariant::TailwindCss,
            embedding_kind: EmbeddingKind::None,
        }
    }

    pub fn new_css_modules() -> Self {
        Self {
            language: CssFileLanguage::Css,
            variant: CssVariant::CssModules,
            embedding_kind: EmbeddingKind::None,
        }
    }

    pub const fn with_embedding_kind(mut self, kind: EmbeddingKind) -> Self {
        self.embedding_kind = kind;
        self
    }

    pub const fn as_embedding_kind(&self) -> &EmbeddingKind {
        &self.embedding_kind
    }

    pub fn with_css_modules(mut self) -> Self {
        self.variant = CssVariant::CssModules;
        self
    }

    pub fn with_tailwind_directives(mut self) -> Self {
        self.variant = CssVariant::TailwindCss;
        self
    }

    pub fn is_css(&self) -> bool {
        self.language.is_css()
    }

    pub fn is_scss(&self) -> bool {
        self.language.is_scss()
    }

    pub fn is_css_modules(&self) -> bool {
        self.variant == CssVariant::CssModules
    }

    pub fn is_vue_embedded(&self) -> bool {
        matches!(
            self.embedding_kind,
            EmbeddingKind::Html(EmbeddingHtmlKind::Vue)
        )
    }

    pub fn is_tailwind_css(&self) -> bool {
        self.variant == CssVariant::TailwindCss
    }

    pub fn set_variant(&mut self, variant: CssVariant) {
        self.variant = variant;
    }

    /// Try to return the CSS file source corresponding to this file name from well-known files
    pub fn try_from_well_known(path: &Utf8Path) -> Result<Self, FileSourceError> {
        // Be careful with definition files, because `Path::extension()` only
        // returns the extension after the _last_ dot:
        let file_name = path.file_name().ok_or(FileSourceError::MissingFileName)?;
        if file_name.ends_with(".module.css") {
            return Self::try_from_extension("module.css");
        }

        match path.extension() {
            Some(extension) => Self::try_from_extension(extension),
            None => Err(FileSourceError::MissingFileExtension),
        }
    }

    /// Try to return the CSS file source corresponding to this file extension
    pub fn try_from_extension(extension: &str) -> Result<Self, FileSourceError> {
        // We assume the file extension is normalized to lowercase
        match extension {
            "css" => Ok(Self::css()),
            #[cfg(feature = "scss")]
            "scss" => Ok(Self::scss()),
            "module.css" => Ok(Self::new_css_modules()),
            _ => Err(FileSourceError::UnknownExtension),
        }
    }

    /// Try to return the CSS file source corresponding to this language ID
    ///
    /// See the [LSP spec] and [VS Code spec] for a list of language identifiers
    ///
    /// [LSP spec]: https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocumentItem
    /// [VS Code spec]: https://code.visualstudio.com/docs/languages/identifiers
    pub fn try_from_language_id(language_id: &str) -> Result<Self, FileSourceError> {
        match language_id {
            "css" => Ok(Self::css()),
            #[cfg(feature = "scss")]
            "scss" => Ok(Self::scss()),
            "tailwindcss" => Ok(Self::tailwind_css()),
            _ => Err(FileSourceError::UnknownLanguageId),
        }
    }
}

impl TryFrom<&Utf8Path> for CssFileSource {
    type Error = FileSourceError;

    fn try_from(path: &Utf8Path) -> Result<Self, Self::Error> {
        if let Ok(file_source) = Self::try_from_well_known(path) {
            return Ok(file_source);
        }

        let Some(extension) = path.extension() else {
            return Err(FileSourceError::MissingFileExtension);
        };
        // We assume the file extensions are case-insensitive
        // and we use the lowercase form of them for pattern matching
        Self::try_from_extension(&extension.to_ascii_lowercase_cow())
    }
}
