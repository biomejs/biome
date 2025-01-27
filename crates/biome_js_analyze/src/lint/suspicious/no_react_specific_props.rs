use crate::JsRuleAction;
use biome_analyze::context::RuleContext;
use biome_analyze::{
    declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource,
};
use biome_console::markup;
use biome_js_factory::make::{jsx_ident, jsx_name};
use biome_js_syntax::{AnyJsxAttributeName, JsxAttribute};
use biome_rowan::{AstNode, BatchMutationExt, TextRange};

declare_lint_rule! {
    /// Prevents React-specific JSX properties from being used.
    ///
    /// This rule is intended for use in JSX-based frameworks (mainly **Solid.js**)
    /// that do not use React-style prop names.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <Hello className="John" />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <Hello class="Doe" />
    /// ```
 pub NoReactSpecificProps {
        version: "1.7.2",
        name: "noReactSpecificProps",
        language: "js",
        sources: &[RuleSource::EslintSolid("no-react-specific-props")],
        recommended: false,
        fix_kind: FixKind::Safe,
        domains: &[RuleDomain::Solid],
    }
}

const REACT_SPECIFIC_JSX_PROPS: &[&str] = &["className", "htmlFor"];

fn get_replacement_for_react_prop(str: &str) -> Option<&'static str> {
    match str {
        "className" => Some("class"),
        "htmlFor" => Some("for"),
        _ => None,
    }
}

impl Rule for NoReactSpecificProps {
    type Query = Ast<JsxAttribute>;
    type State = (TextRange, &'static str);
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let attribute = ctx.query();
        let name = attribute.name().ok()?;
        let range = name.range();
        let name = name.to_trimmed_string();

        if REACT_SPECIFIC_JSX_PROPS.contains(&name.as_str()) {
            let replacement = get_replacement_for_react_prop(&name)?;
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
                markup!("This JSX attribute is specific to React."),
            )
            .detail(
                range,
                "This attribute may not be supported by non-React frameworks, as it is not native to HTML.",
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
