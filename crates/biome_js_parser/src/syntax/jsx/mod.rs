use crate::prelude::*;
pub mod jsx_parse_errors;

use biome_js_syntax::JsSyntaxKind::*;
use biome_parser::diagnostic::expected_token;
use biome_parser::parse_lists::ParseNodeList;
use biome_rowan::TextRange;

use crate::lexer::{JsLexContext, JsReLexContext, JsSyntaxKind, T};
use crate::syntax::expr::{
    is_nth_at_identifier_or_keyword, parse_expression, parse_name, ExpressionContext,
};
use crate::syntax::js_parse_error::{expected_expression, expected_identifier};
use crate::syntax::jsx::jsx_parse_errors::{
    jsx_expected_attribute, jsx_expected_attribute_value, jsx_expected_children,
    jsx_expected_closing_tag,
};
use crate::syntax::typescript::TypeContext;
use crate::JsSyntaxFeature::TypeScript;
use crate::{parser::RecoveryResult, JsParser, ParseRecoveryTokenSet, ParsedSyntax};
use crate::{Absent, Present};

use super::typescript::parse_ts_type_arguments;

// test jsx jsx_element_on_return
// function f() {
//     return <div></div>
// }

// test jsx jsx_element_on_arrow_function
// const f = () => <div></div>;
// const f = () => (<div></div>);

// test jsx jsx_element_as_statements
// <div />

// test_err js jsx_or_type_assertion
// // SCRIPT
// function f() {
//     let a = <div>a</div>; // JSX
//     let b = <string>b; // type assertion
//     let c = <string>b<a>d; // type assertion
//     let d = <div>a</div>/; // ambiguous: JSX or "type assertion a less than regex /div>/". Probably JSX.
//     let d = <string>a</string>/;
// }

// test jsx jsx_equal_content
// <span></span>;
// <span>=</span>;
pub(crate) fn parse_jsx_tag_expression(p: &mut JsParser) -> ParsedSyntax {
    if !p.at(T![<]) {
        return Absent;
    }

    if !p.nth_at(1, T![>]) && !is_nth_at_identifier_or_keyword(p, 1) {
        return Absent;
    }

    let m = p.start();

    // Safety: Safe because `parse_any_jsx_tag only returns Absent if the parser isn't positioned
    // at the `<` token which is tested for at the beginning of the function.
    parse_any_jsx_tag(p, true).unwrap();
    Present(m.complete(p, JSX_TAG_EXPRESSION))
}

// <a ...> or <a ... />
// ^          ^

// test jsx jsx_element_open_close
// function f() {
//     return <div></div>
// }

// test jsx jsx_element_self_close
// function f() {
//     return <div />
// }

// test jsx jsx_closing_token_trivia
// <closing / /* some comment */ >;
// <open><
// /* some comment */ / open>;

// test_err jsx jsx_invalid_text
// <a> test ></a>;
// <b> invalid }</b>;

/// Parses a JSX tag (fragment or element)
///
/// `in_expression` must be `true` if this element is a direct child of the `JsxElementExpression` (root of an expression).
/// It should be false when parsing any child node.
fn parse_any_jsx_tag(p: &mut JsParser, in_expression: bool) -> ParsedSyntax {
    match parse_any_jsx_opening_tag(p, in_expression) {
        Some(OpeningElement::SelfClosing(marker)) => Present(marker),
        Some(OpeningElement::Fragment(fragment_opening)) => {
            let opening_range = fragment_opening.range(p);
            let fragment = fragment_opening.precede(p);
            parse_jsx_children(p);
            expect_closing_fragment(p, in_expression, opening_range);
            Present(fragment.complete(p, JSX_FRAGMENT))
        }
        Some(OpeningElement::Element { name, opening }) => {
            let opening_range = opening.range(p);
            let element = opening.precede(p);

            parse_jsx_children(p);

            expect_closing_element(p, in_expression, name, opening_range);
            Present(element.complete(p, JSX_ELEMENT))
        }
        None => Absent,
    }
}

enum OpeningElement {
    Fragment(CompletedMarker),
    Element {
        name: Option<CompletedMarker>,
        opening: CompletedMarker,
    },
    SelfClosing(CompletedMarker),
}

