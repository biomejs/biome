use biome_analyze::{Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_js_syntax::{
    JsDoWhileStatement, JsElseClause, JsFinallyClause, JsForInStatement, JsForOfStatement,
    JsForStatement, JsFunctionBody, JsIfStatement, JsLanguage, JsReturnStatement, JsStatementList,
    JsSwitchStatement, JsWhileStatement,
};
use biome_languages::JsFileSource;
use biome_rowan::{
    AstNode, AstNodeList, BatchMutation, BatchMutationExt, SyntaxNode, chain_trivia_pieces,
};
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

        // Skip a `return;` that is the entire body of an unbraced `if` or
        // `else`: removing it alone would leave `if (cond)` or `else` with no
        // consequent, which is a syntax error.
        if is_unbraced_if_body(ret) {
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
            // In TypeScript, a trailing `return;` is load-bearing when another
            // code path returns a value: removing it breaks
            // `tsc --noImplicitReturns` (TS7030).
            if ctx.source_type::<JsFileSource>().is_typescript()
                && has_valued_return(&function_root, ret)
            {
                return None;
            }
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
        remove_return_preserving_comments(&mut mutation, ctx.query());
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the unnecessary "<Emphasis>"return"</Emphasis>" statement." }
                .to_owned(),
            mutation,
        ))
    }
}

/// Remove the `return` statement, preserving its leading comments.
///
/// The comments are transferred to the next token's leading trivia. Trailing
/// whitespace is trimmed, and a trailing newline is only kept when the next
/// token doesn't already start on a new line, so the fixed output stays
/// clean and the next token is never commented out. When the statement has
/// no comments, this is equivalent to a plain removal.
fn remove_return_preserving_comments(
    mutation: &mut BatchMutation<JsLanguage>,
    ret: &JsReturnStatement,
) {
    if let (Some(next_token), Some(leading)) = (
        ret.syntax()
            .last_token()
            .and_then(|token| token.next_token()),
        ret.syntax().first_leading_trivia(),
    ) {
        let mut pieces: Vec<_> = leading.pieces().collect();

        // Trim trailing whitespace; newlines are handled below.
        while pieces.last().is_some_and(|piece| piece.is_whitespace()) {
            pieces.pop();
        }

        // Only transfer trivia when there are comments to preserve;
        // otherwise a plain removal keeps the output clean.
        if pieces.iter().any(|piece| piece.is_comments()) {
            let next_starts_with_newline = next_token
                .leading_trivia()
                .pieces()
                .next()
                .is_some_and(|piece| piece.is_newline());

            // If the next token already starts on a new line, drop our own
            // trailing newlines to avoid a blank line.
            if next_starts_with_newline {
                while pieces.last().is_some_and(|piece| piece.is_newline()) {
                    pieces.pop();
                }
            }

            let new_token = next_token.with_leading_trivia_pieces(chain_trivia_pieces(
                pieces.into_iter(),
                next_token.leading_trivia().pieces(),
            ));
            mutation.replace_token_discard_trivia(next_token, new_token);
        }
    }

    mutation.remove_node(ret.clone());
}

/// Check whether the `return` statement is the entire consequent or alternate
/// of an `if` statement without braces.
///
/// Removing just the `return` would leave `if (cond)` or `else` with no
/// statement, which is a syntax error, so the rule stays silent.
fn is_unbraced_if_body(ret: &JsReturnStatement) -> bool {
    let Some(parent) = ret.syntax().parent() else {
        return false;
    };
    JsIfStatement::can_cast(parent.kind()) || JsElseClause::can_cast(parent.kind())
}

/// Check whether the function contains another `return` statement that returns
/// a value (`return expr;`) in the same control flow root.
///
/// With TypeScript's `noImplicitReturns`, a trailing bare `return;` keeps the
/// compiler satisfied when another code path returns a value (TS7030), so the
/// safe fix must not remove it.
fn has_valued_return(function_root: &SyntaxNode<JsLanguage>, ret: &JsReturnStatement) -> bool {
    function_root.descendants().any(|node| {
        let Some(candidate) = JsReturnStatement::cast(node) else {
            return false;
        };
        if candidate.syntax() == ret.syntax() || candidate.argument().is_none() {
            return false;
        }
        // Ignore returns that belong to a nested function or other nested
        // control flow root; only the current function's paths matter.
        for ancestor in candidate.syntax().ancestors() {
            if AnyJsControlFlowRoot::can_cast(ancestor.kind()) {
                return &ancestor == function_root;
            }
        }
        false
    })
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
