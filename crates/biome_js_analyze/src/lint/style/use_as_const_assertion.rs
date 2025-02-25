use crate::JsRuleAction;
use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, AnyTsName, AnyTsType, JsInitializerClause,
    JsPropertyClassMember, JsSyntaxKind, JsVariableDeclarator, TsAsExpression,
    TsTypeAssertionExpression,
};
use biome_rowan::{declare_node_union, AstNode, BatchMutationExt, TextRange};

declare_lint_rule! {
    /// Enforce the use of `as const` over literal type and type annotation.
    ///
    /// In TypeScript, there are three common ways to specify that a value is of a specific type such as `2` and not a general type such as `number`:
    ///
    /// 1. `as const`: telling TypeScript to infer the literal type automatically
    /// 2. `as <literal>`: explicitly telling the literal type to TypeScript
    /// 3. type annotation: explicitly telling the literal type to TypeScript when declare variables
    ///
    /// The rule suggests to use `as const` when you're using `as` with a literal type or type annotation, since `as const` is simpler and doesn't require retyping the value.
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
    /// let foo = { bar: 'baz' as 'baz' };
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// let foo = 'bar';
    /// let foo = 'bar' as const;
    /// let foo: 'bar' = 'bar' as const;
    /// let bar = 'bar' as string;
    /// let foo = { bar: 'baz' };
    /// ```
    pub UseAsConstAssertion {
        version: "1.3.0",
        name: "useAsConstAssertion",
        language: "ts",
        sources: &[RuleSource::EslintTypeScript("prefer-as-const")],
        recommended: false,
        severity: Severity::Warning,
        fix_kind: FixKind::Safe,
    }
}

declare_node_union! {
    pub Query = JsVariableDeclarator | JsPropertyClassMember | TsAsExpression | TsTypeAssertionExpression
}

pub enum RuleState {
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
        }
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let (range, message, note) = match state {
            RuleState::AsAssertion(range) => (
                range,
                markup! {
                    "Use "<Emphasis>"as const"</Emphasis>" instead of "<Emphasis>"as"</Emphasis>" with a literal type."
                },
                markup! {""<Emphasis>"as const"</Emphasis>" doesn't require any update when the asserted value is changed."},
            ),
            RuleState::AngleBracketAssertion(range) => (
                range,
                markup! {
                    "Use "<Emphasis>"as const"</Emphasis>" instead of angle bracket type assertion."
                },
                markup! {"The angle bracket assertion can occasionally be confused with JSX syntax, so using the "<Emphasis>"as const"</Emphasis>" is a preferable alternative."},
            ),
            RuleState::TypeAnnotation(range) => (
                range,
                markup! {
                    "Use "<Emphasis>"as const"</Emphasis>" instead of type annotation."
                },
                markup! {""<Emphasis>"as const"</Emphasis>" doesn't require any update when the value is changed."},
            ),
        };
        Some(RuleDiagnostic::new(rule_category!(), range, message).note(note))
    }

    /// Replace type assertion or annotation with const assertion (`as const`).
    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let query = ctx.query();
        let mut mutation = ctx.root().begin();

        let as_token = make::token_decorated_with_space(JsSyntaxKind::AS_KW);
        let const_reference_type = AnyTsType::from(
            make::ts_reference_type(AnyTsName::JsReferenceIdentifier(
                make::js_reference_identifier(make::ident("const")),
            ))
            .build(),
        );

        match query {
            Query::JsVariableDeclarator(previous_decl) => {
                mutation.remove_node(previous_decl.variable_annotation()?);
                let new_initializer = make::js_initializer_clause(
                    make::token_decorated_with_space(JsSyntaxKind::EQ),
                    AnyJsExpression::TsAsExpression(make::ts_as_expression(
                        previous_decl.initializer()?.expression().ok()?,
                        as_token,
                        const_reference_type,
                    )),
                );
                mutation.replace_node_discard_trivia(previous_decl.initializer()?, new_initializer);
            }
            Query::JsPropertyClassMember(previous_member) => {
                mutation.remove_node(previous_member.property_annotation()?);
                let new_initializer = make::js_initializer_clause(
                    make::token_decorated_with_space(JsSyntaxKind::EQ),
                    AnyJsExpression::TsAsExpression(make::ts_as_expression(
                        previous_member.value()?.expression().ok()?,
                        as_token,
                        const_reference_type,
                    )),
                );
                mutation.replace_node_discard_trivia(previous_member.value()?, new_initializer);
            }
            Query::TsAsExpression(previous_expr) => {
                mutation.replace_node(previous_expr.ty().ok()?, const_reference_type);
            }
            Query::TsTypeAssertionExpression(previous_expr) => {
                let previous_initializer = previous_expr.parent::<JsInitializerClause>()?;
                let new_expr = AnyJsExpression::TsAsExpression(make::ts_as_expression(
                    previous_expr.expression().ok()?,
                    as_token,
                    const_reference_type,
                ));
                mutation.replace_node(previous_initializer.expression().ok()?, new_expr);
            }
        };
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Replace with "<Emphasis>"as const"</Emphasis>"." }.to_owned(),
            mutation,
        ))
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
            if literal.inner_string_text().ok() == specified_literal.inner_string_text().ok() {
                return Some(specified_literal.range());
            }
        }
        (
            AnyJsLiteralExpression::JsNumberLiteralExpression(literal),
            AnyTsType::TsNumberLiteralType(specified_literal),
        ) => {
            if literal.value_token().ok()?.text_trimmed()
                == specified_literal.literal_token().ok()?.text_trimmed()
            {
                return Some(specified_literal.range());
            }
        }
        _ => return None,
    }
    None
}
