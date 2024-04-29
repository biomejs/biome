use super::{compilation_context::CompilationContext, PatternCompiler};
use crate::{grit_context::GritQueryContext, CompileError};
use biome_grit_syntax::GritPatternBefore;
use grit_pattern_matcher::pattern::Before;

pub(crate) struct BeforeCompiler;

impl BeforeCompiler {
    pub(crate) fn from_node(
        node: &GritPatternBefore,
        context: &mut CompilationContext,
    ) -> Result<Before<GritQueryContext>, CompileError> {
        let pattern = PatternCompiler::from_node(&node.pattern()?, context)?;
        Ok(Before::new(pattern))
    }
}
