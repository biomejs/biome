use crate::parser::rewrite_parser::{RewriteMarker, RewriteParser, RewriteToken};
use crate::parser::JsParserCheckpoint;
use crate::prelude::*;
use crate::rewrite::{rewrite_events, RewriteParseEvents};
use crate::syntax::class::parse_initializer_clause;
use crate::syntax::expr::{
    is_at_identifier, parse_conditional_expr, parse_unary_expr, ExpressionContext,
};
use crate::syntax::js_parse_error::{
    expected_assignment_target, expected_identifier, expected_object_member_name,
    invalid_assignment_error,
};
use crate::syntax::object::{is_at_object_member_name, parse_object_member_name};
use crate::syntax::pattern::{ParseArrayPattern, ParseObjectPattern, ParseWithDefaultPattern};
use crate::JsParser;
use crate::ParsedSyntax::{Absent, Present};
use biome_js_syntax::{JsSyntaxKind::*, *};
use biome_parser::diagnostic::expected_any;
use biome_parser::parse_recovery::ParseRecoveryTokenSet;
use biome_rowan::AstNode;

use super::metavariable::is_at_metavariable;

// test js assignment_target
// foo += bar = b ??= 3;
// a.foo -= bar;
// (foo = bar);
// (((foo))) = bar;
// a["test"] = bar;
// a.call().chain().member = x;
// ++count === 3
// a['b'] = c[d] = "test"

// test_err js invalid_assignment_target
// ++a = b;
// (++a) = b;
// (a = b;
// a?.b = b;
// a?.["b"] = b;
// (a +) = b;

// test ts ts_non_null_assignment
// let a;
// a! &= 2;
// let b = { a: null };
// b.a! &= 5

// test ts ts_as_assignment
// let a: any;
// type B<A> = { a: A };
// (a as string) = "string";
// ((a as any) as string) = null;
// ({ b: a as string } = { b: "test" });
// ([ a as string ] = [ "test" ]);
// for (a as string in []) {}
// (a as B<string>) = { a: "test" };
// (<number> a) += 1

// test_err ts ts_as_assignment_no_parenthesize
// let a: any;
// a as string = "string";
// (a() as string) = "string";
// <number> a = 3;

// test ts ts_satisfies_assignment
// let a: any;
// type B<A> = { a: A };
// (a satisfies string) = "string";
// ((a satisfies any) satisfies string) = null;
// ({ b: a satisfies string } = { b: "test" });
// ([ a satisfies string ] = [ "test" ]);
// for (a satisfies string in []) {}
// (a satisfies B<string>) = { a: "test" };

// test_err ts ts_satisfies_assignment_no_parenthesize
// let a: any;
// a satisfies string = "string";
// (a() satisfies string) = "string";

/// Converts the passed in lhs expression to an assignment pattern
/// The passed checkpoint allows to restore the parser to the state before it started parsing the expression.
pub(crate) fn expression_to_assignment_pattern(
    p: &mut JsParser,
    target: CompletedMarker,
    checkpoint: JsParserCheckpoint,
) -> ParsedSyntax {
    match target.kind(p) {
        JS_OBJECT_EXPRESSION => {
            p.rewind(checkpoint);
            ObjectAssignmentPattern.parse_object_pattern(p)
        }
        JS_ARRAY_EXPRESSION => {
            p.rewind(checkpoint);
            ArrayAssignmentPattern.parse_array_pattern(p)
        }
        _ => ParsedSyntax::Present(expression_to_assignment(p, target, checkpoint)),
    }
}

// test js array_or_object_member_assignment
// [{
//   get y() {
//     throw new Test262Error('The property should not be accessed.');
//   },
//   set y(val) {
//     setValue = val;
//   }
// }.y = 42] = [23];
// ({ x: {
//   get y() {
//     throw new Test262Error('The property should not be accessed.');
//   },
//   set y(val) {
//     setValue = val;
//   }
// }.y = 42 } = { x: 23 });
pub(crate) fn parse_assignment_pattern(p: &mut JsParser) -> ParsedSyntax {
    let checkpoint = p.checkpoint();
    let assignment_expression = parse_conditional_expr(p, ExpressionContext::default());

    assignment_expression
        .and_then(|expression| expression_to_assignment_pattern(p, expression, checkpoint))
}

/// Re-parses an expression as an assignment.
pub(crate) fn expression_to_assignment(
    p: &mut JsParser,
    target: CompletedMarker,
    checkpoint: JsParserCheckpoint,
) -> CompletedMarker {
    try_expression_to_assignment(p, target, checkpoint).unwrap_or_else(
        // test_err js js_regex_assignment
        // /=0*_:m/=/*_:|
        |mut invalid_assignment_target| {
            // Doesn't seem to be a valid assignment target. Recover and create an error.
            invalid_assignment_target.change_kind(p, JS_BOGUS_ASSIGNMENT);

            p.error(invalid_assignment_error(
                p,
                invalid_assignment_target.range(p),
            ));

            invalid_assignment_target
        },
    )
}

