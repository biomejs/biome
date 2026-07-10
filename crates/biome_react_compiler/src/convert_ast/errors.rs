use super::*;

pub(super) fn missing(node: &'static str, field: &'static str) -> ReactCompilerError {
    ReactCompilerError::MissingSyntax { node, field }
}

pub(super) fn unsupported(node: &JsSyntaxNode) -> ReactCompilerError {
    ReactCompilerError::UnsupportedSyntax {
        kind: node.kind(),
        range: node.text_trimmed_range(),
    }
}
