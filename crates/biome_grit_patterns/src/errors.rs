use biome_diagnostics::serde::Diagnostic as SerializableDiagnostic;
use biome_diagnostics::Diagnostic;
use biome_rowan::SyntaxError;
use grit_util::ByteRange;
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

// TODO: We definitely need to improve diagnostics.
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

    /// A metavariable was expected at the given range.
    InvalidMetavariableRange(ByteRange),

    /// Incorrect reference to a metavariable.
    MetavariableNotFound(String),

    /// Tried to declare or assign a Grit reserved metavariable.
    ReservedMetavariable(String),

    /// When an unsupported node kind was discovered during compilation.
    UnsupportedKind(u16),

    /// When an unexpected node kind was discovered during compilation.
    UnexpectedKind(u16),

    /// When trying to use an unrecognized function or pattern.
    UnknownFunctionOrPattern(String),

    /// A literal value was too large or too small.
    LiteralOutOfRange(String),

    /// A pattern is required to compile a Grit query.
    MissingPattern,

    /// Bracketed metavariables are only allowed on the right-hand side of
    /// rewrite.
    InvalidBracketedMetavariable,

    /// Unexpected function call argument.
    FunctionArgument(NodeLikeArgumentError),

    /// Unknown function or predicate.
    UnknownFunctionOrPredicate(String),

    /// Unknown variable.
    UnknownVariable(String),
}

impl Diagnostic for CompileError {}

impl From<SyntaxError> for CompileError {
    fn from(error: SyntaxError) -> Self {
        match error {
            SyntaxError::MissingRequiredChild => Self::MissingSyntaxNode,
        }
    }
}

impl From<NodeLikeArgumentError> for CompileError {
    fn from(error: NodeLikeArgumentError) -> Self {
        Self::FunctionArgument(error)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum NodeLikeArgumentError {
    /// Duplicate arguments in invocation.
    DuplicateArguments { name: String },
    /// Only variables are allowed as arguments.
    ExpectedVariable { name: String },
    /// When a named argument is missing its name.
    MissingArgumentName { name: String, variable: String },
    /// Used when too many arguments are specified.
    TooManyArguments { name: String, max_args: usize },
    /// Unknown argument given in function
    UnknownArgument {
        name: String,
        argument: String,
        valid_args: Vec<String>,
    },
    /// Used when an invalid argument is used in a function call.
    UnknownVariable {
        name: String,
        arg_name: String,
        valid_vars: Vec<String>,
    },
}