fn parse_any_jsx_opening_tag(p: &mut JsParser, in_expression: bool) -> Option<OpeningElement> {
    if !p.at(T![<]) {
        return None;
    }

    let m = p.start();
    p.bump(T![<]);

    if p.at(T![>]) {
        // test jsx jsx_fragments
        // <></>;
        // <>abcd</>;
        // <>   whitespace
        // </>;
        // <
        //   /*comment */
        //   >
        //   <
        //   /
        // >;
        p.bump_with_context(T![>], JsLexContext::JsxChild);

        return Some(OpeningElement::Fragment(
            m.complete(p, JSX_OPENING_FRAGMENT),
        ));
    }

    let name = parse_jsx_any_element_name(p).or_add_diagnostic(p, expected_identifier);

    // Don't parse type arguments in JS because it prevents us from doing better error recovery in case the
    // `>` token of the opening element is missing:
    // `<test <inner></test>` The `inner` is it's own element and not the type arguments
    if TypeScript.is_supported(p) {
        // test tsx tsx_element_generics_type
        // <NonGeneric />;
        // <Generic<true> />;
        // <Generic<true>></Generic>;
        let _ = parse_ts_type_arguments(p, TypeContext::default());
    }

    JsxAttributeList.parse_list(p);

    if p.eat(T![/]) {
        // test_err jsx jsx_self_closing_element_missing_r_angle
        // <><test / some test followed by<a /></>;
        expect_jsx_token(p, T![>], !in_expression);

        Some(OpeningElement::SelfClosing(
            m.complete(p, JSX_SELF_CLOSING_ELEMENT),
        ))
    } else {
        // test_err jsx jsx_opening_element_missing_r_angle
        // <><test <inner> some content</inner></test></>
        expect_jsx_token(p, T![>], true);

        Some(OpeningElement::Element {
            opening: m.complete(p, JSX_OPENING_ELEMENT),
            name,
        })
    }
}

fn expect_closing_fragment(
    p: &mut JsParser,
    in_expression: bool,
    opening_range: TextRange,
) -> CompletedMarker {
    let m = p.start();
    p.expect(T![<]);
    p.expect(T![/]);

    // test_err jsx jsx_missing_closing_fragment
    // <>test</test>;
    // <>test<inner> some text</inner>;
    if let Present(name) = parse_jsx_any_element_name(p) {
        p.error(
            p.err_builder(
                "JSX fragment has no corresponding closing tag.",
                opening_range,
            )
            .with_detail(opening_range, "Opening fragment")
            .with_detail(name.range(p), "Closing tag"),
        );
    }

    // test_err jsx jsx_fragment_closing_missing_r_angle
    // <div><>test</ 5 more content</div>
    expect_jsx_token(p, T![>], !in_expression);

    m.complete(p, JSX_CLOSING_FRAGMENT)
}

fn expect_closing_element(
    p: &mut JsParser,
    in_expression: bool,
    opening_name_marker: Option<CompletedMarker>,
    opening_range: TextRange,
) -> CompletedMarker {
    let m = p.start();

    p.expect(T![<]);
    p.expect(T![/]);

    let name_marker = parse_jsx_any_element_name(p);

    // test_err jsx jsx_closing_element_mismatch
    // <test></>;
    // <test></text>;
    // <some><nested></some></nested>;
    // <><5></test></>;
    if let Some(opening_name_marker) = opening_name_marker {
        let opening_name = opening_name_marker.text(p);

        let error = match name_marker {
            Present(name) if name.text(p) != opening_name => {
                let closing_end = if p.at(T![>]) {
                    p.cur_range().end()
                } else {
                    name.range(p).end()
                };

                let closing_range = TextRange::new(m.start(), closing_end);

                Some(jsx_expected_closing_tag(
                    p,
                    opening_name,
                    opening_range,
                    closing_range,
                ))
            }
            Present(_) => None,
            Absent => {
                if p.at(T![>]) {
                    let closing_range = TextRange::new(m.start(), p.cur_range().end());

                    Some(jsx_expected_closing_tag(
                        p,
                        opening_name,
                        opening_range,
                        closing_range,
                    ))
                } else {
                    Some(expected_identifier(p, p.cur_range()))
                }
            }
        };

        if let Some(error) = error {
            p.error(error);
        }
    }

    // test_err jsx jsx_closing_missing_r_angle
    // <><test>abcd</test more content follows here</>
    expect_jsx_token(p, T![>], !in_expression);

    m.complete(p, JSX_CLOSING_ELEMENT)
}

/// Expects a JSX token that may be followed by JSX child content.
/// Ensures that the child content is lexed with the [JsLexContext::JsxChild] context.
fn expect_jsx_token(p: &mut JsParser, token: JsSyntaxKind, before_child_content: bool) {
    if !before_child_content {
        p.expect(token);
    } else if p.at(token) {
        p.bump_with_context(token, JsLexContext::JsxChild);
    } else {
        p.error(expected_token(token));
        // Re-lex the current token as a JSX child.
        p.re_lex(JsReLexContext::JsxChild);
    }
}

