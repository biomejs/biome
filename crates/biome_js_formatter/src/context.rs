pub mod trailing_commas;

use crate::comments::{FormatJsLeadingComment, JsCommentStyle, JsComments};
use biome_deserialize_macros::{Deserializable, Merge};
use biome_formatter::printer::PrinterOptions;
use biome_formatter::{
    AttributePosition, BracketSameLine, BracketSpacing, CstFormatContext, FormatContext,
    FormatElement, FormatOptions, IndentStyle, IndentWidth, LineEnding, LineWidth, ObjectWrap,
    QuoteStyle, TransformSourceMap,
};
use biome_js_syntax::{AnyJsFunctionBody, JsFileSource, JsLanguage};
use std::fmt;
use std::fmt::Debug;
use std::rc::Rc;
use std::str::FromStr;
pub use trailing_commas::TrailingCommas;

#[derive(Debug, Clone)]
pub struct JsFormatContext {
    options: JsFormatOptions,

    /// The comments of the nodes and tokens in the program.
    comments: Rc<JsComments>,

    /// Stores the formatted content of one function body.
    ///
    /// Used during formatting of call arguments where function expressions and arrow function expressions
    /// are formatted a second time if they are the first or last call argument.
    ///
    /// Caching the body in the call arguments formatting is important. It minimises the cases
    /// where the algorithm is quadratic, in case the function or arrow expression contains another
    /// call expression with a function or call expression as first or last argument.
    ///
    /// It's sufficient to only store a single cached body to cover the vast majority of cases
    /// (there's no exception in any of our tests nor benchmark tests). The only case not covered is when
    /// a parameter has an initializer that contains a call expression:
    ///
    /// ```javascript
    ///  test((
    ///    problematic = test(() => body)
    ///  ) => {});
    ///  ```
    ///
    /// This should be rare enough for us not to care about it.
    cached_function_body: Option<(AnyJsFunctionBody, FormatElement)>,

    source_map: Option<TransformSourceMap>,
}

impl JsFormatContext {
    pub fn new(options: JsFormatOptions, comments: JsComments) -> Self {
        Self {
            options,
            comments: Rc::new(comments),
            cached_function_body: None,
            source_map: None,
        }
    }

    /// Returns the formatted content for the passed function body if it is cached or `None` if the currently
    /// cached content belongs to another function body or the cache is empty.
    ///
    /// See [JsFormatContext::cached_function_body] for more in depth documentation.
    pub(crate) fn get_cached_function_body(
        &self,
        body: &AnyJsFunctionBody,
    ) -> Option<FormatElement> {
        self.cached_function_body
            .as_ref()
            .and_then(|(expected_body, formatted)| {
                if expected_body == body {
                    Some(formatted.clone())
                } else {
                    None
                }
            })
    }

    /// Sets the currently cached formatted function body.
    ///
    /// See [JsFormatContext::cached_function_body] for more in depth documentation.
    pub(crate) fn set_cached_function_body(
        &mut self,
        body: &AnyJsFunctionBody,
        formatted: FormatElement,
    ) {
        self.cached_function_body = Some((body.clone(), formatted))
    }

