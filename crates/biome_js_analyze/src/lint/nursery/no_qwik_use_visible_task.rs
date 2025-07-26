use biome_analyze::context::RuleContext;
use biome_analyze::{Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsCallArgument, JsCallExpression, global_identifier};
use biome_rowan::AstSeparatedList;
use biome_rowan::TextRange;
use biome_rule_options::no_qwik_use_visible_task::NoQwikUseVisibleTaskOptions;

declare_lint_rule! {
    /// Disallow `useVisibleTask$()` functions in Qwik components.
    ///
    /// Prevents hydration-blocking operations that hurt Qwik's resumability.
    /// See [Qwik Tasks Documentation](https://qwik.dev/docs/components/tasks/) for proper alternatives.
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
        language: "jsx",
        sources: &[RuleSource::EslintQwik("no-use-visible-task").same()],
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
            let Some(arguments) = call_expression.arguments().ok() else {
                return Some(name.range());
            };
            let args = arguments.args();
            if args.len() < 2 {
                return Some(name.range());
            }
            let Some(Ok(AnyJsCallArgument::AnyJsExpression(expr))) = args.iter().nth(1) else {
                return Some(name.range());
            };
            let Some(obj) = expr.as_js_object_expression() else {
                return Some(name.range());
            };

            for member in obj.members().iter().flatten() {
                if member
                    .as_js_property_object_member()
                    .and_then(|prop| {
                        prop.name().ok().and_then(|name_node| {
                            name_node.name().and_then(|name| {
                                if name == "strategy" {
                                    prop.value().ok().and_then(|value| {
                                        value.as_any_js_literal_expression().and_then(|lit| {
                                            lit.as_js_string_literal_expression().and_then(
                                                |str_lit| {
                                                    str_lit.inner_string_text().ok().and_then(
                                                        |text| {
                                                            let trimmed = text
                                                                .text()
                                                                .trim_matches(['"', '\'']);
                                                            if trimmed == "document-idle" {
                                                                Some(())
                                                            } else {
                                                                None
                                                            }
                                                        },
                                                    )
                                                },
                                            )
                                        })
                                    })
                                } else {
                                    None
                                }
                            })
                        })
                    })
                    .is_some()
                {
                    return None;
                }
            }
            Some(name.range())
        } else {
            None
        }
    }

    fn diagnostic(_: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        RuleDiagnostic::new(
            rule_category!(),
            range,
            markup! {
                "Avoid "<Emphasis>"useVisibleTask$"</Emphasis>" for non-interactive initialization"
            },
        )
        .note(markup! {
            "This hook executes immediately on component mount without user interaction, potentially:"
            "\n- Hurting performance (blocking hydration)"
            "\n- Causing layout shifts (CLS)"
            "\n- Breaking SSR compatibility"
        })
        .note(markup! {
            "Check the "<Hyperlink href="https://qwik.dev/docs/components/tasks/">"Qwik documentation"</Hyperlink>" for suitable alternatives."
        })
        .into()
    }
}
