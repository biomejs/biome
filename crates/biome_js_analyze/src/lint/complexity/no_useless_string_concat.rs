use std::borrow::Cow;

use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make::{
    self, js_binary_expression, js_string_literal, js_string_literal_expression,
};
use biome_js_syntax::{
    numbers::parse_js_number_with_single_rounding, AnyJsExpression, AnyJsLiteralExpression,
    AnyJsTemplateElement, JsBinaryExpression, JsBinaryOperator, JsExpressionStatement,
    JsFunctionBody, JsLanguage, JsModule, JsScript, JsSyntaxKind, JsSyntaxToken,
    JsTemplateExpression, T,
};
use biome_rowan::{
    AstNode, AstNodeList, BatchMutationExt, Direction, SyntaxKindSet, TextRange, TextSize, TokenText,
};
use biome_rule_options::no_useless_string_concat::NoUselessStringConcatOptions;
use biome_unicode_table::{Dispatch, lookup_byte};

use crate::JsRuleAction;
use crate::ast_utils::starts_unterminated_statement;

declare_lint_rule! {
    /// Disallow unnecessary concatenation of string or template literals.
    ///
    /// This rule aims to flag concatenation of string or template literals when they could be combined into a single literal.
    /// Notably, this also includes concatenating a string with a number (unlike the derivative ESLint rule).
    ///
    /// Concatenations split across multiple lines are allowed (such as ones used to prevent exceeding the maximum line width).
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const a = "a" + "b";
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const foo = "string" + 123;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const a = "a" + "b" + "c";
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const a = (foo + "a") + ("b" + "c");
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const path = directory + "/" + "file.js";
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const greeting = `Hello, ${name}` + "! Welcome.";
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const a = 1 + 1;
    /// ```
    ///
    /// ```js
    /// const a = 1 * '2';
    /// ```
    ///
    /// ```js
    /// const a = 1 - 2;
    /// ```
    ///
    /// ```js
    /// const a = foo + bar;
    /// ```
    ///
    /// ```js
    /// const a = 'foo' + bar;
    /// ```
    ///
    /// Multi-line strings are ignored:
    ///
    /// ```js
    /// const multiline = 'foo' + // formatting
    ///           'bar'
    /// ```
    ///
    /// ```js
    /// const alsoMultiline = 'foo'
    ///           + 'bar'
    ///           + `baz`
    /// ```
    ///
    /// Strings that would join into text that looks like a template literal placeholder are ignored,
    /// because they are usually split on purpose:
    ///
    /// ```js
    /// const placeholder = '$' + '{name}';
    /// ```
    pub NoUselessStringConcat {
        version: "1.8.0",
        name: "noUselessStringConcat",
        language: "js",
        sources: &[
            RuleSource::Eslint("no-useless-concat").same(),
            RuleSource::EslintVueJs("no-useless-concat").same(),
            RuleSource::EslintUnicorn("no-useless-concat").inspired(),
        ],
        recommended: false,
        severity: Severity::Information,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for NoUselessStringConcat {
    type Query = Ast<JsBinaryExpression>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = NoUselessStringConcatOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let parent_binary_expression = get_parent_binary_expression(node);

        // Prevent duplicated error reportings when the parent is a useless concatenation too, i.e.: "a" + "b" + "c"
        if parent_binary_expression
            .is_some_and(|parent_expression| get_concatenation_range(&parent_expression).is_some())
        {
            return None;
        }

        get_concatenation_range(node)
    }

    fn diagnostic(_ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "Useless string concatenation."
                },
            )
            .note(markup! {
                "Consider turning the expression into a single string to improve readability and runtime performance."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();
        let left = node.left().ok();
        let right = node.right().ok();

        if is_numeric_calculation(&left) || is_numeric_calculation(&right) {
            return None;
        }

        let left_string = extract_string_value(&left);
        let right_string = extract_string_value(&right);

        let fix_result = match (left, left_string, right_string) {
            // Handle simple concatenations like "a" + "b"
            (_, Some(left_string_value), Some(right_string_value)) => {
                join_literal_text(left_string_value, &right_string_value).map(
                    |concatenated_string| {
                        let string_literal_expression = js_string_literal_expression(
                            escaped_string_literal(&concatenated_string),
                        );
                        mutation
                            .replace_element(node.clone().into(), string_literal_expression.into());
                    },
                )
            }

            // Handle nested concatenations like "a" + "b" + "c"
            (
                Some(AnyJsExpression::JsBinaryExpression(left_binary_expression)),
                None,
                Some(right_string_value),
            ) => concat_binary_expression(&left_binary_expression, right_string_value.as_str())
                .map(|binary_expression| {
                    mutation.replace_element(node.clone().into(), binary_expression.into());
                }),

            // Handle concatenations where the left part is a parenthesized expression, like ("a" + "b") + "c"
            (
                Some(AnyJsExpression::JsParenthesizedExpression(left_parenthesized_expression)),
                _,
                Some(right_string),
            ) => match left_parenthesized_expression.expression() {
                Ok(AnyJsExpression::JsBinaryExpression(left_binary_expression)) => {
                    concat_binary_expression(&left_binary_expression, right_string.as_str()).map(
                        |binary_expression| {
                            mutation.replace_element(node.clone().into(), binary_expression.into());
                        },
                    )
                }
                _ => None,
            },

            _ => None,
        }
        .or_else(|| {
            let template_merge = TemplateMerge::from_binary_expression(node)?;
            if template_merge.reorders_evaluation() {
                return None;
            }
            let template = template_merge.to_template_expression()?;
            let replacement = match template_merge.preceding {
                Some((preceding, operator)) => {
                    js_binary_expression(preceding, operator, template.into()).into()
                }
                None => {
                    // `foo` followed by a template literal on the next line is a tagged template.
                    if starts_unterminated_statement(&node.clone().into()) {
                        return None;
                    }
                    AnyJsExpression::from(template)
                }
            };
            mutation.replace_element(node.clone().into(), replacement.into_syntax().into());
            Some(())
        });

        fix_result.and(Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the useless concatenation" }.to_owned(),
            mutation,
        )))
    }
}

