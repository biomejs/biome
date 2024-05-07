use super::{compilation_context::NodeCompilationContext, PatternCompiler};
use crate::{grit_context::GritQueryContext, CompileError};
use biome_grit_syntax::GritPatternLimit;
use grit_pattern_matcher::pattern::Limit;

pub(crate) struct LimitCompiler;

impl LimitCompiler {
    pub(crate) fn from_node(
        node: &GritPatternLimit,
        context: &mut NodeCompilationContext,
    ) -> Result<Limit<GritQueryContext>, CompileError> {
        let body = PatternCompiler::from_node(&node.pattern()?, context)?;
        let limit = node
            .limit()?
            .value_token()?
            .text_trimmed()
            .parse::<usize>()
            .map_err(|err| {
                CompileError::LiteralOutOfRange(format!("Error parsing limit: {err}"))
            })?;

        Ok(Limit::new(body, limit))
    }
}
