use biome_js_syntax::{JsDebuggerStatement, JsEmptyStatement, JsExpressionStatement};
use biome_rowan::{AstNode, SyntaxResult, declare_node_union};

use crate::{
    FunctionBuilder,
    visitor::{NodeVisitor, StatementStack},
};

declare_node_union! {
    pub(crate) JsSimpleStatement = JsDebuggerStatement | JsEmptyStatement | JsExpressionStatement
}

pub(crate) struct StatementVisitor;

impl NodeVisitor for StatementVisitor {
    type Node = JsSimpleStatement;

    fn enter(
        node: Self::Node,
        builder: &mut FunctionBuilder,
        _: StatementStack,
    ) -> SyntaxResult<Self> {
        builder.append_statement().with_node(node.into_syntax());

        Ok(Self)
    }
}
