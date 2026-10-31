use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, AnyJsStatement, JsArrowFunctionExpression, JsCallExpression,
    JsClassExpression, JsComputedMemberExpression, JsExpressionStatement, JsFunctionExpression,
    JsLanguage, JsObjectExpression, JsParenthesizedExpression, JsReturnStatement, JsStatementList,
    JsStaticMemberExpression, JsSyntaxKind, JsSyntaxToken, JsUnaryExpression, T, TsAsExpression,
    TsNonNullAssertionExpression, TsSatisfiesExpression, TsTypeAssertionExpression,
};
use biome_parser::{TokenSet, token_set};
use biome_rowan::{
    AstNode, AstNodeList, AstSeparatedList, BatchMutationExt, Direction, SyntaxKindSet, TextRange,
    TriviaPieceKind,
};
use biome_rule_options::no_return_array_push::NoReturnArrayPushOptions;

use crate::JsRuleAction;

declare_lint_rule! {
    /// Disallow using the return value of `Array.prototype.push()` and `Array.prototype.unshift()`.
    ///
    /// `push()` and `unshift()` add items to an array and return the new length of the array.
    /// They don't return the added item or the array itself.
    /// Returning or storing this length is almost always a mistake, because the code reads as if
    /// the result were meaningful when it says nothing about what was added.
    ///
    /// To add an item and stop the function, call `push()` or `unshift()` before `return`.
    /// If you need the new length, read the `length` property of the array after adding the item.
    ///
    /// The rule only reports calls with at least one argument.
    /// It ignores calls whose result is discarded with the `void` operator, such as `void items.push(item)`,
    /// and calls whose result is accessed with `.` or `[]`, such as `router.push(to).catch(handleError)`,
    /// because this usually means `push()` is a custom method that returns something other than a length.
    ///
    /// The rule also ignores `push()` calls on objects that are commonly not arrays:
    /// `stream`, `router`, `this`, `this.stream`, `this.router`, `this.$router`,
    /// `process.stdin`, `process.stdout`, and `process.stderr`.
    /// It ignores `push()` and `unshift()` calls on values that are written in a way that shows they
    /// aren't arrays: objects (`{}`), functions, classes, template strings, and `new` expressions
    /// other than `new Array()`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// function add(item) {
    ///     return items.push(item);
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const add = (item) => items.push(item);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const length = items.unshift(item);
    /// ```
    ///
    /// An arrow function without braces returns its body, even when the caller ignores the result:
    ///
    /// ```js,expect_diagnostic
    /// source.forEach((item) => target.push(item));
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// function add(item) {
    ///     items.push(item);
    ///     return;
    /// }
    ///
    /// const addItem = (item) => {
    ///     items.push(item);
    /// };
    ///
    /// items.unshift(item);
    /// const length = items.length;
    ///
    /// function getNextLength(item) {
    ///     items.push(item);
    ///     return items.length;
    /// }
    ///
    /// target.push(...source);
    ///
    /// void items.push(item);
    /// ```
    ///
    pub NoReturnArrayPush {
        version: "2.6.4",
        name: "noReturnArrayPush",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("no-return-array-push").same()],
        recommended: true,
        severity: Severity::Warning,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoReturnArrayPush {
    type Query = Ast<JsCallExpression>;
    /// The outermost expression around the call that has the same value:
    /// the call itself, or the parentheses and TypeScript type assertions that wrap it.
    type State = AnyJsExpression;
    type Signals = Option<Self::State>;
    type Options = NoReturnArrayPushOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        let member = array_method_member(call)?;
        let method = member.member().ok()?;
        let method = method.as_js_name()?.value_token().ok()?;
        let method = method.text_trimmed();
        if !matches!(method, "push" | "unshift") || call.arguments().ok()?.args().is_empty() {
            return None;
        }
        if method == "push" && is_ignored_push_receiver(&member.object().ok()?) {
            return None;
        }
        if is_known_non_array(&member.object().ok()?.omit_parentheses()) {
            return None;
        }

        let outer = AnyJsExpression::from(call.clone())
            .syntax()
            .ancestors()
            .skip(1)
            .take_while(|node| VALUE_PRESERVING_WRAPPERS.matches(node.kind()))
            .last()
            .map_or_else(
                || Some(AnyJsExpression::from(call.clone())),
                AnyJsExpression::cast,
            )?;
        let parent = outer.syntax().parent()?;
        if JsExpressionStatement::can_cast(parent.kind())
            || JsUnaryExpression::cast_ref(&parent).is_some_and(|unary| unary.is_void() == Ok(true))
            || is_object_of_member_access(&outer)
        {
            return None;
        }
        Some(outer)
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let method = array_method_member(ctx.query())?.member().ok()?;
        let method_name = method.as_js_name()?.value_token().ok()?;
        let method_name = method_name.text_trimmed();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                method.range(),
                markup! {
                    "The return value of "<Emphasis>{method_name}"()"</Emphasis>" is used here."
                },
            )
            .note(markup! {
                <Emphasis>{method_name}"()"</Emphasis>" returns the new length of the array, not the added item or the array. Using this length is usually a mistake."
            })
            .note(markup! {
                "Call "<Emphasis>{method_name}"()"</Emphasis>" as its own statement. If you need the new length, read the "<Emphasis>"length"</Emphasis>" property of the array afterward."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, outer: &Self::State) -> Option<JsRuleAction> {
        let call = ctx.query();
        // Type assertions such as `return items.push(item) as number` are left alone,
        // because removing them can change the type that TypeScript infers.
        if outer.clone().omit_parentheses().syntax() != call.syntax() {
            return None;
        }
        let return_statement = JsReturnStatement::cast(outer.syntax().parent()?)?;
        let statement_list = JsStatementList::cast(return_statement.syntax().parent()?)?;
        let first_token = call.syntax().first_token()?;
        // The call becomes the start of a statement. Starting a statement with one of these
        // tokens would either join it with the previous statement or turn it into a declaration.
        if UNSAFE_STATEMENT_STARTS.contains(first_token.kind()) {
            return None;
        }
        let return_token = return_statement.return_token().ok()?;
        if has_comments_outside_call(&return_statement, call.range(), &first_token) {
            return None;
        }

        let semicolon = return_statement.semicolon_token();
        let mut expression = AnyJsExpression::from(call.clone())
            .with_leading_trivia_pieces(return_token.leading_trivia().pieces())?;
        let mut new_return_token =
            make::token(T![return]).with_leading_trivia([(TriviaPieceKind::Whitespace, " ")]);
        if semicolon.is_none() {
            // Without a semicolon, the trailing trivia of the call ends the statement.
            // It moves after the new `return` so that a line comment can't swallow the `return`.
            let trailing = call.syntax().last_token()?.trailing_trivia().pieces();
            new_return_token = new_return_token.with_trailing_trivia_pieces(trailing);
            expression = expression.with_trailing_trivia_pieces([])?;
        }
        let expression_statement = make::js_expression_statement(expression)
            .with_semicolon_token(make::token(T![;]))
            .build();
        let mut new_return = make::js_return_statement(new_return_token);
        if let Some(semicolon) = semicolon {
            new_return = new_return.with_semicolon_token(semicolon);
        }
        let new_return = new_return.build();

        let mut statements = Vec::with_capacity(statement_list.len() + 1);
        for statement in statement_list.iter() {
            if statement.syntax() == return_statement.syntax() {
                statements.push(AnyJsStatement::from(expression_statement.clone()));
                statements.push(AnyJsStatement::from(new_return.clone()));
            } else {
                statements.push(statement);
            }
        }

        let mut mutation = ctx.root().begin();
        mutation.replace_node(statement_list, make::js_statement_list(statements));
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Move the call before the "<Emphasis>"return"</Emphasis>" statement." }
                .to_owned(),
            mutation,
        ))
    }
}

