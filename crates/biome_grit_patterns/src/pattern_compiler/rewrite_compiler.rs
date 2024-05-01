use super::{compilation_context::NodeCompilationContext, PatternCompiler};
use crate::{
    diagnostics::CompilerDiagnostic, grit_code_snippet::GritCodeSnippet,
    grit_context::GritQueryContext, CompileError,
};
use biome_grit_syntax::GritRewrite;
use biome_rowan::AstNode;
use grit_pattern_matcher::pattern::{DynamicPattern, Pattern, Rewrite};

pub(crate) struct RewriteCompiler;

impl RewriteCompiler {
    pub(crate) fn from_node(
        node: &GritRewrite,
        context: &mut NodeCompilationContext,
    ) -> Result<Rewrite<GritQueryContext>, CompileError> {
        let left = PatternCompiler::from_node(&node.left()?, context)?;
        let right = PatternCompiler::from_node_with_rhs(&node.right()?, context, true)?;

        if let (Pattern::CodeSnippet(left_snippet), Pattern::CodeSnippet(right_snippet)) =
            (&left, &right)
        {
            if left_snippet.source == right_snippet.source {
                context.log(CompilerDiagnostic::new_warning(
                    format!(
                        "This is rewriting `{}` into the identical string `{}`, will have no effect.",
                        left_snippet.source, right_snippet.source
                    ),
                    node.syntax().text_trimmed_range()
                ));
            }
        }

        let right = match right {
            Pattern::Dynamic(r) => r,
            Pattern::CodeSnippet(GritCodeSnippet {
                dynamic_snippet: Some(r),
                ..
            }) => r,
            Pattern::Variable(v) => DynamicPattern::Variable(v),
            Pattern::Accessor(a) => DynamicPattern::Accessor(a),
            Pattern::ListIndex(a) => DynamicPattern::ListIndex(a),
            Pattern::CallBuiltIn(c) => DynamicPattern::CallBuiltIn(*c),
            Pattern::CallFunction(c) => DynamicPattern::CallFunction(*c),
            Pattern::CallForeignFunction(c) => DynamicPattern::CallForeignFunction(*c),
            _ => Err(CompileError::UnexpectedKind(
                node.right()
                    .map(|right| right.syntax().kind().into())
                    .unwrap_or_default(),
            ))?,
        };
        Ok(Rewrite::new(left, right, None))
    }
}