/// Returns if the passed `JsBinaryExpression` is the concatenation of two string literals.
fn is_string_concatenation(binary_expression: &JsBinaryExpression) -> bool {
    let left = binary_expression.left().ok();
    let right = binary_expression.right().ok();
    let has_left_string_expression = is_string_expression(&left)
        || is_binary_expression_with_literal_string(&left)
        || is_parenthesized_concatenation(&left);
    // The first operand of a parenthesized concatenation on the right, not its last, is next
    // to the `+`, so it only counts when it is made of literals alone.
    let has_right_string_expression = is_string_expression(&right)
        || (is_parenthesized_concatenation(&right) && extract_string_value(&right).is_some());
    let has_left_numeric_expression = is_numeric_expression(&left) || is_numeric_calculation(&left);
    let has_right_numeric_expression =
        is_numeric_expression(&right) || is_numeric_calculation(&right);
    let has_string_expression = match (
        has_left_string_expression,
        has_left_numeric_expression,
        has_right_string_expression,
        has_right_numeric_expression,
    ) {
        (true, _, true, _) => true,     // "a" + "b"
        (false, true, true, _) => true, // 1 + "a"
        (true, _, false, true) => true, // "a" + 1
        _ => false,
    };

    let operator = binary_expression.operator().ok();
    let has_plus_operator = matches!(operator, Some(JsBinaryOperator::Plus));

    has_plus_operator && has_string_expression
}

fn is_string_expression(expression: &Option<AnyJsExpression>) -> bool {
    expression.clone().is_some_and(|node| {
        let node = node.omit_parentheses();
        match (
            node.as_any_js_literal_expression(),
            node.as_js_template_expression(),
        ) {
            (Some(literal_expression), _) => literal_expression
                .as_js_string_literal_expression()
                .is_some(),
            (_, Some(template_expression)) => template_expression.tag().is_none(),
            _ => false,
        }
    })
}

