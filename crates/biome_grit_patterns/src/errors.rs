use biome_diagnostics::serde::Diagnostic as SerializableDiagnostic;
use biome_diagnostics::Diagnostic;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Diagnostic, Serialize)]
#[diagnostic(
    category = "parse",
    severity = Error,
    message = "Error(s) parsing pattern",
)]
pub struct ParsePatternError {
    diagnostics: Vec<SerializableDiagnostic>,
}

#[derive(Debug, Deserialize, Diagnostic, Serialize)]
#[diagnostic(
    category = "parse",
    severity = Error,
    message = "Error(s) parsing pattern snippet",
)]
pub struct ParseSnippetError {
    diagnostics: Vec<SerializableDiagnostic>,
}

#[derive(Debug, Deserialize, Diagnostic, Serialize)]
pub enum ParseError {
    /// Indicates the (top-level) pattern could not be parsed.
    ParsePatternError(ParsePatternError),

    /// Indicates one of the pattern's snippets could not be parsed.
    ParseSnippetError(ParseSnippetError),
}
