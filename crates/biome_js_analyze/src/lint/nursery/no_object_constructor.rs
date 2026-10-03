use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, JsDirective, JsExpressionStatement, JsLanguage, JsModuleItemList,
    JsNewOrCallExpression, JsPostUpdateExpression, JsStatementList, JsSyntaxKind, JsSyntaxToken,
    JsYieldExpression, T,
};
use biome_parser::{TokenSet, token_set};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, Direction, SyntaxKindSet};
use biome_rule_options::no_object_constructor::NoObjectConstructorOptions;

use crate::{JsRuleAction, services::semantic::Semantic};

declare_lint_rule! {
    /// Disallow calling `Object` without arguments to create an empty object.
    ///
    /// `Object()` and `new Object()` without arguments both create an empty object, the same as `{}`.
    /// Writing `{}` is shorter and clearer, and it always creates a plain object,
    /// even if other code replaces the global `Object`.
    ///
    /// Calls with an argument, such as `Object(value)`, are allowed because they convert `value` to an object.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const obj = Object();
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const obj = new Object();
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const obj = new Object;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const obj = {};
    /// const wrapped = Object(value);
    /// const alsoWrapped = new Object(value);
    /// // Here, `Object` is a parameter, not the built-in `Object`.
    /// const createObject = (Object) => new Object();
    /// ```
    ///
    /// ## See Also
    ///
    /// - To use `[]` instead of the `Array` constructor, see [`useArrayLiterals`](https://biomejs.dev/linter/rules/use-array-literals/).
    /// - To require `new` when calling built-in constructors such as `Object`, see [`useConsistentBuiltinInstantiation`](https://biomejs.dev/linter/rules/use-consistent-builtin-instantiation/).
    ///   This rule reports both `Object()` and `new Object()` when they have no arguments.
    ///
    pub NoObjectConstructor {
        version: "next",
        name: "noObjectConstructor",
        language: "js",
        sources: &[
            RuleSource::Eslint("no-object-constructor").same(),
            RuleSource::Eslint("no-new-object").inspired(),
        ],
        recommended: false,
        severity: Severity::Information,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for NoObjectConstructor {
    type Query = Semantic<JsNewOrCallExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoObjectConstructorOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if node
            .arguments()
            .is_some_and(|arguments| !arguments.args().is_empty())
        {
            return None;
        }
        let callee = node.callee().ok()?.omit_parentheses();
        let AnyJsExpression::JsIdentifierExpression(callee) = callee else {
            return None;
        };
        let reference = callee.name().ok()?;
        if reference.name().ok()?.text() != "Object" || ctx.model().binding(&reference).is_some() {
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
                    "Calling "<Emphasis>"Object"</Emphasis>" without arguments creates an empty object."
                },
            )
            .note(markup! {
                "Writing "<Emphasis>"{}"</Emphasis>" is shorter, and it always creates a plain object, even if other code replaces the global "<Emphasis>"Object"</Emphasis>"."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        if has_inner_comments(node) {
            return None;
        }

        let first_token = node.syntax().first_token()?;
        let statement = node
            .syntax()
            .ancestors()
            .find_map(JsExpressionStatement::cast)
            .filter(|statement| statement.syntax().first_token().as_ref() == Some(&first_token));
        // `{` at the start of a statement or an arrow function body is parsed as a block.
        let needs_parentheses = statement.is_some()
            || first_token
                .prev_token()
                .is_some_and(|token| token.kind() == T![=>]);

        let mut replacement: AnyJsExpression = make::js_object_expression(
            make::token(T!['{']),
            make::js_object_member_list([], []),
            make::token(T!['}']),
        )
        .into();
        if needs_parentheses {
            replacement = make::parenthesized(replacement).into();
        }

        let mut mutation = ctx.root().begin();
        if let Some(statement) = statement.filter(needs_preceding_semicolon) {
            // `foo\n({})` is parsed as `foo({})`, so we insert an empty statement
            // between the previous statement and the replaced one.
            let leading_trivia = first_token.leading_trivia().pieces();
            let trailing_trivia = node.syntax().last_trailing_trivia()?.pieces();
            let replacement = replacement.append_trivia_pieces(trailing_trivia)?;
            let new_statement = statement.syntax().clone().replace_child(
                node.syntax().clone().into(),
                replacement.into_syntax().into(),
            )?;
            let semicolon = make::js_empty_statement(
                make::token(T![;]).with_leading_trivia_pieces(leading_trivia),
            );
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
        } else {
            mutation.replace_node(AnyJsExpression::from(node.clone()), replacement);
        }

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use an object literal." }.to_owned(),
            mutation,
        ))
    }
}

/// Returns `true` if `node` contains comments that would be lost by replacing it.
fn has_inner_comments(node: &JsNewOrCallExpression) -> bool {
    let syntax = node.syntax();
    let first = syntax.first_token();
    let last = syntax.last_token();
    syntax.descendants_tokens(Direction::Next).any(|token| {
        (token.has_leading_comments() && first.as_ref() != Some(&token))
            || (token.has_trailing_comments() && last.as_ref() != Some(&token))
    })
}

/// Returns `true` if a parenthesized expression at the start of `statement`
/// would be parsed as a continuation of the previous statement,
/// e.g. `foo\n({})` is parsed as `foo({})`.
fn needs_preceding_semicolon(statement: &JsExpressionStatement) -> bool {
    let Some(parent) = statement.syntax().parent() else {
        return false;
    };
    // The bodies of control flow statements and labels are preceded by a token
    // that can't be continued by `(`, such as `)`, `else`, `do`, or `:`.
    if !STATEMENT_LISTS.matches(parent.kind()) {
        return false;
    }
    let Some(previous) = statement
        .syntax()
        .first_token()
        .and_then(|token| token.prev_token())
    else {
        return false;
    };
    is_continued_by_parenthesis(&previous)
}

/// Returns `true` if `token` ends an expression that would be called by a following `(`.
fn is_continued_by_parenthesis(token: &JsSyntaxToken) -> bool {
    if STATEMENT_LIST_STARTS.contains(token.kind()) {
        return false;
    }
    let Some(parent) = token.parent() else {
        return false;
    };
    // `token` ends the previous statement, so these parents mean it's a postfix `++`/`--`
    // or a `yield` without argument. A semicolon is always inserted after them.
    if UNCALLABLE_ENDS.matches(parent.kind()) {
        return false;
    }
    // An arrow function with a block body can't be called without parentheses.
    if parent.kind() == JsSyntaxKind::JS_FUNCTION_BODY
        && parent
            .parent()
            .is_some_and(|it| it.kind() == JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION)
    {
        return false;
    }
    parent
        .ancestors()
        .take_while(|node| !STATEMENT_LISTS.matches(node.kind()))
        .any(|node| CALLABLE_ENDS.matches(node.kind()))
}

/// Tokens after which a statement list starts: `;`, `{`, or the `:` of a `case` or `default` clause.
const STATEMENT_LIST_STARTS: TokenSet<JsSyntaxKind> = token_set![T![;], T!['{'], T![:]];

const UNCALLABLE_ENDS: SyntaxKindSet<JsLanguage> =
    JsPostUpdateExpression::KIND_SET.union(JsYieldExpression::KIND_SET);

const STATEMENT_LISTS: SyntaxKindSet<JsLanguage> =
    JsStatementList::KIND_SET.union(JsModuleItemList::KIND_SET);

/// Nodes that are called by a following `(` when they end a statement without a semicolon.
const CALLABLE_ENDS: SyntaxKindSet<JsLanguage> =
    AnyJsExpression::KIND_SET.union(JsDirective::KIND_SET);
