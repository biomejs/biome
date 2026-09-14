use biome_js_syntax::JsThrowStatement;
use biome_rowan::{AstNode, SyntaxResult};

use crate::{
    FunctionBuilder,
    visitor::{NodeVisitor, StatementStack},
};

pub(crate) struct ThrowVisitor;

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
