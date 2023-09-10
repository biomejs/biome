use biome_rowan::{AstNode, SyntaxResult};
use rome_js_syntax::JsThrowStatement;

use crate::control_flow::{
    visitor::{NodeVisitor, StatementStack},
    FunctionBuilder,
};

pub(in crate::control_flow) struct ThrowVisitor;

impl NodeVisitor for ThrowVisitor {
    type Node = JsThrowStatement;

    fn enter(
        node: Self::Node,
        builder: &mut FunctionBuilder,
        _: StatementStack,
    ) -> SyntaxResult<Self> {
        builder.append_return().with_node(node.into_syntax());

        Ok(Self)
    }
}
