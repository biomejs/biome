use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{JsLanguage, JsPostUpdateExpression, JsPreUpdateExpression, JsSyntaxKind};
use biome_rowan::{AstNode, SyntaxNode, declare_node_union};
use biome_rule_options::no_increment_decrement::NoIncrementDecrementOptions;

declare_lint_rule! {
    /// Disallows the usage of the unary operators ++ and --.
    ///
    /// Because the unary ++ and -- operators are subject to automatic semicolon insertion, differences in whitespace can change semantics of source code.
    ///
    /// ```js,expect_diagnostic
    /// let i = 10;
    /// let j = 20;
    ///
    /// i ++
    /// j
    /// // i = 11, j = 20
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// let i = 10;
    /// let j = 20;
    ///
    /// i
    /// ++
    /// j
    /// // i = 10, j = 21
    /// ```
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// let foo = 0;
    /// foo++;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// let bar = 42;
    /// bar--;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// for (let i = 0; i < 10; i++) {
    ///     doSomething(i);
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// for (let i = 0; i < 10;) {
    ///     doSomething(i);
    ///     i++;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// let foo = 0;
    /// foo += 1;
    /// ```
    ///
    /// ```js
    /// let bar = 42;
    /// bar -= 1;
    /// ```
    ///
    /// ```js
    /// for (let i = 0; i < 10; i += 1) {
    ///     doSomething(i);
    /// }
    /// ```
    ///
    /// ```js
    /// for (let i = 0; i < 10;) {
    ///     doSomething(i);
    ///     i += 1;
    /// }
    /// ```
    ///
    /// ## Options
    ///
    /// ### `allowForLoopAfterthoughts`
    ///
    /// Allows unary operators ++ and -- in the afterthought (final expression) of a for loop.
    ///
    /// Default `false`
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "allowForLoopAfterthoughts": true
    ///   }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```js,expect_diagnostic,use_options
    /// for (let i = 0; i < j; j = i++) {
    ///     doSomething(i, j);
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic,use_options
    /// for (let i = 10; i--;) {
    ///     doSomething(i);
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic,use_options
    /// for (let i = 0; i < 10;) i++;
    /// ```
    ///
    /// #### Valid
    ///
    /// ```js,use_options
    /// for (let i = 0; i < 10; i++) {
    ///     doSomething(i);
    /// }
    /// ```
    ///
    /// ```js,use_options
    /// for (let i = 0, j = l; i < l; i++, j--) {
    ///     doSomething(i, j);
    /// }
    /// ```
    ///
    pub NoIncrementDecrement {
        version: "2.3.2",
        name: "noIncrementDecrement",
        language: "js",
        recommended: false,
        sources: &[RuleSource::Eslint("no-plusplus").same()],
    }
}

declare_node_union! {
    pub NoIncrementDecrementQuery = JsPreUpdateExpression | JsPostUpdateExpression
}

impl Rule for NoIncrementDecrement {
    type Query = Ast<NoIncrementDecrementQuery>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoIncrementDecrementOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        if ctx.options().allow_for_loop_afterthoughts() && is_for_loop_afterthought(node.syntax()) {
            return None;
        }

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Unexpected use of increment/decrement unary operator."
                },
            )
            .note(markup! {
                "The unary ++ and -- operators are subject to automatic semicolon insertion, differences in whitespace can change semantics of source code. Instead use += or -=."
            }),
        )
    }
}

fn is_for_loop_afterthought(node: &SyntaxNode<JsLanguage>) -> bool {
    let Some(parent) = node.parent() else {
        return false;
    };

    match parent.kind() {
        JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION | JsSyntaxKind::JS_SEQUENCE_EXPRESSION => {
            is_for_loop_afterthought(&parent)
        }
        JsSyntaxKind::JS_FOR_STATEMENT => {
            if let Some(for_stmt) = biome_js_syntax::JsForStatement::cast(parent.clone())
                && let Some(update) = for_stmt.update()
            {
                return update.syntax().eq(node);
            }
            false
        }
        _ => false,
    }
}
