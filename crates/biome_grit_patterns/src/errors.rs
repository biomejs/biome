use std::fmt::Debug;

use biome_console::{fmt::Formatter, markup};
use biome_diagnostics::Location;
use biome_diagnostics::{category, Category, Diagnostic, LogCategory, Severity};
use biome_parser::diagnostic::ParseDiagnostic;
use biome_rowan::SyntaxError;
use grit_util::ByteRange;

#[derive(Debug, Diagnostic)]
#[diagnostic(
    category = "parse",
    severity = Error,
    message = "Error(s) parsing pattern",
)]
pub struct ParsePatternError {
    pub diagnostics: Vec<ParseDiagnostic>,
}

#[derive(Debug, Diagnostic)]
#[diagnostic(
    category = "parse",
    severity = Error,
    message = "Error(s) parsing pattern snippet",
)]
pub struct ParseSnippetError {
    diagnostics: Vec<ParseDiagnostic>,
}

// TODO: We definitely need to improve diagnostics.
#[derive(Debug)]
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

    /// A node inside a code snippet failed to be normalized for its
    /// equivalence class.
    NormalizationError,

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

impl Diagnostic for CompileError {
    fn category(&self) -> Option<&'static Category> {
        Some(category!("parse"))
    }

    fn message(&self, fmt: &mut Formatter<'_>) -> std::io::Result<()> {
        match self {
            CompileError::ParsePatternError(error) => {
                fmt.write_markup(markup! { "Error parsing pattern" })?;
                match error.diagnostics.first() {
                    Some(diag) => {
                        fmt.write_str(": ")?;
                        diag.message(fmt)
                    }
                    None => Ok(()),
                }
            }
            CompileError::ParseSnippetError(error) => {
                fmt.write_markup(markup! { "Error parsing snippet" })?;
                match error.diagnostics.first() {
                    Some(diag) => {
                        fmt.write_str(": ")?;
                        diag.message(fmt)
                    }
                    None => Ok(()),
                }
            }
            CompileError::MissingSyntaxNode => {
                fmt.write_markup(markup! { "A syntax node was missing" })
            }
            CompileError::DuplicateParameters => {
                fmt.write_markup(markup! { "Duplicate parameters" })
            }
            CompileError::InvalidMetavariableRange(_) => {
                fmt.write_markup(markup! { "Invalid range for metavariable" })
            }
            CompileError::MetavariableNotFound(var) => {
                fmt.write_markup(markup! { "Metavariable not found: "{{var}} })
            }
            CompileError::ReservedMetavariable(var) => {
                fmt.write_markup(markup! { "Reserved metavariable: "{{var}} })
            }
            CompileError::UnsupportedKind(kind) => {
                fmt.write_markup(markup! { "Unsupported syntax kind ("{{kind}}")" })
            }
            CompileError::UnexpectedKind(kind) => {
                fmt.write_markup(markup! { "Unexpected syntax kind ("{{kind}}")" })
            }
            CompileError::UnknownFunctionOrPattern(name) => {
                fmt.write_markup(markup! { "Unknown function or pattern: "{{name}} })
            }
            CompileError::LiteralOutOfRange(value) => {
                fmt.write_markup(markup! { "Literal value out of range: "{{value}} })
            }
            CompileError::MissingPattern => fmt.write_markup(markup! { "Missing pattern" }),
            CompileError::NormalizationError => {
                fmt.write_markup(markup! { "Could not normalize node in code snippet" })
            }
            CompileError::InvalidBracketedMetavariable => {
                fmt.write_markup(markup! { "Invalid bracketed metavariable" })
            }
            CompileError::FunctionArgument(_) => {
                fmt.write_markup(markup! { "Invalid function argument" })
            }
            CompileError::UnknownFunctionOrPredicate(name) => {
                fmt.write_markup(markup! { "Unknown function or predicate: "{{name}} })
            }
            CompileError::UnknownVariable(var) => {
                fmt.write_markup(markup! { "Unknown variable: "{{var}} })
            }
        }
    }

    fn location(&self) -> Location<'_> {
        match self {
            CompileError::ParsePatternError(error) => error
                .diagnostics
                .first()
                .map(Diagnostic::location)
                .unwrap_or_default(),
            _ => Location::default(),
        }
    }

    fn description(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompileError::ParsePatternError(error) => match error.diagnostics.first() {
                Some(diag) => diag.description(fmt),
                None => Ok(()),
            },
            CompileError::ParseSnippetError(error) => match error.diagnostics.first() {
                Some(diag) => diag.description(fmt),
                None => Ok(()),
            },
            CompileError::FunctionArgument(error) => error.fmt(fmt),
            _ => Ok(()),
        }
    }

    fn advices(&self, visitor: &mut dyn biome_diagnostics::Visit) -> std::io::Result<()> {
        match self {
            CompileError::ReservedMetavariable(_) => visitor.record_log(
                LogCategory::Info,
                &markup! { "Try using a different variable name" }.to_owned(),
            ),
            _ => Ok(()),
        }
    }

    fn severity(&self) -> Severity {
        Severity::Error
    }
}

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

#[derive(Debug)]
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
