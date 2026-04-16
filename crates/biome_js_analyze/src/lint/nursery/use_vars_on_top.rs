use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsModuleItem, AnyJsStatement, JsExport, JsExpressionStatement, JsModule, JsModuleItemList,
    JsStatementList, JsSyntaxKind, JsVariableDeclaration, JsVariableDeclarationClause,
    JsVariableStatement,
};
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::use_vars_on_top::UseVarsOnTopOptions;

declare_lint_rule! {
    /// Require `var` declarations to appear at the top of their containing scope.
    ///
    /// Because `var` declarations are hoisted to the top of the nearest function,
    /// script, module, or static block, placing them later in the body makes code
    /// harder to follow. Keeping them at the top makes the scope's variable
    /// declarations easier to find. Note that this is not a problem for `let` and
    /// `const` declarations, which are block-scoped and not hoisted.
    ///
    /// This rule only allows leading standalone `var` statements. At module
    /// scope, leading `export var` declarations are allowed too. Directives and
    /// imports may appear before them.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// function f() {
    ///     doSomething();
    ///     var value = 1;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// function f() {
    ///     var value = 1;
    ///     doSomething(value);
    /// }
    /// ```
    ///
    /// Related:
    /// - [noVar](https://biomejs.dev/linter/rules/no-var/)
    /// - [useConst](https://biomejs.dev/linter/rules/use-const/)
    pub UseVarsOnTop {
        version: "2.4.12",
        name: "useVarsOnTop",
        language: "js",
        recommended: false,
        sources: &[RuleSource::Eslint("vars-on-top").same()],
        severity: Severity::Warning,
    }
}

impl Rule for UseVarsOnTop {
    type Query = Ast<JsVariableDeclaration>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = UseVarsOnTopOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let declaration = ctx.query();

        if !declaration.is_var() || is_var_on_top(declaration).unwrap_or_default() {
            return None;
        }

        Some(declaration.range())
    }

    fn diagnostic(_ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                *range,
                markup! {
                    "This "<Emphasis>"var"</Emphasis>" declaration is not at the top of its containing scope."
                },
            )
            .note(markup! {
                <Emphasis>"var"</Emphasis>" declarations are hoisted to the top of their enclosing function, script, module, or static block, so declaring them later makes the scope harder to read."
            })
            .note(markup! {
                "Move this "<Emphasis>"var"</Emphasis>" declaration before other statements in the same scope. At module scope, imports may remain before it."
            }),
        )
    }
}

fn is_var_on_top(declaration: &JsVariableDeclaration) -> Option<bool> {
    let parent = declaration.syntax().parent()?;

    if let Some(statement) = JsVariableStatement::cast(parent.clone()) {
        return is_top_level_var_statement(&statement);
    }

    if let Some(clause) = JsVariableDeclarationClause::cast(parent.clone()) {
        let parent = clause.syntax().parent()?;

        if matches!(
            parent.kind(),
            JsSyntaxKind::TS_DECLARE_STATEMENT | JsSyntaxKind::TS_EXPORT_DECLARE_CLAUSE
        ) {
            return Some(true);
        }

        if let Some(export) = JsExport::cast(parent) {
            return is_top_level_var_export(&export);
        }
    }

    None
}

fn is_top_level_var_statement(statement: &JsVariableStatement) -> Option<bool> {
    let parent = statement.syntax().parent()?;

    if let Some(statements) = JsStatementList::cast(parent.clone()) {
        let owner = statements.syntax().parent()?;

        if !matches!(
            owner.kind(),
            JsSyntaxKind::JS_FUNCTION_BODY
                | JsSyntaxKind::JS_SCRIPT
                | JsSyntaxKind::JS_STATIC_INITIALIZATION_BLOCK_CLASS_MEMBER
        ) {
            return None;
        }

        return Some(is_top_statement_in_list(&statements, statement.syntax()));
    }

    if let Some(items) = JsModuleItemList::cast(parent) {
        let owner = items.syntax().parent()?;

        if !JsModule::can_cast(owner.kind()) {
            return None;
        }

        return Some(is_top_module_item_in_list(&items, statement.syntax()));
    }

    None
}

fn is_top_level_var_export(export: &JsExport) -> Option<bool> {
    let parent = export.syntax().parent()?;

    let items = JsModuleItemList::cast(parent)?;
    let owner = items.syntax().parent()?;

    if !JsModule::can_cast(owner.kind()) {
        return None;
    }

    Some(is_top_module_item_in_list(&items, export.syntax()))
}

fn is_top_statement_in_list(
    statements: &JsStatementList,
    target: &biome_js_syntax::JsSyntaxNode,
) -> bool {
    for statement in statements {
        if statement.syntax() == target {
            return true;
        }

        if !is_allowed_leading_statement(&statement) {
            return false;
        }
    }

    false
}

fn is_top_module_item_in_list(
    items: &JsModuleItemList,
    target: &biome_js_syntax::JsSyntaxNode,
) -> bool {
    for item in items {
        if item.syntax() == target {
            return true;
        }

        let is_allowed = match item {
            AnyJsModuleItem::JsImport(_) => true,
            AnyJsModuleItem::AnyJsStatement(statement) => is_allowed_leading_statement(&statement),
            AnyJsModuleItem::JsExport(export) => is_variable_export(&export),
        };

        if !is_allowed {
            return false;
        }
    }

    false
}

fn is_allowed_leading_statement(statement: &AnyJsStatement) -> bool {
    is_directive_statement(statement) || is_variable_statement(statement)
}

fn is_directive_statement(statement: &AnyJsStatement) -> bool {
    statement
        .as_js_expression_statement()
        .is_some_and(looks_like_directive)
}

fn looks_like_directive(statement: &JsExpressionStatement) -> bool {
    statement
        .expression()
        .ok()
        .and_then(|expression| expression.as_any_js_literal_expression().cloned())
        .is_some_and(|expression| expression.as_js_string_literal_expression().is_some())
}

fn is_variable_statement(statement: &AnyJsStatement) -> bool {
    matches!(
        statement,
        AnyJsStatement::JsVariableStatement(statement) if statement.declaration().is_ok()
    )
}

fn is_variable_export(export: &JsExport) -> bool {
    export
        .export_clause()
        .ok()
        .and_then(|clause| clause.as_any_js_declaration_clause().cloned())
        .and_then(|clause| clause.as_js_variable_declaration_clause().cloned())
        .and_then(|clause| clause.declaration().ok())
        .is_some()
}
