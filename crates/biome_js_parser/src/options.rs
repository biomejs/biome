/// Options to pass to the JavaScript parser
#[derive(Debug, Clone, Default)]
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
