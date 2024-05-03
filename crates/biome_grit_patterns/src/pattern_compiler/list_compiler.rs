use super::{compilation_context::NodeCompilationContext, PatternCompiler};
use crate::{grit_context::GritQueryContext, CompileError};
use biome_grit_syntax::{AnyGritListPattern, GritList};
use grit_pattern_matcher::pattern::{List, Pattern};

pub(crate) struct ListCompiler;

impl ListCompiler {
    pub(crate) fn from_node(
        node: &GritList,
        context: &mut NodeCompilationContext,
    ) -> Result<List<GritQueryContext>, CompileError> {
        Self::from_node_with_rhs(node, context, false)
    }

    pub(crate) fn from_node_with_rhs(
        node: &GritList,
        context: &mut NodeCompilationContext,
        is_rhs: bool,
    ) -> Result<List<GritQueryContext>, CompileError> {
        let patterns = node
            .patterns()
            .into_iter()
            .map(|pattern| match pattern {
                Ok(pattern) => Ok(compile_list_pattern(&pattern, context, is_rhs)?),
                Err(error) => Err(CompileError::from(error)),
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(List::new(patterns))
    }
}

fn compile_list_pattern(
    node: &AnyGritListPattern,
    context: &mut NodeCompilationContext,
    is_rhs: bool,
) -> Result<Pattern<GritQueryContext>, CompileError> {
    match node {
        AnyGritListPattern::AnyGritPattern(pattern) => {
            PatternCompiler::from_node_with_rhs(pattern, context, is_rhs)
        }
        AnyGritListPattern::GritDotdotdot(_) => Ok(Pattern::Dots),
    }
}