    pub fn with_source_map(mut self, source_map: Option<TransformSourceMap>) -> Self {
        self.source_map = source_map;
        self
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub struct TabWidth(u8);

impl From<u8> for TabWidth {
    fn from(value: u8) -> Self {
        TabWidth(value)
    }
}

impl From<TabWidth> for u8 {
    fn from(width: TabWidth) -> Self {
        width.0
    }
}

impl FormatContext for JsFormatContext {
    type Options = JsFormatOptions;

    fn options(&self) -> &Self::Options {
        &self.options
    }

    fn source_map(&self) -> Option<&TransformSourceMap> {
        self.source_map.as_ref()
    }
}

impl CstFormatContext for JsFormatContext {
    type Language = JsLanguage;
    type Style = JsCommentStyle;
    type CommentRule = FormatJsLeadingComment;

    fn comments(&self) -> &JsComments {
        &self.comments
    }
}

#[derive(Debug, Default, Clone)]
pub struct JsFormatOptions {
    /// The indent style.
    indent_style: IndentStyle,

    /// The indent width.
    indent_width: IndentWidth,

    /// The type of line ending.
    line_ending: LineEnding,

    /// What's the max width of a line. Defaults to 80.
    line_width: LineWidth,

    /// The style for quotes. Defaults to double.
    quote_style: QuoteStyle,

    /// The style for JSX quotes. Defaults to double.
    jsx_quote_style: QuoteStyle,

    /// When properties in objects are quoted. Defaults to as-needed.
    quote_properties: QuoteProperties,

    /// Print trailing commas wherever possible in multi-line comma-separated syntactic structures. Defaults to "all".
    trailing_commas: TrailingCommas,

    /// Whether the formatter prints semicolons for all statements, class members, and type members or only when necessary because of [ASI](https://tc39.es/ecma262/multipage/ecmascript-language-lexical-grammar.html#sec-automatic-semicolon-insertion).
    semicolons: Semicolons,

    /// Whether to add non-necessary parentheses to arrow functions. Defaults to "always".
    arrow_parentheses: ArrowParentheses,

    /// Whether to insert spaces around brackets in object literals. Defaults to true.
    bracket_spacing: BracketSpacing,

    /// Whether to hug the closing bracket of multiline HTML/JSX tags to the end of the last line, rather than being alone on the following line. Defaults to false.
    bracket_same_line: BracketSameLine,

    /// Information related to the current file
    source_type: JsFileSource,

    /// Attribute position style. By default auto.
    attribute_position: AttributePosition,

    /// Whether to enforce collapsing object literals when possible. Defaults to "preserve".
    object_wrap: ObjectWrap,
}

impl JsFormatOptions {
    pub fn new(source_type: JsFileSource) -> Self {
        Self {
            source_type,
            indent_style: IndentStyle::default(),
            indent_width: IndentWidth::default(),
            line_ending: LineEnding::default(),
            line_width: LineWidth::default(),
            quote_style: QuoteStyle::default(),
            jsx_quote_style: QuoteStyle::default(),
            quote_properties: QuoteProperties::default(),
            trailing_commas: TrailingCommas::default(),
            semicolons: Semicolons::default(),
            arrow_parentheses: ArrowParentheses::default(),
            bracket_spacing: BracketSpacing::default(),
            bracket_same_line: BracketSameLine::default(),
            attribute_position: AttributePosition::default(),
            object_wrap: ObjectWrap::default(),
        }
    }

    pub fn with_arrow_parentheses(mut self, arrow_parentheses: ArrowParentheses) -> Self {
        self.arrow_parentheses = arrow_parentheses;
        self
    }

    pub fn with_bracket_spacing(mut self, bracket_spacing: BracketSpacing) -> Self {
        self.bracket_spacing = bracket_spacing;
        self
    }

    pub fn with_bracket_same_line(mut self, bracket_same_line: BracketSameLine) -> Self {
        self.bracket_same_line = bracket_same_line;
        self
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

    pub fn with_quote_style(mut self, quote_style: QuoteStyle) -> Self {
        self.quote_style = quote_style;
        self
    }

    pub fn with_jsx_quote_style(mut self, jsx_quote_style: QuoteStyle) -> Self {
        self.jsx_quote_style = jsx_quote_style;
        self
    }

    pub fn with_quote_properties(mut self, quote_properties: QuoteProperties) -> Self {
        self.quote_properties = quote_properties;
        self
    }

    pub fn with_trailing_commas(mut self, trailing_commas: TrailingCommas) -> Self {
        self.trailing_commas = trailing_commas;
        self
    }

    pub fn with_semicolons(mut self, semicolons: Semicolons) -> Self {
        self.semicolons = semicolons;
        self
    }

    pub fn with_attribute_position(mut self, attribute_position: AttributePosition) -> Self {
        self.attribute_position = attribute_position;
        self
    }

    pub fn with_object_wrap(mut self, object_wrap: ObjectWrap) -> Self {
        self.object_wrap = object_wrap;
        self
    }

    pub fn set_arrow_parentheses(&mut self, arrow_parentheses: ArrowParentheses) {
        self.arrow_parentheses = arrow_parentheses;
    }

    pub fn set_bracket_spacing(&mut self, bracket_spacing: BracketSpacing) {
        self.bracket_spacing = bracket_spacing;
    }

    pub fn set_bracket_same_line(&mut self, bracket_same_line: BracketSameLine) {
        self.bracket_same_line = bracket_same_line;
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

    pub fn set_quote_style(&mut self, quote_style: QuoteStyle) {
        self.quote_style = quote_style;
    }

    pub fn set_jsx_quote_style(&mut self, jsx_quote_style: QuoteStyle) {
        self.jsx_quote_style = jsx_quote_style;
    }

    pub fn set_quote_properties(&mut self, quote_properties: QuoteProperties) {
        self.quote_properties = quote_properties;
    }

    pub fn set_trailing_commas(&mut self, trailing_commas: TrailingCommas) {
        self.trailing_commas = trailing_commas;
    }

    pub fn set_attribute_position(&mut self, attribute_position: AttributePosition) {
        self.attribute_position = attribute_position;
    }

    pub fn set_object_wrap(&mut self, object_wrap: ObjectWrap) {
        self.object_wrap = object_wrap;
    }

    pub fn set_semicolons(&mut self, semicolons: Semicolons) {
        self.semicolons = semicolons;
    }

    pub fn arrow_parentheses(&self) -> ArrowParentheses {
        self.arrow_parentheses
    }

    pub fn bracket_spacing(&self) -> BracketSpacing {
        self.bracket_spacing
    }

    pub fn bracket_same_line(&self) -> BracketSameLine {
        self.bracket_same_line
    }

    pub fn quote_style(&self) -> QuoteStyle {
        self.quote_style
    }

    pub fn jsx_quote_style(&self) -> QuoteStyle {
        self.jsx_quote_style
    }

    pub fn quote_properties(&self) -> QuoteProperties {
        self.quote_properties
    }

    pub fn source_type(&self) -> JsFileSource {
        self.source_type
    }

    pub fn trailing_commas(&self) -> TrailingCommas {
        self.trailing_commas
    }

    pub fn semicolons(&self) -> Semicolons {
        self.semicolons
    }

    pub fn tab_width(&self) -> TabWidth {
        self.indent_width.value().into()
    }

    pub fn attribute_position(&self) -> AttributePosition {
        self.attribute_position
    }

    pub fn object_wrap(&self) -> ObjectWrap {
        self.object_wrap
    }
}

impl FormatOptions for JsFormatOptions {
    fn indent_style(&self) -> IndentStyle {
        self.indent_style
    }

    fn indent_width(&self) -> IndentWidth {
        self.indent_width
    }

    fn line_width(&self) -> LineWidth {
        self.line_width
    }

    fn line_ending(&self) -> LineEnding {
        self.line_ending
    }

    fn as_print_options(&self) -> PrinterOptions {
        PrinterOptions::from(self)
    }
}

impl fmt::Display for JsFormatOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Indent style: {}", self.indent_style)?;
        writeln!(f, "Indent width: {}", self.indent_width.value())?;
        writeln!(f, "Line ending: {}", self.line_ending)?;
        writeln!(f, "Line width: {}", self.line_width.value())?;
        writeln!(f, "Quote style: {}", self.quote_style)?;
        writeln!(f, "JSX quote style: {}", self.jsx_quote_style)?;
        writeln!(f, "Quote properties: {}", self.quote_properties)?;
        writeln!(f, "Trailing commas: {}", self.trailing_commas)?;
        writeln!(f, "Semicolons: {}", self.semicolons)?;
        writeln!(f, "Arrow parentheses: {}", self.arrow_parentheses)?;
        writeln!(f, "Bracket spacing: {}", self.bracket_spacing.value())?;
        writeln!(f, "Bracket same line: {}", self.bracket_same_line.value())?;
        writeln!(f, "Attribute Position: {}", self.attribute_position)?;
        writeln!(f, "Object wrap: {}", self.object_wrap)
    }
}

#[derive(Clone, Copy, Debug, Default, Deserializable, Eq, Hash, Merge, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum QuoteProperties {
    #[default]
    AsNeeded,
    Preserve,
}

impl FromStr for QuoteProperties {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "as-needed" => Ok(Self::AsNeeded),
            "preserve" => Ok(Self::Preserve),
            // TODO: replace this error with a diagnostic
            _ => Err("Value not supported for QuoteProperties"),
        }
    }
}

