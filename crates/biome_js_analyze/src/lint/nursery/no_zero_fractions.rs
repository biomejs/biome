use crate::JsRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, AnyJsMemberExpression, AnyJsModuleItem,
    AnyJsStatement, JsExpressionStatement, JsModuleItemList, JsNumberLiteralExpression,
    JsStatementList, T,
};
use biome_rowan::{AstNode, AstNodeList, AstNodeListExt, BatchMutationExt, SyntaxTriviaPiece};
use biome_rule_options::no_zero_fractions::NoZeroFractionsOptions;

declare_lint_rule! {
    /// Disallow number literals with zero fractions or dangling dots.
    ///
    /// There is no difference in JavaScript between, for example, `1`, `1.0`, and `1.`.
    /// This rule suggests using the shorter form for consistency and brevity.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const foo = 1.0;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const foo = 1.;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const foo = 123.00e20;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const foo = 1;
    /// const bar = -1.1;
    /// const baz = 123.456;
    /// const qux = 1e3;
    /// ```
    ///
    pub NoZeroFractions {
        version: "next",
        name: "noZeroFractions",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("no-zero-fractions").same()],
        recommended: false,
        fix_kind: FixKind::Safe,
        issue_number: Some("9829"),
    }
}

impl Rule for NoZeroFractions {
    type Query = Ast<JsNumberLiteralExpression>;
    type State = State;
    type Signals = Option<Self::State>;
    type Options = NoZeroFractionsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let token = ctx.query().value_token().ok()?;
        diagnostic_kind(token.text_trimmed())
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        let message = match state.kind {
            DiagnosticKind::DanglingDot => markup! {
                "This number literal has a dangling dot."
            },
            DiagnosticKind::ZeroFraction => markup! {
                "This number literal has a zero fraction."
            },
        };

        Some(RuleDiagnostic::new(rule_category!(), node.range(), message).note(markup! {
            "The fractional part is redundant. It has the same runtime value and makes the literal longer than it needs to be."
        }))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let token = node.value_token().ok()?;
        let formatted = state.parts.format(token.text_trimmed())?;
        let replacement = make_replacement_expression(&formatted);

        let mut mutation = ctx.root().begin();
        if needs_parentheses(node, &formatted) {
            let replacement = AnyJsExpression::from(make::js_parenthesized_expression(
                make::token(T!['(']),
                replacement,
                make::token(T![')']),
            ))
            .append_trivia_pieces(node.syntax().last_trailing_trivia()?.pieces())?;

            if let Some(statement) = node.parent::<JsExpressionStatement>() {
                replace_expression_statement(
                    &mut mutation,
                    &statement,
                    replacement,
                    node.syntax().first_leading_trivia()?.pieces().collect(),
                )?;
            } else {
                let replacement = replacement
                    .prepend_trivia_pieces(node.syntax().first_leading_trivia()?.pieces())?;
                mutation.replace_node_discard_trivia(old_expression(node), replacement);
            }
        } else {
            let replacement = replacement
                .as_any_js_literal_expression()?
                .as_js_number_literal_expression()?
                .value_token()
                .ok()?;
            mutation.replace_token_transfer_trivia(token, replacement);
        }

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            match state.kind {
                DiagnosticKind::DanglingDot => markup! { "Remove the dangling dot." }.to_owned(),
                DiagnosticKind::ZeroFraction => {
                    markup! { "Remove the redundant zero fraction." }.to_owned()
                }
            },
            mutation,
        ))
    }
}

#[derive(Clone, Copy)]
pub enum DiagnosticKind {
    DanglingDot,
    ZeroFraction,
}

#[derive(Clone, Copy)]
pub struct State {
    kind: DiagnosticKind,
    parts: NumberLiteralParts,
}

fn diagnostic_kind(raw: &str) -> Option<State> {
    let parts = split_number_literal(raw)?;
    let kind = if parts.fraction(raw).is_empty() {
        DiagnosticKind::DanglingDot
    } else if parts.trimmed_fraction(raw) != parts.fraction(raw) {
        DiagnosticKind::ZeroFraction
    } else {
        return None;
    };

    Some(State { kind, parts })
}

#[derive(Clone, Copy)]
struct NumberLiteralParts {
    dot_index: usize,
    fraction_end: usize,
}

