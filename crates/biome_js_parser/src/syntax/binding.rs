use super::metavariable::{is_at_metavariable, is_nth_at_metavariable, parse_metavariable};
use crate::prelude::*;
use crate::span::Span;
use crate::syntax::class::parse_initializer_clause;
use crate::syntax::expr::{is_nth_at_identifier, parse_identifier, ExpressionContext};
use crate::syntax::js_parse_error::{
    expected_binding, expected_identifier, expected_object_member_name,
};
use crate::syntax::object::{is_at_object_member_name, parse_object_member_name};
use crate::syntax::pattern::{ParseArrayPattern, ParseObjectPattern, ParseWithDefaultPattern};
use crate::JsSyntaxFeature::StrictMode;
use crate::ParsedSyntax::{Absent, Present};
use crate::{JsParser, ParsedSyntax};
use biome_js_syntax::{JsSyntaxKind::*, *};
use biome_parser::diagnostic::expected_any;
use biome_rowan::SyntaxKind as SyntaxKindTrait;

pub(crate) fn parse_binding_pattern(p: &mut JsParser, context: ExpressionContext) -> ParsedSyntax {
    match p.cur() {
        T!['['] => ArrayBindingPattern.parse_array_pattern(p),
        T!['{'] if context.is_object_expression_allowed() => {
            ObjectBindingPattern.parse_object_pattern(p)
        }
        _ => parse_identifier_binding(p),
    }
}

#[inline]
pub(crate) fn is_at_identifier_binding(p: &mut JsParser) -> bool {
    is_nth_at_identifier_binding(p, 0)
}

#[inline]
pub(crate) fn is_nth_at_identifier_binding(p: &mut JsParser, n: usize) -> bool {
    is_nth_at_identifier(p, n) || is_nth_at_metavariable(p, n)
}

#[inline]
pub(crate) fn parse_binding(p: &mut JsParser) -> ParsedSyntax {
    parse_identifier_binding(p)
}

// test_err js binding_identifier_invalid
// async () => { let await = 5; }
// function *foo() {
//    let yield = 5;
// }
// let eval = 5;
// let let = 5;
// const let = 5;
// let a, a;
//
// test_err js binding_identifier_invalid_script
// // SCRIPT
// let let = 5;
// const let = 5;
/// Parses an identifier binding or returns an invalid syntax if the identifier isn't valid in this context.
/// An identifier may not be valid if:
/// * it is named "eval" or "arguments" inside of strict mode
/// * it is named "let" inside of a "let" or "const" declaration
/// * the same identifier is bound multiple times inside of a `let` or const` declaration
/// * it is named "yield" inside of a generator function or in strict mode
/// * it is named "await" inside of an async function
pub(crate) fn parse_identifier_binding(p: &mut JsParser) -> ParsedSyntax {
    if is_at_metavariable(p) {
        return parse_metavariable(p);
    }

    let parsed = parse_identifier(p, JS_IDENTIFIER_BINDING);

    parsed.map(|mut identifier| {
        if identifier.kind(p).is_bogus() {
            return identifier;
        }

        let identifier_name = identifier.text(p);

        if StrictMode.is_supported(p) && matches!(identifier_name, "eval" | "arguments") {
            let err = p.err_builder(
                format!(
                    "Illegal use of `{identifier_name}` as an identifier in strict mode"
                ),
                identifier.range(p),
            );
            p.error(err);

            identifier.change_to_bogus(p);
            return identifier;
        }

        if let Some(parent) = p.state().duplicate_binding_parent {
            if identifier_name == "let" {
                let err = p
                    .err_builder(
                        format!(
                        "`let` cannot be declared as a variable name inside of a `{parent}` declaration",

                    ),
                        identifier.range(p),
                    )
                    .with_hint("Rename the let identifier here");

                p.error(err);
                identifier.change_to_bogus(p);
                return identifier;
            }

            if let Some(existing) = p.state().name_map.get(identifier_name) {
                let err = p
                    .err_builder(
                        format!(
                            "Declarations inside of a `{parent}` declaration may not have duplicates"
                        ),
                        identifier.range(p),
                    )
                    .with_detail(
                        identifier.range(p),
                        format!(
                            "a second declaration of `{identifier_name}` is not allowed"
                        ),
                    )
                    .with_detail(
                        *existing,
                        format!("`{identifier_name}` is first declared here"),
                    );
                p.error(err);
                identifier.change_to_bogus(p);
                return identifier;
            }

            let identifier_name = String::from(identifier_name);
            let identifier_range = identifier.range(p);
            p.state_mut()
                .name_map
                .insert(identifier_name, identifier_range.as_range());
        }

        identifier
    })
}

struct ArrayBindingPatternWithDefault;

impl ParseWithDefaultPattern for ArrayBindingPatternWithDefault {
    #[inline]
    fn pattern_with_default_kind() -> JsSyntaxKind {
        JS_ARRAY_BINDING_PATTERN_ELEMENT
    }

    #[inline]
    fn expected_pattern_error(p: &JsParser, range: TextRange) -> ParseDiagnostic {
        expected_binding(p, range)
    }

    #[inline]
    fn parse_pattern(&self, p: &mut JsParser) -> ParsedSyntax {
        parse_binding_pattern(p, ExpressionContext::default())
    }
}

struct ArrayBindingPattern;