pub(crate) enum AssignmentExprPrecedence {
    Unary,
    Conditional,
}

impl AssignmentExprPrecedence {
    fn parse_expression(&self, p: &mut JsParser, context: ExpressionContext) -> ParsedSyntax {
        match self {
            AssignmentExprPrecedence::Unary => parse_unary_expr(p, context),
            AssignmentExprPrecedence::Conditional => parse_conditional_expr(p, context),
        }
    }
}

pub(crate) fn parse_assignment(
    p: &mut JsParser,
    expr_kind: AssignmentExprPrecedence,
    context: ExpressionContext,
) -> ParsedSyntax {
    let checkpoint = p.checkpoint();
    let assignment_expression = expr_kind.parse_expression(p, context);

    assignment_expression.map(|expr| expression_to_assignment(p, expr, checkpoint))
}

struct ArrayAssignmentPatternElement;

impl ParseWithDefaultPattern for ArrayAssignmentPatternElement {
    #[inline]
    fn pattern_with_default_kind() -> JsSyntaxKind {
        JS_ARRAY_ASSIGNMENT_PATTERN_ELEMENT
    }

    #[inline]
    fn expected_pattern_error(p: &JsParser, range: TextRange) -> ParseDiagnostic {
        expected_assignment_target(p, range)
    }

    #[inline]
    fn parse_pattern(&self, p: &mut JsParser) -> ParsedSyntax {
        parse_assignment_pattern(p)
    }
}

struct ArrayAssignmentPattern;

// test js array_assignment_target
// [foo, bar] = baz;
// [,,,b,,c,] = baz;
// [a = "test", a.b, call().b] = baz;
// [((a))] = baz;
//
// test_err js array_assignment_target_err
// [a a, ++b, ] = test;
// [a, c, ...rest,] = test;
// [a = , = "test"] = test;
// [[a b] [c]]= test;
// [a: b] = c
impl ParseArrayPattern<ArrayAssignmentPatternElement> for ArrayAssignmentPattern {
    #[inline]
    fn bogus_pattern_kind() -> JsSyntaxKind {
        JS_BOGUS_ASSIGNMENT
    }

    #[inline]
    fn array_pattern_kind() -> JsSyntaxKind {
        JS_ARRAY_ASSIGNMENT_PATTERN
    }

    // test js array_assignment_target_rest
    // ([ ...abcd ] = a);
    // ([ ...(abcd) ] = a);
    // ([ ...m.test ] = c);
    // ([ ...m[call()] ] = c);
    // ([ ...any.expression().b ] = c);
    // ([ ...[x, y] ] = b);
    // ([ ...[ ...a ] ] = c);
    //
    // test_err js array_assignment_target_rest_err
    // ([ ... ] = a);
    // ([ ...c = "default" ] = a);
    // ([ ...rest, other_assignment ] = a);
    #[inline]
    fn rest_pattern_kind() -> JsSyntaxKind {
        JS_ARRAY_ASSIGNMENT_PATTERN_REST_ELEMENT
    }

    fn list_kind() -> JsSyntaxKind {
        JS_ARRAY_ASSIGNMENT_PATTERN_ELEMENT_LIST
    }

    #[inline]
    fn expected_element_error(p: &JsParser, range: TextRange) -> ParseDiagnostic {
        expected_any(&["assignment target", "rest element", "comma"], range, p)
    }

    #[inline]
    fn pattern_with_default(&self) -> ArrayAssignmentPatternElement {
        ArrayAssignmentPatternElement
    }
}

struct ObjectAssignmentPattern;

// test js object_assignment_target
// ({} = {});
// ({ bar, baz } = {});
// ({ bar: [baz = "baz"], foo = "foo", ...rest } = {});
impl ParseObjectPattern for ObjectAssignmentPattern {
    #[inline]
    fn bogus_pattern_kind() -> JsSyntaxKind {
        JS_BOGUS_ASSIGNMENT
    }

    #[inline]
    fn object_pattern_kind() -> JsSyntaxKind {
        JS_OBJECT_ASSIGNMENT_PATTERN
    }

    fn list_kind() -> JsSyntaxKind {
        JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY_LIST
    }

    #[inline]
    fn expected_property_pattern_error(p: &JsParser, range: TextRange) -> ParseDiagnostic {
        expected_any(&["assignment target", "rest property"], range, p)
    }

