//! Implements the parsing logic for ES Module syntax

use crate::lexer::TextSize;
use crate::prelude::*;
use crate::state::{EnterAmbientContext, ExportDefaultItem, ExportDefaultItemKind};
use crate::syntax::binding::{
    is_at_identifier_binding, is_nth_at_identifier_binding, parse_binding, parse_identifier_binding,
};
use crate::syntax::class::{
    empty_decorator_list, is_at_export_class_declaration, is_at_export_default_class_declaration,
    is_at_ts_abstract_class_declaration, parse_class_declaration,
    parse_class_export_default_declaration, parse_decorators,
};
use crate::syntax::expr::{
    is_at_expression, is_nth_at_reference_identifier, parse_assignment_expression_or_higher,
    parse_name, parse_reference_identifier, ExpressionContext,
};
use crate::syntax::function::{parse_function_export_default_declaration, LineBreak};
use crate::syntax::js_parse_error::{
    decorators_not_allowed, duplicate_assertion_keys_error, expected_binding, expected_declaration,
    expected_export_clause, expected_export_default_declaration, expected_export_name_specifier,
    expected_expression, expected_identifier, expected_literal_export_name, expected_module_source,
    expected_named_import_specifier, expected_namespace_or_named_import, expected_statement,
};
use crate::syntax::stmt::{parse_statement, semi, StatementContext, STMT_RECOVERY_SET};
use crate::syntax::typescript::ts_parse_error::ts_only_syntax_error;
use crate::syntax::typescript::{
    parse_ts_enum_declaration, parse_ts_import_equals_declaration_rest,
    parse_ts_interface_declaration,
};
use crate::JsSyntaxFeature::TypeScript;
use crate::{Absent, JsParser, ParseRecoveryTokenSet, ParsedSyntax, Present};
use biome_js_syntax::JsSyntaxKind::*;
use biome_js_syntax::{JsSyntaxKind, TextRange, T};
use biome_parser::diagnostic::{expected_any, expected_node};
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::RecoveryResult;
use biome_parser::ParserProgress;
use rustc_hash::FxHashMap;

use super::auxiliary::{is_nth_at_declaration_clause, parse_declaration_clause};
use super::js_parse_error::{expected_named_import, expected_namespace_import};
use super::metavariable::{is_at_metavariable, parse_metavariable};

// test js module
// import a from "b";
// export { a };
// c();
// import { c } from "c";
pub(crate) fn parse_module_body(p: &mut JsParser, statement_list: Marker) {
    parse_module_item_list(p, ModuleItemListParent::Module, statement_list);
}

pub(crate) enum ModuleItemListParent {
    Module,
    Block,
}

impl ModuleItemListParent {
    fn is_module(&self) -> bool {
        matches!(self, ModuleItemListParent::Module)
    }

    #[inline]
    fn is_at_list_end(&self, p: &JsParser) -> bool {
        if p.at(EOF) {
            return true;
        }

        match self {
            ModuleItemListParent::Block => p.at(T!['}']),
            _ => false,
        }
    }
}

pub(crate) fn parse_module_item_list(
    p: &mut JsParser,
    parent: ModuleItemListParent,
    list_marker: Marker,
) {
    let mut progress = ParserProgress::default();

    let recovery_set = if parent.is_module() {
        STMT_RECOVERY_SET
    } else {
        // test_err ts module_closing_curly
        // declare module A {
        //  "name": "troublesome-lib",
        //  "typings": "lib/index.d.ts",
        //  "version": "0.0.1"
        // }

        // don't eat the closing `}` if inside a block
        STMT_RECOVERY_SET.union(token_set!(T!['}']))
    };

    while !parent.is_at_list_end(p) {
        progress.assert_progressing(p);

        let module_item = parse_module_item(p);

        let recovered = module_item.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(JS_BOGUS_STATEMENT, recovery_set),
            expected_statement,
        );

        if recovered.is_err() {
            break;
        }
    }

    list_marker.complete(p, JS_MODULE_ITEM_LIST);
}

fn parse_module_item(p: &mut JsParser) -> ParsedSyntax {
    match p.cur() {
        T![import] if !token_set![T![.], T!['(']].contains(p.nth(1)) => {
            parse_import_or_import_equals_declaration(p)
        }
        T![export] => parse_export(p, Absent),
        T![@] => {
            let decorator_list = parse_decorators(p);

            match p.cur() {
                T![export]
                    if is_at_export_class_declaration(p)
                        || is_at_export_default_class_declaration(p) =>
                {
                    // test js decorator_export_top_level
                    // @decorator
                    // export class Foo { }
                    // @first.field @second @(() => decorator)()
                    // export class Bar {}
                    // @before
                    // export @after class Foo { }
                    //  @before
                    //  export abstract class Foo { }
                    //  @before
                    //  export @after abstract class Foo { }

                    // test ts decorator_export_default_top_level_1
                    // @decorator
                    // export default class Foo { }

                    // test ts decorator_export_default_top_level_2
                    // @first.field @second @(() => decorator)()
                    // export default class Bar {}

                    // test ts decorator_export_default_top_level_3
                    // @before
                    // export default @after class Foo { }

                    // test ts decorator_export_default_top_level_4
                    //  @before
                    //  export default abstract class Foo { }

                    // test ts decorator_export_default_top_level_5
                    //  @before
                    //  export default @after abstract class Foo { }
                    parse_export(p, decorator_list)
                }
                T![class] => {
                    // test js decorator_class_declaration_top_level
                    // @decorator
                    // class Foo { }
                    // @first.field @second @(() => decorator)()
                    // class Bar {}
                    parse_class_declaration(p, decorator_list, StatementContext::StatementList)
                }
                T![abstract] if is_at_ts_abstract_class_declaration(p, LineBreak::DoCheck) => {
                    // test ts decorator_abstract_class_declaration_top_level
                    // @decorator abstract class A {}
                    // @first.field @second @(() => decorator)()
                    // abstract class Bar {}
                    TypeScript.parse_exclusive_syntax(
                        p,
                        |p| {
                            parse_class_declaration(
                                p,
                                decorator_list,
                                StatementContext::StatementList,
                            )
                        },
                        |p, abstract_class| {
                            ts_only_syntax_error(p, "abstract classes", abstract_class.range(p))
                        },
                    )
                }
                _ => {
                    // test_err js decorator_class_declaration_top_level
                    // @decorator
                    // let a;
                    // @decorator1 @decorator2
                    // function Foo() { }
                    decorator_list
                        .add_diagnostic_if_present(p, decorators_not_allowed)
                        .map(|mut marker| {
                            marker.change_kind(p, JS_BOGUS_STATEMENT);
                            marker
                        });

                    parse_module_item(p)
                }
            }
        }
        _ => parse_statement(p, StatementContext::StatementList),
    }
}

