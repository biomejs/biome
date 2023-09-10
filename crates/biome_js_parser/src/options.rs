/// Options to pass to the JavaScript parser
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct JsParserOptions {
    /// Whether the parsing of the class parameter decorators should happen.
    ///
    /// This parameter decorators belong to the old language proposal.
    pub parse_class_parameter_decorators: bool,
}

impl JsParserOptions {
    pub fn with_parse_class_parameter_decorators(mut self) -> Self {
        self.parse_class_parameter_decorators = true;
        self
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
