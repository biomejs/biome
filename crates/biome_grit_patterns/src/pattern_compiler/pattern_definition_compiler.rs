use super::{and_compiler::AndCompiler, compilation_context::NodeCompilationContext};
use crate::{grit_context::GritQueryContext, CompileError};
use biome_grit_syntax::GritPatternDefinition;
use biome_rowan::AstNode;
use grit_pattern_matcher::pattern::{Pattern, PatternDefinition};
use std::collections::BTreeMap;

pub struct PatternDefinitionCompiler;

impl PatternDefinitionCompiler {
    pub fn from_node(
        node: GritPatternDefinition,
        context: &mut NodeCompilationContext,
    ) -> Result<PatternDefinition<GritQueryContext>, CompileError> {
        let name = node.name()?.to_trimmed_string();
        let name = name.trim();
        let mut local_vars = BTreeMap::new();
        let (scope_index, mut context) = create_scope!(context, local_vars);
        // important that this occurs first, as calls assume
        // that parameters are registered first
        let params = context.get_variables(
            &context
                .compilation
                .pattern_definition_info
                .get(name)
                .ok_or_else(|| CompileError::UnknownFunctionOrPattern(name.to_owned()))?
                .parameters,
        );

        let body = Pattern::And(Box::new(AndCompiler::from_patterns(
            node.body()?.patterns(),
            &mut context,
        )?));

        let pattern_def = PatternDefinition::new(name.to_owned(), scope_index, params, body);
        Ok(pattern_def)
    }
}
