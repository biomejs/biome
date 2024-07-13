use super::{
    compilation_context::NodeCompilationContext, variable_compiler::VariableCompiler,
    PatternCompiler,
};
use crate::{
    diagnostics::CompilerDiagnostic, grit_code_snippet::GritCodeSnippet,
    grit_context::GritQueryContext, CompileError,
};
use biome_grit_syntax::{GritPredicateRewrite, GritRewrite, GritSyntaxKind};
use biome_rowan::AstNode;
use grit_pattern_matcher::pattern::{DynamicPattern, Pattern, Rewrite};

pub(crate) struct RewriteCompiler;

impl RewriteCompiler {
    pub(crate) fn from_node(
        node: &GritRewrite,
        context: &mut NodeCompilationContext,
    ) -> Result<Rewrite<GritQueryContext>, CompileError> {
        let left = PatternCompiler::from_node(&node.left()?, context)?;

        let right = node.right()?;
        let right_syntax_kind = right.syntax().kind();
        let right = PatternCompiler::from_node_with_rhs(&right, context, true)?;

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

        let right = to_dynamic_pattern(right, right_syntax_kind)?;

        Ok(Rewrite::new(left, right, None))
    }
}

pub(crate) struct PrRewriteCompiler;

impl PrRewriteCompiler {
    pub(crate) fn from_node(
        node: &GritPredicateRewrite,
        context: &mut NodeCompilationContext,
    ) -> Result<Rewrite<GritQueryContext>, CompileError> {
        let left = Pattern::Variable(VariableCompiler::from_node(&node.left()?, context));

        let right = node.right()?;
        let right = to_dynamic_pattern(
            PatternCompiler::from_node_with_rhs(&right, context, true)?,
            right.syntax().kind(),
        )?;

        Ok(Rewrite::new(left, right, None))
    }
}

fn to_dynamic_pattern(
    pattern: Pattern<GritQueryContext>,
    syntax_kind: GritSyntaxKind,
) -> Result<DynamicPattern<GritQueryContext>, CompileError> {
    let dynamic = match pattern {
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
        _ => Err(CompileError::UnexpectedKind(syntax_kind.into()))?,
    };

    Ok(dynamic)
}
