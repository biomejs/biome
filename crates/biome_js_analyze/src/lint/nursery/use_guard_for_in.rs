use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsStatement, JsForInStatement};
use biome_rowan::{AstNode, AstNodeList};

declare_lint_rule! {
    /// Require `for-in` loops to include an `if` statement.
    ///
    /// Looping over objects with a `for-in` loop will include properties inherited through the prototype chain.
    /// This behavior can lead to unexpected items in your for loop.
    ///
    /// For codebases that do not support ES2022, `Object.prototype.hasOwnProperty.call(foo, key)` can be used as a check that the property is not inherited.
    ///
    /// For codebases that do support ES2022, `Object.hasOwn(foo, key)` can be used as a shorter and more reliable alternative.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// for (key in foo) {
    ///   doSomething(key);
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// for (key in foo) {
    ///   if (Object.hasOwn(foo, key)) {
    ///    doSomething(key);
    ///   }
    /// }
    /// ```
    ///
    /// ```js
    /// for (key in foo) {
    ///   if (Object.prototype.hasOwnProperty.call(foo, key)) {
    ///     doSomething(key);
    ///   }
    /// }
    /// ```
    ///
    /// ```js
    /// for (key in foo) {
    ///   if ({}.hasOwnProperty.call(foo, key)) {
    ///     doSomething(key);
    ///   }
    /// }
    /// ```
    ///
    pub UseGuardForIn {
        version: "1.9.4",
        name: "useGuardForIn",
        language: "js",
        sources: &[RuleSource::Eslint("guard-for-in")],
        recommended: false,
    }
}

impl Rule for UseGuardForIn {
    type Query = Ast<JsForInStatement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let body = node.body().ok()?;

        match body {
            AnyJsStatement::JsEmptyStatement(_) | AnyJsStatement::JsIfStatement(_) => None,
            AnyJsStatement::JsBlockStatement(block) => {
                let statements = block.statements();

                match statements.len() {
                    0 => None,
                    1 => {
                        let first_statement = statements.first()?;
                        if first_statement.as_js_if_statement().is_none() {
                            Some(())
                        } else {
                            None
                        }
                    }
                    _ => {
                        let first_statement = statements.first()?;
                        if let Some(first_if_statement) = first_statement.as_js_if_statement() {
                            match first_if_statement.consequent().ok()? {
                                AnyJsStatement::JsBlockStatement(block)
                                    if block.statements().len() == 1 =>
                                {
                                    if block
                                        .statements()
                                        .first()?
                                        .as_js_continue_statement()
                                        .is_none()
                                    {
                                        Some(())
                                    } else {
                                        None
                                    }
                                }
                                AnyJsStatement::JsContinueStatement(_) => None,
                                _ => Some(()),
                            }
                        } else {
                            Some(())
                        }
                    }
                }
            }
            _ => Some(()),
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "The body of a for-in should be wrapped in an `if` statement."
                },
            )
            .note(markup! {
                "Looping over the object with for-in loop  will include properties that are inherited through the prototype chain, the behaviour can lead to some unexpected items in your loop."
            }).note(markup! {
                "To resolve this issue, add an if statement like `if (Object.hasOwn(foo, key)) {...}` to filter out the extraneous properties. "
            }),
        )
    }
}
