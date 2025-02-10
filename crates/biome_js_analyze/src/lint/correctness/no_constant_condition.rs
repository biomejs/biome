use crate::ast_utils::is_constant_condition;
use crate::services::semantic::Semantic;
use biome_analyze::{context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsExpression, AnyJsStatement, JsConditionalExpression, JsDoWhileStatement, JsForStatement,
    JsFunctionDeclaration, JsFunctionExpression, JsIfStatement, JsStatementList, JsSyntaxKind,
    JsWhileStatement, JsYieldExpression, TextRange,
};
use biome_rowan::{declare_node_union, AstNode};

declare_lint_rule! {
    /// Disallow constant expressions in conditions
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// if (false) {
    ///     doSomethingUnfinished();
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (Boolean(1)) {
    ///     doSomethingAlways();
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (undefined) {
    ///     doSomethingUnfinished();
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// for (;-2;) {
    ///     doSomethingForever();
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// while (typeof x) {
    ///     doSomethingForever();
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// var result = 0 ? a : b;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// if (x === 0) {
    ///     doSomething();
    /// }
    ///
    /// for (;;) {
    ///     doSomethingForever();
    /// }
    ///
    /// while (typeof x === "undefined") {
    ///     doSomething();
    /// }
    ///
    /// do {
    ///     doSomething();
    /// } while (x);
    ///
    /// var result = x !== 0 ? a : b;
    ///
    /// // Exception
    /// while (true) {
    ///     if (x) { break; }
    ///     x = f();
    /// }
    /// ```
    ///
    pub NoConstantCondition    {
        version: "1.0.0",
        name: "noConstantCondition",
        language: "js",
        sources: &[RuleSource::Eslint("no-constant-condition")],
        recommended: true,
        severity: Severity::Error,
    }
}

declare_node_union! {
    pub ConditionalStatement = JsConditionalExpression | JsWhileStatement | JsDoWhileStatement | JsIfStatement | JsForStatement
}

impl Rule for NoConstantCondition {
    type Query = Semantic<ConditionalStatement>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let conditional_stmt = ctx.query();
        let model = ctx.model();

        // We must verify that the conditional statement is within a generator function.
        // If the statement contains a valid yield expression returned from a `while`, `for`, or `do...while` statement,
        // we don't need to examine the statement's `test`.
        if let Some(any_js_stmt) = conditional_stmt.body() {
            if conditional_stmt.is_in_generator_function()
                && has_valid_yield_expression(&any_js_stmt).unwrap_or(false)
            {
                return None;
            }
        }

        let test = conditional_stmt.test()?;
        // Ignore `while (true) { ... }`
        if matches!(conditional_stmt, ConditionalStatement::JsWhileStatement(_))
            && test
                .as_any_js_literal_expression()
                .and_then(|test| test.as_js_boolean_literal_expression())
                .and_then(|test| Some(test.value_token().ok()?.kind() == JsSyntaxKind::TRUE_KW))
                .unwrap_or_default()
        {
            return None;
        }
        let test_range = test.range();
        is_constant_condition(test, true, model).map(|_| test_range)
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            state,
            markup! {
                "Unexpected constant condition."
            },
        ))
    }
}

impl ConditionalStatement {
    fn test(&self) -> Option<AnyJsExpression> {
        match self {
            Self::JsConditionalExpression(it) => it.test().ok(),
            Self::JsWhileStatement(it) => it.test().ok(),
            Self::JsDoWhileStatement(it) => it.test().ok(),
            Self::JsIfStatement(it) => it.test().ok(),
            Self::JsForStatement(it) => it.test(),
        }
    }
    fn body(&self) -> Option<AnyJsStatement> {
        match self {
            Self::JsWhileStatement(it) => it.body().ok(),
            Self::JsDoWhileStatement(it) => it.body().ok(),
            Self::JsForStatement(it) => it.body().ok(),
            _ => None,
        }
    }
    // Checks if the self statement is in a generator function
    fn is_in_generator_function(&self) -> bool {
        self.syntax().ancestors().any(|node| {
            match JsFunctionDeclaration::try_cast(node) {
                Ok(func_decl) => func_decl.star_token(),
                Err(node) => {
                    JsFunctionExpression::cast(node).and_then(|func_expr| func_expr.star_token())
                }
            }
            .is_some()
        })
    }
}

impl From<AnyJsStatement> for ConditionalStatement {
    fn from(node: AnyJsStatement) -> Self {
        match node {
            AnyJsStatement::JsWhileStatement(it) => Self::JsWhileStatement(it),
            AnyJsStatement::JsDoWhileStatement(it) => Self::JsDoWhileStatement(it),
            AnyJsStatement::JsIfStatement(it) => Self::JsIfStatement(it),
            AnyJsStatement::JsForStatement(it) => Self::JsForStatement(it),
            _ => unreachable!(),
        }
    }
}

// Gets a yield expression from the given statement
fn get_yield_expression(stmt: &AnyJsStatement) -> Option<JsYieldExpression> {
    let stmt = stmt.as_js_expression_statement()?;
    let Ok(AnyJsExpression::JsYieldExpression(expr)) = stmt.as_fields().expression else {
        return None;
    };
    Some(expr)
}

fn get_statement_list(stmt: &AnyJsStatement) -> Option<JsStatementList> {
    Some(stmt.as_js_block_statement()?.as_fields().statements)
}

/// Checks if a given statement can return valid yield expression
fn has_valid_yield_expression(stmt: &AnyJsStatement) -> Option<bool> {
    let mut stmt_list = get_statement_list(stmt)?.into_iter();

    loop {
        match stmt_list.next() {
            Some(first_stmt) => {
                if get_yield_expression(&first_stmt).is_some()
                    || stmt_list.any(|stmt| get_yield_expression(&stmt).is_some())
                {
                    return Some(true);
                } else {
                    // We need to examine `while`, `do...while`, and `for` statements more closely,
                    // as there are cases where a yield expression is correctly returned even with nested loops.
                    match first_stmt {
                        AnyJsStatement::JsWhileStatement(stmt) => {
                            stmt_list = get_statement_list(&stmt.body().ok()?)?.into_iter();
                        }
                        AnyJsStatement::JsDoWhileStatement(stmt) => {
                            stmt_list = get_statement_list(&stmt.body().ok()?)?.into_iter();
                        }
                        AnyJsStatement::JsForStatement(stmt) => {
                            stmt_list = get_statement_list(&stmt.body().ok()?)?.into_iter();
                        }
                        _ => return None,
                    }
                }
            }
            None => return None,
        }
    }
}
