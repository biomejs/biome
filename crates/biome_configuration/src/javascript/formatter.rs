use crate::bool::Bool;
use biome_deserialize_macros::{Deserializable, Merge};
use biome_formatter::{
    AttributePosition, BracketSameLine, BracketSpacing, IndentStyle, IndentWidth, LineEnding,
    LineWidth, ObjectWrap, QuoteStyle,
};
use biome_js_formatter::context::{
    trailing_commas::TrailingCommas, ArrowParentheses, QuoteProperties, Semicolons,
};
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};

pub type JsFormatterEnabled = Bool<true>;
pub type BracketSameLineEnabled = Bool<false>;

/// Formatting options specific to the JavaScript files
#[derive(
    Bpaf, Clone, Default, Debug, Deserializable, Deserialize, Eq, Merge, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct JsFormatterConfiguration {
    /// Control the formatter for JavaScript (and its super languages) files.
    #[bpaf(long("javascript-formatter-enabled"), argument("true|false"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<JsFormatterEnabled>,

    /// The type of quotes used in JSX. Defaults to double.
    #[bpaf(long("jsx-quote-style"), argument("double|single"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jsx_quote_style: Option<QuoteStyle>,

    /// When properties in objects are quoted. Defaults to asNeeded.
    #[bpaf(long("quote-properties"), argument("preserve|as-needed"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_properties: Option<QuoteProperties>,

    /// Print trailing commas wherever possible in multi-line comma-separated syntactic structures. Defaults to "all".
    #[bpaf(long("trailing-commas"), argument("all|es5|none"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trailing_commas: Option<TrailingCommas>,

    /// Whether the formatter prints semicolons for all statements or only in for statements where it is necessary because of ASI.
    #[bpaf(long("semicolons"), argument("always|as-needed"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semicolons: Option<Semicolons>,

    /// Whether to add non-necessary parentheses to arrow functions. Defaults to "always".
    #[bpaf(long("arrow-parentheses"), argument("always|as-needed"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arrow_parentheses: Option<ArrowParentheses>,

    /// Whether to hug the closing bracket of multiline HTML/JSX tags to the end of the last line, rather than being alone on the following line. Defaults to false.
    #[bpaf(long("bracket-same-line"), argument("true|false"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bracket_same_line: Option<BracketSameLine>,

    /// The indent style applied to JavaScript (and its super languages) files.
    #[bpaf(
        long("javascript-formatter-indent-style"),
        argument("tab|space"),
        optional
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_style: Option<IndentStyle>,

    /// The size of the indentation applied to JavaScript (and its super languages) files. Default to 2.
    #[bpaf(
        long("javascript-formatter-indent-width"),
        argument("NUMBER"),
        optional
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_width: Option<IndentWidth>,

    /// The type of line ending applied to JavaScript (and its super languages) files.
    #[bpaf(
        long("javascript-formatter-line-ending"),
        argument("lf|crlf|cr"),
        optional
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_ending: Option<LineEnding>,

    /// What's the max width of a line applied to JavaScript (and its super languages) files. Defaults to 80.
    #[bpaf(long("javascript-formatter-line-width"), argument("NUMBER"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_width: Option<LineWidth>,

    /// The type of quotes used in JavaScript code. Defaults to double.
    #[bpaf(long("javascript-formatter-quote-style"), argument("double|single"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_style: Option<QuoteStyle>,

    // it's also a top-level configurable property.
    /// The attribute position style in JSX elements. Defaults to auto.
    #[bpaf(
        long("javascript-formatter-attribute-position"),
        argument("multiline|auto")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_position: Option<AttributePosition>,

    // it's also a top-level configurable property.
    /// Whether to insert spaces around brackets in object literals. Defaults to true.
    #[bpaf(long("javascript-formatter-bracket-spacing"), argument("true|false"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bracket_spacing: Option<BracketSpacing>,

    /// Whether to enforce collapsing object literals when possible. Defaults to preserve.
    #[bpaf(
        long("javascript-formatter-object-wrap"),
        argument("preserve|collapse")
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_wrap: Option<ObjectWrap>,
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

    pub fn object_wrap_resolved(&self) -> ObjectWrap {
        self.object_wrap.unwrap_or_default()
    }
}
