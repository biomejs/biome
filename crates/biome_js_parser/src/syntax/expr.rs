//! Expressions, these include `this`, identifiers, arrays, objects,
//! binary expressions, unary expressions, and more.
//!
//! See the [ECMAScript spec](https://www.ecma-international.org/ecma-262/5.1/#sec-11).

use std::ops::{BitOr, BitOrAssign, Sub};

use super::metavariable::{is_at_metavariable, parse_metavariable};
use super::typescript::*;
use crate::lexer::{JsLexContext, JsReLexContext};
use crate::parser::rewrite_parser::{RewriteMarker, RewriteParser};
use crate::parser::{JsParserCheckpoint, RecoveryResult};
use crate::prelude::*;
use crate::rewrite::rewrite_events;
use crate::rewrite::RewriteParseEvents;
use crate::syntax::assignment::parse_assignment;
use crate::syntax::assignment::AssignmentExprPrecedence;
use crate::syntax::assignment::{expression_to_assignment, expression_to_assignment_pattern};
use crate::syntax::class::{parse_class_expression, parse_decorators};
use crate::syntax::function::{
    is_at_async_function, parse_arrow_function_expression, parse_function_expression, LineBreak,
};
use crate::syntax::js_parse_error;
use crate::syntax::js_parse_error::{decorators_not_allowed, expected_simple_assignment_target};
use crate::syntax::js_parse_error::{
    expected_expression, expected_identifier, invalid_assignment_error,
    private_names_only_allowed_on_left_side_of_in_expression,
};
use crate::syntax::jsx::parse_jsx_tag_expression;
use crate::syntax::object::parse_object_expression;
use crate::syntax::stmt::{is_semi, STMT_RECOVERY_SET};
use crate::syntax::typescript::ts_parse_error::{expected_ts_type, ts_only_syntax_error};
use crate::JsSyntaxFeature::{Jsx, StrictMode, TypeScript};
use crate::ParsedSyntax::{Absent, Present};
use crate::{syntax, JsParser, ParseRecoveryTokenSet, ParsedSyntax};
use biome_js_syntax::{JsSyntaxKind::*, *};
use biome_parser::diagnostic::expected_token;
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::ParserProgress;
use enumflags2::{bitflags, make_bitflags, BitFlags};

pub const EXPR_RECOVERY_SET: TokenSet<JsSyntaxKind> =
    token_set![VAR_KW, R_PAREN, L_PAREN, L_BRACK, R_BRACK];

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) struct ExpressionContext(ExpressionContextFlags);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[bitflags]
#[repr(u8)]
enum ExpressionContextFlag {
    IncludeIn = 1 << 0,
    AllowObjectExpression = 1 << 1,
    InDecorator = 1 << 2,
    AllowTSTypeAssertion = 1 << 3,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct ExpressionContextFlags(BitFlags<ExpressionContextFlag>);

impl ExpressionContextFlags {
    /// Whether `in` should be counted in a binary expression.
    /// This is for `for...in` statements to prevent ambiguity.
    /// Corresponds to `[+In]` in the EcmaScript spec if true
    const INCLUDE_IN: Self = Self(make_bitflags!(ExpressionContextFlag::{IncludeIn}));

    /// If false, object expressions are not allowed to be parsed
    /// inside an expression.
    ///
    /// Also applies for object patterns
    const ALLOW_OBJECT_EXPRESSION: Self =
        Self(make_bitflags!(ExpressionContextFlag::{AllowObjectExpression}));

    /// If `true` then, don't parse computed member expressions because they can as well indicate
    /// the start of a computed class member.
    const IN_DECORATOR: Self = Self(make_bitflags!(ExpressionContextFlag::{InDecorator}));

    /// If `true` allows a typescript type assertion.
    /// Currently disabled on "new" expressions.
    const ALLOW_TS_TYPE_ASSERTION: Self =
        Self(make_bitflags!(ExpressionContextFlag::{AllowTSTypeAssertion}));

    pub fn contains(&self, other: impl Into<ExpressionContextFlags>) -> bool {
        self.0.contains(other.into().0)
    }
}

impl BitOr for ExpressionContextFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        ExpressionContextFlags(self.0 | rhs.0)
    }
}

impl BitOrAssign for ExpressionContextFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl Sub for ExpressionContextFlags {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 & !rhs.0)
    }
}

impl ExpressionContext {
    pub(crate) fn and_include_in(self, include: bool) -> Self {
        self.and(ExpressionContextFlags::INCLUDE_IN, include)
    }

    pub(crate) fn and_object_expression_allowed(self, allowed: bool) -> Self {
        self.and(ExpressionContextFlags::ALLOW_OBJECT_EXPRESSION, allowed)
    }

    pub(crate) fn and_in_decorator(self, in_decorator: bool) -> Self {
        self.and(ExpressionContextFlags::IN_DECORATOR, in_decorator)
    }

    pub(crate) fn and_ts_type_assertion_allowed(self, allowed: bool) -> Self {
        self.and(ExpressionContextFlags::ALLOW_TS_TYPE_ASSERTION, allowed)
    }

    /// Returns true if object expressions or object patterns are valid in this context
    pub(crate) fn is_object_expression_allowed(&self) -> bool {
        self.0
            .contains(ExpressionContextFlags::ALLOW_OBJECT_EXPRESSION)
    }

    /// Returns `true` if the expression parsing includes binary in expressions.
    pub(crate) fn is_in_included(&self) -> bool {
        self.0.contains(ExpressionContextFlags::INCLUDE_IN)
    }

    /// Returns `true` if currently parsing a decorator expression `@<expr>`.
    pub(crate) fn is_in_decorator(&self) -> bool {
        self.0.contains(ExpressionContextFlags::IN_DECORATOR)
    }

    /// Adds the `flag` if `set` is `true`, otherwise removes the `flag`
    fn and(self, flag: ExpressionContextFlags, set: bool) -> Self {
        ExpressionContext(if set { self.0 | flag } else { self.0 - flag })
    }
}

/// Sets the default flags for a context that parses a new root expression (for example, the condition of an if statement)
/// or sub-expression of another expression (the alternate branch of a condition expression).
impl Default for ExpressionContext {
    fn default() -> Self {
        ExpressionContext(
            ExpressionContextFlags::INCLUDE_IN
                | ExpressionContextFlags::ALLOW_OBJECT_EXPRESSION
                | ExpressionContextFlags::ALLOW_TS_TYPE_ASSERTION,
        )
    }
}

/// Parses an expression or recovers to the point of where the next statement starts
pub(crate) fn parse_expression_or_recover_to_next_statement(
    p: &mut JsParser,
    assign: bool,
    context: ExpressionContext,
) -> RecoveryResult {
    let func = if assign {
        syntax::expr::parse_assignment_expression_or_higher
    } else {
        syntax::expr::parse_expression
    };

    func(p, context).or_recover_with_token_set(
        p,
        &ParseRecoveryTokenSet::new(
            JsSyntaxKind::JS_BOGUS_EXPRESSION,
            STMT_RECOVERY_SET.union(token_set![T!['}']]),
        )
        .enable_recovery_on_line_break(),
        expected_expression,
    )
}

/// A literal expression.
///
/// `TRUE | FALSE | NUMBER | STRING | NULL`
// test js literals
// 5
// true
// false
// 5n
// "foo"
// 'bar'
// null
// 0, 0.0, 0n, 0e00
// "test\
// new-line";
// /^[يفمئامئ‍ئاسۆند]/i; //regex with unicode
// /[\p{Control}--[\t\n]]/v;
// /\’/; // regex with escaped non-ascii chars (issue #1941)
// test_err js literals
// 00, 012, 08, 091, 0789 // parser errors
// 01n, 0_0, 01.2 // lexer errors
// "test
// continues" // unterminated string literal
// test_err js regex
// /[\p{Control}--[\t\n]]/vv;
// /[\p{Control}--[\t\n]]/uv;
pub(super) fn parse_literal_expression(p: &mut JsParser) -> ParsedSyntax {
    let literal_kind = match p.cur() {
        JsSyntaxKind::JS_NUMBER_LITERAL => {
            return parse_number_literal_expression(p)
                .or_else(|| parse_big_int_literal_expression(p));
        }
        JsSyntaxKind::JS_STRING_LITERAL => JsSyntaxKind::JS_STRING_LITERAL_EXPRESSION,
        JsSyntaxKind::NULL_KW => JsSyntaxKind::JS_NULL_LITERAL_EXPRESSION,
        JsSyntaxKind::TRUE_KW | JsSyntaxKind::FALSE_KW => {
            JsSyntaxKind::JS_BOOLEAN_LITERAL_EXPRESSION
        }
        T![/] | T![/=] => {
            if p.re_lex(JsReLexContext::Regex) == JS_REGEX_LITERAL {
                JS_REGEX_LITERAL_EXPRESSION
            } else {
                return Absent;
            }
        }
        _ => return Absent,
    };

    let m = p.start();
    p.bump_any();
    Present(m.complete(p, literal_kind))
}

