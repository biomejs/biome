use biome_console::fmt::MarkupElements;
use biome_console::{
    fmt, markup, HorizontalLine, Markup, MarkupBuf, MarkupElement, MarkupNode, Padding,
};
use biome_text_edit::TextEdit;
use std::path::Path;
use std::{env, io, iter};
use terminal_size::terminal_size;
use unicode_width::UnicodeWidthStr;

mod backtrace;
mod diff;
pub(super) mod frame;
mod message;

pub use crate::display::frame::{SourceFile, SourceLocation};
use crate::{
    diagnostic::internal::AsDiagnostic, Advices, Diagnostic, DiagnosticTags, Location, LogCategory,
    Resource, Severity, Visit,
};

pub use self::backtrace::{set_bottom_frame, Backtrace};
pub use self::message::MessageAndDescription;

/// Helper struct from printing the description of a diagnostic into any
/// formatter implementing [std::fmt::Write].
pub struct PrintDescription<'fmt, D: ?Sized>(pub &'fmt D);

impl<D: AsDiagnostic + ?Sized> std::fmt::Display for PrintDescription<'_, D> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0
            .as_diagnostic()
            .description(fmt)
            .map_err(|_| std::fmt::Error)
    }
}

/// Helper struct for printing a diagnostic as markup into any formatter
/// implementing [biome_console::fmt::Write].
pub struct PrintDiagnostic<'fmt, D: ?Sized> {
    diag: &'fmt D,
    verbose: bool,
    search: bool,
}

impl<'fmt, D: AsDiagnostic + ?Sized> PrintDiagnostic<'fmt, D> {
    pub fn simple(diag: &'fmt D) -> Self {
        Self {
            diag,
            verbose: false,
            search: false,
        }
    }

    pub fn verbose(diag: &'fmt D) -> Self {
        Self {
            diag,
            verbose: true,
            search: false,
        }
    }

    pub fn search(diag: &'fmt D) -> Self {
        Self {
            diag,
            verbose: false,
            search: true,
        }
    }
}

impl<D: AsDiagnostic + ?Sized> fmt::Display for PrintDiagnostic<'_, D> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> io::Result<()> {
        let diagnostic = self.diag.as_diagnostic();

        // Print the header for the diagnostic
        fmt.write_markup(markup! {
            {PrintHeader(diagnostic)}"\n\n"
        })?;
        // Wrap the formatter with an indentation level and print the advices
        let mut slot = None;
        let mut fmt = IndentWriter::wrap(fmt, &mut slot, true, "  ");

        if self.search {
            let mut visitor = PrintSearch(&mut fmt);
            print_advices(&mut visitor, diagnostic, self.verbose)
        } else {
            let mut visitor = PrintAdvices(&mut fmt);
            print_advices(&mut visitor, diagnostic, self.verbose)
        }
    }
}

/// Display struct implementing the formatting of a diagnostic header.
pub(crate) struct PrintHeader<'fmt, D: ?Sized>(pub(crate) &'fmt D);

