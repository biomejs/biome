//! Statements, these include `if`, `while`, `for`, `;`, and more.
//!
//! See the [ECMAScript spec](https://www.ecma-international.org/ecma-262/5.1/#sec-12).

use super::binding::*;
use super::class::is_at_ts_abstract_class_declaration;
use super::expr::parse_expression;
use super::metavariable::{is_at_metavariable, is_nth_at_metavariable, parse_metavariable};
use super::module::parse_export;
use super::typescript::*;
use crate::parser::RecoveryResult;
use crate::prelude::*;
use crate::state::{
    BreakableKind, ChangeParserState, EnableStrictMode, EnableStrictModeSnapshot, EnterBreakable,
    LabelledItem, StrictMode as StrictModeState, WithLabel,
};
use crate::syntax::assignment::expression_to_assignment_pattern;
use crate::syntax::class::{parse_class_declaration, parse_decorators, parse_initializer_clause};
use crate::syntax::expr::{
    is_at_expression, is_at_identifier, is_nth_at_identifier,
    parse_assignment_expression_or_higher, parse_expression_or_recover_to_next_statement,
    parse_identifier, ExpressionContext,
};
use crate::syntax::function::{is_at_async_function, parse_function_declaration, LineBreak};
use crate::syntax::js_parse_error;
use crate::syntax::js_parse_error::{decorators_not_allowed, expected_binding, expected_statement};
use crate::syntax::module::parse_import_or_import_equals_declaration;
use crate::syntax::typescript::ts_parse_error::{expected_ts_type, ts_only_syntax_error};

use crate::span::Span;
use crate::JsSyntaxFeature::{StrictMode, TypeScript};
use crate::ParsedSyntax::{Absent, Present};
use crate::{parser, JsParser, JsSyntaxFeature, ParseRecoveryTokenSet};
use biome_js_syntax::{JsSyntaxKind::*, *};
use biome_parser::diagnostic::expected_token;
use biome_parser::parse_lists::{ParseNodeList, ParseSeparatedList};
use biome_parser::ParserProgress;
use biome_rowan::SyntaxKind;

pub const STMT_RECOVERY_SET: TokenSet<JsSyntaxKind> = token_set![
    L_CURLY,
    VAR_KW,
    FUNCTION_KW,
    IF_KW,
    FOR_KW,
    DO_KW,
    WHILE_KW,
    CONTINUE_KW,
    BREAK_KW,
    RETURN_KW,
    WITH_KW,
    SWITCH_KW,
    THROW_KW,
    TRY_KW,
    DEBUGGER_KW,
    FUNCTION_KW,
    CLASS_KW,
    IMPORT_KW,
    EXPORT_KW,
    ABSTRACT_KW,
    INTERFACE_KW,
    ENUM_KW,
    TYPE_KW,
    DECLARE_KW,
    MODULE_KW,
    NAMESPACE_KW,
    LET_KW,
    CONST_KW,
    USING_KW,
    MODULE_KW,
    NAMESPACE_KW,
    GLOBAL_KW,
    T![@],
    T![;]
];

/// Consume an explicit semicolon, or try to automatically insert one,
/// or add an error to the parser if there was none and it could not be inserted
// test js semicolons
// let foo = bar;
// let foo2 = b;
// let foo3;
// let foo4
// let foo5
// function foo6() { return true }
pub(crate) fn semi(p: &mut JsParser, err_range: TextRange) -> bool {
    // test_err js semicolons_err
    // let foo = bar throw foo

    if !optional_semi(p) {
        let err = p
            .err_builder(
                "Expected a semicolon or an implicit semicolon after a statement, but found none",
                p.cur_range(),
            )
            .with_detail(
                p.cur_range(),
                "An explicit or implicit semicolon is expected here...",
            )
            .with_detail(err_range, "...Which is required to end this statement");

        p.error(err);
        false
    } else {
        true
    }
}

/// Eats a semicolon if present but doesn't add an error none is present and the automatic
/// semicolon insertion rule does not apply.
///
/// Returns false if neither a semicolon was present and the current position doesn't allow an automatic
/// semicolon insertion.
pub(crate) fn optional_semi(p: &mut JsParser) -> bool {
    if p.eat(T![;]) {
        return true;
    }

    is_semi(p, 0)
}

