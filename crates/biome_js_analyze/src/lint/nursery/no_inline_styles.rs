use crate::JsRuleAction;
use crate::react::ReactCreateElementCall;
use crate::services::semantic::Semantic;
use crate::utils::batch::JsBatchMutation;
use biome_analyze::context::RuleContext;
use biome_analyze::{FixKind, Rule, RuleDiagnostic, declare_lint_rule};
use biome_console::markup;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::jsx_ext::AnyJsxElement;
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
    /// - [Content Security Policy: Allowing inline styles](https://content-security-policy.com/examples/allow-inline-style)
    ///
    pub NoInlineStyles {
        version: "2.4.9",
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
            NoInlineStylesQuery::AnyJsxElement(element) => {
                let attribute = find_style_attribute(element)?;
                Some(attribute.range())
            }
            NoInlineStylesQuery::JsCallExpression(call_expression) => {
                let model = ctx.model();
                let object_member = find_style_object_member(call_expression, model)?;
                Some(object_member.range())
            }
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state,
                markup! {
                    "Unexpected "<Emphasis>"style"</Emphasis>" attribute."
                },
            )
            .note(markup! {
                "Inline styles make code harder to maintain, reduce reusability, and can prevent effective use of a strict Content Security Policy. Use external CSS classes or stylesheets instead."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();
        match node {
            NoInlineStylesQuery::AnyJsxElement(element) => {
                let attribute = find_style_attribute(element)?;
                mutation.remove_node(attribute);
            }
            NoInlineStylesQuery::JsCallExpression(call_expression) => {
                let model = ctx.model();
                let object_member = find_style_object_member(call_expression, model)?;
                mutation.remove_js_object_member(AnyJsObjectMember::JsPropertyObjectMember(
                    object_member,
                ));
            }
        }

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the "<Emphasis>"style"</Emphasis>" attribute." }.to_owned(),
            mutation,
        ))
    }
}

declare_node_union! {
    pub NoInlineStylesQuery = AnyJsxElement | JsCallExpression
}

fn is_style_value(value_token: &JsSyntaxToken) -> bool {
    value_token.text_trimmed().to_lowercase_cow() == "style"
}

fn find_style_object_member(
    call_expression: &JsCallExpression,
    model: &SemanticModel,
) -> Option<JsPropertyObjectMember> {
    let react_create_element =
        ReactCreateElementCall::from_call_expression(call_expression, model)?;

    if react_create_element.is_custom_component() {
        return None;
    }

    let ReactCreateElementCall { props, .. } = react_create_element;
    let props = props?;

    for member in props.members() {
        if let Some(member) = member.ok()
            && let Some(member) = member.as_js_property_object_member()
            && let Some(name) = member.name().ok()
            && let Some(name) = name.as_js_literal_member_name()
            && let Some(value) = name.value().ok()
            && is_style_value(&value)
        {
            return Some(member.clone());
        }
    }

    None
}

fn find_style_attribute(any_opening: &AnyJsxElement) -> Option<JsxAttribute> {
    if any_opening.is_custom_component() {
        return None;
    }

    for attribute in any_opening.attributes() {
        if let Some(attribute) = attribute.as_jsx_attribute()
            && let Some(name) = attribute.name().ok()
            && let Some(name) = name.as_jsx_name()
            && let Some(value_token) = name.value_token().ok()
            && is_style_value(&value_token)
        {
            return Some(attribute.clone());
        }
    }

    None
}
