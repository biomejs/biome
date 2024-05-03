use biome_diagnostics::serde::Diagnostic as SerializableDiagnostic;
use biome_diagnostics::Diagnostic;
use biome_rowan::SyntaxError;
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

#[derive(Debug, Deserialize, Serialize)]
pub enum CompileError {
    /// Indicates the (top-level) pattern could not be parsed.
    ParsePatternError(ParsePatternError),

    /// Indicates one of the pattern's snippets could not be parsed.
    ParseSnippetError(ParseSnippetError),

    /// Used for missing syntax nodes.
    MissingSyntaxNode,

    /// If a function or bubble pattern has multiple parameters with the same name.
    DuplicateParameters,

    /// Tried to declare or assign a Grit reserved metavariable.
    ReservedMetavariable(String),

    /// When an unsupported node kind was discovered during compilation.
    UnsupportedKind(u16),

    /// When an unexpected node kind was discovered during compilation.
    UnexpectedKind(u16),

    /// A literal value was too large or too small.
    LiteralOutOfRange(String),

    /// A pattern is required to compile a Grit query.
    MissingPattern,
}

impl Diagnostic for CompileError {}

impl From<SyntaxError> for CompileError {
    fn from(error: SyntaxError) -> Self {
        match error {
            SyntaxError::MissingRequiredChild => Self::MissingSyntaxNode,
        }
    }
}
