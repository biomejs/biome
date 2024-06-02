use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, FixKind, Rule, RuleDiagnostic,
    RuleSource, RuleSourceKind,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, AnyJsxAttributeValue, AnyJsxChild, JsSyntaxKind,
    JsSyntaxToken, JsxAttributeInitializerClause, JsxExpressionAttributeValue, T,
};
use biome_rowan::{declare_node_union, AstNode, BatchMutationExt, TextRange};

use crate::JsRuleAction;

declare_rule! {
    /// This rule allows you to enforce curly braces or disallow unnecessary curly braces in JSX props and/or children.
    ///
    /// For situations where JSX expressions are unnecessary, please refer to [the React doc](https://facebook.github.io/react/docs/jsx-in-depth.html) and [this page about JSX gotchas](https://github.com/facebook/react/blob/v15.4.0-rc.3/docs/docs/02.3-jsx-gotchas.md#html-entities).
    ///
    /// By default, this rule will check for and warn about unnecessary curly braces in both JSX props and children.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <Foo>{'Hello world'}</Foo>;
    ///
    /// <Foo foo={'bar'} />;
    ///
    /// <Foo foo=<Bar /> />;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// <Foo>Hello world</Foo>;
    /// <Foo foo="bar" />;
    /// <Foo foo={5} />;
    /// <Foo foo={<Bar />} />;
    /// ```
    ///
    pub UseJsxCurlyBraceConvention {
        version: "next",
        name: "useJsxCurlyBraceConvention",
        language: "js",
        recommended: false,
        sources: &[RuleSource::EslintReact("jsx-curly-brace-presence")],
        source_kind: RuleSourceKind::Inspired,
        fix_kind: FixKind::Safe,
    }
}