pub(super) fn is_semi(p: &mut JsParser, offset: usize) -> bool {
    p.nth_at(offset, T![;])
        || p.nth_at(offset, EOF)
        || p.nth_at(offset, T!['}'])
        || p.has_nth_preceding_line_break(offset)
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum StatementContext {
    If,
    Label,
    Do,
    While,
    With,
    For,
    // Block, Switch consequence, etc.
    StatementList,
}

impl StatementContext {
    pub(crate) fn is_single_statement(&self) -> bool {
        !matches!(self, StatementContext::StatementList)
    }

    pub(crate) fn is_statement_list(&self) -> bool {
        matches!(self, StatementContext::StatementList)
    }
}

/// A generic statement such as a block, if, while, with, etc
///
/// Error handling and recovering happens inside this function, so the
/// caller has to pass a recovery set.
///
/// If not passed, [STMT_RECOVERY_SET] will be used as recovery set
pub(crate) fn parse_statement(p: &mut JsParser, context: StatementContext) -> ParsedSyntax {
    match p.cur() {
        // test_err js import_decl_not_top_level
        // {
        //  import foo from "bar";
        // }

        // make sure we dont try parsing import.meta or import() as declarations
        T![import] if !token_set![T![.], T!['(']].contains(p.nth(1)) => {
            let mut import = parse_import_or_import_equals_declaration(p).unwrap();

            if import.kind(p) == TS_IMPORT_EQUALS_DECLARATION {
                return Present(import);
            }

            import.change_kind(p, JS_BOGUS_STATEMENT);

            let error = match p.source_type().module_kind() {
                ModuleKind::Script => p
                    .err_builder(
                        "Illegal use of an import declaration outside of a module",
                        import.range(p),
                    )
                    .with_hint("not allowed inside scripts"),
                ModuleKind::Module => p
                    .err_builder(
                        "Illegal use of an import declaration not at the top level",
                        import.range(p),
                    )
                    .with_hint("move this declaration to the top level"),
            };

            p.error(error);
            Present(import)
        }
        // test_err js export_decl_not_top_level
        // {
        //  export { pain } from "life";
        // }
        T![export] => parse_non_top_level_export(p, Absent),
        T![;] => parse_empty_statement(p),
        T!['{'] => parse_block_stmt(p),
        T![if] => parse_if_statement(p),
        T![with] => parse_with_statement(p),
        T![while] => parse_while_statement(p),
        T![const] | T![enum] if is_at_ts_enum_declaration(p) => {
            // test_err js enum_in_js
            // enum A {}
            TypeScript.parse_exclusive_syntax(p, parse_ts_enum_declaration, |p, declaration| {
                ts_only_syntax_error(p, "'enum's", declaration.range(p))
            })
        }
        T![var] => parse_variable_statement(p, context),
        T![const] => parse_variable_statement(p, context),
        T![using] if is_nth_at_using_declaration(p, 0) => parse_variable_statement(p, context),
        T![await] => {
            if is_nth_at_using_declaration(p, 0) {
                parse_variable_statement(p, context)
            } else {
                parse_expression_statement(p)
            }
        }
        T![for] => parse_for_statement(p),
        T![do] => parse_do_statement(p),
        T![switch] => parse_switch_statement(p),
        T![try] => parse_try_statement(p),
        T![return] => parse_return_statement(p),
        T![break] => parse_break_statement(p),
        T![continue] => parse_continue_statement(p),
        T![throw] => parse_throw_statement(p),
        T![debugger] => parse_debugger_statement(p),
        // function and async function
        T![function] => parse_function_declaration(p, context),
        T![async] if is_at_async_function(p, LineBreak::DoCheck) => {
            parse_function_declaration(p, context)
        }
        // class and abstract class
        T![class] => parse_class_declaration(p, Absent, context),
        T![@] => {
            let decorator_list = parse_decorators(p);

            match p.cur() {
                T![export] if p.nth_at(1, T![class]) => {
                    // test_err js decorator_export
                    // function foo() {
                    //      @decorator
                    //      export class Foo { }
                    //      @first.field @second @(() => decorator)()
                    //      export class Bar {}
                    // }
                    parse_non_top_level_export(p, decorator_list)
                }
                T![class] => {
                    // test js decorator_class_declaration
                    // function foo() {
                    //      @decorator
                    //      class Foo { }
                    //      @first.field @second @(() => decorator)()
                    //      class Bar {}
                    // }
                    parse_class_declaration(p, decorator_list, context)
                }
                T![abstract] if is_at_ts_abstract_class_declaration(p, LineBreak::DoCheck) => {
                    // test ts decorator_abstract_class_declaration
                    // function foo() {
                    //      @decorator abstract class A {}
                    //      @first.field @second @(() => decorator)()
                    //      abstract class Bar {}
                    // }
                    TypeScript.parse_exclusive_syntax(
                        p,
                        |p| parse_class_declaration(p, decorator_list, context),
                        |p, abstract_class| {
                            ts_only_syntax_error(p, "abstract classes", abstract_class.range(p))
                        },
                    )
                }
                _ => {
                    // test_err js decorator_class_declaration
                    // function bar() {
                    //      @decorator
                    //      let a;
                    //      @decorator @decorator2
                    //      function Foo() { }
                    // }
                    decorator_list
                        .add_diagnostic_if_present(p, decorators_not_allowed)
                        .map(|mut marker| {
                            marker.change_kind(p, JS_BOGUS_STATEMENT);
                            marker
                        });

                    parse_statement(p, context)
                }
            }
        }
        T![abstract] if is_at_ts_abstract_class_declaration(p, LineBreak::DoCheck) => {
            // test_err js abstract_class_in_js
            // abstract class A {}
            TypeScript.parse_exclusive_syntax(
                p,
                |p| parse_class_declaration(p, Absent, context),
                |p, abstract_class| {
                    ts_only_syntax_error(p, "abstract classes", abstract_class.range(p))
                },
            )
        }
        T![ident] if p.nth_at(1, T![:]) => parse_labeled_statement(p, context),
        _ if is_at_identifier(p) && p.nth_at(1, T![:]) => parse_labeled_statement(p, context),
        T![let]
            if is_nth_at_let_variable_statement(p, 0)
                && (p.cur_text() == "let" || !p.has_nth_preceding_line_break(1)) =>
        {
            // test_err js let_newline_in_async_function
            // async function f() {
            //   let
            //   await 0;
            // }

            // test js let_asi_rule
            // // SCRIPT
            // let // NO ASI
            // x = 1;
            // for await (var x of []) let // ASI
            // x = 1;

            // test_err js let_array_with_new_line
            // // SCRIPT
            // L: let
            // [a] = 0;
            if p.nth_at(1, T!['['])
                || context.is_statement_list()
                || !p.has_nth_preceding_line_break(1)
            {
                parse_variable_statement(p, context)
            } else {
                parse_expression_statement(p)
            }
        }
        T![type]
            if !p.has_nth_preceding_line_break(1)
                && (is_nth_at_identifier(p, 1) || is_nth_at_metavariable(p, 1)) =>
        {
            // test ts ts_type_variable
            // let type;
            // type = getFlowTypeInConstructor(symbol, getDeclaringConstructor(symbol)!);
            TypeScript.parse_exclusive_syntax(
                p,
                parse_ts_type_alias_declaration,
                |p, type_alias| ts_only_syntax_error(p, "type alias", type_alias.range(p)),
            )
        }
        T![interface] if is_at_ts_interface_declaration(p) => {
            TypeScript.parse_exclusive_syntax(p, parse_ts_interface_declaration, |p, interface| {
                ts_only_syntax_error(p, "interface", interface.range(p))
            })
        }
        T![declare] if is_at_ts_declare_statement(p) => {
            let declare_range = p.cur_range();
            TypeScript.parse_exclusive_syntax(p, parse_ts_declare_statement, |p, _| {
                p.err_builder(
                    "The 'declare' modifier can only be used in TypeScript files.",
                    declare_range,
                )
            })
        }
        T![async] if is_at_async_function(p, LineBreak::DoNotCheck) => {
            parse_function_declaration(p, context)
        }
        T![module] | T![namespace] | T![global] if is_nth_at_any_ts_namespace_declaration(p, 0) => {
            let name = p.cur_range();
            TypeScript.parse_exclusive_syntax(
                p,
                parse_any_ts_namespace_declaration_statement,
                |p, declaration| {
                    ts_only_syntax_error(p, p.text(name.as_range()), declaration.range(p))
                },
            )
        }
        _ if is_at_expression(p) || is_at_metavariable(p) => parse_expression_statement(p),
        _ => Absent,
    }
}

pub(crate) fn parse_non_top_level_export(
    p: &mut JsParser,
    decorator_list: ParsedSyntax,
) -> ParsedSyntax {
    parse_export(p, decorator_list).map(|mut export| {
        let error = match p.source_type().module_kind() {
            ModuleKind::Module => p
                .err_builder(
                    "Illegal use of an export declaration not at the top level",
                    export.range(p),
                )
                .with_hint("move this declaration to the top level"),
            ModuleKind::Script => p
                .err_builder(
                    "Illegal use of an export declaration outside of a module",
                    export.range(p),
                )
                .with_hint("not allowed inside scripts"),
        };

        p.error(error);
        export.change_kind(p, JS_BOGUS_STATEMENT);
        export
    })
}

// test js labeled_statement
// label1: 1
// label1: 1
// label2: 2
//
// test_err js double_label
// label1: {
//   label2: {
//     label1: {}
//   }
// }
//
// test js labelled_function_declaration
// // SCRIPT
// label1: function a() {}
//
// test_err js labelled_function_declaration_strict_mode
// label1: function a() {}
fn parse_labeled_statement(p: &mut JsParser, context: StatementContext) -> ParsedSyntax {
    let labelled_statement = p.start();
    let x = parse_identifier(p, JS_LABEL).map(|identifier| {
		fn parse_body(p: &mut JsParser, context: StatementContext) -> ParsedSyntax {
			if is_at_identifier(p) && p.nth_at(1, T![:]) && StrictMode.is_unsupported(p) {
				// Re-use the parent context to catch `if (true) label1: label2: function A() {}
				parse_labeled_statement(p, context)
			} else {
				parse_statement(p, StatementContext::Label)
			}
		}

		p.bump(T![:]);

		let identifier_range = identifier.range(p);
		let is_valid_identifier = !identifier.kind(p).is_bogus();
        let label = p.text(identifier_range);

		let body = match p.state().get_labelled_item(label) {
			None => {
				let labelled_item = match p.cur() {
					T![for] | T![do] | T![while] => LabelledItem::Iteration(identifier_range),
					_ => LabelledItem::Other(identifier_range)
				};
				let change = WithLabel(String::from(label), labelled_item);
				p.with_state(change, |p| parse_body(p, context))
			},
			Some(label_item) if is_valid_identifier => {
				let err = p
					.err_builder("Duplicate statement labels are not allowed", identifier_range)
					.with_detail(
						identifier_range,
						format!("a second use of `{label}` here is not allowed"),
					)
					.with_detail(
						*label_item.range(),
						format!("`{label}` is first used as a label here"),
					);

				p.error(err);
				parse_body(p, context)
			},
			Some(_) => {
				// Don't add another error, the identifier is already invalid
				parse_body(p, context)
			}
		};

        match body.or_add_diagnostic(p, expected_statement) {
            Some(mut body) if context.is_single_statement() && body.kind(p) == JS_FUNCTION_DECLARATION => {
                // test_err js labelled_function_decl_in_single_statement_context
                // if (true) label1: label2: function a() {}
                p.error(p.err_builder("Labelled function declarations are only allowed at top-level or inside a block", body.range(p)).with_hint( "Wrap the labelled statement in a block statement"));
                body.change_to_bogus(p);
            },
            // test js labelled_statement_in_single_statement_context
            // if (true) label1: var a = 10;
            _ => {}
        }
        identifier
    });
    if x.is_absent() {
        labelled_statement.abandon(p);
        ParsedSyntax::Absent
    } else {
        ParsedSyntax::Present(labelled_statement.complete(p, JS_LABELED_STATEMENT))
    }
}

// test js ts_keyword_assignments
// declare = 1;
// abstract = 2;
// namespace = 3;
// type = 4;
// module = 5;
// global = 6;
//
// test js ts_keywords_assignments_script
// // SCRIPT
// interface = 1;
// private = 2;
// protected = 3;
// public = 4;
// implements = 5;
fn parse_expression_statement(p: &mut JsParser) -> ParsedSyntax {
    let start = p.cur_range().start();

    let expr =
        parse_expression_or_recover_to_next_statement(p, false, ExpressionContext::default());

    if let Ok(expr) = expr {
        let m = expr.precede(p);
        semi(p, TextRange::new(start, p.cur_range().end()));
        Present(m.complete(p, JS_EXPRESSION_STATEMENT))
    } else {
        Absent
    }
}

// test js debugger_stmt
// debugger;

// test_err js debugger_stmt
// function foo() {
//   debugger {
//     var something = "lorem";
//   }
// }

/// A debugger statement such as `debugger;`
fn parse_debugger_statement(p: &mut JsParser) -> ParsedSyntax {
    if !p.at(T![debugger]) {
        return Absent;
    }
    let m = p.start();
    let range = p.cur_range();
    p.expect(T![debugger]); // debugger keyword
    semi(p, range);
    Present(m.complete(p, JS_DEBUGGER_STATEMENT))
}

/// A throw statement such as `throw new Error("uh oh");`
// test js throw_stmt
// throw new Error("foo");
// throw "foo"
fn parse_throw_statement(p: &mut JsParser) -> ParsedSyntax {
    // test_err js throw_stmt_err
    // throw
    // new Error("oh no :(")
    // throw;
    if !p.at(T![throw]) {
        return Absent;
    }
    let m = p.start();
    let start = p.cur_range().start();
    p.expect(T![throw]); // throw keyword
    if p.has_preceding_line_break() {
        let mut err = p
            .err_builder(
                "Linebreaks between a throw statement and the error to be thrown are not allowed",
                p.cur_range(),
            )
            .with_hint("A linebreak is not allowed here");

        if is_at_expression(p) {
            err = err.with_detail(p.cur_range(), "Help: did you mean to throw this?");
        }

        p.error(err);
    } else {
        parse_expression_or_recover_to_next_statement(p, false, ExpressionContext::default()).ok();
    }

    semi(p, TextRange::new(start, p.cur_range().end()));
    Present(m.complete(p, JS_THROW_STATEMENT))
}

// test js break_stmt
// while (true) {
//   break;
//   foo: {
//    break foo;
//   }
// }
// out: while (true) {
//   break out;
// }

// test_err js break_stmt
// function foo() { break; }
// while (true) {
//   break foo;
// }

/// A break statement with an optional label such as `break a;`
fn parse_break_statement(p: &mut JsParser) -> ParsedSyntax {
    if !p.at(T![break]) {
        return Absent;
    }
    let m = p.start();
    let start = p.cur_range();
    p.expect(T![break]); // break keyword

    let error = if !p.has_preceding_line_break() && is_at_identifier(p) {
        let label_name = p.cur_text();

        let error = match p.state().get_labelled_item(label_name) {
            Some(_) => None,
            None => Some(
                p.err_builder(
                    format!("Use of undefined statement label `{label_name}`",),
                    p.cur_range(),
                )
                .with_hint("This label is used, but it is never defined"),
            ),
        };
        let _ = parse_identifier(p, JS_LABEL);
        error
    } else if !p.state().break_allowed() {
        Some(p.err_builder("A `break` statement can only be used within an enclosing iteration or switch statement.", start, ))
    } else {
        None
    };

    semi(p, TextRange::new(start.start(), p.cur_range().end()));

    if let Some(error) = error {
        p.error(error);
        Present(m.complete(p, JS_BOGUS_STATEMENT))
    } else {
        Present(m.complete(p, JS_BREAK_STATEMENT))
    }
}

// test js continue_stmt
// outer: while(true) {
// while (true) {
//   continue;
//     continue outer;
//    }
//   continue
// }

// test_err js continue_stmt
// function foo() { continue; }
// while (true) {
//   continue foo;
// }
// foo: {
//   continue foo;
// }
/// A continue statement with an optional label such as `continue a;`
fn parse_continue_statement(p: &mut JsParser) -> ParsedSyntax {
    if !p.at(T![continue]) {
        return Absent;
    }
    let m = p.start();
    let start = p.cur_range();
    p.expect(T![continue]); // continue keyword

    // test js async_continue_stmt
    // async: for(a of b) continue async;
    let error = if !p.has_preceding_line_break() && is_at_identifier(p) {
        let label_name = p.cur_text();

        let error = match p.state().get_labelled_item(label_name) {
			Some(LabelledItem::Iteration(_)) => None,
			Some(LabelledItem::Other(range)) => {
				Some(p.err_builder("A `continue` statement can only jump to a label of an enclosing `for`, `while` or `do while` statement.", p.cur_range())
					.with_detail(p.cur_range(), "This label")
					.with_detail(*range, "points to non-iteration statement"))
			}
			None => {
				Some(p
					.err_builder(format!(
						"Use of undefined statement label `{label_name}`"
					), p.cur_range())
					.with_hint(

						"This label is used, but it is never defined",
					))
			}
		};

        let _ = parse_identifier(p, JS_LABEL);
        error
    } else if !p.state().continue_allowed() {
        Some(
            p.err_builder(
                "A `continue` statement can only be used within an enclosing `for`, `while` or `do while` statement.",start ),
        )
    } else {
        None
    };

    semi(p, TextRange::new(start.start(), p.cur_range().end()));

    if let Some(error) = error {
        p.error(error);
        Present(m.complete(p, JS_BOGUS_STATEMENT))
    } else {
        Present(m.complete(p, JS_CONTINUE_STATEMENT))
    }
}

// test js return_stmt
// () => {
//   return;
//   return foo;
//   return
// }
/// A return statement with an optional value such as `return a;`
fn parse_return_statement(p: &mut JsParser) -> ParsedSyntax {
    // test_err js return_stmt_err
    // return;
    // return foo;
    if !p.at(T![return]) {
        return Absent;
    }
    let m = p.start();
    let start = p.cur_range().start();
    p.expect(T![return]);
    if !p.has_preceding_line_break() {
        parse_expression(p, ExpressionContext::default()).ok();
    }

    semi(p, TextRange::new(start, p.cur_range().end()));
    let mut complete = m.complete(p, JS_RETURN_STATEMENT);

    // The frontmatter of Astro files is executed inside a function during the compilation, so it's safe to have illegal returns
    if !p.state().in_function() && !p.source_type.as_embedding_kind().is_astro() {
        let err = p.err_builder(
            "Illegal return statement outside of a function",
            complete.range(p),
        );

        p.error(err);
        complete.change_kind(p, JS_BOGUS_STATEMENT);
    }
    Present(complete)
}

// test js empty_stmt
// ;
/// An empty statement denoted by a single semicolon.
fn parse_empty_statement(p: &mut JsParser) -> ParsedSyntax {
    if p.at(T![;]) {
        let m = p.start();
        p.bump_any(); // bump ;
        m.complete(p, JS_EMPTY_STATEMENT).into()
    } else {
        Absent
    }
}

// test js block_stmt
// {}
// {{{{}}}}
// { foo = bar; }
/// A block statement consisting of statements wrapped in curly brackets.
pub(crate) fn parse_block_stmt(p: &mut JsParser) -> ParsedSyntax {
    parse_block_impl(p, JS_BLOCK_STATEMENT)
}

/// A block wrapped in curly brackets. Can either be a function body or a block statement.
pub(super) fn parse_block_impl(p: &mut JsParser, block_kind: JsSyntaxKind) -> ParsedSyntax {
    if !p.at(T!['{']) {
        return Absent;
    }

    let m = p.start();
    p.bump(T!['{']);

    let (statement_list, strict_snapshot) = if block_kind == JS_FUNCTION_BODY {
        parse_directives(p)
    } else {
        (p.start(), None)
    };

    parse_statements(p, true, statement_list);

    p.expect(T!['}']);

    if let Some(strict_snapshot) = strict_snapshot {
        EnableStrictMode::restore(p.state_mut(), strict_snapshot);
    }

    Present(m.complete(p, block_kind))
}

// test js directives
// // SCRIPT
// "use new"
// let a = 10;
// "use strict"; // not a directive
// function test() {
//   'use strict';
//   let b = 10;
//   'use strict'; // not a directive
// }
// (function () {
//   "use strict";
//   "use strict"
//     .length; // not a directive
//   let c = 10;
//   "use strict"; // not a directive
// });
// let b = () => {
//   "use strict";
//   let e = 10;
//   "use strict";  // not a directive
// }
// {
//   "use strict"; // not a directive
// }
//
//
// test js directives_redundant
// // SCRIPT
// function test() {
//   "use strict";
//   function inner_a() {
//     "use strict";
//   }
//   function inner_b() {
//     function inner_inner() {
//       "use strict";
//     }
//   }
// }
/// Parses the directives and returns
/// * The marker for the following statement list. May already contain a parsed out expression statement
/// * A checkpoint containing the previous strict mode
pub(crate) fn parse_directives(p: &mut JsParser) -> (Marker, Option<EnableStrictModeSnapshot>) {
    let list = p.start();
    let mut directives_list = list.complete(p, JS_DIRECTIVE_LIST);
    let mut strict_mode_snapshot: Option<EnableStrictModeSnapshot> = None;
    let mut progress = ParserProgress::default();

    let statement_list = loop {
        progress.assert_progressing(p);
        // Certainly not a directive, start statement list
        if !p.at(JS_STRING_LITERAL) {
            break p.start();
        }

        let expression = parse_expression(p, ExpressionContext::default())
            .expect("A string token always yields a valid expression");

        // Something like "use strict".length isn't a valid directive
        if expression.kind(p) != JS_STRING_LITERAL_EXPRESSION {
            // Turned out not to be a directive.
            // Start statement list before the just parsed expression statement
            let statement = expression.precede(p).complete(p, JS_EXPRESSION_STATEMENT);
            break statement.precede(p);
        }

        let directive_range = expression.range(p);
        let directive = expression.undo_completion(p);
        semi(p, directive_range);

        let directive_text = p.text(directive_range);

        let directive_is_use_strict =
            directive_text == "\"use strict\"" || directive_text == "'use strict'";

        if directive_is_use_strict && strict_mode_snapshot.is_none() {
            strict_mode_snapshot = Some(
                EnableStrictMode(StrictModeState::Explicit(directive_range)).apply(p.state_mut()),
            );
        }

        directive.complete(p, JS_DIRECTIVE);

        // Extend the directive list to include the just parsed directive
        directives_list = directives_list
            .undo_completion(p)
            .complete(p, JS_DIRECTIVE_LIST);
    };

    (statement_list, strict_mode_snapshot)
}

/// Top level items or items inside of a block statement, this also handles module items so we can
/// easily recover from erroneous module declarations in scripts
pub(crate) fn parse_statements(p: &mut JsParser, stop_on_r_curly: bool, statement_list: Marker) {
    let mut progress = ParserProgress::default();

    // test_err js statements_closing_curly
    // {
    // "name": "troublesome-lib",
    // "typings": "lib/index.d.ts",
    // "version": "0.0.1"
    // }
    let recovery_set = if stop_on_r_curly {
        // Don't eat over the closing '}'
        STMT_RECOVERY_SET.union(token_set![T!['}']])
    } else {
        STMT_RECOVERY_SET
    };

    while !p.at(EOF) {
        progress.assert_progressing(p);
        if stop_on_r_curly && p.at(T!['}']) {
            break;
        }

        if parse_metavariable(p).is_present() {
            continue;
        }

        if parse_statement(p, StatementContext::StatementList)
            .or_recover_with_token_set(
                p,
                &ParseRecoveryTokenSet::new(JS_BOGUS_STATEMENT, recovery_set),
                expected_statement,
            )
            .is_err()
        {
            break;
        }
    }

    statement_list.complete(p, JS_STATEMENT_LIST);
}

/// An expression wrapped in parentheses such as `()`
/// Returns `true` if the closing parentheses is present
fn parenthesized_expression(p: &mut JsParser) -> bool {
    let has_l_paren = p.expect(T!['(']);

    parse_expression(
        p,
        ExpressionContext::default().and_object_expression_allowed(has_l_paren),
    )
    .or_add_diagnostic(p, js_parse_error::expected_expression);

    p.expect(T![')'])
}

/// An if statement such as `if (foo) { bar(); }`
// test js if_stmt
// if (true) {} else {}
// if (true) {}
// if (true) false
// if (bar) {} else if (true) {} else {}
fn parse_if_statement(p: &mut JsParser) -> ParsedSyntax {
    // test_err js if_stmt_err
    // if (true) else {}
    // if (true) else
    // if else {}
    // if () {} else {}
    // if (true)}}}} {}
    if !p.at(T![if]) {
        return Absent;
    }

    let m = p.start();
    p.expect(T![if]);

    // (test)
    parenthesized_expression(p);

    // body
    parse_statement(p, StatementContext::If).or_add_diagnostic(p, expected_statement);

    // else clause
    if p.at(T![else]) {
        let else_clause = p.start();
        p.expect(T![else]);
        parse_statement(p, StatementContext::If).or_add_diagnostic(p, expected_statement);
        else_clause.complete(p, JS_ELSE_CLAUSE);
    }

    Present(m.complete(p, JS_IF_STATEMENT))
}

// test js with_statement
// // SCRIPT
// function f(x, o) {
//   with (o) {
//     console.log(x);
//   }
// }
/// A with statement such as `with (foo) something()`
fn parse_with_statement(p: &mut JsParser) -> ParsedSyntax {
    if !p.at(T![with]) {
        return Absent;
    }

    let m = p.start();
    p.expect(T![with]);
    parenthesized_expression(p);

    parse_statement(p, StatementContext::With).or_add_diagnostic(p, expected_statement);

    let with_stmt = m.complete(p, JS_WITH_STATEMENT);

    // or SloppyMode.exclusive_syntax(...) but this reads better with the error message, saying that
    // it's only forbidden in strict mode
    StrictMode.excluding_syntax(p, with_stmt, |p, marker| {
        p.err_builder(
            "`with` statements are not allowed in strict mode",
            marker.range(p),
        )
    })
}

/// A while statement such as `while(true) { do_something() }`
// test js while_stmt
// while (true) {}
// while (5) {}
fn parse_while_statement(p: &mut JsParser) -> ParsedSyntax {
    // test_err js while_stmt_err
    // while true {}
    // while {}
    // while (true {}
    // while true) }
    if !p.at(T![while]) {
        return Absent;
    }
    let m = p.start();
    p.expect(T![while]);
    parenthesized_expression(p);

    p.with_state(EnterBreakable(BreakableKind::Iteration), |p| {
        parse_statement(p, StatementContext::While)
    })
    .or_add_diagnostic(p, expected_statement);

    Present(m.complete(p, JS_WHILE_STATEMENT))
}

pub(crate) fn is_nth_at_variable_declarations(p: &mut JsParser, n: usize) -> bool {
    match p.nth(n) {
        T![var] | T![const] => true,
        T![await] | T![using] if is_nth_at_using_declaration(p, n) => true,
        T![let] if is_nth_at_let_variable_statement(p, n) => true,
        _ => false,
    }
}

pub(crate) fn is_nth_at_using_declaration(p: &mut JsParser, n: usize) -> bool {
    let (maybe_using, next_cursor) = match p.nth(n) {
        T![using] => (true, n + 1),
        T![await] if p.nth_at(n + 1, T![using]) => (true, n + 2),
        _ => (false, n + 1),
    };

    maybe_using
        && !p.has_nth_preceding_line_break(next_cursor)
        && !p.nth_at(next_cursor, T![await])
        && is_nth_at_identifier(p, next_cursor)
}

pub(crate) fn is_nth_at_let_variable_statement(p: &mut JsParser, n: usize) -> bool {
    if !p.nth_at(n, T![let]) {
        return false;
    }

    matches!(p.nth(n + 1), T!['{'] | T!['[']) || is_nth_at_identifier(p, n + 1)
}

/// A var, const, using or let declaration statement such as `var a = 5, b;` or `let {a, b} = foo;`
// test js var_decl
// var a = 5;
// let { foo, bar } = 5;
// let bar2, foo2;
// const b = 5;
// const { foo5: [bar11], baz6 } = {};
// let foo6 = "lorem", bar7 = "ipsum", third8 = "value", fourth = 6;
// var q, w, e, r, t;
//
// test js using_declaration_statement
// using a = b;
// using c = d, e = _;
// using [g] = h;
// using [j]
// = k;
// await using l = m;
// await
// using p = q;
// await using[r];
// await using ([s] = t);
// await (using [u] = v);
// using w = {};
// using x = null;
// using y = undefined;
// using z = (foo, bar);
//
// test_err js variable_declaration_statement_err
// let a, { b } = { a: 10 }
// const c = 1, { d } = { a: 10 }
// const e;
// let [f];
// const { g };
//
// test_err js using_declaration_statement_err
// using a;
// using {b};
// using c = d, e;
// export using m = n;
// await using f;
// await using g = h, j;
// await using [o] = p;
// export await using q = r;
// await let s;
// await const t = 1;
pub(crate) fn parse_variable_statement(
    p: &mut JsParser,
    context: StatementContext,
) -> ParsedSyntax {
    // test_err js var_decl_err
    // var a =;
    // const b = 5 let c = 5;
    let start = p.cur_range().start();
    let is_var = p.at(T![var]);
    let is_await_using = p.at(T![await]) && p.nth_at(1, T![using]);

    parse_variable_declaration(p, VariableDeclarationParent::VariableStatement).map(|declaration| {
        let m = declaration.precede(p);
        semi(p, TextRange::new(start, p.cur_range().start()));

        let mut statement = m.complete(p, JS_VARIABLE_STATEMENT);

        if !is_var && context.is_single_statement() {
            // test js hoisted_declaration_in_single_statement_context
            // if (true) var a;
            //
            // test_err js lexical_declaration_in_single_statement_context
            // if (true) let a;
            // while (true) const b = 5;
            p.error(
                p.err_builder(
                    "Lexical declaration cannot appear in a single-statement context",
                    statement.range(p),
                )
                .with_hint("Wrap this declaration in a block statement"),
            );
            statement.change_to_bogus(p);
        }

        let is_top_level_module_or_async_fn =
            p.state().in_async() && (p.state().is_top_level() || p.state().in_function());
        if is_await_using && !is_top_level_module_or_async_fn {
            // test_err js await_using_declaration_only_allowed_inside_an_async_function
            // function foo() { await using x = y };
            // foo = function() { await using x = y };
            // foo = () => { await using x = y };
            p.error(
                p.err_builder(
                    "`await using` declarations are only allowed at top-level or inside an async function",
                    statement.range(p),
                )
                .with_hint("Wrap this declaration in an async function"),
            );
            statement.change_to_bogus(p);
        }

        statement
    })
}

pub(super) fn parse_variable_declaration(
    p: &mut JsParser,
    declaration_context: VariableDeclarationParent,
) -> ParsedSyntax {
    let m = p.start();
    if eat_variable_declaration(p, declaration_context).is_some() {
        Present(m.complete(p, JS_VARIABLE_DECLARATION))
    } else {
        m.abandon(p);
        Absent
    }
}

/// What's the parent node of the variable declaration
#[derive(Clone, Debug, Copy, Eq, PartialEq)]
pub(super) enum VariableDeclarationParent {
    /// Declaration inside a `for...of` or `for...in` or `for (;;)` loop
    For,

    /// Declaration as part of a variable statement (`let a`, `const b`, or `var c`).
    VariableStatement,

    /// Declaration as part of another statement, like `export let ...` or `declare let a`
    Clause,
}

/// Parses and consume variable declarations like `var`/`let`/`const`/`using`/`await using`.
/// Returns a tuple where
/// * the first element is the marker to the not yet completed list
/// * the second element is the range of all variable declarations except the first one. Is [None] if
///   there's only one declaration.
fn eat_variable_declaration(
    p: &mut JsParser,
    declaration_parent: VariableDeclarationParent,
) -> Option<(CompletedMarker, Option<TextRange>)> {
    let mut context = VariableDeclaratorContext::new(declaration_parent);

    match p.cur() {
        T![var] => {
            p.bump(T![var]);
        }
        T![const] => {
            p.bump(T![const]);
            context.kind_name = Some("const");
        }
        T![let] => {
            p.bump(T![let]);
            context.kind_name = Some("let");
        }
        T![using] => {
            p.bump(T![using]);
            context.kind_name = Some("using");
        }
        T![await] if p.nth_at(1, T![using]) => {
            p.bump(T![await]);
            p.bump(T![using]);
            context.kind_name = Some("using");
        }
        _ => {
            return None;
        }
    }

    let mut variable_declarator_list = VariableDeclaratorList {
        declarator_context: context,
        remaining_declarator_range: None,
    };
    let list = variable_declarator_list.parse_list(p);

    p.state_mut().name_map.clear();
    Some((list, variable_declarator_list.remaining_declarator_range))
}

struct VariableDeclaratorList {
    declarator_context: VariableDeclaratorContext,
    // Range of the declarators succeeding the first declarator
    // None until this hits the second declarator
    remaining_declarator_range: Option<TextRange>,
}

// test_err js variable_declarator_list_incomplete
// const a = 1,
//
// test_err js variable_declarator_list_empty
// const;
// const
impl ParseSeparatedList for VariableDeclaratorList {
    type Kind = JsSyntaxKind;
    type Parser<'source> = JsParser<'source>;

    const LIST_KIND: Self::Kind = JS_VARIABLE_DECLARATOR_LIST;

    fn parse_element(&mut self, p: &mut JsParser) -> ParsedSyntax {
        parse_variable_declarator(p, &self.declarator_context).map(|declarator| {
            if self.declarator_context.is_first {
                self.declarator_context.is_first = false;
            } else if let Some(range) = self.remaining_declarator_range.as_mut() {
                *range = TextRange::new(range.start(), declarator.range(p).end());
            } else {
                self.remaining_declarator_range = Some(declarator.range(p));
            }
            declarator
        })
    }

    fn is_at_list_end(&self, p: &mut JsParser) -> bool {
        if self.declarator_context.is_first {
            false
        } else {
            !p.at(T![,])
        }
    }

    fn recover(&mut self, p: &mut JsParser, parsed_element: ParsedSyntax) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(JS_BOGUS, STMT_RECOVERY_SET.union(token_set!(T![,])))
                .enable_recovery_on_line_break(),
            expected_binding,
        )
    }

    fn separating_element_kind(&mut self) -> JsSyntaxKind {
        T![,]
    }

    fn finish_list(&mut self, p: &mut JsParser, m: Marker) -> CompletedMarker {
        if self.declarator_context.is_first {
            let m = m.complete(p, JS_BOGUS);
            let range = m.range(p);
            p.error(expected_binding(p, range));
            m
        } else {
            m.complete(p, Self::LIST_KIND)
        }
    }
}