impl fmt::Display for QuoteProperties {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QuoteProperties::AsNeeded => write!(f, "As needed"),
            QuoteProperties::Preserve => write!(f, "Preserve"),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Deserializable, Eq, Hash, Merge, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum Semicolons {
    #[default]
    Always,
    AsNeeded,
}

impl Semicolons {
    pub const fn is_as_needed(&self) -> bool {
        matches!(self, Self::AsNeeded)
    }

    pub const fn is_always(&self) -> bool {
        matches!(self, Self::Always)
    }
}

impl FromStr for Semicolons {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "as-needed" => Ok(Self::AsNeeded),
            "always" => Ok(Self::Always),
            _ => Err("Value not supported for Semicolons. Supported values are 'as-needed' and 'always'."),
        }
    }
}

impl fmt::Display for Semicolons {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Semicolons::AsNeeded => write!(f, "As needed"),
            Semicolons::Always => write!(f, "Always"),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Deserializable, Eq, Hash, Merge, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum ArrowParentheses {
    #[default]
    Always,
    AsNeeded,
}

impl ArrowParentheses {
    pub const fn is_as_needed(&self) -> bool {
        matches!(self, Self::AsNeeded)
    }

    pub const fn is_always(&self) -> bool {
        matches!(self, Self::Always)
    }
}

// Required by [Bpaf]
impl FromStr for ArrowParentheses {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "as-needed"  => Ok(Self::AsNeeded),
            "always"  => Ok(Self::Always),
            _ => Err("Value not supported for Arrow parentheses. Supported values are 'as-needed' and 'always'."),
        }
    }
}

impl fmt::Display for ArrowParentheses {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArrowParentheses::AsNeeded => write!(f, "As needed"),
            ArrowParentheses::Always => write!(f, "Always"),
        }
    }
}