declare_node_union! {
    pub UseJsxCurlyBraceConventionQuery = JsxAttributeInitializerClause | AnyJsxChild
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurlyBraceResolution {
    /// The user should add curly braces around the expression.
    AddBraces,
    /// The user should remove the curly braces around the expression.
    RemoveBraces,
}

impl Rule for UseJsxCurlyBraceConvention {
    type Query = Ast<UseJsxCurlyBraceConventionQuery>;
    type State = CurlyBraceResolution;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let query = ctx.query();
        let has_curly_braces = has_curly_braces(query);
        match query {
            UseJsxCurlyBraceConventionQuery::JsxAttributeInitializerClause(attr) => {
                handle_attr_init_clause(attr, has_curly_braces)
            }
            UseJsxCurlyBraceConventionQuery::AnyJsxChild(child) => {
                handle_jsx_child(child, has_curly_braces)
            }
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let source_range = match &node {
            UseJsxCurlyBraceConventionQuery::JsxAttributeInitializerClause(node) => {
                node.value().map(|value| value.range())
            }
            _ => Ok(node.range()),
        }
        .unwrap_or(node.range());

        let diag = match (state, node) {
            (CurlyBraceResolution::AddBraces, UseJsxCurlyBraceConventionQuery::JsxAttributeInitializerClause(_)) => RuleDiagnostic::new(
                rule_category!(),
                source_range,
                markup! {
                    "Should have curly braces around expression."
                },
            )
            .note(markup! {
                "JSX attribute value should be wrapped in curly braces. This will make the JSX attribute value more readable."
            }),
            (CurlyBraceResolution::AddBraces, UseJsxCurlyBraceConventionQuery::AnyJsxChild(_)) => RuleDiagnostic::new(
                rule_category!(),
                source_range,
                markup! {
                    "Should have curly braces around expression."
                },
            )
            .note(markup! {
                "JSX child should be wrapped in curly braces."
            }),
            (CurlyBraceResolution::RemoveBraces, UseJsxCurlyBraceConventionQuery::JsxAttributeInitializerClause(_)) => RuleDiagnostic::new(
                rule_category!(),
                source_range,
                markup! {
                    "Should not have curly braces around expression."
                },
            )
            .note(markup! {
                "JSX attribute value does not need to be wrapped in curly braces."
            }),
            (CurlyBraceResolution::RemoveBraces, UseJsxCurlyBraceConventionQuery::AnyJsxChild(_)) => RuleDiagnostic::new(
                rule_category!(),
                source_range,
                markup! {
                    "Should not have curly braces around expression."
                },
            )
            .note(markup! {
                "JSX child does not need to be wrapped in curly braces."
            })
        };

        Some(diag)
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();
        match (state, node) {
            (
                CurlyBraceResolution::AddBraces,
                UseJsxCurlyBraceConventionQuery::JsxAttributeInitializerClause(node),
            ) => {
                let value = node
                    .value()
                    .and_then(|value| match value {
                        AnyJsxAttributeValue::AnyJsxTag(node) => {
                            let expr = make::jsx_tag_expression(node);
                            let value = make::jsx_expression_attribute_value(
                                make::token(T!['{']),
                                AnyJsExpression::JsxTagExpression(expr),
                                make::token(T!['}']),
                            );

                            Ok(AnyJsxAttributeValue::JsxExpressionAttributeValue(value))
                        }
                        AnyJsxAttributeValue::JsxExpressionAttributeValue(node) => {
                            Ok(AnyJsxAttributeValue::JsxExpressionAttributeValue(node))
                        }
                        AnyJsxAttributeValue::JsxString(node) => {
                            let value = make::jsx_expression_attribute_value(
                                make::token(T!['{']),
                                AnyJsExpression::AnyJsLiteralExpression(
                                    AnyJsLiteralExpression::JsStringLiteralExpression(
                                        make::js_string_literal_expression(node.value_token()?),
                                    ),
                                ),
                                make::token(T!['}']),
                            );
                            Ok(AnyJsxAttributeValue::JsxExpressionAttributeValue(value))
                        }
                    })
                    .ok()?;
                mutation.replace_node(
                    node.clone(),
                    make::jsx_attribute_initializer_clause(make::token(T![=]), value),
                );
            }
            (CurlyBraceResolution::AddBraces, UseJsxCurlyBraceConventionQuery::AnyJsxChild(_)) => {
                // this should never get hit
                return None;
            }
            (
                CurlyBraceResolution::RemoveBraces,
                UseJsxCurlyBraceConventionQuery::JsxAttributeInitializerClause(node),
            ) => {
                let str_literal = node.value().ok().and_then(|value| {
                    if let AnyJsxAttributeValue::JsxExpressionAttributeValue(node) = value {
                        node.expression().ok().and_then(|expr| {
                            if let AnyJsExpression::AnyJsLiteralExpression(
                                AnyJsLiteralExpression::JsStringLiteralExpression(node),
                            ) = expr
                            {
                                Some(node)
                            } else {
                                None
                            }
                        })
                    } else {
                        None
                    }
                })?;
                let jsx_string = make::jsx_string(str_literal.value_token().ok()?);
                let value = AnyJsxAttributeValue::JsxString(jsx_string);
                mutation.replace_node(
                    node.clone(),
                    make::jsx_attribute_initializer_clause(make::token(T![=]), value),
                );
            }
            (
                CurlyBraceResolution::RemoveBraces,
                UseJsxCurlyBraceConventionQuery::AnyJsxChild(node),
            ) => {
                if let AnyJsxChild::JsxExpressionChild(expr) = node {
                    let str_literal = expr.expression().and_then(|expr| {
                        if let AnyJsExpression::AnyJsLiteralExpression(
                            AnyJsLiteralExpression::JsStringLiteralExpression(node),
                        ) = expr
                        {
                            Some(node)
                        } else {
                            None
                        }
                    })?;
                    let text = &str_literal.value_token().ok()?.token_text();
                    // trim the quotes off of the string literal
                    let text_trimmed = text.clone().slice(TextRange::new(
                        1.into(),
                        text.len().checked_sub(1.into()).unwrap_or(text.len()),
                    ));
                    let jsx_text = biome_js_syntax::AnyJsxChild::JsxText(make::jsx_text(
                        JsSyntaxToken::new_detached(
                            JsSyntaxKind::JS_STRING_LITERAL,
                            &format!("{text_trimmed}"),
                            [],
                            [],
                        ),
                    ))
                    .into_syntax()
                    .into();
                    mutation.replace_element(node.clone().into_syntax().into(), jsx_text);
                }
            }
        }

        let msg = match state {
            CurlyBraceResolution::AddBraces => "Add curly braces around the expression.",
            CurlyBraceResolution::RemoveBraces => "Remove curly braces around the expression.",
        };

        Some(JsRuleAction::new(
            ActionCategory::QuickFix,
            ctx.metadata().applicability(),
            markup! { {msg} }.to_owned(),
            mutation,
        ))
    }
}

fn handle_attr_init_clause(
    attr: &JsxAttributeInitializerClause,
    has_curly_braces: bool,
) -> Option<CurlyBraceResolution> {
    let Ok(node) = attr.value() else {
        return None;
    };

    match node {
        AnyJsxAttributeValue::AnyJsxTag(_) => Some(CurlyBraceResolution::AddBraces),
        AnyJsxAttributeValue::JsxExpressionAttributeValue(node) => {
            if has_curly_braces && contains_string_literal(&node) {
                Some(CurlyBraceResolution::RemoveBraces)
            } else if !has_curly_braces && contains_jsx_tag(&node) {
                Some(CurlyBraceResolution::AddBraces)
            } else {
                None
            }
        }
        AnyJsxAttributeValue::JsxString(_) => None,
    }
}

fn handle_jsx_child(child: &AnyJsxChild, has_curly_braces: bool) -> Option<CurlyBraceResolution> {
    match child {
        AnyJsxChild::JsxExpressionChild(child) => child
            .expression()
            .as_ref()
            .and_then(|node| node.as_any_js_literal_expression())
            .and_then(|node| node.as_js_string_literal_expression())
            .and({
                if has_curly_braces {
                    Some(CurlyBraceResolution::RemoveBraces)
                } else {
                    None
                }
            }),
        AnyJsxChild::JsxText(_) => None,
        _ => None,
    }
}

fn has_curly_braces(node: &UseJsxCurlyBraceConventionQuery) -> bool {
    match node {
        UseJsxCurlyBraceConventionQuery::JsxAttributeInitializerClause(node) => {
            node.value()
                .map(|node| matches!(node, AnyJsxAttributeValue::JsxExpressionAttributeValue(attr) if attr.l_curly_token().is_ok() || attr.r_curly_token().is_ok()))
                .unwrap_or(false)
        }
        UseJsxCurlyBraceConventionQuery::AnyJsxChild(node) => match node {
            AnyJsxChild::JsxExpressionChild(node) => node.l_curly_token().is_ok() || node.r_curly_token().is_ok(),
            AnyJsxChild::JsxSpreadChild(_) => true,
            _ => false,
        },
    }
}

fn contains_string_literal(node: &JsxExpressionAttributeValue) -> bool {
    node.expression()
        .map(|expr| {
            matches!(
                expr,
                AnyJsExpression::AnyJsLiteralExpression(
                    AnyJsLiteralExpression::JsStringLiteralExpression(_)
                )
            )
        })
        .unwrap_or_default()
}

fn contains_jsx_tag(node: &JsxExpressionAttributeValue) -> bool {
    node.expression()
        .map(|expr| matches!(expr, AnyJsExpression::JsxTagExpression(_)))
        .unwrap_or_default()
}
