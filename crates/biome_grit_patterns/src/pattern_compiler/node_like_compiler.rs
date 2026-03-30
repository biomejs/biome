use super::compilation_context::NodeCompilationContext;
use super::{PatternCompiler, call_compiler::*};
use crate::NodeLikeArgumentError;
use crate::grit_node_patterns::{GritNodePattern, GritNodePatternArg};
use crate::grit_target_language::GritNodePatternSource;
use crate::grit_target_node::GritTargetSyntaxKind;
use crate::{CompileError, grit_context::GritQueryContext};
use biome_grit_syntax::{AnyGritMaybeNamedArg, AnyGritPattern, GritNodeLike, GritSyntaxKind};
use biome_rowan::TokenText;
use grit_pattern_matcher::pattern::Pattern;
use std::cmp::Ordering;

pub(crate) struct NodeLikeCompiler;

impl NodeLikeCompiler {
    pub(crate) fn from_node_with_rhs(
        node: &GritNodeLike,
        context: &mut NodeCompilationContext,
        is_rhs: bool,
    ) -> Result<Pattern<GritQueryContext>, CompileError> {
        let name = node.name()?;
        let name = name.value_token()?.token_text_trimmed();

        let lang = &context.compilation.lang;
        if let Some((kind, source)) = lang.resolve_node_pattern_name(&name) {
            node_pattern_from_node_with_name_and_kind(node, name, kind, source, context, is_rhs)
        } else {
            call_pattern_from_node_with_name(node, name.to_string(), context, is_rhs)
        }
    }
}

/// Takes a parsed Grit CST node for node-like syntax (`foo()`) and creates a
/// node pattern.
fn node_pattern_from_node_with_name_and_kind(
    node: &GritNodeLike,
    name: TokenText,
    kind: GritTargetSyntaxKind,
    source: GritNodePatternSource,
    context: &mut NodeCompilationContext,
    is_rhs: bool,
) -> Result<Pattern<GritQueryContext>, CompileError> {
    let node_slots = context
        .compilation
        .lang
        .named_slots_for_node(&name, kind, source);

    // Handle `comment(content = ...)`
    if context.compilation.lang.is_comment_kind(kind) {
        let mut named_args = named_args_from_node(node, &name, context)?;
        let args = match named_args.len().cmp(&1) {
            Ordering::Equal => {
                let (arg_name, node) = named_args.remove(0);
                if arg_name != "content" {
                    Err(NodeLikeArgumentError::UnknownArgument {
                        name: name.to_string(),
                        argument: arg_name,
                        valid_args: vec!["content".to_string()],
                    })?;
                }

                let pattern = PatternCompiler::from_node(&node, context)?;
                vec![GritNodePatternArg::new(0, pattern)]
            }
            Ordering::Less => Vec::new(),
            Ordering::Greater => Err(NodeLikeArgumentError::TooManyArguments {
                name: "comment".to_string(),
                max_args: 1,
            })?,
        };

        return Ok(Pattern::AstNode(Box::new(GritNodePattern { kind, args })));
    }

    let mut args: Vec<GritNodePatternArg> =
        Vec::with_capacity(node.named_args().into_iter().count());
    for (index, arg) in node.named_args().into_iter().enumerate() {
        let (arg_name, slot_index, pattern) = match arg {
            Ok(AnyGritMaybeNamedArg::AnyGritPattern(pattern)) => {
                let AnyGritPattern::GritVariable(variable) = pattern else {
                    return Err(NodeLikeArgumentError::ExpectedVariable {
                        name: name.to_string(),
                    }
                    .into());
                };

                let Some((slot_name, slot_index)) = node_slots.get(index) else {
                    return Err(NodeLikeArgumentError::TooManyArguments {
                        name: name.to_string(),
                        max_args: node_slots.len(),
                    }
                    .into());
                };

                (
                    (*slot_name).to_string(),
                    *slot_index,
                    AnyGritPattern::GritVariable(variable),
                )
            }
            Ok(AnyGritMaybeNamedArg::GritNamedArg(named_arg)) => {
                let arg_name = named_arg.name()?.value_token()?.token_text_trimmed();
                let Some((_, slot_index)) = node_slots
                    .iter()
                    .find(|(slot_name, _)| *slot_name == &*arg_name)
                else {
                    return Err(NodeLikeArgumentError::UnknownArgument {
                        name: name.to_string(),
                        argument: arg_name.to_string(),
                        valid_args: node_slots
                            .iter()
                            .map(|(slot_name, _)| slot_name.to_string())
                            .collect(),
                    }
                    .into());
                };

                (arg_name.to_string(), *slot_index, named_arg.pattern()?)
            }
            Ok(AnyGritMaybeNamedArg::GritBogusNamedArg(_)) => {
                return Err(CompileError::UnexpectedKind(
                    GritSyntaxKind::GRIT_BOGUS_NAMED_ARG.into(),
                ));
            }
            Err(err) => return Err(err.into()),
        };

        if args.iter().any(|arg| arg.slot_index == slot_index) {
            Err(NodeLikeArgumentError::DuplicateArguments { name: arg_name })?;
        }

        let pattern = PatternCompiler::from_node_with_rhs(&pattern, context, is_rhs)?;
        args.push(GritNodePatternArg::new(slot_index, pattern));
    }

    Ok(Pattern::AstNode(Box::new(GritNodePattern { kind, args })))
}
