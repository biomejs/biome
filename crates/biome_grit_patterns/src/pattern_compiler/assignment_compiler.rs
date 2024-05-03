use super::{
    compilation_context::NodeCompilationContext, container_compiler::ContainerCompiler,
    PatternCompiler,
};
use crate::{
    grit_context::GritQueryContext, grit_target_language::GritTargetLanguage, CompileError,
};
use biome_grit_syntax::{GritAssignmentAsPattern, GritPredicateAssignment};
use biome_rowan::AstNode;
use grit_pattern_matcher::pattern::{is_reserved_metavariable, Assignment};
use grit_util::constants::GRIT_METAVARIABLE_PREFIX;

pub(crate) struct AssignmentCompiler;

impl AssignmentCompiler {
    pub(crate) fn from_node(
        node: &GritAssignmentAsPattern,
        context: &mut NodeCompilationContext,
    ) -> Result<Assignment<GritQueryContext>, CompileError> {
        let container = node.container()?;
        let var_text = container.syntax().text_trimmed().to_string();
        if is_reserved_metavariable(&var_text, None::<&GritTargetLanguage>) {
            return Err(CompileError::ReservedMetavariable(
                var_text
                    .trim_start_matches(GRIT_METAVARIABLE_PREFIX)
                    .to_owned(),
            ));
        }

        let variable = ContainerCompiler::from_node(&container, context)?;
        let pattern = PatternCompiler::from_node_with_rhs(&node.pattern()?, context, true)?;

        Ok(Assignment::new(variable, pattern))
    }
}

pub(crate) struct PrAssignmentCompiler;

impl PrAssignmentCompiler {
    pub(crate) fn from_node(
        node: &GritPredicateAssignment,
        context: &mut NodeCompilationContext,
    ) -> Result<Assignment<GritQueryContext>, CompileError> {
        let container = node.container()?;
        let var_text = container.syntax().text_trimmed().to_string();
        if is_reserved_metavariable(&var_text, None::<&GritTargetLanguage>) {
            return Err(CompileError::ReservedMetavariable(
                var_text
                    .trim_start_matches(GRIT_METAVARIABLE_PREFIX)
                    .to_owned(),
            ));
        }

        let variable = ContainerCompiler::from_node(&container, context)?;
        let pattern = PatternCompiler::from_node_with_rhs(&node.pattern()?, context, true)?;

        Ok(Assignment::new(variable, pattern))
    }
}
