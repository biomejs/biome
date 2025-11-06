use biome_rowan::FileSourceError;
use biome_string_case::StrLikeExtension;
use camino::Utf8Path;

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(
    Debug, Clone, Default, Copy, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct HtmlFileSource {
    variant: HtmlVariant,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(
    Debug, Clone, Default, Copy, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum HtmlTextExpressions {
    #[default]
    None,
    Single,
    Double,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
pub enum HtmlVariant {
    Standard(HtmlTextExpressions),
    /// Use this variant to parse an Astro file
    Astro,
    /// Use this variant to parse a Vue file
    Vue,
    /// Use this variant to parse a Svelte file
    Svelte,
}

impl Default for HtmlVariant {
    fn default() -> Self {
        Self::Standard(HtmlTextExpressions::None)
    }
}

impl HtmlFileSource {
    pub fn html() -> Self {
        Self {
            variant: HtmlVariant::default(),
        }
    }

    /// Returns `true` if the current file is `.html` and doesn't support
    /// any text expression capability
    pub const fn is_html(&self) -> bool {
        matches!(self.variant, HtmlVariant::Standard(_))
    }

    pub const fn is_vue(&self) -> bool {
        matches!(self.variant, HtmlVariant::Vue)
    }

    pub const fn is_svelte(&self) -> bool {
        matches!(self.variant, HtmlVariant::Svelte)
    }

    pub const fn is_astro(&self) -> bool {
        matches!(self.variant, HtmlVariant::Astro)
    }

    pub fn variant(&self) -> &HtmlVariant {
        &self.variant
    }

    pub fn text_expressions(&self) -> Option<&HtmlTextExpressions> {
        if let HtmlVariant::Standard(text_expressions) = &self.variant {
            Some(text_expressions)
        } else {
            None
        }
    }

    pub fn html_with_text_expressions() -> Self {
        Self {
            variant: HtmlVariant::Standard(HtmlTextExpressions::Double),
        }
    }

    pub fn astro() -> Self {
        Self {
            variant: HtmlVariant::Astro,
        }
    }

    pub fn vue() -> Self {
        Self {
            variant: HtmlVariant::Vue,
        }
    }
    pub fn svelte() -> Self {
        Self {
            variant: HtmlVariant::Svelte,
        }
    }

    /// Try to return the HTML file source corresponding to this file name from well-known files
    pub fn try_from_well_known(path: &Utf8Path) -> Result<Self, FileSourceError> {
        let Some(extension) = path.extension() else {
            return Err(FileSourceError::MissingFileExtension);
        };

        Self::try_from_extension(extension)
    }

    /// Try to return the HTML file source corresponding to this file extension
    pub fn try_from_extension(extension: &str) -> Result<Self, FileSourceError> {
        // We assume the file extension is normalized to lowercase
        match extension {
            "html" => Ok(Self::html()),
            "astro" => Ok(Self::astro()),
            "vue" => Ok(Self::vue()),
            "svelte" => Ok(Self::svelte()),
            _ => Err(FileSourceError::UnknownExtension),
        }
    }

    /// Try to return the HTML file source corresponding to this language ID
    ///
    /// See the [LSP spec] and [VS Code spec] for a list of language identifiers
    ///
    /// The language ID for Astro is registered by its [VS Code extension]
    ///
    /// [LSP spec]: https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocumentItem
    /// [VS Code spec]: https://code.visualstudio.com/docs/languages/identifiers
    /// [VS Code extension]: https://github.com/withastro/language-tools/blob/0503392b80765c8a1292ddc9c063a1187425c187/packages/vscode/package.json#L140
    pub fn try_from_language_id(language_id: &str) -> Result<Self, FileSourceError> {
        match language_id {
            "html" => Ok(Self::html()),
            "astro" => Ok(Self::astro()),
            "vuejs" | "vue" => Ok(Self::vue()),
            "svelte" => Ok(Self::svelte()),
            _ => Err(FileSourceError::UnknownLanguageId),
        }
    }
}

impl TryFrom<&Utf8Path> for HtmlFileSource {
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
