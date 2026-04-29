use biome_js_syntax::{JsSyntaxKind, TextRange};
use react_compiler::entrypoint::compile_result::CompilerErrorDetailInfo;
use std::fmt::{Display, Formatter};

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
        detail: CompilerErrorDetailInfo,
    },
    CompilerOutput(String),
}

impl Display for ReactCompilerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingSyntax { node, field } => {
                write!(f, "missing required `{field}` field on `{node}`")
            }
            Self::UnsupportedSyntax { kind, range } => {
                write!(f, "unsupported syntax `{kind:?}` at {range:?}")
            }
            Self::InvalidLiteral { range, reason } => {
                write!(f, "invalid literal at {range:?}: {reason}")
            }
            Self::CompilerDiagnostic { detail, .. } => f.write_str(&detail.reason),
            Self::CompilerOutput(message) => f.write_str(message),
        }
    }
}

impl std::error::Error for ReactCompilerError {}

pub type Result<T> = std::result::Result<T, ReactCompilerError>;
