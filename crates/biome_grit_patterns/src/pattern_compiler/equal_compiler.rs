use super::{
    compilation_context::NodeCompilationContext, variable_compiler::VariableCompiler,
    PatternCompiler,
};
use crate::{grit_context::GritQueryContext, CompileError};
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
