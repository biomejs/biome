use crate::react::{ReactApiCall, ReactCreateElementCall};
use crate::semantic_services::Semantic;
use crate::JsRuleAction;
use biome_analyze::context::RuleContext;
use biome_analyze::{declare_rule, FixKind, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_factory::make;
use biome_js_factory::make::{jsx_string, jsx_string_literal};
use biome_js_semantic::SemanticModel;
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_js_syntax::{
    AnyJsLiteralExpression, AnyJsxAttributeValue, JsCallExpression, JsNumberLiteralExpression,
    JsPropertyObjectMember, JsStringLiteralExpression, JsUnaryExpression, JsxAttribute, TextRange,
};
use biome_rowan::{declare_node_union, AstNode, BatchMutationExt};

declare_rule! {
    /// Prevent the usage of positive integers on `tabIndex` property
    ///
    /// Avoid positive `tabIndex` property values to synchronize the flow of the page with keyboard tab order.
    /// ## Accessibility guidelines
    ///
    /// [WCAG 2.4.3](https://www.w3.org/WAI/WCAG21/Understanding/focus-order)
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <div tabIndex={1}>foo</div>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <div tabIndex={"1"} />
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// React.createElement("div", { tabIndex: 1 })
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <div tabIndex="0" />
    /// ```
    ///
    /// ```js
    /// React.createElement("div", { tabIndex: -1 })
    /// ```
    pub(crate) NoPositiveTabindex {
        version: "1.0.0",
        name: "noPositiveTabindex",
        source: RuleSource::EslintJsxA11y("tabindex-no-positive"),
        recommended: true,
        fix_kind: FixKind::Unsafe,
    }
}

declare_node_union! {
    pub(crate) TabindexProp = JsxAttribute | JsPropertyObjectMember
}

declare_node_union! {
    pub(crate) NoPositiveTabindexQuery = AnyJsxElement | JsCallExpression
}

declare_node_union! {
    /// Subset of expressions supported by this rule.
    ///
    /// ## Examples
    ///
    /// - `JsStringLiteralExpression` &mdash; `"5"`
    /// - `JsNumberLiteralExpression` &mdash; `5`
    /// - `JsUnaryExpression` &mdash; `+5` | `-5`
    ///
    pub(crate) AnyNumberLikeExpression = JsStringLiteralExpression | JsNumberLiteralExpression | JsUnaryExpression
}

impl NoPositiveTabindexQuery {
    fn find_tabindex_attribute(&self, model: &SemanticModel) -> Option<TabindexProp> {
        match self {
            NoPositiveTabindexQuery::AnyJsxElement(jsx) => jsx
                .find_attribute_by_name("tabIndex")
                .map(TabindexProp::from),
            NoPositiveTabindexQuery::JsCallExpression(expression) => {
                let react_create_element =
                    ReactCreateElementCall::from_call_expression(expression, model)?;
                react_create_element
                    .find_prop_by_name("tabIndex")
                    .map(TabindexProp::from)
            }
        }
    }
}

impl AnyNumberLikeExpression {
    /// Returns the value of a number-like expression; it returns the expression
    /// text for literal expressions. However, for unary expressions, it only
    /// returns the value for signed numeric expressions.
    pub(crate) fn value(&self) -> Option<String> {
        match self {
            AnyNumberLikeExpression::JsStringLiteralExpression(string_literal) => {
                return Some(string_literal.inner_string_text().ok()?.to_string());
            }
            AnyNumberLikeExpression::JsNumberLiteralExpression(number_literal) => {
                return Some(number_literal.value_token().ok()?.to_string());
            }
            AnyNumberLikeExpression::JsUnaryExpression(unary_expression) => {
                if unary_expression.is_signed_numeric_literal().ok()? {
                    return Some(unary_expression.text());
                }
            }
        }

        None
    }
}

impl Rule for NoPositiveTabindex {
    type Query = Semantic<NoPositiveTabindexQuery>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();
        let tabindex_attribute = node.find_tabindex_attribute(model)?;

        match tabindex_attribute {
            TabindexProp::JsxAttribute(jsx_attribute) => {
                let jsx_any_attribute_value = jsx_attribute.initializer()?.value().ok()?;

                if !attribute_has_valid_tabindex(&jsx_any_attribute_value)? {
                    return Some(jsx_any_attribute_value.syntax().text_trimmed_range());
                }
            }
            TabindexProp::JsPropertyObjectMember(js_object_member) => {
                let expression = js_object_member.value().ok()?;
                let expression_syntax_node = expression.syntax();
                let expression_value =
                    AnyNumberLikeExpression::cast_ref(expression_syntax_node)?.value()?;

                if !is_tabindex_valid(&expression_value) {
                    return Some(expression_syntax_node.text_trimmed_range());
                }
            }
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            state,
            markup!{"Avoid positive values for the "<Emphasis>"tabIndex"</Emphasis>" prop."}.to_owned(),
        )
        .note(
            markup!{
                "Elements with a positive "<Emphasis>"tabIndex"</Emphasis>" override natural page content order. This causes elements without a positive tab index to come last when navigating using a keyboard."
            }.to_owned(),
        )
        .note(
            markup!{
                "Use only 0 and -1 as "<Emphasis>"tabIndex"</Emphasis>" values. Avoid using "<Emphasis>"tabIndex"</Emphasis>" values greater than 0 and CSS properties that can change the order of focusable HTML elements."
            }
        );

        Some(diagnostic)
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let element = ctx.query();
        let model = ctx.model();
        let tabindex_attribute = element.find_tabindex_attribute(model)?;

        let mut mutation = ctx.root().begin();
        match tabindex_attribute {
            TabindexProp::JsxAttribute(jsx_attribute) => {
                let prev_val = jsx_attribute.initializer()?.value().ok()?;
                let new_val = AnyJsxAttributeValue::JsxString(jsx_string(jsx_string_literal("0")));
                mutation.replace_node(prev_val, new_val);
            }
            TabindexProp::JsPropertyObjectMember(js_object_member) => {
                let prev_val = js_object_member.value().ok()?;
                let new_val = biome_js_syntax::AnyJsExpression::AnyJsLiteralExpression(
                    AnyJsLiteralExpression::JsStringLiteralExpression(
                        make::js_string_literal_expression(jsx_string_literal("0")),
                    ),
                );
                mutation.replace_node(prev_val, new_val);
            }
        };

        Some(JsRuleAction {
            category: biome_analyze::ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message:
                markup! { "Replace the "<Emphasis>"tableIndex"</Emphasis>" prop value with 0." }
                    .to_owned(),
            mutation,
        })
    }
}

/// Verify that a JSX attribute value has a valid tab index, meaning it is not positive.
fn attribute_has_valid_tabindex(jsx_any_attribute_value: &AnyJsxAttributeValue) -> Option<bool> {
    match jsx_any_attribute_value {
        AnyJsxAttributeValue::JsxString(jsx_string) => {
            let value = jsx_string.inner_string_text().ok()?.to_string();
            Some(is_tabindex_valid(&value))
        }
        AnyJsxAttributeValue::JsxExpressionAttributeValue(value) => {
            let expression = value.expression().ok()?;
            let expression_value =
                AnyNumberLikeExpression::cast_ref(expression.syntax())?.value()?;

            Some(is_tabindex_valid(&expression_value))
        }
        _ => None,
    }
}

/// Verify if number string is an integer less than equal zero. Non-integer numbers
/// are considered valid.
fn is_tabindex_valid(number_like_string: &str) -> bool {
    let number_string_result = number_like_string.trim().parse::<i32>();

    match number_string_result {
        Ok(number) => number <= 0,
        Err(_) => true,
    }
}
