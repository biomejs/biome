use super::call_compiler::*;
use super::compilation_context::NodeCompilationContext;
use crate::NodeLikeArgumentError;
use crate::{grit_context::GritQueryContext, CompileError};
use biome_grit_syntax::GritPredicateCall;
use biome_rowan::AstNode;
use grit_pattern_matcher::pattern::PrCall;

pub(crate) struct PrCallCompiler;

impl PrCallCompiler {
    pub(crate) fn from_node(
        node: &GritPredicateCall,
        context: &mut NodeCompilationContext,
    ) -> Result<PrCall<GritQueryContext>, CompileError> {
        let name = node.name()?;
        let name = name.text();

        let info = if let Some(info) = context.compilation.predicate_definition_info.get(&name) {
            info
        } else if let Some(info) = context.compilation.function_definition_info.get(&name) {
            info
        } else {
            return Err(CompileError::UnknownFunctionOrPredicate(name));
        };
        let params = collect_params(&info.parameters);
        let expected_params = Some(params.clone());
        let named_args = node_to_args_pairs(
            &name,
            node.named_args(),
            &context.compilation.lang,
            &expected_params,
        )?;
        let args = named_args_to_map(named_args, context)?;
        if args.len() != node.named_args().into_iter().count() {
            Err(NodeLikeArgumentError::DuplicateArguments { name: name.clone() })?
        }

        let args = match_args_to_params(&name, args, &params, &context.compilation.lang)?;
        Ok(PrCall::new(info.index, args))
    }
}
