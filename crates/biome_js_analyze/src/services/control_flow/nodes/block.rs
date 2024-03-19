use biome_control_flow::builder::BlockId;
use biome_js_syntax::{JsBlockStatement, JsLabeledStatement, JsSyntaxToken};
use biome_rowan::{AstNode, SyntaxResult};

use crate::services::control_flow::{
    visitor::{NodeVisitor, StatementStack},
    FunctionBuilder,
};

pub(in crate::services::control_flow) struct BlockVisitor {
    /// If this block has a label, this contains the label token and the ID of
    /// the break block to use as a jump target in `BreakVisitor`
    pub(super) break_block: Option<(JsSyntaxToken, BlockId)>,
}

impl NodeVisitor for BlockVisitor {
    type Node = JsBlockStatement;

    fn enter(
        node: Self::Node,
        builder: &mut FunctionBuilder,
        _: StatementStack,
    ) -> SyntaxResult<Self> {
        let break_block = match node.parent::<JsLabeledStatement>() {
            Some(label) => {
                let label = label.label_token()?;
                let block = builder.append_block();
                Some((label, block))
            }
            None => None,
        };

        Ok(Self { break_block })
    }

    fn exit(
        self,
        _: Self::Node,
        builder: &mut FunctionBuilder,
        _: StatementStack,
    ) -> SyntaxResult<()> {
        if let Some((_, block)) = self.break_block {
            builder.append_jump(false, block);
            builder.set_cursor(block);
        }

        Ok(())
    }
}