struct VariableDeclaratorContext {
    /// What kind of variable declaration is this (`var`, `let`, `const`, 'using')
    kind_name: Option<&'static str>,
    /// Is this the first declaration in the declaration list (a first, b second in `let a, b`)
    is_first: bool,
    /// What's the parent of the variable declaration
    parent: VariableDeclarationParent,
}

impl VariableDeclaratorContext {
    fn new(parent: VariableDeclarationParent) -> Self {
        Self {
            parent,
            kind_name: None,
            is_first: true,
        }
    }

    fn is_var(&self) -> bool {
        self.kind_name.is_none()
    }

    fn is_const(&self) -> bool {
        matches!(self.kind_name, Some("const"))
    }

    fn is_using(&self) -> bool {
        matches!(self.kind_name, Some("using"))
    }
}

// test js scoped_declarations
// let a = {
//   test() {
//     let a = "inner";
//   }
// };
//
// test js using_declarations_inside_for_statement
// for (using x of y) {};
// for await (using x of y) {};
// for (await using x of y) {};
// for await (await using x of y) {};
//
// test_err js invalid_using_declarations_inside_for_statement
// for (await using of x) {};
// for await (await using of x) {};
//
// A single declarator, either `ident` or `ident = assign_expr`
fn parse_variable_declarator(
    p: &mut JsParser,
    context: &VariableDeclaratorContext,
) -> ParsedSyntax {
    p.state_mut().duplicate_binding_parent = context.kind_name;
    let id = parse_binding_pattern(p, ExpressionContext::default());
    p.state_mut().duplicate_binding_parent = None;

    id.map(|id| {
        let id_kind = id.kind(p);
        let id_range = id.range(p);
        let m = id.precede(p);

        let ts_annotation = TypeScript.parse_exclusive_syntax(p, parse_ts_variable_annotation,
            |p, annotation| {
                let name = match annotation.kind(p) {
                    TS_TYPE_ANNOTATION => "type annotation",
                    TS_DEFINITE_VARIABLE_ANNOTATION => "definite assertion assignments",
                    _ => unreachable!(),
                };

                ts_only_syntax_error(p, name, annotation.range(p))
            })
            .ok();

        let last_name_map = std::mem::take(&mut p.state_mut().name_map);
        let duplicate_binding_parent = p.state_mut().duplicate_binding_parent.take();

        let mut initializer = parse_initializer_clause(
            p,
            ExpressionContext::default()
                .and_include_in(context.parent != VariableDeclarationParent::For),
        )
        .ok();

        if let (Some(initializer), Some(ts_annotation)) =
            (initializer.as_mut(), ts_annotation.as_ref())
        {
            if ts_annotation.kind(p) == TS_DEFINITE_VARIABLE_ANNOTATION {
                // test_err ts ts_definite_variable_with_initializer
                // let a!: string = "test";
                p.error(
                    p
                        .err_builder("Declarations with initializers cannot also have definite assignment assertions.", initializer.range(p))

                        .with_detail(ts_annotation.range(p), "Annotation")
                );
            }
        }

        p.state_mut().name_map = last_name_map;
        p.state_mut().duplicate_binding_parent = duplicate_binding_parent;

        // Heuristic to determine if we're in a for of or for in loop. This may be off if
        // the user uses a for of/in with multiple declarations but this isn't allowed anyway.
        let is_in_for_loop = context.parent == VariableDeclarationParent::For && context.is_first;
        let is_in_for_of = is_in_for_loop && p.at(T![of]);
        let is_in_for_in = is_in_for_loop && p.at(T![in]);

        if is_in_for_of || is_in_for_in {
            if TypeScript.is_supported(p) {
                if let Some(mut ts_annotation) = ts_annotation {
                    let err = p
                        .err_builder("`for` statement declarators cannot have a type annotation", ts_annotation.range(p));

                    p.error(err);
                    ts_annotation.change_to_bogus(p);
                }
            }


            // test_err js using_declaration_not_allowed_in_for_in_statement
            // for (using x in y) {};
            // for (await using x in y) {};
            if context.is_using() && is_in_for_in {
                let err = p
                    .err_builder("The left-hand side of a 'for...in' statement cannot be a 'using' declaration", id_range);
                p.error(err);
            }

            if let Some(initializer) = initializer {
                // Initializers are disallowed for `for..in` and `for..of`,
                // except for `for(var ... in ...)` in loose mode

                // test js for_in_initializer_loose_mode
                // // SCRIPT
                // for (var i = 0 in []) {}

                // test_err js for_in_and_of_initializer_loose_mode
                // // SCRIPT
                // for (let i = 0 in []) {}
                // for (const i = 0 in []) {}
                // for (var i = 0 of []) {}
                // for (let i = 0 of []) {}
                // for (const i = 0 of []) {}

                // test_err js for_in_and_of_initializer_strict_mode
                // for (var i = 0 in []) {}
                // for (let i = 0 in []) {}
                // for (const i = 0 in []) {}
                // for (var i = 0 of []) {}
                // for (let i = 0 of []) {}
                // for (const i = 0 of []) {}
                // for (using x = y of z) {};
                // for await (using x = y of z) {};
                // for (await using x = y of z) {};
                // for await (await using x = y of z) {};

                let is_strict = StrictMode.is_supported(p);

                if is_strict || !is_in_for_in || !context.is_var() {
                    let err = p
                        .err_builder(if is_in_for_in {
                            "`for..in` statement declarators cannot have an initializer expression"
                        } else {
                            "`for..of` statement declarators cannot have an initializer expression"
                        }, initializer.range(p));

                    p.error(err);
                }
            }
        } else if initializer.is_none()
            && !p.state().in_ambient_context()
            && matches!(
                id_kind,
                JS_ARRAY_BINDING_PATTERN | JS_OBJECT_BINDING_PATTERN
            )
        {
            let err = p
                .err_builder("Object and Array patterns require initializers.", id_range)
                .with_hint(
                    "This pattern is declared, but it is not given an initialized value.",
                );

            p.error(err);
        } else if initializer.is_none() && context.is_const() && !p.state().in_ambient_context() {
            let err = p
                .err_builder("Const declarations must have an initialized value.", id_range)
                .with_hint( "This variable needs to be initialized.");

            p.error(err);
        } else if initializer.is_none() && context.is_using() {
            let err = p
                .err_builder("Using declarations must have an initialized value.", id_range)
                .with_hint( "This variable needs to be initialized.");

            p.error(err);
        }

        m.complete(p, JS_VARIABLE_DECLARATOR)
    })
}

