use crate::services::control_flow::JsControlFlowGraph;
use biome_control_flow::{ExceptionHandlerKind, InstructionKind, builder::ROOT_BLOCK_ID};
use biome_js_syntax::JsReturnStatement;
use biome_rowan::{AstNode, NodeOrToken, TextRange};
use roaring::RoaringBitmap;

/// A way for a function to finish without returning a value.
#[derive(Debug)]
pub enum InvalidReturn {
    /// The function reaches the end of its body without a `return` statement.
    MissingReturn,
    /// A `return` statement without an argument.
    EmptyReturn(TextRange),
}

/// Returns every way the function of `cfg` can finish without returning a
/// value: a `return` statement without an argument, or reaching the end of the
/// function body.
pub fn invalid_returns(cfg: &JsControlFlowGraph) -> Box<[InvalidReturn]> {
    let mut invalid_returns = Vec::new();
    // stack of blocks to process
    let mut block_stack = vec![ROOT_BLOCK_ID];
    let mut visited_blocks = RoaringBitmap::new();
    visited_blocks.insert(ROOT_BLOCK_ID.index());
    while let Some(block_id) = block_stack.pop() {
        let block = cfg.get(block_id);
        for exception_handler in block.exception_handlers.iter() {
            // Ignore finally handler: they are already in the Control Flow Graph.
            if matches!(exception_handler.kind, ExceptionHandlerKind::Catch) {
                // Avoid cycles and redundant checks.
                if visited_blocks.insert(exception_handler.target.index()) {
                    block_stack.push(exception_handler.target);
                }
            }
        }
        for instruction in block.instructions.iter() {
            match instruction.kind {
                InstructionKind::Statement => {}
                InstructionKind::Jump {
                    conditional,
                    block: jump_block_id,
                    ..
                } => {
                    // Avoid cycles and redundant checks.
                    if visited_blocks.insert(jump_block_id.index()) {
                        block_stack.push(jump_block_id);
                    }
                    if !conditional {
                        // The next instructions are unreachable.
                        break;
                    }
                }
                InstructionKind::Return => {
                    if let Some(NodeOrToken::Node(node)) = &instruction.node {
                        if let Some(return_stmt) = JsReturnStatement::cast_ref(node)
                            && return_stmt.argument().is_none()
                        {
                            invalid_returns.push(InvalidReturn::EmptyReturn(return_stmt.range()));
                        }
                    } else {
                        invalid_returns.push(InvalidReturn::MissingReturn);
                    }
                    // The next instructions are unreachable.
                    break;
                }
            }
        }
    }
    invalid_returns.into_boxed_slice()
}