fn is_numeric_expression(expression: &Option<AnyJsExpression>) -> bool {
    match expression.as_ref() {
        Some(AnyJsExpression::AnyJsLiteralExpression(literal_expression)) => matches!(
            literal_expression,
            AnyJsLiteralExpression::JsNumberLiteralExpression(_)
        ),

        _ => false,
    }
}

fn is_numeric_calculation(expression: &Option<AnyJsExpression>) -> bool {
    match expression.as_ref() {
        Some(AnyJsExpression::JsBinaryExpression(binary_expression)) => {
            is_numeric_expression(&binary_expression.left().ok())
                && is_numeric_expression(&binary_expression.right().ok())
        }
        _ => false,
    }
}

fn is_binary_expression_with_literal_string(expression: &Option<AnyJsExpression>) -> bool {
    // Adding a string to any value produces a string, so `foo + 'a'` is a string whatever `foo` is.
    if let Some(AnyJsExpression::JsBinaryExpression(binary_expression)) = expression {
        return binary_expression.operator() == Ok(JsBinaryOperator::Plus)
            && is_string_expression(&binary_expression.right().ok());
    }

    false
}

fn is_parenthesized_concatenation(expression: &Option<AnyJsExpression>) -> bool {
    if let Some(AnyJsExpression::JsParenthesizedExpression(parenthesized_expression)) = expression {
        return is_binary_expression_with_literal_string(
            &parenthesized_expression.expression().ok(),
        );
    }

    false
}

/// Returns the diagnostic range of the passed `JsBinaryExpression`, or `None` if it is either
/// not a string concatenation or stylistically exempt.
fn get_concatenation_range(binary_expression: &JsBinaryExpression) -> Option<TextRange> {
    if !is_string_concatenation(binary_expression) {
        return None;
    }

    if is_stylistic_concatenation(binary_expression)
        || creates_template_placeholder(binary_expression)
        || creates_use_directive(binary_expression)
    {
        return None;
    }

    let left = binary_expression.left().ok();
    let right = binary_expression.right().ok();
    let range_start = if is_binary_expression_with_literal_string(&left) {
        match left {
            Some(AnyJsExpression::JsBinaryExpression(left_binary_expression)) => {
                extract_concat_range(&left_binary_expression)
            }
            _ => None,
        }
    } else {
        left.map(|left| left.range().start())
    };
    let range_end = right.map(|right| right.range().end());

    match (range_start, range_end) {
        (Some(range_start), Some(range_end)) => Some(TextRange::new(range_start, range_end)),
        _ => None,
    }
}

/// Returns if the passed `JsBinaryExpression` is a multiline string concatenation,
/// meaning either the operator or RHS have a leading newline.
/// Assumes the expression is known to be a valid concatenation that will otherwise trigger the rule.
fn is_stylistic_concatenation(binary_expression: &JsBinaryExpression) -> bool {
    let has_newline_in_operator = binary_expression
        .operator_token()
        .is_ok_and(|operator| operator.has_leading_newline());
    let has_newline_in_right = binary_expression
        .right()
        .is_ok_and(|right| right.syntax().has_leading_newline());

    has_newline_in_operator || has_newline_in_right
}