pub(crate) fn parse_import_or_import_equals_declaration(p: &mut JsParser) -> ParsedSyntax {
    if !p.at(T![import]) {
        return Absent;
    }

    let start = p.cur_range().start();
    let import = p.start();
    p.bump(T![import]);

    debug_assert!(p.state().name_map.is_empty());
    p.state_mut().duplicate_binding_parent = Some("import");

    let statement = if is_at_identifier_binding(p) && (p.nth_at(1, T![=]) || p.nth_at(2, T![=])) {
        let import_equals = parse_ts_import_equals_declaration_rest(p, import, start);
        TypeScript.exclusive_syntax(p, import_equals, |p, decl| {
            ts_only_syntax_error(p, "'import =' declarations", decl.range(p))
        })
    } else {
        // test_err js import_err
        // import;
        // import *;
        // import * as c, { a, b } from "c";
        // import { aa + bb, dd } from "c";
        // import { ab, ac } from "c";
        // import { default } from "c";
        // import { "a" } from "c";
        // import { as x } from "c";
        // import 4 from "c";
        // import y from 4;
        parse_import_clause(p).or_add_diagnostic(p, |p, range| {
            expected_any(
                &["default import", "namespace import", "named import"],
                range,
                p,
            )
        });

        let end = p.cur_range().start();

        semi(p, TextRange::new(start, end));
        Present(import.complete(p, JS_IMPORT))
    };

    p.state_mut().duplicate_binding_parent = None;
    p.state_mut().name_map.clear();

    statement
}

// test js import_default_clause
// import foo from "test";
fn parse_import_clause(p: &mut JsParser) -> ParsedSyntax {
    if p.at(JS_STRING_LITERAL) {
        return parse_import_bare_clause(p);
    }

    let pos = p.source().position();
    let m = p.start();

    // test ts ts_import_clause_types
    // import type from "./mod"; // not a type
    // import type foo from "./mod";
    // import type * as foo2 from "./mod";
    // import type { foo3 } from "mod";
    // import type from from "./mod";
    let is_typed = 'is_typed: {
        if !p.at(T![type]) {
            break 'is_typed false;
        }

        if matches!(p.nth(1), T![*] | T!['{']) {
            break 'is_typed true;
        }

        if !is_nth_at_identifier_binding(p, 1) {
            break 'is_typed false;
        }

        !p.nth_at(1, T![from]) || p.nth_at(2, T![from])
    };

    if is_typed {
        p.eat(T![type]);
    }

    let clause = match p.cur() {
        T![*] => parse_import_namespace_clause_rest(p, m),
        T!['{'] => parse_import_named_clause_rest(p, m),
        _ if is_at_identifier_binding(p) => {
            let default_specifier = p.start();
            parse_identifier_binding(p).unwrap();
            default_specifier.complete(p, JS_DEFAULT_IMPORT_SPECIFIER);
            parse_import_default_clauses_rest(p, m, is_typed)
        }
        _ => {
            // SAFETY: Safe because the parser only eats the "type" keyword if it's followed by
            // either a *, {, or binding
            debug_assert_eq!(pos, p.source().position());
            m.abandon(p);
            return Absent;
        }
    };

    if is_typed {
        TypeScript.exclusive_syntax(p, clause, |p, clause| {
            ts_only_syntax_error(p, "'import type'", clause.range(p))
        })
    } else {
        Present(clause)
    }
}

// test js import_default_clauses
// import e, { f } from "b";

// import g, * as lorem from "c";
/// Parses the rest of a default clause or default named clause.
/// Rest meaning, everything after `type binding`
fn parse_import_default_clauses_rest(
    p: &mut JsParser,
    m: Marker,
    is_typed: bool,
) -> CompletedMarker {
    let syntax_type = match p.cur() {
        T![,] | T!['{'] | T![*] => {
            p.expect(T![,]);
            match p.cur() {
                T![*] => parse_namespace_import_specifier(p),
                _ => parse_named_import_specifier_list(p),
            }
            .or_add_diagnostic(p, expected_namespace_or_named_import);
            if is_typed {
                let start = m.start();
                let end = p.last_end().unwrap_or_else(|| p.cur_range().start());

                // test_err ts ts_typed_default_import_with_named
                // import type A, { B, C } from './a';
                p.error(p.err_builder("A type-only import can specify a default import or named bindings, but not both.",
                    start..end,))
            }
            JS_IMPORT_COMBINED_CLAUSE
        }
        _ => JS_IMPORT_DEFAULT_CLAUSE,
    };
    p.expect(T![from]);
    parse_module_source(p).or_add_diagnostic(p, expected_module_source);
    parse_import_attribute(p).ok();
    m.complete(p, syntax_type)
}

// test js import_bare_clause
// import "test";
// import "no_semicolon"
fn parse_import_bare_clause(p: &mut JsParser) -> ParsedSyntax {
    parse_module_source(p).map(|module_source| {
        let m = module_source.precede(p);
        parse_import_attribute(p).ok();
        m.complete(p, JS_IMPORT_BARE_CLAUSE)
    })
}

// test js import_decl
// import * as foo from "bla";
fn parse_import_namespace_clause_rest(p: &mut JsParser, m: Marker) -> CompletedMarker {
    parse_namespace_import_specifier(p).or_add_diagnostic(p, expected_namespace_import);
    p.expect(T![from]);
    parse_module_source(p).or_add_diagnostic(p, expected_module_source);
    parse_import_attribute(p).ok();

    m.complete(p, JS_IMPORT_NAMESPACE_CLAUSE)
}

