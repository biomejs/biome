use biome_js_syntax::JsReturnStatement;
use biome_rowan::{AstNode, SyntaxResult};

use crate::{
    FunctionBuilder,
    visitor::{NodeVisitor, StatementStack},
};

pub(crate) struct ReturnVisitor;

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