/// Expressions that have the same value as the expression they wrap.
const VALUE_PRESERVING_WRAPPERS: SyntaxKindSet<JsLanguage> = JsParenthesizedExpression::KIND_SET
    .union(TsAsExpression::KIND_SET)
    .union(TsSatisfiesExpression::KIND_SET)
    .union(TsNonNullAssertionExpression::KIND_SET)
    .union(TsTypeAssertionExpression::KIND_SET);

/// Receivers whose syntax shows that they aren't arrays.
const NON_ARRAY_RECEIVERS: SyntaxKindSet<JsLanguage> = JsArrowFunctionExpression::KIND_SET
    .union(JsClassExpression::KIND_SET)
    .union(JsFunctionExpression::KIND_SET)
    .union(JsObjectExpression::KIND_SET);

/// Tokens that can't start an expression statement without changing how the code is parsed.
const UNSAFE_STATEMENT_STARTS: TokenSet<JsSyntaxKind> = token_set![
    T!['('],
    T!['['],
    T!['{'],
    T!['`'],
    T![+],
    T![-],
    T![/],
    T![/=],
    T![<],
    T![async],
    T![class],
    T![function],
    T![let],
    JsSyntaxKind::JS_REGEX_LITERAL,
];

/// Returns the member expression of a call like `items.push(item)` or `items?.push?.(item)`.
fn array_method_member(call: &JsCallExpression) -> Option<JsStaticMemberExpression> {
    match call.callee().ok()?.omit_parentheses() {
        AnyJsExpression::JsStaticMemberExpression(member) => Some(member),
        _ => None,
    }
}

