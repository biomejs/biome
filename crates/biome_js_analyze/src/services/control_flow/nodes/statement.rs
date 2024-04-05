use biome_js_syntax::{JsDebuggerStatement, JsEmptyStatement, JsExpressionStatement};
use biome_rowan::{declare_node_union, AstNode, SyntaxResult};

use crate::services::control_flow::{
    visitor::{NodeVisitor, StatementStack},
    FunctionBuilder,
};

declare_node_union! {
    pub(in crate::services::control_flow) JsSimpleStatement = JsDebuggerStatement | JsEmptyStatement | JsExpressionStatement
}

pub(in crate::services::control_flow) struct StatementVisitor;

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
