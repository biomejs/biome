use crate::JsRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, AnyJsMemberExpression, JsExpressionStatement,
    JsLanguage, JsModuleItemList, JsNumberLiteralExpression, JsStatementList, T,
};
use biome_rowan::{AstNode, BatchMutation, BatchMutationExt, SyntaxTriviaPiece};
use biome_rule_options::no_zero_fractions::NoZeroFractionsOptions;

declare_lint_rule! {
    /// Disallow number literals with zero fractions or dangling dots.
    ///
    /// There is no difference in JavaScript between, for example, `1`, `1.0`, and `1.`.
    /// This rule suggests the shorter form for consistency and brevity.
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
        version: "2.5.16",
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
        State::from_query(ctx.query())
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

        let mut mutation = ctx.root().begin();
        if needs_parentheses(node, &formatted) {
            let replacement = AnyJsExpression::from(make::js_parenthesized_expression(
                make::token(T!['(']),
                make_replacement_expression(&formatted),
                make::token(T![')']),
            ))
            .append_trivia_pieces(node.syntax().last_trailing_trivia()?.pieces())?;
            let leading_trivia = node.syntax().first_leading_trivia()?;

            if let Some(statement) = statement_merged_by_asi(node) {
                insert_semicolon_before(
                    &mut mutation,
                    node,
                    &statement,
                    replacement,
                    leading_trivia.pieces().collect(),
                )?;
            } else {
                let replacement = replacement.prepend_trivia_pieces(leading_trivia.pieces())?;
                mutation.replace_node_discard_trivia(old_expression(node), replacement);
            }
        } else {
            mutation.replace_token_transfer_trivia(token, make::js_number_literal(&formatted));
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

impl State {
    fn from_query(node: &JsNumberLiteralExpression) -> Option<Self> {
        let token = node.value_token().ok()?;
        let raw = token.text_trimmed();
        let parts = split_number_literal(raw)?;
        let kind = if parts.fraction(raw).is_empty() {
            DiagnosticKind::DanglingDot
        } else if parts.trimmed_fraction(raw) != parts.fraction(raw) {
            DiagnosticKind::ZeroFraction
        } else {
            return None;
        };

        Some(Self { kind, parts })
    }
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

/// Returns the expression statement that starts with `node` if wrapping `node` in parentheses
/// would make ASI merge the statement into the previous one, e.g. `foo\n(1).toString()` is
/// parsed as `foo(1).toString()`.
fn statement_merged_by_asi(node: &JsNumberLiteralExpression) -> Option<JsExpressionStatement> {
    let statement = node
        .syntax()
        .ancestors()
        .find_map(JsExpressionStatement::cast)?;
    let first_token = statement.syntax().first_token()?;
    if first_token != node.value_token().ok()? {
        return None;
    }

    let parent = statement.syntax().parent()?;
    if !JsStatementList::can_cast(parent.kind()) && !JsModuleItemList::can_cast(parent.kind()) {
        return None;
    }
    statement.syntax().prev_sibling()?;

    let previous_token = first_token.prev_token()?;
    (previous_token.kind() != T![;]).then_some(statement)
}

/// Replaces `node` with `replacement` and inserts an empty statement before `statement`, so the
/// parenthesized literal can't be merged into the previous statement.
fn insert_semicolon_before(
    mutation: &mut BatchMutation<JsLanguage>,
    node: &JsNumberLiteralExpression,
    statement: &JsExpressionStatement,
    replacement: AnyJsExpression,
    leading_trivia: Vec<SyntaxTriviaPiece<JsLanguage>>,
) -> Option<()> {
    let new_statement = statement.syntax().clone().replace_child(
        node.syntax().clone().into(),
        replacement.into_syntax().into(),
    )?;
    let semicolon =
        make::js_empty_statement(make::token(T![;]).with_leading_trivia_pieces(leading_trivia));

    let list = statement.syntax().parent()?;
    let index = statement.syntax().index();
    let new_list = list.clone().splice_slots(
        index..=index,
        [
            Some(semicolon.into_syntax().into()),
            Some(new_statement.into()),
        ],
    );
    mutation.replace_element_discard_trivia(list.into(), new_list.into());
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
            .is_ok_and(|object| object.syntax() == node.syntax()),
        AnyJsMemberExpression::JsComputedMemberExpression(_) => false,
    }
}

fn is_decimal_integer(formatted: &str) -> bool {
    formatted
        .bytes()
        .all(|byte| byte.is_ascii_digit() || byte == b'_')
}
