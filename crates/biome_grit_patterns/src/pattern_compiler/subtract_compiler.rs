use super::{compilation_context::NodeCompilationContext, PatternCompiler};
use crate::{grit_context::GritQueryContext, CompileError};
use biome_grit_syntax::GritSubOperation;
use grit_pattern_matcher::pattern::Subtract;

pub(crate) struct SubtractCompiler;

impl SubtractCompiler {
    pub(crate) fn from_node(
        node: &GritSubOperation,
        context: &mut NodeCompilationContext,
    ) -> Result<Subtract<GritQueryContext>, CompileError> {
        let left = PatternCompiler::from_node(&node.left()?, context)?;
        let right = PatternCompiler::from_node(&node.right()?, context)?;

        Ok(Subtract::new(left, right))
    }
}
