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
    /// This rule is inspired by the
    /// [`@typescript-eslint/no-unnecessary-template-expression`](https://typescript-eslint.io/rules/no-unnecessary-template-expression/)
    /// rule. Unlike the TypeScript ESLint version, this rule only applies to cases that can
    /// be verified syntactically, without type information.
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
        version: "next",
        name: "noUnnecessaryTemplateExpression",
        language: "js",
        recommended: false,
        severity: Severity::Information,
        sources: &[RuleSource::EslintTypeScript("no-unnecessary-template-expression").inspired()],
        fix_kind: FixKind::Safe,
    }
}

/// Represents a part of the combined string that will replace the template.
#[derive(Debug)]
pub struct TemplateState {
    /// Combined string content for the new string literal.
    content: String,
    /// Whether to use single quotes for the new literal. `false` means double quotes.
    use_single_quotes: bool,
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
        let mut combined = String::new();
        let mut has_any_interpolation = false;

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

                    combined.push_str(text);
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
                            combined.push_str(inner.text());
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

        // Choose the quote style for the replacement string literal.
        // Prefer the user's configured quote style; fall back to whichever avoids escaping.
        let has_single_quote = combined.contains('\'');
        let has_double_quote = combined.contains('"');

        let use_single_quotes = if has_single_quote && !has_double_quote {
            // Must use double quotes to avoid escaping.
            false
        } else if has_double_quote && !has_single_quote {
            // Must use single quotes to avoid escaping.
            true
        } else {
            // Either: no quotes in content or both kinds present.
            // Use the configured preferred quote style (falling back to double).
            !ctx.preferred_quote().is_double()
        };

        Some(TemplateState {
            content: combined,
            use_single_quotes,
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
        let template = ctx.query();
        let mut mutation = ctx.root().begin();

        // Safety: when the combined content contains both `'` and `"` characters,
        // we cannot produce a valid string literal without escaping one of them.
        // The `make::js_string_literal*` functions do not escape content, so we skip
        // the automatic fix in this case and let the user resolve it manually.
        if state.content.contains('\'') && state.content.contains('"') {
            return None;
        }

        let new_token = if state.use_single_quotes {
            make::js_string_literal_single_quotes(&state.content)
        } else {
            make::js_string_literal(&state.content)
        };

        let new_literal = make::js_string_literal_expression(new_token);

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
