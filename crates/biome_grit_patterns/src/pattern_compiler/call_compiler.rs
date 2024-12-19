use super::{compilation_context::NodeCompilationContext, PatternCompiler};
use crate::{grit_context::GritQueryContext, CompileError, NodeLikeArgumentError};
use biome_grit_syntax::{
    AnyGritMaybeNamedArg, AnyGritPattern, GritNamedArgList, GritNodeLike, GritSyntaxKind,
};
use biome_rowan::AstNode;
use grit_pattern_matcher::pattern::{Call, CallBuiltIn, CallFunction, FilePattern, Pattern};
use grit_util::{ByteRange, Language};
use std::collections::BTreeMap;

const VALID_FILE_ARGS: &[&str] = &["$name", "$body"];

/// Takes a parsed Grit CST node for node-like syntax (`foo()`) and creates a
/// node pattern.
pub(super) fn call_pattern_from_node_with_name(
    node: &GritNodeLike,
    name: String,
    context: &mut NodeCompilationContext,
    is_rhs: bool,
) -> Result<Pattern<GritQueryContext>, CompileError> {
    let named_args = named_args_from_node(node, &name, context)?;
    let mut args = named_args_to_map(named_args, context)?;
    let named_args_count = node.named_args().into_iter().count();
    if args.len() != named_args_count {
        return Err(NodeLikeArgumentError::DuplicateArguments { name }.into());
    }

    let lang = &context.compilation.lang;
    if name == "file" {
        for arg in &args {
            if !VALID_FILE_ARGS.contains(&arg.0.as_str()) {
                return Err(NodeLikeArgumentError::UnknownVariable {
                    name,
                    arg_name: arg.0.clone(),
                    valid_vars: VALID_FILE_ARGS
                        .iter()
                        .map(|arg| (*arg).to_string())
                        .collect(),
                }
                .into());
            }
        }

        let name = args
            .remove_entry("$name")
            .map_or(Pattern::Underscore, |p| p.1);
        let body = args.remove_entry("$body").map_or(Pattern::Top, |p| p.1);
        Ok(Pattern::File(Box::new(FilePattern::new(name, body))))
    } else if let Some((index, built_in)) = context.compilation.built_ins.get_with_index(&name) {
        if !is_rhs || !built_in.position.is_pattern() {
            return Err(CompileError::UnexpectedBuiltinCall(name));
        }

        let params = &built_in.params;
        Ok(Pattern::CallBuiltIn(Box::new(call_built_in_from_args(
            args, params, index, lang, &name,
        )?)))
    } else if let Some(info) = context.compilation.function_definition_info.get(&name) {
        let args = match_args_to_params(&name, args, &collect_params(&info.parameters), lang)?;
        Ok(Pattern::CallFunction(Box::new(CallFunction::new(
            info.index, args,
        ))))
    } else if let Some(info) = context.compilation.pattern_definition_info.get(&name) {
        let args = match_args_to_params(&name, args, &collect_params(&info.parameters), lang)?;
        Ok(Pattern::Call(Box::new(Call::new(info.index, args))))
    } else {
        Err(CompileError::UnknownFunctionOrPattern(name))
    }
}

fn call_built_in_from_args(
    mut args: BTreeMap<String, Pattern<GritQueryContext>>,
    params: &[&str],
    index: usize,
    lang: &impl Language,
    name: &str,
) -> Result<CallBuiltIn<GritQueryContext>, CompileError> {
    let mut pattern_params = Vec::with_capacity(args.len());
    for param in params.iter() {
        pattern_params.push(args.remove(&(lang.metavariable_prefix().to_owned() + param)));
    }
    Ok(CallBuiltIn::new(index, name, pattern_params))
}

pub(super) fn collect_params(parameters: &[(String, ByteRange)]) -> Vec<String> {
    parameters.iter().map(|p| p.0.clone()).collect()
}

