use super::{compilation_context::NodeCompilationContext, step_compiler::StepCompiler};
use crate::{grit_context::GritQueryContext, CompileError};
use biome_grit_syntax::{GritFiles, GritSequential};
use grit_pattern_matcher::pattern::{Files, Pattern, Sequential, Some, Step};

pub(crate) struct SequentialCompiler;

impl SequentialCompiler {
    pub(crate) fn from_files_node(
        node: &GritFiles,
        context: &mut NodeCompilationContext,
    ) -> Result<Sequential<GritQueryContext>, CompileError> {
        node.files()
            .into_iter()
            .map(|pattern| {
                let step = StepCompiler::from_node(&pattern?, context)?;
                let some = Pattern::Some(Box::new(Some::new(step.pattern)));
                let files = Pattern::Files(Box::new(Files::new(some)));
                Ok(Step { pattern: files })
            })
            .collect::<Result<Vec<_>, CompileError>>()
            .map(Into::into)
    }

    pub(crate) fn from_node(
        node: &GritSequential,
        context: &mut NodeCompilationContext,
    ) -> Result<Sequential<GritQueryContext>, CompileError> {
        node.sequential()
            .into_iter()
            .map(|pattern| StepCompiler::from_node(&pattern?, context))
            .collect::<Result<Vec<_>, CompileError>>()
            .map(Into::into)
    }
}
