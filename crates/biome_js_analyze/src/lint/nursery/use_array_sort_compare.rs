use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::JsCallExpression;
use biome_rowan::{AstNode, AstSeparatedList};
use biome_rule_options::use_array_sort_compare::UseArraySortCompareOptions;

use crate::services::typed::Typed;

declare_lint_rule! {
    /// Require Array#sort and Array#toSorted calls to always provide a compareFunction.
    ///
    /// When called without a compare function, Array#sort() and Array#toSorted() converts all non-undefined array elements into strings and then compares said strings based off their UTF-16 code units [ECMA specification](https://262.ecma-international.org/9.0/#sec-sortcompare).
    ///
    /// The result is that elements are sorted alphabetically, regardless of their type. For example, when sorting numbers, this results in a "10 before 2" order:
    ///
    /// ```ts,file=example.ts,ignore
    /// [1, 2, 3, 10, 20, 30].sort(); //→ [1, 10, 2, 20, 3, 30]
    /// ```
    ///
    /// This rule reports on any call to the sort methods that do not provide a compare argument.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,file=invalid.ts,expect_diagnostic
    /// const array: any[] = [];
    /// array.sort();
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts,file=valid.ts
    /// const array: any[] = [];
    /// array.sort((a, b) => a - b);
    /// ```
    ///
    pub UseArraySortCompare {
        version: "2.3.5",
        name: "useArraySortCompare",
        language: "js",
        recommended: false,
        sources: &[RuleSource::EslintTypeScript("require-array-sort-compare").same()],
        domains: &[RuleDomain::Types],
    }
}

impl Rule for UseArraySortCompare {
    type Query = Typed<JsCallExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseArraySortCompareOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let binding = node.callee().ok()?.omit_parentheses();
        let callee = binding.as_js_static_member_expression()?;

        let call_object = callee.object().ok()?;
        let ty = ctx.type_of_expression(&call_object);
        if !ty.is_array_of(|_ty| true) {
            return None;
        }

        let call_name = callee.member().ok()?.as_js_name()?.to_trimmed_text();
        if call_name != "sort" && call_name != "toSorted" {
            return None;
        }

        let arguments = node.arguments().ok()?.args();
        if arguments.is_empty() {
            return Some(());
        }

        let binding = arguments.first()?.ok()?;
        let first_arg = binding.as_any_js_expression()?;
        let ty = ctx.type_of_expression(first_arg);
        if ty.is_undefined() || ty.is_null() {
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
                    "Compare function missing."
                },
            )
            .note(markup! {
                "When called without a compare function, Array#sort() and Array#toSorted() converts all non-undefined array elements into strings and then compares said strings based off their UTF-16 code units."
            })
            .note(markup! {
                "Add a compare function to prevent unexpected sorting."
            }),
        )
    }
}
