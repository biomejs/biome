use biome_rowan::{AstNode, SyntaxResult};
use rome_js_syntax::JsReturnStatement;

use crate::control_flow::{
    visitor::{NodeVisitor, StatementStack},
    FunctionBuilder,
};

pub(in crate::control_flow) struct ReturnVisitor;

impl NodeVisitor for ReturnVisitor {
    type Node = JsReturnStatement;

    fn enter(
        node: Self::Node,
        builder: &mut FunctionBuilder,
        _: StatementStack,
    ) -> SyntaxResult<Self> {
        builder.append_return().with_node(node.into_syntax());

        Ok(Self)
    }
}
