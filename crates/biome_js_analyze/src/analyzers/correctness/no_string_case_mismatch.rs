use biome_analyze::context::RuleContext;
use biome_analyze::{declare_rule, ActionCategory, Ast, FixKind, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_factory::make;
use biome_js_syntax::*;
use biome_rowan::{declare_node_union, AstNode, AstSeparatedList, BatchMutationExt};

use crate::JsRuleAction;

declare_rule! {
    /// Disallow comparison of expressions modifying the string case with non-compliant value.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// if (s.toUpperCase() === "Abc") {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// while (s.toLowerCase() === "Abc") {}
    /// ```
    /// ### Valid
    ///
    /// ```js
    /// if (s.toUpperCase() === "ABC") {}
    /// while (s.toLowerCase() === "abc") {}
    /// for (;s.toLocaleLowerCase() === "ABC";) {}
    /// while (s.toLocaleUpperCase() === "abc") {}
    /// for (let s = "abc"; s === "abc"; s = s.toUpperCase()) {}
    /// ```
    pub(crate) NoStringCaseMismatch {
        version: "1.0.0",
        name: "noStringCaseMismatch",
        source: RuleSource::Clippy("match_str_case_mismatch"),
        recommended: true,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoStringCaseMismatch {
    type Query = Ast<QueryCandidate>;
    type State = CaseMismatchInfo;
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Vec<Self::State> {
        let query = ctx.query();
        match query {
            QueryCandidate::JsBinaryExpression(expr) => CaseMismatchInfo::from_binary_expr(expr)
                .into_iter()
                .collect(),
            QueryCandidate::JsSwitchStatement(stmt) => CaseMismatchInfo::from_switch_stmt(stmt),
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let query = ctx.query();
        let mut diagnostic = match query {
            QueryCandidate::JsBinaryExpression(expr) => RuleDiagnostic::new(
                rule_category!(),
                expr.range(),
                markup! { "This expression always returns false." },
            ),
            QueryCandidate::JsSwitchStatement(..) => RuleDiagnostic::new(
                rule_category!(),
                state.literal.range(),
                markup! { "This case will never match." },
            ),
        };
        diagnostic = diagnostic
            .description("This expression always returns false, because the string is converted and will never match")
            .detail(
                state.call.range(),
                markup! {
                    "This call convert the string to " { state.expected_case.description() }
                },
            )
            .detail(
                state.literal.range(),
                markup! {
                    "... but this value is not in " { state.expected_case.description() }
                },
            );
        Some(diagnostic)
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        mutation.replace_node(
            state.literal.clone(),
            AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsStringLiteralExpression(
                    make::js_string_literal_expression(make::js_string_literal(
                        &state.expected_value,
                    )),
                ),
            ),
        );
        Some(JsRuleAction {
            mutation,
            message: markup! {"Use "<Emphasis>{state.expected_case.description()}</Emphasis>" string value."}.to_owned(),
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
        })
    }
}

declare_node_union! {
    pub(crate) QueryCandidate = JsBinaryExpression | JsSwitchStatement
}

pub(crate) struct CaseMismatchInfo {
    expected_case: ExpectedStringCase,
    expected_value: String,
    call: JsCallExpression,
    literal: AnyJsExpression,
}

impl CaseMismatchInfo {
    fn from_binary_expr(expr: &JsBinaryExpression) -> Option<Self> {
        let (left, right) = match expr.as_fields() {
            JsBinaryExpressionFields {
                left: Ok(left),
                right: Ok(right),
                operator_token: Ok(op),
            } if matches!(op.kind(), JsSyntaxKind::EQ2 | JsSyntaxKind::EQ3) => (left, right),
            _ => return None,
        };
        let (call, literal) = match (left, right) {
            (AnyJsExpression::JsCallExpression(call), other)
            | (other, AnyJsExpression::JsCallExpression(call)) => (call, other),
            _ => return None,
        };
        Self::compare_call_with_literal(call, literal)
    }

    fn from_switch_stmt(stmt: &JsSwitchStatement) -> Vec<Self> {
        match stmt.as_fields() {
            JsSwitchStatementFields {
                discriminant: Ok(AnyJsExpression::JsCallExpression(call)),
                cases,
                ..
            } => cases
                .into_iter()
                .filter_map(|case| case.as_js_case_clause().and_then(|case| case.test().ok()))
                .filter_map(|test| Self::compare_call_with_literal(call.clone(), test))
                .collect(),
            _ => Vec::new(),
        }
    }

    fn compare_call_with_literal(call: JsCallExpression, literal: AnyJsExpression) -> Option<Self> {
        let expected_case = ExpectedStringCase::from_call(&call)?;
        let static_value = literal.as_static_value()?;
        let literal_value = static_value.text();
        let expected_value = expected_case.convert(literal_value);
        if literal_value != expected_value {
            Some(Self {
                expected_case,
                expected_value,
                call,
                literal,
            })
        } else {
            None
        }
    }
}

enum ExpectedStringCase {
    Upper,
    Lower,
}

impl ExpectedStringCase {
    fn from_call(call: &JsCallExpression) -> Option<Self> {
        if call.arguments().ok()?.args().len() != 0 {
            return None;
        }
        let callee = call.callee().ok()?;
        let member_expr = AnyJsMemberExpression::cast_ref(callee.syntax())?;
        let member_name = member_expr.member_name()?;
        let member_name = member_name.text();
        if member_name == "toLowerCase" {
            return Some(Self::Lower);
        }
        if member_name == "toUpperCase" {
            return Some(Self::Upper);
        }
        None
    }

    fn convert(&self, s: &str) -> String {
        match self {
            ExpectedStringCase::Upper => s.to_uppercase(),
            ExpectedStringCase::Lower => s.to_lowercase(),
        }
    }

    fn description(&self) -> &str {
        match self {
            ExpectedStringCase::Upper => "upper case",
            ExpectedStringCase::Lower => "lower case",
        }
    }
}
