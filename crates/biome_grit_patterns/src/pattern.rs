use crate::{
    grit_context::{GritExecContext, GritQueryContext},
    pattern_compiler::{compilation_context::CompilationContext, PatternCompiler},
    resolved_pattern::GritResolvedPattern,
    CompileError,
};
use anyhow::Result;
use biome_grit_syntax::{GritRoot, GritRootExt};
use biome_rowan::TextRange;
use grit_pattern_matcher::pattern::{Matcher, Pattern, State, VariableContent};
use im::Vector;

pub struct GritPattern {
    pub(crate) pattern: Pattern<GritQueryContext>,
}

impl GritPattern {
    pub fn execute(&self) -> Result<Vec<TextRange>> {
        let binding = GritResolvedPattern;
        let context = GritExecContext;
        let mut state = State::new(VarRegistry::new(), Vec::new());
        let mut logs = Vec::new().into();

        self.pattern
            .execute(&binding, &mut state, &context, &mut logs)?;
        todo!()
    }

    pub fn from_node(root: GritRoot) -> Result<Self, CompileError> {
        let mut context = CompilationContext::new();

        Ok(Self {
            pattern: PatternCompiler::from_node(
                &root.pattern().ok_or(CompileError::MissingPattern)?,
                &mut context,
            )?,
        })
    }
}

type VarRegistry<'a, P> = Vector<Vector<Vector<Box<VariableContent<'a, P>>>>>;
