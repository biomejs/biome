use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
    options::PreferredQuote,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsAssignment, AnyJsBindingPattern, AnyJsExpression, AnyJsLiteralExpression,
    AnyJsMemberExpression, AnyJsObjectBindingPatternMember, JsAssignmentExpression,
    JsObjectBindingPatternProperty, JsObjectBindingPatternShorthandProperty, JsSyntaxKind, T,
    inner_string_text,
};
use biome_rowan::{AstNode, BatchMutationExt, TextRange, declare_node_union};
use biome_rule_options::use_dom_node_text_content::UseDomNodeTextContentOptions;

use crate::JsRuleAction;

const INNER_TEXT: &str = "innerText";
const TEXT_CONTENT: &str = "textContent";

declare_lint_rule! {
    /// Prefer `.textContent` over `.innerText` for DOM node text.
    ///
    /// Because `innerText` depends on rendered layout and CSS, it should only be used when you specifically need that behavior.
    /// `textContent` is usually faster and more predictable than `innerText`.
    ///
    /// :::note
    /// `textContent` and `innerText` are not equivalent.
    /// See the [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/API/Node/textContent#differences_from_innertext) for the differences between them.
    /// :::
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const text = node.innerText;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const {innerText} = node;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// node["innerText"] = "Biome";
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const text = node.textContent;
    /// ```
    ///
    /// ```js
    /// const {textContent} = node;
    /// ```
    ///
    pub UseDomNodeTextContent {
        version: "2.4.13",
        name: "useDomNodeTextContent",
        language: "js",
        recommended: true,
        sources: &[RuleSource::EslintUnicorn("prefer-dom-node-text-content").same()],
        fix_kind: FixKind::Unsafe,
    }
}

declare_node_union! {
    pub AnyUseDomNodeTextContentQuery =
        AnyJsMemberExpression |
        JsAssignmentExpression |
        JsObjectBindingPatternProperty |
        JsObjectBindingPatternShorthandProperty
}

impl Rule for UseDomNodeTextContent {
    type Query = Ast<AnyUseDomNodeTextContentQuery>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = UseDomNodeTextContentOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        ctx.query().run()
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                *state,
                markup! {
                    <Emphasis>"innerText"</Emphasis>" is not recommended for reading DOM node text."
                },
            )
            .note(markup! {
                <Emphasis>"innerText"</Emphasis>" depends on rendered layout and CSS, so it can be slower and produce different results from the DOM tree's text content."
            })
            .note(markup! {
                "Use "<Emphasis>"textContent"</Emphasis>" when you want the node's text without rendered-text behavior."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();

        match ctx.query() {
            AnyUseDomNodeTextContentQuery::AnyJsMemberExpression(node) => match node {
                AnyJsMemberExpression::JsStaticMemberExpression(node) => {
                    mutation.replace_element(
                        node.member().ok()?.into(),
                        make::js_name(make::ident(TEXT_CONTENT)).into(),
                    );
                }
                AnyJsMemberExpression::JsComputedMemberExpression(node) => {
                    let member = node.member().ok()?;
                    let preferred_quote = ctx.preferred_quote();
                    mutation.replace_node(
                        member.clone(),
                        replacement_string_expression(&member, preferred_quote)?,
                    );
                }
            },
            AnyUseDomNodeTextContentQuery::JsAssignmentExpression(node) => {
                match node.left().ok()?.as_any_js_assignment()? {
                    AnyJsAssignment::JsStaticMemberAssignment(node) => {
                        mutation.replace_element(
                            node.member().ok()?.into(),
                            make::js_name(make::ident(TEXT_CONTENT)).into(),
                        );
                    }
                    AnyJsAssignment::JsComputedMemberAssignment(node) => {
                        let member = node.member().ok()?;
                        let preferred_quote = ctx.preferred_quote();
                        mutation.replace_node(
                            member.clone(),
                            replacement_string_expression(&member, preferred_quote)?,
                        );
                    }
                    _ => return None,
                }
            }
            AnyUseDomNodeTextContentQuery::JsObjectBindingPatternProperty(node) => {
                mutation.replace_element(
                    node.member().ok()?.into(),
                    make::js_literal_member_name(make::ident(TEXT_CONTENT)).into(),
                );
            }
            AnyUseDomNodeTextContentQuery::JsObjectBindingPatternShorthandProperty(node) => {
                let mut replacement = make::js_object_binding_pattern_property(
                    make::js_literal_member_name(make::ident(TEXT_CONTENT)).into(),
                    make::token_with_trailing_space(T![:]),
                    AnyJsBindingPattern::AnyJsBinding(node.identifier().ok()?),
                );

                if let Some(init) = node.init() {
                    replacement = replacement.with_init(init);
                }

                mutation.replace_node(
                    AnyJsObjectBindingPatternMember::from(node.clone()),
                    replacement.build().into(),
                );
            }
        }

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! {
                "Use "<Emphasis>"textContent"</Emphasis>" instead."
            }
            .to_owned(),
            mutation,
        ))
    }
}