// test_err js js_type_variable_annotation
// let a: string, b!: number;
//
// test_err ts ts_variable_annotation_err
// let a!;
//
// test ts ts_type_variable_annotation
// let a: string = "test", b!: number;
// let a // ASI
// !function test() {}
fn parse_ts_variable_annotation(p: &mut JsParser) -> ParsedSyntax {
    if !p.at(T![!]) {
        return parse_ts_type_annotation(p, TypeContext::default());
    }

    if p.has_preceding_line_break() {
        return Absent;
    }

    let m = p.start();
    p.bump(T![!]);

    parse_ts_type_annotation(p, TypeContext::default())
        .or_add_diagnostic(p, |_, _| expected_token(T![:]));

    Present(m.complete(p, TS_DEFINITE_VARIABLE_ANNOTATION))
}

// A do.. while statement, such as `do {} while (true)`

// test js do_while_statement
// do console.log("test"); while(true)
// do {
//   console.log("test")
// } while (true);
// let a = 1;
// do
// do {
//   a = a + 1
// } while(a < 5)
// while (a < 100)
//
// test js do_while_stmt
// do { } while (true)
// do { throw Error("foo") } while (true)
// do { break; } while (true)
fn parse_do_statement(p: &mut JsParser) -> ParsedSyntax {
    // test_err js do_while_stmt_err
    // do while (true)
    // do while ()
    // do while true

    // test_err js do_while_no_continue_break
    // do { } break (continue)
    // do { } continue (break)
    if !p.at(T![do]) {
        return Absent;
    }
    let m = p.start();
    let start = p.cur_range().start();
    p.expect(T![do]);

    p.with_state(EnterBreakable(BreakableKind::Iteration), |p| {
        parse_statement(p, StatementContext::Do)
    })
    .or_add_diagnostic(p, expected_statement);

    p.expect(T![while]);

    // test js do-while-asi
    // do do do ; while (x) while (x) while (x) x = 39;
    // do do ; while (x); while (x) x = 39
    if parenthesized_expression(p) {
        optional_semi(p);
    } else {
        let end_range = p.cur_range().end();
        semi(p, TextRange::new(start, end_range));
    }
    Present(m.complete(p, JS_DO_WHILE_STATEMENT))
}

