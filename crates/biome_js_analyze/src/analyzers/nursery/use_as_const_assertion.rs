use crate::JsRuleAction;
use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic,
};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsLiteralExpression, AnyTsName, AnyTsType, JsPropertyClassMember, JsVariableDeclarator,
    TsAsExpression, TsTypeAssertionExpression,
};
use biome_rowan::{declare_node_union, AstNode, BatchMutationExt, TextRange};

declare_rule! {
    /// Enforce the use of `as const` over literal type and type annotation.
    ///
    /// In TypeScript, there are three common ways to specify that a value is of a specific type (like '2') and not a general type (like 'number'):
    ///
    /// 1. `as const`: telling TypeScript to infer the literal type automatically
    /// 2. `as` with a literal type: explicitly telling the literal type to TypeScript
    /// 3. angle bracket assertion: explicitly telling the literal type to TypeScript
    /// 4. type annotation: explicitly telling the literal type to TypeScript when declare variables
    ///
    /// The rule suggests to use `as const` when you're using `as` with a literal type or type annotation, since `as const` is simpler and doesn't require retyping the value.
    ///
    /// Source: https://typescript-eslint.io/rules/prefer-as-const/
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// let bar: 2 = 2;
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// let foo = <'bar'>'bar';
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// let foo = { bar: 'baz' as 'baz' };
    /// ```
    ///
    /// ## Valid
    ///
    /// ```ts
    /// let foo = 'bar';
    /// let foo = 'bar' as const;
    /// let foo: 'bar' = 'bar' as const;
    /// let bar = 'bar' as string;
    /// let foo = <string>'bar';
    /// let foo = { bar: 'baz' };
    /// ```
    ///
    pub(crate) UseAsConstAssertion {
        version: "next",
        name: "useAsConstAssertion",
        recommended: false,
    }
}

declare_node_union! {
    pub(crate) Query = TsAsExpression | TsTypeAssertionExpression | JsVariableDeclarator | JsPropertyClassMember
}

pub(crate) enum RuleState {
    AsAssertion(TextRange),
    /// The angle bracket assertion is useful when the JSX option is None in tsconfig.json.
    AngleBracketAssertion(TextRange),
    TypeAnnotation(TextRange),
}

impl Rule for UseAsConstAssertion {
    type Query = Ast<Query>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let query = ctx.query();
        match query {
            Query::TsAsExpression(expr) => {
                let literal = expr.expression().ok()?;
                let literal = literal.as_any_js_literal_expression()?;
                let asserted_literal = expr.ty().ok()?;
                let range = check_literal_match(literal, &asserted_literal)?;
                Some(RuleState::AsAssertion(range))
            }
            Query::TsTypeAssertionExpression(expr) => {
                let literal = expr.expression().ok()?;
                let literal = literal.as_any_js_literal_expression()?;
                let asserted_literal = expr.ty().ok()?;
                let range = check_literal_match(literal, &asserted_literal)?;
                Some(RuleState::AngleBracketAssertion(range))
            }
            Query::JsVariableDeclarator(decl) => {
                let literal = decl.initializer()?.expression().ok()?;
                let literal = literal.as_any_js_literal_expression()?;
                let type_annotation = decl.variable_annotation()?;
                let type_annotation = type_annotation.as_ts_type_annotation()?;
                let type_annotation = type_annotation.ty().ok()?;
                let range = check_literal_match(literal, &type_annotation)?;
                Some(RuleState::TypeAnnotation(range))
            }
            Query::JsPropertyClassMember(member) => {
                let literal = member.value()?.expression().ok()?;
                let literal = literal.as_any_js_literal_expression()?;
                let property_annotation = member
                    .property_annotation()?
                    .as_ts_type_annotation()?
                    .ty()
                    .ok()?;
                let range = check_literal_match(literal, &property_annotation)?;
                Some(RuleState::TypeAnnotation(range))
            }
        }
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        match state {
            RuleState::AsAssertion(range) => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    range,
                    markup! {
                        "You should use "<Emphasis>"as const"</Emphasis>" instead of "<Emphasis>"as"</Emphasis>" with a literal type."
                    },
                ).note(markup! {""<Emphasis>"as const"</Emphasis>" is simpler and doesn't require retyping the value."})
            ),
            RuleState::AngleBracketAssertion(range) => {
                Some(RuleDiagnostic::new(
                    rule_category!(),
                    range,
                    markup! {
                        "You should use "<Emphasis>"as const"</Emphasis>" instead of angle bracket type assertion."
                    }
                ).note(markup! {""<Emphasis>"as const"</Emphasis>" is simpler and doesn't require retyping the value."}))
            },
            RuleState::TypeAnnotation(range) => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    range,
                    markup! {
                        "You should use "<Emphasis>"as const"</Emphasis>" instead of type annotation."
                    }
                ).note(markup! {""<Emphasis>"as const"</Emphasis>" is simpler and doesn't require retyping the value."})
            ),
        }
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let query = ctx.query();
        match query {
            Query::TsAsExpression(previous_as_expr) => {
                let mut mutation = ctx.root().begin();
                let new_as_expr = previous_as_expr.clone().with_ty(AnyTsType::from(
                    make::ts_reference_type(AnyTsName::JsReferenceIdentifier(
                        make::js_reference_identifier(make::ident("const")),
                    ))
                    .build(),
                ));
                mutation.replace_node(previous_as_expr.clone(), new_as_expr);
                Some(JsRuleAction {
                    category: ActionCategory::QuickFix,
                    applicability: Applicability::Always,
                    message: markup! { "Replace with "<Emphasis>"as const"</Emphasis>" ." }
                        .to_owned(),
                    mutation,
                })
            }
            Query::TsTypeAssertionExpression(previous_expr) => {
                let mut mutation = ctx.root().begin();
                let new_as_expr = previous_expr.clone().with_ty(AnyTsType::from(
                    make::ts_reference_type(AnyTsName::JsReferenceIdentifier(
                        make::js_reference_identifier(make::ident("const")),
                    ))
                    .build(),
                ));
                mutation.replace_node(previous_expr.clone(), new_as_expr);
                Some(JsRuleAction {
                    category: ActionCategory::QuickFix,
                    applicability: Applicability::Always,
                    message: markup! { "Replace with "<Emphasis>"as const"</Emphasis>" ." }
                        .to_owned(),
                    mutation,
                })
            }
            Query::JsVariableDeclarator(_) => None,
            Query::JsPropertyClassMember(_) => None,
        }
    }
}

/// Checks if the provided literal matches the specified literal type.
/// If a match is found, it returns the [TextRange] of the matched literal.
fn check_literal_match(
    literal: &AnyJsLiteralExpression,
    specified_literal: &AnyTsType,
) -> Option<TextRange> {
    match (literal, specified_literal) {
        (
            AnyJsLiteralExpression::JsStringLiteralExpression(literal),
            AnyTsType::TsStringLiteralType(specified_literal),
        ) => {
            if literal.inner_string_text().ok()? == specified_literal.inner_string_text().ok()? {
                return Some(specified_literal.range());
            }
        }
        (
            AnyJsLiteralExpression::JsNumberLiteralExpression(literal),
            AnyTsType::TsNumberLiteralType(specified_literal),
        ) => {
            if literal.inner_string_text().ok()? == specified_literal.inner_string_text().ok()? {
                return Some(specified_literal.range());
            }
        }
        _ => return None,
    }
    None
}
