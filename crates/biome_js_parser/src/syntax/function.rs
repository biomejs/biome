use crate::parser::ParsedSyntax;
use crate::prelude::*;
use crate::state::{EnterFunction, EnterParameters, SignatureFlags};
use crate::syntax::binding::{
    is_at_identifier_binding, is_nth_at_identifier_binding, parse_binding, parse_binding_pattern,
};
use crate::syntax::class::{
    empty_decorator_list, parse_initializer_clause, parse_parameter_decorators,
};
use crate::syntax::expr::{
    is_nth_at_identifier, parse_assignment_expression_or_higher, ExpressionContext,
};
use crate::syntax::js_parse_error;
use crate::syntax::js_parse_error::{
    decorators_not_allowed, expected_binding, expected_parameter, expected_parameters,
};
use crate::syntax::stmt::{is_semi, parse_block_impl, semi, StatementContext};
use crate::syntax::typescript::ts_parse_error::ts_only_syntax_error;
use crate::syntax::typescript::{
    is_nth_at_type_parameter_modifier, parse_ts_return_type_annotation, parse_ts_type_annotation,
    parse_ts_type_parameters, try_parse, TypeContext,
};

use crate::JsSyntaxFeature::TypeScript;
use crate::ParsedSyntax::{Absent, Present};
use crate::{JsParser, JsSyntaxFeature, ParseRecoveryTokenSet};
use biome_js_syntax::JsSyntaxKind::*;
use biome_js_syntax::{JsSyntaxKind, TextRange, T};
use biome_parser::ParserProgress;
use biome_rowan::SyntaxKind;

use super::metavariable::parse_metavariable;

/// A function declaration, this could be async and or a generator. This takes a marker
/// because you need to first advance over async or start a marker and feed it in.
// test js function_decl
// function foo1() {}
// function *foo2() {}
// async function *foo3() {}
// async function foo4() {}
// function *foo5() {
//   yield foo;
// }
//
// test js function_declaration_script
// // SCRIPT
// function test(await) {}
//
// test_err js function_decl_err
// function() {}
// function foo {}
// function {}
// function *() {}
// async function() {}
// async function *() {}
// function *foo2() {}
// yield foo3;
// function test2(): number {}
// function foo4(await) {}
// function foo5(yield) {}
//
// test_err js function_broken
// function foo())})}{{{  {}
//
// test ts ts_function_statement
// function test(a: string, b?: number, c="default") {}
// function test2<A, B extends A, C = A>(a: A, b: B, c: C) {}
pub(super) fn parse_function_declaration(
    p: &mut JsParser,
    context: StatementContext,
) -> ParsedSyntax {
    if !is_at_function(p) {
        return Absent;
    }

    let m = p.start();
    let mut function = if p.state().in_ambient_context() {
        parse_ambient_function(p, m, AmbientFunctionKind::Declaration)
    } else {
        parse_function(
            p,
            m,
            FunctionKind::Declaration {
                single_statement_context: context.is_single_statement(),
            },
        )
    };

    if context != StatementContext::StatementList && !function.kind(p).is_bogus() {
        if JsSyntaxFeature::StrictMode.is_supported(p) {
            // test_err js function_in_single_statement_context_strict
            // if (true) function a() {}
            // label1: function b() {}
            // while (true) function c() {}
            p.error(p.err_builder("In strict mode code, functions can only be declared at top level or inside a block", function.range(p)).with_hint( "wrap the function in a block statement"));
            function.change_to_bogus(p);
        } else if !matches!(context, StatementContext::If | StatementContext::Label) {
            // test js function_in_if_or_labelled_stmt_loose_mode
            // // SCRIPT
            // label1: function a() {}
            // if (true) function b() {} else function c() {}
            // if (true) function d() {}
            // if (true) "test"; else function e() {}
            p.error(p.err_builder("In non-strict mode code, functions can only be declared at top level, inside a block, or as the body of an if or labelled statement", function.range(p)).with_hint( "wrap the function in a block statement"));
            function.change_to_bogus(p);
        }
    }

    Present(function)
}

pub(super) fn parse_function_expression(p: &mut JsParser) -> ParsedSyntax {
    if !is_at_function(p) {
        return Absent;
    }

    let m = p.start();
    Present(parse_function(p, m, FunctionKind::Expression))
}