    // test js property_assignment_target
    // ({x}= {});
    // ({x: y}= {});
    // ({x: y.test().z}= {});
    // ({x: ((z))}= {});
    // ({x: z["computed"]}= {});
    // ({x = "default"}= {});
    // ({x: y = "default"}= {});
    // ({0: y, [computed]: z} = {});
    //
    // test_err js property_assignment_target_err
    // ({:y} = {});
    // ({=y} = {});
    // ({:="test"} = {});
    // ({:=} = {});
    // ({ a b } = {});
    fn parse_property_pattern(&self, p: &mut JsParser) -> ParsedSyntax {
        let m = p.start();

        let kind = if (is_at_identifier(p) || is_at_metavariable(p) || p.at(T![=]))
            && !p.nth_at(1, T![:])
        {
            parse_assignment(
                p,
                AssignmentExprPrecedence::Conditional,
                ExpressionContext::default(),
            )
            .or_add_diagnostic(p, expected_identifier);
            JS_OBJECT_ASSIGNMENT_PATTERN_SHORTHAND_PROPERTY
        } else if is_at_object_member_name(p) || p.at(T![:]) || p.nth_at(1, T![:]) {
            // If the parser is at an object member name, parse it and look for the colon token next.
            // If `p.nth_at(1, T![:])` is true, it indicates an invalid object member name before the colon,
            // such as `{%: 1}`. In this case, attempt to recover by finding the next colon token.
            parse_object_member_name(p)
                .or_recover_with_token_set(
                    p,
                    &ParseRecoveryTokenSet::new(JS_BOGUS, token_set![T![:]])
                        .enable_recovery_on_line_break(),
                    expected_object_member_name,
                )
                .ok();
            p.expect(T![:]);
            parse_assignment_pattern(p).or_add_diagnostic(p, expected_assignment_target);
            JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY
        } else {
            m.abandon(p);
            return Absent;
        };

        parse_initializer_clause(p, ExpressionContext::default()).ok();

        Present(m.complete(p, kind))
    }

    // test js rest_property_assignment_target
    // ({ ...abcd } = a);
    // ({ ...(abcd) } = a);
    // ({ ...m.test } = c);
    // ({ ...m[call()] } = c);
    // ({ ...any.expression().b } = c);
    // ({ b: { ...a } } = c);
    //
    // test_err js rest_property_assignment_target_err
    // ({ ... } = a);
    // ({ ...c = "default" } = a);
    // ({ ...{a} } = b);
    // ({ ...rest, other_assignment } = a);
    // ({ ...rest, } = a);
    fn parse_rest_property_pattern(&self, p: &mut JsParser) -> ParsedSyntax {
        if !p.at(T![...]) {
            return Absent;
        }

        let m = p.start();
        p.bump(T![...]);

        let target = parse_assignment_pattern(p).or_add_diagnostic(p, expected_assignment_target);

        if let Some(mut target) = target {
            if matches!(
                target.kind(p),
                JS_OBJECT_ASSIGNMENT_PATTERN | JS_ARRAY_ASSIGNMENT_PATTERN
            ) {
                target.change_kind(p, JS_BOGUS_ASSIGNMENT);
                p.error(p.err_builder(
                    "object and array assignment targets are not allowed in rest patterns",
                    target.range(p),
                ));
            }
        }

        Present(m.complete(p, JS_OBJECT_ASSIGNMENT_PATTERN_REST))
    }
}

fn try_expression_to_assignment(
    p: &mut JsParser,
    target: CompletedMarker,
    checkpoint: JsParserCheckpoint,
) -> Result<CompletedMarker, CompletedMarker> {
    if !matches!(
        target.kind(p),
        JS_PARENTHESIZED_EXPRESSION
            | JS_STATIC_MEMBER_EXPRESSION
            | JS_COMPUTED_MEMBER_EXPRESSION
            | JS_IDENTIFIER_EXPRESSION
            | TS_NON_NULL_ASSERTION_EXPRESSION
            | TS_TYPE_ASSERTION_EXPRESSION
            | TS_AS_EXPRESSION
            | TS_SATISFIES_EXPRESSION
    ) {
        return Err(target);
    }

    // At this point it's guaranteed that the root node can be mapped to an assignment,
    // but it's not yet guaranteed if it is valid or not (for example, a static member expression
    // is valid, except if it uses optional chaining).
    let mut reparse_assignment = ReparseAssignment::new();
    rewrite_events(&mut reparse_assignment, checkpoint, p);

    Ok(reparse_assignment.result.unwrap())
}

