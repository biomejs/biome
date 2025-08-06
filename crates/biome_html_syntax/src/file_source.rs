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

impl HtmlFileSource {
    pub const fn is_astro(&self) -> bool {
        matches!(self.variant, HtmlVariant::Astro)
    }

    /// Returns a reference to the underlying HTML variant for this file source.
    pub fn variant(&self) -> &HtmlVariant {
        &self.variant
    }

    /// Returns the text expression capability if the file source is standard HTML.
    ///
    /// Returns `Some(&HtmlTextExpressions)` if the variant is `Standard`, otherwise returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// let html = HtmlFileSource::html_with_text_expressions();
    /// assert_eq!(html.text_expressions(), Some(&HtmlTextExpressions::Double));
    ///
    /// let astro = HtmlFileSource::astro();
    /// assert_eq!(astro.text_expressions(), None);
    /// ```
    pub fn text_expressions(&self) -> Option<&HtmlTextExpressions> {
        if let HtmlVariant::Standard(text_expressions) = &self.variant {
            Some(text_expressions)
        } else {
            None
        }
    }
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
    /// Returns the default HTML variant with no text expression support.
    ///
    /// The default is `Standard(HtmlTextExpressions::None)`.
    ///
    /// # Examples
    ///
    /// ```
    /// let variant = HtmlVariant::default();
    /// assert!(matches!(variant, HtmlVariant::Standard(HtmlTextExpressions::None)));
    /// ```
    fn default() -> Self {
        Self::Standard(HtmlTextExpressions::None)
    }
}

impl HtmlFileSource {
    /// Creates a `HtmlFileSource` representing a standard HTML file with no text expression support.
    ///
    /// # Examples
    ///
    /// ```
    /// let source = HtmlFileSource::html();
    /// assert!(source.is_html());
    /// ```
    pub fn html() -> Self {
        Self {
            variant: HtmlVariant::default(),
        }
    }

    /// Checks if the file source is a standard HTML file without text expression support.
    ///
    /// Returns `true` if the variant is `Standard` with `HtmlTextExpressions::None`.
    ///
    /// # Examples
    ///
    /// ```
    /// let html = HtmlFileSource::html();
    /// assert!(html.is_html());
    ///
    /// let html_with_expr = HtmlFileSource::html_with_text_expressions();
    /// assert!(!html_with_expr.is_html());
    /// ```
    pub fn is_html(&self) -> bool {
        self.variant == HtmlVariant::default()
    }

    /// Creates an HTML file source with support for double text expressions.
    ///
    /// Returns a `HtmlFileSource` representing a standard HTML variant that enables double text expression capability.
    ///
    /// # Examples
    ///
    /// ```
    /// let source = HtmlFileSource::html_with_text_expressions();
    /// assert_eq!(source.text_expressions(), Some(&HtmlTextExpressions::Double));
    /// ```
    pub fn html_with_text_expressions() -> Self {
        Self {
            variant: HtmlVariant::Standard(HtmlTextExpressions::Double),
        }
    }

    /// Creates an `HtmlFileSource` representing an Astro file.
    ///
    /// # Examples
    ///
    /// ```
    /// let source = HtmlFileSource::astro();
    /// assert!(source.is_astro());
    /// ```
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
    pub fn try_from_well_known(_: &Utf8Path) -> Result<Self, FileSourceError> {
        // TODO: to be implemented
        Err(FileSourceError::UnknownFileName)
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
            "vue" => Ok(Self::vue()),
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
