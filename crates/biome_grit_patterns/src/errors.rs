use std::fmt::Debug;

use biome_console::{fmt::Formatter, markup};
use biome_diagnostics::Location;
use biome_diagnostics::{category, Category, Diagnostic, LogCategory, Severity};
use biome_parser::diagnostic::ParseDiagnostic;
use biome_rowan::SyntaxError;
use grit_util::ByteRange;

#[derive(Debug)]
pub enum CompileError {
    /// Indicates the (top-level) pattern could not be parsed.
    ParsePatternError(ParseDiagnostic),

    /// Used for syntax errors.
    SyntaxError(SyntaxError),

    /// A built-in function call was discovered in an unexpected context.
    UnexpectedBuiltinCall(String),

    /// If a function with the same name is defined multiple times.
    DuplicateFunctionDefinition(String),

    /// If a function or bubble pattern has multiple parameters with the same name.
    DuplicateParameters,

    /// If a function with the same name is defined multiple times.
    DuplicatePatternDefinition(String),

    /// If a function with the same name is defined multiple times.
    DuplicatePredicateDefinition(String),

    /// A metavariable was expected at the given range.
    InvalidMetavariableRange(ByteRange),

    /// Raw snippets are only allowed on the right-hand side of a rule.
    InvalidRawSnippetPosition,

    /// Regular expressions are not allowed on the right-hand side of a rule.
    InvalidRegexPosition,

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

    /// Unknown target language.
    UnknownTargetLanguage(String),

    /// Unknown variable.
    UnknownVariable(String),

    /// Unsupported function definition: `{name}`
    UnsupportedFunctionDefinition(String),
}

impl Diagnostic for CompileError {
    fn category(&self) -> Option<&'static Category> {
        Some(category!("parse"))
    }

    fn message(&self, fmt: &mut Formatter<'_>) -> std::io::Result<()> {
        match self {
            CompileError::ParsePatternError(diagnostic) => {
                fmt.write_markup(markup! { "Error parsing pattern: " })?;
                diagnostic.message(fmt)
            }
            CompileError::SyntaxError(SyntaxError::MissingRequiredChild) => {
                fmt.write_markup(markup! { "A syntax node was missing" })
            }
            CompileError::SyntaxError(SyntaxError::UnexpectedBogusNode) => {
                fmt.write_markup(markup! { "Unexpected bogus node" })
            }
            CompileError::SyntaxError(SyntaxError::UnexpectedMetavariable) => {
                fmt.write_markup(markup! { "Unexpected metavariable" })
            }
            CompileError::UnexpectedBuiltinCall(name) => {
                fmt.write_markup(markup! { "Unexpected call to built-in: "{{name}}"()" })
            }
            CompileError::DuplicateFunctionDefinition(name) => {
                fmt.write_markup(markup! { "Duplicate function definition: "{{name}} })
            }
            CompileError::DuplicateParameters => {
                fmt.write_markup(markup! { "Duplicate parameters" })
            }
            CompileError::DuplicatePatternDefinition(name) => {
                fmt.write_markup(markup! { "Duplicate pattern definition: "{{name}} })
            }
            CompileError::DuplicatePredicateDefinition(name) => {
                fmt.write_markup(markup! { "Duplicate predicate definition: "{{name}} })
            }
            CompileError::InvalidMetavariableRange(_) => {
                fmt.write_markup(markup! { "Invalid range for metavariable" })
            }
            CompileError::InvalidRawSnippetPosition => {
                fmt.write_markup(markup! { "Invalid range for metavariable" })
            }
            CompileError::InvalidRegexPosition => fmt.write_markup(
                markup! { "Regular expressions are not allowed on the right-hand side of a rule" },
            ),
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
            CompileError::UnknownTargetLanguage(lang) => {
                fmt.write_markup(markup! { "Unknown target language: "{{lang}} })
            }
            CompileError::UnknownVariable(var) => {
                fmt.write_markup(markup! { "Unknown variable: "{{var}} })
            }
            CompileError::UnsupportedFunctionDefinition(name) => {
                fmt.write_markup(markup! { "Unsupported foreign function definition: "{{name}} })
            }
        }
    }

    fn location(&self) -> Location<'_> {
        match self {
            CompileError::ParsePatternError(diagnostic) => diagnostic.location(),
            _ => Location::default(),
        }
    }

    fn description(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompileError::ParsePatternError(diagnostic) => diagnostic.description(fmt),
            CompileError::FunctionArgument(error) => error.fmt(fmt),
            _ => Ok(()),
        }
    }

    fn advices(&self, visitor: &mut dyn biome_diagnostics::Visit) -> std::io::Result<()> {
        match self {
            CompileError::UnexpectedBuiltinCall(name) => visitor.record_log(
                LogCategory::Info,
                &markup! { "Built-in "{{name}}" can only be used on the right-hand side of a rewrite" }
                    .to_owned(),
            ),
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
        Self::SyntaxError(error)
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
