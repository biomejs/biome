use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make::{self};
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, AnyJsName, AnyJsTemplateElement, JsCallExpression,
    JsComputedMemberExpression, JsLanguage, JsTemplateExpression,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, SyntaxToken, TextRange};
use biome_rule_options::use_trim_start_end::UseTrimStartEndOptions;

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
        sources: &[RuleSource::EslintUnicorn("prefer-string-trim-start-end").same()],
        fix_kind: FixKind::Safe,
        severity: Severity::Information,
    }
}

#[derive(Debug, Clone)]
pub struct UseTrimStartEndState {
    member_name: TrimMethod,
    span: TextRange,
}

impl Rule for UseTrimStartEnd {
    type Query = Ast<JsCallExpression>;
    type State = UseTrimStartEndState;
    type Signals = Option<Self::State>;
    type Options = UseTrimStartEndOptions;

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
                let member_name = TrimMethod::from_str(value.as_string_constant()?)?;
                (member_name, span)
            }
            AnyJsExpression::JsStaticMemberExpression(callee) => {
                let token = callee.member().ok()?.value_token().ok()?;
                let span = token.text_range();
                let member_name = TrimMethod::from_str(token.text_trimmed())?;
                (member_name, span)
            }
            _ => return None,
        };

        Some(UseTrimStartEndState { member_name, span })
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let suggested_name = state.member_name.suggested_name();
        let member_name = state.member_name.current_name();

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.span,
                markup! {
                    "Use "{suggested_name}" instead of "{member_name}"."
                },
            )
            .note(markup! {
                {member_name}" is an alias for "{suggested_name}"."
            }),
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
        let replaced_member_name = state.member_name.suggested_name();
        let replaced_member_name_display = if is_template {
            replaced_member_name
        } else if token.text_trimmed().starts_with('\'') {
            match state.member_name {
                TrimMethod::TrimLeft => "'trimStart'",
                TrimMethod::TrimRight => "'trimEnd'",
            }
        } else if token.text_trimmed().starts_with('"') {
            match state.member_name {
                TrimMethod::TrimLeft => "\"trimStart\"",
                TrimMethod::TrimRight => "\"trimEnd\"",
            }
        } else {
            replaced_member_name
        };

        let mut elements = vec![];
        let template_elements = AnyJsTemplateElement::from(make::js_template_chunk_element(
            make::js_template_chunk(replaced_member_name),
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
        } else if token.text_trimmed().starts_with('\'') {
            AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsStringLiteralExpression(
                    make::js_string_literal_expression(make::js_string_literal_single_quotes(
                        replaced_member_name,
                    )),
                ),
            )
        } else {
            AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsStringLiteralExpression(
                    make::js_string_literal_expression(make::js_string_literal(
                        replaced_member_name,
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
                AnyJsName::JsName(make::js_name(make::ident(replaced_member_name))),
            ))
        };

        let mut mutation = ctx.root().begin();
        mutation.replace_node(callee, call_expression);

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Replace "<Emphasis>{state.member_name.current_name()}</Emphasis>" with "<Emphasis>{replaced_member_name_display}</Emphasis>"." },
            mutation,
        ))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TrimMethod {
    TrimLeft,
    TrimRight,
}

impl TrimMethod {
    const fn current_name(self) -> &'static str {
        match self {
            Self::TrimLeft => "trimLeft",
            Self::TrimRight => "trimRight",
        }
    }

    const fn suggested_name(self) -> &'static str {
        match self {
            Self::TrimLeft => "trimStart",
            Self::TrimRight => "trimEnd",
        }
    }

    fn from_str(text: &str) -> Option<Self> {
        match text {
            "trimLeft" => Some(Self::TrimLeft),
            "trimRight" => Some(Self::TrimRight),
            _ => None,
        }
    }
}

fn extract_token_from_expression(callee: AnyJsExpression) -> Option<SyntaxToken<JsLanguage>> {
    if let AnyJsExpression::JsComputedMemberExpression(expression) = callee {
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
    }
}
