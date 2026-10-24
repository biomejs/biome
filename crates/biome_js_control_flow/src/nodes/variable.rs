use biome_js_syntax::{AnyJsBindingPattern, JsVariableStatement};
use biome_rowan::{AstNode, SyntaxResult};

use crate::{
    FunctionBuilder,
    visitor::{NodeVisitor, StatementStack},
};

pub(crate) struct VariableVisitor;

impl NodeVisitor for VariableVisitor {
    type Node = JsVariableStatement;

    fn enter(
        node: Self::Node,
        builder: &mut FunctionBuilder,
        _: StatementStack,
    ) -> SyntaxResult<Self> {
        let declaration = node.declaration()?;
        for declarator in declaration.declarators() {
            let declarator = declarator?;
            if let Some(initializer) = declarator.initializer() {
                let expr = initializer.expression()?;
                builder.append_statement().with_node(expr.into_syntax());
                // Default values of a destructuring pattern run after the initializer
                let id = declarator.id()?;
                if !matches!(id, AnyJsBindingPattern::AnyJsBinding(_)) {
                    builder.append_statement().with_node(id.into_syntax());
                }
            }
        }

        Ok(Self)
    }
}
