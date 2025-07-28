use crate::JsRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, AnyJsTemplateElement, JsTemplateExpression,
};
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt};

declare_lint_rule! {
    /// Disallow unnecessary template expressions.
    ///
    /// This rule reports template literals that contain only a single interpolation or string literal,
    /// and where the template literal would be equivalent to a simpler expression.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const ab = `${'a'}${'b'}`;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const wrappedText = `${text}`;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const stringWithNumber = `${'1 + 1 = '}${2}`;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const ab = 'ab';
    /// ```
    ///
    /// ```js
    /// const wrappedText = text;
    /// ```
    ///
    /// ```js
    /// const stringWithNumber = '1 + 1 = 2';
    /// ```
    ///
    /// ```js
    /// const text = `Hello, ${name}!`;
    /// ```
    ///
    /// ```js
    /// const multiline = `line 1
    /// line 2`;
    /// ```
    ///
    /// ```js
    /// const withQuotes = `It's "nice"`;
    /// ```
    ///
    pub NoUnnecessaryTemplateExpression {
        version: "2.1.0",
        name: "noUnnecessaryTemplateExpression",
        language: "ts",
        sources: &[RuleSource::EslintTypeScript("no-unnecessary-template-expression").same()],
        recommended: false,
        severity: Severity::Information,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoUnnecessaryTemplateExpression {
    type Query = Ast<JsTemplateExpression>;
    type State = TemplateSimplification;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let template = ctx.query();

        // Skip tagged template literals
        if template.tag().is_some() {
            return None;
        }

        analyze_template(template)
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let template = ctx.query();

        let message = match state {
            TemplateSimplification::SingleInterpolation => {
                markup! { "This template expression can be simplified to a regular expression." }
            }
            TemplateSimplification::ConcatenatedStrings => {
                markup! { "This template expression contains only string literals and can be simplified." }
            }
            TemplateSimplification::StringLiteral => {
                markup! { "This template expression can be replaced with a string literal." }
            }
        };

        Some(RuleDiagnostic::new(
            rule_category!(),
            template.range(),
            message,
        ))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let template = ctx.query();
        let mut mutation = ctx.root().begin();

        let replacement = match state {
            TemplateSimplification::SingleInterpolation => {
                // Extract the single interpolation expression
                if let Some(element) = template.elements().first() {
                    if let AnyJsTemplateElement::JsTemplateElement(template_element) = element {
                        template_element.expression().ok()
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            TemplateSimplification::ConcatenatedStrings | TemplateSimplification::StringLiteral => {
                // Combine all string parts
                let combined_string = combine_string_parts(template)?;

                let string_literal = if ctx.as_preferred_quote().is_double() {
                    make::js_string_literal(&combined_string)
                } else {
                    make::js_string_literal_single_quotes(&combined_string)
                };

                Some(AnyJsExpression::AnyJsLiteralExpression(
                    AnyJsLiteralExpression::JsStringLiteralExpression(
                        make::js_string_literal_expression(string_literal),
                    ),
                ))
            }
        }?;

        mutation.replace_node(
            AnyJsExpression::JsTemplateExpression(template.clone()),
            replacement,
        );

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Replace with a simpler expression." }.to_owned(),
            mutation,
        ))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TemplateSimplification {
    /// Template contains only a single interpolation: `${expr}`
    SingleInterpolation,
    /// Template contains only string interpolations that can be concatenated
    ConcatenatedStrings,
    /// Template is effectively a string literal
    StringLiteral,
}

fn analyze_template(template: &JsTemplateExpression) -> Option<TemplateSimplification> {
    let elements: Vec<_> = template.elements().iter().collect();

    // Check for single interpolation: `${expr}`
    if elements.len() == 1 {
        if let AnyJsTemplateElement::JsTemplateElement(_) = &elements[0] {
            return Some(TemplateSimplification::SingleInterpolation);
        }
    }

    let mut has_interpolation = false;
    let mut all_interpolations_are_strings = true;

    for (i, element) in elements.iter().enumerate() {
        match element {
            AnyJsTemplateElement::JsTemplateElement(template_element) => {
                has_interpolation = true;

                // Check if the interpolation is a string literal
                if let Ok(expr) = template_element.expression() {
                    if !is_string_literal(&expr) {
                        all_interpolations_are_strings = false;
                    }

                    // Check for trailing whitespace preservation pattern
                    // If this is a whitespace string literal followed by a chunk that starts with newline
                    if let AnyJsExpression::AnyJsLiteralExpression(
                        AnyJsLiteralExpression::JsStringLiteralExpression(str_expr),
                    ) = &expr
                    {
                        if let Ok(str_token) = str_expr.value_token() {
                            let value = str_token.text_trimmed();
                            // Remove quotes and check if it's only whitespace
                            let unquoted = value
                                .trim_start_matches('"')
                                .trim_start_matches('\'')
                                .trim_end_matches('"')
                                .trim_end_matches('\'');
                            if is_whitespace(unquoted) {
                                // Check if next element starts with newline
                                if let Some(next) = elements.get(i + 1) {
                                    if let AnyJsTemplateElement::JsTemplateChunkElement(
                                        next_chunk,
                                    ) = next
                                    {
                                        if let Ok(token) = next_chunk.template_chunk_token() {
                                            if token.text_trimmed().starts_with('\n')
                                                || token.text_trimmed().starts_with("\r\n")
                                            {
                                                // Preserve template for trailing whitespace visibility
                                                return None;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    all_interpolations_are_strings = false;
                }
            }
            AnyJsTemplateElement::JsTemplateChunkElement(chunk) => {
                // Check if chunk contains special characters that require template literal
                if let Ok(token) = chunk.template_chunk_token() {
                    let text = token.text_trimmed();
                    // If it contains newlines or both quote types, we need a template literal
                    if text.contains('\n') || (text.contains('\'') && text.contains('"')) {
                        return None;
                    }
                }
            }
        }
    }

    if !has_interpolation {
        // Pure template literal without interpolations
        Some(TemplateSimplification::StringLiteral)
    } else if all_interpolations_are_strings {
        // All interpolations are string literals
        Some(TemplateSimplification::ConcatenatedStrings)
    } else {
        None
    }
}

fn is_string_literal(expr: &AnyJsExpression) -> bool {
    matches!(
        expr,
        AnyJsExpression::AnyJsLiteralExpression(AnyJsLiteralExpression::JsStringLiteralExpression(
            _
        ))
    )
}

fn is_whitespace(s: &str) -> bool {
    s.chars().all(|c| c.is_whitespace())
}

fn combine_string_parts(template: &JsTemplateExpression) -> Option<String> {
    let mut result = String::new();

    for element in template.elements() {
        match element {
            AnyJsTemplateElement::JsTemplateChunkElement(chunk) => {
                if let Ok(token) = chunk.template_chunk_token() {
                    result.push_str(token.text_trimmed());
                }
            }
            AnyJsTemplateElement::JsTemplateElement(template_element) => {
                if let Ok(expr) = template_element.expression() {
                    if let AnyJsExpression::AnyJsLiteralExpression(
                        AnyJsLiteralExpression::JsStringLiteralExpression(string_expr),
                    ) = expr
                    {
                        if let Ok(token) = string_expr.value_token() {
                            // Remove quotes from the string literal
                            let text = token.text_trimmed();
                            let unquoted = text
                                .trim_start_matches(|c| c == '\'' || c == '"')
                                .trim_end_matches(|c| c == '\'' || c == '"');
                            result.push_str(unquoted);
                        }
                    }
                }
            }
        }
    }

    Some(result)
}
