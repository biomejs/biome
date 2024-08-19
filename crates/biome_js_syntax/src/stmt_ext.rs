//! Extended AST node definitions for statements which are unique and special enough to generate code for manually

use crate::{
    AnyJsArrayAssignmentPatternElement, AnyJsAssignmentPattern, AnyJsSwitchClause,
    JsBlockStatement, JsBreakStatement, JsCatchClause, JsContinueStatement, JsFinallyClause,
    JsForVariableDeclaration, JsLabeledStatement, JsStatementList, JsSyntaxKind,
    JsSyntaxToken as SyntaxToken, JsTryFinallyStatement, JsTryStatement, JsVariableDeclaration,
    JsVariableDeclarator, TsModuleDeclaration, T,
};
use biome_rowan::{declare_node_union, AstNode, SyntaxResult};

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

declare_node_union! {
    pub AnyJsTryStatement = JsTryStatement | JsTryFinallyStatement
}

impl AnyJsTryStatement {
    pub fn try_token(&self) -> SyntaxResult<SyntaxToken> {
        match self {
            Self::JsTryStatement(node) => node.try_token(),
            Self::JsTryFinallyStatement(node) => node.try_token(),
        }
    }

    pub fn body(&self) -> SyntaxResult<JsBlockStatement> {
        match self {
            Self::JsTryStatement(node) => node.body(),
            Self::JsTryFinallyStatement(node) => node.body(),
        }
    }

    pub fn catch_clause(&self) -> Option<JsCatchClause> {
        match self {
            Self::JsTryStatement(node) => node.catch_clause().ok(),
            Self::JsTryFinallyStatement(node) => node.catch_clause(),
        }
    }

    pub fn finally_clause(&self) -> Option<JsFinallyClause> {
        match self {
            Self::JsTryStatement(_) => None,
            Self::JsTryFinallyStatement(node) => node.finally_clause().ok(),
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
        let kind_token = self.kind()?;
        Ok(match kind_token.kind() {
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
        let kind_token = self.kind_token()?;
        Ok(match kind_token.kind() {
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
        match self {
            AnyJsVariableDeclaration::JsForVariableDeclaration(decl) => decl.variable_kind(),
            AnyJsVariableDeclaration::JsVariableDeclaration(decl) => decl.variable_kind(),
        }
    }

    pub fn kind_token(&self) -> SyntaxResult<SyntaxToken> {
        match self {
            AnyJsVariableDeclaration::JsVariableDeclaration(x) => x.kind(),
            AnyJsVariableDeclaration::JsForVariableDeclaration(x) => x.kind_token(),
        }
    }
}

impl JsVariableDeclarator {
    /// Variable declaration associated to this declarator.
    pub fn declaration(&self) -> Option<AnyJsVariableDeclaration> {
        self.syntax()
            .ancestors()
            .skip(1)
            .find(|x| x.kind() != JsSyntaxKind::JS_VARIABLE_DECLARATOR_LIST)
            .and_then(AnyJsVariableDeclaration::cast)
    }
}

impl AnyJsArrayAssignmentPatternElement {
    pub fn pattern(self) -> Option<AnyJsAssignmentPattern> {
        match self {
            Self::JsArrayAssignmentPatternElement(p) => p.pattern().ok(),
            Self::JsArrayAssignmentPatternRestElement(p) => p.pattern().ok(),
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

impl JsLabeledStatement {
    pub fn label_token(&self) -> SyntaxResult<SyntaxToken> {
        self.label()?.value_token()
    }
}

impl JsBreakStatement {
    pub fn label_token(&self) -> Option<SyntaxToken> {
        self.label()?.value_token().ok()
    }
}

impl JsContinueStatement {
    pub fn label_token(&self) -> Option<SyntaxToken> {
        self.label()?.value_token().ok()
    }
}

#[cfg(test)]
mod tests {
    use biome_js_factory::syntax::{JsSyntaxKind::*, JsVariableDeclaration};
    use biome_js_factory::JsSyntaxTreeBuilder;
    use biome_rowan::AstNode;

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
