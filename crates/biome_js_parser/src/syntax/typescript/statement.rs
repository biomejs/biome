use crate::parser::RecoveryResult;
use crate::prelude::*;
use crate::syntax::binding::{
    is_nth_at_identifier_binding, parse_binding, parse_identifier_binding,
};
use crate::syntax::class::parse_initializer_clause;
use crate::syntax::expr::{is_nth_at_identifier, parse_name, ExpressionContext};

use super::ts_parse_error::expected_ts_enum_member;
use crate::state::EnterAmbientContext;
use crate::syntax::auxiliary::{is_nth_at_declaration_clause, parse_declaration_clause};
use crate::syntax::js_parse_error::{
    expected_declare_statement, expected_identifier, expected_module_source,
};
use crate::syntax::module::{parse_module_item_list, parse_module_source, ModuleItemListParent};
use crate::syntax::stmt::{semi, STMT_RECOVERY_SET};
use crate::syntax::typescript::ts_parse_error::expected_ts_type;
use crate::syntax::typescript::{
    expect_ts_type_list, parse_ts_identifier_binding, parse_ts_implements_clause, parse_ts_name,
    parse_ts_type, parse_ts_type_parameters, TypeContext, TypeMembers,
};
use crate::{syntax, Absent, JsParser, ParseRecoveryTokenSet, ParsedSyntax, Present};
use biome_js_syntax::{JsSyntaxKind::*, *};
use biome_parser::diagnostic::expected_token;
use biome_parser::parse_lists::{ParseNodeList, ParseSeparatedList};

fn parse_literal_as_ts_enum_member(p: &mut JsParser) -> ParsedSyntax {
    let m = p.start();
    match p.cur() {
        JS_STRING_LITERAL | T![ident] => {
            p.bump_any();
        }
        t if t.is_keyword() => {
            p.bump_remap(T![ident]);
        }
        JS_NUMBER_LITERAL => {
            let err = p.err_builder("An enum member cannot have a numeric name", p.cur_range());
            p.error(err);
            m.abandon(p);
            return Absent;
        }
        _ => {
            m.abandon(p);
            return Absent;
        }
    }
    Present(m.complete(p, TS_LITERAL_ENUM_MEMBER_NAME))
}

/// An individual enum member
fn parse_ts_enum_member(p: &mut JsParser) -> ParsedSyntax {
    let member = p.start();

    let name = match p.cur() {
        T!['['] => syntax::object::parse_computed_member_name(p),
        T![#] => {
            let err = p.err_builder("An `enum` member cannot be private", p.cur_range());
            p.error(err);
            syntax::class::parse_private_class_member_name(p).map(|mut x| {
                x.change_to_bogus(p);
                x
            })
        }
        _ => parse_literal_as_ts_enum_member(p),
    };

    if name.is_absent() {
        member.abandon(p);
        return Absent;
    }

    let _ = parse_initializer_clause(p, ExpressionContext::default());

    Present(member.complete(p, TS_ENUM_MEMBER))
}
struct TsEnumMembersList;

impl ParseSeparatedList for TsEnumMembersList {
    type Kind = JsSyntaxKind;
    type Parser<'source> = JsParser<'source>;

    const LIST_KIND: Self::Kind = TS_ENUM_MEMBER_LIST;

    fn parse_element(&mut self, p: &mut JsParser) -> ParsedSyntax {
        parse_ts_enum_member(p)
    }

    fn is_at_list_end(&self, p: &mut JsParser) -> bool {
        p.at(T!['}'])
    }

    fn recover(&mut self, p: &mut JsParser, parsed_element: ParsedSyntax) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(
                JS_BOGUS_MEMBER,
                STMT_RECOVERY_SET.union(token_set![JsSyntaxKind::IDENT, T![,], T!['}']]),
            )
            .enable_recovery_on_line_break(),
            expected_ts_enum_member,
        )
    }

    fn separating_element_kind(&mut self) -> JsSyntaxKind {
        T![,]
    }

    fn allow_trailing_separating_element(&self) -> bool {
        true
    }
}

#[inline(always)]
fn is_reserved_enum_name(name: &str) -> bool {
    super::is_reserved_type_name(name)
}

