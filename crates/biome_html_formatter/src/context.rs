use std::{fmt, rc::Rc, str::FromStr};

use biome_deserialize_macros::{Deserializable, Merge};
use biome_formatter::{
    printer::PrinterOptions, AttributePosition, BracketSameLine, CstFormatContext, FormatContext,
    FormatOptions, IndentStyle, IndentWidth, LineEnding, LineWidth, TransformSourceMap,
};
use biome_html_syntax::{HtmlFileSource, HtmlLanguage};

use crate::comments::{FormatHtmlLeadingComment, HtmlCommentStyle, HtmlComments};

#[derive(Debug, Clone, Default)]
pub struct HtmlFormatOptions {
    /// The indent style.
    indent_style: IndentStyle,

    /// The indent width.
    indent_width: IndentWidth,

    /// The type of line ending.
    line_ending: LineEnding,

    /// What's the max width of a line. Defaults to 80.
    line_width: LineWidth,

    /// Attribute position style. By default auto.
    attribute_position: AttributePosition,

    /// Put the `>` of a multi-line HTML or JSX element at the end of the last line instead of being alone on the next line (does not apply to self closing elements).
    ///
    /// See: <https://prettier.io/docs/en/options.html#bracket-line>
    bracket_same_line: BracketSameLine,

    /// Whether to consider whitespace as significant. Default is `css`.
    ///
    /// Whitespace inside HTML elements can sometimes affect the rendering of the page.
    /// See:
    /// - <https://prettier.io/docs/en/options.html#html-whitespace-sensitivity>
    /// - <https://prettier.io/blog/2018/11/07/1.15.0#whitespace-sensitive-formatting>
    whitespace_sensitivity: WhitespaceSensitivity,

    /// Whether to indent the content of `<script>` and `<style>` tags. Default is `false`.
    indent_script_and_style: IndentScriptAndStyle,
}

impl HtmlFormatOptions {
    pub fn new(_file_source: HtmlFileSource) -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn with_indent_style(mut self, indent_style: IndentStyle) -> Self {
        self.indent_style = indent_style;
        self
    }

    pub fn with_indent_width(mut self, indent_width: IndentWidth) -> Self {
        self.indent_width = indent_width;
        self
    }

    pub fn with_line_ending(mut self, line_ending: LineEnding) -> Self {
        self.line_ending = line_ending;
        self
    }

    pub fn with_line_width(mut self, line_width: LineWidth) -> Self {
        self.line_width = line_width;
        self
    }

    pub fn with_attribute_position(mut self, attribute_position: AttributePosition) -> Self {
        self.attribute_position = attribute_position;
        self
    }

    pub fn with_bracket_same_line(mut self, bracket_same_line: BracketSameLine) -> Self {
        self.bracket_same_line = bracket_same_line;
        self
    }

    pub fn with_whitespace_sensitivity(
        mut self,
        whitespace_sensitivity: WhitespaceSensitivity,
    ) -> Self {
        self.whitespace_sensitivity = whitespace_sensitivity;
        self
    }

    pub fn with_indent_script_and_style(
        mut self,
        indent_script_and_style: IndentScriptAndStyle,
    ) -> Self {
        self.indent_script_and_style = indent_script_and_style;
        self
    }

    pub fn indent_style(&self) -> IndentStyle {
        self.indent_style
    }

    pub fn indent_width(&self) -> IndentWidth {
        self.indent_width
    }

    pub fn line_ending(&self) -> LineEnding {
        self.line_ending
    }

    pub fn line_width(&self) -> LineWidth {
        self.line_width
    }

    pub fn attribute_position(&self) -> AttributePosition {
        self.attribute_position
    }

    pub fn bracket_same_line(&self) -> BracketSameLine {
        self.bracket_same_line
    }

    pub fn whitespace_sensitivity(&self) -> WhitespaceSensitivity {
        self.whitespace_sensitivity
    }

    pub fn indent_script_and_style(&self) -> IndentScriptAndStyle {
        self.indent_script_and_style
    }

    pub fn set_indent_style(&mut self, indent_style: IndentStyle) {
        self.indent_style = indent_style;
    }

    pub fn set_indent_width(&mut self, indent_width: IndentWidth) {
        self.indent_width = indent_width;
    }

    pub fn set_line_ending(&mut self, line_ending: LineEnding) {
        self.line_ending = line_ending;
    }

    pub fn set_line_width(&mut self, line_width: LineWidth) {
        self.line_width = line_width;
    }

    pub fn set_attribute_position(&mut self, attribute_position: AttributePosition) {
        self.attribute_position = attribute_position;
    }

    pub fn set_bracket_same_line(&mut self, bracket_same_line: BracketSameLine) {
        self.bracket_same_line = bracket_same_line;
    }

    pub fn set_whitespace_sensitivity(&mut self, whitespace_sensitivity: WhitespaceSensitivity) {
        self.whitespace_sensitivity = whitespace_sensitivity;
    }

    pub fn set_indent_script_and_style(&mut self, indent_script_and_style: IndentScriptAndStyle) {
        self.indent_script_and_style = indent_script_and_style;
    }
}

