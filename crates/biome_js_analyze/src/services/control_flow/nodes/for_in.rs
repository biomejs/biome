use biome_control_flow::builder::BlockId;
use biome_js_syntax::{JsForInStatement, JsLabeledStatement, JsSyntaxToken};
use biome_rowan::{AstNode, SyntaxResult};

use crate::services::control_flow::{
    visitor::{NodeVisitor, StatementStack},
    FunctionBuilder,
};

pub(in crate::services::control_flow) struct ForInVisitor {
    // `label`, `continue_block` and `break_block` are used by the
    // `ContinueVisitor` and `BreakVisitor`
    pub(super) label: Option<JsSyntaxToken>,
    pub(super) continue_block: BlockId,
    pub(super) break_block: BlockId,
}

impl NodeVisitor for ForInVisitor {
    type Node = JsForInStatement;

    fn enter(
        node: Self::Node,
        builder: &mut FunctionBuilder,
        _: StatementStack,
    ) -> SyntaxResult<Self> {
        let continue_block = builder.append_block();
        let loop_block = builder.append_block();
        let break_block = builder.append_block();

        builder.append_jump(false, continue_block);

        builder.set_cursor(continue_block);
        builder
            .append_jump(true, loop_block)
            .with_node(node.initializer()?.into_syntax());

        builder.append_jump(false, break_block);

        let label = node
            .parent::<JsLabeledStatement>()
            .and_then(|label| label.label_token().ok());

        builder.set_cursor(loop_block);

        Ok(Self {
            label,
            continue_block,
            break_block,
        })
    }

    fn exit(
        self,
        _: Self::Node,
        builder: &mut FunctionBuilder,
        _: StatementStack,
    ) -> SyntaxResult<()> {
        let Self {
            continue_block,
            break_block,
            ..
        } = self;

        builder.append_jump(false, continue_block);

        builder.set_cursor(break_block);

        Ok(())
    }
}