/// Returns `true` if `receiver` is one of the objects with a `push()` method that commonly
/// isn't an array: `stream`, `router`, `this`, `this.stream`, `this.router`, `this.$router`,
/// `process.stdin`, `process.stdout`, or `process.stderr`.
fn is_ignored_push_receiver(receiver: &AnyJsExpression) -> bool {
    match receiver.clone().omit_parentheses() {
        AnyJsExpression::JsThisExpression(_) => true,
        AnyJsExpression::JsIdentifierExpression(identifier) => identifier
            .name()
            .and_then(|name| name.value_token())
            .is_ok_and(|name| matches!(name.text_trimmed(), "stream" | "router")),
        AnyJsExpression::JsStaticMemberExpression(member) => {
            let Some(name) = member
                .member()
                .ok()
                .and_then(|member| member.as_js_name().and_then(|name| name.value_token().ok()))
            else {
                return false;
            };
            match member.object().map(AnyJsExpression::omit_parentheses) {
                Ok(AnyJsExpression::JsThisExpression(_)) => {
                    matches!(name.text_trimmed(), "stream" | "router" | "$router")
                }
                Ok(AnyJsExpression::JsIdentifierExpression(object)) => {
                    object
                        .name()
                        .and_then(|object| object.value_token())
                        .is_ok_and(|object| object.text_trimmed() == "process")
                        && matches!(name.text_trimmed(), "stdin" | "stdout" | "stderr")
                }
                _ => false,
            }
        }
        _ => false,
    }
}

/// Returns `true` if the syntax of `receiver` shows that it isn't an array,
/// such as an object literal or `new Foo()`.
fn is_known_non_array(receiver: &AnyJsExpression) -> bool {
    if NON_ARRAY_RECEIVERS.matches(receiver.syntax().kind()) {
        return true;
    }
    let new_expression = match receiver {
        // A tagged template can return anything, but a plain template is always a string.
        AnyJsExpression::JsTemplateExpression(template) => return template.tag().is_none(),
        AnyJsExpression::JsNewExpression(new_expression) => new_expression,
        _ => return false,
    };
    // `new Array()` creates an array.
    !new_expression
        .callee()
        .ok()
        .and_then(|callee| {
            callee
                .omit_parentheses()
                .as_js_identifier_expression()?
                .name()
                .ok()
        })
        .and_then(|name| name.value_token().ok())
        .is_some_and(|name| name.text_trimmed() == "Array")
}

/// Returns `true` if the parent of `expression` reads a property of it, as in `expression.foo` or `expression[0]`.
fn is_object_of_member_access(expression: &AnyJsExpression) -> bool {
    let Some(parent) = expression.syntax().parent() else {
        return false;
    };
    if let Some(member) = JsStaticMemberExpression::cast_ref(&parent) {
        return member
            .object()
            .is_ok_and(|object| object.syntax() == expression.syntax());
    }
    JsComputedMemberExpression::cast_ref(&parent)
        .and_then(|member| member.object().ok())
        .is_some_and(|object| object.syntax() == expression.syntax())
}

/// Returns `true` if `return_statement` has comments that the fix can't keep: comments after
/// the `return` keyword or around the parentheses of the argument. Comments before `return`
/// and inside the call are kept.
fn has_comments_outside_call(
    return_statement: &JsReturnStatement,
    call_range: TextRange,
    call_first_token: &JsSyntaxToken,
) -> bool {
    let semicolon = return_statement.semicolon_token();
    return_statement
        .syntax()
        .descendants_tokens(Direction::Next)
        .any(|token| {
            if &token == call_first_token {
                return token.has_leading_comments();
            }
            if call_range.contains_range(token.text_trimmed_range()) {
                return false;
            }
            if semicolon.as_ref() == Some(&token) {
                return token.has_leading_comments();
            }
            token.has_trailing_comments()
                || (token.kind() != T![return] && token.has_leading_comments())
        })
}