struct ReparseAssignment {
    // Stores the unfinished parents
    // Index 0: Re-mapped kind of the node
    // Index 1: Started marker. A `None` marker means that this node should be dropped
    //          from the re-written tree
    parents: Vec<(JsSyntaxKind, Option<RewriteMarker>)>,
    // Stores the completed assignment node (valid or invalid).
    result: Option<CompletedMarker>,
    // Tracks if the visitor is still inside an assignment
    inside_assignment: bool,
}

impl ReparseAssignment {
    pub fn new() -> Self {
        Self {
            parents: Vec::default(),
            result: None,
            inside_assignment: true,
        }
    }
}

/// Rewrites expressions to assignments
/// * Converts parenthesized expression to parenthesized assignment
/// * Converts computed/static member expressions to computed/static member assignment.
///   Validates that the operator isn't `?.` .
/// * Converts identifier expressions to identifier assignment, drops the inner reference identifier
impl RewriteParseEvents for ReparseAssignment {
    fn start_node(&mut self, kind: JsSyntaxKind, p: &mut RewriteParser) {
        if !self.inside_assignment {
            self.parents.push((kind, Some(p.start())));
            return;
        }

        // Make sure to also add the kind to the match in `try_expression_to_assignment`
        let mapped_kind = match kind {
            JS_PARENTHESIZED_EXPRESSION => JS_PARENTHESIZED_ASSIGNMENT,
            JS_STATIC_MEMBER_EXPRESSION => {
                self.inside_assignment = false;
                JS_STATIC_MEMBER_ASSIGNMENT
            }
            JS_COMPUTED_MEMBER_EXPRESSION => {
                self.inside_assignment = false;
                JS_COMPUTED_MEMBER_ASSIGNMENT
            }
            JS_IDENTIFIER_EXPRESSION => JS_IDENTIFIER_ASSIGNMENT,
            TS_NON_NULL_ASSERTION_EXPRESSION => TS_NON_NULL_ASSERTION_ASSIGNMENT,
            TS_AS_EXPRESSION => TS_AS_ASSIGNMENT,
            TS_SATISFIES_EXPRESSION => TS_SATISFIES_ASSIGNMENT,
            TS_TYPE_ASSERTION_EXPRESSION => TS_TYPE_ASSERTION_ASSIGNMENT,
            JS_REFERENCE_IDENTIFIER => {
                self.parents.push((kind, None)); // Omit reference identifiers
                return;
            }
            _ => {
                self.inside_assignment = false;
                if AnyTsType::can_cast(kind)
                    && matches!(
                        self.parents.last(),
                        Some((
                            TS_AS_ASSIGNMENT
                                | TS_SATISFIES_ASSIGNMENT
                                | TS_TYPE_ASSERTION_ASSIGNMENT,
                            _
                        ))
                    )
                {
                    kind
                } else {
                    JS_BOGUS_ASSIGNMENT
                }
            }
        };

        self.parents.push((mapped_kind, Some(p.start())));
    }

    fn finish_node(&mut self, p: &mut RewriteParser) {
        let (kind, m) = self.parents.pop().unwrap();

        if let Some(m) = m {
            let mut completed = m.complete(p, kind);

            match kind {
                JS_IDENTIFIER_ASSIGNMENT => {
                    // test_err js eval_arguments_assignment
                    // eval = "test";
                    // arguments = "test";
                    let name = completed.text(p);
                    if matches!(name, "eval" | "arguments") && p.is_strict_mode() {
                        let error = p.err_builder(
                            format!("Illegal use of `{name}` as an identifier in strict mode"),
                            completed.range(p),
                        );
                        p.error(error);

                        completed.change_to_bogus(p);
                    }
                }
                JS_BOGUS_ASSIGNMENT => {
                    let range = completed.range(p);
                    p.error(
                        p.err_builder(
                            format!("Invalid assignment to `{}`", completed.text(p)),
                            range,
                        )
                        .with_hint("This expression cannot be assigned to"),
                    );
                }
                _ => {}
            }

            self.result = Some(completed.into());
        }

        if AnyTsType::can_cast(kind)
            && matches!(
                self.parents.last(),
                Some((
                    TS_TYPE_ASSERTION_ASSIGNMENT | TS_AS_ASSIGNMENT | TS_SATISFIES_ASSIGNMENT,
                    _
                ))
            )
        {
            self.inside_assignment = true;
        }
    }

    fn token(&mut self, token: RewriteToken, p: &mut RewriteParser) {
        let parent = self.parents.last_mut();

        if let Some((parent_kind, _)) = parent {
            if matches!(
                *parent_kind,
                JS_COMPUTED_MEMBER_ASSIGNMENT | JS_STATIC_MEMBER_ASSIGNMENT
            ) && token.kind == T![?.]
            {
                *parent_kind = JS_BOGUS_ASSIGNMENT
            }
        }

        p.bump(token)
    }
}