pub(super) fn match_args_to_params(
    name: &str,
    mut args: BTreeMap<String, Pattern<GritQueryContext>>,
    params: &[String],
    language: &impl Language,
) -> Result<Vec<Option<Pattern<GritQueryContext>>>, CompileError> {
    for arg_name in args.keys() {
        if !params.contains(arg_name) {
            Err(NodeLikeArgumentError::UnknownVariable {
                name: name.to_string(),
                arg_name: arg_name.clone(),
                valid_vars: params
                    .iter()
                    .map(|p| p.strip_prefix(language.metavariable_prefix()).unwrap_or(p))
                    .map(str::to_string)
                    .collect(),
            })?
        }
    }

    Ok(params.iter().map(|param| args.remove(param)).collect())
}

pub(super) fn named_args_from_node(
    node: &GritNodeLike,
    name: &str,
    context: &mut NodeCompilationContext,
) -> Result<Vec<(String, AnyGritPattern)>, CompileError> {
    let expected_params = if let Some(built_in) = context
        .compilation
        .built_ins
        .as_slice()
        .iter()
        .find(|built_in| built_in.name == name)
    {
        Some(
            built_in
                .params
                .iter()
                .map(|param| (*param).to_string())
                .collect(),
        )
    } else if let Some(info) = context.compilation.function_definition_info.get(name) {
        Some(collect_params(&info.parameters))
    } else {
        context
            .compilation
            .pattern_definition_info
            .get(name)
            .map(|info| collect_params(&info.parameters))
    };

    node_to_args_pairs(
        name,
        node.named_args(),
        &context.compilation.lang,
        &expected_params,
    )
}

pub(super) fn named_args_to_map(
    named_args: Vec<(String, AnyGritPattern)>,
    context: &mut NodeCompilationContext,
) -> Result<BTreeMap<String, Pattern<GritQueryContext>>, CompileError> {
    let args = named_args
        .into_iter()
        .map(|(name, pattern)| {
            let key = context.compilation.lang.metavariable_prefix().to_owned() + &name;
            let pattern = PatternCompiler::from_node_with_rhs(&pattern, context, true)?;
            Ok((key, pattern))
        })
        .collect::<Result<_, CompileError>>()?;
    Ok(args)
}

pub(super) fn node_to_args_pairs(
    fn_name: &str,
    named_args: GritNamedArgList,
    lang: &impl Language,
    expected_params: &Option<Vec<String>>,
) -> Result<Vec<(String, AnyGritPattern)>, CompileError> {
    named_args
        .into_iter()
        .enumerate()
        .map(|(i, node)| match node {
            Ok(AnyGritMaybeNamedArg::AnyGritPattern(pattern)) => {
                let var = match pattern {
                    AnyGritPattern::GritVariable(var) => var,
                    _ => Err(NodeLikeArgumentError::ExpectedVariable {
                        name: fn_name.to_string(),
                    })?,
                };

                let name = var.to_trimmed_string();
                let name = name
                    .strip_prefix(lang.metavariable_prefix())
                    .filter(|stripped| {
                        expected_params.as_ref().map_or(true, |expected| {
                            expected.iter().any(|exp| exp == &name || exp == stripped)
                        })
                    })
                    .or_else(|| {
                        expected_params.as_ref().and_then(|params| {
                            params
                                .get(i)
                                .map(|p| p.strip_prefix(lang.metavariable_prefix()).unwrap_or(p))
                        })
                    })
                    .ok_or_else(|| {
                        if let Some(exp) = expected_params {
                            NodeLikeArgumentError::TooManyArguments {
                                name: fn_name.to_string(),
                                max_args: exp.len(),
                            }
                        } else {
                            NodeLikeArgumentError::MissingArgumentName {
                                name: fn_name.to_string(),
                                variable: name.clone(),
                            }
                        }
                    })?;
                Ok((name.to_owned(), AnyGritPattern::GritVariable(var)))
            }
            Ok(AnyGritMaybeNamedArg::GritNamedArg(named_arg)) => {
                let name = named_arg.name()?;
                let pattern = named_arg.pattern()?;
                Ok((name.to_trimmed_string(), pattern))
            }
            Ok(AnyGritMaybeNamedArg::GritBogusNamedArg(_)) => Err(CompileError::UnexpectedKind(
                GritSyntaxKind::GRIT_BOGUS_NAMED_ARG.into(),
            )),
            Err(err) => Err(err.into()),
        })
        .collect()
}
