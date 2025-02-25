use biome_analyze::RuleSource;
use biome_analyze::{context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::AnyJsTemplateElement;
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, JsBinaryExpression, JsBinaryOperator,
    JsParenthesizedExpression, JsStringLiteralExpression, JsSyntaxKind, JsSyntaxToken,
    JsTemplateElementList, JsTemplateExpression, T,
};
use biome_rowan::{AstNode, BatchMutationExt, WalkEvent};

use crate::JsRuleAction;

declare_lint_rule! {
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
    pub UseTemplate {
        version: "1.0.0",
        name: "useTemplate",
        language: "js",
        sources: &[RuleSource::Eslint("prefer-template")],
        recommended: false,
        severity: Severity::Warning,
        fix_kind: FixKind::Unsafe,
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
                <Emphasis>"Template"</Emphasis>" literals are preferred over "<Emphasis>"string concatenation."</Emphasis>
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
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use a "<Emphasis>"template literal"</Emphasis>"." }.to_owned(),
            mutation,
        ))
    }
}

/// Returns true if `node` can be converted to a template literal.
///
/// This is the case, if:
///
/// - the binary expression contains the `+` operator,
/// - the binary expression contains a string-like literal and a non-string-like or one of them and
///   an interpolated template literal
///
/// String-like literals are string literals and untagged template literals.
fn can_be_template_literal(node: &JsBinaryExpression) -> Option<bool> {
    let mut iter = node.syntax().preorder();
    let mut has_constant_string_constituent = false;
    let mut has_interpolated_string_constituent = false;
    let mut has_non_constant_string_constituent = false;
    while let Some(walk) = iter.next() {
        let WalkEvent::Enter(node) = walk else {
            continue;
        };
        let expression = AnyJsExpression::cast(node)?;
        match &expression {
            AnyJsExpression::JsParenthesizedExpression(_) => continue,
            AnyJsExpression::JsBinaryExpression(binary)
                if binary.operator() == Ok(JsBinaryOperator::Plus) =>
            {
                // Long literals can be split to multiline using `+`, so let's ignore that.
                // see https://github.com/biomejs/biome/issues/4947
                if binary.operator_token().ok()?.has_leading_newline()
                    || binary.right().ok()?.syntax().has_leading_newline()
                {
                    return Some(false);
                }

                continue;
            }
            AnyJsExpression::JsTemplateExpression(template) if template.is_constant() => {
                has_constant_string_constituent = true;
            }
            AnyJsExpression::JsTemplateExpression(template) if template.tag().is_none() => {
                has_interpolated_string_constituent = true;
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
        if (has_constant_string_constituent
            && (has_non_constant_string_constituent || has_interpolated_string_constituent))
            || (has_interpolated_string_constituent && has_non_constant_string_constituent)
        {
            return Some(true);
        }
        iter.skip_subtree();
    }
    Some(false)
}

fn template_expression_from_binary_expression(
    node: &JsBinaryExpression,
) -> Option<JsTemplateExpression> {
    // For each nested binary expression (with operator `+`), we keep track of the last left expressions
    // and whether it evaluates to a string.
    // Once we see a string/template literal, we insert all of `left_expressions_stack` in
    // `template_elements` and the seen string/template literal.
    // Any subsequent expression is directly inserted in `template_elements` if its parent
    // evaluates to a string.
    let mut template_elements = vec![];
    let mut left_expressions_stack = vec![];
    let mut binary_evaluates_to_string_stack = vec![];

    let mut iter = node.syntax().preorder();
    while let Some(walk) = iter.next() {
        match walk {
            WalkEvent::Enter(node) => match AnyJsExpression::cast(node)? {
                AnyJsExpression::JsParenthesizedExpression(_) => {}
                AnyJsExpression::JsBinaryExpression(ref binary)
                    if binary.operator() == Ok(JsBinaryOperator::Plus) =>
                {
                    left_expressions_stack.push(vec![]);
                    binary_evaluates_to_string_stack.push(false);
                }
                _ => iter.skip_subtree(),
            },
            WalkEvent::Leave(node) => {
                let expression = AnyJsExpression::cast(node)?;
                match &expression {
                    // Skip parenthesized expressions, because they would be added twice to
                    // `left_expressions_stack` or `template_elements` (see the last match arm):
                    // First the contained expression and then the parenthesized expression itself.
                    AnyJsExpression::JsParenthesizedExpression(_) => continue,
                    AnyJsExpression::JsBinaryExpression(binary)
                        if binary.operator() == Ok(JsBinaryOperator::Plus) =>
                    {
                        left_expressions_stack.pop()?;

                        if binary_evaluates_to_string_stack.pop()? {
                            if let Some(parent_evaluates_to_string) =
                                binary_evaluates_to_string_stack.last_mut()
                            {
                                *parent_evaluates_to_string = true;
                            }
                        } else if !template_elements.is_empty()
                            && *binary_evaluates_to_string_stack.last().unwrap_or(&false)
                        {
                            template_elements.push(template_element_from(expression)?)
                        } else if let Some(left_expressions) = left_expressions_stack.last_mut() {
                            left_expressions.push(expression)
                        }
                    }
                    AnyJsExpression::JsTemplateExpression(template) if template.tag().is_none() => {
                        *binary_evaluates_to_string_stack.last_mut()? = true;

                        for left_expression in
                            left_expressions_stack.iter_mut().flat_map(|v| v.drain(..))
                        {
                            template_elements.push(template_element_from(left_expression)?)
                        }
                        flatten_template_element_list(&mut template_elements, template.elements())?;
                    }
                    AnyJsExpression::AnyJsLiteralExpression(
                        AnyJsLiteralExpression::JsStringLiteralExpression(string_literal),
                    ) => {
                        *binary_evaluates_to_string_stack.last_mut()? = true;

                        for left_expression in
                            left_expressions_stack.iter_mut().flat_map(|v| v.drain(..))
                        {
                            template_elements.push(template_element_from(left_expression)?)
                        }
                        template_elements.push(template_chunk_from(string_literal)?);
                    }
                    _expression => {
                        if !template_elements.is_empty()
                            && *binary_evaluates_to_string_stack.last()?
                        {
                            template_elements.push(template_element_from(expression)?)
                        } else {
                            left_expressions_stack.last_mut()?.push(expression)
                        }
                    }
                }
            }
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

fn template_chunk_from(string_literal: &JsStringLiteralExpression) -> Option<AnyJsTemplateElement> {
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
            AnyJsTemplateElement::JsTemplateElement(ref ele) => {
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
