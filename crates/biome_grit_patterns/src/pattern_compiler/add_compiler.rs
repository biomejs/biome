use super::{compilation_context::CompilationContext, PatternCompiler};
use crate::{grit_context::GritQueryContext, CompileError};
use biome_grit_syntax::GritAddOperation;
use grit_pattern_matcher::pattern::Add;

pub(crate) struct AddCompiler;

impl AddCompiler {
    pub(crate) fn from_node(
        node: &GritAddOperation,
        context: &mut CompilationContext,
    ) -> Result<Add<GritQueryContext>, CompileError> {
        let left = PatternCompiler::from_node(&node.left()?, context)?;
        let right = PatternCompiler::from_node(&node.right()?, context)?;

        Ok(Add::new(left, right))
    }
}