struct JsxChildrenList;

impl ParseNodeList for JsxChildrenList {
    type Kind = JsSyntaxKind;
    type Parser<'source> = JsParser<'source>;
    const LIST_KIND: Self::Kind = JsSyntaxKind::JSX_CHILD_LIST;

    fn parse_element(&mut self, p: &mut JsParser) -> ParsedSyntax {
        match p.cur() {
            // test jsx jsx_element_children
            // <a>
            //     <b>
            //        <d></d>
            //        <e></e>
            //     </b>
            //     <c></c>
            // </a>
            T![<] => parse_any_jsx_tag(p, false),
            T!['{'] => parse_jsx_expression_child(p),
            // test jsx jsx_text
            // <a>test</a>;
            // <a>   whitespace handling </a>;
            // <a> multi
            //    line
            //          node
            // </a>;
            // <test>\u3333</test> // no error for invalid unicode escape
            JsSyntaxKind::JSX_TEXT_LITERAL => {
                let m = p.start();
                p.bump(JSX_TEXT_LITERAL);
                ParsedSyntax::Present(m.complete(p, JSX_TEXT))
            }
            _ => ParsedSyntax::Absent,
        }
    }

    fn is_at_list_end(&self, p: &mut JsParser) -> bool {
        let at_l_angle0 = p.at(T![<]);
        let at_slash1 = p.nth_at(1, T![/]);
        at_l_angle0 && at_slash1
    }

    fn recover(&mut self, p: &mut JsParser, parsed_element: ParsedSyntax) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(
                JsSyntaxKind::JS_BOGUS,
                token_set![T![<], T![>], T!['{'], T!['}']],
            ),
            jsx_expected_children,
        )
    }
}

#[inline]
fn parse_jsx_children(p: &mut JsParser) {
    JsxChildrenList.parse_list(p);
}

fn parse_jsx_expression_child(p: &mut JsParser) -> ParsedSyntax {
    if !p.at(T!['{']) {
        return ParsedSyntax::Absent;
    }

    let m = p.start();
    p.bump(T!['{']);

    // test jsx jsx_children_spread
    // <div>{...a}</div>;
    // <div>{...a}After</div>;
    let is_spread = p.eat(T![...]);

    let expr = parse_jsx_assignment_expression(p, is_spread);

    if is_spread {
        // test_err jsx jsx_spread_no_expression
        // <test>{...}</test>
        expr.or_add_diagnostic(p, expected_expression);
    }

    // test jsx jsx_children_expression_then_text
    // <test>
    //     {/* comment */}
    //      some
    //      text
    // </test>

    // test_err jsx jsx_children_expression_missing_r_curly
    // <test>
    //   { 5 + 3
    //   some text
    // </test>
    expect_jsx_token(p, T!['}'], true);

    let kind = if is_spread {
        JsSyntaxKind::JSX_SPREAD_CHILD
    } else {
        JsSyntaxKind::JSX_EXPRESSION_CHILD
    };

    ParsedSyntax::Present(m.complete(p, kind))
}

// test jsx jsx_member_element_name
// <a.b.c.d></a.b.c.d>;
// <a-b.c></a-b.c>;
// <Abcd></Abcd>;
//
// test_err jsx jsx_namespace_member_element_name
// <namespace:a></namespace:a>;
// <namespace:a.b></namespace:a.b>;
fn parse_jsx_any_element_name(p: &mut JsParser) -> ParsedSyntax {
    let name = parse_jsx_name_or_namespace(p);
    name.map(|mut name| {
        if name.kind(p) == JSX_NAME && (p.at(T![.]) || !is_intrinsic_element(name.text(p))) {
            name.change_kind(p, JSX_REFERENCE_IDENTIFIER)
        } else if name.kind(p) == JSX_NAMESPACE_NAME && p.at(T![.]) {
            let error = p.err_builder(
                "JSX property access expressions cannot include JSX namespace names.",
                name.range(p),
            );
            p.error(error);
            name.change_to_bogus(p);
        }

        while p.at(T![.]) {
            let m = name.precede(p);
            p.bump(T![.]);
            parse_name(p).or_add_diagnostic(p, expected_identifier);
            name = m.complete(p, JSX_MEMBER_NAME)
        }

        name
    })
}