impl AnyUseDomNodeTextContentQuery {
    fn run(&self) -> Option<TextRange> {
        match self {
            Self::AnyJsMemberExpression(node) => inner_text_member_expression_range(node),
            Self::JsAssignmentExpression(node) => inner_text_assignment_range(node),
            Self::JsObjectBindingPatternProperty(node) => inner_text_binding_property_range(node),
            Self::JsObjectBindingPatternShorthandProperty(node) => {
                inner_text_shorthand_binding_property_range(node)
            }
        }
    }
}

/// Returns the range of `innerText` when used in a member expression.
///
/// ## Examples
///
/// ```js
/// node.innerText;
/// //   ^^^^^^^^^
/// node["innerText"];
/// //   ^^^^^^^^^^^
/// ```
fn inner_text_member_expression_range(node: &AnyJsMemberExpression) -> Option<TextRange> {
    match node {
        AnyJsMemberExpression::JsStaticMemberExpression(node) => {
            let member = node.member().ok()?;
            let token = member.value_token().ok()?;
            if token.text_trimmed() == INNER_TEXT {
                Some(member.range())
            } else {
                None
            }
        }
        AnyJsMemberExpression::JsComputedMemberExpression(node) => {
            let member = node.member().ok()?;
            if is_inner_text_literal_expression(&member) {
                Some(member.range())
            } else {
                None
            }
        }
    }
}

/// Returns the range of `innerText` when used on the left side of an assignment.
///
/// ## Examples
///
/// ```js
/// node.innerText = value;
/// //   ^^^^^^^^^
/// node["innerText"] = value;
/// //   ^^^^^^^^^^^
/// ```
fn inner_text_assignment_range(node: &JsAssignmentExpression) -> Option<TextRange> {
    match node.left().ok()?.as_any_js_assignment()? {
        AnyJsAssignment::JsStaticMemberAssignment(node) => {
            let member = node.member().ok()?.as_js_name()?.value_token().ok()?;
            if member.text_trimmed() == INNER_TEXT {
                Some(member.text_trimmed_range())
            } else {
                None
            }
        }
        AnyJsAssignment::JsComputedMemberAssignment(node) => {
            let member = node.member().ok()?;
            if is_inner_text_literal_expression(&member) {
                Some(member.range())
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Returns the range of `innerText` in an object binding property.
///
/// ## Examples
///
/// ```js
/// const { innerText: textContent } = node;
/// //      ^^^^^^^^^
/// ```
fn inner_text_binding_property_range(node: &JsObjectBindingPatternProperty) -> Option<TextRange> {
    let binding = node.member().ok()?;
    let member = binding.as_js_literal_member_name()?;
    if member.value().ok()?.kind() == JsSyntaxKind::IDENT
        && member.name().ok()?.text() == INNER_TEXT
    {
        Some(member.range())
    } else {
        None
    }
}

/// Returns the range of `innerText` in a shorthand object binding property.
///
/// ## Examples
///
/// ```js
/// const { innerText } = node;
/// //      ^^^^^^^^^
/// ```
fn inner_text_shorthand_binding_property_range(
    node: &JsObjectBindingPatternShorthandProperty,
) -> Option<TextRange> {
    let binding = node.identifier().ok()?;
    let identifier = binding.as_js_identifier_binding()?;
    let token = identifier.name_token().ok()?;
    if token.text_trimmed() == INNER_TEXT {
        Some(token.text_trimmed_range())
    } else {
        None
    }
}

fn is_inner_text_literal_expression(member: &AnyJsExpression) -> bool {
    let member = member.clone().omit_parentheses();
    let Some(literal) = member.as_any_js_literal_expression() else {
        return false;
    };
    let Ok(token) = literal.value_token() else {
        return false;
    };

    token.kind() == JsSyntaxKind::JS_STRING_LITERAL
        && inner_string_text(&token).text() == INNER_TEXT
}

fn replacement_string_expression(
    member: &AnyJsExpression,
    preferred_quote: PreferredQuote,
) -> Option<AnyJsExpression> {
    let member = member.clone().omit_parentheses();
    let _ = member.as_any_js_literal_expression()?;

    let replacement_token = if preferred_quote.is_double() {
        make::js_string_literal(TEXT_CONTENT)
    } else {
        make::js_string_literal_single_quotes(TEXT_CONTENT)
    };

    Some(AnyJsExpression::AnyJsLiteralExpression(
        AnyJsLiteralExpression::JsStringLiteralExpression(make::js_string_literal_expression(
            replacement_token,
        )),
    ))
}
