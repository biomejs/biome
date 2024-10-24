use crate::services::semantic::Semantic;
use biome_analyze::RuleSource;
use biome_analyze::{context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::JsCallExpression;
use biome_rowan::{AstNode, AstSeparatedList};

declare_lint_rule! {
    /// Enforce the consistent use of the radix argument when using `parseInt()`.
    ///
    /// When using the `parseInt()` function it is common to omit the second argument, the radix, and let the function try to determine from the first argument what type of number it is. By default, `parseInt()` will autodetect decimal and hexadecimal (via `0x` prefix). Prior to ECMAScript 5, `parseInt()` also autodetected octal literals, which caused problems because many developers assumed a leading `0` would be ignored.
    ///
    /// This confusion led to the suggestion that you always use the radix parameter to `parseInt()` to eliminate unintended consequences.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// parseInt("071");
    /// parseInt(someValue);
    /// parseInt("071", "abc");
    /// parseInt("071", 37);
    /// parseInt();
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// parseInt("071", 10);
    /// parseInt("071", 8);
    /// parseFloat(someValue);
    /// ```
    ///
    pub UseParseIntRadix {
        version: "next",
        name: "useParseIntRadix",
        language: "js",
        recommended: true,
        sources: &[RuleSource::Eslint("radix")],
    }
}

impl Rule for UseParseIntRadix {
    type Query = Semantic<JsCallExpression>;
    type State = usize;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call_expression = ctx.query();

        let object_name = call_expression.callee().ok()?.get_callee_object_name()?;

        if !matches!(object_name.text(), "Number" | "parseInt") {
            return None;
        }

        let member_name = call_expression.callee().ok()?.get_callee_member_name()?;
        if member_name.text() != "parseInt" {
            return None;
        }

        let argument_count = call_expression.arguments().ok()?.args().len();

        if argument_count >= 2 {
            return None;
        }

        Some(argument_count)
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        let title = if *state == 0 {
            markup!("Missing parameters")
        } else {
            markup!("Missing radix parameter")
        };

        Some(RuleDiagnostic::new(rule_category!(), node.range(), title))
    }
}
