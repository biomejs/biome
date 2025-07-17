use crate::JsRuleAction;
use biome_analyze::context::RuleContext;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make::{jsx_ident, jsx_name};
use biome_js_syntax::{AnyJsxAttributeName, JsxAttribute};
use biome_rowan::{AstNode, BatchMutationExt, TextRange};
use biome_rule_options::no_qwik_react_props::NoQwikReactPropsOptions;

declare_lint_rule! {
    /// Disallow React-specific className/htmlFor props in Qwik components.
    ///
    /// This rule is intended for use in Qwik applications to prevent the use of React-specific
    /// prop names that should be replaced with standard HTML attributes.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <div className="container" />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <label htmlFor="input" />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <div class="container" />
    /// ```
    ///
    /// ```jsx
    /// <label for="input" />
    /// ```
 pub NoQwikReactProps {
        version: "next",
        name: "noQwikReactProps",
        language: "jsx",
        sources: &[RuleSource::EslintQwik("no-react-props").inspired()],
        recommended: true,
        severity: Severity::Warning,
        fix_kind: FixKind::Safe,
        domains: &[RuleDomain::Qwik],
    }
}

fn get_replacement_for_react_prop(str: &str) -> Option<&'static str> {
    match str {
        "className" => Some("class"),
        "htmlFor" => Some("for"),
        _ => None,
    }
}

impl Rule for NoQwikReactProps {
    type Query = Ast<JsxAttribute>;
    type State = (TextRange, &'static str);
    type Signals = Option<Self::State>;
    type Options = NoQwikReactPropsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let attribute = ctx.query();
        let name = attribute.name().ok()?;
        let range = name.range();
        let name_token = name.name_token().ok()?;
        let name_text = name_token.token_text();

        if let Some(replacement) = get_replacement_for_react_prop(&name_text) {
            Some((range, replacement))
        } else {
            None
        }
    }

    fn diagnostic(_: &RuleContext<Self>, (range, _): &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup!("This JSX attribute is specific to React and can cause issues in Qwik components, as Qwik expects standard HTML attributes for optimal reactivity and compatibility."),
            )
            .detail(
                range,
                "Replace React-specific props like className/htmlFor with their standard HTML equivalents (class/for) to ensure proper behavior in Qwik applications.",
            ),
        )
    }

    fn action(ctx: &RuleContext<Self>, (_, replacement): &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        let original_name_node = ctx.query().name().ok()?;

        let new_name_node = AnyJsxAttributeName::JsxName(jsx_name(jsx_ident(replacement)));
        mutation.replace_node(original_name_node, new_name_node);

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! {
                {format!("Replace this attribute name with {replacement:?}")}
            }
            .to_owned(),
            mutation,
        ))
    }
}