// test js import_named_clause
// import {} from "a";
// import { a, b, c, } from "b";
// import { f as x, default as w, "a-b-c" as y } from "b";
fn parse_import_named_clause_rest(p: &mut JsParser, m: Marker) -> CompletedMarker {
    parse_named_import_specifier_list(p).or_add_diagnostic(p, expected_named_import);
    p.expect(T![from]);
    parse_module_source(p).or_add_diagnostic(p, expected_module_source);
    parse_import_attribute(p).ok();

    m.complete(p, JS_IMPORT_NAMED_CLAUSE)
}

fn parse_namespace_import_specifier(p: &mut JsParser) -> ParsedSyntax {
    if !p.at(T![*]) {
        return Absent;
    }

    let m = p.start();
    p.bump_any();
    p.expect(T![as]);
    parse_binding(p).or_add_diagnostic(p, expected_binding);

    Present(m.complete(p, JS_NAMESPACE_IMPORT_SPECIFIER))
}

fn parse_named_import_specifier_list(p: &mut JsParser) -> ParsedSyntax {
    if !p.at(T!['{']) {
        return Absent;
    }

    let m = p.start();
    p.bump(T!['{']);
    NamedImportSpecifierList.parse_list(p);
    p.expect(T!['}']);

    Present(m.complete(p, JS_NAMED_IMPORT_SPECIFIERS))
}

struct NamedImportSpecifierList;

impl ParseSeparatedList for NamedImportSpecifierList {
    type Kind = JsSyntaxKind;
    type Parser<'source> = JsParser<'source>;
    const LIST_KIND: Self::Kind = JS_NAMED_IMPORT_SPECIFIER_LIST;

    fn parse_element(&mut self, p: &mut JsParser) -> ParsedSyntax {
        parse_any_named_import_specifier(p)
    }

    fn is_at_list_end(&self, p: &mut JsParser) -> bool {
        p.at(T!['}'])
    }

    fn recover(&mut self, p: &mut JsParser, parsed_element: ParsedSyntax) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(
                JS_BOGUS_NAMED_IMPORT_SPECIFIER,
                STMT_RECOVERY_SET.union(token_set![T![,], T!['}'], T![;]]),
            )
            .enable_recovery_on_line_break(),
            expected_named_import_specifier,
        )
    }

    fn separating_element_kind(&mut self) -> JsSyntaxKind {
        T![,]
    }

    fn allow_trailing_separating_element(&self) -> bool {
        true
    }
}

// test ts ts_named_import_specifier_with_type
// import { type, type as } from "./mod";
// import { type as other } from "./mod"
// import { type as as } from "./mod";
// import { type as as as } from "./mod"
// import { type "test-abcd" as test } from "./mod";
//
// test_err ts ts_named_import_specifier_error
// import { default } from "./mod";
// import { type default } from "./mod";
// import { "literal-name" } from "./mod";
// import { type "literal-name" } from "./mod";
// import {
fn parse_any_named_import_specifier(p: &mut JsParser) -> ParsedSyntax {
    if !is_nth_at_literal_export_name(p, 0) {
        // covers `type` and `as` too
        return Absent;
    }

    let m = p.start();

    let metadata = specifier_metadata(
        p,
        is_nth_at_literal_export_name,
        is_nth_at_identifier_binding,
    );

    if metadata.is_type {
        p.expect(T![type]);
    }

    let specifier =
        if metadata.has_alias || p.at(JS_STRING_LITERAL) || p.cur().is_non_contextual_keyword() {
            if metadata.is_local_name_missing {
                // test_err js import_as_identifier_err
                // import { as c } from "test";
                p.error(expected_literal_export_name(
                    p,
                    TextRange::new(p.cur_range().start(), p.cur_range().start()),
                ));
            } else {
                // test js import_as_as_as_identifier
                // import { as as as } from "test";
                parse_literal_export_name(p).or_add_diagnostic(p, expected_literal_export_name);
            }

            p.expect(T![as]);
            parse_binding(p).or_add_diagnostic(p, expected_binding);
            m.complete(p, JS_NAMED_IMPORT_SPECIFIER)
        } else {
            // test js import_as_identifier
            // import { as } from "test";
            parse_binding(p).or_add_diagnostic(p, expected_identifier);
            m.complete(p, JS_SHORTHAND_NAMED_IMPORT_SPECIFIER)
        };

    if metadata.is_type {
        TypeScript.exclusive_syntax(p, specifier, |p, specifier| {
            ts_only_syntax_error(p, "'import { type x ident }'", specifier.range(p))
        })
    } else {
        Present(specifier)
    }
}

// test js import_attribute
// import "x" with { type: "json" }
// import "foo" with { "type": "json" };
// import foo from "foo.json" with { type: "json" };
// import {test} from "foo.json" with { for: "for" }
// import foo_json from "foo.json" with { type: "json", hasOwnProperty: "true" };
// import "x" with
// { type: "json" }

// test_err js import_attribute_err
// import "foo" with { type, "json" };
// import { foo } with { type: "json" };
// import foo2 from "foo.json" with { "type": "json", type: "html", "type": "js" };
// import "x" with;
// import ipsum from "ipsum.json" with { type: "json", lazy: true, startAtLine: 1 };
// import { a } from "a.json" with
fn parse_import_attribute(p: &mut JsParser) -> ParsedSyntax {
    if p.has_preceding_line_break() {
        return Absent;
    }
    if !p.at(T![assert]) && !p.at(T![with]) {
        return Absent;
    }

    let m = p.start();
    match p.cur() {
        T![with] => {
            p.expect(T![with]);
        }
        _ => {
            m.abandon(p);
            return Absent;
        }
    };

    // bump "with"
    p.expect(T!['{']);
    ImportAssertionList::default().parse_list(p);
    p.expect(T!['}']);

    Present(m.complete(p, JS_IMPORT_ASSERTION))
}

#[derive(Default)]
pub(crate) struct ImportAssertionList {
    assertion_keys: FxHashMap<String, TextRange>,
}

impl ParseSeparatedList for ImportAssertionList {
    type Kind = JsSyntaxKind;
    type Parser<'source> = JsParser<'source>;

