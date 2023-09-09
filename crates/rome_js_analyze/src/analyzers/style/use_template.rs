use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic,
};
use biome_console::markup;
use biome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::AnyJsTemplateElement::{self, JsTemplateElement};
use rome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, JsBinaryExpression, JsBinaryOperator,
    JsParenthesizedExpression, JsStringLiteralExpression, JsSyntaxKind, JsSyntaxToken,
    JsTemplateElementList, JsTemplateExpression, T,
};
use rome_rowan::{AstNode, BatchMutationExt, WalkEvent};

use crate::JsRuleAction;

declare_rule! {
    /// Prefer template literals over string concatenation.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const s = foo + "baz";
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const s = 1 + 2 + "foo" + 3;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const s = 1 * 2 + "foo";
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const s = 1 + "foo" + 2 + "bar" + "baz" + 3;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// let s = "foo" + "bar" + `baz`;
    /// ```
    ///
    /// ```js
    /// let s = `value: ${1}`;
    /// ```
    pub(crate) UseTemplate {
        version: "1.0.0",
        name: "useTemplate",
        recommended: true,
    }
}

impl Rule for UseTemplate {
    type Query = Ast<JsBinaryExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        // Do not handle binary operations contained in a binary operation with operator `+`
        if node
            .syntax()
            .ancestors()
            .skip(1) // skip node
            .find(|x| !JsParenthesizedExpression::can_cast(x.kind()))
            .and_then(JsBinaryExpression::cast)
            .is_some_and(|parent| parent.operator() == Ok(JsBinaryOperator::Plus))
        {
            return None;
        }
        can_be_template_literal(node)?.then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
                ""<Emphasis>"Template"</Emphasis>" literals are preferred over "<Emphasis>"string concatenation."</Emphasis>""
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();
        let template = template_expression_from_binary_expression(node)?;
        mutation.replace_node(
            AnyJsExpression::JsBinaryExpression(node.clone()),
            AnyJsExpression::JsTemplateExpression(template),
        );
        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Use a "<Emphasis>"template literal"</Emphasis>"." }.to_owned(),
            mutation,
        })
    }
}

/// Returns true if `node` can be converted to a template literal.
///
/// This is the case, if:
///
/// - the binary expression contains the `+` operator,
/// - the binary expression contains a string-like literal and a non-string-like
///
/// String-like literals are string literals and untagged template literals.
fn can_be_template_literal(node: &JsBinaryExpression) -> Option<bool> {
    let mut iter = node.syntax().preorder();
    let mut has_constant_string_constituent = false;
    let mut has_non_constant_string_constituent = false;
    while let Some(walk) = iter.next() {
        if let WalkEvent::Enter(node) = walk {
            let node = AnyJsExpression::cast(node)?;
            match node {
                AnyJsExpression::JsParenthesizedExpression(_) => continue,
                AnyJsExpression::JsBinaryExpression(node)
                    if node.operator().ok()? == JsBinaryOperator::Plus =>
                {
                    continue
                }
                AnyJsExpression::JsTemplateExpression(ref template) if template.is_constant() => {
                    has_constant_string_constituent = true;
                }
                AnyJsExpression::AnyJsLiteralExpression(
                    AnyJsLiteralExpression::JsStringLiteralExpression(_),
                ) => {
                    has_constant_string_constituent = true;
                }
                _ => {
                    has_non_constant_string_constituent = true;
                }
            }
            if has_non_constant_string_constituent && has_constant_string_constituent {
                return Some(true);
            }
            iter.skip_subtree();
        }
    }
    Some(false)
}

fn template_expression_from_binary_expression(
    node: &JsBinaryExpression,
) -> Option<JsTemplateExpression> {
    // While `template_elements` is empty, we keep track of the last left node.
    // Once we see the first string/template literal,
    // we insert `last_left_node` in `template_elements` and the seen string/template literal.
    // ANy subsequent expression is directly inserted in `template_elements`.
    let mut template_elements = vec![];
    let mut last_left_node = None;
    let mut iter = node.syntax().preorder();
    while let Some(walk) = iter.next() {
        match walk {
            WalkEvent::Enter(node) => {
                let node = AnyJsExpression::cast(node)?;
                match node {
                    AnyJsExpression::JsParenthesizedExpression(_) => continue,
                    AnyJsExpression::JsBinaryExpression(node)
                        if matches!(node.operator().ok()?, JsBinaryOperator::Plus) =>
                    {
                        continue;
                    }
                    AnyJsExpression::JsTemplateExpression(ref template)
                        if template.tag().is_none() =>
                    {
                        if let Some(last_node) = last_left_node.take() {
                            template_elements.push(template_element_from(last_node)?)
                        }
                        flatten_template_element_list(&mut template_elements, template.elements())?;
                    }
                    AnyJsExpression::AnyJsLiteralExpression(
                        AnyJsLiteralExpression::JsStringLiteralExpression(string_literal),
                    ) => {
                        if let Some(last_node) = last_left_node.take() {
                            template_elements.push(template_element_from(last_node)?)
                        }
                        template_elements.push(template_chuck_from(&string_literal)?);
                    }
                    node if !template_elements.is_empty() => {
                        template_elements.push(template_element_from(node)?)
                    }
                    _ => {}
                }
                iter.skip_subtree();
            }
            WalkEvent::Leave(node) if template_elements.is_empty() => {
                last_left_node = AnyJsExpression::cast(node);
            }
            _ => {}
        }
    }
    Some(
        make::js_template_expression(
            make::token(T!['`']),
            make::js_template_element_list(template_elements),
            make::token(T!['`']),
        )
        .build(),
    )
}

fn template_chuck_from(string_literal: &JsStringLiteralExpression) -> Option<AnyJsTemplateElement> {
    let text = string_literal.inner_string_text().ok()?;
    Some(AnyJsTemplateElement::from(make::js_template_chunk_element(
        make::js_template_chunk(text.text()),
    )))
}

fn template_element_from(expr: AnyJsExpression) -> Option<AnyJsTemplateElement> {
    Some(AnyJsTemplateElement::from(make::js_template_element(
        JsSyntaxToken::new_detached(JsSyntaxKind::DOLLAR_CURLY, "${", [], []),
        expr.with_leading_trivia_pieces([])?
            .with_trailing_trivia_pieces([])?,
        make::token(T!['}']),
    )))
}

/// Flatten a [JsTemplateElementList] of [JsTemplate] which could possibly be recursive, into a `Vec<JsAnyTemplateElement>`
/// ## Example
/// flatten
/// ```js
/// `${1 + 2 + `${a}test` }bar`
/// ```
/// into
/// `[1, 2, a, "test", "bar"]`
fn flatten_template_element_list(
    result: &mut Vec<AnyJsTemplateElement>,
    list: JsTemplateElementList,
) -> Option<()> {
    for element in list {
        match element {
            AnyJsTemplateElement::JsTemplateChunkElement(_) => result.push(element),
            JsTemplateElement(ref ele) => {
                let expr = ele.expression().ok()?;
                match expr {
                    AnyJsExpression::JsTemplateExpression(template) => {
                        flatten_template_element_list(result, template.elements())?;
                    }
                    _ => {
                        result.push(element);
                    }
                }
            }
        }
    }
    Some(())
}
