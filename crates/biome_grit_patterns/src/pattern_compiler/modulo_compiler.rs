use super::{PatternCompiler, compilation_context::NodeCompilationContext};
use crate::{CompileError, grit_context::GritQueryContext};
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
