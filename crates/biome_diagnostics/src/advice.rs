use crate::Applicability;
use crate::{
    display::Backtrace,
    location::{AsResource, AsSourceCode, AsSpan},
    Location,
};
use biome_console::fmt::{self, Display};
use biome_console::{markup, MarkupBuf};
use biome_text_edit::TextEdit;
use serde::{Deserialize, Serialize};
use std::io;

/// Trait implemented by types that support emitting advices into a diagnostic
pub trait Advices {
    fn record(&self, visitor: &mut dyn Visit) -> io::Result<()>;
}

/// The `Visit` trait is used to collect advices from a diagnostic: a visitor
/// instance is provided to the [Diagnostic::advices](super::Diagnostic::advices)
/// and [Diagnostic::verbose_advices](super::Diagnostic::verbose_advices) methods,
/// and the diagnostic implementation is expected to call into the various `record_*`
/// methods to communicate advices to the user.
pub trait Visit {
    /// Prints a single log entry with the provided category and markup.
    fn record_log(&mut self, category: LogCategory, text: &dyn fmt::Display) -> io::Result<()> {
        let _ = (category, text);
        Ok(())
    }

    /// Prints an unordered list of items.
    fn record_list(&mut self, list: &[&dyn fmt::Display]) -> io::Result<()> {
        let _ = list;
        Ok(())
    }

    /// Prints a code frame outlining the provided source location.
    fn record_frame(&mut self, location: Location<'_>) -> io::Result<()> {
        let _ = location;
        Ok(())
    }

    /// Prints the diff between the `prev` and `next` strings.
    fn record_diff(&mut self, diff: &TextEdit) -> io::Result<()> {
        let _ = diff;
        Ok(())
    }

    /// Prints a Rust backtrace.
    fn record_backtrace(
        &mut self,
        title: &dyn fmt::Display,
        backtrace: &Backtrace,
    ) -> io::Result<()> {
        let _ = (title, backtrace);
        Ok(())
    }

    /// Prints a command to the user.
    fn record_command(&mut self, command: &str) -> io::Result<()> {
        let _ = command;
        Ok(())
    }

    /// Prints a group of advices under a common title.
    fn record_group(&mut self, title: &dyn fmt::Display, advice: &dyn Advices) -> io::Result<()> {
        let _ = (title, advice);
        Ok(())
    }

    /// ## Warning
    ///
    /// The implementation of the table, for now, is tailored for two columns, and it assumes that
    /// the longest cell is on top.
    fn record_table(
        &mut self,
        padding: usize,
        headers: &[MarkupBuf],
        columns: &[&[MarkupBuf]],
    ) -> io::Result<()> {
        let _ = (headers, columns, padding);
        Ok(())
    }
}

/// The category for a log advice, defines how the message should be presented
/// to the user.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum LogCategory {
    /// The advice doesn't have any specific category, the message will be
    /// printed as plain markup.
    None,
    /// Print the advices with the information style.
    Info,
    /// Print the advices with the warning style.
    Warn,
    /// Print the advices with the error style.
    Error,
}

/// Utility type implementing [Advices] that emits a single log advice with
/// the provided category and text.
#[derive(Debug)]
pub struct LogAdvice<T> {
    pub category: LogCategory,
    pub text: T,
}

impl<T: Display> Advices for LogAdvice<T> {
    fn record(&self, visitor: &mut dyn Visit) -> io::Result<()> {
        visitor.record_log(self.category, &self.text)
    }
}

/// Utility advice that prints a list of items.
#[derive(Debug)]
pub struct ListAdvice<T> {
    pub list: Vec<T>,
}

impl<T: Display> Advices for ListAdvice<T> {
    fn record(&self, visitor: &mut dyn Visit) -> io::Result<()> {
        if self.list.is_empty() {
            visitor.record_log(LogCategory::Warn, &"The list is empty.")
        } else {
            let pattern_list: Vec<_> = self
                .list
                .iter()
                .map(|pattern| pattern as &dyn Display)
                .collect();

            visitor.record_list(&pattern_list)
        }
    }
}

/// Utility type implementing [Advices] that emits a single code frame
/// advice with the provided path, span and source code.
#[derive(Debug)]
pub struct CodeFrameAdvice<Path, Span, SourceCode> {
    pub path: Path,
    pub span: Span,
    pub source_code: SourceCode,
}

impl<Path, Span, SourceCode> Advices for CodeFrameAdvice<Path, Span, SourceCode>
where
    Path: AsResource,
    Span: AsSpan,
    SourceCode: AsSourceCode,
{
    fn record(&self, visitor: &mut dyn Visit) -> io::Result<()> {
        let location = Location::builder()
            .resource(&self.path)
            .span(&self.span)
            .source_code(&self.source_code)
            .build();

        visitor.record_frame(location)?;

        Ok(())
    }
}

/// Utility type implementing [Advices] that emits a diff advice with the
/// provided prev and next text.
#[derive(Debug)]
pub struct DiffAdvice<D> {
    pub diff: D,
}

impl<D> Advices for DiffAdvice<D>
where
    D: AsRef<TextEdit>,
{
    fn record(&self, visitor: &mut dyn Visit) -> io::Result<()> {
        visitor.record_diff(self.diff.as_ref())
    }
}

/// Utility type implementing [Advices] that emits a command advice with
/// the provided text.
#[derive(Debug)]
pub struct CommandAdvice<T> {
    pub command: T,
}

impl<T> Advices for CommandAdvice<T>
where
    T: AsRef<str>,
{
    fn record(&self, visitor: &mut dyn Visit) -> io::Result<()> {
        visitor.record_command(self.command.as_ref())
    }
}

#[derive(Clone, Debug)]
/// Utility type implementing [Advices] that emits a
/// code suggestion with the provided text
pub struct CodeSuggestionAdvice<M: Clone> {
    pub applicability: Applicability,
    pub msg: M,
    pub suggestion: TextEdit,
}

impl<M: Clone> Advices for CodeSuggestionAdvice<M>
where
    M: Display,
{
    fn record(&self, visitor: &mut dyn Visit) -> io::Result<()> {
        let applicability = match self.applicability {
            Applicability::Always => "Safe fix",
            Applicability::MaybeIncorrect => "Unsafe fix",
        };

        visitor.record_log(
            LogCategory::Info,
            &markup! {
                {applicability}": "{self.msg}
            },
        )?;

        visitor.record_diff(&self.suggestion)
    }
}
