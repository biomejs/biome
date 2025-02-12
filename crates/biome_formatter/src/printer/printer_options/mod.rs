use crate::{FormatOptions, IndentStyle, IndentWidth, LineEnding, LineWidth};

/// Options that affect how the [crate::Printer] prints the format tokens
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrinterOptions {
    /// Width of an indent in characters.
    /// if indent_style is set to IndentStyle::Tab, treat tab visualization width as this value.
    pub indent_width: IndentWidth,

    /// What's the max width of a line. Defaults to 80
    pub print_width: PrintWidth,

    /// The type of line ending to apply to the printed input
    pub line_ending: LineEnding,

    /// Whether the printer should use tabs or spaces to indent code and if spaces, by how many.
    pub indent_style: IndentStyle,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct PrintWidth(u32);

impl PrintWidth {
    pub fn new(width: u32) -> Self {
        Self(width)
    }
}

impl Default for PrintWidth {
    fn default() -> Self {
        LineWidth::default().into()
    }
}

impl From<LineWidth> for PrintWidth {
    fn from(width: LineWidth) -> Self {
        Self(u32::from(u16::from(width)))
    }
}

impl From<PrintWidth> for usize {
    fn from(width: PrintWidth) -> Self {
        width.0 as usize
    }
}

impl<'a, O> From<&'a O> for PrinterOptions
where
    O: FormatOptions,
{
    fn from(options: &'a O) -> Self {
        PrinterOptions::default()
            .with_indent_style(options.indent_style())
            .with_indent_width(options.indent_width())
            .with_print_width(options.line_width().into())
            .with_line_ending(options.line_ending())
    }
}

impl PrinterOptions {
    pub fn with_print_width(mut self, width: PrintWidth) -> Self {
        self.print_width = width;
        self
    }

    pub fn with_indent_style(mut self, style: IndentStyle) -> Self {
        self.indent_style = style;

        self
    }

    pub fn with_indent_width(mut self, width: IndentWidth) -> Self {
        self.indent_width = width;

        self
    }

    pub fn with_line_ending(mut self, line_ending: LineEnding) -> Self {
        self.line_ending = line_ending;

        self
    }

    pub(crate) fn indent_style(&self) -> IndentStyle {
        self.indent_style
    }

    /// Width of an indent in characters.
    pub(super) const fn indent_width(&self) -> IndentWidth {
        self.indent_width
    }

    #[expect(dead_code)]
    pub(super) const fn line_ending(&self) -> LineEnding {
        self.line_ending
    }
}

impl Default for PrinterOptions {
    fn default() -> Self {
        PrinterOptions {
            indent_width: IndentWidth::default(),
            print_width: PrintWidth::default(),
            indent_style: Default::default(),
            line_ending: LineEnding::Lf,
        }
    }
}
