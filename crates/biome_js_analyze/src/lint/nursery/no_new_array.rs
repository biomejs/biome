use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsArrayElement, AnyJsCallArgument, AnyJsExpression, JsArrayExpression, JsArrowFunctionExpression,
    JsClassExpression, JsFunctionExpression, JsImportCallExpression, JsInExpression,
    JsInstanceofExpression, JsLanguage, JsNewExpression, JsObjectExpression,
    JsRegexLiteralExpression, JsUnaryOperator, JsxTagExpression, T, global_identifier,
    static_value::StaticValue,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, SyntaxKindSet};
use biome_rule_options::no_new_array::NoNewArrayOptions;

use crate::{JsRuleAction, ast_utils::starts_unterminated_statement, services::semantic::Semantic};

declare_lint_rule! {
    /// Disallow `new Array()` with a single argument.
    ///
    /// `new Array()` with one argument does two different things depending on the argument.
    /// If the argument is a number, it creates an empty array of that length.
    /// Otherwise, it creates an array that holds the argument as its only item.
    /// Because of this, readers can't tell from the code alone which one was intended.
    ///
    /// Use an array literal, such as `[value]`, to create an array that holds a value.
    /// Use `Array.from({ length })` to create an array of a given length.
    /// Note that `new Array(length)` creates an array of empty slots,
    /// while `Array.from({ length })` creates an array filled with `undefined`.
    /// Some array methods, such as `forEach()` and `filter()`, skip empty slots.
    ///
    /// When the argument is never a number, such as a string or an object,
    /// Biome replaces the call with an array literal.
    /// Biome doesn't offer a fix when the call has comments that the fix would remove.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const length = 10;
    /// const array = new Array(length);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const array = new Array("value");
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const length = 10;
    /// const empty = Array.from({ length });
    /// const array = ["value"];
    /// const pair = new Array(1, 2);
    /// ```
    ///
    /// ## See Also
    ///
    /// - To use array literals instead of `Array()` with no arguments, several arguments, or a spread argument such as `new Array(...items)`, see [`useArrayLiterals`](https://biomejs.dev/linter/rules/use-array-literals/).
    ///
    pub NoNewArray {
        version: "2.6.7",
        name: "noNewArray",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("no-new-array").inspired()],
        recommended: false,
        severity: Severity::Information,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for NoNewArray {
    type Query = Semantic<JsNewExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoNewArrayOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let arguments = node.arguments()?;
        let [first, second] = arguments.get_arguments_by_index([0, 1]);
        // A spread argument is reported by `useArrayLiterals`.
        if second.is_some() || !matches!(first?, AnyJsCallArgument::AnyJsExpression(_)) {
            return None;
        }
        let callee = node.callee().ok()?.omit_parentheses();
        let (reference, name) =
            global_identifier(&callee.as_any_global_identifier_expression()?)?;
        if name.text() != "Array" || ctx.model().binding(&reference).is_some() {
            return None;
        }
        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "This "<Emphasis>"new Array()"</Emphasis>" call has a single argument, so its result depends on whether the argument is a number."
                },
            )
            .note(markup! {
                "With a number, "<Emphasis>"new Array()"</Emphasis>" creates an empty array of that length. With any other value, it creates an array that holds the value."
            })
            .note(markup! {
                "Use an array literal such as "<Emphasis>"[value]"</Emphasis>" to create an array that holds the value, or "<Emphasis>"Array.from({ length })"</Emphasis>" to create an array of a given length."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        if node.type_arguments().is_some()
            || starts_unterminated_statement(&AnyJsExpression::from(node.clone()))
        {
            return None;
        }
        let arguments = node.arguments()?;
        let l_paren = arguments.l_paren_token().ok()?;
        let r_paren = arguments.r_paren_token().ok()?;
        if node.new_token().ok()?.has_trailing_comments()
            || node.callee().ok()?.syntax().has_comments_descendants()
            || l_paren.has_leading_comments()
        {
            return None;
        }
        let argument_list = arguments.args();
        let argument = argument_list.first()?.ok()?.as_any_js_expression()?.clone();
        if !is_never_number(&argument) {
            return None;
        }

        let replacement = make::js_array_expression(
            make::token(T!['[']).with_trailing_trivia_pieces(l_paren.trailing_trivia().pieces()),
            // Keeping the trailing comma keeps the comments attached to it.
            make::js_array_element_list(
                [AnyJsArrayElement::AnyJsExpression(argument)],
                argument_list.trailing_separator(),
            ),
            make::token(T![']']).with_leading_trivia_pieces(r_paren.leading_trivia().pieces()),
        );
        let mut mutation = ctx.root().begin();
        mutation.replace_node::<AnyJsExpression>(node.clone().into(), replacement.into());
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use an array literal." }.to_owned(),
            mutation,
        ))
    }
}

/// Returns `true` if `expression` can be determined from its syntax alone to never
/// evaluate to a number.
///
/// Returns `false` when the value is a number, or when its type depends on runtime values,
/// such as a variable reference or `a || b`. An `undefined` identifier also returns `false`,
/// because a local variable can be named `undefined`.
fn is_never_number(expression: &AnyJsExpression) -> bool {
    let Some(expression) = expression.inner_expression() else {
        return false;
    };
    match &expression {
        // A tag function can return any value.
        AnyJsExpression::JsTemplateExpression(template) => template.tag().is_none(),
        AnyJsExpression::JsUnaryExpression(unary) => matches!(
            unary.operator(),
            Ok(JsUnaryOperator::Delete
                | JsUnaryOperator::LogicalNot
                | JsUnaryOperator::Typeof
                | JsUnaryOperator::Void)
        ),
        AnyJsExpression::JsBinaryExpression(binary) => binary.is_comparison_operator(),
        _ if NEVER_NUMBER_KINDS.matches(expression.syntax().kind()) => true,
        _ => expression.as_static_value().is_some_and(|value| {
            !matches!(value, StaticValue::Number(_) | StaticValue::Undefined(_))
        }),
    }
}

/// Expressions whose value is always an object, a function, or a boolean.
const NEVER_NUMBER_KINDS: SyntaxKindSet<JsLanguage> = JsArrayExpression::KIND_SET
    .union(JsArrowFunctionExpression::KIND_SET)
    .union(JsClassExpression::KIND_SET)
    .union(JsFunctionExpression::KIND_SET)
    .union(JsImportCallExpression::KIND_SET)
    .union(JsInExpression::KIND_SET)
    .union(JsInstanceofExpression::KIND_SET)
    .union(JsNewExpression::KIND_SET)
    .union(JsObjectExpression::KIND_SET)
    .union(JsRegexLiteralExpression::KIND_SET)
    .union(JsxTagExpression::KIND_SET);
