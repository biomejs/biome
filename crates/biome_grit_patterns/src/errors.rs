use std::fmt::Formatter;

use biome_diagnostics::{Category, Diagnostic, DiagnosticTags, Location, Severity, Visit};
use biome_parser::diagnostic::ParseDiagnostic;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Diagnostic, Serialize)]
#[diagnostic(
    category = "parse",
    severity = Error,
    message = "Error(s) parsing pattern",
)]
pub struct ParsePatternError {
    diagnostics: Vec<ParseDiagnostic>,
}

#[derive(Debug, Deserialize, Diagnostic, Serialize)]
#[diagnostic(
    category = "parse",
    severity = Error,
    message = "Error(s) parsing pattern snippet",
)]
pub struct ParseSnippetError {
    diagnostics: Vec<ParseDiagnostic>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ParseError {
    /// Indicates the (top-level) pattern could not be parsed.
    ParsePatternError(ParsePatternError),

    /// Indicates one of the pattern's snippets could not be parsed.
    ParseSnippetError(ParseSnippetError),
}

impl Diagnostic for ParseError {
    fn category(&self) -> Option<&'static Category> {
        match self {
            Self::ParsePatternError(error) => error.category(),
            Self::ParseSnippetError(err) => err.category(),
        }
    }

    fn description(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParsePatternError(error) => error.description(fmt),
            Self::ParseSnippetError(error) => error.description(fmt),
        }
    }

    fn message(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        match self {
            Self::ParsePatternError(error) => error.message(fmt),
            Self::ParseSnippetError(err) => err.message(fmt),
        }
    }

    fn severity(&self) -> Severity {
        match self {
            Self::ParsePatternError(err) => err.severity(),
            Self::ParseSnippetError(err) => err.severity(),
        }
    }

    fn tags(&self) -> DiagnosticTags {
        match self {
            Self::ParsePatternError(err) => err.tags(),
            Self::ParseSnippetError(err) => err.tags(),
        }
    }

    fn location(&self) -> Location<'_> {
        match self {
            Self::ParsePatternError(err) => err.location(),
            Self::ParseSnippetError(err) => err.location(),
        }
    }

    fn source(&self) -> Option<&dyn Diagnostic> {
        match self {
            Self::ParsePatternError(error) => Diagnostic::source(error),
            Self::ParseSnippetError(error) => Diagnostic::source(error),
        }
    }

    fn advices(&self, visitor: &mut dyn Visit) -> std::io::Result<()> {
        match self {
            Self::ParsePatternError(err) => err.advices(visitor),
            Self::ParseSnippetError(err) => err.advices(visitor),
        }
    }
    fn verbose_advices(&self, visitor: &mut dyn Visit) -> std::io::Result<()> {
        match self {
            Self::ParsePatternError(err) => err.verbose_advices(visitor),
            Self::ParseSnippetError(err) => err.verbose_advices(visitor),
        }
    }
}
