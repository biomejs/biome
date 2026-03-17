use crate::JsRuleAction;
use crate::react::ReactCreateElementCall;
use crate::services::semantic::Semantic;
use crate::utils::batch::JsBatchMutation;
use biome_analyze::context::RuleContext;
use biome_analyze::{FixKind, Rule, RuleDiagnostic, declare_lint_rule};
use biome_console::markup;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsObjectMember, JsCallExpression, JsPropertyObjectMember, JsSyntaxToken, JsxAttribute,
};
use biome_rowan::{AstNode, BatchMutationExt, TextRange, declare_node_union};
use biome_rule_options::no_inline_styles::NoInlineStylesOptions;
use biome_string_case::StrOnlyExtension;

declare_lint_rule! {
    /// Disallow the use of inline styles.
    ///
    /// Inline styles via the `style` attribute make code harder to maintain and override,
    /// prevent reusability of styling, and can be a security concern when implementing
    /// a strict Content Security Policy (CSP).
    ///
    /// Instead of inline styles, use CSS classes, CSS modules, or a styling library.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <div style={{ color: "red" }}>Error</div>
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// React.createElement("div", { style: { color: "red" } });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <div className="text-red">Error</div>
    /// ```
    ///
    /// ```js
    /// React.createElement("div", { className: "container" });
    /// ```
    ///
    /// ## Resources
    ///
    /// - [html-eslint: no-inline-styles](https://html-eslint.org/docs/rules/no-inline-styles)
    /// - [Content Security Policy: Allowing inline styles](https://content-security-policy.com/examples/allow-inline-style)
    ///
    pub NoInlineStyles {
        version: "next",
        name: "noInlineStyles",
        language: "js",
        recommended: false,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoInlineStyles {
    type Query = Semantic<NoInlineStylesQuery>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = NoInlineStylesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        match node {
            NoInlineStylesQuery::JsxAttribute(jsx_attribute) => {
                let name = jsx_attribute.name().ok()?;
                let name = name.as_jsx_name()?;
                let value_token = name.value_token().ok()?;
                if is_style_value(&value_token) {
                    return Some(jsx_attribute.range());
                }
            }
            NoInlineStylesQuery::JsCallExpression(call_expression) => {
                let model = ctx.model();
                let property_member = find_style_attribute(call_expression, model)?;
                return Some(property_member.range());
            }
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state,
                markup! {
                    "Avoid using the "<Emphasis>"style"</Emphasis>" attribute. Prefer external CSS classes instead of inline styles."
                },
            )
            .note(markup! {
                "Inline styles make code harder to maintain, reduce reusability, and can prevent effective use of a strict Content Security Policy."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();
        match node {
            NoInlineStylesQuery::JsxAttribute(jsx_attribute) => {
                mutation.remove_node(jsx_attribute.clone());
                Some(JsRuleAction::new(
                    ctx.metadata().action_category(ctx.category(), ctx.group()),
                    ctx.metadata().applicability(),
                    markup! { "Remove the "<Emphasis>"style"</Emphasis>" attribute." }.to_owned(),
                    mutation,
                ))
            }
            NoInlineStylesQuery::JsCallExpression(call_expression) => {
                let model = ctx.model();
                let property_member = find_style_attribute(call_expression, model)?;

                mutation.remove_js_object_member(AnyJsObjectMember::JsPropertyObjectMember(
                    property_member,
                ));
                Some(JsRuleAction::new(
                    ctx.metadata().action_category(ctx.category(), ctx.group()),
                    ctx.metadata().applicability(),
                    markup! { "Remove the "<Emphasis>"style"</Emphasis>" attribute." }.to_owned(),
                    mutation,
                ))
            }
        }
    }
}

declare_node_union! {
    pub NoInlineStylesQuery = JsxAttribute | JsCallExpression
}

fn is_style_value(value_token: &JsSyntaxToken) -> bool {
    value_token.text_trimmed().to_lowercase_cow() == "style"
}

fn find_style_attribute(
    call_expression: &JsCallExpression,
    model: &SemanticModel,
) -> Option<JsPropertyObjectMember> {
    let react_create_element =
        ReactCreateElementCall::from_call_expression(call_expression, model)?;
    let ReactCreateElementCall { props, .. } = react_create_element;

    let props = props?;
    for member in props.members() {
        let member = member.ok()?;
        let property_member = member.as_js_property_object_member()?;
        let name = property_member.name().ok()?;
        let name = name.as_js_literal_member_name()?;
        let value = name.value().ok()?;
        if is_style_value(&value) {
            return Some(property_member.clone());
        }
    }

    None
}
