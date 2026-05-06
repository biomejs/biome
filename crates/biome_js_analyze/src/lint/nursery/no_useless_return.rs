use biome_analyze::{Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_js_syntax::{
    JsDoWhileStatement, JsFinallyClause, JsForInStatement, JsForOfStatement, JsForStatement,
    JsFunctionBody, JsReturnStatement, JsStatementList, JsSwitchStatement, JsWhileStatement,
};
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt};
use biome_rule_options::no_useless_return::NoUselessReturnOptions;

use crate::JsRuleAction;
use crate::services::control_flow::AnyJsControlFlowRoot;

declare_lint_rule! {
    /// Disallow redundant return statements.
    ///
    /// A `return;` statement with nothing after it is redundant when it is the
    /// last reachable statement in a function body. Removing it does not change
    /// the function's behavior, as execution naturally falls through to the end.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// function foo() {
    ///     return;
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function foo() {
    ///     doSomething();
    ///     return;
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function foo() {
    ///     if (condition) {
    ///         bar();
    ///         return;
    ///     }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// function foo() {
    ///     return 5;
    /// }
    /// ```
    ///
    /// ```js
    /// function foo() {
    ///     if (condition) {
    ///         return;
    ///     }
    ///     bar();
    /// }
    /// ```
    ///
    /// ```js
    /// function foo() {
    ///     for (const x of xs) {
    ///         return;
    ///     }
    /// }
    /// ```
    ///
    pub NoUselessReturn {
        version: "2.3.15",
        name: "noUselessReturn",
        language: "js",
        sources: &[RuleSource::Eslint("no-useless-return").inspired()],
        recommended: false,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for NoUselessReturn {
    type Query = Ast<JsReturnStatement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoUselessReturnOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let ret = ctx.query();

        // Skip returns with a value: `return expr;`
        if ret.argument().is_some() {
            return None;
        }

        // Find the enclosing function
        let function_root = ret
            .syntax()
            .ancestors()
            .find(|node| AnyJsControlFlowRoot::can_cast(node.kind()))?;

        // Bail if the control flow root is a module/script (top-level return)
        if biome_js_syntax::JsModule::can_cast(function_root.kind())
            || biome_js_syntax::JsScript::can_cast(function_root.kind())
        {
            return None;
        }

        // Bail if return is inside a loop or switch between it and the function root
        if is_inside_loop_or_switch(ret, &function_root) {
            return None;
        }

        // Check if the return is in tail position
        if is_tail_position(ret, &function_root) {
            return Some(());
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "This "<Emphasis>"return"</Emphasis>" statement is unnecessary."
                },
            )
            .note("Removing this statement does not change the control flow of the function."),
        )
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        mutation.remove_node(ctx.query().clone());
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the unnecessary "<Emphasis>"return"</Emphasis>" statement." }
                .to_owned(),
            mutation,
        ))
    }
}

/// Check if the return statement is inside a loop or switch statement
/// between the return and the enclosing function root.
fn is_inside_loop_or_switch(
    ret: &JsReturnStatement,
    function_root: &biome_rowan::SyntaxNode<biome_js_syntax::JsLanguage>,
) -> bool {
    for ancestor in ret.syntax().ancestors() {
        if &ancestor == function_root {
            break;
        }
        if JsForStatement::can_cast(ancestor.kind())
            || JsForInStatement::can_cast(ancestor.kind())
            || JsForOfStatement::can_cast(ancestor.kind())
            || JsWhileStatement::can_cast(ancestor.kind())
            || JsDoWhileStatement::can_cast(ancestor.kind())
            || JsSwitchStatement::can_cast(ancestor.kind())
        {
            return true;
        }
    }
    false
}

/// Check if the return statement is in tail position relative to the function root.
///
/// A return is in tail position if, walking from the return up to the function body,
/// every intermediate node allows the return to be the "last thing that happens":
/// - In a `JsStatementList`, the node must be the last element.
/// - Block statements, if/else, try/catch, and labeled statements are transparent.
/// - The function body itself confirms tail position.
/// - A `finally` clause conservatively bails out (return in finally has override semantics).
fn is_tail_position(
    ret: &JsReturnStatement,
    function_root: &biome_rowan::SyntaxNode<biome_js_syntax::JsLanguage>,
) -> bool {
    let mut current = ret.syntax().clone();

    loop {
        let Some(parent) = current.parent() else {
            return false;
        };

        // Reached the function body — we're in tail position
        if JsFunctionBody::can_cast(parent.kind()) {
            return true;
        }

        // Bail at the function root (shouldn't normally reach here without
        // hitting JsFunctionBody first, but be safe)
        if &parent == function_root {
            return false;
        }

        if JsStatementList::can_cast(parent.kind()) {
            // The current node must be the last element of the statement list
            let list = JsStatementList::cast(parent.clone()).unwrap();
            let last = list.iter().last();
            if let Some(last_stmt) = last {
                if last_stmt.syntax() != &current {
                    return false;
                }
            } else {
                return false;
            }
        } else if JsFinallyClause::can_cast(parent.kind()) {
            // Return in finally has override semantics, conservatively bail
            return false;
        } else if biome_js_syntax::JsBlockStatement::can_cast(parent.kind())
            || biome_js_syntax::JsIfStatement::can_cast(parent.kind())
            || biome_js_syntax::JsElseClause::can_cast(parent.kind())
            || biome_js_syntax::JsCatchClause::can_cast(parent.kind())
            || biome_js_syntax::JsTryStatement::can_cast(parent.kind())
            || biome_js_syntax::JsTryFinallyStatement::can_cast(parent.kind())
            || biome_js_syntax::JsLabeledStatement::can_cast(parent.kind())
        {
            // These are "transparent" — pass through
        } else {
            // Unknown/unsupported node kind — bail
            return false;
        }

        current = parent;
    }
}
