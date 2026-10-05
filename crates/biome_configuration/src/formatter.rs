use crate::bool::Bool;
use biome_deserialize_macros::{Deserializable, Merge};
use biome_formatter::{
    AttributePosition, BracketSameLine, BracketSpacing, DelimiterSpacing, Expand, IndentStyle,
    IndentWidth, LineEnding, LineWidth, TrailingNewline,
};
#[cfg(feature = "cli")]
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};

pub type FormatterEnabled = Bool<true>;
pub type UseEditorconfigEnabled = Bool<false>;
pub type FormatWithErrorsEnabled = Bool<false>;

/// Configures formatting for selected files.
#[derive(Clone, Deserializable, Debug, Default, Deserialize, Eq, PartialEq, Merge, Serialize)]
#[cfg_attr(feature = "cli", derive(Bpaf))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct FormatterConfiguration {
    /// Enables or disables the formatter. Defaults to `true`.
    #[cfg_attr(feature = "cli", bpaf(hide))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<FormatterEnabled>,

    /// Allows formatting files that contain syntax errors when set to `true`. Defaults to `false`.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("format-with-errors"), argument("true|false"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_with_errors: Option<FormatWithErrorsEnabled>,

    /// Uses tabs or spaces for indentation. Defaults to `tab`.
    #[cfg_attr(feature = "cli", bpaf(long("indent-style"), argument("tab|space")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_style: Option<IndentStyle>,

    /// Sets the indentation width. With space indentation, this is the number of spaces emitted per
    /// indentation level. With tab indentation, Biome emits one tab per level and uses this value as
    /// the tab's display width when calculating line length. Accepted values are `0` through `24`.
    /// Defaults to `2`.
    #[cfg_attr(feature = "cli", bpaf(long("indent-width"), argument("NUMBER")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent_width: Option<IndentWidth>,

    /// Selects the line ending. `auto` uses the platform convention. Defaults to `lf`.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("line-ending"), argument("lf|crlf|cr|auto"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_ending: Option<LineEnding>,

    /// Sets the preferred maximum line width used when deciding where to wrap code. Some content,
    /// such as long unbreakable strings, may still exceed this width. Accepted values are `1` through
    /// `320`. Defaults to `80`.
    #[cfg_attr(feature = "cli", bpaf(long("line-width"), argument("NUMBER")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_width: Option<LineWidth>,

    /// The attribute position style in HTML-like languages. Defaults to `auto`.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("attribute-position"), argument("multiline|auto"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_position: Option<AttributePosition>,

    /// Controls the placement of the closing bracket for multiline HTML and JSX opening tags. Biome
    /// places the bracket at the end of the last attribute line when enabled and on its own line when
    /// disabled. This also affects self-closing HTML elements, but self-closing JSX elements are
    /// unaffected. Defaults to `false`.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("bracket-same-line"), argument("true|false"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bracket_same_line: Option<BracketSameLine>,

    /// Controls spaces inside braces in supported single-line structures. The affected structures
    /// vary by language. Defaults to `true`.
    #[cfg_attr(feature = "cli", bpaf(long("bracket-spacing"), argument("true|false")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bracket_spacing: Option<BracketSpacing>,

    /// Controls spaces immediately inside supported delimiters when their content fits on one line.
    /// It doesn't add spaces before opening delimiters or inside empty delimiters.
    ///
    /// The affected delimiters vary by language. Defaults to `false`.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("delimiter-spacing"), argument("true|false"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delimiter_spacing: Option<DelimiterSpacing>,

    /// Controls whether arrays and objects are formatted on one line or multiple lines.
    ///
    /// `auto` formats objects on multiple lines if the first property has a newline, and arrays on
    /// one line if they fit.
    ///
    /// `always` formats arrays and objects on multiple lines.
    ///
    /// `never` formats arrays and objects on one line if they fit.
    ///
    /// Defaults to `auto`.
    ///
    /// When formatting `package.json`, Biome uses `always` unless configured otherwise.
    #[cfg_attr(feature = "cli", bpaf(long("expand"), argument("auto|always|never")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Expand>,

    /// Whether to add a trailing newline at the end of the file. Defaults to `true`. Disabling
    /// this option can cause compatibility problems with other tools.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("trailing-newline"), argument("true|false"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trailing_newline: Option<TrailingNewline>,

    /// Controls whether Biome loads formatter settings from an `.editorconfig` file. Biome reads
    /// `indent_style`, `indent_size`, `end_of_line`, and `insert_final_newline`. Settings in
    /// `biome.json` or `biome.jsonc` take precedence. Biome loads only one `.editorconfig`. Nested
    /// cascading configurations and the EditorConfig `root` property are not supported. Defaults to
    /// `false`.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("use-editorconfig"), argument("true|false"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_editorconfig: Option<UseEditorconfigEnabled>,

    /// A list of glob patterns selecting files to format. If omitted, all files selected by
    /// `files.includes` remain eligible for formatting. An empty list selects no files. This option
    /// can only narrow the files selected by `files.includes`.
    #[cfg_attr(feature = "cli", bpaf(pure(Default::default()), hide))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includes: Option<Vec<biome_glob::NormalizedGlob>>,
}

impl FormatterConfiguration {
    pub fn is_enabled(&self) -> bool {
        self.enabled.unwrap_or_default().into()
    }

    pub fn format_with_errors_resolved(&self) -> bool {
        self.format_with_errors.unwrap_or_default().into()
    }

    pub fn indent_style_resolved(&self) -> IndentStyle {
        self.indent_style.unwrap_or_default()
    }

    pub fn indent_width_resolved(&self) -> IndentWidth {
        self.indent_width.unwrap_or_default()
    }

    pub fn line_ending_resolved(&self) -> LineEnding {
        self.line_ending.unwrap_or_default()
    }

    pub fn line_width_resolved(&self) -> LineWidth {
        self.line_width.unwrap_or_default()
    }

    pub fn attribute_position_resolved(&self) -> AttributePosition {
        self.attribute_position.unwrap_or_default()
    }

    pub fn bracket_spacing_resolved(&self) -> BracketSpacing {
        self.bracket_spacing.unwrap_or_default()
    }

    pub fn delimiter_spacing_resolved(&self) -> DelimiterSpacing {
        self.delimiter_spacing.unwrap_or_default()
    }

    pub fn expand_resolved(&self) -> Expand {
        self.expand.unwrap_or_default()
    }

    pub fn trailing_newline_resolved(&self) -> TrailingNewline {
        self.trailing_newline.unwrap_or_default()
    }

    pub fn use_editorconfig_resolved(&self) -> bool {
        self.use_editorconfig.unwrap_or_default().into()
    }
}
