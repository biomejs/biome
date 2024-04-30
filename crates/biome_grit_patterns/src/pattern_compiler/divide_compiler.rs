use super::{compilation_context::NodeCompilationContext, PatternCompiler};
use crate::{grit_context::GritQueryContext, CompileError};
use biome_grit_syntax::GritDivOperation;
use grit_pattern_matcher::pattern::Divide;

pub(crate) struct DivideCompiler;

impl DivideCompiler {
    pub(crate) fn from_node(
        node: &GritDivOperation,
        context: &mut NodeCompilationContext,
    ) -> Result<Divide<GritQueryContext>, CompileError> {
        let left = PatternCompiler::from_node(&node.left()?, context)?;
        let right = PatternCompiler::from_node(&node.right()?, context)?;

        Ok(Divide::new(left, right))
    }
}