/// Parses the header of a for statement into the current node and returns whatever it is a for in/of or "regular" for statement
fn parse_for_head(p: &mut JsParser, has_l_paren: bool, is_for_await: bool) -> JsSyntaxKind {
    // for (;...
    if p.at(T![;]) {
        parse_normal_for_head(p);
        return JS_FOR_STATEMENT;
    }

    // `for (let...` | `for (const...` | `for (var...`

    if is_nth_at_variable_declarations(p, 0) {
        let m = p.start();

        let (declarations, additional_declarations) =
            eat_variable_declaration(p, VariableDeclarationParent::For).unwrap();

        let is_in = p.at(T![in]);
        let is_of = p.at(T![of]);

        if is_in || is_of {
            // remove the intermediate list node created by parse variable declarations that is not needed
            // for a ForInOrOfInitializer where the variable declaration is a direct child.
            declarations.undo_completion(p).abandon(p);

            if let Some(additional_declarations_range) = additional_declarations {
                p.error(
                    p.err_builder(
                        format!(
                            "Only a single declaration is allowed in a `for...{}` statement.",
                            if is_of { "of" } else { "in" },
                        ),
                        additional_declarations_range,
                    )
                    .with_hint("additional declarations"),
                );
            }

            m.complete(p, JS_FOR_VARIABLE_DECLARATION);

            parse_for_of_or_in_head(p)
        } else {
            m.complete(p, JS_VARIABLE_DECLARATION);
            parse_normal_for_head(p);
            JS_FOR_STATEMENT
        }
    } else {
        // for (some_expression`
        let checkpoint = p.checkpoint();

        let starts_with_async_of =
            p.at(T![async]) && p.nth_at(1, T![of]) && p.cur_text() == "async";
        let init_expr = parse_expression(
            p,
            ExpressionContext::default()
                .and_include_in(false)
                .and_object_expression_allowed(has_l_paren),
        );

        if p.at(T![in]) || p.at(T![of]) {
            // for (assignment_pattern in ...
            if let Present(mut assignment) = init_expr.and_then(|assignment_expr| {
                expression_to_assignment_pattern(p, assignment_expr, checkpoint)
            }) {
                if TypeScript.is_supported(p)
                    && p.at(T![in])
                    && matches!(
                        assignment.kind(p),
                        JS_ARRAY_ASSIGNMENT_PATTERN | JS_OBJECT_ASSIGNMENT_PATTERN
                    )
                {
                    let err = p.err_builder("the left hand side of a `for..in` statement cannot be a destructuring pattern", assignment.range(p));
                    p.error(err);
                    assignment.change_to_bogus(p);
                } else if !is_for_await && starts_with_async_of {
                    //  for ( [lookahead ∉ { let, async of }] LeftHandSideExpression[?Yield, ?Await] of AssignmentExpression[+In, ?Yield, ?Await] ) Statement[?Yield, ?Await, ?Return]
                    // [+Await] for await ( [lookahead ≠ let] LeftHandSideExpression[?Yield, ?Await] of AssignmentExpression[+In, ?Yield, ?Await] ) Statement[?Yield, ?Await, ?Return]

                    // test js for_await_async_identifier
                    // let async;
                    // async function fn() {
                    //   for await (async of [7]);
                    // }

                    // test_err js for_of_async_identifier
                    // let async;
                    // for (async of [1]) ;
                    p.error(p.err_builder(
                        "The left-hand side of a `for...of` statement may not be `async`",
                        assignment.range(p),
                    ));
                    assignment.change_to_bogus(p);
                }
            }

            return parse_for_of_or_in_head(p);
        }

        init_expr.or_add_diagnostic(p, js_parse_error::expected_expression);

        parse_normal_for_head(p);
        JS_FOR_STATEMENT
    }
}

