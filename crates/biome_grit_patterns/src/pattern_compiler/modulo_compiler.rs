use super::{compilation_context::NodeCompilationContext, PatternCompiler};
use crate::{grit_context::GritQueryContext, CompileError};
use biome_grit_syntax::GritModOperation;
use grit_pattern_matcher::pattern::Modulo;

pub(crate) struct ModuloCompiler;

impl ModuloCompiler {
    pub(crate) fn from_node(
        node: &GritModOperation,
        context: &mut NodeCompilationContext,
    ) -> Result<Modulo<GritQueryContext>, CompileError> {
        let left = PatternCompiler::from_node(&node.left()?, context)?;
        let right = PatternCompiler::from_node(&node.right()?, context)?;

        Ok(Modulo::new(left, right))
    }
}
