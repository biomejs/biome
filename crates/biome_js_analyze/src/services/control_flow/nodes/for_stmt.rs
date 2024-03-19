use biome_control_flow::builder::BlockId;
use biome_js_syntax::{JsForStatement, JsLabeledStatement, JsSyntaxToken};
use biome_rowan::{AstNode, SyntaxResult};

use crate::services::control_flow::{
    visitor::{NodeVisitor, StatementStack},
    FunctionBuilder,
};

pub(in crate::services::control_flow) struct ForVisitor {
    // `label`, `continue_block` and `break_block` are used by the
    // `ContinueVisitor` and `BreakVisitor`
    pub(super) label: Option<JsSyntaxToken>,
    pub(super) continue_block: BlockId,
    pub(super) break_block: BlockId,
    cond_block: BlockId,
    loop_block: BlockId,
}

impl NodeVisitor for ForVisitor {
    type Node = JsForStatement;

    fn enter(
        node: Self::Node,
        builder: &mut FunctionBuilder,
        _: StatementStack,
    ) -> SyntaxResult<Self> {
        // Immediately evaluate the initializer statement
        if let Some(initializer) = node.initializer() {
            builder
                .append_statement()
                .with_node(initializer.into_syntax());
        }

        // Create the condition block and unconditionally jump to it
        let cond_block = builder.append_block();
        builder.append_jump(false, cond_block);

        // Create the continue block and break block immediately
        let continue_block = builder.append_block();
        let break_block = builder.append_block();

        // Fill the continue block
        builder.set_cursor(continue_block);

        if let Some(update) = node.update() {
            builder.append_statement().with_node(update.into_syntax());
        }

        builder.append_jump(false, cond_block);

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
            cond_block,
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
            cond_block,
            loop_block,
            ..
        } = self;

        // Insert an unconditional jump to the continue block at the end of the loop body
        builder.append_jump(false, continue_block);

        // Write the condition block
        builder.set_cursor(cond_block);

        if let Some(test) = node.test() {
            builder
                .append_jump(true, loop_block)
                .with_node(test.syntax().clone());
        } else {
            builder.append_jump(false, loop_block);
        }

        builder.append_jump(false, break_block);

        // Set the cursor to the break block and move of to the next statement
        builder.set_cursor(break_block);

        Ok(())
    }
}
