mod formatter;

use crate::configuration::javascript::formatter::{javascript_formatter, JavascriptFormatter};
use crate::configuration::merge::MergeWith;
use biome_deserialize::StringSet;
use biome_deserialize::StringSet;
use biome_js_formatter::context::{
    trailing_comma::TrailingComma, ArrowParentheses, QuoteProperties, QuoteStyle, Semicolons,
};
use bpaf::Bpaf;
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};

/// A set of options applied to the JavaScript files
#[derive(Default, Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Bpaf)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(default, deny_unknown_fields)]
pub struct JavascriptConfiguration {
    /// Formatting options
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(external(javascript_formatter), optional)]
    pub formatter: Option<JavascriptFormatter>,

    /// Parsing options
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(external(javascript_parser), optional)]
    pub parser: Option<JavascriptParser>,

    /// A list of global bindings that should be ignored by the analyzers
    ///
    /// If defined here, they should not emit diagnostics.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(hide)]
    pub globals: Option<StringSet>,
    //
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(external(javascript_organize_imports), optional)]
    pub organize_imports: Option<JavascriptOrganizeImports>,
}

impl MergeWith<JavascriptConfiguration> for JavascriptConfiguration {
    fn merge_with(&mut self, other: JavascriptConfiguration) {
        if let Some(other_formatter) = other.formatter {
            let formatter = self
                .formatter
                .get_or_insert_with(JavascriptFormatter::default);
            formatter.merge_with(other_formatter);
        }
    }
}

impl MergeWith<Option<JavascriptFormatter>> for JavascriptConfiguration {
    fn merge_with(&mut self, other: Option<JavascriptFormatter>) {
        if let Some(other_formatter) = other {
            let formatter = self
                .formatter
                .get_or_insert_with(JavascriptFormatter::default);
            formatter.merge_with(other_formatter);
        }
    }
}

impl JavascriptConfiguration {
    pub(crate) const KNOWN_KEYS: &'static [&'static str] =
        &["formatter", "globals", "organizeImports", "parser"];

    pub fn with_formatter() -> Self {
        Self {
            formatter: Some(JavascriptFormatter::default()),
            ..JavascriptConfiguration::default()
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize, Eq, PartialEq, Clone, Bpaf)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(default, deny_unknown_fields)]
pub struct JavascriptOrganizeImports {}

/// Options that changes how the JavaScript parser behaves
#[derive(Default, Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Bpaf)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct JavascriptParser {
    #[bpaf(hide)]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// It enables the experimental and unsafe parsing of parameter decorators
    ///
    /// These decorators belong to an old proposal, and they are subject to change.
    pub unsafe_parameter_decorators_enabled: Option<bool>,
}

impl JavascriptParser {
    pub(crate) const KNOWN_KEYS: &'static [&'static str] = &["unsafeParameterDecoratorsEnabled"];
}

impl MergeWith<JavascriptParser> for JavascriptParser {
    fn merge_with(&mut self, other: JavascriptParser) {
        if let Some(unsafe_parameter_decorators_enabled) = other.unsafe_parameter_decorators_enabled
        {
            self.unsafe_parameter_decorators_enabled = Some(unsafe_parameter_decorators_enabled);
        }
    }
}
