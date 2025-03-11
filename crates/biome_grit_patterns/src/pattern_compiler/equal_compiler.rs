use super::{
    PatternCompiler, compilation_context::NodeCompilationContext,
    variable_compiler::VariableCompiler,
};
use crate::{CompileError, grit_context::GritQueryContext};
use biome_grit_syntax::GritPredicateEqual;
use grit_pattern_matcher::pattern::Equal;

pub(crate) struct PrEqualCompiler;

impl PrEqualCompiler {
    pub(crate) fn from_node(
        node: &GritPredicateEqual,
        context: &mut NodeCompilationContext,
    ) -> Result<Equal<GritQueryContext>, CompileError> {
        let variable = VariableCompiler::from_node(&node.left()?, context);
        let pattern = PatternCompiler::from_node_with_rhs(&node.right()?, context, true)?;

        Ok(Equal::new(variable, pattern))
    }
}