fn parse_ts_enum_id(p: &mut JsParser, enum_token_range: TextRange) {
    match parse_binding(p) {
        Present(id) => {
            let text = p.text(id.range(p));
            if is_reserved_enum_name(text) {
                let err = p.err_builder(
                    format!(
                        "`{text}` cannot be used as a enum name because it is already reserved"
                    ),
                    id.range(p),
                );

                p.error(err);
            }
        }
        // test_err ts enum_decl_no_id
        // enum {A,B,C}
        // enum 1 {A,B,C}
        Absent => {
            if p.nth_at(1, L_CURLY) {
                let range = p.cur_range();

                let m = p.start();
                p.bump_any();
                let _ = m.complete(p, JS_BOGUS_BINDING);

                let err = p.err_builder("invalid `enum` name", range);
                p.error(err);
            } else {
                let err = p.err_builder(
                    "`enum` statements must have a name",
                    TextRange::new(enum_token_range.start(), p.cur_range().start()),
                );
                p.error(err);
            }
        }
    }
}

pub(crate) fn is_at_ts_enum_declaration(p: &mut JsParser) -> bool {
    is_nth_at_ts_enum_declaration(p, 0)
}

pub(crate) fn is_nth_at_ts_enum_declaration(p: &mut JsParser, n: usize) -> bool {
    match p.nth(n) {
        T![enum] => true,
        T![const] => p.nth_at(n + 1, T![enum]),
        _ => false,
    }
}

// test ts typescript_enum
// enum A {}
// enum B { a, b, c }
// const enum C { A = 1, B = A * 2, ["A"] = 3, }
//
// test_err ts typescript_enum_incomplete
// enum A {
pub(crate) fn parse_ts_enum_declaration(p: &mut JsParser) -> ParsedSyntax {
    if !is_at_ts_enum_declaration(p) {
        return Absent;
    }

    let m = p.start();
    p.eat(T![const]);

    let enum_token_range = p.cur_range();
    p.expect(T![enum]);
    parse_ts_enum_id(p, enum_token_range);

    // test_err ts enum_no_l_curly
    // enum;
    // enum A;
    p.expect(T!['{']);
    TsEnumMembersList.parse_list(p);

    // test_err ts enum_no_r_curly
    // enum {;
    // enum A {;
    p.expect(T!['}']);

    Present(m.complete(p, TS_ENUM_DECLARATION))
}

pub(crate) fn parse_ts_type_alias_declaration(p: &mut JsParser) -> ParsedSyntax {
    if !p.at(T![type]) {
        return Absent;
    }

    let start = p.cur_range().start();
    let m = p.start();
    p.expect(T![type]);
    parse_ts_identifier_binding(p, super::TsIdentifierContext::Type)
        .or_add_diagnostic(p, expected_identifier);
    parse_ts_type_parameters(
        p,
        TypeContext::default()
            .and_allow_in_out_modifier(true)
            .and_type_or_interface_declaration(true),
    )
    .ok();
    p.expect(T![=]);
    parse_ts_type(p, TypeContext::default()).or_add_diagnostic(p, expected_ts_type);

    semi(p, TextRange::new(start, p.cur_range().end()));

    Present(m.complete(p, TS_TYPE_ALIAS_DECLARATION))
}

// test ts ts_declare_const_initializer
// declare module test { const X; }
//
// test_err ts ts_declare_using
// declare using x: null
// declare await using x: null
pub(crate) fn parse_ts_declare_statement(p: &mut JsParser) -> ParsedSyntax {
    if !is_at_ts_declare_statement(p) {
        return Absent;
    }

    let stmt_start_pos = p.cur_range().start();
    let m = p.start();
    p.expect(T![declare]);

    p.with_state(EnterAmbientContext, |p| {
        // test_err ts ts_declare_const_initializer
        // declare @decorator class D {}
        // declare @decorator abstract class D {}
        parse_declaration_clause(p, stmt_start_pos).or_add_diagnostic(p, expected_declare_statement)
    });

    Present(m.complete(p, TS_DECLARE_STATEMENT))
}

#[inline]
pub(crate) fn is_at_ts_declare_statement(p: &mut JsParser) -> bool {
    if !p.at(T![declare]) || p.has_nth_preceding_line_break(1) {
        return false;
    }

    if matches!(p.nth(1), T![using])
        || (matches!(p.nth(1), T![await]) && matches!(p.nth(2), T![using]))
    {
        return false;
    }

    is_nth_at_declaration_clause(p, 1)
}

#[inline]
pub(crate) fn is_at_ts_interface_declaration(p: &mut JsParser) -> bool {
    is_nth_at_ts_interface_declaration(p, 0)
}

#[inline]
pub(crate) fn is_nth_at_ts_interface_declaration(p: &mut JsParser, n: usize) -> bool {
    if !p.nth_at(n, T![interface]) || p.has_nth_preceding_line_break(n + 1) {
        return false;
    }

    is_nth_at_identifier_binding(p, n + 1) || p.nth_at(n + 1, T!['{'])
}

// test ts ts_interface
// interface A {}
// interface B { prop: string, method(): string, [index: number]: string, new(): B }

