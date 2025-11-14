use biome_js_syntax::JsFileSource;

/// Options to pass to the JavaScript parser
#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct JsParserOptions {
    /// Enables parsing of Grit metavariables.
    /// Defaults to `false`.
    #[cfg_attr(feature = "serde", serde(default))]
    pub grit_metavariables: bool,

    /// Whether the parsing of the class parameter decorators should happen.
    ///
    /// This parameter decorators belong to the old language proposal.
    #[cfg_attr(feature = "serde", serde(default))]
    pub parse_class_parameter_decorators: bool,
}

impl JsParserOptions {
    /// Enables parsing of Grit metavariables.
    pub fn with_metavariables(mut self) -> Self {
        self.grit_metavariables = true;
        self
    }

    pub fn with_parse_class_parameter_decorators(mut self) -> Self {
        self.parse_class_parameter_decorators = true;
        self
    }

    pub fn should_parse_metavariables(&self) -> bool {
        self.grit_metavariables
    }

    /// Should parse parameter decorators inside classes, e.g.:
    ///
    /// ```js
    /// class C {
    ///   post(@Param() name) {}
    /// }
    /// ```
    pub fn should_parse_parameter_decorators(&self) -> bool {
        self.parse_class_parameter_decorators
    }
}

impl From<&JsFileSource> for JsParserOptions {
    /// Derive parser options from the file source type.
    ///
    /// This allows parser configuration to be automatically determined based on
    /// file characteristics (language, variant, embedding kind, etc.) rather than
    /// requiring explicit option passing.
    ///
    /// Currently returns default options for all file types, but provides a
    /// centralized place to configure file-type-specific parsing behavior in the future.
    /// For example, Glimmer template parsing (.gjs/.gts files) is handled at the
    /// lexer level by checking the embedding kind.
    fn from(file_source: &JsFileSource) -> Self {
        let mut options = Self::default();

        // File-type-specific options could be configured here based on:
        // - file_source.language() - JavaScript vs TypeScript
        // - file_source.variant() - Standard vs JSX
        // - file_source.module_kind() - Script vs Module
        // - file_source.as_embedding_kind() - Astro, Vue, Svelte, Glimmer, etc.
        //
        // For now, Glimmer-specific behavior (template lexing) is handled in the lexer
        // by checking file_source.as_embedding_kind().is_glimmer()

        options
    }
}
