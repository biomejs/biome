use biome_analyze::context::RuleContext;
use biome_analyze::{declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource, RuleSourceKind};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    JsAssignmentExpression, JsExpressionStatement, JsForStatement, JsParenthesizedExpression,
    JsSequenceExpression,
};
use biome_rowan::AstNode;

declare_lint_rule! {
    /// Disallow assignments in expressions.
    ///
    /// In expressions, it is common to mistype a comparison operator (such as `==`) as an assignment operator (such as `=`).
    /// Moreover, the use of assignments in expressions is confusing.
    /// Indeed, expressions are often considered as side-effect free.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// let a, b;
    /// a = (b = 1) + 1;
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// let a;
    /// if (a = 1) {
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// function f(a) {
    ///     return a = 1;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// let a;
    /// a = 1;
    /// ```
    pub NoAssignInExpressions {
        version: "1.0.0",
        name: "noAssignInExpressions",
        language: "js",
        sources: &[RuleSource::Eslint("no-cond-assign")],
        source_kind: RuleSourceKind::Inspired,
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for NoAssignInExpressions {
    type Query = Ast<JsAssignmentExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let assign = ctx.query();
        let mut ancestor = assign
            .syntax()
            .ancestors()
            .take_while(|x| {
                // Allow parens and multiple assign such as `a = b = (c = d)`
                JsAssignmentExpression::can_cast(x.kind())
                    || JsParenthesizedExpression::can_cast(x.kind())
            })
            .last()?;
        let mut prev_ancestor = ancestor;
        ancestor = prev_ancestor.parent()?;
        while JsSequenceExpression::can_cast(ancestor.kind()) {
            // Allow statements separated by sequences such as `a = 1, b = 2`
            prev_ancestor = ancestor;
            ancestor = prev_ancestor.parent()?;
        }
        if JsExpressionStatement::can_cast(ancestor.kind()) {
            None
        } else if let Some(for_stmt) = JsForStatement::cast(ancestor) {
            if let Some(for_test) = for_stmt.test() {
                // Disallow assignment in test part of a `for`
                (for_test.syntax() == &prev_ancestor).then_some(())
            } else {
                // Allow assignment in initializer and update parts of a `for`
                None
            }
        } else {
            Some(())
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let assign = ctx.query();
        Some(RuleDiagnostic::new(
            rule_category!(),
            assign.range(),
            markup! {
                "The "<Emphasis>"assignment"</Emphasis>" should not be in an "<Emphasis>"expression"</Emphasis>"."
            },
        ).note(
            "The use of assignments in expressions is confusing.\nExpressions are often considered as side-effect free."
        ))
    }
}
