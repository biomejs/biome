use biome_formatter::prelude::*;
use biome_formatter::{
    comments::Comments, CstFormatContext, FormatContext, FormatOptions, GroupId,
    TransformSourceMap,
};
use biome_astro_syntax::AstroLanguage;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct AstroFormatOptions {
    /// The indent style.
    indent_style: IndentStyle,

    /// The indent width.
    indent_width: IndentWidth,

    /// The line width.
    line_width: LineWidth,

    /// The line ending.
    line_ending: LineEnding,

    /// Astro-specific options
    astro_allow_shorthand: bool,
}

impl Default for AstroFormatOptions {
    fn default() -> Self {
        Self {
            indent_style: IndentStyle::default(),
            indent_width: IndentWidth::default(),
            line_width: LineWidth::default(),
            line_ending: LineEnding::default(),
            astro_allow_shorthand: true,
        }
    }
}

impl AstroFormatOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_indent_style(mut self, indent_style: IndentStyle) -> Self {
        self.indent_style = indent_style;
        self
    }

    pub fn with_indent_width(mut self, indent_width: IndentWidth) -> Self {
        self.indent_width = indent_width;
        self
    }

    pub fn with_line_width(mut self, line_width: LineWidth) -> Self {
        self.line_width = line_width;
        self
    }

    pub fn with_line_ending(mut self, line_ending: LineEnding) -> Self {
        self.line_ending = line_ending;
        self
    }

    pub fn with_astro_allow_shorthand(mut self, allow_shorthand: bool) -> Self {
        self.astro_allow_shorthand = allow_shorthand;
        self
    }

    pub fn astro_allow_shorthand(&self) -> bool {
        self.astro_allow_shorthand
    }
}

impl FormatOptions for AstroFormatOptions {
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

    fn attribute_position(&self) -> AttributePosition {
        AttributePosition::Auto
    }

    fn bracket_spacing(&self) -> BracketSpacing {
        BracketSpacing::default()
    }

    fn as_print_options(&self) -> PrinterOptions {
        PrinterOptions::from(self)
    }
}

impl From<&AstroFormatOptions> for PrinterOptions {
    fn from(options: &AstroFormatOptions) -> Self {
        PrinterOptions::default()
            .with_indent(options.indent_style())
            .with_indent_width(options.indent_width())
            .with_line_width(options.line_width())
            .with_line_ending(options.line_ending())
    }
}

pub struct AstroFormatContext {
    options: AstroFormatOptions,
    comments: Rc<Comments<AstroLanguage>>,
    source_map: Option<TransformSourceMap>,
}

impl AstroFormatContext {
    pub fn new(options: AstroFormatOptions, comments: Comments<AstroLanguage>) -> Self {
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

impl FormatContext for AstroFormatContext {
    type Options = AstroFormatOptions;

    fn options(&self) -> &Self::Options {
        &self.options
    }

    fn source_map(&self) -> Option<&TransformSourceMap> {
        self.source_map.as_ref()
    }
}

impl CstFormatContext for AstroFormatContext {
    type Language = AstroLanguage;

    fn comments(&self) -> &Comments<Self::Language> {
        &self.comments
    }
}