impl fmt::Display for HtmlFormatOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Indent style: {}", self.indent_style)?;
        writeln!(f, "Indent width: {}", self.indent_width.value())?;
        writeln!(f, "Line ending: {}", self.line_ending)?;
        writeln!(f, "Line width: {}", self.line_width.value())?;
        writeln!(f, "Attribute Position: {}", self.attribute_position)?;
        writeln!(f, "Bracket same line: {}", self.bracket_same_line)?;
        writeln!(f, "Whitespace sensitivity: {}", self.whitespace_sensitivity)?;
        writeln!(
            f,
            "Indent script and style: {}",
            self.indent_script_and_style.value()
        )?;
        Ok(())
    }
}

impl FormatOptions for HtmlFormatOptions {
    fn indent_style(&self) -> IndentStyle {
        self.indent_style
    }

    fn indent_width(&self) -> IndentWidth {
        self.indent_width
    }

    fn line_ending(&self) -> LineEnding {
        self.line_ending
    }

    fn line_width(&self) -> LineWidth {
        self.line_width
    }

    fn as_print_options(&self) -> biome_formatter::prelude::PrinterOptions {
        PrinterOptions::from(self)
    }
}

/// Whitespace sensitivity for HTML formatting.
///
/// The following two cases won't produce the same output:
///
/// |                |      html      |    output    |
/// | -------------- | :------------: | :----------: |
/// | with spaces    | `1<b> 2 </b>3` | 1<b> 2 </b>3 |
/// | without spaces |  `1<b>2</b>3`  |  1<b>2</b>3  |
///
/// This happens because whitespace is significant in inline elements.
///
/// As a consequence of this, the formatter must format blocks that look like this (assume a small line width, <20):
/// ```html
/// <span>really long content</span>
/// ```
/// as this, where the content hugs the tags:
/// ```html
/// <span
///    >really long content</span
/// >
/// ```
///
/// Note that this is only necessary for inline elements. Block elements do not have this restriction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserializable, Merge)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum WhitespaceSensitivity {
    /// The formatter considers whitespace significant for elements that have an "inline" display style by default in
    /// browser's user agent style sheets.
    #[default]
    Css,
    /// Leading and trailing whitespace in content is considered significant for all elements.
    ///
    /// The formatter should leave at least one whitespace character if whitespace is present.
    /// Otherwise, if there is no whitespace, it should not add any after `>` or before `<`. In other words, if there's no whitespace, the text content should hug the tags.
    ///
    /// Example of text hugging the tags:
    /// ```html
    /// <b
    ///     >content</b
    /// >
    /// ```
    Strict,
    /// Whitespace is considered insignificant. The formatter is free to remove or add whitespace as it sees fit.
    Ignore,
}

impl fmt::Display for WhitespaceSensitivity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Css => std::write!(f, "css"),
            Self::Strict => std::write!(f, "strict"),
            Self::Ignore => std::write!(f, "ignore"),
        }
    }
}

impl FromStr for WhitespaceSensitivity {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "css" => Ok(Self::Css),
            "strict" => Ok(Self::Strict),
            "ignore" => Ok(Self::Ignore),
            _ => Err("Value not supported for WhitespaceSensitivity. Supported values are 'css', 'strict' and 'ignore'."),
        }
    }
}

impl WhitespaceSensitivity {
    pub const fn is_css(&self) -> bool {
        matches!(self, Self::Css)
    }

    pub const fn is_strict(&self) -> bool {
        matches!(self, Self::Strict)
    }
}

/// Whether to indent the content of `<script>` and `<style>` tags for HTML-ish templating languages (Vue, Svelte, etc.).
///
/// When true, the content of `<script>` and `<style>` tags will be indented one level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Deserializable, Merge)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct IndentScriptAndStyle(bool);

impl IndentScriptAndStyle {
    pub fn new(value: bool) -> Self {
        Self(value)
    }

    pub fn value(&self) -> bool {
        self.0
    }
}

impl From<bool> for IndentScriptAndStyle {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl FromStr for IndentScriptAndStyle {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match bool::from_str(s) {
            Ok(value) => Ok(Self(value)),
            Err(_) => Err(
                "Value not supported for IndentScriptAndStyle. Supported values are 'true' and 'false'.",
            ),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HtmlFormatContext {
    options: HtmlFormatOptions,

    /// The comments of the nodes and tokens in the document.
    comments: Rc<HtmlComments>,

    source_map: Option<TransformSourceMap>,
}

impl HtmlFormatContext {
    pub fn new(options: HtmlFormatOptions, comments: HtmlComments) -> Self {
        Self {
            options,
            comments: Rc::new(comments),
            source_map: None,
        }
    }

    pub fn with_source_map(mut self, source_map: Option<TransformSourceMap>) -> Self {
        self.source_map = source_map;
        self
    }
}

impl FormatContext for HtmlFormatContext {
    type Options = HtmlFormatOptions;

    fn options(&self) -> &Self::Options {
        &self.options
    }

    fn source_map(&self) -> Option<&TransformSourceMap> {
        self.source_map.as_ref()
    }
}

impl CstFormatContext for HtmlFormatContext {
    type Language = HtmlLanguage;
    type Style = HtmlCommentStyle;
    type CommentRule = FormatHtmlLeadingComment;

    fn comments(&self) -> &HtmlComments {
        &self.comments
    }
}
