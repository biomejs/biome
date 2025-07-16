use biome_analyze::context::RuleContext;
use biome_analyze::{Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsCallArgument, JsCallExpression, global_identifier};
use biome_rowan::AstSeparatedList;
use biome_rowan::TextRange;
use biome_rule_options::no_qwik_use_visible_task::NoQwikUseVisibleTaskOptions;

declare_lint_rule! {
    /// Disallow useVisibleTask$() functions in Qwik components.
    ///
    /// This rule is intended for use in Qwik applications to prevent the use of
    /// useVisibleTask$() functions which are not recommended in Qwik.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// useVisibleTask$(() => {
    ///   console.log('Component is visible');
    /// });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// useTask$(() => {
    ///   console.log('Task executed');
    /// });
    /// ```
    ///
    pub NoQwikUseVisibleTask {
        version: "next",
        name: "noQwikUseVisibleTask",
        language: "js",
        sources: &[RuleSource::EslintQwik("no-use-visible-task").inspired()],
        recommended: true,
        severity: Severity::Warning,
        domains: &[RuleDomain::Qwik],
    }
}

impl Rule for NoQwikUseVisibleTask {
    type Query = Ast<JsCallExpression>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = NoQwikUseVisibleTaskOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call_expression = ctx.query();
        let callee = call_expression.callee().ok()?.omit_parentheses();
        let (_, name) = global_identifier(&callee)?;

        if name.text() == "useVisibleTask$" {
            if let Some(arguments) = call_expression.arguments().ok() {
                let args = arguments.args();
                if args.len() >= 2 {
                    if let Some(Ok(AnyJsCallArgument::AnyJsExpression(expr))) = args.iter().nth(1) {
                        if let Some(obj) = expr.as_js_object_expression() {
                            for member in obj.members() {
                                if let Ok(member) = member {
                                    if let Some(prop) = member.as_js_property_object_member() {
                                        if let Ok(name) = prop.name() {
                                            if let Some(name) = name.name() {
                                                if name == "strategy" {
                                                    if let Ok(value) = prop.value() {
                                                        if let Some(str_lit) = value
                                                            .as_any_js_literal_expression()
                                                            .and_then(|lit| {
                                                                lit.as_js_string_literal_expression(
                                                                )
                                                            })
                                                        {
                                                            if let Ok(text) =
                                                                str_lit.inner_string_text()
                                                            {
                                                                if text
                                                                    .text()
                                                                    .trim_matches(['"', '\''])
                                                                    == "document-idle"
                                                                {
                                                                    return None;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Some(name.range())
        } else {
            None
        }
    }

    fn diagnostic(_: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup!("useVisibleTask$() should be used with care in Qwik applications."),
            )
            .detail(
                range,
                "Consider using useTask$() or other Qwik lifecycle functions instead.",
            ),
        )
    }
}
