use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_js_factory::make::{self};
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, AnyJsName, AnyJsTemplateElement, JsCallExpression,
    JsComputedMemberExpression, JsLanguage, JsSyntaxKind, JsSyntaxToken, JsTemplateExpression,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, SyntaxToken, TextRange};

use crate::JsRuleAction;

declare_lint_rule! {
    /// Enforce the use of `String.trimStart()` and `String.trimEnd()` over `String.trimLeft()` and `String.trimRight()`.
    ///
    /// While `String.trimLeft()` and `String.trimRight()` are aliases for `String.trimStart()` and `String.trimEnd()`,
    /// only using the latter pair ensures consistency and is preferable for their direction-independent wording.
    ///
    /// Note that `String.trimStart()` and `String.trimEnd()` methods do not take any parameters. Any arguments passed to these methods will be ignored.
    /// See the MDN documentation for more details:
    /// - [String.prototype.trimStart()](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/trimStart)
    /// - [String.prototype.trimEnd()](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/trimEnd)
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const foo = bar.trimLeft();
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const foo = bar.trimRight();
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const foo = bar.trimStart();
    /// ```
    ///
    /// ```js
    /// const foo = bar.trimEnd();
    /// ```
    ///
    pub UseTrimStartEnd {
        version: "1.9.0",
        name: "useTrimStartEnd",
        language: "js",
        recommended: false,
        sources: &[RuleSource::EslintUnicorn("prefer-string-trim-start-end")],
        fix_kind: FixKind::Safe,
    }
}

#[derive(Debug, Clone)]
pub struct UseTrimStartEndState {
    member_name: String,
    span: TextRange,
}

impl Rule for UseTrimStartEnd {
    type Query = Ast<JsCallExpression>;
    type State = UseTrimStartEndState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let arguments = node.arguments().ok()?;
        let args = arguments.args();

        if !args.is_empty() {
            // If arguments are present, it suggests this function call may not be intended for `String.trimStart()` or `String.trimEnd()`,
            // as these methods do not accept parameters according to the specification:
            // - https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/trimStart#parameters
            // - https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/trimEnd#parameters
            return None;
        }

        let callee = node.callee().ok()?;
        let (member_name, span) = match callee {
            AnyJsExpression::JsComputedMemberExpression(callee) => {
                let member = callee.member().ok()?;
                let value = member.as_static_value()?;
                let span = value.range();
                let member_name = value.as_string_constant()?.to_string();
                (member_name, span)
            }
            AnyJsExpression::JsStaticMemberExpression(callee) => {
                let token = callee.member().ok()?.value_token().ok()?;
                let span = token.text_range();
                let member_name = token.text_trimmed().to_string();
                (member_name, span)
            }
            _ => return None,
        };

        Some(UseTrimStartEndState { member_name, span })
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let suggested_name = generate_suggested_name(&state.member_name)?;

        let diagnostic_message = markup! {
            "Use "{suggested_name}" instead of "{state.member_name}"."
        }
        .to_owned();
        let note_message = {
            markup! {
                ""{state.member_name}" is an alias for "{suggested_name}"."
            }
            .to_owned()
        };