/// Tests if this is an intrinsic element name. Intrinsic elements are such elements
/// that are built in, for example HTML elements. This implementation uses React's semantic
/// and assumes that anything starting with a lower case character is an intrinsic element, and
/// that custom components start with an upper case character.
///
/// Resources: [TypeScript's documentation on intrinsic elements](https://www.typescriptlang.org/docs/handbook/jsx.html#intrinsic-elements)
fn is_intrinsic_element(element_name: &str) -> bool {
    if let Some(first) = element_name.chars().next() {
        first.is_lowercase()
    } else {
        false
    }
}

// test jsx jsx_any_name
// <a-b-c-d-e></a-b-c-d-e>;
// <a-b-c-d-e />;
// <if />;
// <namespace:name></namespace:name>;
// <dashed-namespaced:dashed-name />;
fn parse_jsx_name_or_namespace(p: &mut JsParser) -> ParsedSyntax {
    parse_jsx_name(p).map(|identifier| {
        if p.at(T![:]) {
            let m = identifier.precede(p);
            p.bump(T![:]);
            parse_jsx_name(p).or_add_diagnostic(p, expected_identifier);
            m.complete(p, JSX_NAMESPACE_NAME)
        } else {
            identifier
        }
    })
}

fn parse_jsx_name(p: &mut JsParser) -> ParsedSyntax {
    p.re_lex(JsReLexContext::JsxIdentifier);

    if p.at(JSX_IDENT) {
        let name = p.start();
        p.bump(JSX_IDENT);
        Present(name.complete(p, JSX_NAME))
    } else {
        Absent
    }
}

struct JsxAttributeList;
// test jsx jsx_element_attributes
// function f() {
//     return <div string_literal="a" expression={1} novalue el=<a/>></div>;
// }
// <div dashed-name='test' use:validate="abcd" />;
// <div use-dashed_underscore:validate="ahaha" />;
// <div multiline-string='test
//   continues here' />;
// <div invalid-unicode-escape="\u10000\u20000" />;
impl ParseNodeList for JsxAttributeList {
    type Kind = JsSyntaxKind;
    type Parser<'source> = JsParser<'source>;
    const LIST_KIND: Self::Kind = JsSyntaxKind::JSX_ATTRIBUTE_LIST;

    fn parse_element(&mut self, p: &mut JsParser) -> ParsedSyntax {
        if matches!(p.cur(), T!['{'] | T![...]) {
            parse_jsx_spread_attribute(p)
        } else {
            parse_jsx_attribute(p)
        }
    }

    fn is_at_list_end(&self, p: &mut JsParser) -> bool {
        matches!(p.cur(), T![>] | T![/] | T![<])
    }

    fn recover(&mut self, p: &mut JsParser, parsed_element: ParsedSyntax) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(
                JsSyntaxKind::JS_BOGUS,
                token_set![T![/], T![>], T![<], T!['{'], T!['}'], T![...], T![ident]],
            ),
            jsx_expected_attribute,
        )
    }
}

fn parse_jsx_attribute(p: &mut JsParser) -> ParsedSyntax {
    if !is_nth_at_identifier_or_keyword(p, 0) {
        return Absent;
    }

    let m = p.start();

    // SAFETY: Guaranteed to succeed because the parser is at an identifier or keyword
    parse_jsx_name_or_namespace(p).unwrap();
    let _ = parse_jsx_attribute_initializer_clause(p);

    Present(m.complete(p, JsSyntaxKind::JSX_ATTRIBUTE))
}

// test jsx jsx_spread_attribute
// let obj = {};
// <a {...obj} />;
//
// test_err jsx jsx_spread_attribute_error
// let obj = {};
// <a {...obj, other} />;
// <a ...obj} />;
// <a {obj} />;
// <div
//       {...{} /*
//       // @ts-ignore */ /* prettier-ignore */
//       invalidProp="HelloWorld"
//     />;
fn parse_jsx_spread_attribute(p: &mut JsParser) -> ParsedSyntax {
    if !matches!(p.cur(), T![...] | T!['{']) {
        return Absent;
    }

    let m = p.start();

    p.expect(T!['{']);
    p.expect(T![...]);

    let argument = parse_expression(p, ExpressionContext::default()).map(|mut expr| {
        if expr.kind(p) == JS_SEQUENCE_EXPRESSION {
            p.error(p.err_builder(
                "Comma operator isn't a valid value for a JSX spread argument.",
                expr.range(p),
            ));
            expr.change_to_bogus(p);
        }

        expr
    });

    argument.or_add_diagnostic(p, expected_expression);

    p.expect(T!['}']);

    Present(m.complete(p, JSX_SPREAD_ATTRIBUTE))
}