pub(crate) fn parse_big_int_literal_expression(p: &mut JsParser) -> ParsedSyntax {
    if !p.at(JS_NUMBER_LITERAL) || !p.cur_text().ends_with('n') {
        return Absent;
    }

    let m = p.start();
    p.bump_remap(JsSyntaxKind::JS_BIGINT_LITERAL);
    Present(m.complete(p, JS_BIGINT_LITERAL_EXPRESSION))
}

pub(crate) fn parse_number_literal_expression(p: &mut JsParser) -> ParsedSyntax {
    let cur_src = p.cur_text();
    if !p.at(JS_NUMBER_LITERAL) || cur_src.ends_with('n') {
        return Absent;
    }

    // Forbid legacy octal number in strict mode
    if p.state().strict().is_some()
        && cur_src.starts_with('0')
        && cur_src
            .as_bytes()
            .get(1)
            .is_some_and(|b| b.is_ascii_digit())
    {
        let err_msg = if cur_src.contains(['8', '9']) {
            "Decimals with leading zeros are not allowed in strict mode."
        } else {
            "\"0\"-prefixed octal literals are deprecated; use the \"0o\" prefix instead."
        };
        p.error(p.err_builder(err_msg, p.cur_range()));
    }

    let m = p.start();
    p.bump_any();
    Present(m.complete(p, JS_NUMBER_LITERAL_EXPRESSION))
}

/// Parses an assignment expression or any higher expression
/// https://tc39.es/ecma262/multipage/ecmascript-language-expressions.html#prod-AssignmentExpression
pub(crate) fn parse_assignment_expression_or_higher(
    p: &mut JsParser,
    context: ExpressionContext,
) -> ParsedSyntax {
    let arrow_expression = parse_arrow_function_expression(p);

    if arrow_expression.is_present() {
        return arrow_expression;
    }

    parse_assignment_expression_or_higher_base(p, context)
}

fn parse_assignment_expression_or_higher_base(
    p: &mut JsParser,
    context: ExpressionContext,
) -> ParsedSyntax {
    // test js reparse_yield_as_identifier
    // // SCRIPT
    // function foo() { yield *bar; }
    // function bar() { yield; }
    // function baz() { yield }
    if p.at(T![yield]) && (p.state().in_generator() || is_nth_at_expression(p, 1)) {
        return Present(parse_yield_expression(p, context));
    }

    let checkpoint = p.checkpoint();
    parse_conditional_expr(p, context)
        .and_then(|target| parse_assign_expr_recursive(p, target, checkpoint, context))
}

// test js assign_expr
// foo += bar = b ??= 3;
// foo -= bar;
// (foo = bar);
// [foo, bar] = baz;
// [foo, bar = "default", ...rest] = baz;
// [,,,foo,bar] = baz;
// ({ bar, baz } = {});
// ({ bar: [baz = "baz"], foo = "foo", ...rest } = {});

// test_err js assign_expr_right
// (foo = );

// test_err js assign_expr_left
// ( = foo);

// test js assign_eval_member_or_computed_expr
// eval.foo = 10
// arguments[1] = "baz"
// eval[2] = "Chungking Express"

// test_err js assign_eval_or_arguments
// eval = 0
// eval ??= 2
// eval *= 4
// arguments = "foo"
// arguments ||= "baz"
// ({ eval } = o)
// ({ foo: { eval }}) = o
fn parse_assign_expr_recursive(
    p: &mut JsParser,
    mut target: CompletedMarker,
    checkpoint: JsParserCheckpoint,
    context: ExpressionContext,
) -> ParsedSyntax {
    let assign_operator = p.cur();
    if is_assign_token(assign_operator) {
        let target = if matches!(
            target.kind(p),
            JS_BINARY_EXPRESSION | TS_TYPE_ASSERTION_EXPRESSION
        ) {
            // Special handling for binary expressions and type assertions to avoid having to deal with `a as string = ...`
            // inside of the `ReparseAssignment` implementation because not using parentheses is valid
            // in for heads `for (a as any in []) {}`
            p.error(invalid_assignment_error(p, target.range(p)));
            target.change_kind(p, JS_BOGUS_ASSIGNMENT);
            target
        } else {
            match expression_to_assignment_pattern(p, target, checkpoint) {
                Present(target) => target,
                Absent => return Absent,
            }
        };

        let m = target.precede(p);
        p.expect(assign_operator);

        parse_assignment_expression_or_higher(p, context.and_object_expression_allowed(true))
            .or_add_diagnostic(p, js_parse_error::expected_expression_assignment);
        Present(m.complete(p, JS_ASSIGNMENT_EXPRESSION))
    } else {
        Present(target)
    }
}

fn is_assign_token(kind: JsSyntaxKind) -> bool {
    matches!(
        kind,
        T![=]
            | T![+=]
            | T![-=]
            | T![*=]
            | T![/=]
            | T![%=]
            | T![<<=]
            | T![>>=]
            | T![>>>=]
            | T![&=]
            | T![|=]
            | T![^=]
            | T![&&=]
            | T![||=]
            | T![??=]
            | T![**=]
    )
}

// test js yield_expr
// function *foo() {
//  yield foo;
//  yield* foo;
//  yield;
//  yield
//  yield
// }
fn parse_yield_expression(p: &mut JsParser, context: ExpressionContext) -> CompletedMarker {
    let m = p.start();
    let yield_range = p.cur_range();
    p.expect(T![yield]);

    // test js yield_in_generator_function
    // function* foo() { yield 10; }
    // function* foo() { yield *bar; }
    // function* foo() { yield; }
    if !is_semi(p, 0) && (p.at(T![*]) || is_at_expression(p)) {
        let argument = p.start();
        p.eat(T![*]);
        parse_assignment_expression_or_higher(p, context.and_object_expression_allowed(true)).ok();
        argument.complete(p, JS_YIELD_ARGUMENT);
    }

    let mut yield_expr = m.complete(p, JS_YIELD_EXPRESSION);

    // test_err js yield_at_top_level_module
    // yield 10;

    // test_err js yield_at_top_level_script
    // // SCRIPT
    // yield 10;

    // test_err js yield_in_non_generator_function_script
    // // SCRIPT
    // function foo() { yield bar; }
    // function foo() { yield 10; }

    // test_err js yield_in_non_generator_function_module
    // function foo() { yield; }
    // function foo() { yield foo; }
    // function foo() { yield *foo; }
    if !(p.state().in_generator() && p.state().in_function()) {
        // test_err js yield_expr_in_parameter_initializer
        // function* test(a = yield "test") {}
        // function test2(a = yield "test") {}
        p.error(p.err_builder(
            "`yield` is only allowed within generator functions.",
            yield_range,
        ));
        yield_expr.change_to_bogus(p);
    }

    yield_expr
}

/// A conditional expression such as `foo ? bar : baz`
// test js conditional_expr
// foo ? bar : baz
// foo ? bar : baz ? bar : baz
pub(super) fn parse_conditional_expr(p: &mut JsParser, context: ExpressionContext) -> ParsedSyntax {
    // test_err js conditional_expr_err
    // foo ? bar baz
    // foo ? bar baz ? foo : bar
    // foo ? bar :
    let lhs = parse_binary_or_logical_expression(p, OperatorPrecedence::lowest(), context);

    if p.at(T![?]) {
        lhs.map(|marker| {
            let m = marker.precede(p);
            p.bump(T![?]);

            parse_conditional_expr_consequent(p, ExpressionContext::default())
                .or_add_diagnostic(p, js_parse_error::expected_expression_assignment);

            p.expect(T![:]);

            parse_assignment_expression_or_higher(p, context)
                .or_add_diagnostic(p, js_parse_error::expected_expression_assignment);
            m.complete(p, JS_CONDITIONAL_EXPRESSION)
        })
    } else {
        lhs
    }
}