impl<D: Diagnostic + ?Sized> fmt::Display for PrintHeader<'_, D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> io::Result<()> {
        let Self(diagnostic) = *self;

        // Wrap the formatter with a counter to measure the width of the printed text
        let mut slot = None;
        let mut fmt = CountWidth::wrap(f, &mut slot);

        // Print the diagnostic location if it has a file path
        let location = diagnostic.location();
        let file_name = match &location.resource {
            Some(Resource::File(file)) => Some(file),
            _ => None,
        };

        let is_vscode = {
            if cfg!(debug_assertions) {
                false
            } else {
                env::var("TERM_PROGRAM").unwrap_or_default() == "vscode"
            }
        };
        // https://github.com/JetBrains/jediterm/issues/253#issuecomment-1280492436
        // https://github.com/JetBrains/intellij-community/blob/5ca79d879617e9cc82f61590b8d157d6a4ad8746/plugins/terminal/src/org/jetbrains/plugins/terminal/runner/LocalOptionsConfigurer.java#L94
        let is_jetbrains = {
            if cfg!(debug_assertions) {
                false
            } else {
                env::var("TERM_PROGRAM").unwrap_or_default() == "JetBrains-JediTerm"
            }
        };

        if let Some(name) = file_name {
            if is_vscode {
                fmt.write_str(name)?;
            } else {
                let path_name = Path::new(name);
                if path_name.is_absolute() {
                    let link = format!("file://{name}");
                    fmt.write_markup(markup! {
                        <Hyperlink href={link}>{name}</Hyperlink>
                    })?;
                } else if is_jetbrains {
                    fmt.write_str(&format!(" at {name}"))?;
                } else if cfg!(debug_assertions) && cfg!(windows) {
                    fmt.write_str(name.replace('\\', "/").as_str())?;
                } else {
                    fmt.write_str(name)?;
                }
            }

            // Print the line and column position if the location has a span and source code
            // (the source code is necessary to convert a byte offset into a line + column)
            if let (Some(span), Some(source_code)) = (location.span, location.source_code) {
                let file = SourceFile::new(source_code);
                if let Ok(location) = file.location(span.start()) {
                    fmt.write_markup(markup! {
                        ":"{location.line_number.get()}":"{location.column_number.get()}
                    })?;
                }
            }

            fmt.write_str(" ")?;
        }

        // Print the category of the diagnostic, with a hyperlink if
        // the category has an associated link
        if let Some(category) = diagnostic.category() {
            if let Some(link) = category.link() {
                fmt.write_markup(markup! {
                    <Hyperlink href={link}>{category.name()}</Hyperlink>" "
                })?;
            } else {
                fmt.write_markup(markup! {
                    {category.name()}" "
                })?;
            }
        }

        // Print the internal, fixable and fatal tags
        let tags = diagnostic.tags();

        if tags.contains(DiagnosticTags::INTERNAL) {
            fmt.write_markup(markup! {
                <Inverse><Error>" INTERNAL "</Error></Inverse>" "
            })?;
        }

        if tags.contains(DiagnosticTags::FIXABLE) {
            fmt.write_markup(markup! {
                <Inverse>" FIXABLE "</Inverse>" "
            })?;
        }

        if tags.contains(DiagnosticTags::DEPRECATED_CODE) {
            fmt.write_markup(markup! {
                <Inverse>" DEPRECATED "</Inverse>" "
            })?;
        }

        if tags.contains(DiagnosticTags::VERBOSE) {
            fmt.write_markup(markup! {
                <Inverse>" VERBOSE "</Inverse>" "
            })?;
        }
        if diagnostic.severity() == Severity::Fatal {
            fmt.write_markup(markup! {
                <Inverse><Error>" FATAL "</Error></Inverse>" "
            })?;
        }

        // Load the printed width for the header, and fill the rest of the line
        // with the '━' line character up to 100 columns with at least 10 characters
        let header_width = {
            if cfg!(debug_assertions) {
                100
            } else {
                terminal_size().map_or(100, |(width, _)| width.0 as usize)
            }
        };
        const MIN_WIDTH: usize = 10;

        let text_width = slot.map_or(0, |writer| writer.width);
        let line_width = header_width.saturating_sub(text_width).max(MIN_WIDTH);
        HorizontalLine::new(line_width).fmt(f)
    }
}

/// Wrapper for a type implementing [fmt::Write] that counts the total width of
/// all printed characters.
struct CountWidth<'a, W: ?Sized> {
    writer: &'a mut W,
    width: usize,
}

impl<'write> CountWidth<'write, dyn fmt::Write + 'write> {
    /// Wrap the writer in an existing [fmt::Formatter] with an instance of [CountWidth].
    fn wrap<'slot, 'fmt: 'write + 'slot>(
        fmt: &'fmt mut fmt::Formatter<'_>,
        slot: &'slot mut Option<Self>,
    ) -> fmt::Formatter<'slot> {
        fmt.wrap_writer(|writer| slot.get_or_insert(Self { writer, width: 0 }))
    }
}

impl<W: fmt::Write + ?Sized> fmt::Write for CountWidth<'_, W> {
    fn write_str(&mut self, elements: &fmt::MarkupElements<'_>, content: &str) -> io::Result<()> {
        self.writer.write_str(elements, content)?;
        self.width += UnicodeWidthStr::width(content);
        Ok(())
    }

    fn write_fmt(
        &mut self,
        elements: &fmt::MarkupElements<'_>,
        content: std::fmt::Arguments<'_>,
    ) -> io::Result<()> {
        if let Some(content) = content.as_str() {
            self.write_str(elements, content)
        } else {
            let content = content.to_string();
            self.write_str(elements, &content)
        }
    }
}

