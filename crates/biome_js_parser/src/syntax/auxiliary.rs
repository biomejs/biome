use crate::prelude::*;
use crate::syntax::class::{parse_class_declaration, parse_decorators};
use crate::syntax::function::parse_function_declaration;
use crate::syntax::js_parse_error::decorators_not_allowed;
use crate::syntax::module::parse_import_or_import_equals_declaration;
use crate::syntax::stmt::{
    is_nth_at_variable_declarations, parse_variable_declaration, semi, StatementContext,
    VariableDeclarationParent,
};
use crate::syntax::typescript::{
    is_nth_at_any_ts_namespace_declaration, is_nth_at_ts_interface_declaration,
    parse_any_ts_namespace_declaration_clause, parse_ts_enum_declaration,
    parse_ts_interface_declaration, parse_ts_type_alias_declaration,
};
use crate::{Absent, JsParser, ParsedSyntax};
use biome_js_syntax::JsSyntaxKind::{JS_BOGUS_STATEMENT, JS_VARIABLE_DECLARATION_CLAUSE};
use biome_js_syntax::T;
use biome_rowan::{TextRange, TextSize};

// test js export_variable_clause
// export let a;
// export const b = 3;
// export var c, d, e = 3;
//
// test_err js export_variable_clause_error
// export let a = ;
// export const b;
// export let d, c;
pub(crate) fn parse_variable_declaration_clause(p: &mut JsParser) -> ParsedSyntax {
    let start = p.cur_range().start();

    parse_variable_declaration(p, VariableDeclarationParent::Clause).map(|declaration| {
        let m = declaration.precede(p);
        semi(p, TextRange::new(start, p.cur_range().end()));
        m.complete(p, JS_VARIABLE_DECLARATION_CLAUSE)
    })
}

pub(crate) fn is_nth_at_declaration_clause(p: &mut JsParser, n: usize) -> bool {
    if matches!(
        p.nth(n),
        T![function] | T![const] | T![enum] | T![class] | T![import] | T![@]
    ) {
        return true;
    }

    if is_nth_at_variable_declarations(p, n) {
        return true;
    }

    if is_nth_at_any_ts_namespace_declaration(p, n) {
        return true;
    }

    if p.has_nth_preceding_line_break(n + 1) {
        return false;
    }

    if p.nth_at(n, T![type]) && !p.nth_at(n + 1, T![*]) && !p.nth_at(n + 1, T!['{']) {
        return true;
    }

    if is_nth_at_ts_interface_declaration(p, n) {
        return true;
    }

    if p.nth_at(n, T![async]) && p.nth_at(n + 1, T![function]) {
        return true;
    }

    if p.nth_at(n, T![abstract]) && p.nth_at(n + 1, T![class]) {
        return true;
    }

    false
}

pub(crate) fn parse_declaration_clause(p: &mut JsParser, stmt_start_pos: TextSize) -> ParsedSyntax {
    match p.cur() {
        T![function] => parse_function_declaration(p, StatementContext::StatementList),
        T![@] => {
            let decorator_list = parse_decorators(p);

            match p.cur() {
                T![class] | T![abstract] if !p.state().in_ambient_context() => {
                    // test js decorator_export_class_clause
                    // export @decorator class Bar {};
                    // export @first @second class Foo {
                    //     constructor() {}
                    // }

                    //test ts decorator_abstract_export_class_clause
                    // export @decorator abstract class Bar {};
                    // export @first @second abstract class Foo {
                    //     constructor() {}
                    // }
                    parse_class_declaration(p, decorator_list, StatementContext::StatementList)
                }
                _ => {
                    // test_err js decorator_export_class_clause
                    // @decorator
                    // export let a;
                    // @decorator1 @decorator2
                    // export function Foo() { }
                    decorator_list
                        .add_diagnostic_if_present(p, decorators_not_allowed)
                        .map(|mut marker| {
                            marker.change_kind(p, JS_BOGUS_STATEMENT);
                            marker
                        });

                    parse_declaration_clause(p, stmt_start_pos)
                }
            }
        }
        T![class] | T![abstract] => {
            parse_class_declaration(p, Absent, StatementContext::StatementList)
        }
        T![const] => {
            if p.nth_at(1, T![enum]) {
                parse_ts_enum_declaration(p)
            } else {
                // test ts ts_ambient_const_variable_statement
                // declare const a, b, c, d = "test";
                parse_variable_declaration_clause(p)
            }
        }
        // test ts ts_ambient_var_statement
        // declare var a, b, c;
        T![var] => parse_variable_declaration_clause(p),
        T![enum] => {
            // test ts ts_ambient_enum_statement
            // declare enum A { X, Y, Z }
            // declare const enum B { X, Y, Z }
            parse_ts_enum_declaration(p)
        }
        T![import] => parse_import_or_import_equals_declaration(p),
        T![async] => parse_function_declaration(p, StatementContext::StatementList),
        T![type] => {
            // test ts ts_declare_type_alias
            // declare type A = string;
            // declare type B = string | number & { a: string, b: number }
            parse_ts_type_alias_declaration(p)
        }
        T![interface] => {
            // test ts ts_ambient_interface
            // declare interface A { b: string, c: number }
            parse_ts_interface_declaration(p)
        }
        T![let] => {
            // test ts ts_ambient_let_variable_statement
            // declare let a, b, c, d;
            parse_variable_declaration_clause(p)
        }
        T![namespace] | T![global] | T![module] => {
            parse_any_ts_namespace_declaration_clause(p, stmt_start_pos)
        }

        _ => Absent,
    }
}
