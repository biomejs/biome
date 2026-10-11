use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, RuleSuppressions, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsExpression, JsDoWhileStatement, JsElseClause, JsForInStatement, JsForOfStatement,
    JsForStatement, JsIfStatement, JsLanguage, JsSwitchStatement, JsTryFinallyStatement,
    JsTryStatement, JsWhileStatement, JsWithStatement,
};
use biome_rowan::{AstNode, WalkEvent, declare_node_union};
use biome_rule_options::no_excessive_nested_blocks::NoExcessiveNestedBlocksOptions;

use crate::ast_utils::is_function_boundary;

declare_lint_rule! {
    /// Enforce a maximum depth that blocks can be nested.
    ///
    /// Deeply nested code is difficult to read and follow, because every level adds another
    /// condition the reader has to keep in mind. This rule reports `if`, `switch`, `try`, and
    /// loop statements nested beyond the configured limit.
    ///
    /// An `else if` doesn't add a level of nesting. Each function, method, and `static {}` block
    /// in a class starts counting from zero again.
    ///
    /// Only the outermost statement exceeding the limit is reported: statements nested inside it
    /// aren't reported again.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// function foo() {
    ///     for (const item of items) {
    ///         if (item.enabled) {
    ///             while (item.next()) {
    ///                 try {
    ///                     if (item.done) {
    ///                         break;
    ///                     }
    ///                 } catch {}
    ///             }
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// function foo() {
    ///     for (const item of items) {
    ///         if (!item.enabled) {
    ///             continue;
    ///         }
    ///         processItem(item);
    ///     }
    /// }
    ///
    /// function processItem(item) {
    ///     while (item.next()) {
    ///         try {
    ///             if (item.done) {
    ///                 break;
    ///             }
    ///         } catch {}
    ///     }
    /// }
    ///
    /// if (a) {
    ///     if (b) {
    ///         if (c) {
    ///             if (d) {
    ///             } else if (e) {
    ///             } else if (f) {
    ///             }
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// ## Options
    ///
    /// ### max
    ///
    /// Default: 4
    ///
    /// The maximum block nesting depth allowed.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "max": 2
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```js,use_options,expect_diagnostic
    /// if (a) {
    ///     if (b) {
    ///         if (c) {}
    ///     }
    /// }
    /// ```
    ///
    /// #### Valid
    ///
    /// ```js,use_options
    /// if (a) {
    ///     if (b) {}
    /// }
    /// ```
    ///
    pub NoExcessiveNestedBlocks {
        version: "next",
        name: "noExcessiveNestedBlocks",
        language: "js",
        sources: &[RuleSource::Eslint("max-depth").same()],
        severity: Severity::Warning,
        recommended: false,
    }
}

declare_node_union! {
    pub AnyJsNestingStatement =
        JsIfStatement
        | JsSwitchStatement
        | JsTryStatement
        | JsTryFinallyStatement
        | JsDoWhileStatement
        | JsWhileStatement
        | JsWithStatement
        | JsForStatement
        | JsForInStatement
        | JsForOfStatement
}

impl Rule for NoExcessiveNestedBlocks {
    type Query = Ast<AnyJsNestingStatement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoExcessiveNestedBlocksOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let statement = ctx.query();

        if !increases_depth(statement) {
            return None;
        }

        // Only check whether the depth exceeds the maximum, so stop counting at `max + 1`.
        // Statements nested inside a reported one are suppressed and never reach this point,
        // which means a reported statement is always exactly `max + 1` levels deep.
        let max = usize::from(ctx.options().max());
        statement
            .syntax()
            .ancestors()
            .take_while(|ancestor| !is_function_boundary(ancestor.kind()))
            .filter_map(AnyJsNestingStatement::cast)
            .filter(increases_depth)
            .nth(max)
            .map(|_| ())
    }

    fn suppressed_nodes(
        ctx: &RuleContext<Self>,
        _state: &Self::State,
        suppressions: &mut RuleSuppressions<JsLanguage>,
    ) {
        // Every statement nested inside the reported one is nested even deeper, so only the
        // outermost one is reported. Nested functions start counting from zero again, so the
        // statements they contain are checked on their own.
        let mut preorder = ctx.query().syntax().preorder();
        // Skip the reported statement itself.
        preorder.next();

        while let Some(event) = preorder.next() {
            let WalkEvent::Enter(node) = event else {
                continue;
            };
            let kind = node.kind();

            // Expressions can only contain statements inside functions, which are skipped anyway.
            if AnyJsExpression::can_cast(kind) || is_function_boundary(kind) {
                preorder.skip_subtree();
            } else if AnyJsNestingStatement::can_cast(kind) {
                suppressions.suppress_node(node);
            }
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let statement = ctx.query();
        let keyword = statement.syntax().first_token()?;
        let max = ctx.options().max();
        let depth = usize::from(max) + 1;

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                keyword.text_trimmed_range(),
                markup! {
                    "This block is nested too deeply."
                },
            )
            .note(markup! {
                "Blocks nested "{depth}" levels deep are hard to read and maintain. The configured maximum is "{max}"."
            })
            .note(markup! {
                "Return early or extract the nested code into a separate function to reduce nesting."
            }),
        )
    }
}

/// Returns `false` for the `if` of an `else if`, which continues the chain of its parent `if`
/// rather than adding a level of nesting.
fn increases_depth(statement: &AnyJsNestingStatement) -> bool {
    !matches!(
        statement,
        AnyJsNestingStatement::JsIfStatement(if_statement)
            if if_statement.parent::<JsElseClause>().is_some()
    )
}