fn parse_jsx_attribute_initializer_clause(p: &mut JsParser) -> ParsedSyntax {
    if !p.at(T![=]) {
        return Absent;
    }

    let m = p.start();

    p.bump_with_context(T![=], JsLexContext::JsxAttributeValue);

    // test_err jsx jsx_element_attribute_missing_value
    // function f() {
    //     return <div string_literal= ></div>;
    // }
    parse_jsx_attribute_value(p).or_add_diagnostic(p, jsx_expected_attribute_value);

    ParsedSyntax::Present(m.complete(p, JsSyntaxKind::JSX_ATTRIBUTE_INITIALIZER_CLAUSE))
}

fn parse_jsx_attribute_value(p: &mut JsParser) -> ParsedSyntax {
    match p.cur() {
        // test jsx jsx_element_attribute_expression
        // <div id={1} />;
        // <div className={prefix`none`} />;
        T!['{'] => parse_jsx_expression_attribute_value(p),
        // test jsx jsx_element_attribute_element
        // <div id=<a/> />;
        T![<] => parse_any_jsx_tag(p, true),
        // test jsx jsx_element_attribute_string_literal
        // <div id="a" />;
        JsSyntaxKind::JSX_STRING_LITERAL => {
            let m = p.start();
            p.bump(JSX_STRING_LITERAL);
            ParsedSyntax::Present(m.complete(p, JSX_STRING))
        }
        _ => ParsedSyntax::Absent,
    }
}
// test_err jsx jsx_element_attribute_expression_error
// <div className={asdf asdf} />;
fn parse_jsx_expression_attribute_value(p: &mut JsParser) -> ParsedSyntax {
    if !p.at(T!['{']) {
        return ParsedSyntax::Absent;
    }

    let m = p.start();
    p.bump(T!['{']);
    parse_jsx_assignment_expression(p, false).or_add_diagnostic(p, expected_expression);
    if !p.expect(T!['}']) && p.nth_at(1, T!['}']) {
        p.parse_as_skipped_trivia_tokens(|p| {
            p.bump_any();
        });
        p.expect(T!['}']);
    }

    ParsedSyntax::Present(m.complete(p, JSX_EXPRESSION_ATTRIBUTE_VALUE))
}

// test jsx jsx_children_expression
// let x;
// let a;
// let b;
// let key;
// let f = () => {};
// <div>
//   {1}
//   {9007199254740991n}
//   {""}
//   {true}
//   {null}
//   {undefined}
//   {/a/}
//   {[]}
//   {x => console.log(x)}
//   {x = 1}
//   {await x}
//   {1 + 1}
//   {f()}
//   {a[b]}
//   {a?1:2}
//   {function f() {}}
//   {function () {}}
//   {a}
//   {import("a.js")}
//   {key in a}
//   {a instanceof Object}
//   {a && b}
//   {new f()}
//   {{}}
//   {(a)}
//   {a++}
//   {++a}
//   {a,b}
//   {a.b}
//   {super.a()}
//   {this}
//   {delete a.a}
//   {void a}
//   {typeof a}
//   {+a}
//   {-a}
//   {!a}
//   {~a}
//   {``}
//   {/* A JSX comment */}
//   {/* Multi
//       line
//   */}
//   {}
// </div>
// function *f() {
//     return <div>
//         {yield a}
//     </div>;
// }

// test_err jsx jsx_children_expressions_not_accepted
// <div>
//   {import.meta}
//   {class A{}}
//   {super()}
//   {new.target}
// </div>
fn parse_jsx_assignment_expression(p: &mut JsParser, is_spread: bool) -> ParsedSyntax {
    let expr = parse_expression(p, ExpressionContext::default());

    expr.map(|mut expr| {
        let msg = if is_spread {
            "This expression is not valid as a JSX spread expression"
        } else {
            "This expression is not valid as a JSX expression."
        };

        let err = match expr.kind(p) {
            JsSyntaxKind::JS_IMPORT_META_EXPRESSION
            | JsSyntaxKind::JS_NEW_TARGET_EXPRESSION
            | JsSyntaxKind::JS_CLASS_EXPRESSION => Some(p.err_builder(msg, expr.range(p))),
            JsSyntaxKind::JS_SEQUENCE_EXPRESSION if is_spread => {
                Some(p.err_builder(msg, expr.range(p)))
            }
            _ => None,
        };

        if let Some(err) = err {
            p.error(err);
            expr.change_to_bogus(p);
        }

        expr
    })
}