fn extract_string_value(expression: &Option<AnyJsExpression>) -> Option<String> {
    match expression {
        Some(AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsStringLiteralExpression(string_literal_expression),
        )) => string_literal_expression
            .inner_string_text()
            .map(|token_text| token_text.to_string())
            .ok(),

        Some(AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsNumberLiteralExpression(number_literal_expression),
        )) => {
            let token = number_literal_expression.value_token().ok()?;
            let text = token.text_trimmed();
            let number_value = parse_js_number_with_single_rounding(text)?;
            let mut buffer = ryu_js::Buffer::new();
            Some(buffer.format(number_value).to_string())
        }

        Some(AnyJsExpression::JsBinaryExpression(binary_expression))
            if binary_expression.operator() == Ok(JsBinaryOperator::Plus) =>
        {
            let left = binary_expression.left().ok()?;
            let right = binary_expression.right().ok()?;
            // Adding two numbers produces their sum, not their digits joined. Literals on
            // different lines are left apart.
            if (is_number_literal(&left) && is_number_literal(&right))
                || is_stylistic_concatenation(binary_expression)
            {
                return None;
            }
            join_literal_text(
                extract_string_value(&Some(left))?,
                &extract_string_value(&Some(right))?,
            )
        }

        Some(AnyJsExpression::JsParenthesizedExpression(parenthesized_expression)) => {
            extract_string_value(&parenthesized_expression.expression().ok())
        }

        Some(AnyJsExpression::JsTemplateExpression(template_expression)) => {
            // Tagged templates (e.g. sql`query`) have different semantics and
            // their string value cannot be extracted.
            if template_expression.tag().is_some() {
                return None;
            }

            let is_useless_template_literal = template_expression
                .elements()
                .into_iter()
                .all(|element| element.as_js_template_chunk_element().is_some());

            if is_useless_template_literal {
                let concatenated_string = template_expression.elements().into_iter().fold(
                    String::new(),
                    |acc, element| {
                        if let Some(chunk) = element.as_js_template_chunk_element() {
                            return acc + chunk.to_trimmed_text().text();
                        }
                        acc
                    },
                );

                // A string literal can't contain a line break, unlike a template literal.
                if concatenated_string.contains(['\n', '\r']) {
                    return None;
                }
                return Some(concatenated_string);
            }

            None
        }

        _ => None,
    }
}

/// Returns the start of the string literals that end the `+` chain `binary_expression`,
/// as `'a'` in `foo + 'a' + 'b'`.
fn extract_concat_range(binary_expression: &JsBinaryExpression) -> Option<TextSize> {
    let mut start = binary_expression.right().ok()?.range().start();
    if is_stylistic_concatenation(binary_expression) {
        return Some(start);
    }
    let mut current = binary_expression.left().ok()?;
    while let AnyJsExpression::JsBinaryExpression(binary_expression) = &current {
        if !is_binary_expression_with_literal_string(&Some(current.clone())) {
            return Some(start);
        }
        let right = binary_expression.right().ok()?;
        start = right.range().start();
        if is_stylistic_concatenation(binary_expression) {
            return Some(start);
        }
        current = binary_expression.left().ok()?;
    }
    if is_string_expression(&Some(current.clone())) {
        start = current.range().start();
    }
    Some(start)
}

fn get_parent_binary_expression(node: &JsBinaryExpression) -> Option<JsBinaryExpression> {
    let mut current_node = node.parent();

    while current_node.is_some() {
        if let Some(AnyJsExpression::JsBinaryExpression(binary_expression)) = current_node {
            return Some(binary_expression);
        } else if let Some(c) = current_node {
            current_node = c.parent()
        }
    }

    None
}

fn escaped_string_literal(value: &str) -> JsSyntaxToken {
    js_string_literal(&escape_double_quotes(value))
}

fn escape_double_quotes(value: &str) -> Cow<'_, str> {
    let mut escaped = String::new();
    let mut bytes = value.bytes().enumerate();
    let mut last_copied_index = 0;

    while let Some((index, byte)) = bytes.next() {
        if byte == b'\\' {
            bytes.next();
        } else if byte == b'"' {
            if escaped.is_empty() {
                escaped = String::with_capacity(value.len());
            }
            escaped.push_str(&value[last_copied_index..index]);
            escaped.push('\\');
            last_copied_index = index;
        }
    }

    if escaped.is_empty() {
        Cow::Borrowed(value)
    } else {
        escaped.push_str(&value[last_copied_index..]);
        Cow::Owned(escaped)
    }
}