    const LIST_KIND: Self::Kind = JS_IMPORT_ASSERTION_ENTRY_LIST;

    fn parse_element(&mut self, p: &mut JsParser) -> ParsedSyntax {
        parse_import_attribute_entry(p, &mut self.assertion_keys)
    }

    fn is_at_list_end(&self, p: &mut JsParser) -> bool {
        p.at(T!['}'])
    }

    fn recover(&mut self, p: &mut JsParser, parsed_element: ParsedSyntax) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(
                JS_BOGUS_IMPORT_ASSERTION_ENTRY,
                STMT_RECOVERY_SET.union(token_set![T![,], T!['}']]),
            )
            .enable_recovery_on_line_break(),
            |p, range| expected_node("import attribute entry", range, p),
        )
    }

    fn separating_element_kind(&mut self) -> JsSyntaxKind {
        T![,]
    }

    fn allow_trailing_separating_element(&self) -> bool {
        true
    }
}

fn parse_import_attribute_entry(
    p: &mut JsParser,
    seen_assertion_keys: &mut FxHashMap<String, TextRange>,
) -> ParsedSyntax {
    let m = p.start();
    let key_range = p.cur_range();

    let key = match p.cur() {
        JS_STRING_LITERAL => Some(p.cur_text().trim_matches(&['\'', '"'][..])),
        T![ident] => Some(p.cur_text()),
        t if t.is_keyword() => Some(p.cur_text()),
        _ => None,
    }
    .map(String::from);

    match p.cur() {
        JS_STRING_LITERAL | T![ident] => {
            p.bump_any();
        }
        t if t.is_keyword() => {
            p.bump_remap(T![ident]);
        }
        T![:] => {
            p.error(expected_any(
                &["identifier", "string literal"],
                p.cur_range(),
                p,
            ));
        }
        _ => {
            m.abandon(p);
            return Absent;
        }
    };

    let mut valid = true;

    if let Some(key) = key {
        if let Some(first_use) = seen_assertion_keys.get(&key) {
            p.error(duplicate_assertion_keys_error(
                p, &key, *first_use, key_range,
            ));
            valid = false;
        } else {
            seen_assertion_keys.insert(key, key_range);
        }
    };

    p.expect(T![:]);
    p.expect(JS_STRING_LITERAL);

    let mut entry = m.complete(p, JS_IMPORT_ASSERTION_ENTRY);

    if !valid {
        entry.change_to_bogus(p);
    }

    Present(entry)
}

// test_err js export_err
// export
//
// test_err js ts_export_syntax_in_js
// let a, b, c;
// export type { a };
// export { type b };
// export { type c as cc };
// export type { d } from "./d";
// export { type e } from "./e";
// export { type e as ee } from "./e";

// test_err js export_huge_function_in_script
// // SCRIPT
// export function A () { return "Kinsmen hot Moria tea serves. Sticky camp spell covering forged they're Oakenshield vines. Admirable relatives march regained wheel Ere eternally on rest parts unhappy? Leave hundreds market's Argonath answered avail grieve doing goodness! Wrong miserable well-wishers wander stood immediately neither Agreed goat poison holes fire? Nobody tosses a Dwarf. Brigands Bilbo Baggins prisoner stinker birthday injuries. Kili's loosened shy spiders till. Gandalf's death was not in vain. Nor would he have you give up hope. Bread kindly ghost Beorn's jelly. Andûril two-faced bitterness biding seemed says drinking splendor feed light unnoticed one! Carven nearest Eärendil fireworks former. Mattress smelling wandering teaching appear taste wise Mithril uprooted winter forebearers wheel. Let's beside Proudfoots succumbed! Excuse Anárion stolen helpless nudge study shown holding form? Changes point Snowbourn material side outer highest eaves flash-flame relic descendant lurking. Thousand death Agreed oppose whole? Glóin head's hurts feasting fight shiny legacy. Thror's broken odds suffice believe well-protected? Rightfully manners begged Maggot's fairer. Unheard-of grog shields sad wondering gardener killed gone Galadriel! Pan Frodo fingers spreads magic parting amount interest idly naked. It's some form of Elvish. I can't read it. Silverwork Wraiths riddled enchantment apple anywhere."; }
pub(super) fn parse_export(p: &mut JsParser, decorators_list: ParsedSyntax) -> ParsedSyntax {
    if !p.at(T![export]) {
        return Absent;
    }

    let stmt_start = p.cur_range().start();
    let decorators_list = decorators_list.or_else(|| empty_decorator_list(p));

    let m = decorators_list.precede(p);

    p.bump(T![export]);

    let clause = if is_nth_at_declaration_clause(p, 0) {
        // test js export_class_clause
        // export class A {}
        // export class A extends B {}

        // test js export_function_clause
        // export function test(a, b) {}
        // export function* test2(a, b) {}
        // export async function test3(a, b, ) {}

        // test ts ts_export_enum_declaration
        // export enum A { X, Y }
        // export const enum B { X, Y }

        // test ts ts_export_interface_declaration
        // export interface A {}

        parse_declaration_clause(p, stmt_start)
    } else {
        match p.cur() {
            T!['{'] => parse_export_named_or_named_from_clause(p),
            T![default] => parse_export_default_clause(p),
            T![type] if p.nth_at(1, T!['{']) => parse_export_named_or_named_from_clause(p),
            T![type] if p.nth_at(1, T![*]) => parse_export_from_clause(p),
            T![*] => parse_export_from_clause(p),
            T![=] => TypeScript.parse_exclusive_syntax(
                p,
                parse_ts_export_assignment_clause,
                |p, clause| ts_only_syntax_error(p, "'export ='", clause.range(p)),
            ),
            T![from] => parse_export_from_clause(p),
            T![as] if p.nth_at(1, T![namespace]) => TypeScript.parse_exclusive_syntax(
                p,
                parse_ts_export_namespace_clause,
                |p, clause| ts_only_syntax_error(p, "'export as namespace'", clause.range(p)),
            ),
            T![declare] if !p.has_nth_preceding_line_break(1) => TypeScript.parse_exclusive_syntax(
                p,
                |p| parse_ts_export_declare_clause(p, stmt_start),
                |p, clause| ts_only_syntax_error(p, "'export declare'", clause.range(p)),
            ),
            _ if p.nth_at(1, T![from]) => parse_export_from_clause(p),
            _ => Absent,
        }
    };

    clause.or_add_diagnostic(p, expected_export_clause);

    Present(m.complete(p, JS_EXPORT))
}