// test js array_binding
// let a = "b";
// let [c, b] = [1, 2];
// let [d, ...abcd] = [1];
// let [e = "default", x] = []
// let [, f, ...rest] = []
// let [[...rest2], { g }] = []
//
// test_err js array_binding_err
// let [a b] = [1, 2];
// let [="default"] = [1, 2];
// let ["default"] = [1, 2];
// let [[c ] = [];
//
// test js array_binding_rest
// let [ ...abcd ] = a;
// let [ ...[x, y] ] = b;
// let [ ...[ ...a ] ] = c;
//
// test_err js array_binding_rest_err
// let [ ... ] = a;
// let [ ...c = "default" ] = a;
// let [ ...rest, other_assignment ] = a;
impl ParseArrayPattern<ArrayBindingPatternWithDefault> for ArrayBindingPattern {
    #[inline]
    fn bogus_pattern_kind() -> JsSyntaxKind {
        JS_BOGUS_BINDING
    }

    #[inline]
    fn array_pattern_kind() -> JsSyntaxKind {
        JS_ARRAY_BINDING_PATTERN
    }

    #[inline]
    fn rest_pattern_kind() -> JsSyntaxKind {
        JS_ARRAY_BINDING_PATTERN_REST_ELEMENT
    }

    fn list_kind() -> JsSyntaxKind {
        JS_ARRAY_BINDING_PATTERN_ELEMENT_LIST
    }

    #[inline]
    fn expected_element_error(p: &JsParser, range: TextRange) -> ParseDiagnostic {
        expected_any(
            &[
                "identifier",
                "object pattern",
                "array pattern",
                "rest pattern",
            ],
            range,
            p,
        )
    }

    #[inline]
    fn pattern_with_default(&self) -> ArrayBindingPatternWithDefault {
        ArrayBindingPatternWithDefault
    }
}

// test_err js object_binding_pattern
// let { 5 } } = { eval: "foo" };
// let { eval } = { eval: "foo" };
// let { 5, 6 } = { eval: "foo" };
// let { default , eval: } = {};
struct ObjectBindingPattern;

impl ParseObjectPattern for ObjectBindingPattern {
    #[inline]
    fn bogus_pattern_kind() -> JsSyntaxKind {
        JS_BOGUS_BINDING
    }

    #[inline]
    fn object_pattern_kind() -> JsSyntaxKind {
        JS_OBJECT_BINDING_PATTERN
    }

    fn list_kind() -> JsSyntaxKind {
        JS_OBJECT_BINDING_PATTERN_PROPERTY_LIST
    }

    #[inline]
    fn expected_property_pattern_error(p: &JsParser, range: TextRange) -> ParseDiagnostic {
        expected_any(&["identifier", "member name", "rest pattern"], range, p)
    }

    // test js object_property_binding
    // let { foo: bar  } = {}
    // let { foo: bar_bar = baz } = {}
    //
    // test_err js object_property_binding_err
    // let { foo: , bar } = {}
    // let { : lorem = "test" } = {}
    // let { , ipsum: bazz } = {}
    //
    // test js object_shorthand_property
    // let { a, b } = c
    // let { d = "default", e = call() } = c
    //
    // test_err js object_shorthand_property_err
    // let { a b } = c
    // let { = "test" } = c
    // let { , d } = c
    fn parse_property_pattern(&self, p: &mut JsParser) -> ParsedSyntax {
        if !is_at_object_member_name(p)
            && !is_at_metavariable(p)
            && !p.at_ts(token_set![T![:], T![=]])
        {
            return Absent;
        }

        let m = p.start();

        let kind = if p.at(T![=])
            || ((is_at_identifier_binding(p) || is_at_metavariable(p)) && !p.nth_at(1, T![:]))
        {
            parse_binding(p).or_add_diagnostic(p, expected_identifier);
            JS_OBJECT_BINDING_PATTERN_SHORTHAND_PROPERTY
        } else {
            parse_object_member_name(p).or_add_diagnostic(p, expected_object_member_name);
            if p.expect(T![:]) {
                parse_binding_pattern(p, ExpressionContext::default())
                    .or_add_diagnostic(p, expected_binding);
            }
            JS_OBJECT_BINDING_PATTERN_PROPERTY
        };

        // test js destructuring_initializer_binding
        // const { value, f = (value) => value } = item
        let parent = p.state_mut().duplicate_binding_parent.take();
        parse_initializer_clause(p, ExpressionContext::default()).ok();
        p.state_mut().duplicate_binding_parent = parent;

        Present(m.complete(p, kind))
    }

    // test js rest_property_binding
    // let { ...abcd } = a;
    // let { b: { ...a } } = c;
    //
    // test_err js rest_property_binding_err
    // let { ... } = a;
    // let { ...c = "default" } = a;
    // let { ...{a} } = b;
    // let { ...rest, other_assignment } = a;
    // let { ...rest2, } = a;
    // async function test() {
    //   let { ...await } = a;
    // }
    fn parse_rest_property_pattern(&self, p: &mut JsParser) -> ParsedSyntax {
        if p.at(T![...]) {
            let m = p.start();
            p.bump(T![...]);

            let inner = parse_binding_pattern(p, ExpressionContext::default())
                .or_add_diagnostic(p, expected_identifier);

            if let Some(mut inner) = inner {
                if inner.kind(p) != JS_IDENTIFIER_BINDING {
                    let inner_range = inner.range(p);
                    // Don't add multiple errors
                    if inner.kind(p) != JS_BOGUS_BINDING {
                        p.error(p.err_builder("Expected identifier binding", inner_range,).with_hint( "Object rest patterns must bind to an identifier, other patterns are not allowed."));
                    }

                    inner.change_kind(p, JS_BOGUS_BINDING);
                }
            }

            Present(m.complete(p, JS_OBJECT_BINDING_PATTERN_REST))
        } else {
            Absent
        }
    }
}
