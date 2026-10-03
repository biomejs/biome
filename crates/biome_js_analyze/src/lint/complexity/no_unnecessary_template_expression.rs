use crate::JsRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, AnyJsTemplateElement, JsTemplateExpression,
    inner_string_text,
};
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt};
use biome_rule_options::no_unnecessary_template_expression::NoUnnecessaryTemplateExpressionOptions;

declare_lint_rule! {
    /// Disallow unnecessary template expressions.
    ///
    /// A template expression (or template literal) is unnecessary when it only contains
    /// string literal expressions that could be written as a regular string literal instead.
    ///
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const a = `${'hello'}`;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const b = `${"world"}`;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const c = `${'hello'}${'world'}`;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const d = `prefix_${'suffix'}`;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// // Template with a non-string-literal expression
    /// const a = `${someVariable}`;
    /// ```
    ///
    /// ```js
    /// // Template with a non-string-literal interpolation mixed with text
    /// const b = `Hello, ${name}!`;
    /// ```
    ///
    /// ```js
    /// // Tagged templates are never flagged
    /// const c = html`${'foo'}`;
    /// ```
    ///
    /// ```js
    /// // Templates with newlines in the text part need the template syntax
    /// const d = `line one
    /// ${'line two'}`;
    /// ```
    ///
    pub NoUnnecessaryTemplateExpression {
        version: "2.4.13",
        name: "noUnnecessaryTemplateExpression",
        language: "js",
        recommended: false,
        severity: Severity::Information,
        sources: &[RuleSource::EslintTypeScript("no-unnecessary-template-expression").inspired()],
        fix_kind: FixKind::Safe,
    }
}

#[derive(Debug)]
pub struct TemplateState {
    /// Whether any fragment contains a single quote.
    has_single_quote: bool,
    /// Whether any fragment contains a double quote.
    has_double_quote: bool,
}

impl Rule for NoUnnecessaryTemplateExpression {
    type Query = Ast<JsTemplateExpression>;
    type State = TemplateState;
    type Signals = Option<Self::State>;
    type Options = NoUnnecessaryTemplateExpressionOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let template = ctx.query();

        // Tagged templates (e.g. html`${'foo'}`) should never be flagged.
        if template.tag().is_some() {
            return None;
        }

        // Collect all elements and check whether each is flaggable.
        // We flag the template if every interpolation is a string literal expression
        // AND the text chunks don't contain literal newlines (which require the template syntax).
        let mut has_any_interpolation = false;
        let mut has_single_quote = false;
        let mut has_double_quote = false;

        for element in template.elements().iter() {
            match element {
                AnyJsTemplateElement::JsTemplateChunkElement(chunk) => {
                    let token = chunk.template_chunk_token().ok()?;
                    let text = token.text_trimmed();

                    // Literal newlines in chunk text make the template necessary: a plain string
                    // literal cannot span multiple lines without an escape sequence.
                    if text.contains('\n') || text.contains('\r') {
                        return None;
                    }

                    has_single_quote |= text.contains('\'');
                    has_double_quote |= text.contains('"');
                }
                AnyJsTemplateElement::JsTemplateElement(elem) => {
                    has_any_interpolation = true;
                    let expr = elem.expression().ok()?;

                    match &expr {
                        AnyJsExpression::AnyJsLiteralExpression(
                            AnyJsLiteralExpression::JsStringLiteralExpression(string_lit),
                        ) => {
                            let value_token = string_lit.value_token().ok()?;
                            // inner_string_text strips the surrounding quotes while preserving
                            // the escape sequences exactly as written in the source.
                            let inner = inner_string_text(&value_token);
                            let inner_text = inner.text();
                            has_single_quote |= inner_text.contains('\'');
                            has_double_quote |= inner_text.contains('"');
                        }
                        // Any expression that is not a string literal makes the template necessary
                        // (e.g. a number literal or a variable whose type we don't know).
                        _ => return None,
                    }
                }
            }
        }

        // Only report when at least one interpolation was found; templates with zero
        // interpolations are handled by the `noUnusedTemplateLiteral` rule.
        if !has_any_interpolation {
            return None;
        }

        Some(TemplateState {
            has_single_quote,
            has_double_quote,
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "This template expression is unnecessary."
                },
            )
            .note(markup! {
                "The template only contains string literal expressions. A regular string literal can be used instead."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        // A string literal cannot represent both quote kinds without escaping.
        if state.has_single_quote && state.has_double_quote {
            return None;
        }

        let template = ctx.query();
        let mut content = String::new();
        for element in template.elements().iter() {
            match element {
                AnyJsTemplateElement::JsTemplateChunkElement(chunk) => {
                    let token = chunk.template_chunk_token().ok()?;
                    content.push_str(token.text_trimmed());
                }
                AnyJsTemplateElement::JsTemplateElement(elem) => {
                    let expr = elem.expression().ok()?;
                    let AnyJsExpression::AnyJsLiteralExpression(
                        AnyJsLiteralExpression::JsStringLiteralExpression(string_lit),
                    ) = &expr
                    else {
                        return None;
                    };
                    let value_token = string_lit.value_token().ok()?;
                    let inner = inner_string_text(&value_token);
                    content.push_str(inner.text());
                }
            }
        }

        let new_token = if state.has_single_quote {
            make::js_string_literal(&content)
        } else if state.has_double_quote {
            make::js_string_literal_single_quotes(&content)
        } else {
            // Prefer the configured quote style when either quote style works.
            if ctx.preferred_quote().is_double() {
                make::js_string_literal(&content)
            } else {
                make::js_string_literal_single_quotes(&content)
            }
        };

        let new_literal = make::js_string_literal_expression(new_token);
        let mut mutation = ctx.root().begin();

        mutation.replace_node(
            AnyJsExpression::JsTemplateExpression(template.clone()),
            AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsStringLiteralExpression(new_literal),
            ),
        );

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Replace the template expression with a string literal." }.to_owned(),
            mutation,
        ))
    }
}
