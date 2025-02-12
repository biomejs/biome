use super::{and_compiler::PrAndCompiler, compilation_context::NodeCompilationContext};
use crate::{grit_context::GritQueryContext, CompileError};
use biome_rowan::AstNode;
use grit_pattern_matcher::pattern::{GritFunctionDefinition, Predicate};
use std::collections::BTreeMap;

pub struct FunctionDefinitionCompiler;

impl FunctionDefinitionCompiler {
    pub fn from_node(
        node: biome_grit_syntax::GritFunctionDefinition,
        context: &mut NodeCompilationContext,
    ) -> Result<GritFunctionDefinition<GritQueryContext>, CompileError> {
        let name = node.name()?.to_trimmed_string();
        let name = name.trim();
        let mut local_vars = BTreeMap::new();
        let (scope_index, mut context) = create_scope!(context, local_vars);
        // important that this occurs first, as calls assume
        // that parameters are registered first
        let params = context.get_variables(
            &context
                .compilation
                .function_definition_info
                .get(name)
                .ok_or_else(|| CompileError::UnknownFunctionOrPredicate(name.to_owned()))?
                .parameters,
        );

        let body = Predicate::And(Box::new(PrAndCompiler::from_predicates(
            node.body()?.predicates(),
            &mut context,
        )?));

        let pattern_def = GritFunctionDefinition::new(
            name.to_owned(),
            scope_index,
            params,
            local_vars.values().copied().collect(),
            body,
        );
        Ok(pattern_def)
    }
}
