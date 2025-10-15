use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, JsFileSource, JsStringLiteralExpression, JsxAttribute,
    JsxExpressionAttributeValue, JsxString, JsxText, inner_string_text,
};
use biome_rowan::{AstNode, AstNodeList, TextRange, declare_node_union};
use biome_rule_options::no_jsx_literals::NoJsxLiteralsOptions;

declare_lint_rule! {
    /// Disallow string literals inside JSX elements.
    ///
    /// This rule discourages the use of
    /// string literals directly within JSX elements. String literals in JSX can make code harder
    /// to maintain, especially in applications that require internationalization or dynamic content.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <div>Hello World</div>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <>Welcome to our site</>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <span>
    ///   Please enter your name
    /// </span>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <div>{'Hello World'}</div>
    /// ```
    ///
    /// ```jsx
    /// <>{'Welcome to our site'}</>
    /// ```
    ///
    /// ```jsx
    /// <span>
    ///   {'Please enter your name'}
    /// </span>
    /// ```
    ///
    /// ```jsx
    /// <div>{`Hello ${name}`}</div>
    /// ```
    ///
    /// ## Options
    ///
    /// ### `noStrings`
    ///
    /// When enabled, the rule will also flag string literals inside JSX expressions and attributes.
    ///
    /// > **Default:** `false`
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "noStrings": true
    ///   }
    /// }
    /// ```
    ///
    /// ```jsx,expect_diagnostic,use_options
    /// <span>
    ///   {'Please enter your name'}
    /// </span>
    /// ```
    /// ```jsx,expect_diagnostic,use_options
    /// <Component title="Hello!" />
    /// ```
    ///
    ///
    ///
    /// ### `allowedStrings`
    ///
    /// An array of strings that are allowed as literals. This can be useful for common words
    /// or characters that don't need to be wrapped in expressions.
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "allowedStrings": ["Hello", "&nbsp;", "·"]
    ///   }
    /// }
    /// ```
    ///
    /// ```jsx,use_options
    /// <>
    ///   <div>Hello</div>
    ///   <div>&nbsp;</div>
    ///   <div>·</div>
    /// </>
    /// ```
    ///
    /// ### `ignoreProps`
    ///
    /// When enabled, the rule will ignore string literals used as prop values.
    ///
    /// > **Default:** `false`
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "ignoreProps": true
    ///   }
    /// }
    /// ```
    ///
    /// ```jsx,use_options
    /// <>
    ///   <Component title="Welcome" />
    ///   <input placeholder="Enter name" />
    /// </>
    /// ```
    ///
    pub NoJsxLiterals {
        version: "2.2.4",
        name: "noJsxLiterals",
        language: "jsx",
        recommended: false,
        sources: &[RuleSource::EslintReact("jsx-no-literals").same()],
    }
}

impl Rule for NoJsxLiterals {
    type Query = Ast<AnyJsxText>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = NoJsxLiteralsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let file_source = ctx.source_type::<JsFileSource>();
        if !file_source.is_jsx() {
            return None;
        }

        let node = ctx.query();
        let options = ctx.options();

        if options.ignore_props
            && node
                .syntax()
                .ancestors()
                .skip(1)
                .any(|n| JsxAttribute::can_cast(n.kind()))
        {
            return None;
        }

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
                match expression {
                    AnyJsExpression::AnyJsLiteralExpression(
                        AnyJsLiteralExpression::JsStringLiteralExpression(string_literal),
                    ) => string_literal.value_token().ok()?,
                    AnyJsExpression::JsTemplateExpression(expression) => {
                        return if expression.elements().len() <= 1 {
                            Some(expression.range())
                        } else {
                            None
                        };
                    }

                    _ => return None,
                }
            }
        };

        for allowed_string in &options.allowed_strings {
            if inner_string_text(&value_token) == allowed_string.as_ref() {
                return None;
            }
        }

        if inner_string_text(&value_token).trim().is_empty() {
            return None;
        }

        Some(value_token.text_trimmed_range())
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state,
                markup! {
                    "Incorrect use of string literal detected."
                },
            )
            .note(markup! {
                "String literals in JSX can make code harder to maintain and internationalize."
            })
            .note(markup! {
                "Consider avoiding hardcoded strings entirely."
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