// test ts ts_index_signature_interface_member
// interface A {
//     [a: number]: string;
// }
// interface B {
//     [index: string]: { prop }
// }
// interface C {
//     readonly [a: number]: string;
// }

// test_err ts ts_index_signature_interface_member_cannot_be_static
// interface A {
//     static [index: string]: string
// }
// interface B {
//     public [index: string]: string
// }
// interface C {
//     private [index: string]: string
// }

// test_err ts ts_index_signature_interface_member_cannot_have_visibility_modifiers
// interface A {
//     public  [a: number]: string;
// }
// interface B {
//     private  [a: number]: string;
// }
// interface C {
//     protected  [a: number]: string;
// }

// test_err ts interface_cannot_be_reserved_world
// interface undefined {}
// interface any {}
pub(crate) fn parse_ts_interface_declaration(p: &mut JsParser) -> ParsedSyntax {
    if !is_at_ts_interface_declaration(p) {
        return Absent;
    }

    let m = p.start();
    p.expect(T![interface]);
    parse_ts_identifier_binding(p, super::TsIdentifierContext::Type)
        .or_add_diagnostic(p, expected_identifier);
    parse_ts_type_parameters(
        p,
        TypeContext::default()
            .and_allow_in_out_modifier(true)
            .and_type_or_interface_declaration(true),
    )
    .ok();
    eat_interface_heritage_clause(p);
    p.expect(T!['{']);
    TypeMembers::default().parse_list(p);
    p.expect(T!['}']);

    Present(m.complete(p, TS_INTERFACE_DECLARATION))
}

// test_err ts ts_interface_heritage_clause_error
// interface A {}
// interface B implements A {}
// interface C extends A extends B {}
// interface D extends {}
// interface E extends A, {}
/// Eats an interface's `extends` or an `extends` (not allowed but for better recovery) clauses
/// Attaches the clauses to the currently active node
fn eat_interface_heritage_clause(p: &mut JsParser) {
    let mut first_extends: Option<CompletedMarker> = None;
    loop {
        if p.at(T![extends]) {
            let extends = parse_ts_extends_clause(p).expect(
                "expected an extends clause because parser is positioned at the extends keyword",
            );

            if let Some(first_extends) = first_extends.as_ref() {
                p.error(
                    p.err_builder("'extends' clause already seen.", extends.range(p))
                        .with_detail(first_extends.range(p), "first 'extends' clause"),
                )
            } else {
                first_extends = Some(extends);
            }
        } else if p.at(T![implements]) {
            let implements =
                parse_ts_implements_clause(p).expect("positioned at the implements keyword");
            p.error(p.err_builder(
                "Interface declaration cannot have 'implements' clause.",
                implements.range(p),
            ));
        } else {
            break;
        }
    }
}

// test ts ts_interface_extends_clause
// interface A<Prop> { prop: Prop }
// interface B extends A<string> {}
// interface C extends A<number>, B {}
fn parse_ts_extends_clause(p: &mut JsParser) -> ParsedSyntax {
    if !p.at(T![extends]) {
        return Absent;
    }

    let m = p.start();
    p.expect(T![extends]);
    expect_ts_type_list(p, "extends");
    Present(m.complete(p, TS_EXTENDS_CLAUSE))
}

#[inline]
pub(crate) fn is_nth_at_any_ts_namespace_declaration(p: &mut JsParser, n: usize) -> bool {
    if p.nth_at(n, T![global]) {
        return p.nth_at(n + 1, T!['{']);
    }

    if p.has_nth_preceding_line_break(n + 1) {
        return false;
    }

    if matches!(p.nth(n), T![namespace] | T![module]) {
        return is_nth_at_identifier(p, n + 1) || p.nth_at(n + 1, JS_STRING_LITERAL);
    }

    false
}

pub(crate) fn parse_any_ts_namespace_declaration_clause(
    p: &mut JsParser,
    stmt_start_pos: TextSize,
) -> ParsedSyntax {
    match p.cur() {
        T![global] => parse_ts_global_declaration(p),
        T![namespace] | T![module] => {
            parse_ts_namespace_or_module_declaration_clause(p, stmt_start_pos)
        }
        _ => Absent,
    }
}

pub(crate) fn parse_any_ts_namespace_declaration_statement(p: &mut JsParser) -> ParsedSyntax {
    parse_any_ts_namespace_declaration_clause(p, p.cur_range().start())
}