/// Parses the parenthesized part of a non for in or for of statement
/// Expects to be positioned right after the initializer
fn parse_normal_for_head(p: &mut JsParser) {
    p.expect(T![;]);

    if !p.at(T![;]) {
        parse_expression(p, ExpressionContext::default())
            .or_add_diagnostic(p, js_parse_error::expected_expression);
    }

    p.expect(T![;]);

    if !p.at(T![')']) {
        parse_expression(p, ExpressionContext::default())
            .or_add_diagnostic(p, js_parse_error::expected_expression);
    }
}

/// Expects to be positioned right before the of or in keyword
fn parse_for_of_or_in_head(p: &mut JsParser) -> JsSyntaxKind {
    let is_in = p.at(T![in]);

    if is_in {
        p.bump_any();
        parse_expression(p, ExpressionContext::default())
            .or_add_diagnostic(p, js_parse_error::expected_expression);

        JS_FOR_IN_STATEMENT
    } else {
        p.expect(T![of]);

        parse_assignment_expression_or_higher(p, ExpressionContext::default())
            .or_add_diagnostic(p, js_parse_error::expected_expression_assignment);

        JS_FOR_OF_STATEMENT
    }
}

/// Either a traditional for statement or a for.. in statement
// test js for_stmt
// for (let i = 5; i < 10; i++) {}
// for (let { foo, bar } of {}) {}
// for (foo in {}) {}
// for (;;) {}
// for (let foo of []) {}
// for (let i = 5, j = 6; i < j; ++j) {}
// for await (let a of []) {}
fn parse_for_statement(p: &mut JsParser) -> ParsedSyntax {
    // test_err js for_stmt_err
    // for ;; {}
    // for let i = 5; i < 10; i++ {}
    // for let i = 5; i < 10; ++i {}
    // for (in []) {}
    // for (let i, j = 6 of []) {}
    // for await (let a in []) {}
    // for await (let i = 0; i < 10; ++i) {}
    // for (let [a];;) {}
    if !p.at(T![for]) {
        return Absent;
    }

    let m = p.start();
    p.expect(T![for]);

    let mut await_range = None;
    if p.at(T![await]) {
        await_range = Some(p.cur_range());
        p.expect(T![await]);
    }

    let has_l_paren = p.expect(T!['(']);
    let kind = parse_for_head(p, has_l_paren, await_range.is_some());
    p.expect(T![')']);

    p.with_state(EnterBreakable(BreakableKind::Iteration), |p| {
        parse_statement(p, StatementContext::For)
    })
    .or_add_diagnostic(p, expected_statement);

    let mut completed = m.complete(p, kind);

    if kind != JS_FOR_OF_STATEMENT {
        if let Some(await_range) = await_range {
            p.error(
                p.err_builder(
                    "await can only be used in conjunction with `for...of` statements",
                    await_range,
                )
                .with_detail(await_range, "Remove the await here")
                .with_detail(
                    completed.range(p),
                    "or convert this to a `for...of` statement",
                ),
            );
            completed.change_kind(p, JS_BOGUS_STATEMENT)
        }
    }

    Present(completed)
}

