use crate::{FormatOptions, IndentStyle, IndentWidth, LineWidth};

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
        Self(u16::from(width) as u32)
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

    pub(crate) fn indent_style(&self) -> IndentStyle {
        self.indent_style
    }

    /// Width of an indent in characters.
    pub(super) const fn indent_width(&self) -> IndentWidth {
        self.indent_width
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LineEnding {
    ///  Line Feed only (\n), common on Linux and macOS as well as inside git repos
    LineFeed,

    /// Carriage Return + Line Feed characters (\r\n), common on Windows
    CarriageReturnLineFeed,

    /// Carriage Return character only (\r), used very rarely
    CarriageReturn,
}

impl LineEnding {
    #[inline]
    pub const fn as_str(&self) -> &'static str {
        match self {
            LineEnding::LineFeed => "\n",
            LineEnding::CarriageReturnLineFeed => "\r\n",
            LineEnding::CarriageReturn => "\r",
        }
    }
}

impl Default for PrinterOptions {
    fn default() -> Self {
        PrinterOptions {
            indent_width: 2.into(),
            print_width: PrintWidth::default(),
            indent_style: Default::default(),
            line_ending: LineEnding::LineFeed,
        }
    }
}
