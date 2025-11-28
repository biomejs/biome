use crate::services::typed::Typed;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::JsCallExpression;
use biome_js_type_info::{Literal, ResolvedTypeData, TypeData};
use biome_rowan::{AstNode, AstSeparatedList};
use biome_rule_options::use_regexp_exec::UseRegexpExecOptions;

declare_lint_rule! {
    /// Enforce `RegExp#exec` over `String#match` if no global flag is provided.
    ///
    /// String#match is defined to work the same as RegExp#exec when the regular expression does not include the g flag.
    /// Keeping to consistently using one of the two can help improve code readability.
    ///
    /// RegExp#exec may also be slightly faster than String#match; this is the reason to choose it as the preferred usage.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,file=invalid.ts,expect_diagnostic
    /// 'something'.match(/thing/);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts,file=valid.ts
    /// /thing/.exec('something');
    /// ```
    ///
    pub UseRegexpExec {
        version: "next",
        name: "useRegexpExec",
        language: "js",
        recommended: false,
        sources: &[RuleSource::EslintTypeScript("prefer-regexp-exec").same(), RuleSource::EslintRegexp("prefer-regexp-exec").same()],
        domains: &[RuleDomain::Project],
    }
}

impl Rule for UseRegexpExec {
    type Query = Typed<JsCallExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseRegexpExecOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let binding = node.callee().ok()?.omit_parentheses();
        let callee = binding.as_js_static_member_expression()?;

        let call_object = callee.object().ok()?;
        if !ctx
            .type_of_expression(&call_object)
            .is_string_or_string_literal()
        {
            return None;
        }

        let call_name = callee.member().ok()?.as_js_name()?.to_trimmed_text();
        if call_name != "match" {
            return None;
        }

        let args = node.arguments().ok()?.args();
        let first_arg = args.first()?.ok()?;
        let express = first_arg.as_any_js_expression()?;

        let value_type = ctx.type_of_expression(express);

        if value_type
            .resolved_data()
            .map(ResolvedTypeData::as_raw_data)
            .is_some_and(|ty| match ty {
                TypeData::Literal(literal) => match literal.as_ref() {
                    Literal::RegExp(literal) => !literal.flags.contains('g'),
                    _ => false,
                },
                _ => false,
            })
        {
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
                    "Prefer "<Emphasis>"RegExp#exec()"</Emphasis>" over "<Emphasis>"String#match()"</Emphasis>" when searching within a string."
                },
            )
            .note(markup! {
                "Use "<Emphasis>"RegExp#exec()"</Emphasis>" instead of "<Emphasis>"String#match()"</Emphasis>" for consistent and slightly faster regex matching."
            }),
        )
    }
}