impl NumberLiteralParts {
    fn before<'a>(&self, raw: &'a str) -> &'a str {
        debug_assert!(self.dot_index <= raw.len());
        raw.get(..self.dot_index).unwrap_or("")
    }

    fn fraction<'a>(&self, raw: &'a str) -> &'a str {
        let fraction_start = self.dot_index.saturating_add(1);
        debug_assert!(fraction_start <= self.fraction_end);
        debug_assert!(self.fraction_end <= raw.len());
        raw.get(fraction_start..self.fraction_end).unwrap_or("")
    }

    fn after<'a>(&self, raw: &'a str) -> &'a str {
        debug_assert!(self.fraction_end <= raw.len());
        raw.get(self.fraction_end..).unwrap_or("")
    }

    fn trimmed_fraction<'a>(&self, raw: &'a str) -> &'a str {
        self.fraction(raw).trim_end_matches(['0', '_'])
    }

    fn format(&self, raw: &str) -> Option<String> {
        let trimmed_fraction = self.trimmed_fraction(raw);

        let mut formatted = String::new();
        if self.before(raw).is_empty() && trimmed_fraction.is_empty() {
            formatted.push('0');
        } else {
            formatted.push_str(self.before(raw));
            if !trimmed_fraction.is_empty() {
                formatted.push('.');
                formatted.push_str(trimmed_fraction);
            }
        }
        formatted.push_str(self.after(raw));

        if formatted == raw {
            None
        } else {
            Some(formatted)
        }
    }
}

fn split_number_literal(raw: &str) -> Option<NumberLiteralParts> {
    let dot_index = raw.find('.')?;
    let (_, after_dot) = raw.split_at(dot_index);
    let after_dot = &after_dot[1..];
    let fraction_end = after_dot
        .find(|c: char| !c.is_ascii_digit() && c != '_')
        .unwrap_or(after_dot.len());

    Some(NumberLiteralParts {
        dot_index,
        fraction_end: dot_index + 1 + fraction_end,
    })
}

fn make_replacement_expression(formatted: &str) -> AnyJsExpression {
    AnyJsExpression::AnyJsLiteralExpression(AnyJsLiteralExpression::JsNumberLiteralExpression(
        make::js_number_literal_expression(make::js_number_literal(formatted)),
    ))
}

fn old_expression(node: &JsNumberLiteralExpression) -> AnyJsExpression {
    AnyJsExpression::AnyJsLiteralExpression(AnyJsLiteralExpression::JsNumberLiteralExpression(
        node.clone(),
    ))
}

fn replace_expression_statement(
    mutation: &mut biome_rowan::BatchMutation<biome_js_syntax::JsLanguage>,
    statement: &JsExpressionStatement,
    replacement: AnyJsExpression,
    leading_trivia: Vec<SyntaxTriviaPiece<biome_js_syntax::JsLanguage>>,
) -> Option<()> {
    let mut new_statement = make::js_expression_statement(replacement);
    if let Some(semicolon_token) = statement.semicolon_token() {
        new_statement = new_statement.with_semicolon_token(semicolon_token);
    }
    let new_statement = new_statement.build();

    if let Some(parent) = statement.parent::<JsStatementList>() {
        let index = parent
            .iter()
            .position(|item: AnyJsStatement| item.syntax() == statement.syntax())?;

        let replacement_items: Vec<AnyJsStatement> = if index > 0 {
            vec![
                make::js_empty_statement(
                    make::token(T![;]).with_leading_trivia_pieces(leading_trivia.clone()),
                )
                .into(),
                new_statement.into(),
            ]
        } else {
            vec![new_statement.into()]
        };

        mutation.replace_node(
            parent.clone(),
            parent.splice(index..=index, replacement_items),
        );
        return Some(());
    }

    let parent = statement.parent::<JsModuleItemList>()?;
    let index = parent
        .iter()
        .position(|item: AnyJsModuleItem| item.syntax() == statement.syntax())?;

    let replacement_items: Vec<AnyJsModuleItem> = if index > 0 {
        vec![
            AnyJsModuleItem::AnyJsStatement(
                make::js_empty_statement(
                    make::token(T![;]).with_leading_trivia_pieces(leading_trivia),
                )
                .into(),
            ),
            AnyJsModuleItem::AnyJsStatement(new_statement.into()),
        ]
    } else {
        vec![AnyJsModuleItem::AnyJsStatement(new_statement.into())]
    };

    mutation.replace_node(
        parent.clone(),
        parent.splice(index..=index, replacement_items),
    );
    Some(())
}

fn needs_parentheses(node: &JsNumberLiteralExpression, formatted: &str) -> bool {
    if !is_decimal_integer(formatted) {
        return false;
    }

    let Some(parent) = node.syntax().parent() else {
        return false;
    };
    let Some(parent) = AnyJsMemberExpression::cast(parent) else {
        return false;
    };

    match parent {
        AnyJsMemberExpression::JsStaticMemberExpression(parent) => parent
            .object()
            .ok()
            .is_some_and(|object| object.syntax() == node.syntax()),
        AnyJsMemberExpression::JsComputedMemberExpression(_) => false,
    }
}

fn is_decimal_integer(formatted: &str) -> bool {
    formatted
        .bytes()
        .all(|byte| byte.is_ascii_digit() || byte == b'_')
}
