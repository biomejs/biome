use biome_control_flow::builder::BlockId;
use biome_js_syntax::{JsDoWhileStatement, JsLabeledStatement, JsSyntaxToken};
use biome_rowan::{AstNode, SyntaxResult};

use crate::services::control_flow::{
    visitor::{NodeVisitor, StatementStack},
    FunctionBuilder,
};

pub(in crate::services::control_flow) struct DoWhileVisitor {
    // `label`, `continue_block` and `break_block` are used by the
    // `ContinueVisitor` and `BreakVisitor`
    pub(super) label: Option<JsSyntaxToken>,
    pub(super) continue_block: BlockId,
    pub(super) break_block: BlockId,
    body_block: BlockId,
}

impl NodeVisitor for DoWhileVisitor {
    type Node = JsDoWhileStatement;

    fn enter(
        node: Self::Node,
        builder: &mut FunctionBuilder,
        _: StatementStack,
    ) -> SyntaxResult<Self> {
        let body_block = builder.append_block();

        // Unconditionally jump into the loop
        builder.append_jump(false, body_block);

        let continue_block = builder.append_block();
        let break_block = builder.append_block();

        let label = node
            .parent::<JsLabeledStatement>()
            .and_then(|label| label.label_token().ok());

        // Fill the body block
        builder.set_cursor(body_block);

        Ok(Self {
            label,
            continue_block,
            break_block,
            body_block,
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
            body_block,
            ..
        } = self;

        // Insert an implicit jump to the continue block at the end of the loop
        builder.append_jump(false, continue_block);

        // Fill the continue block
        builder.set_cursor(continue_block);
        builder
            .append_jump(true, body_block)
            .with_node(node.test()?.into_syntax());

        builder.append_jump(false, break_block);

        // Set the cursor to the break block and move to the next statement
        builder.set_cursor(break_block);

        Ok(())
    }
}
