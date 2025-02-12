use std::borrow::Cow;

use biome_analyze::context::RuleContext;
use biome_analyze::{declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::*;
use biome_rowan::{declare_node_union, AstNode, AstSeparatedList, BatchMutationExt};
use biome_string_case::StrOnlyExtension;

use crate::JsRuleAction;

declare_lint_rule! {
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
    pub NoStringCaseMismatch {
        version: "1.0.0",
        name: "noStringCaseMismatch",
        language: "js",
        sources: &[RuleSource::Clippy("match_str_case_mismatch")],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoStringCaseMismatch {
    type Query = Ast<QueryCandidate>;
    type State = CaseMismatchInfo;
    type Signals = Box<[Self::State]>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let query = ctx.query();
        match query {
            QueryCandidate::JsBinaryExpression(expr) => CaseMismatchInfo::from_binary_expr(expr)
                .into_iter()
                .collect(),
            QueryCandidate::JsSwitchStatement(stmt) => CaseMismatchInfo::from_switch_stmt(stmt),
        }
        .into_boxed_slice()
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
        let static_value = state.literal.as_static_value()?;

        let expected_value = state.expected_case.convert(static_value.text());
        mutation.replace_node(
            state.literal.clone(),
            AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsStringLiteralExpression(
                    make::js_string_literal_expression(if ctx.as_preferred_quote().is_double() {
                        make::js_string_literal(&expected_value)
                    } else {
                        make::js_string_literal_single_quotes(&expected_value)
                    }),
                ),
            ),
        );
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! {"Use "<Emphasis>{state.expected_case.description()}</Emphasis>" string value."}.to_owned(),
            mutation,
        ))
    }
}

declare_node_union! {
    pub QueryCandidate = JsBinaryExpression | JsSwitchStatement
}

pub struct CaseMismatchInfo {
    expected_case: StringCase,
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
        let expected_case = StringCase::from_call(&call)?;
        let value = literal.as_static_value()?;
        let literal_value = value.text();
        let mut case_iter = CharCaseIterator::from(literal_value);
        let is_mismatch = case_iter.any(|case| case != expected_case);
        is_mismatch.then_some(Self {
            expected_case,
            call,
            literal,
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
enum StringCase {
    Upper,
    Lower,
}

impl StringCase {
    fn from_call(call: &JsCallExpression) -> Option<Self> {
        if call.arguments().ok()?.args().len() != 0 {
            return None;
        }
        let callee = call.callee().ok()?;
        let member_expr = AnyJsMemberExpression::cast(callee.into_syntax())?;
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

    fn convert<'a>(&self, s: &'a str) -> Cow<'a, str> {
        match self {
            StringCase::Upper => Cow::Owned(s.to_uppercase()),
            StringCase::Lower => s.to_lowercase_cow(),
        }
    }

    fn description(&self) -> &str {
        match self {
            StringCase::Upper => "upper case",
            StringCase::Lower => "lower case",
        }
    }
}

struct CharCaseIterator<'a> {
    iter: std::str::Chars<'a>,
}
impl<'a> CharCaseIterator<'a> {
    fn from(s: &'a str) -> Self {
        CharCaseIterator { iter: s.chars() }
    }
}
impl Iterator for CharCaseIterator<'_> {
    type Item = StringCase;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(c) = self.iter.next() {
            match c {
                '\\' => {
                    match self.iter.next()? {
                        'x' => {
                            // \xHH
                            self.iter.next();
                            self.iter.next();
                        }
                        'u' => {
                            if self.iter.next()? == '{' {
                                // \u{H}, \u{HH}, ..., \u{HHHHHH}
                                while self.iter.next()? != '}' {}
                            } else {
                                // \uHHHH
                                self.iter.next();
                                self.iter.next();
                                self.iter.next();
                                self.iter.next();
                            }
                        }
                        _ => {
                            // \n, ...
                            self.iter.next();
                        }
                    }
                }
                c => {
                    if c.is_uppercase() {
                        return Some(StringCase::Upper);
                    } else if c.is_lowercase() {
                        return Some(StringCase::Lower);
                    }
                }
            }
        }
        None
    }
}