struct SwitchCaseStatementList;

impl ParseNodeList for SwitchCaseStatementList {
    type Kind = JsSyntaxKind;
    type Parser<'source> = JsParser<'source>;

    const LIST_KIND: Self::Kind = JS_STATEMENT_LIST;

    fn parse_element(&mut self, p: &mut JsParser) -> ParsedSyntax {
        parse_statement(p, StatementContext::StatementList)
    }

    fn is_at_list_end(&self, p: &mut JsParser) -> bool {
        p.at_ts(token_set![T![default], T![case], T!['}']])
    }

    fn recover(
        &mut self,
        p: &mut JsParser,
        parsed_element: ParsedSyntax,
    ) -> parser::RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(JS_BOGUS_STATEMENT, STMT_RECOVERY_SET),
            js_parse_error::expected_case,
        )
    }
}

// We return the range in case its a default clause so we can report multiple default clauses in a better way
fn parse_switch_clause(p: &mut JsParser, first_default: &mut Option<TextRange>) -> ParsedSyntax {
    let m = p.start();
    match p.cur() {
        T![default] => {
            // in case we have two `default` expression, we mark the second one
            // as `JS_CASE_CLAUSE` where the "default" keyword is an bogus node
            let syntax_kind = if first_default.is_some() {
                let discriminant = p.start();
                p.bump_any(); // interpret `default` as the test of the case
                discriminant.complete(p, JS_BOGUS_EXPRESSION);
                JS_CASE_CLAUSE
            } else {
                p.expect(T![default]);
                JS_DEFAULT_CLAUSE
            };

            p.expect(T![:]);
            SwitchCaseStatementList.parse_list(p);
            let default = m.complete(p, syntax_kind);
            if let Some(first_default_range) = first_default {
                let err = p
                    .err_builder(
                        "Multiple default clauses inside of a switch statement are not allowed",
                        default.range(p),
                    )
                    .with_detail(default.range(p), "a second clause here is not allowed")
                    .with_detail(
                        *first_default_range,
                        "the first default clause is defined here",
                    );

                p.error(err);
            }

            Present(default)
        }
        T![case] => {
            p.expect(T![case]);
            parse_expression(p, ExpressionContext::default())
                .or_add_diagnostic(p, js_parse_error::expected_expression);
            p.expect(T![:]);

            SwitchCaseStatementList.parse_list(p);
            Present(m.complete(p, JS_CASE_CLAUSE))
        }
        _ => {
            m.abandon(p);
            Absent
        }
    }
}
#[derive(Default)]
struct SwitchCasesList {
    first_default: Option<TextRange>,
}

