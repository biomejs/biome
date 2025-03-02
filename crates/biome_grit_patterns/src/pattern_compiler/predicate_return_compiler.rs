use super::{PatternCompiler, compilation_context::NodeCompilationContext};
use crate::{CompileError, grit_context::GritQueryContext};
use biome_grit_syntax::GritPredicateReturn;
use grit_pattern_matcher::pattern::PrReturn;

pub(crate) struct PrReturnCompiler;

impl PrReturnCompiler {
    pub(crate) fn from_node(
        node: &GritPredicateReturn,
        context: &mut NodeCompilationContext,
    ) -> Result<PrReturn<GritQueryContext>, CompileError> {
        let pattern = PatternCompiler::from_node_with_rhs(&node.pattern()?, context, true)?;
        Ok(PrReturn::new(pattern))
    }
}