/// Write the advices for `diagnostic` into `visitor`.
fn print_advices<V, D>(visitor: &mut V, diagnostic: &D, verbose: bool) -> io::Result<()>
where
    V: Visit,
    D: Diagnostic + ?Sized,
{
    // Visit the advices of the diagnostic with a lightweight visitor that
    // detects if the diagnostic has any frame or backtrace advice
    let mut frame_visitor = FrameVisitor {
        location: diagnostic.location(),
        skip_frame: false,
    };

    diagnostic.advices(&mut frame_visitor)?;

    let skip_frame = frame_visitor.skip_frame;

    // Print the message for the diagnostic as a log advice
    print_message_advice(visitor, diagnostic, skip_frame)?;

    // Print the other advices for the diagnostic
    diagnostic.advices(visitor)?;

    // Print the tags of the diagnostic as advices
    print_tags_advices(visitor, diagnostic)?;

    // If verbose printing is enabled, print the verbose advices in a nested group
    if verbose {
        // Count the number of verbose advices in the diagnostic
        let mut counter = CountAdvices(0);
        diagnostic.verbose_advices(&mut counter)?;

        // If the diagnostic has any verbose advice, print the group
        if !counter.is_empty() {
            let verbose_advices = PrintVerboseAdvices(diagnostic);
            visitor.record_group(&"Verbose advice", &verbose_advices)?;
        }
    }

    Ok(())
}

/// Advice visitor used to detect if the diagnostic contains any frame or backtrace diagnostic.
#[derive(Debug)]
struct FrameVisitor<'diag> {
    location: Location<'diag>,
    skip_frame: bool,
}

impl Visit for FrameVisitor<'_> {
    fn record_frame(&mut self, location: Location<'_>) -> io::Result<()> {
        if location == self.location {
            self.skip_frame = true;
        }
        Ok(())
    }

    fn record_backtrace(&mut self, _: &dyn fmt::Display, _: &Backtrace) -> io::Result<()> {
        self.skip_frame = true;
        Ok(())
    }
}

/// Print the message and code frame for the diagnostic as advices.
fn print_message_advice<V, D>(visitor: &mut V, diagnostic: &D, skip_frame: bool) -> io::Result<()>
where
    V: Visit,
    D: Diagnostic + ?Sized,
{
    // Print the entire message / cause chain for the diagnostic to a MarkupBuf
    let message = {
        let mut message = MarkupBuf::default();
        let mut fmt = fmt::Formatter::new(&mut message);
        fmt.write_markup(markup!({ PrintCauseChain(diagnostic) }))?;
        message
    };

    // Print a log advice for the message, with a special fallback if the buffer is empty
    if message.is_empty() {
        visitor.record_log(
            LogCategory::None,
            &markup! {
                <Dim>"no diagnostic message provided"</Dim>
            },
        )?;
    } else {
        let category = match diagnostic.severity() {
            Severity::Fatal | Severity::Error => LogCategory::Error,
            Severity::Warning => LogCategory::Warn,
            Severity::Information | Severity::Hint => LogCategory::Info,
        };

        visitor.record_log(category, &message)?;
    }

    // If the diagnostic has no explicit code frame or backtrace advice, print
    // a code frame advice with the location of the diagnostic
    if !skip_frame {
        let location = diagnostic.location();
        if location.span.is_some() {
            visitor.record_frame(location)?;
        }
    }

    Ok(())
}

/// Display wrapper for printing the "cause chain" of a diagnostic, with the
/// message of this diagnostic and all of its sources.
struct PrintCauseChain<'fmt, D: ?Sized>(&'fmt D);

impl<D: Diagnostic + ?Sized> fmt::Display for PrintCauseChain<'_, D> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> io::Result<()> {
        let Self(diagnostic) = *self;

        diagnostic.message(fmt)?;

        let chain = iter::successors(diagnostic.source(), |prev| prev.source());
        for diagnostic in chain {
            fmt.write_str("\n\nCaused by:\n")?;

            let mut slot = None;
            let mut fmt = IndentWriter::wrap(fmt, &mut slot, true, "  ");
            diagnostic.message(&mut fmt)?;
        }

        Ok(())
    }
}

struct PrintSearch<'a, 'b>(&'a mut fmt::Formatter<'b>);

impl Visit for PrintSearch<'_, '_> {
    fn record_frame(&mut self, location: Location<'_>) -> io::Result<()> {
        frame::print_highlighted_frame(self.0, location)
    }
}

/// Implementation of [Visitor] that prints the advices for a diagnostic.
struct PrintAdvices<'a, 'b>(&'a mut fmt::Formatter<'b>);

