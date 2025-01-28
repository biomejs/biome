use crate::services::semantic::Semantic;
use biome_analyze::{
    context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic, RuleDomain, RuleSource,
    RuleSourceKind,
};
use biome_console::markup;
use biome_js_syntax::{
    JsObjectBindingPattern, JsParameters, JsVariableDeclarator, JsxExpressionAttributeValue,
};
use biome_rowan::{AstNode, AstSeparatedList, TextRange};
use biome_string_case::Case;

declare_lint_rule! {
    /// Succinct description of the rule.
    ///
    /// Put context and details about the rule.
    /// As a starting point, you can take the description of the corresponding _ESLint_ rule (if any).
    ///
    /// Try to stay consistent with the descriptions of implemented rules.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// var a = 1;
    /// a = 2;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// // var a = 1;
    /// ```
    ///
    pub NoDestructuredProps {
        version: "next",
        name: "noDestructuredProps",
        language: "js",
        domains: &[RuleDomain::Solid],
        recommended: false,
        sources: &[RuleSource::EslintSolid("no-destructure")],
        source_kind: RuleSourceKind::Inspired,
    }
}

impl Rule for NoDestructuredProps {
    type Query = Semantic<JsxExpressionAttributeValue>;
    type State = (Box<str>, TextRange);
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();
        let value = node
            .expression()
            .ok()?
            .as_js_identifier_expression()?
            .name()
            .ok()?;
        let binding = model.binding(&value)?;

        let binding_pattern = binding
            .syntax()
            .ancestors()
            .find_map(|node| JsObjectBindingPattern::cast(node))?;

        let parameters = binding_pattern
            .syntax()
            .ancestors()
            .find_map(|node| JsParameters::cast(node))?;

        // In solid, a component can't accept more than one property
        if parameters.items().len() > 1 {
            return None;
        }

        let variable_declarator = binding_pattern
            .syntax()
            .ancestors()
            .find_map(|node| JsVariableDeclarator::cast(node))?;

        let name = variable_declarator.id().ok()?;
        let name = name.as_any_js_binding()?.as_js_identifier_binding()?;

        let text = name.name_token().ok()?;

        if Case::identify(text.text_trimmed(), false) == Case::Pascal {
            return Some((
                Box::from(value.name().ok()?.text()),
                binding_pattern.range(),
            ));
        }

        None
    }

    fn diagnostic(
        ctx: &RuleContext<Self>,
        (binding_name, range): &Self::State,
    ) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "This variable shouldn't be destructured."
                },
            )
            .detail(
                range,
                markup! {
                    "This is where the props were destructured."
                },
            ).note(
                markup!{
                    "In Solid, props must be used with property accesses (props."{binding_name}") to preserve reactivity."
                }
            ).note(
                markup!{
                    "Remove the destructuring and use props."{binding_name}" instead."
                })
        )
    }
}
