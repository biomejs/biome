use biome_analyze::{Ast, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, JsFileSource, JsIdentifierBinding,
    JsStringLiteralExpression, JsxExpressionAttributeValue, JsxString, JsxText,
};
use biome_rowan::{AstNode, AstNodeList, declare_node_union};
use biome_rule_options::no_jsx_literals::NoJsxLiteralsOptions;

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
    pub NoJsxLiterals {
        version: "next",
        name: "noJsxLiterals",
        language: "jsx",
        recommended: false,
    }
}

impl Rule for NoJsxLiterals {
    type Query = Ast<AnyJsxText>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoJsxLiteralsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let file_source = ctx.source_type::<JsFileSource>();
        if !file_source.is_jsx() {
            return None;
        }

        let node = ctx.query();
        let options = ctx.options();
        let value_token = match node {
            AnyJsxText::JsxText(text) => text.value_token().ok()?,
            AnyJsxText::JsStringLiteralExpression(expression) => {
                if !options.no_strings {
                    return None;
                }
                expression.value_token().ok()?
            }
            AnyJsxText::JsxString(string) => {
                if !options.no_strings {
                    return None;
                }
                string.value_token().ok()?
            }
            AnyJsxText::JsxExpressionAttributeValue(expression) => {
                if !options.no_strings {
                    return None;
                }
                let expression = expression.expression().ok()?;
                return match expression {
                    AnyJsExpression::JsTemplateExpression(expression) => {
                        if expression.elements().len() == 0 {
                            Some(())
                        } else if expression.elements().len() == 1 {
                            Some(())
                        } else {
                            None
                        }
                    }

                    _ => None,
                };
            }
        };

        for allowed_string in &options.allowed_strings {
            if value_token.text() == allowed_string.as_ref() {
                return None;
            }
        }

        if value_token.text().trim().is_empty() {
            return None;
        }

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "The use of JSX literals is not allowed."
                },
            )
            .note(markup! {
                "This note will give you more information."
            }),
        )
    }
}

declare_node_union! {
    pub AnyJsxText = JsxText
        | JsStringLiteralExpression
        | JsxString
        | JsxExpressionAttributeValue
}
