use crate::configuration::PlainIndentStyle;
use crate::configuration::{deserialize_line_width, serialize_line_width};
use biome_deserialize_macros::{Deserializable, Merge, Partial};
use biome_formatter::{AttributePosition, LineEnding, LineWidth, QuoteStyle};
use biome_js_formatter::context::trailing_comma::TrailingComma;
use biome_js_formatter::context::{ArrowParentheses, QuoteProperties, Semicolons};
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};

/// Formatting options specific to the JavaScript files
#[derive(Clone, Debug, Deserialize, Eq, Partial, PartialEq, Serialize)]
#[partial(derive(Bpaf, Clone, Deserializable, Eq, Merge, PartialEq))]
#[partial(cfg_attr(feature = "schema", derive(schemars::JsonSchema)))]
#[partial(serde(rename_all = "camelCase", default, deny_unknown_fields))]
pub struct JavascriptFormatter {
    /// The type of quotes used in JSX. Defaults to double.
    #[partial(bpaf(long("jsx-quote-style"), argument("double|single"), optional))]
    pub jsx_quote_style: QuoteStyle,

    /// When properties in objects are quoted. Defaults to asNeeded.
    #[partial(bpaf(long("quote-properties"), argument("preserve|as-needed"), optional))]
    pub quote_properties: QuoteProperties,

    /// Print trailing commas wherever possible in multi-line comma-separated syntactic structures. Defaults to "all".
    #[partial(bpaf(long("trailing-comma"), argument("all|es5|none"), optional))]
    pub trailing_comma: TrailingComma,

    /// Whether the formatter prints semicolons for all statements or only in for statements where it is necessary because of ASI.
    #[partial(bpaf(long("semicolons"), argument("always|as-needed"), optional))]
    pub semicolons: Semicolons,

    /// Whether to add non-necessary parentheses to arrow functions. Defaults to "always".
    #[partial(bpaf(long("arrow-parentheses"), argument("always|as-needed"), optional))]
    pub arrow_parentheses: ArrowParentheses,

    /// Whether to insert spaces around brackets in object literals. Defaults to true.
    #[partial(bpaf(long("bracket-spacing"), argument("true|false"), optional))]
    pub bracket_spacing: bool,

    /// Whether to hug the closing bracket of multiline HTML/JSX tags to the end of the last line, rather than being alone on the following line. Defaults to false.
    #[partial(bpaf(long("bracket-same-line"), argument("true|false"), optional))]
    pub bracket_same_line: bool,

    /// Control the formatter for JavaScript (and its super languages) files.
    #[partial(bpaf(long("javascript-formatter-enabled"), argument("true|false"), optional))]
    pub enabled: bool,

    /// The indent style applied to JavaScript (and its super languages) files.
    #[partial(bpaf(
        long("javascript-formatter-indent-style"),
        argument("tab|space"),
        optional
    ))]
    pub indent_style: Option<PlainIndentStyle>,

    /// The size of the indentation applied to JavaScript (and its super languages) files. Default to 2.
    #[partial(deserializable(deprecated(use_instead = "javascript.formatter.indentWidth")))]
    #[partial(bpaf(long("javascript-formatter-indent-size"), argument("NUMBER"), optional))]
    pub indent_size: Option<u8>,

    /// The size of the indentation applied to JavaScript (and its super languages) files. Default to 2.
    #[partial(bpaf(
        long("javascript-formatter-indent-width"),
        argument("NUMBER"),
        optional
    ))]
    pub indent_width: Option<u8>,

    /// The type of line ending applied to JavaScript (and its super languages) files.
    #[partial(bpaf(
        long("javascript-formatter-line-ending"),
        argument("lf|crlf|cr"),
        optional
    ))]
    pub line_ending: Option<LineEnding>,

    /// What's the max width of a line applied to JavaScript (and its super languages) files. Defaults to 80.
    #[partial(serde(
        deserialize_with = "deserialize_line_width",
        serialize_with = "serialize_line_width"
    ))]
    #[partial(bpaf(long("javascript-formatter-line-width"), argument("NUMBER"), optional))]
    pub line_width: Option<LineWidth>,

    // TODO: Rename the argument to `javascript-formatter-quote-style` once
    // it's also a top-level configurable property.
    /// The type of quotes used in JavaScript code. Defaults to double.
    #[partial(bpaf(long("quote-style"), argument("double|single"), optional))]
    pub quote_style: QuoteStyle,

    // it's also a top-level configurable property.
    /// The attribute position style in JavaScript code. Defaults to auto.
    #[partial(bpaf(
        long("javascript-attribute-position"),
        argument("multiline|auto"),
        optional
    ))]
    pub attribute_position: AttributePosition,
}

impl Default for JavascriptFormatter {
    fn default() -> Self {
        Self {
            enabled: true,
            jsx_quote_style: Default::default(),
            quote_properties: Default::default(),
            trailing_comma: Default::default(),
            semicolons: Default::default(),
            arrow_parentheses: Default::default(),
            bracket_spacing: true,
            bracket_same_line: Default::default(),
            indent_style: Default::default(),
            indent_size: Default::default(),
            indent_width: Default::default(),
            line_ending: Default::default(),
            line_width: Default::default(),
            quote_style: Default::default(),
            attribute_position: Default::default(),
        }
    }
}