impl PrintAdvices<'_, '_> {
    fn print_log(
        &mut self,
        kind: MarkupElement<'_>,
        prefix: char,
        text: &dyn fmt::Display,
    ) -> io::Result<()> {
        self.0.write_markup(Markup(&[MarkupNode {
            elements: &[MarkupElement::Emphasis, kind.clone()],
            content: &prefix as &dyn fmt::Display,
        }]))?;

        self.0.write_str(" ")?;

        let mut slot = None;
        let mut fmt = IndentWriter::wrap(self.0, &mut slot, false, "  ");
        fmt.write_markup(Markup(&[MarkupNode {
            elements: &[kind],
            content: text,
        }]))?;

        self.0.write_str("\n\n")
    }
}

impl Visit for PrintAdvices<'_, '_> {
    fn record_log(&mut self, category: LogCategory, text: &dyn fmt::Display) -> io::Result<()> {
        match category {
            LogCategory::None => self.0.write_markup(markup! { {text}"\n\n" }),
            LogCategory::Info => self.print_log(MarkupElement::Info, '\u{2139}', text),
            LogCategory::Warn => self.print_log(MarkupElement::Warn, '\u{26a0}', text),
            LogCategory::Error => self.print_log(MarkupElement::Error, '\u{2716}', text),
        }
    }

    fn record_list(&mut self, list: &[&dyn fmt::Display]) -> io::Result<()> {
        for item in list {
            let mut slot = None;
            let mut fmt = IndentWriter::wrap(self.0, &mut slot, false, "  ");
            fmt.write_markup(markup! {
                "- "{*item}"\n"
            })?;
        }

        if list.is_empty() {
            Ok(())
        } else {
            self.0.write_str("\n")
        }
    }

    fn record_frame(&mut self, location: Location<'_>) -> io::Result<()> {
        frame::print_frame(self.0, location)
    }

    fn record_diff(&mut self, diff: &TextEdit) -> io::Result<()> {
        diff::print_diff(self.0, diff)
    }

    fn record_backtrace(
        &mut self,
        title: &dyn fmt::Display,
        backtrace: &Backtrace,
    ) -> io::Result<()> {
        let mut backtrace = backtrace.clone();
        backtrace.resolve();

        if backtrace.is_empty() {
            return Ok(());
        }

        self.record_log(LogCategory::Info, title)?;

        backtrace::print_backtrace(self.0, &backtrace)
    }

    fn record_command(&mut self, command: &str) -> io::Result<()> {
        self.0.write_markup(markup! {
            <Emphasis>"$"</Emphasis>" "{command}"\n\n"
        })
    }

    fn record_group(&mut self, title: &dyn fmt::Display, advice: &dyn Advices) -> io::Result<()> {
        self.0.write_markup(markup! {
            <Emphasis>{title}</Emphasis>"\n\n"
        })?;

        let mut slot = None;
        let mut fmt = IndentWriter::wrap(self.0, &mut slot, true, "  ");
        let mut visitor = PrintAdvices(&mut fmt);
        advice.record(&mut visitor)
    }

    fn record_table(
        &mut self,
        padding: usize,
        headers: &[MarkupBuf],
        columns: &[&[MarkupBuf]],
    ) -> io::Result<()> {
        debug_assert_eq!(
            headers.len(),
            columns.len(),
            "headers and columns must have the same number length"
        );

        if columns.is_empty() {
            return Ok(());
        }

        let mut headers_iter = headers.iter().enumerate();
        let rows_number = columns[0].len();
        let columns_number = columns.len();

        let mut longest_cell = 0;
        for current_row_index in 0..rows_number {
            for current_column_index in 0..columns_number {
                let cell = columns
                    .get(current_column_index)
                    .and_then(|c| c.get(current_row_index));
                if let Some(cell) = cell {
                    if current_column_index == 0 && current_row_index == 0 {
                        longest_cell = cell.text_len();
                        for (index, header_cell) in headers_iter.by_ref() {
                            self.0.write_markup(markup!({ header_cell }))?;
                            if index < headers.len() - 1 {
                                self.0.write_markup(
                                    markup! {{Padding::new(padding + longest_cell.saturating_sub(header_cell.text_len()))}},
                                )?;
                            }
                        }

                        self.0.write_markup(markup! {"\n\n"})?;
                    }
                    let extra_padding = longest_cell.saturating_sub(cell.text_len());

                    self.0.write_markup(markup!({ cell }))?;
                    if columns_number != current_column_index + 1 {
                        self.0
                            .write_markup(markup! {{Padding::new(padding + extra_padding)}})?;
                    }
                }
            }
            self.0.write_markup(markup!("\n"))?;
        }

        Ok(())
    }
}