fn concat_binary_expression(
    left_binary_expression: &JsBinaryExpression,
    right_string_value: &str,
) -> Option<JsBinaryExpression> {
    let current_right = left_binary_expression.right().ok();

    if let Some(current_right) = current_right.filter(|right| is_string_expression(&Some(right.clone()))) {
        let value = extract_string_value(&Some(current_right.clone()))?;
        let concatenated_string = join_literal_text(value, right_string_value)?;
        // Keep the line break or comment before the literal that the new string replaces.
        let string_literal_expression =
            AnyJsExpression::AnyJsLiteralExpression(AnyJsLiteralExpression::from(
                js_string_literal_expression(escaped_string_literal(&concatenated_string))
                    .with_leading_trivia_pieces(
                        current_right.syntax().first_leading_trivia()?.pieces(),
                    )?,
            ));
        let left = left_binary_expression.left().ok()?;
        let operator = left_binary_expression.operator_token().ok()?;

        // Only concatenate strings when we are sure that the evaluated expression will turn
        // into a string. Literals on different lines are left apart, as in the whole
        // concatenation. If the literals further left can't be joined, keep them as they are.
        if !is_stylistic_concatenation(left_binary_expression)
            && let AnyJsExpression::JsBinaryExpression(binary_expression) = &left
            && is_binary_expression_with_literal_string(&Some(left.clone()))
            && let Some(result) = concat_binary_expression(binary_expression, &concatenated_string)
        {
            return Some(result);
        }
        return Some(js_binary_expression(left, operator, string_literal_expression));
    }

    Some(left_binary_expression.clone())
}

/// Returns `true` if joining the literals at the end of `binary_expression` creates text that
/// looks like a template literal placeholder, as in `'$' + '{name}'`. Such text is usually
/// split on purpose, so it isn't read as a placeholder.
fn creates_template_placeholder(binary_expression: &JsBinaryExpression) -> bool {
    if let Some(template_merge) = TemplateMerge::from_binary_expression(binary_expression) {
        return template_merge.creates_placeholder();
    }
    trailing_literal_text(binary_expression).is_some_and(|text| has_template_placeholder(&text))
}

/// Returns the values of the literals at the end of the `+` chain `binary_expression`, joined
/// together, as `ab` in `foo + 'a' + 'b'`.
fn trailing_literal_text(binary_expression: &JsBinaryExpression) -> Option<String> {
    let mut text = extract_string_value(&binary_expression.right().ok())?;
    let mut current = binary_expression.left().ok()?.omit_parentheses();
    loop {
        match current {
            AnyJsExpression::JsBinaryExpression(binary_expression)
                if binary_expression.operator() == Ok(JsBinaryOperator::Plus) =>
            {
                let Some(value) = extract_string_value(&binary_expression.right().ok()) else {
                    return Some(text);
                };
                text.insert_str(0, &value);
                if is_stylistic_concatenation(&binary_expression) {
                    return Some(text);
                }
                current = binary_expression.left().ok()?.omit_parentheses();
            }
            current => {
                if let Some(value) = extract_string_value(&Some(current)) {
                    text.insert_str(0, &value);
                }
                return Some(text);
            }
        }
    }
}

/// Returns `true` if `text` contains `${` and `}` with at least one character between them.
fn has_template_placeholder(text: &str) -> bool {
    text.match_indices("${")
        .any(|(index, _)| text[index + 2..].find('}').is_some_and(|end| end > 0))
}

/// Returns `true` if the fix creates a directive such as `"use strict"`, which changes how the
/// code around it runs. The new string literal statement becomes a directive itself, and so do
/// the string literal statements right after it.
fn creates_use_directive(binary_expression: &JsBinaryExpression) -> bool {
    let Some(statement) = binary_expression.parent::<JsExpressionStatement>() else {
        return false;
    };
    // The parser keeps directives apart from the statements that follow them, so the statement
    // becomes a directive only if it is the first statement of its body.
    if statement.syntax().prev_sibling().is_some()
        || !statement
            .syntax()
            .grand_parent()
            .is_some_and(|node| DIRECTIVE_PARENT_KINDS.matches(node.kind()))
    {
        return false;
    }
    // The new string literal statement also turns string literal statements right after it
    // into directives. The parser can put the `;` of the statement in an empty statement.
    let is_next_statement_string = statement
        .syntax()
        .siblings(Direction::Next)
        .skip(1)
        .find(|node| node.kind() != JsSyntaxKind::JS_EMPTY_STATEMENT)
        .and_then(JsExpressionStatement::cast)
        .and_then(|statement| statement.expression().ok())
        .is_some_and(|expression| {
            expression
                .as_any_js_literal_expression()
                .is_some_and(|literal| literal.as_js_string_literal_expression().is_some())
        });
    is_next_statement_string
        || extract_string_value(&Some(binary_expression.clone().into()))
            .is_some_and(|text| is_use_directive(&text))
}

