use biome_control_flow::builder::BlockId;
use biome_js_syntax::{JsLabeledStatement, JsSyntaxToken, JsWhileStatement};
use biome_rowan::{AstNode, SyntaxResult};

use crate::services::control_flow::{
    visitor::{NodeVisitor, StatementStack},
    FunctionBuilder,
};

pub(in crate::services::control_flow) struct WhileVisitor {
    // `label`, `continue_block` and `break_block` are used by the
    // `ContinueVisitor` and `BreakVisitor`
    pub(super) label: Option<JsSyntaxToken>,
    pub(super) continue_block: BlockId,
    pub(super) break_block: BlockId,
    loop_block: BlockId,
}

impl NodeVisitor for WhileVisitor {
    type Node = JsWhileStatement;

    fn enter(
        node: Self::Node,
        builder: &mut FunctionBuilder,
        _: StatementStack,
    ) -> SyntaxResult<Self> {
        // Create the continue and break blocks
        let continue_block = builder.append_block();
        let break_block = builder.append_block();

        // Unconditionally jump to the continue block
        builder.append_jump(false, continue_block);

        // Create the loop block and fill it with the loop body statement
        let loop_block = builder.append_block();
        builder.set_cursor(loop_block);

        let label = node
            .parent::<JsLabeledStatement>()
            .and_then(|label| label.label_token().ok());

        Ok(Self {
            label,
            continue_block,
            break_block,
            loop_block,
        })
    }

    fn exit(
        self,
        node: Self::Node,
        builder: &mut FunctionBuilder,
        _: StatementStack,
    ) -> SyntaxResult<()> {
        let Self {
            continue_block,
            break_block,
            loop_block,
            ..
        } = self;

        // Insert an unconditional jump to the continue block at the end of the loop body
        builder.append_jump(false, continue_block);

        // Write the continue block
        builder.set_cursor(continue_block);
        builder
            .append_jump(true, loop_block)
            .with_node(node.test()?.into_syntax());

        builder.append_jump(false, break_block);

        // Set the cursor to the break block and move of to the next statement
        builder.set_cursor(break_block);

        Ok(())
    }
}
