use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsMemberExpression, JsCallExpression, JsNewOrCallExpression, global_identifier,
};
use biome_rowan::AstNode;
use biome_rule_options::no_uninvoked_array_callback::NoUninvokedArrayCallbackOptions;

use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow array methods with callbacks on arrays that only have empty slots.
    ///
    /// Calling `new Array(5)` or `Array(5)` with a single number doesn't create an array of five
    /// `undefined` values. It creates an array with a `length` of `5` and five empty slots, which
    /// hold no value at all. Methods such as `map()`, `filter()`, and `forEach()` skip empty slots,
    /// so the callback passed to them never runs. For example, `new Array(3).map(() => 0)` gives
    /// an array with three empty slots, not `[0, 0, 0]`.
    ///
    /// This rule reports calls to `every()`, `filter()`, `flatMap()`, `forEach()`, `map()`,
    /// `reduce()`, `reduceRight()`, `some()`, `sort()`, and `toSorted()` with a callback, directly
    /// on an array created by calling `Array` with a single number.
    ///
    /// To run the callback for every slot, fill the array first with `fill()`, or create it with
    /// `Array.from({ length: 5 }, callback)`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const list = new Array(5).map(() => createElement());
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const list = Array(5).filter(function (item) {
    ///     return item !== undefined;
    /// });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// new Array(5).forEach((_, index) => console.log(index));
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const list = new Array(5).fill().map(() => createElement());
    /// const items = Array.from({ length: 5 }, () => createElement());
    /// const values = new Array(1, 2, 3).map((value) => value * 2);
    /// const index = new Array(5).findIndex((item) => item === undefined);
    /// ```
    ///
    pub NoUninvokedArrayCallback {
        version: "next",
        name: "noUninvokedArrayCallback",
        language: "js",
        sources: &[RuleSource::Oxc("uninvoked-array-callback").inspired()],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for NoUninvokedArrayCallback {
    type Query = Semantic<JsCallExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoUninvokedArrayCallbackOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        let callee = callee_member(call)?;
        let method = callee.member_name()?;
        if SKIPPING_METHODS.binary_search(&method.text()).is_err() {
            return None;
        }
        // A spread argument may be empty, and `sort(undefined)` asks for the default sort
        // order, so neither is known to pass a callback.
        let [callback] = call.arguments().ok()?.get_arguments_by_index([0]);
        if callback?
            .as_any_js_expression()?
            .clone()
            .omit_parentheses()
            .as_js_identifier_expression()
            .is_some_and(|identifier| identifier.name().is_ok_and(|name| name.is_undefined()))
        {
            return None;
        }

        let array =
            JsNewOrCallExpression::cast(callee.object().ok()?.omit_type_wrappers()?.into_syntax())?;
        let [length, extra] = array.arguments()?.get_arguments_by_index([0, 1]);
        if extra.is_some() {
            return None;
        }
        length?
            .as_any_js_expression()?
            .clone()
            .omit_parentheses()
            .as_any_js_literal_expression()?
            .as_js_number_literal_expression()?;

        let constructor = array.callee().ok()?.omit_parentheses();
        let (reference, name) =
            global_identifier(&constructor.as_any_global_identifier_expression()?)?;
        if name.text() != "Array" || ctx.model().binding(&reference).is_some() {
            return None;
        }

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let callee = callee_member(ctx.query())?;
        let method = callee.member_name()?;
        let member_range = match &callee {
            AnyJsMemberExpression::JsStaticMemberExpression(member) => {
                member.member().ok()?.range()
            }
            AnyJsMemberExpression::JsComputedMemberExpression(member) => {
                member.member().ok()?.range()
            }
        };
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                member_range,
                markup! {
                    "The callback passed to "<Emphasis>{method.text()}"()"</Emphasis>" is never called."
                },
            )
            .detail(
                callee.object().ok()?.range(),
                markup! {
                    "This array only has empty slots."
                },
            )
            .note(markup! {
                "Calling "<Emphasis>"Array"</Emphasis>" with a single number creates an array with that many empty slots instead of "<Emphasis>"undefined"</Emphasis>" values. "<Emphasis>{method.text()}"()"</Emphasis>" doesn't call its callback for empty slots."
            })
            .note(markup! {
                "Fill the array first, for example with "<Emphasis>"new Array(length).fill()"</Emphasis>", or create it with "<Emphasis>"Array.from({ length }, callback)"</Emphasis>"."
            }),
        )
    }
}

/// Returns the member expression that `call` calls, such as `array.map` in `array.map(callback)`.
fn callee_member(call: &JsCallExpression) -> Option<AnyJsMemberExpression> {
    AnyJsMemberExpression::cast(call.callee().ok()?.omit_parentheses().into_syntax())
}

/// Array methods that never call their callback on an array that only has empty slots.
/// `find()`, `findIndex()`, `findLast()`, and `findLastIndex()` aren't listed because they
/// call their callback with `undefined` for empty slots.
///
/// Sorted so it can be searched with `binary_search`.
const SKIPPING_METHODS: &[&str] = &[
    "every",
    "filter",
    "flatMap",
    "forEach",
    "map",
    "reduce",
    "reduceRight",
    "some",
    "sort",
    "toSorted",
];

#[cfg(test)]
mod tests {
    use super::SKIPPING_METHODS;

    #[test]
    fn skipping_methods_are_sorted() {
        for pair in SKIPPING_METHODS.windows(2) {
            assert!(
                pair[0].cmp(pair[1]).is_lt(),
                "{} should come before {}",
                pair[1],
                pair[0]
            );
        }
    }
}
