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
use biome_rule_options::no_react_props::NoReactPropsOptions;

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
 pub NoReactProps {
        version: "1.0.0",
        name: "noReactProps",
        language: "js",
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

impl Rule for NoReactProps {
    type Query = Ast<JsxAttribute>;
    type State = (TextRange, &'static str);
    type Signals = Option<Self::State>;
    type Options = NoReactPropsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let attribute = ctx.query();
        let name = attribute.name().ok()?;
        let range = name.range();
        let name = name.to_trimmed_text();

        if let Some(replacement) = get_replacement_for_react_prop(&name) {
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
                markup!("This JSX attribute is specific to React and should not be used in Qwik."),
            )
            .detail(
                range,
                "Use standard HTML attributes instead of React-specific prop names.",
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