/// Specialized version of [parse_assignment_expression_or_higher].
/// We need to make sure that on a successful arrow expression parse that
/// the next token is `:`.
// test js arrow_expr_in_alternate
// a ? (b) : a => {};
// test ts ts_arrow_exrp_in_alternate
// a ? (b) : a => {};
// test jsx jsx_arrow_exrp_in_alternate
// bar ? (foo) : (<a>{() => {}}</a>);
fn parse_conditional_expr_consequent(p: &mut JsParser, context: ExpressionContext) -> ParsedSyntax {
    let checkpoint = p.checkpoint();

    let arrow_expression = parse_arrow_function_expression(p);
    if arrow_expression.is_present() && p.at(T![:]) {
        return arrow_expression;
    }

    p.rewind(checkpoint);
    parse_assignment_expression_or_higher_base(p, context)
}

pub(crate) fn is_at_binary_operator(p: &JsParser, context: ExpressionContext) -> bool {
    let cur_kind = p.cur();

    match cur_kind {
        T![in] => context.is_in_included(),
        kind => OperatorPrecedence::try_from_binary_operator(kind).is_some(),
    }
}

/// A binary expression such as `2 + 2` or `foo * bar + 2` or a logical expression 'a || b'
fn parse_binary_or_logical_expression(
    p: &mut JsParser,
    left_precedence: OperatorPrecedence,
    context: ExpressionContext,
) -> ParsedSyntax {
    let mut left = parse_unary_expr(p, context).or_else(|| parse_private_name(p));

    let mut stack: Vec<(OperatorPrecedence, Option<(JsSyntaxKind, Marker)>)> =
        vec![(left_precedence, None)];

    while let Some((mut left_precedence, previous_marker)) = stack.pop() {
        if let Some((expression_kind, m)) = previous_marker {
            left.or_add_diagnostic(p, expected_expression);

            left = Present(m.complete(p, expression_kind));
        }

        // Use a loop to eat all binary expressions with the same precedence.
        // At first, the algorithm makes the impression that it recurse for every right-hand side expression.
        // This is true, but `parse_binary_or_logical_expression` immediately returns if the
        // current operator has the same or a lower precedence than the left-hand side expression. Thus,
        // the algorithm goes at most `count(OperatorPrecedence)` levels deep.
        loop {
            let op = p.re_lex(JsReLexContext::BinaryOperator);

            if (op == T![as] && p.has_preceding_line_break())
                || (op == T![satisfies] && p.has_preceding_line_break())
                || (op == T![in] && !context.is_in_included())
            {
                break;
            }

            // This isn't spec compliant but improves error recovery in case the `}` is missing
            // inside of a JSX attribute expression value or an expression child.
            // Prevents that it parses `</` as less than followed by a RegEx if JSX is enabled and only if
            // there's no whitespace between the two tokens.
            // The downside of this is that `a </test/` will be incorrectly left unparsed. I think this is
            // a worth compromise and compatible with what TypeScript's doing.
            if Jsx.is_supported(p)
                && op == T![<]
                && p.nth_at(1, T![/])
                && !p.source_mut().has_next_preceding_trivia()
            {
                break;
            }

            let new_precedence = match OperatorPrecedence::try_from_binary_operator(op) {
                Some(precedence) => precedence,
                // Not a binary operator
                None => break,
            };

            let stop_at_current_operator = if new_precedence.is_right_to_left() {
                new_precedence < left_precedence
            } else {
                new_precedence <= left_precedence
            };

            if stop_at_current_operator {
                break;
            }

            let op_range = p.cur_range();

            let mut is_bogus = false;
            if let Present(left) = &mut left {
                if op == T![**] && left.kind(p) == JS_UNARY_EXPRESSION {
                    let err = p
					.err_builder(
						"unparenthesized unary expression can't appear on the left-hand side of '**'",
                        left.range(p)
					)
					.with_detail(op_range, "The operation")
					.with_detail(left.range(p), "The left-hand side");

                    p.error(err);
                    is_bogus = true;
                } else if op != T![in] && left.kind(p) == JS_PRIVATE_NAME {
                    p.error(private_names_only_allowed_on_left_side_of_in_expression(
                        p,
                        left.range(p),
                    ));
                    left.change_kind(p, JS_BOGUS_EXPRESSION);
                }
            } else {
                let err = p
                    .err_builder(
                        format!(
                            "Expected an expression for the left hand side of the `{}` operator.",
                            p.text(op_range),
                        ),
                        op_range,
                    )
                    .with_hint("This operator requires a left hand side value");
                p.error(err);
            }

            let m = left.precede(p);
            p.bump(op);

            if op == T![as] {
                parse_ts_type(p, TypeContext::default()).or_add_diagnostic(p, expected_ts_type);
                let mut as_expression = m.complete(p, TS_AS_EXPRESSION);

                if TypeScript.is_unsupported(p) {
                    p.error(ts_only_syntax_error(
                        p,
                        "'as' expression",
                        as_expression.range(p),
                    ));
                    as_expression.change_to_bogus(p);
                }
                left = Present(as_expression);
                continue;
            }

            if op == T![satisfies] {
                parse_ts_type(p, TypeContext::default()).or_add_diagnostic(p, expected_ts_type);
                let mut satisfies_expression = m.complete(p, TS_SATISFIES_EXPRESSION);

                if TypeScript.is_unsupported(p) {
                    p.error(ts_only_syntax_error(
                        p,
                        "'satisfies' expression",
                        satisfies_expression.range(p),
                    ));
                    satisfies_expression.change_to_bogus(p);
                }
                left = Present(satisfies_expression);
                continue;
            }

            let expression_kind = if is_bogus {
                JS_BOGUS_EXPRESSION
            } else {
                match op {
                    T![??] | T![||] | T![&&] => JS_LOGICAL_EXPRESSION,
                    T![instanceof] => JS_INSTANCEOF_EXPRESSION,
                    T![in] => JS_IN_EXPRESSION,
                    _ => JS_BINARY_EXPRESSION,
                }
            };

            stack.push((left_precedence, Some((expression_kind, m))));

            left_precedence = new_precedence;
            left = parse_unary_expr(p, context).or_else(|| parse_private_name(p));
        }

        if let Present(left) = &mut left {
            // Left at this point becomes the right-hand side of a binary expression
            // or is a standalone expression. Private names aren't allowed as standalone expressions
            // nor on the right-hand side
            if left.kind(p) == JS_PRIVATE_NAME {
                left.change_kind(p, JS_BOGUS_EXPRESSION);
                p.error(private_names_only_allowed_on_left_side_of_in_expression(
                    p,
                    left.range(p),
                ));
            }
        }
    }

    left
}

/// A member or new expression with subscripts. e.g. `new foo`, `new Foo()`, `foo`, or `foo().bar[5]`
// test js new_exprs
// new Foo()
// new foo;
// new.target
// new new new new Foo();
// new Foo(bar, baz, 6 + 6, foo[bar] + ((foo) => {}) * foo?.bar)
// test_err js new_exprs
// new;
fn parse_member_expression_or_higher(p: &mut JsParser, context: ExpressionContext) -> ParsedSyntax {
    parse_primary_expression(p, context)
        .map(|lhs| parse_member_expression_rest(p, lhs, context, true, &mut false))
}

// test_err js subscripts_err
// foo()?.baz[].;
// BAR`b
fn parse_member_expression_rest(
    p: &mut JsParser,
    lhs: CompletedMarker,
    context: ExpressionContext,
    allow_optional_chain: bool,
    in_optional_chain: &mut bool,
) -> CompletedMarker {
    let mut progress = ParserProgress::default();
    let mut lhs = lhs;

    while !p.at(EOF) {
        progress.assert_progressing(p);
        lhs = match p.cur() {
            T![.] => parse_static_member_expression(p, lhs, T![.]).unwrap(),
            // Don't parse out `[` as a member expression because it may as well be the start of a computed class member
            T!['['] if !context.is_in_decorator() => {
                parse_computed_member_expression(p, lhs, false).unwrap()
            }
            T![?.] if allow_optional_chain => {
                let completed = if p.nth_at(1, T!['[']) {
                    parse_computed_member_expression(p, lhs, true).unwrap()
                } else if is_nth_at_any_name(p, 1) {
                    parse_static_member_expression(p, lhs, T![?.]).unwrap()
                } else if p.nth_at(1, BACKTICK) {
                    let m = lhs.precede(p);
                    p.bump(T![?.]);
                    let template_literal = p.start();
                    parse_template_literal(p, template_literal, true, true);
                    m.complete(p, JS_BOGUS_EXPRESSION)
                } else {
                    // '(' or any other unexpected character
                    break;
                };
                *in_optional_chain = true;
                completed
            }
            T![!] if !p.has_preceding_line_break() => {
                // test ts ts_non_null_assertion_expression
                // let a = { b: {} };
                // a!;
                // function test() {}
                // test()!
                // 	a.b.c!;
                // a!!!!!!;
                let m = lhs.precede(p);
                p.bump(T![!]);

                let mut non_null = m.complete(p, TS_NON_NULL_ASSERTION_EXPRESSION);

                if TypeScript.is_unsupported(p) {
                    non_null.change_to_bogus(p);
                    p.error(ts_only_syntax_error(
                        p,
                        "non-null assertions",
                        non_null.range(p),
                    ));
                }

                non_null
            }
            BACKTICK => {
                // test ts ts_optional_chain_call
                // (<A, B>() => {})?.<A, B>();
                let m = match lhs.kind(p) {
                    TS_INSTANTIATION_EXPRESSION => lhs.undo_completion(p),
                    _ => lhs.precede(p),
                };
                parse_template_literal(p, m, *in_optional_chain, true)
            }
            T![<] | T![<<] => {
                //  only those two possible token in cur position `parse_ts_type_arguments_in_expression` could possibly return a `Present(_)`
                if let Present(_) = parse_ts_type_arguments_in_expression(p, context) {
                    let new_marker = lhs.precede(p);
                    lhs = new_marker.complete(p, JsSyntaxKind::TS_INSTANTIATION_EXPRESSION);
                    continue;
                };
                break;
            }
            _ => {
                break;
            }
        };
    }

    lhs
}