impl ParseNodeList for SwitchCasesList {
    type Kind = JsSyntaxKind;
    type Parser<'source> = JsParser<'source>;

    const LIST_KIND: Self::Kind = JS_SWITCH_CASE_LIST;

    fn parse_element(&mut self, p: &mut JsParser) -> ParsedSyntax {
        let clause = parse_switch_clause(p, &mut self.first_default);

        if let Present(marker) = &clause {
            if marker.kind(p) == JS_DEFAULT_CLAUSE && self.first_default.is_none() {
                self.first_default = Some(marker.range(p));
            }
        }

        clause
    }

    fn is_at_list_end(&self, p: &mut JsParser) -> bool {
        p.at(T!['}'])
    }

    fn recover(&mut self, p: &mut JsParser, parsed_element: ParsedSyntax) -> RecoveryResult {
        if let Present(marker) = parsed_element {
            Ok(marker)
        } else {
            let m = p.start();
            let statements = p.start();

            let recovered_element = parsed_element.or_recover_with_token_set(
                p,
                &ParseRecoveryTokenSet::new(
                    JS_BOGUS_STATEMENT,
                    token_set![T![default], T![case], T!['}']],
                )
                .enable_recovery_on_line_break(),
                js_parse_error::expected_case_or_default,
            );

            match recovered_element {
                Ok(marker) => {
                    statements.complete(p, JS_STATEMENT_LIST);
                    m.complete(p, JS_CASE_CLAUSE);
                    Ok(marker)
                }
                Err(err) => {
                    statements.abandon(p);
                    m.abandon(p);
                    Err(err)
                }
            }
        }
    }
}

/// A switch statement such as
///
/// ```js
/// switch (a) {
///     case foo:
///         bar();
/// }
/// ```
// test js switch_stmt
// switch (foo) {
//  case bar:
//  default:
// }
fn parse_switch_statement(p: &mut JsParser) -> ParsedSyntax {
    // test_err js switch_stmt_err
    // switch foo {}
    // switch {}
    // switch { var i = 0 }
    // switch { var i = 0; case "bar": {} }
    // switch (foo) {
    //   default: {}
    //   default: {}
    // }
    // switch (foo) { case : }

    if !p.at(T![switch]) {
        return Absent;
    }
    let m = p.start();
    p.expect(T![switch]);
    parenthesized_expression(p);
    p.expect(T!['{']);

    p.with_state(EnterBreakable(BreakableKind::Switch), |p| {
        SwitchCasesList::default().parse_list(p)
    });

    p.expect(T!['}']);
    Present(m.complete(p, JS_SWITCH_STATEMENT))
}

fn parse_catch_clause(p: &mut JsParser) -> ParsedSyntax {
    if !p.at(T![catch]) {
        return Absent;
    }

    let m = p.start();
    p.expect(T![catch]);

    parse_catch_declaration(p).ok();
    parse_block_stmt(p).or_add_diagnostic(p, js_parse_error::expected_block_statement);

    Present(m.complete(p, JS_CATCH_CLAUSE))
}

fn parse_catch_declaration(p: &mut JsParser) -> ParsedSyntax {
    if !p.at(T!['(']) {
        return Absent;
    }

    let declaration_marker = p.start();

    p.bump_any(); // bump (
    parse_binding_pattern(p, ExpressionContext::default()).or_add_diagnostic(p, expected_binding);

    // test ts ts_catch_declaration
    // try {} catch (error: any) {}
    // try {} catch (error: unknown) {}
    if p.at(T![:]) && is_nth_at_identifier(p, 1) {
        // test_err ts ts_catch_declaration_non_any_unknown_type_annotation
        // try {} catch (error: Error) {}
        JsSyntaxFeature::TypeScript
            .parse_exclusive_syntax(
                p,
                |p| {
                    let annotation = p.start();
                    p.bump(T![:]);

                    if let Some(ty) = parse_ts_type(p, TypeContext::default()).or_add_diagnostic(p, expected_ts_type) {
                        if !matches!(ty.kind(p), TS_ANY_TYPE | TS_UNKNOWN_TYPE) {
                            p.error(
                                p
                                    .err_builder("Catch clause variable type annotation must be 'any' or 'unknown' if specified.", ty.range(p))
                            );
                        }
                    }

                    Present(annotation.complete(p, TS_TYPE_ANNOTATION))
                },
                |p, annotation| {
                    ts_only_syntax_error(p, "type annotation", annotation.range(p))
                },
            )
            .ok();
    }

    p.expect(T![')']);

    Present(declaration_marker.complete(p, JS_CATCH_DECLARATION))
}

/// A try statement such as
///
/// ```js
/// try {
///     something();
/// } catch (a) {
///
/// }
/// ```
// test js try_stmt
// try {} catch {}
// try {} catch (e) {}
// try {} catch {} finally {}
// try {} catch (e) {} finally {}
// try {} finally {}
pub(crate) fn parse_try_statement(p: &mut JsParser) -> ParsedSyntax {
    // TODO: recover from `try catch` and `try finally`. The issue is block_items
    // will cause infinite recursion because parsing a stmt would not consume the catch token
    // and block_items would not exit, and if we exited on any error that would greatly limit
    // block_items error recovery

    if !p.at(T![try]) {
        return Absent;
    }

    let m = p.start();
    p.expect(T![try]);

    parse_block_stmt(p).or_add_diagnostic(p, js_parse_error::expected_block_statement);

    let catch = parse_catch_clause(p);

    if p.at(T![finally]) {
        catch.ok();

        let finalizer = p.start();
        p.expect(T![finally]);
        parse_block_stmt(p).or_add_diagnostic(p, js_parse_error::expected_block_statement);
        finalizer.complete(p, JS_FINALLY_CLAUSE);
        Present(m.complete(p, JS_TRY_FINALLY_STATEMENT))
    } else {
        catch.or_add_diagnostic(p, js_parse_error::expected_catch_clause);
        Present(m.complete(p, JS_TRY_STATEMENT))
    }
}
