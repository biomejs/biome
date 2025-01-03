use std::collections::BTreeMap;

use super::compilation_context::NodeCompilationContext;
use super::{call_compiler::*, PatternCompiler};
use crate::NodeLikeArgumentError;
use crate::{grit_context::GritQueryContext, CompileError};
use biome_grit_syntax::GritPredicateCall;
use biome_rowan::AstNode;
use grit_pattern_matcher::pattern::{CallBuiltIn, PrCall, Predicate};
use grit_util::Language;

pub(crate) struct PrCallCompiler;

impl PrCallCompiler {
    pub(crate) fn from_node(
        node: &GritPredicateCall,
        context: &mut NodeCompilationContext,
    ) -> Result<Predicate<GritQueryContext>, CompileError> {
        let name = node.name()?;
        let name = name.to_trimmed_string();

        if let Some((index, built_in)) = context.compilation.built_ins.get_with_index(&name) {
            if !built_in.position.is_predicate() {
                return Err(CompileError::UnexpectedBuiltinCall(name));
            }

            let params: Vec<String> = built_in
                .params
                .iter()
                .map(|param| (*param).to_string())
                .collect();
            let expected_params = Some(params.clone());
            let named_args = node_to_args_pairs(
                &name,
                node.named_args(),
                &context.compilation.lang,
                &expected_params,
            )?;

            let args = named_args
                .into_iter()
                .map(|(name, node)| {
                    let pattern = PatternCompiler::from_node_with_rhs(&node, context, true)?;
                    Ok((name, pattern))
                })
                .collect::<Result<BTreeMap<_, _>, CompileError>>()?;
            if args.len() != node.named_args().into_iter().count() {
                Err(NodeLikeArgumentError::DuplicateArguments { name: name.clone() })?
            }

            let args = match_args_to_params(&name, args, &params, &context.compilation.lang)?;
            return Ok(Predicate::CallBuiltIn(Box::new(CallBuiltIn::new(
                index, &name, args,
            ))));
        }

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

        let args = named_args
            .into_iter()
            .map(|(name, node)| {
                let pattern = PatternCompiler::from_node_with_rhs(&node, context, true)?;
                let name = context.compilation.lang.metavariable_prefix().to_owned() + &name;
                Ok((name, pattern))
            })
            .collect::<Result<BTreeMap<_, _>, CompileError>>()?;
        if args.len() != node.named_args().into_iter().count() {
            Err(NodeLikeArgumentError::DuplicateArguments { name: name.clone() })?
        }

        let args = match_args_to_params(&name, args, &params, &context.compilation.lang)?;
        Ok(Predicate::Call(Box::new(PrCall::new(info.index, args))))
    }
}