fn parse_export_named_or_named_from_clause(p: &mut JsParser) -> ParsedSyntax {
    let checkpoint = p.checkpoint();
    match parse_export_named_clause(p) {
        Present(_) if p.at(T![from]) => {
            p.rewind(checkpoint);
            parse_export_named_from_clause(p)
        }
        t => t,
    }
}

// test js export_named_clause
// export { a };
// export { a as z, b as "y", c as default }
// export { as };
//
// test_err js export_named_clause_err
// export { default as "b" };
// export { "a" as b };
// export { as b };
// export { a as 5 };
// export { a b c };
//
// test ts ts_export_type_named
// type A = string;
// export type { A };
//
// export {
//
// test_err ts ts_export_type
// export type
fn parse_export_named_clause(p: &mut JsParser) -> ParsedSyntax {
    if !matches!(p.cur(), T!['{'] | T![type]) {
        return Absent;
    }

    let start = p.cur_range().start();
    let m = p.start();

    let has_type = p.eat(T![type]);
    p.bump(T!['{']);
    ExportNamedSpecifierList.parse_list(p);
    p.expect(T!['}']);

    semi(p, TextRange::new(start, p.cur_range().start()));

    let clause = m.complete(p, JS_EXPORT_NAMED_CLAUSE);

    if has_type {
        TypeScript.exclusive_syntax(p, clause, |p, clause| {
            ts_only_syntax_error(p, "'export type' declarations", clause.range(p))
        })
    } else {
        Present(clause)
    }
}

struct ExportNamedSpecifierList;

impl ParseSeparatedList for ExportNamedSpecifierList {
    type Kind = JsSyntaxKind;
    type Parser<'source> = JsParser<'source>;

    const LIST_KIND: Self::Kind = JS_EXPORT_NAMED_SPECIFIER_LIST;

    fn parse_element(&mut self, p: &mut JsParser) -> ParsedSyntax {
        parse_any_export_named_specifier(p)
    }

    fn is_at_list_end(&self, p: &mut JsParser) -> bool {
        p.at(T!['}'])
    }

    fn recover(&mut self, p: &mut JsParser, parsed_element: ParsedSyntax) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(
                JS_BOGUS,
                STMT_RECOVERY_SET.union(token_set![T![,], T!['}'], T![;]]),
            )
            .enable_recovery_on_line_break(),
            expected_export_name_specifier,
        )
    }

    fn separating_element_kind(&mut self) -> JsSyntaxKind {
        T![,]
    }

    fn allow_trailing_separating_element(&self) -> bool {
        true
    }
}

fn parse_any_export_named_specifier(p: &mut JsParser) -> ParsedSyntax {
    if !matches!(p.cur(), T![type] | T![as] | T![default]) && !is_nth_at_literal_export_name(p, 0) {
        return Absent;
    }

    let m = p.start();

    let metadata = specifier_metadata(
        p,
        is_nth_at_reference_identifier,
        is_nth_at_literal_export_name,
    );

    // test ts ts_export_named_type_specifier
    // export { type }
    // export { type type }
    // export { type as somethingElse }
    if metadata.is_type {
        p.expect(T![type]);
    }

    // test_err js export_as_identifier_err
    // export { as c }

    if metadata.is_local_name_missing {
        p.error(expected_identifier(
            p,
            TextRange::new(p.cur_range().start(), p.cur_range().start()),
        ));
    } else if is_nth_at_reference_identifier(p, 0) {
        parse_reference_identifier(p).ok();
    } else {
        // We need to parse "default" or any string literal here so the "export ... from..." rewind later works.
        let is_string = matches!(p.cur(), JS_STRING_LITERAL);

        if let Some(export_name) =
            parse_literal_export_name(p).or_add_diagnostic(p, expected_identifier)
        {
            let error = if is_string {
                p.err_builder(
                    "A string literal cannot be used as an export binding without `from`.",
                    export_name.range(p),
                )
            } else {
                p.err_builder(
                    format!(
                        "\"{}\" can only be used with \"export ... from ...\"",
                        export_name.text(p)
                    ),
                    export_name.range(p),
                )
            };

            p.error(error);
        }
    }

    // test js export_as_identifier
    // export { as };
    // export { as as as }
    //
    let specifier = if metadata.has_alias {
        p.expect(T![as]);
        parse_literal_export_name(p).or_add_diagnostic(p, expected_literal_export_name);

        m.complete(p, JS_EXPORT_NAMED_SPECIFIER)
    } else {
        m.complete(p, JS_EXPORT_NAMED_SHORTHAND_SPECIFIER)
    };

    if metadata.is_type {
        TypeScript.exclusive_syntax(p, specifier, |p, specifier| {
            ts_only_syntax_error(p, "export { type ident }'", specifier.range(p))
        })
    } else {
        Present(specifier)
    }
}

#[derive(Default, Debug)]
struct SpecifierMetadata {
    // Is this a type export (`export { type test }`) or a regular value export (`export { test }`)
    is_type: bool,
    // Is this an aliased export (`export { t as test }`) or not
    has_alias: bool,
    // For error recovery in case the local name is missing: `export { as test }`
    is_local_name_missing: bool,
}