/// The nodes that can start with directives.
const DIRECTIVE_PARENT_KINDS: SyntaxKindSet<JsLanguage> = JsFunctionBody::KIND_SET
    .union(JsScript::KIND_SET)
    .union(JsModule::KIND_SET);

/// Returns `true` if the source text of a string starts with `use` and a space, like
/// `use strict` and `use client`. A backslash after `use` counts as a space, because it can
/// start an escape sequence for one.
fn is_use_directive(text: &str) -> bool {
    text.get(..3)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("use"))
        && text[3..].starts_with(|c: char| c.is_whitespace() || c == '\\')
}

/// Two literals joined by `+` that the fix merges into one template literal, because one of
/// them is a template literal with placeholders, as in `` `Hello, ${name}` + "!" ``.
struct TemplateMerge {
    /// The operands before `left` in a `+` chain and the `+` after them, as `foo +` in
    /// `` foo + `a${b}` + "c" ``.
    preceding: Option<(AnyJsExpression, JsSyntaxToken)>,
    left: AnyJsExpression,
    right: AnyJsExpression,
}

impl TemplateMerge {
    fn from_binary_expression(binary_expression: &JsBinaryExpression) -> Option<Self> {
        let right = binary_expression.right().ok()?;
        let (preceding, left) = match binary_expression.left().ok()? {
            AnyJsExpression::JsBinaryExpression(left)
                if is_binary_expression_with_literal_string(&Some(left.clone().into())) =>
            {
                (
                    Some((left.left().ok()?, left.operator_token().ok()?)),
                    left.right().ok()?,
                )
            }
            left => (None, left),
        };
        if !is_string_expression(&Some(left.clone())) || !is_string_expression(&Some(right.clone()))
        {
            return None;
        }
        let left = left.omit_parentheses();
        let right = right.omit_parentheses();
        if !has_placeholders(&left) && !has_placeholders(&right) {
            return None;
        }
        Some(Self {
            preceding,
            left,
            right,
        })
    }

    /// Returns `true` if `left` ends with `$` and `right` starts with `{`, so the merged
    /// template literal would have a new placeholder.
    fn creates_placeholder(&self) -> bool {
        let ends_with_dollar = match &self.left {
            AnyJsExpression::JsTemplateExpression(template) => template
                .elements()
                .last()
                .and_then(|element| chunk_token(&element))
                .is_some_and(|token| token.text_trimmed().ends_with('$')),
            left => string_literal_text(left).is_some_and(|text| text.text().ends_with('$')),
        };
        let starts_with_curly = match &self.right {
            AnyJsExpression::JsTemplateExpression(template) => template
                .elements()
                .first()
                .and_then(|element| chunk_token(&element))
                .is_some_and(|token| token.text_trimmed().starts_with('{')),
            right => string_literal_text(right).is_some_and(|text| text.text().starts_with('{')),
        };
        ends_with_dollar && starts_with_curly
    }

    /// Returns `true` if the merge moves the placeholders of `right` before the `+` that
    /// follows the preceding operands, as in `` foo + 'a' + `${bar()}` ``. The fix would
    /// call `bar()` before converting `foo` to a string.
    fn reorders_evaluation(&self) -> bool {
        self.preceding.as_ref().is_some_and(|(preceding, _)| {
            extract_string_value(&Some(preceding.clone())).is_none()
        }) && has_placeholders(&self.right)
    }

