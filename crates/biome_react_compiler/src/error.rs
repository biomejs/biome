use biome_console::fmt::Formatter;
use biome_console::markup;
use biome_diagnostics::{Category, Diagnostic, DiagnosticTags, Location, Severity, category};
use biome_js_syntax::{JsSyntaxKind, TextRange};
use react_compiler::entrypoint::compile_result::CompilerErrorDetailInfo;

#[derive(Debug, Clone)]
pub enum ReactCompilerError {
    MissingSyntax {
        node: &'static str,
        field: &'static str,
    },
    UnsupportedSyntax {
        kind: JsSyntaxKind,
        range: TextRange,
    },
    InvalidLiteral {
        range: TextRange,
        reason: &'static str,
    },
    CompilerDiagnostic {
        range: Option<TextRange>,
        // Boxed: `CompilerErrorDetailInfo` is large, and boxing it keeps
        // `ReactCompilerError` (and every `Result` that returns it) small.
        detail: Box<CompilerErrorDetailInfo>,
    },
    CompilerOutput(String),
}

impl ReactCompilerError {
    pub fn range(&self) -> Option<&TextRange> {
        match self {
            Self::UnsupportedSyntax { range, .. } | Self::InvalidLiteral { range, .. } => {
                Some(range)
            }
            Self::CompilerDiagnostic { range, .. } => range.as_ref(),
            Self::MissingSyntax { .. } | Self::CompilerOutput(_) => None,
        }
    }
}

impl Diagnostic for ReactCompilerError {
    fn category(&self) -> Option<&'static Category> {
        match self {
            Self::CompilerDiagnostic { .. } => Some(category!("lint/nursery/useReactCompiler")),
            _ => Some(category!("internalError/panic")),
        }
    }

    fn severity(&self) -> Severity {
        match self {
            Self::CompilerDiagnostic { .. } => Severity::Error,
            _ => Severity::Warning,
        }
    }

    fn message(&self, fmt: &mut Formatter<'_>) -> std::io::Result<()> {
        match self {
            Self::MissingSyntax { node, field } => fmt.write_markup(markup! {
                "Missing required "<Emphasis>{*field}</Emphasis>" field on "<Emphasis>{*node}</Emphasis>"."
            }),
            Self::UnsupportedSyntax { kind, .. } => {
                let kind = format!("{kind:?}");
                fmt.write_markup(markup! {
                    "Unsupported syntax "<Emphasis>{kind}</Emphasis>"."
                })
            }
            Self::InvalidLiteral { reason, .. } => fmt.write_markup(markup! {
                "Invalid literal: "{*reason}"."
            }),
            Self::CompilerDiagnostic { detail, .. } => {
                let reason = detail.reason.as_str();
                fmt.write_markup(markup! { {reason} })
            }
            Self::CompilerOutput(message) => {
                let message = message.as_str();
                fmt.write_markup(markup! { {message} })
            }
        }
    }

    fn description(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingSyntax { node, field } => {
                write!(fmt, "missing required `{field}` field on `{node}`")
            }
            Self::UnsupportedSyntax { kind, range } => {
                write!(fmt, "unsupported syntax `{kind:?}` at {range:?}")
            }
            Self::InvalidLiteral { range, reason } => {
                write!(fmt, "invalid literal at {range:?}: {reason}")
            }
            Self::CompilerDiagnostic { detail, .. } => fmt.write_str(&detail.reason),
            Self::CompilerOutput(message) => fmt.write_str(message),
        }
    }

    fn location(&self) -> Location<'_> {
        match self.range() {
            Some(range) => Location::builder().span(range).build(),
            None => Location::builder().build(),
        }
    }

    fn tags(&self) -> DiagnosticTags {
        match self {
            Self::CompilerDiagnostic { .. } => DiagnosticTags::empty(),
            _ => DiagnosticTags::INTERNAL,
        }
    }
}

pub type Result<T> = std::result::Result<T, ReactCompilerError>;