// test_err ts ts_new_operator
// new A<test><test>();

// test ts ts_new_operator
// var c2 = new T<string>;  // Ok
// var x1 = new SS<number>(); // OK
// var x3 = new SS();         // OK
// var x4 = new SS;           // OK
fn parse_new_expr(p: &mut JsParser, context: ExpressionContext) -> ParsedSyntax {
    if !p.at(T![new]) {
        return Absent;
    }

    let m = p.start();
    p.expect(T![new]);

    // new.target
    if p.eat(T![.]) {
        if p.at(T![ident]) && p.cur_text() == "target" {
            p.bump_remap(TARGET);
        } else if is_at_identifier(p) {
            let identifier_range = p.cur_range();
            let name = p.cur_text();
            let error = p
                .err_builder(
                    format!("'{name}' is not a valid meta-property for keyword 'new'."),
                    identifier_range,
                )
                .with_hint("Did you mean 'target'?");

            p.error(error);
            p.bump_remap(T![ident]);
        } else {
            p.error(expected_identifier(p, p.cur_range()));
        }

        return Present(m.complete(p, JS_NEW_TARGET_EXPRESSION));
    }

    if let Some(lhs) = parse_primary_expression(p, context.and_ts_type_assertion_allowed(false))
        .or_add_diagnostic(p, expected_expression)
        .map(|expr| parse_member_expression_rest(p, expr, context, false, &mut false))
    {
        // test_err ts invalid_optional_chain_from_new_expressions
        // new Test<string>?.test();
        // new Test?.test();
        // new A.b?.c()
        // new (A.b)?.c()
        // new (A.b?.()).c()
        // new A.b?.()()
        if p.at(T![?.]) {
            let error = p
                .err_builder("Invalid optional chain from new expression.", p.cur_range())
                .with_hint(format!("Did you mean to call '{}()'?", lhs.text(p)));

            p.error(error);
        }
        if let TS_INSTANTIATION_EXPRESSION = lhs.kind(p) {
            lhs.undo_completion(p).abandon(p)
        };
    }

    // test ts ts_new_with_type_arguments
    // class Test<A, B, C> {}
    // new Test<A, B, C>();

    if p.at(T!['(']) {
        parse_call_arguments(p).unwrap();
    }

    Present(m.complete(p, JS_NEW_EXPRESSION))
}

// test js super_expression
// class Test extends B {
//   constructor() {
//     super();
//   }
//   test() {
//     super.test(a, b);
//     super[1];
//   }
// }
//
// test_err js super_expression_err
// class Test extends B {
//   test() {
//     super();
//     super?.test();
//   }
// }
// super();
fn parse_super_expression(p: &mut JsParser) -> ParsedSyntax {
    if !p.at(T![super]) {
        return Absent;
    }
    let super_marker = p.start();
    p.expect(T![super]);
    let mut super_expression = super_marker.complete(p, JS_SUPER_EXPRESSION);

    if p.at(T![?.]) {
        super_expression.change_kind(p, JS_BOGUS_EXPRESSION);
        p.error(p.err_builder(
            "Super doesn't support optional chaining as super can never be null",
            super_expression.range(p),
        ));
    } else if p.at(T!['(']) && !p.state().in_constructor() {
        p.error(p.err_builder(
            "`super` is only valid inside of a class constructor of a subclass.",
            super_expression.range(p),
        ));
        super_expression.change_kind(p, JS_BOGUS_EXPRESSION);
    }

    match p.cur() {
        T![.] | T!['['] | T!['('] | T![?.] => Present(super_expression),
        _ => parse_static_member_expression(p, super_expression, T![.]),
    }
}

// test js subscripts
// foo`bar`
// foo(bar)(baz)(baz)[bar]

/// A static member expression for accessing a property
// test js static_member_expression
// foo.bar
// foo.await
// foo.yield
// foo.for
// foo?.for
// foo?.bar
// class Test {
//   #bar
//   test(other) {
//     this.#bar;
//     this?.#bar;
//     other.#bar;
//     other?.#bar;
//   }
// }
fn parse_static_member_expression(
    p: &mut JsParser,
    lhs: CompletedMarker,
    operator: JsSyntaxKind,
) -> ParsedSyntax {
    // test ts ts_instantiation_expression_property_access
    // f<b>?.(c);
    // f<b>?.[c];
    // (f<b>).c;
    // (f<b>)?.c;
    // (f<b>)?.[c];
    if lhs.kind(p) == TS_INSTANTIATION_EXPRESSION {
        // test_err ts ts_instantiation_expression_property_access
        // f<b>.c;
        // f<b>?.c;
        // a?.f<c>.d;
        // f<a>.g<b>;
        p.error(p.err_builder(
            "An instantiation expression cannot be followed by a property access.",
            lhs.range(p),
        ).with_hint("You can either wrap the instantiation expression in parentheses, or delete the type arguments."));
    }

    let m = lhs.precede(p);
    p.expect(operator);

    parse_any_name(p).or_add_diagnostic(p, expected_identifier);

    Present(m.complete(p, JS_STATIC_MEMBER_EXPRESSION))
}

