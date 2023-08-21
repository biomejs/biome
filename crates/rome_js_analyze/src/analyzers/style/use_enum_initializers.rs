use crate::JsRuleAction;
use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{AnyJsExpression, AnyJsLiteralExpression, JsSyntaxKind, TsEnumDeclaration};
use rome_rowan::{AstNode, BatchMutationExt};

declare_rule! {
    /// Require that each enum member value be explicitly initialized.
    ///
    /// _TypeScript_ enums are a practical way to organize semantically related constant values.
    /// Members of enums that don't have explicit values are by default given sequentially increasing numbers.
    ///
    /// When the value of enum members are important,
    /// allowing implicit values for enum members can cause bugs if enum declarations are modified over time.
    ///
    /// Source: https://typescript-eslint.io/rules/prefer-enum-initializers
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// enum Version {
    ///     V1,
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// enum Status {
    ///     Open = 1,
    ///     Close,
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// enum Color {
    ///     Red = "Red",
    ///     Green = "Green",
    ///     Blue,
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// enum Status {
    ///     Open = 1,
    ///     Close = 2,
    /// }
    /// ```
    ///
    /// ```ts
    /// enum Color {
    ///     Red = "Red",
    ///     Green = "Green",
    ///     Blue = "Blue",
    /// }
    /// ```
    ///
    /// ```ts
    /// declare enum Weather {
    ///     Rainy,
    ///     Sunny,
    /// }
    /// ```
    pub(crate) UseEnumInitializers {
        version: "1.0.0",
        name: "useEnumInitializers",
        recommended: true,
    }
}

impl Rule for UseEnumInitializers {
    // We apply the rule on an entire enum declaration to avoid reporting
    // a diagnostic for every enum members without initializers.
    type Query = Ast<TsEnumDeclaration>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let enum_declaration = ctx.query();
        if enum_declaration.is_ambient() {
            // In ambient declarations, enum members without initializers are opaque types.
            // They generally represent an enum with complex initializers.
            return None;
        }
        for enum_member in enum_declaration.members() {
            let enum_member = enum_member.ok()?;
            if enum_member.initializer().is_none() {
                return Some(());
            }
        }
        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let enum_declaration = ctx.query();
        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            enum_declaration.id().ok()?.range(),
            markup! {
                "This "<Emphasis>"enum declaration"</Emphasis>" contains members that are implicitly initialized."
            },
        );
        for enum_member in enum_declaration.members() {
            let enum_member = enum_member.ok()?;
            if enum_member.initializer().is_none() {
                diagnostic = diagnostic.detail(enum_member.range(), markup! {
                    "This "<Emphasis>"enum member"</Emphasis>" should be explicitly initialized."
                });
            }
        }
        Some(diagnostic.note(
            "Allowing implicit initializations for enum members can cause bugs if enum declarations are modified over time."
        ))
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let enum_declaration = ctx.query();
        let mut mutation = ctx.root().begin();
        let mut has_mutations = false;
        let mut next_member_value = EnumInitializer::Integer(0);
        for enum_member in enum_declaration.members() {
            let enum_member = enum_member.ok()?;
            if let Some(initializer) = enum_member.initializer() {
                next_member_value = EnumInitializer::Other;
                let expr = initializer.expression().ok()?.omit_parentheses();
                if let Some(expr) = expr.as_any_js_literal_expression() {
                    match expr {
                        AnyJsLiteralExpression::JsNumberLiteralExpression(expr) => {
                            let n = expr.value_token().ok()?;
                            let n = n.text_trimmed();
                            if let Ok(n) = n.parse::<i64>() {
                                next_member_value = EnumInitializer::Integer(n + 1);
                            }
                        }
                        AnyJsLiteralExpression::JsStringLiteralExpression(expr) => {
                            if enum_member.name().ok()?.name() == expr.inner_string_text().ok() {
                                next_member_value = EnumInitializer::EnumName;
                            }
                        }
                        _ => {}
                    }
                }
            } else {
                let x = match next_member_value {
                    EnumInitializer::Integer(n) => {
                        next_member_value = EnumInitializer::Integer(n + 1);
                        Some(AnyJsLiteralExpression::JsNumberLiteralExpression(
                            make::js_number_literal_expression(make::js_number_literal(n)),
                        ))
                    }
                    EnumInitializer::EnumName => {
                        let enum_name = enum_member.name().ok()?.name()?;
                        let enum_name = enum_name.text();
                        Some(AnyJsLiteralExpression::JsStringLiteralExpression(
                            make::js_string_literal_expression(make::js_string_literal(enum_name)),
                        ))
                    }
                    EnumInitializer::Other => None,
                };
                if let Some(x) = x {
                    has_mutations = true;
                    let new_enum_member =
                        enum_member
                            .clone()
                            .with_initializer(Some(make::js_initializer_clause(
                                make::token_decorated_with_space(JsSyntaxKind::EQ),
                                AnyJsExpression::AnyJsLiteralExpression(x),
                            )));
                    mutation.replace_node_discard_trivia(enum_member, new_enum_member);
                }
            }
        }
        if has_mutations {
            return Some(JsRuleAction {
                category: ActionCategory::QuickFix,
                applicability: Applicability::MaybeIncorrect,
                message: markup! { "Initialize all enum members." }.to_owned(),
                mutation,
            });
        }
        None
    }
}

enum EnumInitializer {
    // The member is initialized with an integer
    Integer(i64),
    /// The member name is equal to the member value
    EnumName,
    Other,
}
