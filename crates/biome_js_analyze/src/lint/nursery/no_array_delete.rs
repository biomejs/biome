use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsLiteralExpression, JsComputedMemberExpression,
    JsExpressionStatement, JsSyntaxKind, JsUnaryExpression, JsUnaryOperator, T,
};
use biome_parser::{TokenSet, token_set};
use biome_rowan::{AstNode, BatchMutationExt, Direction};
use biome_rule_options::no_array_delete::NoArrayDeleteOptions;

use crate::JsRuleAction;
use crate::services::typed::Typed;

declare_lint_rule! {
    /// Disallow using the `delete` operator on array elements.
    ///
    /// The `delete` operator removes a property from an object.
    /// When you use it on an array element, the element is removed, but the array doesn't get shorter
    /// and the elements after it don't move.
    /// The array is left with an empty slot, also called a "hole", at that position:
    ///
    /// ```ts,ignore
    /// const array = [1, 2, 3];
    /// delete array[1];
    /// console.log(array); // [1, <1 empty item>, 3]
    /// console.log(array.length); // 3
    /// ```
    ///
    /// Array methods treat holes in different ways. For example, `forEach()` skips them, while
    /// `for...of` loops and `includes()` treat them as `undefined`.
    /// To remove elements from an array, use the
    /// [`splice()`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/splice)
    /// method instead.
    ///
    /// The rule needs type information to know whether a value is an array, so it only reports
    /// values that are known to be arrays or tuples.
    ///
    /// The fix replaces `delete array[index]` with `array.splice(index, 1)`. It's unsafe, because
    /// `splice()` moves the elements after the removed one to lower positions, and it returns the
    /// removed elements instead of `true`.
    /// The fix isn't offered when the expression contains comments, or when the code doesn't use
    /// semicolons and removing `delete` would join the statement to the previous line.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic,file=invalid-array.ts
    /// declare const array: number[];
    /// delete array[0];
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=invalid-tuple.ts
    /// declare const tuple: [string, number];
    /// delete tuple[1];
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts,file=valid.ts
    /// declare const array: number[];
    /// array.splice(0, 1);
    ///
    /// declare const object: Record<string, number>;
    /// delete object["key"];
    /// ```
    ///
    pub NoArrayDelete {
        version: "next",
        name: "noArrayDelete",
        language: "js",
        recommended: true,
        severity: Severity::Error,
        sources: &[RuleSource::EslintTypeScript("no-array-delete").same()],
        fix_kind: FixKind::Unsafe,
        domains: &[RuleDomain::Types],
    }
}

impl Rule for NoArrayDelete {
    type Query = Typed<JsUnaryExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoArrayDeleteOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if node.operator().ok()? != JsUnaryOperator::Delete {
            return None;
        }
        let object = deleted_member(node)?.object().ok()?;
        ctx.type_of_expression(&object)
            .is_some_and(|ty| ty.is_all_array_or_tuple())
            .then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "Avoid using the "<Emphasis>"delete"</Emphasis>" operator on array elements."
                },
            )
            .note(markup! {
                "Deleting an array element leaves an empty slot in the array and doesn't change its length."
            })
            .note(markup! {
                "Use "<Emphasis>"splice()"</Emphasis>" to remove the element instead."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        if has_inner_comments(node) {
            return None;
        }
        let member = deleted_member(node)?;
        let object = member.object().ok()?;
        if may_continue_previous_statement(node, &object) {
            return None;
        }
        let key = member.member().ok()?;
        // A sequence expression such as `a, b` must be wrapped in parentheses, or the
        // call would receive each of its parts as a separate argument.
        let key = if key.as_js_sequence_expression().is_some() {
            AnyJsExpression::JsParenthesizedExpression(make::js_parenthesized_expression(
                make::token(T!['(']),
                key,
                make::token(T![')']),
            ))
        } else {
            key
        };
        let operator = if member.optional_chain_token().is_some() {
            T![?.]
        } else {
            T![.]
        };

        let splice_call = make::js_call_expression(
            make::js_static_member_expression(
                object,
                make::token(operator),
                make::js_name(make::ident("splice")).into(),
            )
            .into(),
            make::js_call_arguments(
                make::token(T!['(']),
                make::js_call_argument_list(
                    [
                        AnyJsCallArgument::AnyJsExpression(key),
                        AnyJsCallArgument::AnyJsExpression(
                            AnyJsExpression::AnyJsLiteralExpression(
                                AnyJsLiteralExpression::JsNumberLiteralExpression(
                                    make::js_number_literal_expression(make::js_number_literal(1)),
                                ),
                            ),
                        ),
                    ],
                    [make::token_with_trailing_space(T![,])],
                ),
                make::token(T![')']),
            ),
        )
        .build();

        let mut mutation = ctx.root().begin();
        mutation.replace_node_transfer_trivia(
            AnyJsExpression::JsUnaryExpression(node.clone()),
            AnyJsExpression::JsCallExpression(splice_call),
        )?;
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use "<Emphasis>"splice()"</Emphasis>" instead." }.to_owned(),
            mutation,
        ))
    }
}

/// Returns the `array[index]` expression that `delete` is applied to, ignoring
/// parentheses around it.
fn deleted_member(node: &JsUnaryExpression) -> Option<JsComputedMemberExpression> {
    node.argument()
        .ok()?
        .omit_parentheses()
        .as_js_computed_member_expression()
        .cloned()
}

/// Returns whether `node` has comments that the fix would lose or misplace, because
/// they are attached to a token that the fix removes, or they would end up before
/// code that the fix adds.
fn has_inner_comments(node: &JsUnaryExpression) -> bool {
    let syntax = node.syntax();
    let first = syntax.first_token();
    let last = syntax.last_token();
    syntax.descendants_tokens(Direction::Next).any(|token| {
        (token.has_leading_comments() && first.as_ref() != Some(&token))
            || (token.has_trailing_comments() && last.as_ref() != Some(&token))
    })
}

/// Returns whether removing `delete` could make the statement continue the previous
/// one. In code without semicolons, `foo()\n(array)[0]` is read as `foo()(array)[0]`,
/// so the fix isn't safe when `object` starts with such a token and `delete` starts a
/// statement that doesn't follow a `;`, `{`, or `:`.
fn may_continue_previous_statement(node: &JsUnaryExpression, object: &AnyJsExpression) -> bool {
    if !object
        .syntax()
        .first_token()
        .is_some_and(|token| CONTINUING_TOKENS.contains(token.kind()))
    {
        return false;
    }
    let Some(delete_token) = node.syntax().first_token() else {
        return false;
    };
    let starts_statement = node
        .syntax()
        .ancestors()
        .find_map(JsExpressionStatement::cast)
        .is_some_and(|statement| statement.syntax().first_token().as_ref() == Some(&delete_token));
    starts_statement
        && delete_token
            .prev_token()
            .is_some_and(|token| !STATEMENT_LIST_STARTS.contains(token.kind()))
}

/// Tokens that continue the previous statement when a line starts with them and the
/// previous line doesn't end with a semicolon.
const CONTINUING_TOKENS: TokenSet<JsSyntaxKind> =
    token_set![T!['('], T!['['], T!['`'], JsSyntaxKind::JS_REGEX_LITERAL];

/// Tokens after which a statement list starts: `;`, `{`, or the `:` of a `case` or `default` clause.
const STATEMENT_LIST_STARTS: TokenSet<JsSyntaxKind> = token_set![T![;], T!['{'], T![:]];