pub(super) fn parse_private_name(p: &mut JsParser) -> ParsedSyntax {
    if !p.at(T![#]) {
        return Absent;
    }

    let m = p.start();
    let hash_end = p.cur_range().end();
    p.expect(T![#]);

    if (is_nth_at_identifier_or_keyword(p, 0)) && hash_end != p.cur_range().start() {
        // test_err js private_name_with_space
        // class A {
        // 	# test;
        // }
        p.error(
            p.err_builder(
                "Unexpected space or comment between `#` and identifier",
                hash_end..p.cur_range().start(),
            )
            .with_hint("remove the space here"),
        );
        Present(m.complete(p, JS_BOGUS))
    } else {
        if p.cur().is_keyword() {
            p.bump_remap(T![ident]);
        } else if p.at(T![ident]) {
            p.bump(T![ident]);
        } else {
            p.error(expected_identifier(p, p.cur_range()));
        }
        Present(m.complete(p, JS_PRIVATE_NAME))
    }
}

pub(super) fn parse_any_name(p: &mut JsParser) -> ParsedSyntax {
    match p.cur() {
        T![#] => parse_private_name(p),
        t if t.is_metavariable() => parse_metavariable(p),
        _ => parse_name(p),
    }
}

/// An array expression for property access or indexing, such as `foo[0]` or `foo?.["bar"]`
// test js computed_member_expression
// foo[bar]
// foo[5 + 5]
// foo["bar"]
// foo[bar][baz]
// foo?.[bar]
fn parse_computed_member_expression(
    p: &mut JsParser,
    lhs: CompletedMarker,
    optional_chain: bool,
) -> ParsedSyntax {
    // test_err js bracket_expr_err
    // foo[]
    // foo?.[]
    // foo[
    let m = lhs.precede(p);
    if optional_chain {
        p.expect(T![?.]);
    }

    p.expect(T!['[']);
    // test js computed_member_in
    // for ({}["x" in {}];;) {}
    parse_expression(p, ExpressionContext::default()).or_add_diagnostic(p, expected_expression);

    p.expect(T![']']);

    Present(m.complete(p, JS_COMPUTED_MEMBER_EXPRESSION))
}

/// An identifier name, either an ident or a keyword
pub(super) fn parse_name(p: &mut JsParser) -> ParsedSyntax {
    if is_at_name(p) {
        let m = p.start();
        p.bump_remap(T![ident]);
        Present(m.complete(p, JS_NAME))
    } else {
        Absent
    }
}

/// Arguments to a function.
///
/// `"(" (AssignExpr ",")* ")"`
// test js call_arguments
// function foo(...args) {}
// let a, b, c, d;
// foo(a);
// foo(a, b,);
// foo(a, b, ...c);
// foo(...a, ...b, c, ...d,);
//
// test_err js invalid_arg_list
// function foo(...args) {}
// let a, b, c;
// foo(a,b;
// foo(a,b var;
// foo (,,b);
// foo (a, ...);
fn parse_call_arguments(p: &mut JsParser) -> ParsedSyntax {
    if !p.at(T!['(']) {
        return Absent;
    }

    // test js in_expr_in_arguments
    // function foo() {}
    // for (foo("call" in foo);;) {}

    let m = p.start();
    p.bump(T!['(']);
    let args_list = p.start();
    let mut first = true;
    let mut progress = ParserProgress::default();

    while !p.at(EOF) && !p.at(T![')']) {
        if first {
            first = false;
        } else {
            p.expect(T![,]);
        }

        if p.at(T![')']) {
            break;
        }

        progress.assert_progressing(p);

        let argument = if p.at(T![...]) {
            // already do a check on "..." so it's safe to unwrap
            parse_spread_element(p, ExpressionContext::default())
        } else {
            parse_assignment_expression_or_higher(p, ExpressionContext::default())
        };

        if argument.is_absent() && p.at(T![,]) {
            argument.or_add_diagnostic(p, js_parse_error::expected_expression);
            // missing element
            continue;
        }

        if argument
            .or_recover_with_token_set(
                p,
                &ParseRecoveryTokenSet::new(
                    JS_BOGUS_EXPRESSION,
                    EXPR_RECOVERY_SET.union(token_set!(T![')'], T![;], T![...])),
                )
                .enable_recovery_on_line_break(),
                js_parse_error::expected_expression,
            )
            .is_err()
        {
            break;
        }
    }

    args_list.complete(p, JS_CALL_ARGUMENT_LIST);
    p.expect(T![')']);
    Present(m.complete(p, JS_CALL_ARGUMENTS))
}

// test js parenthesized_sequence_expression
// (a, b);
// (a, b, c);
// (a, b, c, d, e, f);
// (a, b, c, d, e, f)
// (a, b, c)

// test_err js incomplete_parenthesized_sequence_expression
// (a,;
// (a, b, c;

// test js js_parenthesized_expression
// ((foo))
// (foo)

fn parse_parenthesized_expression(p: &mut JsParser) -> ParsedSyntax {
    if !p.at(T!['(']) {
        return Absent;
    }

    let m = p.start();
    p.bump(T!['(']);

    // test js for_with_in_in_parenthesized_expression
    // for((true,"selectionStart"in true);;) {}
    if p.at(T![')']) {
        // test_err js empty_parenthesized_expression
        // ();
        p.error(
            p.err_builder(
                "Parenthesized expression didnt contain anything",
                p.cur_range(),
            )
            .with_hint("Expected an expression here"),
        );
    } else {
        let first = parse_assignment_expression_or_higher(p, ExpressionContext::default());

        if p.at(T![,]) {
            parse_sequence_expression_recursive(p, first, ExpressionContext::default())
                .or_add_diagnostic(p, expected_expression);
        }
    }

    p.expect(T![')']);
    Present(m.complete(p, JS_PARENTHESIZED_EXPRESSION))
}

/// A general expression.
pub(crate) fn parse_expression(p: &mut JsParser, context: ExpressionContext) -> ParsedSyntax {
    if is_at_metavariable(p) {
        return parse_metavariable(p);
    }

    let first = parse_assignment_expression_or_higher(p, context);

    if p.at(T![,]) {
        parse_sequence_expression_recursive(p, first, context)
    } else {
        first
    }
}

// test js sequence_expr
// 1, 2, 3, 4, 5

// test_err js sequence_expr
// 1, 2, , 4
fn parse_sequence_expression_recursive(
    p: &mut JsParser,
    left: ParsedSyntax,
    context: ExpressionContext,
) -> ParsedSyntax {
    if !p.at(T![,]) {
        return left;
    }

    let mut left = left;

    while p.at(T![,]) {
        let sequence_expr_marker =
            left.precede_or_add_diagnostic(p, js_parse_error::expected_expression);
        p.bump(T![,]);
        parse_assignment_expression_or_higher(p, context).or_add_diagnostic(p, expected_expression);

        left = Present(sequence_expr_marker.complete(p, JS_SEQUENCE_EXPRESSION))
    }

    left
}

#[inline]
pub(crate) fn is_at_expression(p: &mut JsParser) -> bool {
    is_nth_at_expression(p, 0)
}

pub(crate) fn is_nth_at_expression(p: &mut JsParser, n: usize) -> bool {
    match p.nth(n) {
        T![!]
        | T!['(']
        | T!['[']
        | T!['{']
        | T![++]
        | T![--]
        | T![~]
        | T![+]
        | T![-]
        | T![throw]
        | T![new]
        | T![typeof]
        | T![void]
        | T![delete]
        | T![ident]
        | T![...]
        | T![this]
        | T![yield]
        | T![function]
        | T![class]
        | T![import]
        | T![super]
        | T![#]
        | T![<]
        | T![/]
        | T![/=]
        | BACKTICK
        | TRUE_KW
        | FALSE_KW
        | JS_NUMBER_LITERAL
        | JS_BIGINT_LITERAL
        | JS_STRING_LITERAL
        | NULL_KW => true,
        t => t.is_contextual_keyword() || t.is_future_reserved_keyword(),
    }
}

/// A primary expression such as a literal, an object, an array, or `this`.
fn parse_primary_expression(p: &mut JsParser, context: ExpressionContext) -> ParsedSyntax {
    let parsed_literal_expression = parse_literal_expression(p);
    if parsed_literal_expression.is_present() {
        return parsed_literal_expression;
    }

    let complete = match p.cur() {
        t if t.is_metavariable() => return parse_metavariable(p),
        T![this] => {
            // test js this_expr
            // this
            // this.foo
            let m = p.start();
            p.expect(T![this]);
            m.complete(p, JS_THIS_EXPRESSION)
        }
        T![@] => {
            let decorator_list = parse_decorators(p);

            return match p.cur() {
                T![class] => {
                    // test js decorator_expression_class
                    // let a = @decorator class {};
                    // let b = @first @second class foo {
                    //  constructor() {}
                    // }
                    parse_class_expression(p, decorator_list)
                }
                _ => {
                    // test_err js decorator_expression_class
                    // let a = @decorator () => {};
                    // let b = @first @second function foo() {}
                    // let a = @decorator ( () => {} )
                    decorator_list
                        .add_diagnostic_if_present(p, decorators_not_allowed)
                        .map(|mut marker| {
                            marker.change_kind(p, JS_BOGUS_EXPRESSION);
                            marker
                        });
                    parse_assignment_expression_or_higher(p, context)
                }
            };
        }
        T![class] => {
            // test js class_expr
            // let a = class {};
            // let b = class foo {
            //  constructor() {}
            // }
            // foo[class {}]
            parse_class_expression(p, Absent).unwrap()
        }
        // test js async_ident
        // let a = async;
        T![async] if is_at_async_function(p, LineBreak::DoCheck) => {
            // test js async_function_expr
            // let a = async function() {};
            // let b = async function foo() {};
            parse_function_expression(p).unwrap()
        }
        T![function] => {
            // test js function_expr
            // let a = function() {}
            // let b = function foo() {}

            parse_function_expression(p).unwrap()
        }
        // test js grouping_expr
        // ((foo))
        // (foo)
        T!['('] => parse_parenthesized_expression(p).unwrap(),
        T!['['] => parse_array_expr(p).unwrap(),
        T!['{'] if context.is_object_expression_allowed() => parse_object_expression(p).unwrap(),

        // test_err js import_keyword_in_expression_position
        // let a = import;
        T![import] if matches!(p.nth(1), T![.] | T!['(']) => {
            let m = p.start();
            p.bump_any();

            // test js import_meta
            // import.meta
            if p.eat(T![.]) {
                // test_err js import_no_meta
                // import.foo
                // import.metaa
                if p.at(T![ident]) && p.text(p.cur_range()) == "meta" {
                    p.bump_remap(META);
                    m.complete(p, JS_IMPORT_META_EXPRESSION)
                } else if p.at(T![ident]) {
                    let err = p.err_builder(
                        format!(
                            "Expected `meta` following an import keyword, but found `{}`",
                            p.text(p.cur_range())
                        ),
                        p.cur_range(),
                    );

                    p.err_and_bump(err, JS_BOGUS);
                    m.complete(p, JS_IMPORT_META_EXPRESSION)
                } else {
                    let err = p.err_builder(
                        "Expected `meta` following an import keyword, but found none",
                        p.cur_range(),
                    );

                    p.error(err);
                    m.complete(p, JS_BOGUS)
                }
            } else {
                // test js import_call
                // import("foo")
                // import("foo", { assert: { type: 'json' } })
                // import("foo", { with: { 'resolution-mode': 'import' } })

                // test_err js import_invalid_args
                // import()
                // import(...["foo"])
                // import("foo", { assert: { type: 'json' } }, "bar")
                // import("foo", { with: { type: 'json' } }, "bar")
                let args = p.start();
                p.bump(T!['(']);
                let args_list = p.start();

                let mut progress = ParserProgress::default();
                let mut error_range_start = p.cur_range().start();
                let mut args_count = 0;

                while !p.at(EOF) && !p.at(T![')']) {
                    progress.assert_progressing(p);
                    args_count += 1;

                    if args_count == 3 {
                        error_range_start = p.cur_range().start();
                    }

                    if p.at(T![...]) {
                        parse_spread_element(p, context)
                            .add_diagnostic_if_present(p, |p, range| {
                                p.err_builder("`...` is not allowed in `import()`", range)
                            })
                            .map(|mut marker| {
                                marker.change_to_bogus(p);
                                marker
                            });
                    } else {
                        parse_assignment_expression_or_higher(p, ExpressionContext::default())
                            .or_add_diagnostic(p, js_parse_error::expected_expression_assignment);
                    }

                    if p.at(T![,]) {
                        p.bump_any();
                    } else {
                        break;
                    }
                }

                args_list.complete(p, JS_CALL_ARGUMENT_LIST);
                if args_count == 0 || args_count > 2 {
                    let err = p.err_builder(
                        "`import()` requires exactly one or two arguments. ",
                        error_range_start..p.cur_range().end(),
                    );
                    p.error(err);
                }

                p.expect(T![')']);
                args.complete(p, JS_CALL_ARGUMENTS);
                m.complete(p, JS_IMPORT_CALL_EXPRESSION)
            }
        }
        T![new] => parse_new_expr(p, context).unwrap(),

        BACKTICK => {
            let m = p.start();
            parse_template_literal(p, m, false, false)
        }
        ERROR_TOKEN => {
            let m = p.start();
            p.bump_any();
            m.complete(p, JS_BOGUS)
        }
        T![ident] => parse_identifier_expression(p).unwrap(),
        // test jsx jsx_primary_expression
        // let a = <test>abcd</test>.c;

        // test ts type_assertion_primary_expression
        // let a = <number>undefined;

        // test_err ts ts_type_assertions_not_valid_at_new_expr
        // var test2 = new <any>Test2();

        // test ts ts_type_assertion
        // let a = <number>b;
        T![<] if Jsx.is_supported(p) => return parse_jsx_tag_expression(p),

        // test_err js primary_expr_invalid_recovery
        // let a = \; foo();
        t if t.is_contextual_keyword() || t.is_future_reserved_keyword() => {
            // test js identifier_reference
            // // SCRIPT
            // foo;
            // yield;
            // await;
            parse_identifier_expression(p).unwrap()
        }
        _ => {
            return Absent;
        }
    };

    Present(complete)
}

fn parse_identifier_expression(p: &mut JsParser) -> ParsedSyntax {
    parse_reference_identifier(p)
        .map(|identifier| identifier.precede(p).complete(p, JS_IDENTIFIER_EXPRESSION))
}

// test_err js identifier
// yield;
// await;
pub(crate) fn parse_reference_identifier(p: &mut JsParser) -> ParsedSyntax {
    parse_identifier(p, JS_REFERENCE_IDENTIFIER)
}

pub(crate) fn is_nth_at_reference_identifier(p: &mut JsParser, n: usize) -> bool {
    is_nth_at_identifier(p, n)
}

// test js identifier_loose_mode
// // SCRIPT
// foo;
// yield;
// await;
//
// test js identifier
// foo;
// let accessor = 5;
//
// test_err js identifier_err
// yield;
// await;
// async function test(await) {}
// function* test(yield) {}
// enum;
// implements;
// interface;

/// Parses an identifier if it is valid in this context or returns `Invalid` if the context isn't valid in this context.
/// An identifier is invalid if:
/// * It is named `await` inside of an async function
/// * It is named `yield` inside of a generator function or in strict mode
pub(super) fn parse_identifier(p: &mut JsParser, kind: JsSyntaxKind) -> ParsedSyntax {
    if is_at_metavariable(p) {
        return parse_metavariable(p);
    }

    if !is_at_identifier(p) {
        return Absent;
    }

    let error = match p.cur() {
        T![yield] if p.state().in_generator() => Some(p.err_builder(
            "Illegal use of `yield` as an identifier in generator function",
            p.cur_range(),
        )),
        t if t.is_future_reserved_keyword() => {
            if StrictMode.is_supported(p) {
                let name = p.cur_text();
                Some(p.err_builder(
                    format!(
                        "Illegal use of reserved keyword `{name}` as an identifier in strict mode"
                    ),
                    p.cur_range(),
                ))
            } else {
                None
            }
        }
        // test ts await_in_ambient_context
        // declare const await: any;
        T![await] if !p.state().in_ambient_context() => {
            if p.state().in_async() {
                Some(p.err_builder(
                    "Illegal use of `await` as an identifier in an async context",
                    p.cur_range(),
                ))
            } else if p.source_type().is_module() {
                Some(p.err_builder(
                    "Illegal use of `await` as an identifier inside of a module",
                    p.cur_range(),
                ))
            } else {
                None
            }
        }
        _ => None,
    };

    let m = p.start();
    p.bump_remap(T![ident]);
    let mut identifier = m.complete(p, kind);

    if let Some(error) = error {
        p.error(error);
        identifier.change_to_bogus(p);
    }

    Present(identifier)
}

#[inline]
pub(crate) fn is_at_identifier(p: &mut JsParser) -> bool {
    is_nth_at_identifier(p, 0)
}

#[inline]
pub(crate) fn is_nth_at_identifier(p: &mut JsParser, n: usize) -> bool {
    p.nth_at(n, T![ident])
        || p.nth(n).is_contextual_keyword()
        || p.nth(n).is_future_reserved_keyword()
}

#[inline]
pub(crate) fn is_nth_at_identifier_or_keyword(p: &mut JsParser, n: usize) -> bool {
    p.nth(n).is_keyword() || is_nth_at_identifier(p, n)
}

/// A template literal such as "`abcd ${efg}`"
// test js template_literal
// let a = `foo ${bar}`;
// let b = ``;
// let c = `${foo}`;
// let d = `foo`;
// let e = `${{ a: "string" }}`;
// test_err js template_literal
// let a = `foo ${}`
// let b = `${a a}`
fn parse_template_literal(
    p: &mut JsParser,
    marker: Marker,
    in_optional_chain: bool,
    tagged: bool,
) -> CompletedMarker {
    p.bump_with_context(BACKTICK, JsLexContext::TemplateElement { tagged });

    let elements_list = p.start();
    parse_template_elements(
        p,
        JS_TEMPLATE_CHUNK_ELEMENT,
        JS_TEMPLATE_ELEMENT,
        tagged,
        |p| {
            parse_expression(p, ExpressionContext::default())
                .or_add_diagnostic(p, js_parse_error::expected_expression)
        },
    );

    elements_list.complete(p, JS_TEMPLATE_ELEMENT_LIST);

    // test_err js template_literal_unterminated
    // let a = `${foo} bar

    // The lexer emits an error for unterminated template literals
    p.eat(BACKTICK);
    let mut completed = marker.complete(p, JS_TEMPLATE_EXPRESSION);

    // test_err js template_after_optional_chain
    // obj.val?.prop`template`
    // obj.val?.[expr]`template`
    // obj.func?.(args)`template`
    if in_optional_chain {
        p.error(p.err_builder(
            "Tagged template expressions are not permitted in an optional chain.",
            completed.range(p),
        ));
        completed.change_kind(p, JS_BOGUS_EXPRESSION);
    }

    completed
}

#[inline]
pub(crate) fn parse_template_elements<P>(
    p: &mut JsParser,
    chunk_kind: JsSyntaxKind,
    element_kind: JsSyntaxKind,
    tagged: bool,
    parse_element: P,
) where
    P: Fn(&mut JsParser) -> Option<CompletedMarker>,
{
    while !p.at(EOF) && !p.at(BACKTICK) {
        match p.cur() {
            TEMPLATE_CHUNK => {
                let m = p.start();
                p.bump_with_context(TEMPLATE_CHUNK, JsLexContext::TemplateElement { tagged });
                m.complete(p, chunk_kind);
            },
            DOLLAR_CURLY => {
                let e = p.start();
                p.bump(DOLLAR_CURLY);

                parse_element(p);
                if !p.at(T!['}']) {
                    p.error(expected_token(T!['}']));
                    // Seems there's more. For example a `${a a}`. We must eat all tokens away to avoid a panic because of an unexpected token
                    let _ =  ParseRecoveryTokenSet::new(JS_BOGUS, token_set![T!['}'], TEMPLATE_CHUNK, DOLLAR_CURLY, ERROR_TOKEN, BACKTICK]).recover(p);
                    if !p.at(T!['}']) {
                        e.complete(p, element_kind);
                        // Failed to fully recover, unclear where we are now, exit
                        break;
                    }
                }

                p.bump_with_context(T!['}'], JsLexContext::TemplateElement { tagged });
                e.complete(p, element_kind);
            }
            ERROR_TOKEN => {
                let err = p.err_builder("Invalid template literal",p.cur_range(), );
                p.error(err);
                p.bump_with_context(p.cur(), JsLexContext::TemplateElement { tagged });
            }
            t => unreachable!("Anything not template chunk or dollarcurly should have been eaten by the lexer, but {:?} was found", t),
        };
    }
}

struct ArrayElementsList;

impl ParseSeparatedList for ArrayElementsList {
    type Kind = JsSyntaxKind;
    type Parser<'a> = JsParser<'a>;
    const LIST_KIND: JsSyntaxKind = JS_ARRAY_ELEMENT_LIST;

    fn parse_element(&mut self, p: &mut JsParser) -> ParsedSyntax {
        match p.cur() {
            T![...] => parse_spread_element(p, ExpressionContext::default()),
            T![,] => Present(p.start().complete(p, JS_ARRAY_HOLE)),
            _ => parse_assignment_expression_or_higher(p, ExpressionContext::default()),
        }
    }

    fn is_at_list_end(&self, p: &mut JsParser) -> bool {
        p.at(T![']'])
    }

    fn recover(&mut self, p: &mut JsParser, parsed_element: ParsedSyntax) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(
                JS_BOGUS_EXPRESSION,
                EXPR_RECOVERY_SET.union(token_set!(T![']'])),
            ),
            js_parse_error::expected_array_element,
        )
    }

    fn separating_element_kind(&mut self) -> JsSyntaxKind {
        T![,]
    }

    fn allow_trailing_separating_element(&self) -> bool {
        true
    }
}

/// An array literal such as `[foo, bar, ...baz]`.
// test js array_expr
// [foo, bar];
// [foo];
// [,foo];
// [foo,];
// [,,,,,foo,,,,];
// [...a, ...b];
// test_err js array_expr_incomplete
// let a = [
fn parse_array_expr(p: &mut JsParser) -> ParsedSyntax {
    if !p.at(T!['[']) {
        return Absent;
    }
    let m = p.start();
    p.bump(T!['[']);

    // test js array_element_in_expr
    // for(["a" in {}];;) {}
    ArrayElementsList.parse_list(p);

    p.expect(T![']']);
    Present(m.complete(p, JS_ARRAY_EXPRESSION))
}

// test_err js spread
// [...]
/// A spread element consisting of three dots and an assignment expression such as `...foo`
pub(crate) fn parse_spread_element(p: &mut JsParser, context: ExpressionContext) -> ParsedSyntax {
    if !p.at(T![...]) {
        return Absent;
    }
    let m = p.start();
    p.bump(T![...]);
    parse_assignment_expression_or_higher(p, context)
        .or_add_diagnostic(p, js_parse_error::expected_expression_assignment);
    Present(m.complete(p, JS_SPREAD))
}

/// A left hand side expression, either a member expression or a call expression such as `foo()`.
pub(super) fn parse_lhs_expr(p: &mut JsParser, context: ExpressionContext) -> ParsedSyntax {
    // super.foo and super[bar]
    // test js super_property_access
    // super.foo
    // super[bar]
    // super[foo][bar]
    let lhs = if p.at(T![super]) {
        parse_super_expression(p)
    } else {
        parse_member_expression_or_higher(p, context)
    };

    lhs.map(|lhs_marker| parse_call_expression_rest(p, lhs_marker, context))
}

fn parse_call_expression_rest(
    p: &mut JsParser,
    lhs: CompletedMarker,
    context: ExpressionContext,
) -> CompletedMarker {
    let mut lhs = lhs;
    let mut in_optional_chain = false;
    loop {
        lhs = parse_member_expression_rest(p, lhs, context, true, &mut in_optional_chain);

        if !matches!(p.cur(), T![?.] | T![<] | T![<<] | T!['(']) {
            break lhs;
        }

        // Cloning here is necessary because parsing out the type arguments may rewind in which
        // case we want to return the `lhs`.
        let m = match lhs.kind(p) {
            TS_INSTANTIATION_EXPRESSION if !p.at(T![?.]) => lhs.clone().undo_completion(p),
            _ => lhs.clone().precede(p),
        };

        let start_pos = p.source().position();
        let optional_chain_call = p.eat(T![?.]);
        in_optional_chain = in_optional_chain || optional_chain_call;

        // test ts ts_call_expr_with_type_arguments
        // function a<A, B, C>() {}
        // a<A, B, C>();
        // (() => { a }).a<A, B, C>()
        // (() => a)<A, B, C>();
        // type A<T> = T;
        // a<<T>(arg: T) => number, number, string>();

        let type_arguments = if optional_chain_call {
            let type_arguments = parse_ts_type_arguments_in_expression(p, context).ok();
            if p.cur() == BACKTICK {
                // test ts ts_tagged_template_literal
                // html<A, B>`abcd`
                // html<A, B>`abcd`._string
                lhs = parse_template_literal(p, m, optional_chain_call, true);
                continue;
            }
            type_arguments
        } else {
            None
        };

        if type_arguments.is_some() || p.at(T!['(']) {
            parse_call_arguments(p)
                .or_add_diagnostic(p, |p, _| expected_token(T!['(']).into_diagnostic(p));
            lhs = m.complete(p, JS_CALL_EXPRESSION);
        } else {
            break if optional_chain_call {
                // If the `?.` is present and what followed was neither a valid type arguments nor valid arguments.
                // In this case, parse this as a static member access with an optional chain

                // test_err ts optional_chain_call_without_arguments
                // let a = { test: null };
                // a.test?.;
                // a.test?.<ab;
                p.error(expected_identifier(p, p.cur_range()));
                m.complete(p, JS_STATIC_MEMBER_EXPRESSION)
            } else {
                // test ts optional_chain_call_less_than
                // String(item)?.b < 0;
                // String(item)?.b <aBcd;

                // Safety:
                // * The method initially checks if the parsers at a '<', '(', or '?.' token.
                // * if the parser is at '?.': It takes the branch right above, ensuring that no token was consumed
                // * if the parser is at '<': `parse_ts_type_arguments_in_expression` rewinds if what follows aren't  valid type arguments and this is the only way we can reach this branch
                // * if the parser is at '(': This always parses out as valid arguments.
                debug_assert_eq!(p.source().position(), start_pos);
                m.abandon(p);
                lhs
            };
        }
    }
}

/// A postifx expression, either `LHSExpr [no linebreak] ++` or `LHSExpr [no linebreak] --`.
// test js postfix_expr
// foo++
// foo--
fn parse_postfix_expr(p: &mut JsParser, context: ExpressionContext) -> ParsedSyntax {
    let checkpoint = p.checkpoint();
    let lhs = parse_lhs_expr(p, context);
    lhs.map(|marker| {
        if !p.has_preceding_line_break() {
            // test js post_update_expr
            // foo++
            // foo--
            match p.cur() {
                T![++] => {
                    let assignment_target = expression_to_assignment(p, marker, checkpoint);
                    let m = assignment_target.precede(p);
                    p.bump(T![++]);
                    m.complete(p, JS_POST_UPDATE_EXPRESSION)
                }
                T![--] => {
                    let assignment_target = expression_to_assignment(p, marker, checkpoint);
                    let m = assignment_target.precede(p);
                    p.bump(T![--]);
                    m.complete(p, JS_POST_UPDATE_EXPRESSION)
                }
                _ => marker,
            }
        } else {
            marker
        }
    })
}

/// A unary expression such as `!foo` or `++bar`
pub(super) fn parse_unary_expr(p: &mut JsParser, context: ExpressionContext) -> ParsedSyntax {
    const UNARY_SINGLE: TokenSet<JsSyntaxKind> =
        token_set![T![delete], T![void], T![typeof], T![+], T![-], T![~], T![!]];

    if p.at(T![await]) {
        // test js await_expression
        // async function test() {
        //   await inner();
        //   await (inner()) + await inner();
        // }
        // async function inner() {
        //   return 4;
        // }
        // await test();

        // test_err js no_top_level_await_in_scripts
        // // SCRIPT
        // async function test() {}
        // await test();
        let m = p.start();
        let checkpoint = p.checkpoint();
        let await_range = p.cur_range();
        p.expect(T![await]);
        let unary = parse_unary_expr(p, context);

        let is_top_level_module_or_async_fn =
            p.state().in_async() && (p.state().is_top_level() || p.state().in_function());

        if !is_top_level_module_or_async_fn {
            // test js reparse_await_as_identifier
            // // SCRIPT
            // function test() { a = await; }
            // function test2() { return await; }
            if unary.is_absent() {
                p.rewind(checkpoint);
                m.abandon(p);
                return parse_identifier_expression(p);
            }

            // test_err js await_in_parameter_initializer
            // async function test(a = await b()) {}
            // function test2(a = await b()) {}

            // test_err js await_in_static_initialization_block_member
            // // SCRIPT
            // class A { static { await; } }
            // class B { static { await 10; } }

            // test_err js await_in_non_async_function
            // function test() { await 10; }

            // test_err js await_in_module
            // let await = 10;
            // console.log(await);
            p.error(p.err_builder(
                "`await` is only allowed within async functions and at the top levels of modules.",
                await_range,
            ));

            let expr = m.complete(p, JS_BOGUS_EXPRESSION);
            return Present(expr);
        }

        unary.or_add_diagnostic(p, js_parse_error::expected_unary_expression);
        let expr = m.complete(p, JS_AWAIT_EXPRESSION);
        return Present(expr);
    }

    // This is a type assertion expression if the parser is at the `<` token and JSX is disabled
    // JSX elements are parsed in parse_primary_expression.
    if p.at(T![<]) && Jsx.is_unsupported(p) {
        return TypeScript.parse_exclusive_syntax(
            p,
            |p| parse_ts_type_assertion_expression(p, context),
            |p, assertion| ts_only_syntax_error(p, "type assertion", assertion.range(p)),
        );
    }

    // test js pre_update_expr
    // ++foo
    // --foo
    if p.at(T![++]) {
        let m = p.start();
        p.bump(T![++]);
        parse_assignment(p, AssignmentExprPrecedence::Unary, context)
            .or_add_diagnostic(p, expected_simple_assignment_target);
        let complete = m.complete(p, JS_PRE_UPDATE_EXPRESSION);
        return Present(complete);
    }
    if p.at(T![--]) {
        let m = p.start();
        p.bump(T![--]);
        parse_assignment(p, AssignmentExprPrecedence::Unary, context)
            .or_add_diagnostic(p, expected_simple_assignment_target);
        let complete = m.complete(p, JS_PRE_UPDATE_EXPRESSION);
        return Present(complete);
    }

    // test js js_unary_expressions
    // delete a['test'];
    // void a;
    // typeof a;
    // +1;
    // -1;
    // ~1;
    // !true;
    // -a + -b + +a;

    // test_err js unary_expr
    // ++ ;
    // -- ;
    // -;

    if p.at_ts(UNARY_SINGLE) {
        let m = p.start();
        let op = p.cur();

        let is_delete = op == T![delete];

        if is_delete {
            p.expect(T![delete]);
        } else {
            p.bump_any();
        }

        // test js unary_delete
        // delete obj.key;
        // delete (obj).key;
        // delete obj.#member.key;
        // delete (obj.#member).key;
        // delete func().#member.key;
        // delete (func().#member).key;
        // delete obj?.#member.key;
        // delete (obj?.#member).key;
        // delete obj?.inner.#member.key;
        // delete (obj?.inner.#member).key;
        // delete obj[key];
        // delete (obj)[key];
        // delete obj.#member[key];
        // delete (obj.#member)[key];
        // delete func().#member[key];
        // delete (func().#member)[key];
        // delete obj?.#member[key];
        // delete (obj?.#member)[key];
        // delete obj?.inner.#member[key];
        // delete (obj?.inner.#member)[key];
        // delete (obj.#key, obj.key);
        // delete (#key in obj);
        // delete (obj.key);
        // delete (console.log(1));
        // delete (() => {});

        // test js unary_delete_nested
        // class TestClass { #member = true; method() { delete func(this.#member) } }
        // class TestClass { #member = true; method() { delete [this.#member] } }
        // class TestClass { #member = true; method() { delete { key: this.#member } } }
        // class TestClass { #member = true; method() { delete (() => { this.#member; }) } }
        // class TestClass { #member = true; method() { delete (param => { this.#member; }) } }
        // class TestClass { #member = true; method() { delete (async () => { this.#member; }) } }

        // test_err js unary_delete
        // delete ident;
        // delete obj.#member;
        // delete func().#member;
        // delete obj?.#member;
        // delete obj?.inner.#member;

        // test_err js unary_delete_parenthesized
        // delete (ident);
        // delete ((ident));
        // delete (obj.key, ident);
        // delete (obj.#member);
        // delete (func().#member);
        // delete (obj?.#member);
        // delete (obj?.inner.#member);
        // delete (obj.key, obj.#key);

        let mut kind = JS_UNARY_EXPRESSION;

        if is_delete {
            let checkpoint = p.checkpoint();
            parse_unary_expr(p, context).ok();

            let mut rewriter = DeleteExpressionRewriter::default();
            rewrite_events(&mut rewriter, checkpoint, p);

            rewriter.result.take().inspect(|_| {
                if StrictMode.is_supported(p) {
                    if let Some(range) = rewriter.exited_ident_expr {
                        kind = JS_BOGUS_EXPRESSION;
                        p.error(p.err_builder(
                            "the target for a delete operator cannot be a single identifier",
                            range,
                        ));
                    }
                }

                if let Some(range) = rewriter.exited_private_member_expr {
                    kind = JS_BOGUS_EXPRESSION;
                    p.error(p.err_builder(
                        "the target for a delete operator cannot be a private member",
                        range,
                    ));
                }
            })
        } else {
            parse_unary_expr(p, context).ok()
        };

        return Present(m.complete(p, kind));
    }

    parse_postfix_expr(p, context)
}

#[derive(Default)]
struct DeleteExpressionRewriter {
    stack: Vec<(RewriteMarker, JsSyntaxKind)>,
    result: Option<CompletedMarker>,
    /// Set to true immediately after the rewriter exits an identifier expression
    exited_ident_expr: Option<TextRange>,
    /// Set to true immediately after the rewriter exits a private name
    exited_private_name: bool,
    /// Set to true immediately after the rewriter exits a member expresison with a private name
    exited_private_member_expr: Option<TextRange>,
}

impl RewriteParseEvents for DeleteExpressionRewriter {
    fn start_node(&mut self, kind: JsSyntaxKind, p: &mut RewriteParser) {
        self.stack.push((p.start(), kind));
        self.exited_ident_expr.take();
        self.exited_private_name = false;
        self.exited_private_member_expr.take();
    }

    fn finish_node(&mut self, p: &mut RewriteParser) {
        let (m, kind) = self.stack.pop().expect("stack depth mismatch");
        let node = m.complete(p, kind);

        if kind != JS_PARENTHESIZED_EXPRESSION && kind != JS_SEQUENCE_EXPRESSION {
            self.exited_private_member_expr =
                if self.exited_private_name && kind == JS_STATIC_MEMBER_EXPRESSION {
                    Some(node.range(p))
                } else {
                    None
                };

            self.exited_ident_expr = if kind == JS_IDENTIFIER_EXPRESSION {
                Some(node.range(p))
            } else {
                None
            };

            self.exited_private_name = kind == JS_PRIVATE_NAME;
        }

        self.result = Some(node.into());
    }
}

pub(super) fn is_at_name(p: &mut JsParser) -> bool {
    is_nth_at_name(p, 0)
}

pub(super) fn is_nth_at_name(p: &mut JsParser, offset: usize) -> bool {
    p.nth_at(offset, T![ident]) || p.nth(offset).is_keyword()
}

pub(super) fn is_nth_at_any_name(p: &mut JsParser, n: usize) -> bool {
    is_nth_at_name(p, n) || p.nth_at(n, T![#])
}
