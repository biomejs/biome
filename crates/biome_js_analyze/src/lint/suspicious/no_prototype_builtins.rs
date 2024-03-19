use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_js_syntax::{AnyJsMemberExpression, JsCallExpression, TextRange};
use biome_rowan::AstNode;

declare_rule! {
    /// Disallow direct use of `Object.prototype` builtins.
    ///
    /// ECMAScript 5.1 added `Object.create` which allows the creation of an object with a custom prototype.
    /// This pattern is often used for objects used as Maps. However, this pattern can lead to errors
    /// if something else relies on prototype properties/methods.
    /// Moreover, the methods could be shadowed, this can lead to random bugs and denial of service
    /// vulnerabilities. For example, calling `hasOwnProperty` directly on parsed JSON like `{"hasOwnProperty": 1}` could lead to vulnerabilities.
    /// To avoid subtle bugs like this, you should call these methods from `Object.prototype`.
    /// For example, `foo.isPrototypeof(bar)` should be replaced with `Object.prototype.isPrototypeof.call(foo, "bar")`
    /// As for the `hasOwn` method, `foo.hasOwn("bar")` should be replaced with `Object.hasOwn(foo, "bar")`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// var invalid = foo.hasOwnProperty("bar");
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// var invalid = foo.isPrototypeOf(bar);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// var invalid = foo.propertyIsEnumerable("bar");
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// var valid = Object.hasOwn(foo, "bar");
    /// var valid = Object.prototype.isPrototypeOf.call(foo, bar);
    /// var valid = {}.propertyIsEnumerable.call(foo, "bar");
    /// ```
    ///
    pub NoPrototypeBuiltins {
        version: "1.0.0",
        name: "noPrototypeBuiltins",
        source: RuleSource::Eslint("no-prototype-builtins"),
        recommended: true,
    }
}

pub struct RuleState {
    prototype_builtins_method_name: String,
    text_range: TextRange,
}

impl Rule for NoPrototypeBuiltins {
    type Query = Ast<JsCallExpression>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call_expr = ctx.query();
        let callee = call_expr.callee().ok()?.omit_parentheses();
        if let Some(member_expr) = AnyJsMemberExpression::cast_ref(callee.syntax()) {
            let member_name = member_expr.member_name()?;
            let member_name_text = member_name.text();
            return is_prototype_builtins(member_name_text).then_some(RuleState {
                prototype_builtins_method_name: member_name_text.to_string(),
                text_range: member_name.range(),
            });
        }
        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let diag = RuleDiagnostic::new(
            rule_category!(),
            state.text_range,
            markup! {
                "Do not access Object.prototype method '"{&state.prototype_builtins_method_name}"' from target object."
            },
        );

        if state.prototype_builtins_method_name == "hasOwnProperty" {
            Some(
                diag.note(markup! {
                    "It's recommended using "<Emphasis>"Object.hasOwn()"</Emphasis>" instead of using "<Emphasis>"Object.hasOwnProperty()"</Emphasis>"."
                })
                .note(markup! {
                    "See "<Hyperlink href="https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/hasOwn">"MDN web docs"</Hyperlink>" for more details."
                }),
            )
        } else {
            Some(diag)
        }
    }
}

/// Chekcks if the `Object.prototype` builtins called directly.
fn is_prototype_builtins(token_text: &str) -> bool {
    matches!(
        token_text,
        "hasOwnProperty" | "isPrototypeOf" | "propertyIsEnumerable"
    )
}
