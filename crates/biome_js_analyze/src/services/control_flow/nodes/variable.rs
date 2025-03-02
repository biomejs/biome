use biome_js_syntax::JsVariableStatement;
use biome_rowan::{AstNode, SyntaxResult};

use crate::services::control_flow::{
    FunctionBuilder,
    visitor::{NodeVisitor, StatementStack},
};

pub(in crate::services::control_flow) struct VariableVisitor;

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
