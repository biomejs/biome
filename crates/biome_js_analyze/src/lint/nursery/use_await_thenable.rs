use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::JsAwaitExpression;
use biome_rowan::AstNode;
use biome_rule_options::use_await_thenable::UseAwaitThenableOptions;

use crate::services::typed::Typed;

declare_lint_rule! {
    /// Enforce that `await` is _only_ used on `Promise` values.
    ///
    /// :::caution
    /// At the moment, this rule only checks for instances of the global
    /// `Promise` class. This is a major shortcoming compared to the ESLint
    /// rule if you are using custom `Promise`-like implementations such as
    /// [Bluebird](http://bluebirdjs.com/) or in-house solutions.
    /// :::
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic,file=invalid-primitive.js
    /// await 'value';
    /// ```
    ///
    /// ```js,expect_diagnostic,file=invalid-function-call.js
    /// const createValue = () => 'value';
    /// await createValue();
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js,file=valid-examples.js
    /// await Promise.resolve('value');
    ///
    /// const createValue = async () => 'value';
    /// await createValue();
    /// ```
    ///
    pub UseAwaitThenable {
        version: "2.3.9",
        name: "useAwaitThenable",
        language: "js",
        recommended: false,
        sources: &[RuleSource::EslintTypeScript("use-await-thenable").inspired()],
        domains: &[RuleDomain::Types],
    }
}

impl Rule for UseAwaitThenable {
    type Query = Typed<JsAwaitExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseAwaitThenableOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let expression = node.argument().ok()?;
        let ty = ctx.type_of_expression(&expression);

        // Uncomment the following line for debugging convenience:
        //let printed = format!("type of {expression:?} = {ty:?}");

        (ty.is_inferred() && !ty.is_promise_instance()).then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "`await` used on a non-Promise value."
                },
            )
            .note(markup! {
                "This may happen if you accidentally used `await` on a synchronous value."
            })
            .note(markup! {
                "Please ensure the value is not a custom \"thenable\" implementation before removing the `await`: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise#thenables"
            }),
        )
    }
}