// test ts ts_export_type_specifier
// let as;
// let type;
// let a;
// export { type };
// export { type as as };
// export { as as as };
// export { type as as as }
// export { type type };
// export { type as };
// export { type a as aa };
fn specifier_metadata<LocalNamePred, AliasPred>(
    p: &mut JsParser,
    is_nth_name: LocalNamePred,
    is_nth_alias: AliasPred,
) -> SpecifierMetadata
where
    LocalNamePred: Fn(&mut JsParser, usize) -> bool,
    AliasPred: Fn(&mut JsParser, usize) -> bool,
{
    let mut metadata = SpecifierMetadata::default();

    // This may be a typed import/export, but it could also be the name of the import/export:
    // ```ts
    // { type}              // name: `type`
    // { type type }        // name: `type`    type-export: `true`
    // { type as }          // name: `as`      type-export: `true`
    // { type as as }       // name: `type`    type-export: `false` (aliased to `as`)
    // { type as as as }    // name: `as`      type-export: `true`, aliased to `as`
    // ```
    if p.at(T![type]) {
        // `{ type identifier }`

        if p.nth_at(1, T![as]) {
            // `{ type as ... }`

            if p.nth_at(2, T![as]) {
                metadata.has_alias = true;
                // `{ type as as }`: Type can either be an identifier or the type keyword

                if is_nth_alias(p, 3) {
                    // `{ type as as x }` or `{ type as as "x"}`
                    metadata.is_type = true;
                }
            } else if is_nth_alias(p, 2) {
                // `{ type as x }` or `{ type as "x" }`
                metadata.has_alias = true;
            } else {
                // `{ type as }`
                metadata.is_type = true;
            }
        } else {
            // `{ type x }` or `{ type "x" }` or `{ type x as }` or `{ type default as }`
            metadata.is_type = is_nth_name(p, 1) || p.nth_at(1, T![default]);
            metadata.has_alias = p.nth_at(2, T![as]);
        }
    } else if p.at(T![as]) && is_nth_alias(p, 1) {
        metadata.has_alias = true;

        // error recovery case in case someone typed "as x" but forgot the local name.
        // `{ as x }`
        metadata.is_local_name_missing = !p.nth_at(1, T![as]);
    } else if p.nth_at(1, T![as]) {
        // `{ x as ... }`
        metadata.has_alias = true;
    }

    metadata
}

// test js export_from_clause
// export {
//     default as a } from "b";
// export { default as a } from "b";
// export * from "a";
// export * as c from "b";
// export * as default from "b"
// export * from "mod" with { type: "json" }
// export type * from "types";
// export type * as types from "types";
//
// test_err js export_from_clause_err
// export *;
// export * from 5;
// export as from "test";
// export from "test";
// export type *;
// export type * from;
// export type * as from;
// export type * as ns from;
fn parse_export_from_clause(p: &mut JsParser) -> ParsedSyntax {
    if !p.at(T![type]) && !p.at(T![*]) && !p.at(T![from]) && !p.nth_at(1, T![from]) {
        return Absent;
    }

    let start = p.cur_range().start();
    let m = p.start();

    p.eat(T![type]);
    p.expect(T![*]);

    parse_export_as_clause(p).ok();
    p.expect(T![from]);
    parse_module_source(p).or_add_diagnostic(p, expected_module_source);
    parse_import_attribute(p).ok();
    semi(p, TextRange::new(start, p.cur_range().end()));

    Present(m.complete(p, JS_EXPORT_FROM_CLAUSE))
}

// test js export_named_from_clause
// export { a, default } from "mod";
// export { a as z, b as "y", c as default } from "mod"
// export { as } from "mod";
// export { default as "b" } from "mod";
// export { "a" as b } from "mod";
// export { a } from "mod" with { type: "json" }
// export { "a" } from "./mod";
// export {
//      "a"
// } from "./mod";
//
// test_err js export_named_from_clause_err
// export { as b } from "mod";
// export { a as 5 } from "mod";
// export { a b c } from "mod";
// export { 5 as b } from "mod";
//
// test ts ts_export_type_named_from
// export type { A } from "a";
//
// test_err js escaped_from
// export {} \u0066rom "./escaped-from.js";
fn parse_export_named_from_clause(p: &mut JsParser) -> ParsedSyntax {
    if !matches!(p.cur(), T!['{'] | T![type]) {
        return Absent;
    }

    let start = p.cur_range().start();
    let m = p.start();

    let has_type = p.eat(T![type]);

    p.bump(T!['{']);
    ExportNamedFromSpecifierList.parse_list(p);
    p.expect(T!['}']);

    p.expect(T![from]);

    parse_module_source(p).or_add_diagnostic(p, expected_module_source);
    parse_import_attribute(p).ok();

    semi(p, TextRange::new(start, p.cur_range().start()));

    let clause = m.complete(p, JS_EXPORT_NAMED_FROM_CLAUSE);

    if has_type {
        TypeScript.exclusive_syntax(p, clause, |p, clause| {
            ts_only_syntax_error(p, "'export type' declarations", clause.range(p))
        })
    } else {
        Present(clause)
    }
}

struct ExportNamedFromSpecifierList;

impl ParseSeparatedList for ExportNamedFromSpecifierList {
    type Kind = JsSyntaxKind;
    type Parser<'source> = JsParser<'source>;

    const LIST_KIND: Self::Kind = JS_EXPORT_NAMED_FROM_SPECIFIER_LIST;

    fn parse_element(&mut self, p: &mut JsParser) -> ParsedSyntax {
        parse_export_named_from_specifier(p)
    }

    fn is_at_list_end(&self, p: &mut JsParser) -> bool {
        p.at(T!['}'])
    }

    fn recover(&mut self, p: &mut JsParser, parsed_element: ParsedSyntax) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(
                JS_BOGUS,
                STMT_RECOVERY_SET.union(token_set![T![,], T!['}'], T![;]]),
            )
            .enable_recovery_on_line_break(),
            expected_literal_export_name,
        )
    }

    fn separating_element_kind(&mut self) -> JsSyntaxKind {
        T![,]
    }

    fn allow_trailing_separating_element(&self) -> bool {
        true
    }
}

// test ts ts_export_named_from_specifier_with_type
// export { type A } from "a"
// export { type } from "./type";
fn parse_export_named_from_specifier(p: &mut JsParser) -> ParsedSyntax {
    if !matches!(p.cur(), T![type] | T![as]) && !is_nth_at_literal_export_name(p, 0) {
        return Absent;
    }

    let m = p.start();
    let metadata = specifier_metadata(
        p,
        is_nth_at_reference_identifier,
        is_nth_at_literal_export_name,
    );

    if metadata.is_type {
        p.expect(T![type]);
    }

    if metadata.is_local_name_missing {
        p.error(expected_literal_export_name(
            p,
            TextRange::new(p.cur_range().start(), p.cur_range().start()),
        ));
    } else {
        parse_literal_export_name(p).or_add_diagnostic(p, expected_literal_export_name);
    }

    if metadata.has_alias {
        parse_export_as_clause(p).ok();
    }

    let specifier = Present(m.complete(p, JS_EXPORT_NAMED_FROM_SPECIFIER));

    if metadata.is_type {
        TypeScript.exclusive_syntax(p, specifier, |p, specifier| {
            ts_only_syntax_error(p, "export { type ident }''", specifier.range(p))
        })
    } else {
        specifier
    }
}

