use super::{PatternCompiler, compilation_context::NodeCompilationContext};
use crate::{CompileError, grit_context::GritQueryContext};
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
