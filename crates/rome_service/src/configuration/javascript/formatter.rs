use crate::configuration::{formatter_configuration, FormatterConfiguration};
use crate::MergeWith;
use bpaf::Bpaf;
use rome_js_formatter::context::trailing_comma::TrailingComma;
use rome_js_formatter::context::{ArrowParentheses, QuoteProperties, QuoteStyle, Semicolons};
use serde::{Deserialize, Serialize};

/// Formatting options specific to the JavaScript files
#[derive(Default, Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Bpaf)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct JavascriptFormatter {
    /// Control the formatter for JavaScript (and its super languages) files.
    #[bpaf(
        long("--javascript-formatter-enabled"),
        argument("true|false"),
        optional
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<QuoteStyle>,

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

    /// Whether to add non-necessary parentheses to arrow functions. Defaults to "always".
    #[bpaf(external(formatter_configuration), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides: Option<FormatterOverride>,
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

struct FormatterOverride {}