    /// Returns `None` if a string literal has an escape sequence that template literals don't
    /// allow, such as the legacy octal escape `\1`.
    fn to_template_expression(&self) -> Option<JsTemplateExpression> {
        let mut elements = Vec::new();
        for literal in [&self.left, &self.right] {
            if let AnyJsExpression::JsTemplateExpression(template) = literal {
                for element in template.elements() {
                    push_template_element(&mut elements, element)?;
                }
                continue;
            }
            let text = string_literal_text(literal)?;
            if has_template_incompatible_escape(text.text()) {
                return None;
            }
            if !text.text().is_empty() {
                push_template_element(
                    &mut elements,
                    make::js_template_chunk_element(make::js_template_chunk(text.text())).into(),
                )?;
            }
        }
        Some(
            make::js_template_expression(
                make::token(T!['`']),
                make::js_template_element_list(elements),
                make::token(T!['`']),
            )
            .build(),
        )
    }
}

fn has_placeholders(literal: &AnyJsExpression) -> bool {
    literal
        .as_js_template_expression()
        .is_some_and(|template| !template.is_constant())
}

fn string_literal_text(literal: &AnyJsExpression) -> Option<TokenText> {
    literal
        .as_any_js_literal_expression()?
        .as_js_string_literal_expression()?
        .inner_string_text()
        .ok()
}

fn chunk_token(element: &AnyJsTemplateElement) -> Option<JsSyntaxToken> {
    element
        .as_js_template_chunk_element()?
        .template_chunk_token()
        .ok()
}

/// Appends `element` to `elements`, and joins it with the last element when both are text.
/// Returns `None` if joining the text changes an escape sequence, as in [join_literal_text].
fn push_template_element(
    elements: &mut Vec<AnyJsTemplateElement>,
    element: AnyJsTemplateElement,
) -> Option<()> {
    let previous = elements.last().and_then(chunk_token);
    let (Some(previous), Some(next)) = (previous, chunk_token(&element)) else {
        elements.push(element);
        return Some(());
    };
    let text = join_literal_text(
        previous.text_trimmed().to_owned(),
        next.text_trimmed(),
    )?;
    elements.pop();
    elements.push(
        make::js_template_chunk_element(JsSyntaxToken::new_detached(
            JsSyntaxKind::TEMPLATE_CHUNK,
            &text,
            [],
            [],
        ))
        .into(),
    );
    Some(())
}

/// Returns `true` if the source text of a string literal has an escape sequence that template
/// literals don't allow: `\1` to `\9`, or `\0` followed by a digit.
fn has_template_incompatible_escape(text: &str) -> bool {
    let mut bytes = text.bytes();
    while let Some(byte) = bytes.next() {
        if byte != b'\\' {
            continue;
        }
        match bytes.next().map(lookup_byte) {
            Some(Dispatch::DIG) => return true,
            Some(Dispatch::ZER)
                if bytes
                    .clone()
                    .next()
                    .is_some_and(is_digit) =>
            {
                return true;
            }
            _ => {}
        }
    }
    false
}

fn is_number_literal(expression: &AnyJsExpression) -> bool {
    matches!(
        expression.clone().omit_parentheses(),
        AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsNumberLiteralExpression(_)
        )
    )
}

/// Joins the source text of two literals. Returns `None` if the digits at the start of `right`
/// would extend an escape sequence at the end of `left`, as `'\0' + '1'` would form `'\01'`.
fn join_literal_text(mut left: String, right: &str) -> Option<String> {
    if right.bytes().next().is_some_and(is_digit) {
        let digits = left.bytes().rev().take_while(|byte| is_digit(*byte)).count();
        let backslashes = left.as_bytes()[..left.len() - digits]
            .iter()
            .rev()
            .take_while(|byte| **byte == b'\\')
            .count();
        if digits > 0 && backslashes % 2 == 1 {
            return None;
        }
    }
    left.push_str(right);
    Some(left)
}

fn is_digit(byte: u8) -> bool {
    matches!(lookup_byte(byte), Dispatch::ZER | Dispatch::DIG)
}