fn parse_export_default_clause(p: &mut JsParser) -> ParsedSyntax {
    if !p.at(T![default]) {
        return Absent;
    }

    let start = p.cur_range().start();
    let m = p.start();
    p.bump(T![default]);

    let (clause, default_item_kind) = match p.cur() {
        T![@] => {
            let decorator_list = parse_decorators(p);

            match p.cur() {
                // test ts decorator_class_export_default_declaration_clause
                // @decorator
                // export default class Foo { }
                T![class] => parse_class_export_default_declaration_clause(p, m, decorator_list),
                T![abstract] if p.nth_at(1, T![class]) => {
                    // test ts decorator_abstract_class_export_default_declaration_clause
                    // @decorator
                    // export default abstract class Foo { }
                    parse_class_export_default_declaration_clause(p, m, decorator_list)
                }
                _ => {
                    decorator_list
                        .add_diagnostic_if_present(p, decorators_not_allowed)
                        .map(|mut marker| {
                            marker.change_kind(p, JS_BOGUS_STATEMENT);
                            marker
                        });

                    match p.cur() {
                        // test_err ts decorator_function_export_default_declaration_clause
                        // @decorator
                        // export default function foo() { }
                        T![function] => parse_function_export_default_declaration_clause(p, m),
                        // test_err ts decorator_async_function_export_default_declaration_clause
                        // @decorator
                        // export default async function foo() { }
                        T![async] if p.nth_at(1, T![function]) => {
                            parse_function_export_default_declaration_clause(p, m)
                        }
                        // test_err ts decorator_interface_export_default_declaration_clause
                        // @decorator
                        // export default interface A { }
                        T![interface] if !p.has_nth_preceding_line_break(1) => {
                            parse_ts_interface_export_default_declaration_clause(p, m)
                        }
                        // test_err ts decorator_enum_export_default_declaration_clause
                        // @decorator
                        // export default enum A { X, Y, Z }
                        T![enum] => parse_ts_enum_export_default_declaration_clause(p, m),
                        _ => (
                            // test_err ts decorator_export_default_expression_clause
                            // @decorator
                            // export default a;
                            parse_export_default_expression_clause(p, m, start),
                            ExportDefaultItemKind::Expression,
                        ),
                    }
                }
            }
        }
        T![class] => parse_class_export_default_declaration_clause(p, m, Absent),
        T![abstract] if p.nth_at(1, T![class]) => {
            parse_class_export_default_declaration_clause(p, m, Absent)
        }
        T![function] => parse_function_export_default_declaration_clause(p, m),
        T![async] if p.nth_at(1, T![function]) => {
            parse_function_export_default_declaration_clause(p, m)
        }
        T![interface] if !p.has_nth_preceding_line_break(1) => {
            parse_ts_interface_export_default_declaration_clause(p, m)
        }
        T![enum] => parse_ts_enum_export_default_declaration_clause(p, m),
        _ => (
            parse_export_default_expression_clause(p, m, start),
            ExportDefaultItemKind::Expression,
        ),
    };

    clause.map(|mut clause| {
        // test_err js multiple_default_exports_err
        // export default (class {})
        // export default a + b;
        // export default (function a() {})
        if let Some(existing_default_item) =
            p.state().default_item.as_ref().filter(|_| p.is_module())
        {
            if existing_default_item.kind.is_mergeable(&default_item_kind) {
                // It's ok to have multiple overload declarations and an implementation.
                // This check won't catch if there are multiple implementations for the same overload
                // or if the overloads define different functions.
            } else {
                let err = p
                    .err_builder(
                        "Illegal duplicate default export declarations",
                        clause.range(p),
                    )
                    .with_detail(clause.range(p), "multiple default exports are erroneous")
                    .with_detail(
                        existing_default_item.range.clone(),
                        "the module's default export is first defined here",
                    );

                p.error(err);
                clause.change_kind(p, JsSyntaxKind::JS_BOGUS);
            }
        }
        // TypeScript supports multiple `export default interface` They all get merged together

        // test ts ts_export_default_multiple_interfaces
        // export default interface A { a: string; }
        // export default interface B { a: string }
        // export default function test() {}
        else if !default_item_kind.is_interface() {
            p.state_mut().default_item = Some(ExportDefaultItem {
                range: clause.range(p).into(),
                kind: default_item_kind,
            });
        }

        clause
    })
}

fn parse_class_export_default_declaration_clause(
    p: &mut JsParser,
    m: Marker,
    decorator_list: ParsedSyntax,
) -> (ParsedSyntax, ExportDefaultItemKind) {
    let declaration = parse_class_export_default_declaration(p, decorator_list);

    declaration.or_add_diagnostic(p, expected_export_default_declaration);

    (
        Present(m.complete(p, JS_EXPORT_DEFAULT_DECLARATION_CLAUSE)),
        ExportDefaultItemKind::ClassDeclaration,
    )
}

fn parse_function_export_default_declaration_clause(
    p: &mut JsParser,
    m: Marker,
) -> (ParsedSyntax, ExportDefaultItemKind) {
    let declaration = parse_function_export_default_declaration(p);

    let item_kind = match declaration.kind(p) {
        Some(TS_DECLARE_FUNCTION_DECLARATION | TS_DECLARE_FUNCTION_EXPORT_DEFAULT_DECLARATION) => {
            ExportDefaultItemKind::FunctionOverload
        }
        _ => ExportDefaultItemKind::FunctionDeclaration,
    };

    declaration.or_add_diagnostic(p, expected_export_default_declaration);

    (
        Present(m.complete(p, JS_EXPORT_DEFAULT_DECLARATION_CLAUSE)),
        item_kind,
    )
}