/// Print the fatal and internal tags for the diagnostic as log advices.
fn print_tags_advices<V, D>(visitor: &mut V, diagnostic: &D) -> io::Result<()>
where
    V: Visit,
    D: Diagnostic + ?Sized,
{
    if diagnostic.severity() == Severity::Fatal {
        visitor.record_log(LogCategory::Warn, &"Biome exited as this error could not be handled and resulted in a fatal error. Please report it if necessary.")?;
    }

    if diagnostic.tags().contains(DiagnosticTags::INTERNAL) {
        visitor.record_log(LogCategory::Warn, &"This diagnostic was derived from an internal Biome error. Potential bug, please report it if necessary.")?;
    }

    Ok(())
}

/// Advice visitor that counts how many advices are visited.
struct CountAdvices(usize);

impl CountAdvices {
    fn is_empty(&self) -> bool {
        self.0 == 0
    }
}

impl Visit for CountAdvices {
    fn record_log(&mut self, _: LogCategory, _: &dyn fmt::Display) -> io::Result<()> {
        self.0 += 1;
        Ok(())
    }

    fn record_list(&mut self, _: &[&dyn fmt::Display]) -> io::Result<()> {
        self.0 += 1;
        Ok(())
    }

    fn record_frame(&mut self, _: Location<'_>) -> io::Result<()> {
        self.0 += 1;
        Ok(())
    }

    fn record_diff(&mut self, _: &TextEdit) -> io::Result<()> {
        self.0 += 1;
        Ok(())
    }

    fn record_backtrace(&mut self, _: &dyn fmt::Display, _: &Backtrace) -> io::Result<()> {
        self.0 += 1;
        Ok(())
    }

    fn record_command(&mut self, _: &str) -> io::Result<()> {
        self.0 += 1;
        Ok(())
    }

    fn record_group(&mut self, _: &dyn fmt::Display, _: &dyn Advices) -> io::Result<()> {
        self.0 += 1;
        Ok(())
    }
}

/// Implements [Advices] for verbose advices of a diagnostic.
struct PrintVerboseAdvices<'a, D: ?Sized>(&'a D);

impl<D: Diagnostic + ?Sized> Advices for PrintVerboseAdvices<'_, D> {
    fn record(&self, visitor: &mut dyn Visit) -> io::Result<()> {
        self.0.verbose_advices(visitor)
    }
}

/// Wrapper type over [fmt::Write] that injects `ident_text` at the start of
/// every line.
struct IndentWriter<'a, W: ?Sized> {
    writer: &'a mut W,
    pending_indent: bool,
    ident_text: &'static str,
}

impl<'write> IndentWriter<'write, dyn fmt::Write + 'write> {
    fn wrap<'slot, 'fmt: 'write + 'slot>(
        fmt: &'fmt mut fmt::Formatter<'_>,
        slot: &'slot mut Option<Self>,
        pending_indent: bool,
        ident_text: &'static str,
    ) -> fmt::Formatter<'slot> {
        fmt.wrap_writer(|writer| {
            slot.get_or_insert(Self {
                writer,
                pending_indent,
                ident_text,
            })
        })
    }
}

