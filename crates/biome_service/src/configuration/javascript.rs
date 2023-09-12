use crate::configuration::merge::MergeWith;
use biome_deserialize::StringSet;
use biome_js_formatter::context::{
    trailing_comma::TrailingComma, ArrowParentheses, QuoteProperties, QuoteStyle, Semicolons,
};
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

/// Formatting options specific to the JavaScript files
#[derive(Default, Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Bpaf)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct JavascriptFormatter {
    /// The type of quotes used in JavaScript code. Defaults to double.
    #[bpaf(long("quote-style"), argument("double|single"), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_style: Option<QuoteStyle>,
    /// The type of quotes used in JSX. Defaults to double.
    #[bpaf(long("jsx-quote-style"), argument("double|single"), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jsx_quote_style: Option<QuoteStyle>,
    /// When properties in objects are quoted. Defaults to asNeeded.
    #[bpaf(long("quote-properties"), argument("preserve|as-needed"), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_properties: Option<QuoteProperties>,
    /// Print trailing commas wherever possible in multi-line comma-separated syntactic structures. Defaults to "all".
    #[bpaf(long("trailing-comma"), argument("all|es5|none"), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trailing_comma: Option<TrailingComma>,
    /// Whether the formatter prints semicolons for all statements or only in for statements where it is necessary because of ASI.
    #[bpaf(long("semicolons"), argument("always|as-needed"), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semicolons: Option<Semicolons>,
    /// Whether to add non-necessary parentheses to arrow functions. Defaults to "always".
    #[bpaf(long("arrow-parentheses"), argument("always|as-needed"), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arrow_parentheses: Option<ArrowParentheses>,
}

impl JavascriptFormatter {
    pub(crate) const KNOWN_KEYS: &'static [&'static str] = &[
        "quoteStyle",
        "jsxQuoteStyle",
        "quoteProperties",
        "trailingComma",
        "semicolons",
        "arrowParentheses",
    ];
}

impl MergeWith<JavascriptFormatter> for JavascriptFormatter {
    fn merge_with(&mut self, other: JavascriptFormatter) {
        if let Some(arrow_parentheses) = other.arrow_parentheses {
            self.arrow_parentheses = Some(arrow_parentheses);
        }
        if let Some(quote_properties) = other.quote_properties {
            self.quote_properties = Some(quote_properties);
        }
        if let Some(quote_style) = other.quote_style {
            self.quote_style = Some(quote_style);
        }
        if let Some(jsx_quote_style) = other.jsx_quote_style {
            self.jsx_quote_style = Some(jsx_quote_style);
        }
        if let Some(semicolons) = other.semicolons {
            self.semicolons = Some(semicolons);
        }
        if let Some(trailing_comma) = other.trailing_comma {
            self.trailing_comma = Some(trailing_comma);
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
