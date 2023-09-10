use biome_rowan::{AstNode, SyntaxResult};
use rome_js_syntax::JsVariableStatement;

use crate::control_flow::{
    visitor::{NodeVisitor, StatementStack},
    FunctionBuilder,
};

pub(in crate::control_flow) struct VariableVisitor;

impl NodeVisitor for VariableVisitor {
    type Node = JsVariableStatement;

    fn enter(
        node: Self::Node,
        builder: &mut FunctionBuilder,
        _: StatementStack,
    ) -> SyntaxResult<Self> {
        let declaration = node.declaration()?;
        for declarator in declaration.declarators() {
            if let Some(initializer) = declarator?.initializer() {
                let expr = initializer.expression()?;
                builder.append_statement().with_node(expr.into_syntax());
            }
        }

        Ok(Self)
    }
}