        Some(
            RuleDiagnostic::new(rule_category!(), state.span, diagnostic_message)
                .note(note_message),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let callee = node.callee().ok()?;

        let is_computed_member = JsComputedMemberExpression::can_cast(callee.syntax().kind());
        let computed_member_expression_opt = if is_computed_member {
            callee.as_js_computed_member_expression()
        } else {
            None
        };
        let is_template = if is_computed_member {
            if let Ok(computed_member) = computed_member_expression_opt?.member() {
                JsTemplateExpression::can_cast(computed_member.syntax().kind())
            } else {
                false
            }
        } else {
            false
        };
        // Need to keep the original token to replace it with the new token.
        // `.as_static_value()` strips the information of tick tokens.
        let token = extract_token_from_expression(callee.clone())?;
        let replaced_member_name = suggested_name(&token)?;

        let mut elements = vec![];
        let template_elements = AnyJsTemplateElement::from(make::js_template_chunk_element(
            make::js_template_chunk(replaced_member_name.as_str()),
        ));
        elements.push(template_elements);

        let callee_object = match callee {
            AnyJsExpression::JsStaticMemberExpression(ref expression) => expression.object().ok(),
            AnyJsExpression::JsComputedMemberExpression(ref expression) => expression.object().ok(),
            _ => None,
        };

        let transformed_expression = if is_template {
            AnyJsExpression::JsTemplateExpression(
                make::js_template_expression(
                    computed_member_expression_opt?
                        .member()
                        .ok()?
                        .as_js_template_expression()?
                        .l_tick_token()
                        .ok()?,
                    make::js_template_element_list(elements),
                    computed_member_expression_opt?
                        .member()
                        .ok()?
                        .as_js_template_expression()?
                        .r_tick_token()
                        .ok()?,
                )
                .build(),
            )
        } else {
            AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsStringLiteralExpression(
                    make::js_string_literal_expression(JsSyntaxToken::new_detached(
                        // Need to handle "'text'" and "\"text\"".
                        // make::js_string_literal() call the function that format the text as below:
                        // > &format!("\"{text}\"")
                        JsSyntaxKind::JS_STRING_LITERAL,
                        &replaced_member_name,
                        [],
                        [],
                    )),
                ),
            )
        };

        let call_expression = if is_computed_member {
            AnyJsExpression::JsComputedMemberExpression(
                make::js_computed_member_expression(
                    callee_object?,
                    computed_member_expression_opt?.l_brack_token().ok()?,
                    transformed_expression,
                    computed_member_expression_opt?.r_brack_token().ok()?,
                )
                .build(),
            )
        } else {
            AnyJsExpression::JsStaticMemberExpression(make::js_static_member_expression(
                callee_object?,
                callee
                    .as_js_static_member_expression()?
                    .operator_token()
                    .ok()?,
                AnyJsName::JsName(make::js_name(make::ident(&replaced_member_name))),
            ))
        };

        let mut mutation = ctx.root().begin();
        mutation.replace_node(callee, call_expression);

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Replace "<Emphasis>{state.member_name}</Emphasis>" with "<Emphasis>{replaced_member_name}</Emphasis>"." }
                .to_owned(),
            mutation,
        ))
    }
}

fn extract_token_from_expression(callee: AnyJsExpression) -> Option<SyntaxToken<JsLanguage>> {
    let token = if let AnyJsExpression::JsComputedMemberExpression(expression) = callee {
        let member = expression.member().ok()?;
        match member {
            AnyJsExpression::AnyJsLiteralExpression(literal) => literal.value_token().ok(),
            AnyJsExpression::JsTemplateExpression(element) => {
                element.elements().into_iter().find_map(|x| {
                    x.as_js_template_chunk_element()
                        .and_then(|chunk| chunk.template_chunk_token().ok())
                })
            }
            _ => None,
        }
    } else if let AnyJsExpression::JsStaticMemberExpression(expression) = callee {
        expression.member().ok()?.value_token().ok()
    } else {
        None
    };
    token
}

// Handle "'text'" and "\"text\"" and "text" cases
fn suggested_name(text: &SyntaxToken<JsLanguage>) -> Option<String> {
    let trimmed = text.text_trimmed();

    let is_single_quoted = trimmed.starts_with('\'') && trimmed.ends_with('\'');
    let is_double_quoted = trimmed.starts_with('"') && trimmed.ends_with('"');

    let unquoted = if is_single_quoted {
        trimmed.trim_matches('\'')
    } else if is_double_quoted {
        trimmed.trim_matches('"')
    } else {
        trimmed
    };

    generate_suggested_name(unquoted).map(|suggested_name| {
        if is_single_quoted {
            format!("'{suggested_name}'")
        } else if is_double_quoted {
            format!("\"{suggested_name}\"")
        } else {
            suggested_name.to_string()
        }
    })
}

fn generate_suggested_name(member_name: &str) -> Option<&str> {
    match member_name {
        "trimLeft" => Some("trimStart"),
        "trimRight" => Some("trimEnd"),
        _ => None,
    }
}
