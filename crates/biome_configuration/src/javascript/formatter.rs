use crate::bool::Bool;
use biome_deserialize_macros::{Deserializable, Merge};
use biome_formatter::{
    AttributePosition, BracketSameLine, BracketSpacing, DelimiterSpacing, Expand, IndentStyle,
    IndentWidth, LineEnding, LineWidth, QuoteStyle, TrailingNewline,
};
use biome_js_formatter::context::{
    ArrowParentheses, OperatorLinebreak, QuoteProperties, Semicolons,
    trailing_commas::TrailingCommas,
};
#[cfg(feature = "cli")]
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};

pub type JsFormatterEnabled = Bool<true>;
pub type BracketSameLineEnabled = Bool<false>;

/// Formatting options specific to the JavaScript files
#[derive(Clone, Default, Debug, Deserializable, Deserialize, Eq, Merge, PartialEq, Serialize)]
#[cfg_attr(feature = "cli", derive(Bpaf))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct JsFormatterConfiguration {
    /// Controls the formatter for JavaScript and languages that extend it.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("javascript-formatter-enabled"), argument("true|false"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<JsFormatterEnabled>,

    /// The type of quotes used in JSX. Defaults to `double`.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("jsx-quote-style"), argument("double|single"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jsx_quote_style: Option<QuoteStyle>,

    /// Controls when object properties are quoted. Defaults to `asNeeded` in configuration
    /// (`as-needed` on the CLI).
    #[cfg_attr(
        feature = "cli",
        bpaf(long("quote-properties"), argument("preserve|as-needed"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_properties: Option<QuoteProperties>,

    /// Prints trailing commas wherever possible in multiline comma-separated structures. Defaults
    /// to `all`.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("trailing-commas"), argument("all|es5|none"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trailing_commas: Option<TrailingCommas>,

    /// Prints semicolons after every statement or only where needed to avoid automatic semicolon
    /// insertion hazards. Defaults to `always`.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("semicolons"), argument("always|as-needed"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semicolons: Option<Semicolons>,

    /// Whether to add parentheses around arrow function parameters. Defaults to `always`.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("arrow-parentheses"), argument("always|as-needed"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arrow_parentheses: Option<ArrowParentheses>,

    /// Whether to hug the closing bracket of multiline HTML/JSX tags to the end of the last line,
    /// rather than being alone on the following line. If unset, inherits the global bracket
    /// placement setting.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("bracket-same-line"), argument("true|false"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bracket_same_line: Option<BracketSameLine>,

    /// The indent style applied to JavaScript and languages that extend it. If unset, inherits the
    /// global indentation style.
    #[cfg_attr(
        feature = "cli",
        bpaf(
            long("javascript-formatter-indent-style"),
            argument("tab|space"),
            optional
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_style: Option<IndentStyle>,

    /// The indentation width applied to JavaScript and languages that extend it. If unset,
    /// inherits the global indentation width.
    #[cfg_attr(
        feature = "cli",
        bpaf(
            long("javascript-formatter-indent-width"),
            argument("NUMBER"),
            optional
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_width: Option<IndentWidth>,

    /// The line ending applied to JavaScript and languages that extend it. If unset, inherits the
    /// global line ending.
    #[cfg_attr(
        feature = "cli",
        bpaf(
            long("javascript-formatter-line-ending"),
            argument("lf|crlf|cr|auto"),
            optional
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_ending: Option<LineEnding>,

    /// The maximum line width applied to JavaScript and languages that extend it. If unset,
    /// inherits the global line width.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("javascript-formatter-line-width"), argument("NUMBER"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_width: Option<LineWidth>,

    /// The type of quotes used in JavaScript code. Defaults to `double`.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("javascript-formatter-quote-style"), argument("double|single"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_style: Option<QuoteStyle>,

    // it's also a top-level configurable property.
    /// The attribute position style in JSX elements. If unset, inherits the global attribute
    /// position setting.
    #[cfg_attr(
        feature = "cli",
        bpaf(
            long("javascript-formatter-attribute-position"),
            argument("multiline|auto")
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_position: Option<AttributePosition>,

    // it's also a top-level configurable property.
    /// Whether to insert spaces inside braces in object literals. If unset, inherits the global
    /// bracket spacing setting.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("javascript-formatter-bracket-spacing"), argument("true|false"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bracket_spacing: Option<BracketSpacing>,

    // it's also a top-level configurable property.
    /// Controls spaces immediately inside supported JavaScript and TypeScript delimiters when their
    /// content fits on one line. It doesn't add spaces before opening delimiters or inside empty
    /// delimiters.
    ///
    /// It affects parentheses, square brackets, template interpolations, TypeScript angle brackets,
    /// JSX expression braces, and logical NOT. In operator chains, only the final operator receives
    /// a following space.
    ///
    /// If unset, inherits the global delimiter spacing setting.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("javascript-formatter-delimiter-spacing"), argument("true|false"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delimiter_spacing: Option<DelimiterSpacing>,

    /// Uses the same `auto`, `always`, and `never` behavior as the global expansion setting.
    ///
    /// If unset, inherits the global expansion setting.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("javascript-formatter-expand"), argument("auto|always|never"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Expand>,

    /// When breaking binary expressions into multiple lines, whether to break them before or after
    /// the binary operator. Defaults to `after`.
    #[cfg_attr(
        feature = "cli",
        bpaf(
            long("javascript-formatter-operator-linebreak"),
            argument("before|after")
        )
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_linebreak: Option<OperatorLinebreak>,

    /// Whether to add a trailing newline at the end of the file. If unset, inherits the global
    /// trailing newline setting.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("javascript-formatter-trailing-newline"), argument("true|false"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trailing_newline: Option<TrailingNewline>,
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
