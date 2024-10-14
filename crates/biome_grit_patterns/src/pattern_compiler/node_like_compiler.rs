use super::compilation_context::NodeCompilationContext;
use super::{call_compiler::*, PatternCompiler};
use crate::grit_node_patterns::{GritNodePattern, GritNodePatternArg};
use crate::grit_target_node::GritTargetSyntaxKind;
use crate::NodeLikeArgumentError;
use crate::{grit_context::GritQueryContext, CompileError};
use biome_grit_syntax::GritNodeLike;
use biome_rowan::AstNode;
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
        let name = name.to_trimmed_string();

        let lang = &context.compilation.lang;
        if let Some(kind) = lang.kind_by_name(&name) {
            node_pattern_from_node_with_name_and_kind(node, name, kind, context, is_rhs)
        } else {
            call_pattern_from_node_with_name(node, name, context, is_rhs)
        }
    }
}

/// Takes a parsed Grit CST node for node-like syntax (`foo()`) and creates a
/// node pattern.
fn node_pattern_from_node_with_name_and_kind(
    node: &GritNodeLike,
    name: String,
    kind: GritTargetSyntaxKind,
    context: &mut NodeCompilationContext,
    is_rhs: bool,
) -> Result<Pattern<GritQueryContext>, CompileError> {
    let mut named_args = named_args_from_node(node, &name, context)?;

    // Handle `comment(content = ...)`
    if context.compilation.lang.is_comment_kind(kind) {
        let args = match named_args.len().cmp(&1) {
            Ordering::Equal => {
                let (arg_name, node) = named_args.remove(0);
                if arg_name != "content" {
                    Err(NodeLikeArgumentError::UnknownArgument {
                        name,
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

    let mut args: Vec<GritNodePatternArg> = Vec::with_capacity(named_args.len());
    for (arg_name, node) in named_args {
        let node_slots = &context.compilation.lang.named_slots_for_kind(kind);

        let Some((_, slot_index)) = node_slots
            .iter()
            .find(|(slot_name, _)| *slot_name == arg_name)
        else {
            return Err(NodeLikeArgumentError::UnknownArgument {
                name,
                argument: arg_name,
                valid_args: node_slots
                    .iter()
                    .map(|(_, slot_name)| slot_name.to_string())
                    .collect(),
            })?;
        };

        if args.iter().any(|arg| arg.slot_index == *slot_index) {
            Err(NodeLikeArgumentError::DuplicateArguments { name: arg_name })?;
        }

        let pattern = PatternCompiler::from_node_with_rhs(&node, context, is_rhs)?;
        args.push(GritNodePatternArg::new(*slot_index, pattern));
    }

    Ok(Pattern::AstNode(Box::new(GritNodePattern { kind, args })))
}
