use std::any::TypeId;

use biome_js_syntax::JsContinueStatement;
use biome_rowan::{AstNode, SyntaxError, SyntaxResult};

use crate::services::control_flow::{
    nodes::{DoWhileVisitor, ForInVisitor, ForOfVisitor, ForVisitor, WhileVisitor},
    visitor::{FunctionVisitor, NodeVisitor, StatementStack, VisitorAdapter},
    FunctionBuilder,
};

pub(in crate::services::control_flow) struct ContinueVisitor;

impl NodeVisitor for ContinueVisitor {
    type Node = JsContinueStatement;

    fn enter(
        node: Self::Node,
        builder: &mut FunctionBuilder,
        state: StatementStack,
    ) -> SyntaxResult<Self> {
        let label = node.label_token();

        let continue_block = state
            .stack
            .iter()
            .rev()
            .take_while(|(type_id, _)| *type_id != TypeId::of::<VisitorAdapter<FunctionVisitor>>())
            .find_map(|(type_id, index)| {
                let (block_label, block) = if let Some(visitor) =
                    state.try_downcast::<ForVisitor>(*type_id, *index)
                {
                    (visitor.label.as_ref(), visitor.continue_block)
                } else if let Some(visitor) = state.try_downcast::<ForInVisitor>(*type_id, *index) {
                    (visitor.label.as_ref(), visitor.continue_block)
                } else if let Some(visitor) = state.try_downcast::<ForOfVisitor>(*type_id, *index) {
                    (visitor.label.as_ref(), visitor.continue_block)
                } else if let Some(visitor) = state.try_downcast::<WhileVisitor>(*type_id, *index) {
                    (visitor.label.as_ref(), visitor.continue_block)
                } else if let Some(visitor) = state.try_downcast::<DoWhileVisitor>(*type_id, *index)
                {
                    (visitor.label.as_ref(), visitor.continue_block)
                } else {
                    return None;
                };

                match (block_label, &label) {
                    (Some(a), Some(b)) => {
                        if a.text_trimmed() == b.text_trimmed() {
                            Some(block)
                        } else {
                            None
                        }
                    }

                    (None, None) => Some(block),
                    _ => None,
                }
            })
            .ok_or(SyntaxError::MissingRequiredChild)?;

        builder
            .append_jump(false, continue_block)
            .with_node(node.into_syntax());

        Ok(Self)
    }
}