// test ts ts_namespace_declaration
// declare namespace a {}
// declare namespace a.b.c.d {}
// declare namespace a.b { function test(): string }
// namespace X { }
//
// test ts ts_module_declaration
// declare module a {}
// declare module a.b.c.d {}
// declare module a.b { function test(): string }
// module X {}
//
// test ts ts_external_module_declaration
// declare module "a";
// declare module "b"
// declare module "import" {}
//
// test_err ts ts_module_err
// declare module a; // missing body
// declare module "a" declare module "b"; // missing semi
fn parse_ts_namespace_or_module_declaration_clause(
    p: &mut JsParser,
    stmt_start_pos: TextSize,
) -> ParsedSyntax {
    if !matches!(p.cur(), T![namespace] | T![module]) {
        return Absent;
    }

    let m = p.start();

    if !p.eat(T![namespace]) {
        p.expect(T![module]);

        if p.at(JS_STRING_LITERAL) {
            parse_module_source(p).expect("expected module source to be present because parser is positioned at a string literal");

            let body = parse_ts_module_block(p);

            if body.is_absent() {
                if p.at(T![;]) {
                    let body = p.start();
                    p.bump(T![;]);
                    body.complete(p, TS_EMPTY_EXTERNAL_MODULE_DECLARATION_BODY);
                } else {
                    semi(p, TextRange::new(stmt_start_pos, p.cur_range().end()));
                }
            }

            return Present(m.complete(p, TS_EXTERNAL_MODULE_DECLARATION));
        }
    }

    parse_ts_module_name(p).or_add_diagnostic(p, expected_identifier);

    parse_ts_module_block(p).or_add_diagnostic(p, |_, _| expected_token(T!['{']));
    Present(m.complete(p, TS_MODULE_DECLARATION))
}

// test ts built_in_module_name
// // https://github.com/rome/tools/issues/2959
// module number {}
// module string {}
// declare module never {}
fn parse_ts_module_name(p: &mut JsParser) -> ParsedSyntax {
    let mut left = parse_ts_identifier_binding(p, super::TsIdentifierContext::Module);

    while p.at(T![.]) {
        let m = left.precede_or_add_diagnostic(p, expected_identifier);
        p.bump(T![.]);
        parse_name(p).or_add_diagnostic(p, expected_identifier);
        left = Present(m.complete(p, TS_QUALIFIED_MODULE_NAME));
    }

    left
}

fn parse_ts_module_block(p: &mut JsParser) -> ParsedSyntax {
    if !p.at(T!['{']) {
        return Absent;
    }

    let m = p.start();
    p.bump(T!['{']);
    let items_list = p.start();
    parse_module_item_list(p, ModuleItemListParent::Block, items_list);
    p.expect(T!['}']);

    Present(m.complete(p, TS_MODULE_BLOCK))
}

// test ts ts_global_declaration
// declare module "./test" {
//  global {
//      let VERSION: string;
//  }
// }
// declare module "foo" {
//  global
//  { }
// }
// declare global {}
// declare global
// { }
//
// test ts ts_global_variable
// let global;
// global // not a global declaration
// console.log("a");
fn parse_ts_global_declaration(p: &mut JsParser) -> ParsedSyntax {
    if !p.at(T![global]) {
        return Absent;
    }

    let m = p.start();
    p.expect(T![global]);
    parse_ts_module_block(p).or_add_diagnostic(p, |_, _| expected_token(T!['{']));
    Present(m.complete(p, TS_GLOBAL_DECLARATION))
}

// test ts ts_import_equals_declaration
// import x = require("./test");
// namespace a.b {}
// import y = a;
// import z = a.b;
// import type A = require("./a");
// export import n = a;

/// Parses everything after the `import` of an import equals declaration
pub(crate) fn parse_ts_import_equals_declaration_rest(
    p: &mut JsParser,
    m: Marker,
    stmt_start_pos: TextSize,
) -> CompletedMarker {
    if is_nth_at_identifier_binding(p, 1) {
        p.eat(T![type]);
    }

    parse_identifier_binding(p).or_add_diagnostic(p, expected_identifier);

    p.expect(T![=]);

    if p.at(T![require]) {
        parse_ts_external_module_reference(p)
            .expect("Expect module reference to return Present because parser is at require token");
    } else {
        parse_ts_name(p).or_add_diagnostic(p, expected_identifier);
    }

    semi(p, TextRange::new(stmt_start_pos, p.cur_range().end()));
    m.complete(p, TS_IMPORT_EQUALS_DECLARATION)
}

fn parse_ts_external_module_reference(p: &mut JsParser) -> ParsedSyntax {
    if !p.at(T![require]) {
        return Absent;
    }

    let m = p.start();
    p.expect(T![require]);
    p.expect(T!['(']);
    parse_module_source(p).or_add_diagnostic(p, expected_module_source);
    p.expect(T![')']);

    Present(m.complete(p, TS_EXTERNAL_MODULE_REFERENCE))
}