fn parse_ts_interface_export_default_declaration_clause(
    p: &mut JsParser,
    m: Marker,
) -> (ParsedSyntax, ExportDefaultItemKind) {
    // test ts ts_export_default_interface
    // export default interface A { }
    let declaration =
        TypeScript.parse_exclusive_syntax(p, parse_ts_interface_declaration, |p, interface| {
            ts_only_syntax_error(p, "interface", interface.range(p))
        });

    declaration.or_add_diagnostic(p, expected_export_default_declaration);

    (
        Present(m.complete(p, JS_EXPORT_DEFAULT_DECLARATION_CLAUSE)),
        ExportDefaultItemKind::Interface,
    )
}

fn parse_ts_enum_export_default_declaration_clause(
    p: &mut JsParser,
    m: Marker,
) -> (ParsedSyntax, ExportDefaultItemKind) {
    // test_err ts ts_export_default_enum
    // export default enum A { X, Y, Z }
    let declaration = parse_ts_enum_declaration(p).map(|enum_declaration| {
        p.error(
            p.err_builder(
                "'export default' isn't allowed for 'enum's. Move the 'enum' declaration in its own statement and then export the enum's name.",
                              enum_declaration.range(p))
        );

        enum_declaration
    });

    declaration.or_add_diagnostic(p, expected_export_default_declaration);

    (
        Present(m.complete(p, JS_EXPORT_DEFAULT_DECLARATION_CLAUSE)),
        ExportDefaultItemKind::Declaration,
    )
}

// test js export_default_expression_clause
// export default a;
//
// test_err js export_default_expression_clause_err
// export default a, b;
fn parse_export_default_expression_clause(
    p: &mut JsParser,
    m: Marker,
    start: TextSize,
) -> ParsedSyntax {
    if !is_at_expression(p) {
        // test_err js export_default_expression_broken
        // export default ;
        // export default @decorator
        // export default
        // export default @decorator
        m.abandon(p);
        return Absent;
    }

    parse_assignment_expression_or_higher(p, ExpressionContext::default())
        .or_add_diagnostic(p, expected_expression);

    semi(p, TextRange::new(start, p.cur_range().start()));
    Present(m.complete(p, JS_EXPORT_DEFAULT_EXPRESSION_CLAUSE))
}

fn parse_export_as_clause(p: &mut JsParser) -> ParsedSyntax {
    if !p.at(T![as]) {
        return Absent;
    }

    let m = p.start();
    p.expect(T![as]);

    parse_literal_export_name(p).or_add_diagnostic(p, expected_literal_export_name);

    Present(m.complete(p, JS_EXPORT_AS_CLAUSE))
}

// test ts ts_export_namespace_clause
// export function isPrime(x: number): boolean;
// export as namespace mathLib;
fn parse_ts_export_namespace_clause(p: &mut JsParser) -> ParsedSyntax {
    if !p.at(T![as]) && !p.nth_at(1, T![namespace]) {
        return Absent;
    }

    let m = p.start();
    let start_pos = p.cur_range().start();
    p.expect(T![as]);
    p.expect(T![namespace]);
    parse_name(p).or_add_diagnostic(p, expected_identifier);
    semi(p, TextRange::new(start_pos, p.cur_range().end()));

    Present(m.complete(p, TS_EXPORT_AS_NAMESPACE_CLAUSE))
}

// test ts ts_export_assignment_identifier
// declare const a: { b: string }
// export = a;
// export = class {}
// export = function () {}
// export = 4 + 3 + a;
//
// test ts ts_export_assignment_qualified_name
// declare const a: { b: string }
// export = a.b;
fn parse_ts_export_assignment_clause(p: &mut JsParser) -> ParsedSyntax {
    if !p.at(T![=]) {
        return Absent;
    }

    let m = p.start();
    let start_pos = p.cur_range().start();
    p.bump(T![=]);
    parse_assignment_expression_or_higher(p, ExpressionContext::default())
        .or_add_diagnostic(p, expected_expression);
    semi(p, TextRange::new(start_pos, p.cur_range().end()));
    Present(m.complete(p, TS_EXPORT_ASSIGNMENT_CLAUSE))
}

// test ts ts_export_declare
// export declare const a: string;
// export declare interface A {}
// export declare enum B {}
// export declare type C = string;
// export declare class D {}
// export declare function e()
fn parse_ts_export_declare_clause(p: &mut JsParser, stmt_start: TextSize) -> ParsedSyntax {
    if !p.at(T![declare]) {
        return Absent;
    }

    let m = p.start();
    p.expect(T![declare]);
    p.with_state(EnterAmbientContext, |p| {
        // test_err ts ts_export_declare
        // export declare @decorator class D {}
        // export declare @decorator abstract class D {}
        parse_declaration_clause(p, stmt_start).or_add_diagnostic(p, expected_declaration)
    });

    Present(m.complete(p, TS_EXPORT_DECLARE_CLAUSE))
}

fn is_nth_at_literal_export_name(p: &mut JsParser, n: usize) -> bool {
    match p.nth(n) {
        JS_STRING_LITERAL | T![ident] => true,
        t if t.is_keyword() || t.is_metavariable() => true,
        _ => false,
    }
}

fn parse_literal_export_name(p: &mut JsParser) -> ParsedSyntax {
    match p.cur() {
        JS_STRING_LITERAL | T![ident] => {
            let m = p.start();
            p.bump_any();
            Present(m.complete(p, JS_LITERAL_EXPORT_NAME))
        }
        t if t.is_keyword() => {
            let m = p.start();
            p.bump_remap(T![ident]);
            Present(m.complete(p, JS_LITERAL_EXPORT_NAME))
        }
        t if t.is_metavariable() => parse_metavariable(p),
        _ => Absent,
    }
}

pub(crate) fn parse_module_source(p: &mut JsParser) -> ParsedSyntax {
    if p.at(JS_STRING_LITERAL) {
        let m = p.start();
        p.bump_any();
        Present(m.complete(p, JS_MODULE_SOURCE))
    } else if is_at_metavariable(p) {
        parse_metavariable(p)
    } else {
        Absent
    }
}
