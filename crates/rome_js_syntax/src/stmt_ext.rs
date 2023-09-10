//! Extended AST node definitions for statements which are unique and special enough to generate code for manually

use crate::{
    AnyJsArrayAssignmentPatternElement, AnyJsAssignmentPattern, AnyJsSwitchClause,
    JsForVariableDeclaration, JsStatementList, JsSyntaxToken as SyntaxToken, JsVariableDeclaration,
    TsModuleDeclaration, T,
};
use biome_rowan::{declare_node_union, SyntaxResult};

impl AnyJsSwitchClause {
    pub fn clause_token(&self) -> SyntaxResult<SyntaxToken> {
        match &self {
            AnyJsSwitchClause::JsCaseClause(item) => item.case_token(),
            AnyJsSwitchClause::JsDefaultClause(item) => item.default_token(),
        }
    }

    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        match &self {
            AnyJsSwitchClause::JsCaseClause(item) => item.colon_token(),
            AnyJsSwitchClause::JsDefaultClause(item) => item.colon_token(),
        }
    }

    pub fn consequent(&self) -> JsStatementList {
        match &self {
            AnyJsSwitchClause::JsCaseClause(item) => item.consequent(),
            AnyJsSwitchClause::JsDefaultClause(item) => item.consequent(),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum JsVariableKind {
    Const,
    Let,
    Var,
    Using,
}

impl JsVariableDeclaration {
    /// Whether the declaration is a const declaration
    pub fn is_const(&self) -> bool {
        self.variable_kind() == Ok(JsVariableKind::Const)
    }

    /// Whether the declaration is a let declaration
    pub fn is_let(&self) -> bool {
        self.variable_kind() == Ok(JsVariableKind::Let)
    }

    /// Whether the declaration is a var declaration
    pub fn is_var(&self) -> bool {
        self.variable_kind() == Ok(JsVariableKind::Var)
    }

    pub fn variable_kind(&self) -> SyntaxResult<JsVariableKind> {
        let token_kind = self.kind().map(|t| t.kind())?;

        Ok(match token_kind {
            T![const] => JsVariableKind::Const,
            T![let] => JsVariableKind::Let,
            T![var] => JsVariableKind::Var,
            T![using] => JsVariableKind::Using,
            _ => unreachable!(),
        })
    }
}

impl JsForVariableDeclaration {
    /// Whether the declaration is a const declaration
    pub fn is_const(&self) -> bool {
        self.variable_kind() == Ok(JsVariableKind::Const)
    }

    /// Whether the declaration is a let declaration
    pub fn is_let(&self) -> bool {
        self.variable_kind() == Ok(JsVariableKind::Let)
    }

    /// Whether the declaration is a var declaration
    pub fn is_var(&self) -> bool {
        self.variable_kind() == Ok(JsVariableKind::Var)
    }

    pub fn variable_kind(&self) -> SyntaxResult<JsVariableKind> {
        let token_kind = self.kind_token().map(|t| t.kind())?;

        Ok(match token_kind {
            T![const] => JsVariableKind::Const,
            T![let] => JsVariableKind::Let,
            T![var] => JsVariableKind::Var,
            T![using] => JsVariableKind::Using,
            _ => unreachable!(),
        })
    }
}

declare_node_union! {
    pub AnyJsVariableDeclaration = JsVariableDeclaration | JsForVariableDeclaration
}

impl AnyJsVariableDeclaration {
    pub fn variable_kind(&self) -> SyntaxResult<JsVariableKind> {
        match self {
            AnyJsVariableDeclaration::JsForVariableDeclaration(decl) => decl.variable_kind(),
            AnyJsVariableDeclaration::JsVariableDeclaration(decl) => decl.variable_kind(),
        }
    }
}

impl AnyJsArrayAssignmentPatternElement {
    pub fn pattern(self) -> Option<AnyJsAssignmentPattern> {
        match self {
            Self::AnyJsAssignmentPattern(p) => Some(p),
            Self::JsArrayAssignmentPatternRestElement(p) => p.pattern().ok(),
            Self::JsAssignmentWithDefault(p) => p.pattern().ok(),
            Self::JsArrayHole(_) => None,
        }
    }
}

impl TsModuleDeclaration {
    pub fn is_module(&self) -> SyntaxResult<bool> {
        Ok(self.module_or_namespace()?.kind() == T![module])
    }

    pub fn is_namespace(&self) -> SyntaxResult<bool> {
        Ok(self.module_or_namespace()?.kind() == T![namespace])
    }
}

#[cfg(test)]
mod tests {
    use biome_rowan::AstNode;
    use rome_js_factory::syntax::{JsSyntaxKind::*, JsVariableDeclaration};
    use rome_js_factory::JsSyntaxTreeBuilder;

    #[test]
    fn is_var_check() {
        let mut tree_builder = JsSyntaxTreeBuilder::new();
        tree_builder.start_node(JS_VARIABLE_DECLARATION);
        tree_builder.token(VAR_KW, "var");
        tree_builder.start_node(JS_VARIABLE_DECLARATOR_LIST);
        tree_builder.start_node(JS_VARIABLE_DECLARATOR);

        tree_builder.start_node(JS_IDENTIFIER_BINDING);
        tree_builder.token(IDENT, "a");
        tree_builder.finish_node();

        tree_builder.finish_node(); // declarator
        tree_builder.finish_node(); // list
        tree_builder.finish_node(); // declaration

        let root = tree_builder.finish();

        let var_decl = JsVariableDeclaration::cast(root).unwrap();

        assert!(var_decl.is_var());
    }
}