impl<W: fmt::Write + ?Sized> fmt::Write for IndentWriter<'_, W> {
    fn write_str(
        &mut self,
        elements: &fmt::MarkupElements<'_>,
        mut content: &str,
    ) -> io::Result<()> {
        while !content.is_empty() {
            if self.pending_indent {
                self.writer
                    .write_str(&MarkupElements::Root, self.ident_text)?;
                self.pending_indent = false;
            }

            if let Some(index) = content.find('\n') {
                let (start, end) = content.split_at(index + 1);
                self.writer.write_str(elements, start)?;
                self.pending_indent = true;
                content = end;
            } else {
                return self.writer.write_str(elements, content);
            }
        }

        Ok(())
    }

    fn write_fmt(
        &mut self,
        elements: &fmt::MarkupElements<'_>,
        content: std::fmt::Arguments<'_>,
    ) -> io::Result<()> {
        if let Some(content) = content.as_str() {
            self.write_str(elements, content)
        } else {
            let content = content.to_string();
            self.write_str(elements, &content)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io;

    use biome_console::{fmt, markup};
    use biome_diagnostics::{DiagnosticTags, Severity};
    use biome_diagnostics_categories::{category, Category};
    use biome_text_edit::TextEdit;
    use biome_text_size::{TextRange, TextSize};
    use serde_json::{from_value, json};

    use crate::{self as biome_diagnostics};
    use crate::{
        Advices, Diagnostic, Location, LogCategory, PrintDiagnostic, Resource, SourceCode, Visit,
    };

    #[derive(Debug)]
    struct TestDiagnostic<A> {
        path: Option<String>,
        span: Option<TextRange>,
        source_code: Option<String>,
        advice: Option<A>,
        verbose_advice: Option<A>,
        source: Option<Box<dyn Diagnostic>>,
    }

    impl<A> TestDiagnostic<A> {
        fn empty() -> Self {
            Self {
                path: None,
                span: None,
                source_code: None,
                advice: None,
                verbose_advice: None,
                source: None,
            }
        }

        fn with_location() -> Self {
            Self {
                path: Some(String::from("path")),
                span: Some(TextRange::at(TextSize::from(0), TextSize::from(6))),
                source_code: Some(String::from("source code")),
                advice: None,
                verbose_advice: None,
                source: None,
            }
        }
    }

    impl<A: Advices + std::fmt::Debug> Diagnostic for TestDiagnostic<A> {
        fn category(&self) -> Option<&'static Category> {
            Some(category!("internalError/io"))
        }

        fn severity(&self) -> Severity {
            Severity::Error
        }

        fn description(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(fmt, "diagnostic message")
        }

        fn message(&self, fmt: &mut fmt::Formatter<'_>) -> io::Result<()> {
            write!(fmt, "diagnostic message")
        }

        fn advices(&self, visitor: &mut dyn Visit) -> io::Result<()> {
            if let Some(advice) = &self.advice {
                advice.record(visitor)?;
            }

            Ok(())
        }

        fn verbose_advices(&self, visitor: &mut dyn Visit) -> io::Result<()> {
            if let Some(advice) = &self.verbose_advice {
                advice.record(visitor)?;
            }

            Ok(())
        }

        fn location(&self) -> Location<'_> {
            Location::builder()
                .resource(&self.path)
                .span(&self.span)
                .source_code(&self.source_code)
                .build()
        }

        fn tags(&self) -> DiagnosticTags {
            DiagnosticTags::FIXABLE
        }

        fn source(&self) -> Option<&dyn Diagnostic> {
            self.source.as_deref()
        }
    }

    #[derive(Debug)]
    struct LogAdvices;

    impl Advices for LogAdvices {
        fn record(&self, visitor: &mut dyn Visit) -> io::Result<()> {
            visitor.record_log(LogCategory::Error, &"error")?;
            visitor.record_log(LogCategory::Warn, &"warn")?;
            visitor.record_log(LogCategory::Info, &"info")?;
            visitor.record_log(LogCategory::None, &"none")
        }
    }

    #[derive(Debug)]
    struct ListAdvice;

    impl Advices for ListAdvice {
        fn record(&self, visitor: &mut dyn Visit) -> io::Result<()> {
            visitor.record_list(&[&"item 1", &"item 2"])
        }
    }

    #[derive(Debug)]
    struct FrameAdvice;

    impl Advices for FrameAdvice {
        fn record(&self, visitor: &mut dyn Visit) -> io::Result<()> {
            visitor.record_frame(Location {
                resource: Some(Resource::File("other_path")),
                span: Some(TextRange::new(TextSize::from(8), TextSize::from(16))),
                source_code: Some(SourceCode {
                    text: "context location context",
                    line_starts: None,
                }),
            })
        }
    }

    #[derive(Debug)]
    struct DiffAdvice;

    impl Advices for DiffAdvice {
        fn record(&self, visitor: &mut dyn Visit) -> io::Result<()> {
            let diff =
                TextEdit::from_unicode_words("context before context", "context after context");
            visitor.record_diff(&diff)
        }
    }

    #[derive(Debug)]
    struct BacktraceAdvice;

    impl Advices for BacktraceAdvice {
        fn record(&self, visitor: &mut dyn Visit) -> io::Result<()> {
            let backtrace = from_value(json!([
                {
                    "ip": 0x0f0f_0f0f,
                    "symbols": [
                        {
                            "name": "crate::module::function",
                            "filename": "crate/src/module.rs",
                            "lineno": 8,
                            "colno": 16
                        }
                    ]
                }
            ]));

            visitor.record_backtrace(&"Backtrace Title", &backtrace.unwrap())
        }
    }

    #[derive(Debug)]
    struct CommandAdvice;

    impl Advices for CommandAdvice {
        fn record(&self, visitor: &mut dyn Visit) -> io::Result<()> {
            visitor.record_command("biome command --argument")
        }
    }

    #[derive(Debug)]
    struct GroupAdvice;

    impl Advices for GroupAdvice {
        fn record(&self, visitor: &mut dyn Visit) -> io::Result<()> {
            visitor.record_group(&"Group Title", &LogAdvices)
        }
    }

    #[test]
    fn test_header() {
        let diag = TestDiagnostic::<LogAdvices>::with_location();

        let diag = markup!({ PrintDiagnostic::verbose(&diag) }).to_owned();

        let expected = markup!{
            "path:1:1 internalError/io "<Inverse>" FIXABLE "</Inverse>" ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n"
            "\n"
            "  "
            <Emphasis><Error>"✖"</Error></Emphasis>" "<Error>"diagnostic message"</Error>"\n"
            "  \n"
            "  "
            <Emphasis><Error>">"</Error></Emphasis>" "<Emphasis>"1 │ "</Emphasis>"source code\n"
            "   "<Emphasis>"   │ "</Emphasis><Emphasis><Error>"^^^^^^"</Error></Emphasis>"\n"
            "  \n"
        }.to_owned();

        assert_eq!(
            diag, expected,
            "\nactual:\n{diag:#?}\nexpected:\n{expected:#?}"
        );
    }
    #[test]
    fn test_log_advices() {
        let diag = TestDiagnostic {
            advice: Some(LogAdvices),
            ..TestDiagnostic::empty()
        };

        let diag = markup!({ PrintDiagnostic::verbose(&diag) }).to_owned();

        let expected = markup!{
            "internalError/io "<Inverse>" FIXABLE "</Inverse>" ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n"
            "\n"
            "  "
            <Emphasis><Error>"✖"</Error></Emphasis>" "<Error>"diagnostic message"</Error>"\n"
            "  \n"
            "  "
            <Emphasis><Error>"✖"</Error></Emphasis>" "<Error>"error"</Error>"\n"
            "  \n"
            "  "
            <Emphasis><Warn>"⚠"</Warn></Emphasis>" "<Warn>"warn"</Warn>"\n"
            "  \n"
            "  "
            <Emphasis><Info>"ℹ"</Info></Emphasis>" "<Info>"info"</Info>"\n"
            "  \n"
            "  none\n"
            "  \n"
        }.to_owned();

        assert_eq!(
            diag, expected,
            "\nactual:\n{diag:#?}\nexpected:\n{expected:#?}"
        );
    }

    #[test]
    fn test_list_advice() {
        let diag = TestDiagnostic {
            advice: Some(ListAdvice),
            ..TestDiagnostic::empty()
        };

        let diag = markup!({ PrintDiagnostic::verbose(&diag) }).to_owned();

        let expected = markup!{
            "internalError/io "<Inverse>" FIXABLE "</Inverse>" ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n"
            "\n"
            "  "
            <Emphasis><Error>"✖"</Error></Emphasis>" "<Error>"diagnostic message"</Error>"\n"
            "  \n"
            "  - item 1\n"
            "  - item 2\n"
            "  \n"
        }.to_owned();

        assert_eq!(
            diag, expected,
            "\nactual:\n{diag:#?}\nexpected:\n{expected:#?}"
        );
    }

    #[test]
    fn test_frame_advice() {
        let diag = TestDiagnostic {
            advice: Some(FrameAdvice),
            ..TestDiagnostic::empty()
        };

        let diag = markup!({ PrintDiagnostic::verbose(&diag) }).to_owned();

        let expected = markup!{
            "internalError/io "<Inverse>" FIXABLE "</Inverse>" ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n"
            "\n"
            "  "
            <Emphasis><Error>"✖"</Error></Emphasis>" "<Error>"diagnostic message"</Error>"\n"
            "  \n"
            "  "
            <Emphasis><Error>">"</Error></Emphasis>" "<Emphasis>"1 │ "</Emphasis>"context location context\n"
            "   "<Emphasis>"   │ "</Emphasis>"        "<Emphasis><Error>"^^^^^^^^"</Error></Emphasis>"\n"
            "  \n"
        }.to_owned();

        assert_eq!(
            diag, expected,
            "\nactual:\n{diag:#?}\nexpected:\n{expected:#?}"
        );
    }

    #[test]
    fn test_diff_advice() {
        let diag = TestDiagnostic {
            advice: Some(DiffAdvice),
            ..TestDiagnostic::empty()
        };

        let diag = markup!({ PrintDiagnostic::verbose(&diag) }).to_owned();

        let expected = markup!{
            "internalError/io "<Inverse>" FIXABLE "</Inverse>" ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n"
            "\n"
            "  "
            <Emphasis><Error>"✖"</Error></Emphasis>" "<Error>"diagnostic message"</Error>"\n"
            "  \n"
            "  "
            <Error>"-"</Error>" "<Error>"context"</Error><Error><Dim>"·"</Dim></Error><Error><Emphasis>"before"</Emphasis></Error><Error><Dim>"·"</Dim></Error><Error>"context"</Error>"\n"
            "  "
            <Success>"+"</Success>" "<Success>"context"</Success><Success><Dim>"·"</Dim></Success><Success><Emphasis>"after"</Emphasis></Success><Success><Dim>"·"</Dim></Success><Success>"context"</Success>"\n"
            "  \n"
        }.to_owned();

        assert_eq!(
            diag, expected,
            "\nactual:\n{diag:#?}\nexpected:\n{expected:#?}"
        );
    }

    #[test]
    fn test_backtrace_advice() {
        let diag = TestDiagnostic {
            advice: Some(BacktraceAdvice),
            ..TestDiagnostic::empty()
        };

        let diag = markup!({ PrintDiagnostic::verbose(&diag) }).to_owned();

        let expected = markup!{
            "internalError/io "<Inverse>" FIXABLE "</Inverse>" ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n"
            "\n"
            "  "
            <Emphasis><Error>"✖"</Error></Emphasis>" "<Error>"diagnostic message"</Error>"\n"
            "  \n"
            "  "
            <Emphasis><Info>"ℹ"</Info></Emphasis>" "<Info>"Backtrace Title"</Info>"\n"
            "  \n"
            "     0: crate::module::function\n"
            "            at crate/src/module.rs:8:16\n"
        }.to_owned();

        assert_eq!(
            diag, expected,
            "\nactual:\n{diag:#?}\nexpected:\n{expected:#?}"
        );
    }

    #[test]
    fn test_command_advice() {
        let diag = TestDiagnostic {
            advice: Some(CommandAdvice),
            ..TestDiagnostic::empty()
        };

        let diag = markup!({ PrintDiagnostic::verbose(&diag) }).to_owned();

        let expected = markup!{
            "internalError/io "<Inverse>" FIXABLE "</Inverse>" ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n"
            "\n"
            "  "
            <Emphasis><Error>"✖"</Error></Emphasis>" "<Error>"diagnostic message"</Error>"\n"
            "  \n"
            "  "
            <Emphasis>"$"</Emphasis>" biome command --argument\n"
            "  \n"
        }.to_owned();

        assert_eq!(
            diag, expected,
            "\nactual:\n{diag:#?}\nexpected:\n{expected:#?}"
        );
    }

    #[test]
    fn test_group_advice() {
        let diag = TestDiagnostic {
            advice: Some(GroupAdvice),
            ..TestDiagnostic::empty()
        };

        let diag = markup!({ PrintDiagnostic::verbose(&diag) }).to_owned();

        let expected = markup!{
            "internalError/io "<Inverse>" FIXABLE "</Inverse>" ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n"
            "\n"
            "  "
            <Emphasis><Error>"✖"</Error></Emphasis>" "<Error>"diagnostic message"</Error>"\n"
            "  \n"
            "  "
            <Emphasis>"Group Title"</Emphasis>"\n"
            "  \n"
            "    "
            <Emphasis><Error>"✖"</Error></Emphasis>" "<Error>"error"</Error>"\n"
            "    \n"
            "    "
            <Emphasis><Warn>"⚠"</Warn></Emphasis>" "<Warn>"warn"</Warn>"\n"
            "    \n"
            "    "
            <Emphasis><Info>"ℹ"</Info></Emphasis>" "<Info>"info"</Info>"\n"
            "    \n"
            "    none\n"
            "    \n"
        }.to_owned();

        assert_eq!(
            diag, expected,
            "\nactual:\n{diag:#?}\nexpected:\n{expected:#?}"
        );
    }
}