// test js export_default_function_clause
// export default function test(a, b) {}
//
// test ts ts_export_default_function_overload
// export default function test(a: string): string;
// export default function test(a: string | undefined): string { return "hello" }
//
// test ts ts_export_function_overload
// export function test(a: string): string;
// export function test(a: string | undefined): string { return "hello" }
pub(super) fn parse_function_export_default_declaration(p: &mut JsParser) -> ParsedSyntax {
    if !is_at_function(p) {
        return Absent;
    }

    let m = p.start();

    Present(if p.state().in_ambient_context() {
        parse_ambient_function(p, m, AmbientFunctionKind::ExportDefault)
    } else {
        parse_function(p, m, FunctionKind::ExportDefault)
    })
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum AmbientFunctionKind {
    Declaration,
    ExportDefault,
}

impl AmbientFunctionKind {
    const fn is_export_default(&self) -> bool {
        matches!(self, AmbientFunctionKind::ExportDefault)
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum FunctionKind {
    Declaration {
        // https://tc39.es/ecma262/multipage/additional-ecmascript-features-for-web-browsers.html#sec-functiondeclarations-in-ifstatement-statement-clauses
        single_statement_context: bool,
    },
    Expression,
    ExportDefault,
}

impl FunctionKind {
    const fn is_export_default(&self) -> bool {
        matches!(self, FunctionKind::ExportDefault)
    }

    fn is_id_optional(&self) -> bool {
        matches!(self, FunctionKind::Expression | FunctionKind::ExportDefault)
    }

    fn is_expression(&self) -> bool {
        matches!(self, FunctionKind::Expression)
    }

    fn is_in_single_statement_context(&self) -> bool {
        matches!(
            self,
            FunctionKind::Declaration {
                single_statement_context: true
            }
        )
    }
}

impl From<FunctionKind> for JsSyntaxKind {
    fn from(kind: FunctionKind) -> Self {
        match kind {
            FunctionKind::Declaration { .. } => JS_FUNCTION_DECLARATION,
            FunctionKind::Expression => JS_FUNCTION_EXPRESSION,
            FunctionKind::ExportDefault => JS_FUNCTION_EXPORT_DEFAULT_DECLARATION,
        }
    }
}

fn is_at_function(p: &mut JsParser) -> bool {
    p.at_ts(token_set![T![async], T![function]]) || is_at_async_function(p, LineBreak::DoNotCheck)
}

#[inline]
fn parse_function(p: &mut JsParser, m: Marker, kind: FunctionKind) -> CompletedMarker {
    let mut flags = SignatureFlags::empty();

    let in_async = is_at_async_function(p, LineBreak::DoNotCheck);
    if in_async {
        // test_err js function_escaped_async
        // void \u0061sync function f(){}
        p.eat(T![async]);
        flags |= SignatureFlags::ASYNC;
    }

    p.expect(T![function]);
    let generator_range = if p.at(T![*]) {
        let range = p.cur_range();
        p.bump(T![*]);
        flags |= SignatureFlags::GENERATOR;
        Some(range)
    } else {
        None
    };

    let id = parse_function_id(p, kind, flags);

    if !kind.is_id_optional() {
        id.or_add_diagnostic(p, |p, range| {
            p.err_builder(
                "expected a name for the function in a function declaration, but found none",
                range,
            )
        });
    }

    TypeScript
        .parse_exclusive_syntax(
            p,
            |p| parse_ts_type_parameters(p, TypeContext::default().and_allow_const_modifier(true)),
            |p, marker| {
                p.err_builder(
                    "type parameters can only be used in TypeScript files",
                    marker.range(p),
                )
            },
        )
        .ok();

    let parameter_context = if !kind.is_expression() && TypeScript.is_supported(p) {
        // It isn't known at this point if this is a function overload definition (body is missing)
        // or a regular function implementation.
        // Let's go with the laxer of the two. Ideally, these verifications should be part of
        // a second compiler pass.
        ParameterContext::Declaration
    } else {
        ParameterContext::Implementation
    };

    parse_parameter_list(p, parameter_context, TypeContext::default(), flags)
        .or_add_diagnostic(p, js_parse_error::expected_parameters);

    TypeScript
        .parse_exclusive_syntax(
            p,
            |p| parse_ts_return_type_annotation(p, TypeContext::default()),
            |p, marker| {
                p.err_builder(
                    "return types can only be used in TypeScript files",
                    marker.range(p),
                )
            },
        )
        .ok();

    let body = parse_function_body(p, flags);

    // test ts ts_function_overload
    // function test(a: string): void;
    // function test(a: string | undefined): void {}
    // function no_semi(a: string)
    // function no_semi(a: string) {}
    // async function async_overload(a: string)
    // async function async_overload(a: string) {}
    if body.is_absent()
        && TypeScript.is_supported(p)
        && is_semi(p, 0)
        && !kind.is_in_single_statement_context()
        && !kind.is_expression()
    {
        p.eat(T![;]);

        // test_err ts ts_function_overload_generator
        // function* test(a: string);
        // function* test(a: string) {}
        if let Some(generator_range) = generator_range {
            p.error(p.err_builder(
                "An overload signature cannot be declared as a generator.",
                generator_range,
            ));
        }

        if kind.is_export_default() {
            m.complete(p, TS_DECLARE_FUNCTION_EXPORT_DEFAULT_DECLARATION)
        } else {
            m.complete(p, TS_DECLARE_FUNCTION_DECLARATION)
        }
    } else {
        body.or_add_diagnostic(p, js_parse_error::expected_function_body);

        let mut function = m.complete(p, kind.into());

        // test_err js async_or_generator_in_single_statement_context
        // if (true) async function t() {}
        // if (true) function* t() {}
        if kind.is_in_single_statement_context() && (in_async || generator_range.is_some()) {
            p.error(p.err_builder("`async` and generator functions can only be declared at top level or inside a block", function.range(p) ));
            function.change_to_bogus(p);
        }

        function
    }
}

// test_err js break_in_nested_function
// while (true) {
//   function helper() {
//     break;
//   }
// }
pub(super) fn parse_function_body(p: &mut JsParser, flags: SignatureFlags) -> ParsedSyntax {
    p.with_state(EnterFunction(flags), |p| {
        parse_block_impl(p, JS_FUNCTION_BODY)
    })
}

fn parse_function_id(p: &mut JsParser, kind: FunctionKind, flags: SignatureFlags) -> ParsedSyntax {
    match kind {
        // Takes the async and generator restriction from the expression
        FunctionKind::Expression => {
            // test js function_expression_id
            // // SCRIPT
            // (function await() {});
            // (function yield() {});
            // (async function yield() {});
            // (function* await() {})
            //
            // test_err js function_expression_id_err
            // (async function await() {});
            // (function* yield() {});
            // function* test() { function yield() {} }
            p.with_state(EnterFunction(flags), parse_binding)
        }
        // Inherits the async and generator from the parent
        _ => {
            // test js function_id
            // // SCRIPT
            // function test() {}
            // function await(test) {}
            // async function await(test) {}
            // function yield(test) {}
            // function* yield(test) {}
            //
            //
            // test_err js function_id_err
            // function* test() {
            //   function yield(test) {}
            // }
            parse_binding(p)
        }
    }
}

// test ts ts_declare_function
// declare function test<A, B, R>(a: A, b: B): R;
// declare function test2({ a }?: { a: "string" })
// declare
// function not_a_declaration() {}
//
// test_err ts ts_declare_function_with_body
// declare function test<A>(a: A): string { return "ambient function with a body"; }
//
// test ts ts_ambient_function
// declare module a {
//   function test(): string;
// }
fn parse_ambient_function(
    p: &mut JsParser,
    m: Marker,
    kind: AmbientFunctionKind,
) -> CompletedMarker {
    let stmt_start = p.cur_range().start();

    // test_err ts ts_declare_async_function
    // declare async function test();
    let is_async = p.at(T![async]);
    if is_async {
        p.error(p.err_builder(
            "'async' modifier cannot be used in an ambient context.",
            p.cur_range(),
        ));
        p.bump(T![async]);
    }

    p.expect(T![function]);

    let is_generator = p.at(T![*]);
    if is_generator {
        // test_err ts ts_declare_generator_function
        // declare function* test(): void;
        // declare module 'x' {
        //   export default function* test(): void
        // }
        p.error(p.err_builder(
            "Generators are not allowed in an ambient context.",
            p.cur_range(),
        ));
        p.bump(T![*]);
    }

    let binding = parse_binding(p);
    let binding_range = p.cur_range();

    parse_ts_type_parameters(p, TypeContext::default().and_allow_const_modifier(true)).ok();
    parse_parameter_list(
        p,
        ParameterContext::Declaration,
        TypeContext::default(),
        SignatureFlags::empty(),
    )
    .or_add_diagnostic(p, expected_parameters);
    parse_ts_return_type_annotation(p, TypeContext::default()).ok();

    if let Present(body) = parse_function_body(p, SignatureFlags::empty()) {
        p.error(
            p.err_builder(
                "A 'declare' function cannot have a function body",
                body.range(p),
            )
            .with_hint("remove this body"),
        );
    }

    semi(p, TextRange::new(stmt_start, p.cur_range().start()));

    if is_async {
        m.complete(p, JS_BOGUS_STATEMENT)
    } else if kind.is_export_default() {
        // test ts ts_declare_function_export_default_declaration
        // declare module 'x' {
        //   export default function(option: any): void
        // }
        // declare module 'y' {
        //   export default function test(option: any): void
        // }
        m.complete(p, TS_DECLARE_FUNCTION_EXPORT_DEFAULT_DECLARATION)
    } else {
        // test_err ts ts_declare_function_export_declaration_missing_id
        // declare module 'x' {
        //   export function(option: any): void
        // }
        if binding.is_absent() {
            p.error(expected_binding(p, binding_range));
        }
        // test ts ts_declare_function_export_declaration
        // declare module 'x' {
        //   export function test(option: any): void
        // }
        m.complete(p, TS_DECLARE_FUNCTION_DECLARATION)
    }
}

pub(crate) fn parse_ts_type_annotation_or_error(p: &mut JsParser) -> ParsedSyntax {
    TypeScript.parse_exclusive_syntax(
        p,
        |p| parse_ts_type_annotation(p, TypeContext::default()),
        |p, annotation| {
            p.err_builder(
                "return types can only be used in TypeScript files",
                annotation.range(p),
            )
            .with_hint("remove this type annotation")
        },
    )
}

/// Tells [is_at_async_function] if it needs to check line breaks
#[derive(PartialEq, Eq)]
pub(crate) enum LineBreak {
    // check line breaks
    DoCheck,
    // do not check line break
    DoNotCheck,
}

#[inline]
/// Checks if the parser is inside a "async function"
pub(super) fn is_at_async_function(p: &mut JsParser, should_check_line_break: LineBreak) -> bool {
    let async_function_tokens = p.at(T![async]) && p.nth_at(1, T![function]);
    if should_check_line_break == LineBreak::DoCheck {
        async_function_tokens && !p.has_nth_preceding_line_break(1)
    } else {
        async_function_tokens
    }
}

/// There are cases where the parser must speculatively parse a syntax. For example,
/// parsing `<string>(test)` very much looks like an arrow expression *except* that it isn't followed
/// by a `=>`. This enum tells a parse function if ambiguity should be tolerated or if it should stop if it is not.
#[derive(Debug, Copy, Clone)]
pub(crate) enum Ambiguity {
    /// Ambiguity is allowed. A parse method should continue even if an expected character is missing.
    Allowed,

    /// Ambiguity isn't allowed. A parse method should stop parsing if an expected character is missing
    /// and let the caller decide what to do in this case.
    Disallowed,
}

impl Ambiguity {
    fn is_disallowed(&self) -> bool {
        matches!(self, Ambiguity::Disallowed)
    }
}

pub(crate) fn parse_arrow_function_expression(p: &mut JsParser) -> ParsedSyntax {
    parse_parenthesized_arrow_function_expression(p)
        .or_else(|| parse_arrow_function_with_single_parameter(p))
}

/// Tries to parse the header of a parenthesized arrow function expression.
///
/// The header is everything coming before the (or everything up and including the `=>` token):
/// `async (a) =>`.
///
/// Returns the [Marker] for the parsed arrow function header that must be completed by the caller.
///
/// ## Errors
///
/// Returns `Err` if `ambiguity` is [Ambiguity::Disallowed] and the syntax
/// is ambiguous. The `Err` contains the [Marker] of the syntax parsed to this point. It's up
/// to the caller to abandon or complete the returned marker.
///
/// For example, the parser speculatively tries to parse `<string>(test)` as an arrow
/// function because the start very much looks like one, except that the `=>` token is missing
/// (it's a TypeScript `<string>` cast followed by a parenthesized expression).
fn try_parse_parenthesized_arrow_function_head(
    p: &mut JsParser,
    ambiguity: Ambiguity,
) -> Result<(Marker, SignatureFlags), Marker> {
    let m = p.start();

    // test_err js arrow_escaped_async
    // \u0061sync () => {}
    let flags = if p.eat(T![async]) {
        SignatureFlags::ASYNC
    } else {
        SignatureFlags::empty()
    };

    if p.at(T![<]) {
        parse_ts_type_parameters(p, TypeContext::default().and_allow_const_modifier(true)).ok();

        if ambiguity.is_disallowed() && p.last() != Some(T![>]) {
            return Err(m);
        }
    }

    if !p.at(T!['(']) && ambiguity.is_disallowed() {
        return Err(m);
    }

    // test_err ts ts_decorator_on_arrow_function { "parse_class_parameter_decorators": true }
    // const method = (@dec x, second, @dec third = 'default') => {};
    // const method = (@dec.fn() x, second, @dec.fn() third = 'default') => {};
    // const method = (@dec() x, second, @dec() third = 'default') => {};
    parse_parameter_list(
        p,
        ParameterContext::Arrow,
        TypeContext::default(),
        arrow_function_parameter_flags(p, flags),
    )
    .or_add_diagnostic(p, expected_parameters);

    if p.last() != Some(T![')']) && ambiguity.is_disallowed() {
        return Err(m);
    }

    TypeScript
        .parse_exclusive_syntax(
            p,
            |p| parse_ts_return_type_annotation(p, TypeContext::default()),
            |p, annotation| ts_only_syntax_error(p, "return type annotation", annotation.range(p)),
        )
        .ok();

    if p.has_preceding_line_break() {
        p.error(p.err_builder("Line terminator not permitted before arrow.", p.cur_range()));
    }

    if !p.expect(T![=>]) && ambiguity.is_disallowed() {
        return Err(m);
    }

    Ok((m, flags))
}

// test ts ts_arrow_function_type_parameters
// let a = <A, B extends A, C = string>(a: A, b: B, c: C) => "hello";
// let b = async <A, B>(a: A, b: B): Promise<string> => "hello";
fn parse_possible_parenthesized_arrow_function_expression(p: &mut JsParser) -> ParsedSyntax {
    let start_pos = p.cur_range().start();

    // Test if we already tried to parse this position as an arrow function and failed.
    // If so, bail out immediately.
    if p.state().not_parenthesized_arrow.contains(&start_pos) {
        return Absent;
    }

    match try_parse(p, |p| {
        try_parse_parenthesized_arrow_function_head(p, Ambiguity::Disallowed)
    }) {
        Ok((m, flags)) => {
            parse_arrow_body(p, flags).or_add_diagnostic(p, js_parse_error::expected_arrow_body);

            Present(m.complete(p, JS_ARROW_FUNCTION_EXPRESSION))
        }
        Err(m) => {
            // SAFETY: Abandoning the marker here is safe because `try_parse` rewinds if
            // the callback returns `Err` (which is the case that this branch is handling).
            m.abandon(p);

            p.state_mut().not_parenthesized_arrow.insert(start_pos);
            Absent
        }
    }
}

fn parse_parenthesized_arrow_function_expression(p: &mut JsParser) -> ParsedSyntax {
    let is_parenthesized = is_parenthesized_arrow_function_expression(p);
    match is_parenthesized {
        IsParenthesizedArrowFunctionExpression::True => {
            let (m, flags) = try_parse_parenthesized_arrow_function_head(p, Ambiguity::Allowed).expect("'CompletedMarker' because function should never return 'Err' if called with 'Ambiguity::Allowed'.");
            parse_arrow_body(p, flags).or_add_diagnostic(p, js_parse_error::expected_arrow_body);
            Present(m.complete(p, JS_ARROW_FUNCTION_EXPRESSION))
        }
        IsParenthesizedArrowFunctionExpression::Unknown => {
            parse_possible_parenthesized_arrow_function_expression(p)
        }
        IsParenthesizedArrowFunctionExpression::False => Absent,
    }
}

#[derive(Debug, Copy, Clone)]
enum IsParenthesizedArrowFunctionExpression {
    True,
    False,
    Unknown,
}

// test js paren_or_arrow_expr
// (foo);
// (foo) => {};
// (5 + 5);
// ({foo, bar, b: [f, ...baz]}) => {};
// (foo, ...bar) => {}

// test_err js paren_or_arrow_expr_invalid_params
// (5 + 5) => {}
// (a, ,b) => {}
// (a, b) =>;
// (a: string;
// (a, b)
//  => {}

fn is_parenthesized_arrow_function_expression(
    p: &mut JsParser,
) -> IsParenthesizedArrowFunctionExpression {
    match p.cur() {
        // These could be the start of a parenthesized arrow function expression but needs further verification
        T!['('] | T![<] => {
            is_parenthesized_arrow_function_expression_impl(p, SignatureFlags::empty())
        }
        T![async] => {
            // test js async_arrow_expr
            // let a = async foo => {}
            // let b = async (bar) => {}
            // async (foo, bar, ...baz) => foo
            if p.has_nth_preceding_line_break(1) {
                IsParenthesizedArrowFunctionExpression::False
            } else if matches!(p.nth(1), T!['('] | T![<]) {
                is_parenthesized_arrow_function_expression_impl(p, SignatureFlags::ASYNC)
            } else {
                IsParenthesizedArrowFunctionExpression::False
            }
        }

        // Not entirely correct but that's probably what the user intended
        T![=>] => IsParenthesizedArrowFunctionExpression::True,
        _ => IsParenthesizedArrowFunctionExpression::False,
    }
}

// Tests if the parser is at an arrow function expression
fn is_parenthesized_arrow_function_expression_impl(
    p: &mut JsParser,
    flags: SignatureFlags,
) -> IsParenthesizedArrowFunctionExpression {
    let n = usize::from(flags.contains(SignatureFlags::ASYNC));

    match p.nth(n) {
        T!['('] => {
            match p.nth(n + 1) {
                T![')'] => {
                    // '()' is an arrow expression if followed by an '=>', a type annotation or body.
                    // Otherwise, a parenthesized expression with a missing inner expression
                    match p.nth(n + 2) {
                        T![=>] | T![:] | T!['{'] => IsParenthesizedArrowFunctionExpression::True,
                        _ => IsParenthesizedArrowFunctionExpression::False,
                    }
                }
                // Rest parameter '(...a' is certainly not a parenthesized expression
                T![...] => IsParenthesizedArrowFunctionExpression::True,
                // '([ ...', '({ ... } can either be a parenthesized object or array expression or a destructing parameter
                T!['['] | T!['{'] => IsParenthesizedArrowFunctionExpression::Unknown,

                // '(@' can be a decorator or a parenthesized arrow function
                T![@] => IsParenthesizedArrowFunctionExpression::Unknown,
                // '(a...'
                _ if is_nth_at_identifier_binding(p, n + 1) || p.nth_at(n + 1, T![this]) => {
                    match p.nth(n + 2) {
                        // '(a: ' must be a type annotation
                        T![:] => IsParenthesizedArrowFunctionExpression::True,

                        // Unclear because it could either be
                        // * '(a = ': an initializer or a parenthesized assignment expression
                        // * '(a, ': separator to next parameter or a parenthesized sequence expression
                        // * '(a)': a single parameter OR a parenthesized expression
                        T![=] | T![,] | T![')'] => IsParenthesizedArrowFunctionExpression::Unknown,

                        T![?] => {
                            // Disambiguate between an optional parameter and a parenthesized conditional expression
                            match p.nth(n + 3) {
                                // '(a?:' | '(a?,' | '(a?=' | '(a?)'
                                T![:] | T![,] | T![=] | T![')'] => {
                                    IsParenthesizedArrowFunctionExpression::True
                                }
                                _ => IsParenthesizedArrowFunctionExpression::False,
                            }
                        }
                        _ => IsParenthesizedArrowFunctionExpression::False,
                    }
                }
                _ => IsParenthesizedArrowFunctionExpression::False,
            }
        }
        // potential start of type parameters
        T![<] => {
            // test jsx jsx_type_arguments
            // // These may look like a valid arrows but are JSX
            // <A extends>() =</A>;
            // <A extends="B">() =</A>;
            // <A extends ok>() =</A>;
            // <const A>() =</const>;
            // <const A extends/>;
            // <A extends/>;

            // test tsx tsx_type_arguments
            // // These are valid type arguments
            // <A,>() => {};
            // <const A,>() => {};
            // <A extends B>() => {};
            // <A=string>() => {};
            // <A, B>() => {};
            // <A extends B<C>>() => {};

            if JsSyntaxFeature::Jsx.is_supported(p) {
                // Disambiguate between JSX and type parameters
                // Type parameters of arrow functions accept only the `const` modifier.
                let n = if p.nth_at(n + 1, T![const]) { n + 1 } else { n };
                if !is_nth_at_identifier(p, n + 1) {
                    // <5...
                    return IsParenthesizedArrowFunctionExpression::False;
                };
                match p.nth(n + 2) {
                    T![extends] => {
                        // `<a extends=` OR `<a extends/>` OR `<a extends>` is a JSX element
                        // and a `extends` type refinement: `<A extends string>`
                        if matches!(p.nth(n + 3), T![=] | T![/] | T![>]) {
                            IsParenthesizedArrowFunctionExpression::False
                        }
                        // `<A extends B>` Could be either
                        else if is_nth_at_identifier(p, n + 3) {
                            IsParenthesizedArrowFunctionExpression::Unknown
                        } else {
                            // <A extends B> must be type arguments
                            IsParenthesizedArrowFunctionExpression::True
                        }
                    }
                    // `<A=` or `<A,` or always type arguments and never JSX tags
                    T![=] | T![,] => IsParenthesizedArrowFunctionExpression::True,
                    _ => IsParenthesizedArrowFunctionExpression::False,
                }
            } else if is_nth_at_type_parameter_modifier(p, n + 1) {
                // <const T>...
                IsParenthesizedArrowFunctionExpression::True
            } else if !is_nth_at_identifier(p, n + 1) {
                // <5...
                IsParenthesizedArrowFunctionExpression::False
            } else {
                // <a...
                IsParenthesizedArrowFunctionExpression::Unknown
            }
        }
        _ => IsParenthesizedArrowFunctionExpression::False,
    }
}

/// Computes the signature flags for parsing the parameters of an arrow expression. These
/// have different semantics from parsing the body
fn arrow_function_parameter_flags(p: &JsParser, mut flags: SignatureFlags) -> SignatureFlags {
    if p.state().in_generator() {
        // Arrow functions inherit whatever yield is a valid identifier name from the parent.
        flags |= SignatureFlags::GENERATOR;
    }

    // The arrow function is in an async context if the outer function is in an async context or itself is
    // declared async
    if p.state().in_async() {
        flags |= SignatureFlags::ASYNC;
    }

    flags
}

// test js arrow_expr_single_param
// // SCRIPT
// foo => {}
// yield => {}
// await => {}
// baz =>
// {}
fn parse_arrow_function_with_single_parameter(p: &mut JsParser) -> ParsedSyntax {
    if !is_arrow_function_with_single_parameter(p) {
        return Absent;
    }

    let m = p.start();
    let is_async = p.at(T![async]) && is_nth_at_identifier_binding(p, 1);

    let flags = if is_async {
        p.eat(T![async]);
        SignatureFlags::ASYNC
    } else {
        SignatureFlags::empty()
    };

    // test_err js async_arrow_expr_await_parameter
    // let a = async await => {}
    // async() => { (a = await) => {} };
    // async() => { (a = await 10) => {} };
    p.with_state(EnterParameters(arrow_function_parameter_flags(p, flags)), parse_binding)
        .expect("Expected function parameter to be present as guaranteed by is_arrow_function_with_simple_parameter");

    p.bump(T![=>]);
    parse_arrow_body(p, flags).or_add_diagnostic(p, js_parse_error::expected_arrow_body);

    Present(m.complete(p, JS_ARROW_FUNCTION_EXPRESSION))
}

fn is_arrow_function_with_single_parameter(p: &mut JsParser) -> bool {
    // a => ...
    if p.nth_at(1, T![=>]) {
        // test js single_parameter_arrow_function_with_parameter_named_async
        // let id = async => async;
        is_at_identifier_binding(p) && !p.has_nth_preceding_line_break(1)
    }
    // async ident => ...
    else {
        p.at(T![async])
            && !p.has_nth_preceding_line_break(1)
            && is_nth_at_identifier_binding(p, 1)
            && !p.has_nth_preceding_line_break(2)
            && p.nth_at(2, T![=>])
    }
}

fn parse_arrow_body(p: &mut JsParser, mut flags: SignatureFlags) -> ParsedSyntax {
    // test js arrow_in_constructor
    // class A {
    //   constructor() {
    //     () => { super() };
    //     () => super();
    //  }
    // }
    if p.state().in_constructor() {
        flags |= SignatureFlags::CONSTRUCTOR
    }

    if p.at(T!['{']) {
        parse_function_body(p, flags)
    } else {
        p.with_state(EnterFunction(flags), |p| {
            parse_assignment_expression_or_higher(p, ExpressionContext::default())
        })
    }
}

pub(crate) fn parse_any_parameter(
    p: &mut JsParser,
    decorator_list: ParsedSyntax,
    parameter_context: ParameterContext,
    expression_context: ExpressionContext,
    type_context: TypeContext,
) -> ParsedSyntax {
    let parameter = match p.cur() {
        T![...] => parse_rest_parameter(p, decorator_list, expression_context, type_context),
        T![this] => {
            // test_err ts ts_decorator_this_parameter_option { "parse_class_parameter_decorators": true }
            // class A {
            //   method(@dec this) {}
            //   method(@dec(val) this) {}
            //   method(@dec.fn(val) this) {}
            // }
            decorator_list
                .add_diagnostic_if_present(p, decorators_not_allowed)
                .map(|mut decorator_list| {
                    decorator_list.change_to_bogus(p);
                    decorator_list
                });
            parse_ts_this_parameter(p, type_context)
        }
        _ => parse_formal_parameter(
            p,
            decorator_list,
            parameter_context,
            expression_context,
            type_context,
        ),
    };

    parameter.map(|mut parameter| {
        if parameter.kind(p) == TS_THIS_PARAMETER {
            if TypeScript.is_unsupported(p) {
                parameter.change_to_bogus(p);
                p.error(ts_only_syntax_error(
                    p,
                    "this parameter",
                    parameter.range(p),
                ));
            } else if parameter_context.is_arrow_function() {
                // test_err ts ts_arrow_function_this_parameter
                // let a = (this: string) => {}
                parameter.change_to_bogus(p);
                p.error(p.err_builder(
                    "An arrow function cannot have a 'this' parameter.",
                    parameter.range(p),
                ));
            }
        }

        parameter
    })
}

pub(crate) fn parse_rest_parameter(
    p: &mut JsParser,
    decorator_list: ParsedSyntax,
    expression_context: ExpressionContext,
    type_context: TypeContext,
) -> ParsedSyntax {
    if !p.at(T![...]) {
        return Absent;
    }

    let m = decorator_list
        .or_else(|| empty_decorator_list(p))
        .precede(p);
    p.bump(T![...]);
    parse_binding_pattern(p, expression_context).or_add_diagnostic(p, expected_binding);

    let mut valid = true;

    if p.eat(T![?]) {
        let err = p.err_builder("rest patterns cannot be optional", p.cur_range());

        p.error(err);
        valid = false;
    }

    // type annotation `...foo: number[]`
    TypeScript
        .parse_exclusive_syntax(
            p,
            |p| parse_ts_type_annotation(p, type_context),
            |p, annotation| ts_only_syntax_error(p, "type annotation", annotation.range(p)),
        )
        .ok();

    if let Present(initializer) = parse_initializer_clause(p, ExpressionContext::default()) {
        // test_err js arrow_rest_in_expr_in_initializer
        // for ((...a = "b" in {}) => {};;) {}
        let err = p.err_builder(
            "rest elements may not have default initializers",
            initializer.range(p),
        );

        p.error(err);
        valid = false;
    }

    let mut rest_parameter = m.complete(p, JS_REST_PARAMETER);

    if p.at(T![,]) {
        let err = p.err_builder(
            "rest elements may not have trailing commas",
            rest_parameter.range(p),
        );

        p.error(err);
        valid = false;
    }

    if !valid {
        rest_parameter.change_to_bogus(p);
    }

    Present(rest_parameter)
}

// test ts ts_this_parameter
// function a(this) {}
// function b(this: string) {}
pub(crate) fn parse_ts_this_parameter(p: &mut JsParser, context: TypeContext) -> ParsedSyntax {
    if !p.at(T![this]) {
        return Absent;
    }

    let parameter = p.start();
    p.expect(T![this]);
    parse_ts_type_annotation(p, context).ok();
    Present(parameter.complete(p, TS_THIS_PARAMETER))
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum ParameterContext {
    /// Regular parameter in a class method implementation: `class A { method(a) {} }`
    ClassImplementation,

    /// Regular parameter in a function / method implementation: `function x(a) {}`
    Implementation,

    /// Parameter of a function/method declaration: `declare function x(a);`
    Declaration,

    /// Parameter of a setter function: `set a(b: string)`
    Setter,

    /// Parameter of a class setter method: `class A { set a(b: string) }`
    ClassSetter,

    /// Parameter of an arrow function
    Arrow,

    /// Parameter inside a TS property parameter: `constructor(private a)`
    ParameterProperty,
}

impl ParameterContext {
    pub fn is_any_setter(&self) -> bool {
        self.is_setter() || self.is_class_setter()
    }

    pub fn is_setter(&self) -> bool {
        self == &ParameterContext::Setter
    }

    pub fn is_class_setter(&self) -> bool {
        self == &ParameterContext::ClassSetter
    }

    pub fn is_class_method_implementation(&self) -> bool {
        self == &ParameterContext::ClassImplementation
    }

    pub fn is_any_class_method(&self) -> bool {
        self.is_class_method_implementation() || self.is_class_setter()
    }

    pub fn is_parameter_property(&self) -> bool {
        self == &ParameterContext::ParameterProperty
    }

    pub fn is_arrow_function(&self) -> bool {
        self == &ParameterContext::Arrow
    }
}

// test ts ts_formal_parameter
// function a(x) {}
// function b({ x, y } = {}) {}
// function c(x: string, y?: number, z: string = "test") {}
//
// test_err ts ts_formal_parameter_error
// function a(x?: string = "test") {}
// function b(...rest: string[] = "init") {}
// function c(...rest, b: string) {}
//
// test_err js js_formal_parameter_error
// function a(x: string) {}
// function b(x?) {}
pub(crate) fn parse_formal_parameter(
    p: &mut JsParser,
    decorator_list: ParsedSyntax,
    parameter_context: ParameterContext,
    expression_context: ExpressionContext,
    type_context: TypeContext,
) -> ParsedSyntax {
    // test ts ts_formal_parameter_decorator { "parse_class_parameter_decorators": true }
    // class Foo {
    //    constructor(@dec x) {}
    //    method(@dec x) {}
    // }

    // test_err ts ts_formal_parameter_decorator_option
    // class Foo {
    //    constructor(@dec x) {}
    //    method(@dec x) {}
    // }

    // test_err ts ts_formal_parameter_decorator { "parse_class_parameter_decorators": true }
    // function a(@dec x) {}

    // we use a checkpoint to avoid bogus nodes if the binding pattern fails to parse.
    let checkpoint = p.checkpoint();

    let m = decorator_list
        .or_else(|| empty_decorator_list(p))
        .precede(p);

    if let Present(binding) = parse_binding_pattern(p, expression_context) {
        let binding_kind = binding.kind(p);
        let binding_range = binding.range(p);

        let mut valid = true;

        let is_optional = if p.at(T![?]) {
            if TypeScript.is_unsupported(p) {
                p.error(ts_only_syntax_error(
                    p,
                    "optional parameters",
                    p.cur_range(),
                ));
                valid = false;
            } else if parameter_context.is_any_setter() {
                p.error(p.err_builder(
                    "A 'set' accessor cannot have an optional parameter.",
                    p.cur_range(),
                ));
                valid = false;
            }

            p.bump(T![?]);
            true
        } else {
            false
        };

        // test ts ts_parameter_option_binding_pattern
        // declare namespace A {
        //   export class Ajv {
        //     errorsText(errors?: string[] | null | undefined, { separator, dataVar }?: ErrorsTextOptions): string;
        //   }
        // }
        if valid
            && matches!(
                binding_kind,
                JS_OBJECT_BINDING_PATTERN | JS_ARRAY_BINDING_PATTERN
            )
            && parameter_context.is_parameter_property()
        {
            valid = false;
            p.error(p.err_builder(
                "A parameter property may not be declared using a binding pattern.",
                binding_range,
            ));
        }

        TypeScript
            .parse_exclusive_syntax(
                p,
                |p| parse_ts_type_annotation(p, type_context),
                |p, annotation| ts_only_syntax_error(p, "Type annotations", annotation.range(p)),
            )
            .ok();

        if let Present(initializer) = parse_initializer_clause(p, expression_context) {
            if valid && parameter_context.is_any_setter() && TypeScript.is_supported(p) {
                p.error(p.err_builder(
                    "A 'set' accessor parameter cannot have an initializer.",
                    initializer.range(p),
                ));
            } else if is_optional && valid {
                p.error(p.err_builder(
                    "Parameter cannot have question mark and initializer",
                    initializer.range(p),
                ));
            }
        }

        let mut parameter = m.complete(p, JS_FORMAL_PARAMETER);

        if !valid {
            parameter.change_to_bogus(p);
        }

        Present(parameter)
    } else {
        m.abandon(p);
        p.rewind(checkpoint);
        Absent
    }
}

/// Skips over the binding token of a parameter. Useful in the context of lookaheads to determine
/// if any typescript specific syntax like `:` is present after the parameter name.
/// Returns `true` if the function skipped over a valid binding, returns false if the parser
/// is not positioned at a binding.
pub(super) fn skip_parameter_start(p: &mut JsParser) -> bool {
    if is_at_identifier_binding(p) || p.at(T![this]) {
        p.bump_any();
        return true;
    }

    if p.at(T!['[']) || p.at(T!['{']) {
        // Array or object pattern. Try to parse it and return true if there were no parsing errors
        let previous_error_count = p.context().diagnostics().len();
        let pattern = parse_binding_pattern(p, ExpressionContext::default());
        pattern.is_present() && p.context().diagnostics().len() == previous_error_count
    } else {
        false
    }
}

// test js parameter_list
// function evalInComputedPropertyKey({ [computed]: ignored }) {}
/// parse the whole list of parameters, brackets included
pub(super) fn parse_parameter_list(
    p: &mut JsParser,
    parameter_context: ParameterContext,
    type_context: TypeContext,
    flags: SignatureFlags,
) -> ParsedSyntax {
    if !p.at(T!['(']) {
        return Absent;
    }
    let m = p.start();
    parse_parameters_list(
        p,
        flags,
        |p, expression_context| {
            let decorator_list = parse_parameter_decorators(p);

            let decorator_list = if parameter_context.is_any_class_method() {
                // test_err ts ts_decorator_on_class_method
                // class A {
                //     method(@dec x, second, @dec third = 'default') {}
                //     method(@dec.fn() x, second, @dec.fn() third = 'default') {}
                //     method(@dec() x, second, @dec() third = 'default') {}
                //     static method(@dec x, second, @dec third = 'default') {}
                //     static method(@dec.fn() x, second, @dec.fn() third = 'default') {}
                //     static method(@dec() x, second, @dec() third = 'default') {}
                // }

                // test ts ts_decorator_on_class_method { "parse_class_parameter_decorators": true }
                // class A {
                //     method(@dec x, second, @dec third = 'default') {}
                //     method(@dec.fn() x, second, @dec.fn() third = 'default') {}
                //     method(@dec() x, second, @dec() third = 'default') {}
                //     static method(@dec x, second, @dec third = 'default') {}
                //     static method(@dec.fn() x, second, @dec.fn() third = 'default') {}
                //     static method(@dec() x, second, @dec() third = 'default') {}
                // }
                decorator_list
            } else {
                // test_err ts ts_decorator_on_function_declaration { "parse_class_parameter_decorators": true }
                // function method(@dec x, second, @dec third = 'default') {}
                // function method(@dec.fn() x, second, @dec.fn() third = 'default') {}
                // function method(@dec() x, second, @dec() third = 'default') {}

                // test_err ts ts_decorator_on_function_expression { "parse_class_parameter_decorators": true }
                // const expr = function method(@dec x, second, @dec third = 'default') {}
                // const expr = function method(@dec.fn() x, second, @dec.fn() third = 'default') {}
                // const expr = function method(@dec() x, second, @dec() third = 'default') {}

                // test_err ts ts_decorator_on_function_type { "parse_class_parameter_decorators": true }
                // type I = (@dec x, second, @dec third = 'default') => string;
                // type I = (@dec.fn() x, second, @dec.fn() third = 'default') => string;
                // type I = (@dec() x, second, @dec() third = 'default') => string;

                // test_err ts ts_decorator_on_constructor_type { "parse_class_parameter_decorators": true }
                // type I = new(@dec x, second, @dec third = 'default') => string;
                // type I = abstract new(@dec.fn() x, second, @dec.fn() third = 'default') => string;
                // type I = abstract new(@dec() x, second, @dec() third = 'default') => string;

                // test_err ts ts_decorator_on_signature_member { "parse_class_parameter_decorators": true }
                // type A = {new (@dec x, second, @dec third = 'default'): string; }
                // type B = {method(@dec.fn() x, second, @dec.fn() third = 'default'): string; }
                // type C = {
                //  new(@dec() x, second, @dec() third = 'default'): string;
                //	method(@dec.fn() x, second, @dec.fn() third = 'default'): string;
                // }

                // test_err ts ts_decorator_on_ambient_function { "parse_class_parameter_decorators": true }
                // declare module a {
                // 		function method(@dec x, second, @dec third = 'default') {}
                // 		function method(@dec.fn() x, second, @dec.fn() third = 'default') {}
                // 		function method(@dec() x, second, @dec() third = 'default') {}
                // }
                // declare function method(@dec x, second, @dec third = 'default')
                // declare function method(@dec.fn() x, second, @dec.fn() third = 'default')
                // declare function method(@dec() x, second, @dec() third = 'default')
                decorator_list
                    .add_diagnostic_if_present(p, decorators_not_allowed)
                    .map(|mut decorator_list| {
                        decorator_list.change_to_bogus(p);
                        decorator_list
                    })
                    .into()
            };

            parse_any_parameter(
                p,
                decorator_list,
                parameter_context,
                expression_context,
                type_context,
            )
        },
        JS_PARAMETER_LIST,
    );

    Present(m.complete(p, JS_PARAMETERS))
}

/// Parses a (param, param) list into the current active node
pub(super) fn parse_parameters_list(
    p: &mut JsParser,
    flags: SignatureFlags,
    parse_parameter: impl Fn(&mut JsParser, ExpressionContext) -> ParsedSyntax,
    list_kind: JsSyntaxKind,
) {
    let mut first = true;
    let has_l_paren = p.expect(T!['(']);

    p.with_state(EnterParameters(flags), |p| {
        let parameters_list = p.start();
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

            if parse_metavariable(p).is_present() {
                continue;
            }

            let parameter = parse_parameter(
                p,
                ExpressionContext::default().and_object_expression_allowed(!first || has_l_paren),
            );

            if parameter.is_absent() && p.at(T![,]) {
                // a missing parameter,
                parameter.or_add_diagnostic(p, expected_parameter);
                continue;
            }

            // test_err js formal_params_no_binding_element
            // function foo(true) {}

            // test_err js formal_params_invalid
            // function (a++, c) {}
            let recovered_result = parameter.or_recover_with_token_set(
                p,
                &ParseRecoveryTokenSet::new(
                    JS_BOGUS_PARAMETER,
                    token_set![
                        T![ident],
                        T![await],
                        T![yield],
                        T![this],
                        T![,],
                        T!['['],
                        T![...],
                        T!['{'],
                        T![')'],
                        T![;],
                    ],
                )
                .enable_recovery_on_line_break(),
                js_parse_error::expected_parameter,
            );

            if recovered_result.is_err() {
                break;
            }
        }

        parameters_list.complete(p, list_kind);
    });

    p.expect(T![')']);
}
