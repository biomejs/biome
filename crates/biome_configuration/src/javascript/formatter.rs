use crate::bool::Bool;
use biome_deserialize_macros::{Deserializable, Merge};
use biome_formatter::{
    AttributePosition, BracketSameLine, BracketSpacing, Expand, IndentStyle, IndentWidth,
    LineEnding, LineWidth, QuoteStyle,
};
use biome_js_formatter::context::{
    ArrowParentheses, OperatorLinebreak, QuoteProperties, Semicolons,
    trailing_commas::TrailingCommas,
};
use clap::Args;
use serde::{Deserialize, Serialize};

pub type JsFormatterEnabled = Bool<true>;
pub type BracketSameLineEnabled = Bool<false>;

/// Formatting options specific to the JavaScript files
#[derive(
    Args, Clone, Default, Debug, Deserializable, Deserialize, Eq, Merge, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
#[group(skip)]
pub struct JsFormatterConfiguration {
    /// Control the formatter for JavaScript (and its super languages) files.
    #[arg(long = "javascript-formatter-enabled", value_name = "true|false")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<JsFormatterEnabled>,

    /// The type of quotes used in JSX. Defaults to double.
    #[arg(long = "jsx-quote-style", value_name = "double|single")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jsx_quote_style: Option<QuoteStyle>,

    /// When properties in objects are quoted. Defaults to asNeeded.
    #[arg(long = "quote-properties", value_name = "preserve|as-needed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_properties: Option<QuoteProperties>,

    /// Print trailing commas wherever possible in multi-line comma-separated syntactic structures. Defaults to "all".
    #[arg(long = "trailing-commas", value_name = "all|es5|none")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trailing_commas: Option<TrailingCommas>,

    /// Whether the formatter prints semicolons for all statements or only in for statements where it is necessary because of ASI.
    #[arg(long = "semicolons", value_name = "always|as-needed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semicolons: Option<Semicolons>,

    /// Whether to add non-necessary parentheses to arrow functions. Defaults to "always".
    #[arg(long = "arrow-parentheses", value_name = "always|as-needed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arrow_parentheses: Option<ArrowParentheses>,

    /// Whether to hug the closing bracket of multiline HTML/JSX tags to the end of the last line, rather than being alone on the following line. Defaults to false.
    #[arg(long = "bracket-same-line", value_name = "true|false")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bracket_same_line: Option<BracketSameLine>,

    /// The indent style applied to JavaScript (and its super languages) files.
    #[arg(long = "javascript-formatter-indent-style", value_name = "tab|space")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_style: Option<IndentStyle>,

    /// The size of the indentation applied to JavaScript (and its super languages) files. Default to 2.
    #[arg(long = "javascript-formatter-indent-width", value_name = "NUMBER")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_width: Option<IndentWidth>,

    /// The type of line ending applied to JavaScript (and its super languages) files. `auto` uses CRLF on Windows and LF on other platforms.
    #[arg(
        long = "javascript-formatter-line-ending",
        value_name = "lf|crlf|cr|auto"
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_ending: Option<LineEnding>,

    /// What's the max width of a line applied to JavaScript (and its super languages) files. Defaults to 80.
    #[arg(long = "javascript-formatter-line-width", value_name = "NUMBER")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_width: Option<LineWidth>,

    /// The type of quotes used in JavaScript code. Defaults to double.
    #[arg(
        long = "javascript-formatter-quote-style",
        value_name = "double|single"
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_style: Option<QuoteStyle>,

    // it's also a top-level configurable property.
    /// The attribute position style in JSX elements. Defaults to auto.
    #[arg(
        long = "javascript-formatter-attribute-position",
        value_name = "multiline|auto"
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_position: Option<AttributePosition>,

    // it's also a top-level configurable property.
    /// Whether to insert spaces around brackets in object literals. Defaults to true.
    #[arg(
        long = "javascript-formatter-bracket-spacing",
        value_name = "true|false"
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bracket_spacing: Option<BracketSpacing>,

    /// Whether to expand arrays and objects on multiple lines.
    /// When set to `auto`, object literals are formatted on multiple lines if the first property has a newline,
    /// and array literals are formatted on a single line if it fits in the line.
    /// When set to `always`, these literals are formatted on multiple lines, regardless of length of the list.
    /// When set to `never`, these literals are formatted on a single line if it fits in the line.
    /// When formatting `package.json`, Biome will use `always` unless configured otherwise. Defaults to "auto".
    #[arg(long = "javascript-formatter-expand", value_name = "auto|always|never")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Expand>,

    /// When breaking binary expressions into multiple lines, whether to break them before or after the binary operator. Defaults to "after".
    #[arg(
        long = "javascript-formatter-operator-linebreak",
        value_name = "before|after"
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_linebreak: Option<OperatorLinebreak>,
}

impl JsFormatterConfiguration {
    pub fn is_enabled(&self) -> bool {
        self.enabled.unwrap_or_default().into()
    }

    pub fn jsx_quote_style_resolved(&self) -> QuoteStyle {
        self.jsx_quote_style.unwrap_or_default()
    }

    pub fn quote_properties_resolved(&self) -> QuoteProperties {
        self.quote_properties.unwrap_or_default()
    }

    pub fn trailing_commas_resolved(&self) -> TrailingCommas {
        self.trailing_commas.unwrap_or_default()
    }

    pub fn semicolons_resolved(&self) -> Semicolons {
        self.semicolons.unwrap_or_default()
    }

    pub fn arrow_parentheses_resolved(&self) -> ArrowParentheses {
        self.arrow_parentheses.unwrap_or_default()
    }

    pub fn bracket_same_line_resolved(&self) -> BracketSameLine {
        self.bracket_same_line.unwrap_or_default()
    }

    pub fn quote_style_resolved(&self) -> QuoteStyle {
        self.quote_style.unwrap_or_default()
    }

    pub fn expand_resolved(&self) -> Expand {
        self.expand.unwrap_or_default()
    }

    pub fn operator_linebreak_resolved(&self) -> OperatorLinebreak {
        self.operator_linebreak.unwrap_or_default()
    }
}
