use crate::parser::{ParsedSyntax, RecoveryResult};
use crate::prelude::*;
use crate::state::{
    EnableStrictMode, EnterClassPropertyInitializer, EnterClassStaticInitializationBlock,
    EnterParameters, SignatureFlags,
};
use crate::syntax::binding::parse_binding;
use crate::syntax::expr::{
    parse_assignment_expression_or_higher, parse_lhs_expr, parse_private_name, ExpressionContext,
};
use crate::syntax::function::{
    parse_any_parameter, parse_formal_parameter, parse_function_body, parse_parameter_list,
    parse_parameters_list, parse_ts_type_annotation_or_error, ParameterContext,
};
use crate::syntax::js_parse_error;
use crate::syntax::js_parse_error::{
    decorator_must_precede_modifier, decorators_not_allowed, expected_binding, expected_expression,
    invalid_decorator_error, modifier_already_seen, modifier_cannot_be_used_with_modifier,
    modifier_must_precede_modifier, parameter_decorators_not_allowed,
};
use crate::syntax::object::{
    is_at_literal_member_name, parse_computed_member_name, parse_literal_member_name,
};
use crate::syntax::stmt::{optional_semi, parse_statements, StatementContext};
use crate::syntax::typescript::ts_parse_error::{
    ts_accessibility_modifier_already_seen, ts_accessor_type_parameters_error,
    ts_constructor_type_parameters_error, ts_modifier_cannot_appear_on_a_constructor_declaration,
    ts_modifier_cannot_appear_on_a_parameter, ts_only_syntax_error,
    ts_set_accessor_return_type_error,
};
use crate::syntax::typescript::{
    is_reserved_type_name, parse_ts_implements_clause, parse_ts_return_type_annotation,
    parse_ts_type_annotation, parse_ts_type_arguments, parse_ts_type_parameters, TypeContext,
};

use crate::JsSyntaxFeature::TypeScript;
use crate::ParsedSyntax::{Absent, Present};
use crate::{JsParser, StrictMode};
use biome_js_syntax::JsSyntaxKind::*;
use biome_js_syntax::TextSize;
use biome_js_syntax::{JsSyntaxKind, T};
use biome_parser::parse_lists::ParseNodeList;
use biome_parser::parse_recovery::ParseRecoveryTokenSet;
use biome_parser::ParserProgress;
use biome_rowan::{SyntaxKind, TextRange};
use drop_bomb::DebugDropBomb;
use enumflags2::{bitflags, make_bitflags, BitFlags};
use smallvec::SmallVec;
use std::fmt::Debug;
use std::ops::{Add, BitOr, BitOrAssign};
use std::slice::Iter;

use super::function::LineBreak;
use super::js_parse_error::unexpected_body_inside_ambient_context;
use super::metavariable::{is_at_metavariable, parse_metavariable};
use super::typescript::ts_parse_error::{self, unexpected_abstract_member_with_body};
use super::typescript::{
    expect_ts_index_signature_member, is_at_ts_index_signature_member, MemberParent,
};

pub(crate) fn is_at_ts_abstract_class_declaration(
    p: &mut JsParser,
    should_check_line_break: LineBreak,
) -> bool {
    let is_abstract = p.at(T![abstract]) && p.nth_at(1, T![class]);
    if should_check_line_break == LineBreak::DoCheck {
        is_abstract && !p.has_nth_preceding_line_break(1)
    } else {
        is_abstract
    }
}

pub(crate) fn is_at_export_class_declaration(p: &mut JsParser) -> bool {
    p.at(T![export]) && (p.nth_at(1, T![class]) || p.nth_at(1, T![@]) || p.nth_at(1, T![abstract]))
}

pub(crate) fn is_at_export_default_class_declaration(p: &mut JsParser) -> bool {
    p.at(T![export])
        && p.nth_at(1, T![default])
        && (p.nth_at(2, T![class]) || p.nth_at(2, T![@]) || p.nth_at(2, T![abstract]))
}

/// Parses a class expression, e.g. let a = class {}
pub(super) fn parse_class_expression(
    p: &mut JsParser,
    decorator_list: ParsedSyntax,
) -> ParsedSyntax {
    if !p.at(T![class]) {
        return Absent;
    }

    Present(parse_class(p, ClassKind::Expression, decorator_list))
}

// test js class_declaration
// class foo {}
// class foo extends bar {}
// class foo extends foo.bar {}

// test_err js class_decl_err
// class {}
// class extends bar {}
// class foo { set {} }
// class extends {}

// test ts ts_abstract_classes
// abstract class A {}
// abstract class ConcreteMembers {
//     name: string;
//     constructor(name: string) { this.name = name; }
//     display(): void { console.log(this.name); }
//     public get my_name() { return this.name; }
//     public set my_name(name) { this.name = name; }
//     #private_method() { }
// }
// abstract class AbstractMembers {
//     abstract name: string;
//     abstract display();
//     abstract get my_name();
//     abstract set my_name(val);
//     abstract set my_age(age,);
// }

// test_err ts typescript_abstract_classes_incomplete
// abstract class {};

// test_err ts typescript_abstract_classes_invalid_abstract_constructor
// abstract class A { abstract constructor();};

// test ts ts_decorate_computed_member
// class Test {
// @test
// ['a']: string;
// }

// test ts ts_decorated_class_members
// class Test {
//   @test prop: string;
//   @test method() {}
//   @test get getter() {}
//   @test set setter(a) {}
// }

// test_err ts ts_invalid_decorated_class_members
// abstract class Test {
//   @test constructor() {}
//   @test declare prop;
//   @test method();
//   @test [index: string]: string;
//   @test abstract method2();
//   @test abstract get getter();
//   @test abstract set setter(val);
// }

/// Parses a class declaration if it is valid and otherwise returns [Invalid].
///
/// A class can be invalid if
/// * It uses an illegal identifier name
pub(super) fn parse_class_declaration(
    p: &mut JsParser,
    decorator_list: ParsedSyntax,
    context: StatementContext,
) -> ParsedSyntax {
    if !matches!(p.cur(), T![abstract] | T![class]) {
        return Absent;
    }

    let mut class = parse_class(p, ClassKind::Declaration, decorator_list);

    if !class.kind(p).is_bogus() && context.is_single_statement() {
        // test_err js class_in_single_statement_context
        // if (true) class A {}
        p.error(
            p.err_builder(
                "Classes can only be declared at top level or inside a block",
                class.range(p),
            )
            .with_hint("wrap the class in a block statement"),
        );
        class.change_to_bogus(p)
    }

    Present(class)
}

// test js export_default_class_clause
// export default class {}

// test ts typescript_export_default_abstract_class_case
// export default abstract class {}
pub(super) fn parse_class_export_default_declaration(
    p: &mut JsParser,
    decorator_list: ParsedSyntax,
) -> ParsedSyntax {
    if !matches!(p.cur(), T![abstract] | T![class]) {
        return Absent;
    }

    Present(parse_class(p, ClassKind::ExportDefault, decorator_list))
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum ClassKind {
    Declaration,
    Expression,
    ExportDefault,
}

impl ClassKind {
    fn is_id_optional(&self) -> bool {
        matches!(self, ClassKind::Expression | ClassKind::ExportDefault)
    }
}

impl From<ClassKind> for JsSyntaxKind {
    fn from(kind: ClassKind) -> Self {
        match kind {
            ClassKind::Declaration => JS_CLASS_DECLARATION,
            ClassKind::Expression => JS_CLASS_EXPRESSION,
            ClassKind::ExportDefault => JS_CLASS_EXPORT_DEFAULT_DECLARATION,
        }
    }
}

// test js class_named_abstract_is_valid_in_js
// class abstract {}

// test ts ts_class_named_abstract_is_valid_in_ts
// class abstract {}
#[inline]
fn parse_class(p: &mut JsParser, kind: ClassKind, decorator_list: ParsedSyntax) -> CompletedMarker {
    let decorator_list = decorator_list.or_else(|| empty_decorator_list(p));

    let m = decorator_list.precede(p);
    let is_abstract = p.eat(T![abstract]);

    let class_token_range = p.cur_range();
    p.expect(T![class]);

    let p = &mut *p.with_scoped_state(EnableStrictMode(StrictMode::Class(p.cur_range())));

    // test_err ts class_decl_no_id
    // class {}
    // class implements B {}
    let id = match p.cur() {
        T![implements] if TypeScript.is_supported(p) => Absent,
        T![extends] => Absent,
        _ => parse_binding(p),
    };

    // parse class id
    match id {
        Present(id) => {
            let text = p.text(id.range(p));
            if TypeScript.is_supported(p) && is_reserved_type_name(text) {
                // test_err ts ts_class_name_reserved_as_type
                // class undefined {}
                // class string {}
                // class any {}
                let err = p
                    .err_builder(format!(
                            "`{text}` cannot be used as a class name because it is already reserved as a type"
                        ),id.range(p), );

                p.error(err);
            }
        }
        Absent => {
            if !kind.is_id_optional() {
                let err = p.err_builder(
                    "class declarations must have a name",
                    class_token_range.start()..p.cur_range().start(),
                );

                p.error(err);
            }
        }
    }

    // test ts ts_class_type_parameters
    // class BuildError<A, B, C> {}

    // test_err ts ts_class_type_parameters_errors
    // class BuildError<> {}
    TypeScript
        .parse_exclusive_syntax(
            p,
            |p| {
                parse_ts_type_parameters(
                    p,
                    TypeContext::default()
                        .and_allow_in_out_modifier(true)
                        .and_allow_const_modifier(true),
                )
            },
            |p, type_parameters| {
                ts_only_syntax_error(p, "class type parameters", type_parameters.range(p))
            },
        )
        .ok();

    eat_class_heritage_clause(p);

    p.expect(T!['{']);
    ClassMembersList {
        inside_abstract_class: is_abstract,
    }
    .parse_list(p);
    p.expect(T!['}']);

    m.complete(p, kind.into())
}

// test_err js class_extends_err
// class A extends bar extends foo {}
// class B extends bar, foo {}
// class C implements B {}
//
// test_err ts ts_class_heritage_clause_errors
// class A {}
// interface Int {}
// class B implements Int extends A {}
// class C implements Int implements Int {}
// class D implements {}
// class E extends {}
// class F extends E, {}
/// Eats a class's 'implements' and 'extends' clauses, attaching them to the current active node.
/// Implements error recovery in case a class has multiple extends/implements clauses or if they appear
/// out of order
fn eat_class_heritage_clause(p: &mut JsParser) {
    let mut first_extends: Option<CompletedMarker> = None;
    let mut first_implements: Option<CompletedMarker> = None;

    loop {
        match p.cur() {
            T![extends] => {
                let current = parse_extends_clause(p).expect(
                    "Expected extends clause because parser is positioned at extends keyword",
                );

                match first_extends.as_ref() {
                    None => {
                        first_extends = {
                            if let Some(first_implements) = first_implements.as_ref() {
                                p.error(
                                    p.err_builder(
                                        "'extends' clause must precede 'implements' clause.",
                                        current.range(p),
                                    )
                                    .with_detail(
                                        first_implements.range(p),
                                        "This is where implements was found",
                                    ),
                                )
                            }

                            Some(current)
                        }
                    }
                    Some(first_extends) => p.error(
                        p.err_builder("'extends' clause already seen.", current.range(p))
                            .with_detail(first_extends.range(p), "first 'extends' clause"),
                    ),
                }
            }
            T![implements] => {
                let mut current = parse_ts_implements_clause(p).expect("expected 'implements' clause because parser is positioned at 'implements' keyword.");

                match first_implements.as_ref() {
                    None => {
                        first_implements = {
                            if TypeScript.is_unsupported(p) {
                                p.error(p.err_builder(
                                    "classes can only implement interfaces in TypeScript files",
                                    current.range(p),
                                ));
                                current.change_to_bogus(p);
                            }
                            Some(current)
                        }
                    }
                    Some(first_implements) => {
                        p.error(
                            p.err_builder("'implements' clause already seen.", current.range(p))
                                .with_detail(
                                    first_implements.range(p),
                                    "first 'implements' clause",
                                ),
                        );
                    }
                }
            }
            _ => break,
        }
    }
}

// test ts ts_extends_generic_type
// type IHasVisualizationModel = string;
// class D extends C<IHasVisualizationModel> {
//     x = "string";
// }
fn parse_extends_clause(p: &mut JsParser) -> ParsedSyntax {
    if !p.at(T![extends]) {
        return Absent;
    }

    let m = p.start();
    let extends_end = p.cur_range().end();
    p.expect(T![extends]);

    if parse_extends_expression(p).is_absent() {
        p.error(p.err_builder("'extends' list cannot be empty.", extends_end..extends_end))
    } else {
        TypeScript
            .parse_exclusive_syntax(
                p,
                |p| parse_ts_type_arguments(p, TypeContext::default()),
                |p, arguments| ts_only_syntax_error(p, "type arguments", arguments.range(p)),
            )
            .ok();
    }

    while p.at(T![,]) {
        let comma_range = p.cur_range();
        p.bump(T![,]);

        let extra = p.start();
        if parse_extends_expression(p).is_absent() {
            p.error(p.err_builder("Trailing comma not allowed.", comma_range));
            extra.abandon(p);
            break;
        }

        parse_ts_type_arguments(p, TypeContext::default()).ok();

        let extra_class = extra.complete(p, JS_BOGUS);

        p.error(p.err_builder(
            "Classes can only extend a single class.",
            extra_class.range(p),
        ));
    }

    Present(m.complete(p, JS_EXTENDS_CLAUSE))
}

fn parse_extends_expression(p: &mut JsParser) -> ParsedSyntax {
    if p.at(T!['{']) && p.nth_at(1, T!['}']) {
        // Don't eat the body of the class as an object expression except if we have
        // * extends {} {
        // * extends {} implements
        // * extends {},
        if !matches!(p.nth(2), T![extends] | T![implements] | T!['{'] | T![,]) {
            return Absent;
        }
    }

    parse_lhs_expr(p, ExpressionContext::default())
}

struct ClassMembersList {
    inside_abstract_class: bool,
}

impl ParseNodeList for ClassMembersList {
    type Kind = JsSyntaxKind;
    type Parser<'source> = JsParser<'source>;

    const LIST_KIND: JsSyntaxKind = JS_CLASS_MEMBER_LIST;

    fn parse_element(&mut self, p: &mut JsParser) -> ParsedSyntax {
        parse_class_member(p, self.inside_abstract_class)
    }

    fn is_at_list_end(&self, p: &mut JsParser) -> bool {
        p.at(T!['}'])
    }

    fn recover(&mut self, p: &mut JsParser, parsed_element: ParsedSyntax) -> RecoveryResult {
        // test_err js invalid_method_recover
        // class {
        //   [1 + 1] = () => {
        //     let a=;
        //   };
        // };
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(
                JS_BOGUS_MEMBER,
                token_set![
                    T![;],
                    T![ident],
                    T![readonly],
                    T![private],
                    T![protected],
                    T![public],
                    T![override],
                    T![declare],
                    T![static],
                    T![accessor],
                    T![async],
                    T![yield],
                    T!['}'],
                    T![#],
                    T![@],
                ],
            ),
            js_parse_error::expected_class_member,
        )
    }
}

// test js class_declare
// class B { declare() {} }
// class B { declare = foo }

// test js static_method
// class foo {
//  static foo(bar) {}
//  static *foo() {}
//  static async foo() {}
//  static async *foo() {}
// }
fn parse_class_member(p: &mut JsParser, inside_abstract_class: bool) -> ParsedSyntax {
    if is_at_metavariable(p) {
        return parse_metavariable(p);
    }

    let member_marker = p.start();
    // test js class_empty_element
    // class foo { ;;;;;;;;;; get foo() {};;;;}
    if p.eat(T![;]) {
        return Present(member_marker.complete(p, JS_EMPTY_CLASS_MEMBER));
    }

    let mut modifiers = parse_class_member_modifiers(p, false);

    if is_at_static_initialization_block_class_member(p) {
        return Present(parse_static_initialization_block_class_member(
            p,
            member_marker,
            modifiers,
        ));
    }

    let member = parse_class_member_impl(p, member_marker, &mut modifiers);

    match member {
        Present(mut member) => {
            let mut valid = true;
            if !inside_abstract_class {
                // test_err ts ts_concrete_class_with_abstract_members
                // class A {
                //    abstract my_age: number;
                //    abstract name(): string;
                //    abstract get age(): number;
                //    abstract set age(v);
                // }
                if let Some(abstract_token_range) =
                    modifiers.get_first_range(ModifierKind::Abstract)
                {
                    let err = p.err_builder(
                        "Only abstract classes can have abstract members",
                        abstract_token_range,
                    );
                    p.error(err);
                    valid = false;
                }
            }

            let modifiers_valid = modifiers.validate_and_complete(p, member.kind(p));

            if !valid || !modifiers_valid {
                member.change_to_bogus(p);
            }

            Present(member)
        }
        Absent => {
            // If the modifier list contains a modifier other than a decorator, such modifiers can also be valid member names.
            debug_assert!(!modifiers
                .flags
                .contains(ModifierFlags::ALL_MODIFIERS_EXCEPT_DECORATOR));

            // test_err ts ts_broken_class_member_modifiers
            // class C {
            // 	@decorator
            // 	}
            // class CC {
            // 	@
            // 	}
            // class @
            // class C@
            modifiers.abandon(p);
            Absent
        }
    }
}

// test ts ts_index_signature_class_member
// class A {
//     [a: number]: string;
// }
// class B {
//     [index: string]: { prop }
// }

// test ts ts_index_signature_class_member_can_be_static
// class A {
//     static [a: number]: string;
// }
// class B {
//     static readonly [a: number]: string;
// }

fn parse_index_signature_class_member(p: &mut JsParser, member_marker: Marker) -> ParsedSyntax {
    TypeScript.parse_exclusive_syntax(
        p,
        |p| {
            Present(expect_ts_index_signature_member(
                p,
                member_marker,
                MemberParent::Class,
                TypeContext::default(),
            ))
        },
        |p, member| ts_only_syntax_error(p, "Index signatures", member.range(p)),
    )
}

fn parse_class_member_impl(
    p: &mut JsParser,
    member_marker: Marker,
    modifiers: &mut ClassMemberModifiers,
) -> ParsedSyntax {
    let start_token_pos = p.source().position();
    let generator_range = p.cur_range();

    // Seems like we're at a generator method
    if p.at(T![*]) {
        p.bump_any(); // bump * token
        if is_at_constructor(p, modifiers) {
            let err = p.err_builder("constructors can't be generators", generator_range);

            p.error(err);
        }

        return Present(parse_method_class_member(
            p,
            member_marker,
            modifiers,
            SignatureFlags::GENERATOR,
        ));
    };

    // Seems like we're at an async method
    if p.at(T![async])
        && !p.nth_at(1, T![?])
        && !p.nth_at(1, T![;])
        && !p.nth_at(1, T![=])
        && !is_at_method_class_member(p, 1)
        && !p.has_nth_preceding_line_break(1)
    {
        let async_range = p.cur_range();
        p.expect(T![async]);

        let mut flags = SignatureFlags::ASYNC;

        if p.eat(T![*]) {
            flags |= SignatureFlags::GENERATOR;
        }

        return Present(if is_at_constructor(p, modifiers) {
            let err = p.err_builder("constructors cannot be async", async_range);

            p.error(err);
            parse_class_member_name(p, modifiers).unwrap();
            parse_constructor_class_member_body(p, member_marker, modifiers)
        } else {
            parse_method_class_member(p, member_marker, modifiers, flags)
        });
    }

    // Seems like we're at an index member
    if is_at_ts_index_signature_member(p) {
        return parse_index_signature_class_member(p, member_marker);
    }

    // test js getter_class_member
    // class Getters {
    //   get foo() {}
    //   get static() {}
    //   static get bar() {}
    //   get "baz"() {}
    //   get ["a" + "b"]() {}
    //   get 5() {}
    //   get #private() {}
    // }
    // class NotGetters {
    //   get() {}
    //   async get() {}
    //   static get() {}
    // }
    //
    // test_err js method_getter_err
    // class foo {
    //  get {}
    // }
    //

    // test js setter_class_member
    // class Setters {
    //   set foo(a) {}
    //   set bax(a,) {}
    //   set static(a) {}
    //   static set bar(a) {}
    //   static set baz(a,) {}
    //   set "baz"(a) {}
    //   set ["a" + "b"](a) {}
    //   set 5(a) {}
    //   set 6(a,) {}
    //   set #private(a) {}
    // }
    // class NotSetters {
    //   set(a) {}
    //   async set(a) {}
    //   static set(a) {}
    // }
    //
    // test_err js setter_class_member
    // class Setters {
    //   set foo() {}
    // }
    if matches!(p.cur(), T![get] | T![set]) && is_at_class_member_name(p, 1) {
        let is_getter = p.at(T![get]);
        if is_getter {
            p.expect(T![get]);
        } else {
            p.expect(T![set]);
        }

        // So we've seen a get that now must be followed by a getter/setter name
        parse_class_member_name(p, modifiers)
            .or_add_diagnostic(p, js_parse_error::expected_class_member_name);

        // test_err ts ts_getter_setter_type_parameters
        // class Test {
        //  get a<A>(): A {}
        //  set a<A>(value: A) {}
        // }

        // test_err ts ts_getter_setter_type_parameters_errors
        // class Test {
        //  get a<>(): A {}
        //  set a<>(value: A) {}
        // }
        if let Present(type_parameters) = parse_ts_type_parameters(p, TypeContext::default()) {
            p.error(ts_accessor_type_parameters_error(p, &type_parameters))
        }

        let completed = if is_getter {
            p.expect(T!['(']);
            p.expect(T![')']);
            parse_ts_type_annotation_or_error(p).ok();

            let member_kind = expect_accessor_body(p, &member_marker, modifiers);
            member_marker.complete(p, member_kind.as_getter_syntax_kind())
        } else {
            let has_l_paren = p.expect(T!['(']);
            p.with_state(EnterParameters(SignatureFlags::empty()), |p| {
                let decorator_list = parse_parameter_decorators(p);

                // test ts ts_decorator_on_class_setter { "parse_class_parameter_decorators": true }
                // class A {
                //     set val(@dec x) {}
                //     set val(@dec.fn() x) {}
                //     set val(@dec() x) {}
                // }

                // test_err ts ts_decorator_on_class_setter
                // class A {
                //     set val(@dec x) {}
                //     set val(@dec.fn() x) {}
                //     set val(@dec() x) {}
                // }
                parse_formal_parameter(
                    p,
                    decorator_list,
                    ParameterContext::ClassSetter,
                    ExpressionContext::default().and_object_expression_allowed(has_l_paren),
                    TypeContext::default(),
                )
            })
            .or_add_diagnostic(p, js_parse_error::expected_parameter);

            if p.at(T![,]) {
                p.bump_any();
            }

            p.expect(T![')']);

            // test_err ts ts_setter_return_type_annotation
            // class Test {
            //     set a(value: string): void {}
            // }
            if let Present(return_type_annotation) =
                parse_ts_return_type_annotation(p, TypeContext::default())
            {
                p.error(ts_set_accessor_return_type_error(
                    p,
                    &return_type_annotation,
                ));
            }

            let member_kind = expect_accessor_body(p, &member_marker, modifiers);
            member_marker.complete(p, member_kind.as_setter_syntax_kind())
        };

        return Present(completed);
    }

    let is_constructor = is_at_constructor(p, modifiers);
    let member_name = parse_class_member_name(p, modifiers)
        .or_add_diagnostic(p, js_parse_error::expected_class_member_name);

    if is_at_method_class_member(p, 0) {
        // test js class_static_constructor_method
        // class B { static constructor() {} }

        // test js constructor_class_member
        // class Foo {
        //   constructor(a) {
        //     this.a = a;
        //   }
        // }
        // class Bar {
        //   "constructor"(b) {
        //     this.b = b;
        //   }
        // }
        return if is_constructor {
            Present(parse_constructor_class_member_body(
                p,
                member_marker,
                modifiers,
            ))
        } else {
            // test js method_class_member
            // class Test {
            //   method() {}
            //   async asyncMethod() {}
            //   async* asyncGeneratorMethod() {}
            //   * generatorMethod() {}
            //   "foo"() {}
            //   ["foo" + "bar"]() {}
            //   5() {}
            //   #private() {}
            // }
            // class ContextualKeywords {
            //    // Methods called static
            //   static() {}
            //   async static() {}
            //   * static() {}
            //   async* static() {}
            //   declare() {}
            //   get() {} // Method called get
            //   set() {} // Method called set
            // }
            // class Static {
            //   static method() {}
            //   static async asyncMethod() {}
            //   static async* asyncGeneratorMethod() {}
            //   static * generatorMethod() {}
            //   static static() {}
            //   static async static() {}
            //   static async* static() {}
            //   static * static() {}
            // }
            Present(parse_method_class_member_rest(
                p,
                member_marker,
                modifiers,
                SignatureFlags::empty(),
            ))
        };
    }

    match member_name {
        Some(_) => {
            // test js property_class_member
            // class foo {
            //   property
            //   declare;
            //   initializedProperty = "a"
            //   "a";
            //   5
            //   ["a" + "b"]
            //   static staticProperty
            //   static staticInitializedProperty = 1
            //   #private
            //   #privateInitialized = "a"
            //   static #staticPrivate
            //   static #staticPrivateInitializedProperty = 1
            // }
            //
            // test_err js class_declare_member
            // class B { declare foo }

            // test ts ts_property_class_member_can_be_named_set_or_get
            // class B { set: String; get: Number }
            let mut property = parse_property_class_member_body(p, member_marker, modifiers);

            if !property.kind(p).is_bogus() && is_constructor {
                let err = p.err_builder(
                    "class properties may not be called `constructor`",
                    property.range(p),
                );

                p.error(err);
                property.change_to_bogus(p);
            }

            Present(property)
        }
        None => {
            // test_err js block_stmt_in_class
            // class S{{}}
            debug_assert_eq!(
                p.source().position(),
                start_token_pos,
                "Parser shouldn't be progressing when returning Absent"
            );

            member_marker.abandon(p);
            Absent
        }
    }
}

fn is_at_static_initialization_block_class_member(p: &mut JsParser) -> bool {
    p.at(T![static]) && p.nth_at(1, T!['{'])
}

// test js static_initialization_block_member
// class A {
//   static a;
//   static {
//     this.a = "test";
//   }
// }
//
fn parse_static_initialization_block_class_member(
    p: &mut JsParser,
    member_marker: Marker,
    modifiers: ClassMemberModifiers,
) -> CompletedMarker {
    if modifiers.is_empty() {
        modifiers.abandon(p);
    } else {
        // test_err ts ts_class_initializer_with_modifiers
        // class A {
        //   public static { }
        // }
        p.error(p.err_builder(
            "Static class blocks cannot have any modifier.",
            modifiers.list_marker.range(p),
        ));
        modifiers.validate_and_complete(p, JS_STATIC_INITIALIZATION_BLOCK_CLASS_MEMBER);
    }

    p.expect(T![static]);
    p.expect(T!['{']);
    p.with_state(EnterClassStaticInitializationBlock, |p| {
        let statement_list = p.start();
        parse_statements(p, true, statement_list)
    });
    p.expect(T!['}']);

    member_marker.complete(p, JS_STATIC_INITIALIZATION_BLOCK_CLASS_MEMBER)
}

/// Parses the body of a property class member (anything after the member name). If the current member is abstract, the [ParsedSyntax]
/// will have kind TS_PROPERTY_SIGNATURE_CLASS_MEMBER, otehrwise will be JS_PROPERTY_CLASS_MEMBER.
///
///  # Arguments
///
/// * `p` - Parser being used
/// * `member_marker` - Marker that will be completed at the end of this function
/// * `modifiers` - All the member modifiers parsed previously. This will be used for validation and for the [ParsedSyntax::kind]
fn parse_property_class_member_body(
    p: &mut JsParser,
    member_marker: Marker,
    modifiers: &ClassMemberModifiers,
) -> CompletedMarker {
    let annotation = parse_ts_property_annotation(p, modifiers).ok();

    // test js class_await_property_initializer
    // // SCRIPT
    // async function* test() {
    //   class A {
    //     prop = await;
    //   }
    // }

    // test_err js class_yield_property_initializer
    // // SCRIPT
    // async function* test() {
    //   class A {
    //     prop = yield;
    //   }
    // }

    let initializer_syntax = p.with_state(EnterClassPropertyInitializer, |p| {
        parse_initializer_clause(p, ExpressionContext::default())
    });

    expect_member_semi(p, &member_marker, "class property");

    let is_signature = modifiers.is_signature() || p.state().in_ambient_context();
    let kind = if !is_signature {
        JS_PROPERTY_CLASS_MEMBER
    } else if initializer_syntax.is_present() {
        TS_INITIALIZED_PROPERTY_SIGNATURE_CLASS_MEMBER
    } else {
        TS_PROPERTY_SIGNATURE_CLASS_MEMBER
    };

    let member = member_marker.complete(p, kind);

    if let Present(initializer) = &initializer_syntax {
        if modifiers.has(ModifierKind::Abstract) {
            // test_err ts ts_abstract_property_cannot_have_initiliazers
            // abstract class A {
            //     abstract name: string = "";
            // }
            p.error(p.err_builder(
                "Property cannot have an initializer because it is marked abstract.",
                initializer.range(p),
            ));
        } else if modifiers.has(ModifierKind::Declare) || p.state().in_ambient_context() {
            // test ts ts_readonly_property_initializer_ambient_context
            // declare class A { readonly prop = "test"; }
            // class B { declare readonly prop = "test"; }
            // declare class A { private readonly prop = "test"; }
            // class B { declare private readonly prop = "test"; }
            // declare class A { static readonly prop = "test"; }
            // class B { declare static readonly prop = "test"; }

            if !modifiers.has(ModifierKind::Readonly) {
                // test_err ts ts_property_initializer_ambient_context
                // declare class A { prop = "test"; }
                // class B { declare prop = "test"; }

                p.error(p.err_builder(
                    "In ambient contexts, properties with initializers need to be readonly.",
                    initializer.range(p),
                ));
            } else if let Some(annotation) = annotation {
                // test_err ts ts_annotated_property_initializer_ambient_context
                // declare class T { readonly b: string = "test"; }
                // class T { declare readonly b: string = "test"; }

                p.error(p.err_builder(
                    "In ambient contexts, properties cannot have both a type annotation and an initializer.",
                    initializer.range(p),
                ).with_detail(annotation.range(p), "The type annotation is here:"));
            }
        }
    }

    member
}

fn expect_member_semi(p: &mut JsParser, member_marker: &Marker, name: &str) {
    if !optional_semi(p) {
        // Gets the start of the member
        let end = p.last_end().unwrap_or_else(|| p.cur_range().start());

        let err = p.err_builder(
            format!("expected a semicolon to end the {name}, but found none"),
            member_marker.start()..end,
        );

        p.error(err);
    }
}

// test_err js js_class_property_with_ts_annotation
// class A {
//  a: string;
//  b?: string;
//  c!: string
// }
//
// test ts ts_class_property_annotation
// class A {
//   a: string;
//   b?: string = "test";
//   c!: string;
// }
fn parse_ts_property_annotation(
    p: &mut JsParser,
    modifiers: &ClassMemberModifiers,
) -> ParsedSyntax {
    if !p.at(T![?]) && !p.at(T![!]) {
        return parse_ts_type_annotation_or_error(p);
    }

    let m = p.start();
    let mut valid = true;

    // test ts ts_abstract_property_can_be_optional
    // abstract class A {
    //      abstract name?: string;
    // }
    let optional_range = match optional_member_token(p) {
        Ok(optional_range) => optional_range,
        Err(optional_range) => {
            valid = false;
            Some(optional_range)
        }
    };

    let definite_range = if p.at(T![!]) {
        let range = p.cur_range();
        p.bump(T![!]);

        if TypeScript.is_unsupported(p) {
            let error = p.err_builder("`!` modifiers can only be used in TypeScript files", range);

            p.error(error);
            valid = false;
        }
        // test_err ts ts_abstract_property_cannot_be_definite
        // abstract class A {
        //      abstract name!: string;
        // }
        else if modifiers.has(ModifierKind::Abstract) {
            p.error(p.err_builder(
                "A definite assignment operator '!' cannot appear on an 'abstract' property.",
                range,
            ));
            valid = false;
        } else if modifiers.has(ModifierKind::Declare) || p.state().in_ambient_context() {
            // test_err ts ts_definite_assignment_in_ambient_context
            // declare class A { prop!: string }
            // class B { declare prop!: string }
            p.error(p.err_builder(
                "Definite assignment operators '!' aren't allowed in ambient contexts.",
                range,
            ));
        }

        Some(range)
    } else {
        None
    };

    let mut annotation = match (optional_range, definite_range) {
        (Some(_), None) => {
            parse_ts_type_annotation(p, TypeContext::default()).ok();
            m.complete(p, TS_OPTIONAL_PROPERTY_ANNOTATION)
        }
        (None, Some(_)) => {
            parse_ts_type_annotation(p, TypeContext::default()).or_add_diagnostic(p, |p, range| {
                p.err_builder("Properties with definite assignment assertions must also have type annotations.",range, )
            });
            m.complete(p, TS_DEFINITE_PROPERTY_ANNOTATION)
        }
        (Some(optional_range), Some(definite_range)) => {
            parse_ts_type_annotation(p, TypeContext::default()).ok();
            let error = p
                .err_builder(
                    "class properties cannot be both optional and definite",
                    definite_range,
                )
                .with_detail(definite_range, "The definite")
                .with_detail(optional_range, "The optional");

            p.error(error);

            m.complete(p, JS_BOGUS)
        }
        // handled by the test at the beginning of the function that returns if the parser isn't at a
        // ! or ? token.
        (None, None) => unreachable!(),
    };

    if !valid {
        annotation.change_to_bogus(p);
    }

    Present(annotation)
}

/// Eats the '?' token for optional member. Emits an error if this isn't typescript
fn optional_member_token(p: &mut JsParser) -> Result<Option<TextRange>, TextRange> {
    if p.at(T![?]) {
        let range = p.cur_range();
        p.bump(T![?]);

        // test_err js optional_member
        // class B { foo?; }
        if TypeScript.is_supported(p) {
            Ok(Some(range))
        } else {
            let err = p.err_builder("`?` modifiers can only be used in TypeScript files", range);

            p.error(err);
            Err(range)
        }
    } else {
        Ok(None)
    }
}

// test_err js class_property_initializer
// class B { lorem = ; }
pub(crate) fn parse_initializer_clause(
    p: &mut JsParser,
    context: ExpressionContext,
) -> ParsedSyntax {
    if p.at(T![=]) {
        let m = p.start();
        p.bump(T![=]);

        parse_assignment_expression_or_higher(p, context)
            .or_add_diagnostic(p, js_parse_error::expected_expression_assignment);

        Present(m.complete(p, JS_INITIALIZER_CLAUSE))
    } else {
        Absent
    }
}

fn parse_method_class_member(
    p: &mut JsParser,
    m: Marker,
    modifiers: &mut ClassMemberModifiers,
    flags: SignatureFlags,
) -> CompletedMarker {
    parse_class_member_name(p, modifiers)
        .or_add_diagnostic(p, js_parse_error::expected_class_member_name);
    parse_method_class_member_rest(p, m, modifiers, flags)
}

// test_err js class_member_method_parameters
// class B { foo(a {} }

// test ts ts_method_class_member
// class Test {
//   test<A, B extends A, R>(a: A, b: B): R {}
// }

/// Parses the body (everything after the identifier name) of a method class member
/// that includes: parameters and its types, return type and method body
fn parse_method_class_member_rest(
    p: &mut JsParser,
    m: Marker,
    modifiers: &ClassMemberModifiers,
    flags: SignatureFlags,
) -> CompletedMarker {
    // test ts ts_optional_method_class_member
    // class A { test?() {} }
    let optional = optional_member_token(p);

    TypeScript
        .parse_exclusive_syntax(
            p,
            |p| parse_ts_type_parameters(p, TypeContext::default().and_allow_const_modifier(true)),
            |p, marker| ts_only_syntax_error(p, "type parameters", marker.range(p)),
        )
        .ok();

    let parameter_context = if modifiers.is_signature() {
        ParameterContext::Declaration
    } else {
        // Not perfect. It may turn out that this is a method overload without a body in which case
        // this isn't an implementation.
        ParameterContext::ClassImplementation
    };

    parse_parameter_list(p, parameter_context, TypeContext::default(), flags)
        .or_add_diagnostic(p, js_parse_error::expected_class_parameters);

    TypeScript
        .parse_exclusive_syntax(
            p,
            |p| parse_ts_return_type_annotation(p, TypeContext::default()),
            |p, annotation| ts_only_syntax_error(p, "return type annotation", annotation.range(p)),
        )
        .ok();

    let member_kind = expect_method_body(p, &m, modifiers, ClassMethodMemberKind::Method(flags));
    let mut member = m.complete(p, member_kind.as_method_syntax_kind());

    let is_async = flags.contains(SignatureFlags::ASYNC);

    // test_err ts typescript_abstract_classes_invalid_abstract_async_member
    // abstract class B { abstract async a(); }
    if modifiers.has(ModifierKind::Abstract) && is_async {
        let err = ts_parse_error::abstract_member_cannot_be_async(
            p,
            &modifiers.get_first_range_unchecked(ModifierKind::Abstract),
        );
        p.error(err);
        member.change_to_bogus(p);
    } else if flags.contains(SignatureFlags::GENERATOR) && member_kind.is_signature() {
        // test_err ts ts_method_signature_generator
        // declare class A { * method(); }
        // abstract class B { abstract * method(); }
        // class C {
        //      * overload();
        //      * overload() {}
        // }
        p.error(p.err_builder(
            "A method signature cannot be declared as a generator.",
            member.range(p),
        ));
    } else if p.state().in_ambient_context() && is_async {
        // test_err ts ts_ambient_async_method
        // declare class A { async method(); }
        p.error(p.err_builder(
            "'async' modifier cannot be used in an ambient context.",
            member.range(p),
        ));
        member.change_to_bogus(p);
    } else if optional.is_err() {
        // error already emitted by `optional_member_token()`
        member.change_to_bogus(p);
    }

    member
}

#[derive(Debug)]
enum MemberKind {
    Signature,
    Declaration,
}

impl MemberKind {
    const fn is_signature(&self) -> bool {
        matches!(self, MemberKind::Signature)
    }

    const fn as_method_syntax_kind(&self) -> JsSyntaxKind {
        match self {
            MemberKind::Signature => TS_METHOD_SIGNATURE_CLASS_MEMBER,
            MemberKind::Declaration => JS_METHOD_CLASS_MEMBER,
        }
    }

    const fn as_constructor_syntax_kind(&self) -> JsSyntaxKind {
        match self {
            MemberKind::Signature => TS_CONSTRUCTOR_SIGNATURE_CLASS_MEMBER,
            MemberKind::Declaration => JS_CONSTRUCTOR_CLASS_MEMBER,
        }
    }

    const fn as_setter_syntax_kind(&self) -> JsSyntaxKind {
        match self {
            MemberKind::Signature => TS_SETTER_SIGNATURE_CLASS_MEMBER,
            MemberKind::Declaration => JS_SETTER_CLASS_MEMBER,
        }
    }

    const fn as_getter_syntax_kind(&self) -> JsSyntaxKind {
        match self {
            MemberKind::Signature => TS_GETTER_SIGNATURE_CLASS_MEMBER,
            MemberKind::Declaration => JS_GETTER_CLASS_MEMBER,
        }
    }
}

#[derive(Debug)]
enum ClassMethodMemberKind {
    /// `get` or `set`
    Accessor,

    /// A class's constructor function
    Constructor,

    /// Any other regular method
    Method(SignatureFlags),
}

impl ClassMethodMemberKind {
    /// The body of methods is optional if it's a method overload definition
    /// ```ts
    /// class Test {
    ///   method();
    ///   method() { ... }
    /// }
    /// ```
    const fn is_body_optional(&self) -> bool {
        matches!(
            self,
            ClassMethodMemberKind::Method(_) | ClassMethodMemberKind::Constructor
        )
    }

    const fn is_constructor(&self) -> bool {
        matches!(self, ClassMethodMemberKind::Constructor)
    }

    const fn signature_flags(&self) -> SignatureFlags {
        match self {
            ClassMethodMemberKind::Method(flags) => *flags,
            ClassMethodMemberKind::Constructor => SignatureFlags::CONSTRUCTOR,
            ClassMethodMemberKind::Accessor => SignatureFlags::empty(),
        }
    }
}

/// Parses the body of a method (constructor, getter, setter, or regular method).
///
/// TypeScript supports method signatures. These are methods without a body (and are terminated by a semicolon or ASI).
/// A method is a signature if one of the following applies
/// * The member has an `abstract` modifier (not supported by constructors)
/// * It's a declaration in an ambient context (`declare class { ... }` or `declare namespace { class { ... } }`).
/// * It's a method overload (doesn't apply to getters/setters)
///
/// The method determines which case applies to the current member and emits a diagnostic if:
/// * the body is absent for a method declaration
/// * the body is present for a method signature
/// * a method signature isn't terminate by a semicolon or ASI
///
/// The method returns the inferred kind (signature or declaration) of the parsed method body
fn expect_method_body(
    p: &mut JsParser,
    member_marker: &Marker,
    modifiers: &ClassMemberModifiers,
    method_kind: ClassMethodMemberKind,
) -> MemberKind {
    let body = parse_function_body(p, method_kind.signature_flags());

    // test ts typescript_members_can_have_no_body_in_ambient_context
    // declare class Test {
    //     constructor();
    //     name();
    //     get test(): string;
    //     set test(v);
    // }
    // declare namespace n {
    //      class Test {
    //          constructor()
    //          name()
    //          get test(): string
    //          set test(v)
    //      }
    // }

    // test_err ts typescript_members_with_body_in_ambient_context_should_err
    // declare class Test {
    //     constructor() {}
    //     name() {}
    //     get test(): string { return ""; }
    //     set test(v) {}
    // }
    // declare namespace n {
    //      class Test {
    //          constructor() {}
    //          name() {}
    //          get test(): string { return ""; }
    //          set test(v) {}
    //      }
    // }
    if p.state().in_ambient_context() {
        match body {
            Present(body) => p.error(unexpected_body_inside_ambient_context(p, body.range(p))),
            Absent => {
                // test_err ts ts_ambient_context_semi
                // declare class A { method() method2() method3() }
                expect_member_semi(p, member_marker, "method declaration")
            }
        }
        MemberKind::Signature
    }
    // test_err ts typescript_abstract_class_member_should_not_have_body
    // abstract class AbstractMembers {
    //     abstract constructor() { }
    //     abstract display(): void { }
    //     abstract get my_name() { }
    //     abstract set my_name(name) { }
    //     abstract #private_name() { }
    // }
    else if modifiers.has(ModifierKind::Abstract) && !method_kind.is_constructor() {
        match body {
            Present(body) => p.error(unexpected_abstract_member_with_body(p, body.range(p))),
            Absent => {
                // test_err ts ts_abstract_member_ansi
                // abstract class A { abstract constructor() abstract method() abstract get getter() abstract set setter(value) abstract prop }
                expect_member_semi(p, member_marker, "method declaration")
            }
        }
        MemberKind::Signature
    }
    // test ts ts_method_and_constructor_overload
    // class Test {
    //      constructor();
    //      constructor(a: String) // ASI
    //      constructor(a?: String) {}
    //      async method(): Promise<String>;
    //      method(a: String): Promise<String> // ASI
    //      async method(a?: String): Promise<String> { return "test" }
    // }
    else if method_kind.is_body_optional()
        && TypeScript.is_supported(p)
        && body.is_absent()
        && optional_semi(p)
    {
        MemberKind::Signature
    } else {
        // test_err ts ts_method_members_with_missing_body
        // class Test {
        //      constructor() method() get test()
        //      set test(value)
        // }
        body.or_add_diagnostic(p, js_parse_error::expected_class_method_body);
        MemberKind::Declaration
    }
}

// test_err js getter_class_no_body
// class Getters {
//   get foo()
// }

// test_err js setter_class_no_body
// class Setters {
//   set foo(a)
// }
fn expect_accessor_body(
    p: &mut JsParser,
    member_marker: &Marker,
    modifiers: &ClassMemberModifiers,
) -> MemberKind {
    expect_method_body(p, member_marker, modifiers, ClassMethodMemberKind::Accessor)
}

fn parse_constructor_class_member_body(
    p: &mut JsParser,
    member_marker: Marker,
    modifiers: &ClassMemberModifiers,
) -> CompletedMarker {
    if let Ok(Some(range)) = optional_member_token(p) {
        let err = p.err_builder("constructors cannot be optional", range);

        p.error(err);
    }

    // test_err ts ts_constructor_type_parameters
    // class A { constructor<A>(b) {} }
    // class A { constructor<>(b) {} }
    if let Present(type_parameters) = parse_ts_type_parameters(p, TypeContext::default()) {
        p.error(ts_constructor_type_parameters_error(p, &type_parameters));
    }

    parse_constructor_parameter_list(p)
        .or_add_diagnostic(p, js_parse_error::expected_constructor_parameters);

    if let Present(marker) = parse_ts_type_annotation(p, TypeContext::default()) {
        let err = p.err_builder("constructors cannot have type annotations", marker.range(p));

        p.error(err);
    }

    let constructor_kind = expect_method_body(
        p,
        &member_marker,
        modifiers,
        ClassMethodMemberKind::Constructor,
    );

    member_marker.complete(p, constructor_kind.as_constructor_syntax_kind())
}

fn parse_constructor_parameter_list(p: &mut JsParser) -> ParsedSyntax {
    let m = p.start();

    // test js super_expression_in_constructor_parameter_list
    // class A extends B { constructor(c = super()) {} }
    //
    // test_err js super_expression_in_constructor_parameter_list
    // class A extends B { constructor(super()) {} }
    let flags = SignatureFlags::CONSTRUCTOR;

    parse_parameters_list(
        p,
        flags,
        parse_constructor_parameter,
        JS_CONSTRUCTOR_PARAMETER_LIST,
    );
    Present(m.complete(p, JS_CONSTRUCTOR_PARAMETERS))
}

// test_err js js_constructor_parameter_reserved_names
// // SCRIPT
// class A { constructor(readonly, private, protected, public) {} }
fn parse_constructor_parameter(p: &mut JsParser, context: ExpressionContext) -> ParsedSyntax {
    // test_err js class_constructor_parameter
    // class B { constructor(protected b) {} }

    // test ts ts_decorator_constructor { "parse_class_parameter_decorators": true }
    // class C {
    //     constructor(@foo readonly x: number) {}
    // }
    // class CC {
    //     constructor(@foo @dec(arg) readonly x: number) {}
    // }
    // class CC {
    //     constructor(@foo @dec.method(arg) readonly x: number) {}
    // }
    // class CCC {
    //     constructor(@foo @dec.method(arg) private readonly x: number) {}
    // }

    // test_err ts ts_decorator_constructor
    // class C {
    //     constructor(@foo readonly x: number) {}
    // }
    // class CC {
    //     constructor(@foo @dec(arg) readonly x: number) {}
    // }
    // class CC {
    //     constructor(@foo @dec.method(arg) readonly x: number) {}
    // }
    // class CCC {
    //     constructor(@foo @dec.method(arg) private readonly x: number) {}
    // }
    let decorator_list = parse_parameter_decorators(p);

    if is_nth_at_modifier(p, 0, true) {
        // test ts ts_property_parameter
        // class A { constructor(private x, protected y, public z) {} }
        // class B { constructor(readonly w, private readonly x, protected readonly y, public readonly z) {} }
        // class C { constructor(private x: string, readonly y?, z = "default", ...rest) {} }
        //
        // test_err ts ts_property_parameter_pattern
        // class A { constructor(private { x, y }, protected [a, b]) {} }
        let property_parameter = decorator_list
            .or_else(|| empty_decorator_list(p))
            .precede(p);

        // test_err js class_constructor_parameter_readonly
        // class B { constructor(readonly b) {} }

        let modifiers = parse_class_member_modifiers(p, true);

        // we pass decorator list as Absent because TsPropertyParameter has its own decorator list
        parse_formal_parameter(
            p,
            Absent,
            ParameterContext::ParameterProperty,
            context,
            TypeContext::default(),
        )
        .or_add_diagnostic(p, expected_binding);

        let kind = if modifiers.validate_and_complete(p, TS_PROPERTY_PARAMETER) {
            TS_PROPERTY_PARAMETER
        } else {
            JS_BOGUS_PARAMETER
        };

        Present(property_parameter.complete(p, kind))
    } else {
        parse_any_parameter(
            p,
            decorator_list,
            ParameterContext::ClassImplementation,
            context,
            TypeContext::default(),
        )
        .map(|mut parameter| {
            // test_err ts ts_constructor_this_parameter
            // class C { constructor(this) {} }
            if parameter.kind(p) == TS_THIS_PARAMETER {
                p.error(p.err_builder(
                    "A constructor cannot have a 'this' parameter.",
                    parameter.range(p),
                ));
                parameter.change_to_bogus(p);
            }
            parameter
        })
    }
}

fn is_at_class_member_name(p: &mut JsParser, offset: usize) -> bool {
    matches!(p.nth(offset), T![#] | T!['[']) || is_at_literal_member_name(p, offset)
}

/// Parses a `AnyJsClassMemberName` and returns its completion marker
fn parse_class_member_name(p: &mut JsParser, modifiers: &mut ClassMemberModifiers) -> ParsedSyntax {
    modifiers.set_private_member_name(p.at(T![#]));
    match p.cur() {
        T![#] => parse_private_class_member_name(p),
        T!['['] => parse_computed_member_name(p),
        _ => parse_literal_member_name(p),
    }
}

pub(crate) fn parse_private_class_member_name(p: &mut JsParser) -> ParsedSyntax {
    parse_private_name(p).map(|mut name| {
        name.change_kind(p, JS_PRIVATE_CLASS_MEMBER_NAME);
        name
    })
}

fn is_at_method_class_member(p: &mut JsParser, mut offset: usize) -> bool {
    if p.nth_at(offset, T![?]) {
        offset += 1;
    }

    p.nth_at(offset, T!['(']) || p.nth_at(offset, T![<])
}

pub(crate) fn is_nth_at_modifier(p: &mut JsParser, n: usize, constructor_parameter: bool) -> bool {
    // Test if this modifier is followed by another modifier, member name or any other token that
    // starts a new member. If that's the case, then this is fairly likely a modifier. If not, then
    // this is probably not a modifier, but the name of the member. For example, all these are valid
    // members: `static() {}, private() {}, protected() {}`... but are modifiers if followed by another modifier or a name:
    // `static x() {} private static() {}`...
    if !matches!(
        p.nth(n),
        T![declare]
            | T![public]
            | T![protected]
            | T![private]
            | T![override]
            | T![static]
            | T![accessor]
            | T![readonly]
            | T![abstract]
    ) {
        return false;
    }

    // The new line is allowed between the `static` and the member name.
    if !matches!(p.nth(n), T![static]) && p.has_nth_preceding_line_break(n + 1) {
        return false;
    }

    let followed_by_any_member = is_at_class_member_name(p, n + 1);
    let followed_by_class_member = !constructor_parameter && p.nth_at(n + 1, T![*]);
    let followed_by_parameter = constructor_parameter && matches!(p.nth(n + 1), T!['{'] | T!['[']);

    followed_by_any_member || followed_by_class_member || followed_by_parameter
}

// test js static_generator_constructor_method
// class A {
// 	static async * constructor() {}
// 	static * constructor() {}
// }
fn is_at_constructor(p: &JsParser, modifiers: &ClassMemberModifiers) -> bool {
    !modifiers.has(ModifierKind::Static)
        && (p.at(T![constructor]) || matches!(p.cur_text(), "\"constructor\"" | "'constructor'"))
}

// test js class_member_modifiers
// class A { public() {} }
// class A { static protected() {} }
// class A { static }

// test js class_member_modifiers_no_asi
// class A {
//   static
//   foo() {}
// }
//

/// Parses all possible modifiers regardless of what the current member is. It's up to the caller
/// to create diagnostics for not allowed modifiers.
fn parse_class_member_modifiers(
    p: &mut JsParser,
    constructor_parameter: bool,
) -> ClassMemberModifiers {
    let mut modifiers = ClassMemberModifierList::default();
    let list = p.start();
    let mut progress = ParserProgress::default();
    let mut flags = ModifierFlags::empty();

    while let Some(modifier) = parse_modifier(p, constructor_parameter) {
        progress.assert_progressing(p);
        flags |= modifier.kind.as_flags();
        modifiers.add_modifier(modifier);
    }

    // It's unclear what kind of list this is supposed to be at this moment.
    // Create an `JS_BOGUS` node. The list type gets changed later on by calling
    // `complete` or `abandon` when the member kind is known,
    let list = list.complete(p, JS_BOGUS);
    ClassMemberModifiers::new(modifiers, list, flags)
}

// test_err js class_declare_method
// class B { declare fn() {} }
//
// test_err js class_member_modifier
// class A { abstract foo; }
fn parse_modifier(p: &mut JsParser, constructor_parameter: bool) -> Option<ClassMemberModifier> {
    // decorator modifiers can't be valid member names.
    let is_at_decorator_modifier = p.cur() == T![@] || p.nth_at(1, T![@]);

    if !is_nth_at_modifier(p, 0, constructor_parameter) && !is_at_decorator_modifier {
        // all modifiers can also be valid member names. That's why we shouldn't parse a modifier
        // if it isn't followed by a valid member name or another modifier
        return None;
    }

    let modifier_kind = match p.cur() {
        T![declare] => ModifierKind::Declare,
        T![public] => ModifierKind::Public,
        T![protected] => ModifierKind::Protected,
        T![private] => ModifierKind::Private,
        T![override] => ModifierKind::Override,
        T![static] => ModifierKind::Static,
        T![accessor] => ModifierKind::Accessor,
        T![readonly] => ModifierKind::Readonly,
        T![abstract] => ModifierKind::Abstract,
        T![@] => ModifierKind::Decorator,
        _ => {
            return None;
        }
    };

    match modifier_kind {
        ModifierKind::Decorator => {
            // test ts decorator_class_member
            // class Foo {
            // // properties
            // @dec foo = 2;
            // @dec @(await dec) @dec() foo = 2;
            // @dec public foo = 1;
            // @dec @(await dec) @dec() public foo = 1;
            // @dec static foo = 2;
            // @dec @(await dec) @dec() static foo = 2;
            // @dec accessor foo = 2;
            // @dec @(await dec) @dec() accessor foo = 2;
            // @dec readonly foo = 2;
            // @dec @(await dec) @dec() readonly foo = 2;
            // @dec override foo = 2;
            // @dec @(await dec) @dec() override foo = 2;
            // // methods
            // @dec foo() {}
            // @dec @(await dec) @dec() foo() {}
            // @dec public foo() {}
            // @dec @(await dec) @dec() public foo() {}
            // @dec static foo() {}
            // @dec @(await dec) @dec() static foo() {}
            // @dec override foo() {}
            // @dec @(await dec) @dec() override foo() {}
            // // getters
            // @dec get foo() {}
            // @dec @(await dec) @dec() get foo() {}
            // @dec public get foo() {}
            // @dec @(await dec) @dec() public get foo() {}
            // @dec static get foo() {}
            // @dec @(await dec) @dec() static get foo() {}
            // @dec override get foo() {}
            // @dec @(await dec) @dec() override get foo() {}
            // // setters
            // @dec set foo(val) {}
            // @dec @(await dec) @dec() set foo(val) {}
            // @dec public set foo(val) {}
            // @dec @(await dec) @dec() public set foo(val) {}
            // @dec static set foo(val) {}
            // @dec @(await dec) @dec() static set foo(val) {}
            // @dec override set foo(val) {}
            // @dec @(await dec) @dec() override set foo(val) {}
            // }

            // SAFETY: Success is guaranteed since the parser is at the @ token.
            // The function takes care of handling any potential syntax errors.
            let decorator = parse_decorator(p).unwrap();

            let range = decorator.range(p);

            Some(ClassMemberModifier {
                start: range.start(),
                length: range.len(),
                kind: modifier_kind,
            })
        }
        _ => {
            let m = p.start();
            let range = p.cur_range();
            p.bump_any();
            m.complete(p, modifier_kind.as_syntax_kind());

            Some(ClassMemberModifier {
                start: range.start(),
                length: range.len(),
                kind: modifier_kind,
            })
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[bitflags]
#[repr(u16)]
enum ModifierFlag {
    Declare = 1 << 0,
    Private = 1 << 1,
    Protected = 1 << 2,
    Public = 1 << 3,
    Static = 1 << 4,
    Readonly = 1 << 5,
    Abstract = 1 << 6,
    Override = 1 << 7,
    PrivateName = 1 << 8,
    Accessor = 1 << 9,
    Decorator = 1 << 10,
}

/// Bitflag of class member modifiers.
/// Useful to cheaply track all already seen modifiers of a member (instead of using a HashSet<ModifierKind>).
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct ModifierFlags(BitFlags<ModifierFlag>);

impl ModifierFlags {
    const DECLARE: Self = Self(make_bitflags!(ModifierFlag::{Declare}));
    const PRIVATE: Self = Self(make_bitflags!(ModifierFlag::{Private}));
    const PROTECTED: Self = Self(make_bitflags!(ModifierFlag::{Protected}));
    const PUBLIC: Self = Self(make_bitflags!(ModifierFlag::{Public}));
    const STATIC: Self = Self(make_bitflags!(ModifierFlag::{Static}));
    const READONLY: Self = Self(make_bitflags!(ModifierFlag::{Readonly}));
    const ABSTRACT: Self = Self(make_bitflags!(ModifierFlag::{Abstract}));
    const OVERRIDE: Self = Self(make_bitflags!(ModifierFlag::{Override}));
    const PRIVATE_NAME: Self = Self(make_bitflags!(ModifierFlag::{PrivateName}));
    const ACCESSOR: Self = Self(make_bitflags!(ModifierFlag::{Accessor}));
    const DECORATOR: Self = Self(make_bitflags!(ModifierFlag::{Decorator}));

    const ACCESSIBILITY: Self = Self(make_bitflags!(ModifierFlag::{Private | Protected |  Public}));

    const ALL_MODIFIERS_EXCEPT_DECORATOR: Self = Self(
        make_bitflags!(ModifierFlag::{Declare | Private | Protected | Public | Static | Readonly | Abstract | Override | PrivateName | Accessor}),
    );

    pub const fn empty() -> Self {
        Self(BitFlags::EMPTY)
    }

    pub fn contains(&self, other: impl Into<ModifierFlags>) -> bool {
        self.0.contains(other.into().0)
    }

    pub fn intersects(&self, other: impl Into<ModifierFlags>) -> bool {
        self.0.intersects(other.into().0)
    }

    pub fn set(&mut self, other: impl Into<ModifierFlags>, cond: bool) {
        self.0.set(other.into().0, cond)
    }
}

impl BitOr for ModifierFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        ModifierFlags(self.0 | rhs.0)
    }
}

impl BitOrAssign for ModifierFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

/// The different modifiers a class member may have.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum ModifierKind {
    Declare,
    Abstract,
    Private,
    Protected,
    Public,
    Static,
    Accessor,
    Readonly,
    Override,
    Decorator,
}

impl ModifierKind {
    const fn is_ts_modifier(&self) -> bool {
        !matches!(
            self,
            ModifierKind::Static | ModifierKind::Accessor | ModifierKind::Decorator
        )
    }

    const fn as_syntax_kind(&self) -> JsSyntaxKind {
        match self {
            ModifierKind::Declare => TS_DECLARE_MODIFIER,
            ModifierKind::Abstract => TS_ABSTRACT_MODIFIER,
            ModifierKind::Private | ModifierKind::Protected | ModifierKind::Public => {
                TS_ACCESSIBILITY_MODIFIER
            }
            ModifierKind::Static => JS_STATIC_MODIFIER,
            ModifierKind::Accessor => JS_ACCESSOR_MODIFIER,
            ModifierKind::Readonly => TS_READONLY_MODIFIER,
            ModifierKind::Override => TS_OVERRIDE_MODIFIER,
            ModifierKind::Decorator => JS_DECORATOR,
        }
    }

    const fn as_flags(&self) -> ModifierFlags {
        match self {
            ModifierKind::Declare => ModifierFlags::DECLARE,
            ModifierKind::Abstract => ModifierFlags::ABSTRACT,
            ModifierKind::Private => ModifierFlags::PRIVATE,
            ModifierKind::Protected => ModifierFlags::PROTECTED,
            ModifierKind::Public => ModifierFlags::PUBLIC,
            ModifierKind::Static => ModifierFlags::STATIC,
            ModifierKind::Accessor => ModifierFlags::ACCESSOR,
            ModifierKind::Readonly => ModifierFlags::READONLY,
            ModifierKind::Override => ModifierFlags::OVERRIDE,
            ModifierKind::Decorator => ModifierFlags::DECORATOR,
        }
    }
}

/// Stores the range of a parsed modifier with its kind
#[derive(Debug, Clone)]
struct ClassMemberModifier {
    kind: ModifierKind,

    // The start position of the modifier in the source text
    start: TextSize,

    // The length of the modifier text.
    length: TextSize,
}

impl ClassMemberModifier {
    fn as_text_range(&self) -> TextRange {
        TextRange::new(self.start, self.start.add(self.length))
    }
}

// 4 is sufficient to store all valid modifiers without requiring any heap allocations.
#[derive(Debug, Default)]
struct ClassMemberModifierList(SmallVec<[ClassMemberModifier; 4]>);

impl ClassMemberModifierList {
    /// Sets the range of a parsed modifier
    fn add_modifier(&mut self, modifier: ClassMemberModifier) {
        self.0.push(modifier);
    }

    fn iter(&self) -> Iter<'_, ClassMemberModifier> {
        self.0.iter()
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// Stores the kind of parsed modifiers with their ranges.
#[derive(Debug)]
#[must_use]
struct ClassMemberModifiers {
    modifiers: ClassMemberModifierList,
    // Stores all added flags. Useful to determine with `O(1)` if a modifier exists in the list or not
    flags: ModifierFlags,
    list_marker: CompletedMarker,
    bomb: DebugDropBomb,
}

impl ClassMemberModifiers {
    fn new(
        modifiers: ClassMemberModifierList,
        list_marker: CompletedMarker,
        flags: ModifierFlags,
    ) -> Self {
        Self {
            modifiers,
            list_marker,
            flags,
            bomb: DebugDropBomb::new("list must either be 'completed' or 'abandoned' by calling 'complete' or 'abandon'.")
        }
    }

    fn set_private_member_name(&mut self, private_name: bool) {
        self.flags.set(ModifierFlags::PRIVATE_NAME, private_name);
    }

    /// Returns true if the passed in modifier is present in the source.
    fn has(&self, kind: ModifierKind) -> bool {
        self.flags.contains(kind.as_flags())
    }

    fn is_signature(&self) -> bool {
        self.has(ModifierKind::Abstract) || self.has(ModifierKind::Declare)
    }

    /// Returns the range for the passed in modifier or [None] if the modifier isn't set or is a missing marker
    fn get_first_range(&self, kind: ModifierKind) -> Option<TextRange> {
        if self.flags.contains(kind.as_flags()) {
            self.modifiers
                .iter()
                .find(|m| m.kind == kind)
                .map(|m| m.as_text_range())
        } else {
            None
        }
    }

    /// Returns the range of the passed in modifier.
    ///
    /// ## Safety
    /// Expected that the caller first checked that such a modifier is present (Either by using `has`
    /// or by iterating over all modifiers and keeping track of the modifier it has seen).
    fn get_first_range_unchecked(&self, kind: ModifierKind) -> TextRange {
        self.get_first_range(kind)
            .unwrap_or_else(|| panic!("Expected modifier of kind {kind:?} to be present"))
    }

    fn is_empty(&self) -> bool {
        self.modifiers.is_empty()
    }

    /// Abandons the marker for the modifier list
    fn abandon(mut self, p: &mut JsParser) {
        self.list_marker.undo_completion(p).abandon(p);
        self.bomb.defuse();
    }

    /// Validates if these modifiers are valid for a member of the given kind and
    /// completes the modifier list.
    ///
    /// Returns `true` if all modifiers are valid.
    fn validate_and_complete(mut self, p: &mut JsParser, member_kind: JsSyntaxKind) -> bool {
        self.bomb.defuse();

        let list_kind = match member_kind {
            JS_PROPERTY_CLASS_MEMBER => JS_PROPERTY_MODIFIER_LIST,
            TS_PROPERTY_SIGNATURE_CLASS_MEMBER | TS_INITIALIZED_PROPERTY_SIGNATURE_CLASS_MEMBER => {
                TS_PROPERTY_SIGNATURE_MODIFIER_LIST
            }
            JS_GETTER_CLASS_MEMBER | JS_SETTER_CLASS_MEMBER | JS_METHOD_CLASS_MEMBER => {
                JS_METHOD_MODIFIER_LIST
            }
            TS_GETTER_SIGNATURE_CLASS_MEMBER
            | TS_SETTER_SIGNATURE_CLASS_MEMBER
            | TS_METHOD_SIGNATURE_CLASS_MEMBER => TS_METHOD_SIGNATURE_MODIFIER_LIST,
            JS_CONSTRUCTOR_CLASS_MEMBER | TS_CONSTRUCTOR_SIGNATURE_CLASS_MEMBER => {
                JS_CONSTRUCTOR_MODIFIER_LIST
            }
            TS_INDEX_SIGNATURE_CLASS_MEMBER => TS_INDEX_SIGNATURE_MODIFIER_LIST,
            TS_PROPERTY_PARAMETER => TS_PROPERTY_PARAMETER_MODIFIER_LIST,
            JS_BOGUS_MEMBER | JS_STATIC_INITIALIZATION_BLOCK_CLASS_MEMBER => {
                // Error recovery kicked in. There's no "right" list to pick in this case, let's just remove it
                self.list_marker.undo_completion(p).abandon(p);
                return false;
            }
            t => panic!("Unknown member kind {t:?}"),
        };

        self.list_marker.change_kind(p, list_kind);

        let mut preceding_modifiers = ModifierFlags::empty();
        let mut valid = true;

        for modifier in self.modifiers.iter() {
            if let Some(diagnostic) =
                self.check_class_member_modifier(p, modifier, preceding_modifiers, member_kind)
            {
                p.error(diagnostic);
                valid = false;
            }

            preceding_modifiers |= modifier.kind.as_flags(); // Keep track of the seen modifiers
        }

        valid
    }

    // test js js_class_property_member_modifiers
    // class Test {
    //     static a = 1;
    //     accessor b = 1;
    //     static accessor c = 1;
    // }
    // class Foo {
    //     accessor
    //     foo
    // }

    // test ts ts_class_property_member_modifiers
    // class Base {
    //   base1;
    // }
    // abstract class Test extends Base {
    //     declare a: string;
    //     private declare c: string;
    //     declare private d: string;
    //     declare private readonly e: string;
    //     private readonly declare f: string;
    //     declare private static readonly g: string;
    //     private static readonly declare h: string;
    //     protected readonly abstract i: string;
    //     protected abstract readonly j: string;
    //     protected override readonly base1: string;
    //     private static accessor readonly k: string;
    //     protected abstract accessor readonly l: string;
    // }

    // test_err ts ts_class_modifier_precedence
    // class Base { base1; base2; base3; base4;}
    // abstract class Test extends Base {
    //     // Accessibility
    //     readonly private a: string;
    //     override protected base1;
    //     static private b: string;
    //     abstract protected d: string;
    //     // Static
    //     readonly static c: string;
    //     accessor static d: string;
    //     override static base2: string;
    //     // Accessor
    //     readonly accessor e: string;
    //     override accessor f: string;
    //     // abstract
    //     override abstract base3: string;
    //     // override
    //     readonly override base4: string;
    // }

    // test_err ts ts_class_invalid_modifier_combinations
    // class Base { base1; base2; }
    // abstract class Test extends Base {
    //     override override base1;
    //     declare declare a;
    //     private protected public c;
    //     private private d;
    //     protected protected e;
    //     public public f;
    //     abstract abstract g;
    //     static static h;
    //     readonly readonly i;
    //     override declare base2;
    //     private abstract j;
    //     abstract #j;
    //     abstract static k;
    //     abstract async l();
    //     declare async m();
    //     declare #l;
    //     declare accessor p;
    //     accessor accessor r;
    // }

    // test_err js class_invalid_modifiers
    // class A { public foo() {} }
    // class B { static static foo() {} }
    // class C { accessor foo() {} }
    fn check_class_member_modifier(
        &self,
        p: &JsParser,
        modifier: &ClassMemberModifier,
        preceding_modifiers: ModifierFlags,
        member_kind: JsSyntaxKind,
    ) -> Option<ParseDiagnostic> {
        // test_err js index_signature_class_member_in_js
        // class A {
        //     [a: number]: string;
        // }
        if TypeScript.is_unsupported(p) && modifier.kind.is_ts_modifier() {
            return Some(p.err_builder(
                format!(
                    "'{}' modifier can only be used in TypeScript files",
                    p.text(modifier.as_text_range())
                ),
                modifier.as_text_range(),
            ));
        }

        // test_err ts ts_index_signature_class_member_cannot_have_visibility_modifiers
        // class A {
        //     public  [a: number]: string;
        // }
        // class B {
        //     private  [a: number]: string;
        // }
        // class C {
        //     protected  [a: number]: string;
        // }

        if modifier.kind == ModifierKind::Decorator
            && !matches!(
                member_kind,
                JS_PROPERTY_CLASS_MEMBER
                    | JS_METHOD_CLASS_MEMBER
                    | JS_GETTER_CLASS_MEMBER
                    | JS_SETTER_CLASS_MEMBER
                    | TS_PROPERTY_SIGNATURE_CLASS_MEMBER
                    | TS_INITIALIZED_PROPERTY_SIGNATURE_CLASS_MEMBER
            )
        {
            // test ts decorator_class_member_in_ts
            // abstract class Qux {
            //   @dec declare static foo: string;
            // }
            // class Bar {
            //    @dec readonly foo = '123';
            // }

            // test_err ts decorator_class_member
            // class Foo {
            //    @dec constructor() {}
            //    @dec [index: string]: { props: string }
            // }
            // class Quiz {
            //    @dec public constructor() {}
            // }
            // class Bar extends Foo {
            //    @dec
            //    constructor();
            //    constructor(a: String)
            //    constructor(a?: String) {}
            // }
            // declare class Baz {
            //   @dec method();
            //   @dec get foo();
            //   @dec set foo(a);
            // }
            return Some(decorators_not_allowed(p, modifier.as_text_range()));
        } else if member_kind == TS_INDEX_SIGNATURE_CLASS_MEMBER
            && !matches!(modifier.kind, ModifierKind::Static | ModifierKind::Readonly)
        {
            // test_err ts ts_index_signature_class_member_cannot_be_abstract
            // abstract class A {
            //     abstract [a: number]: string;
            // }

            // test_err ts ts_index_signature_class_member_cannot_be_accessor
            // abstract class A {
            //     accessor [a: number]: string;
            // }
            return Some(p.err_builder(
                format!(
                    "'{}' modifier cannot appear on an index signature.",
                    p.text(modifier.as_text_range())
                ),
                modifier.as_text_range(),
            ));
        } else if matches!(
            member_kind,
            JS_CONSTRUCTOR_CLASS_MEMBER | TS_CONSTRUCTOR_SIGNATURE_CLASS_MEMBER
        ) && !matches!(
            modifier.kind,
            ModifierKind::Private | ModifierKind::Protected | ModifierKind::Public
        ) {
            return Some(ts_modifier_cannot_appear_on_a_constructor_declaration(
                p,
                modifier.as_text_range(),
            ));
        } else if
        // test ts class_constructor_parameter_modifiers
        // class Base { name!: string; other!: string }
        // class Sub extends Base {
        //  constructor(private priv: string, protected prot: string, public pub: string, override name: string, readonly read: string, protected override readonly other: string) {
        //      super();
        //  }
        // }
        member_kind == TS_PROPERTY_PARAMETER
            && !matches!(
                modifier.kind,
                ModifierKind::Private
                    | ModifierKind::Protected
                    | ModifierKind::Public
                    | ModifierKind::Override
                    | ModifierKind::Readonly
            )
        {
            return Some(ts_modifier_cannot_appear_on_a_parameter(
                p,
                modifier.as_text_range(),
            ));
        }

        match modifier.kind {
            ModifierKind::Decorator => {
                if preceding_modifiers.intersects(ModifierFlags::ALL_MODIFIERS_EXCEPT_DECORATOR) {
                    // test_err ts decorator_precede_class_member
                    // class Bar {
                    //   public @dec get foo() {}
                    //   static @dec foo: string;
                    //   readonly @dec test() {}
                    //   private @dec test() {}
                    //   protected @dec test() {}
                    // }
                    // class Qux extends Bar {
                    //   public @dec get foo() {}
                    //   static @dec foo: string;
                    //   readonly @dec test() {}
                    //   private @dec test() {}
                    //   accessor @dec test() {}
                    // }
                    return Some(decorator_must_precede_modifier(p, modifier.as_text_range()));
                }
            }
            ModifierKind::Readonly => {
                if preceding_modifiers.contains(ModifierFlags::READONLY) {
                    return Some(modifier_already_seen(
                        p,
                        modifier.as_text_range(),
                        self.get_first_range_unchecked(ModifierKind::Readonly),
                    ));
                } else if !matches!(
                    member_kind,
                    JS_PROPERTY_CLASS_MEMBER
                        | TS_PROPERTY_SIGNATURE_CLASS_MEMBER
                        | TS_INITIALIZED_PROPERTY_SIGNATURE_CLASS_MEMBER
                        | TS_INDEX_SIGNATURE_CLASS_MEMBER
                        | JS_BOGUS_MEMBER
                        | TS_PROPERTY_PARAMETER
                ) {
                    // test_err ts ts_readonly_modifier_non_class_or_indexer
                    // class Test {
                    //   readonly constructor() {}
                    //   readonly method() {}
                    //   readonly get test() { return "a"; }
                    //   readonly set test(value: string) {}
                    // }
                    return Some(p.err_builder(
                        "Readonly can only appear on a property declaration or index signature.",
                        modifier.as_text_range(),
                    ));
                }
            }
            ModifierKind::Declare => {
                // test_err ts ts_class_declare_modifier_error
                // class Test {
                //     declare method(): string;
                //     declare constructor(declare readonly prop) {}
                //     declare get test() { return "a" }
                //     declare set test(value: string) {}
                //     declare [name: string]: string;
                //     declare accessor foo: string;
                // }
                if preceding_modifiers.contains(ModifierFlags::DECLARE) {
                    return Some(modifier_already_seen(
                        p,
                        modifier.as_text_range(),
                        self.get_first_range_unchecked(ModifierKind::Declare),
                    ));
                } else if self.flags.contains(ModifierFlags::ACCESSOR) {
                    return Some(modifier_cannot_be_used_with_modifier(
                        p,
                        modifier.as_text_range(),
                        self.get_first_range_unchecked(ModifierKind::Accessor),
                    ));
                } else if self.flags.contains(ModifierFlags::OVERRIDE) {
                    return Some(modifier_cannot_be_used_with_modifier(
                        p,
                        modifier.as_text_range(),
                        self.get_first_range_unchecked(ModifierKind::Override),
                    ));
                } else if !matches!(
                    member_kind,
                    JS_PROPERTY_CLASS_MEMBER
                        | TS_PROPERTY_SIGNATURE_CLASS_MEMBER
                        | TS_INITIALIZED_PROPERTY_SIGNATURE_CLASS_MEMBER
                ) {
                    return Some(p.err_builder(
                        "'declare' modifier is only allowed on properties.",
                        modifier.as_text_range(),
                    ));
                } else if self.flags.contains(ModifierFlags::PRIVATE_NAME) {
                    // test_err ts ts_declare_property_private_name
                    // class A { declare #name(); };
                    return Some(p.err_builder(
                        "'declare' modifier cannot be used with a private identifier'.",
                        modifier.as_text_range(),
                    ));
                }
            }
            ModifierKind::Abstract => {
                if preceding_modifiers.contains(ModifierFlags::ABSTRACT) {
                    return Some(modifier_already_seen(
                        p,
                        modifier.as_text_range(),
                        self.get_first_range_unchecked(ModifierKind::Abstract),
                    ));
                } else if !matches!(
                    member_kind,
                    JS_METHOD_CLASS_MEMBER
                        | TS_METHOD_SIGNATURE_CLASS_MEMBER
                        | JS_PROPERTY_CLASS_MEMBER
                        | TS_PROPERTY_SIGNATURE_CLASS_MEMBER
                        | TS_INITIALIZED_PROPERTY_SIGNATURE_CLASS_MEMBER
                        | JS_SETTER_CLASS_MEMBER
                        | TS_SETTER_SIGNATURE_CLASS_MEMBER
                        | JS_GETTER_CLASS_MEMBER
                        | TS_GETTER_SIGNATURE_CLASS_MEMBER
                        | JS_BOGUS_MEMBER
                ) {
                    return Some(
                        p.err_builder("'abstract' modifier can only appear on a class, method or property declaration.",modifier.as_text_range(), )
                    );
                } else if self.flags.contains(ModifierFlags::STATIC) {
                    // test_err ts typescript_abstract_classes_invalid_static_abstract_member
                    // abstract class A { abstract static fn1(); }
                    // abstract class B { static abstract fn1(); }
                    return Some(modifier_cannot_be_used_with_modifier(
                        p,
                        modifier.as_text_range(),
                        self.get_first_range_unchecked(ModifierKind::Static),
                    ));
                } else if preceding_modifiers.contains(ModifierFlags::ACCESSOR) {
                    // test_err ts typescript_abstract_classes_abstract_accessor_precedence
                    // abstract class A { accessor abstract foo: number; }
                    return Some(modifier_must_precede_modifier(
                        p,
                        modifier.as_text_range(),
                        self.get_first_range_unchecked(ModifierKind::Accessor),
                    ));
                } else if preceding_modifiers.contains(ModifierFlags::OVERRIDE) {
                    return Some(modifier_must_precede_modifier(
                        p,
                        modifier.as_text_range(),
                        self.get_first_range_unchecked(ModifierKind::Override),
                    ));
                } else if self.flags.contains(ModifierFlags::PRIVATE_NAME) {
                    // test_err ts typescript_abstract_classes_invalid_abstract_private_member
                    // abstract class A { abstract #name(); };
                    return Some(p.err_builder(
                        "'abstract' modifier cannot be used with a private identifier'.",
                        modifier.as_text_range(),
                    ));
                }
            }
            ModifierKind::Private | ModifierKind::Protected | ModifierKind::Public => {
                if preceding_modifiers.intersects(ModifierFlags::ACCESSIBILITY) {
                    let range = if preceding_modifiers.contains(ModifierFlags::PRIVATE) {
                        self.get_first_range_unchecked(ModifierKind::Private)
                    } else if preceding_modifiers.contains(ModifierFlags::PROTECTED) {
                        self.get_first_range_unchecked(ModifierKind::Protected)
                    } else {
                        self.get_first_range_unchecked(ModifierKind::Public)
                    };

                    return Some(ts_accessibility_modifier_already_seen(
                        p,
                        modifier.as_text_range(),
                        range,
                    ));
                } else if preceding_modifiers.contains(ModifierFlags::OVERRIDE) {
                    return Some(modifier_must_precede_modifier(
                        p,
                        modifier.as_text_range(),
                        self.get_first_range_unchecked(ModifierKind::Override),
                    ));
                } else if preceding_modifiers.contains(ModifierFlags::STATIC) {
                    return Some(modifier_must_precede_modifier(
                        p,
                        modifier.as_text_range(),
                        self.get_first_range_unchecked(ModifierKind::Static),
                    ));
                } else if preceding_modifiers.contains(ModifierFlags::ACCESSOR) {
                    return Some(modifier_must_precede_modifier(
                        p,
                        modifier.as_text_range(),
                        self.get_first_range_unchecked(ModifierKind::Accessor),
                    ));
                } else if preceding_modifiers.contains(ModifierFlags::READONLY) {
                    return Some(modifier_must_precede_modifier(
                        p,
                        modifier.as_text_range(),
                        self.get_first_range_unchecked(ModifierKind::Readonly),
                    ));
                } else if self.flags.contains(ModifierFlags::ABSTRACT)
                    && modifier.kind == ModifierKind::Private
                {
                    return Some(modifier_cannot_be_used_with_modifier(
                        p,
                        modifier.as_text_range(),
                        self.get_first_range_unchecked(ModifierKind::Abstract),
                    ));
                } else if preceding_modifiers.contains(ModifierFlags::ABSTRACT) {
                    return Some(modifier_must_precede_modifier(
                        p,
                        modifier.as_text_range(),
                        self.get_first_range_unchecked(ModifierKind::Abstract),
                    ));
                } else if self.flags.contains(ModifierFlags::PRIVATE_NAME) {
                    // test_err ts typescript_classes_invalid_accessibility_modifier_private_member
                    // class A { private #name; protected #other; public #baz; };
                    return Some(p.err_builder(
                        "An accessibility modifier cannot be used with a private identifier.",
                        modifier.as_text_range(),
                    ));
                }
            }
            ModifierKind::Static => {
                if preceding_modifiers.contains(ModifierFlags::STATIC) {
                    return Some(modifier_already_seen(
                        p,
                        modifier.as_text_range(),
                        self.get_first_range_unchecked(ModifierKind::Static),
                    ));
                // test_err js class_member_static_accessor_precedence
                // class A {
                //     accessor static foo = 1;
                // }
                } else if preceding_modifiers.contains(ModifierFlags::ACCESSOR) {
                    return Some(modifier_must_precede_modifier(
                        p,
                        modifier.as_text_range(),
                        self.get_first_range_unchecked(ModifierKind::Accessor),
                    ));
                // test_err ts ts_index_signature_class_member_static_readonly_precedence
                // class A {
                //     readonly static [a: number]: string;
                // }
                } else if preceding_modifiers.contains(ModifierFlags::READONLY) {
                    return Some(modifier_must_precede_modifier(
                        p,
                        modifier.as_text_range(),
                        self.get_first_range_unchecked(ModifierKind::Readonly),
                    ));
                } else if preceding_modifiers.contains(ModifierFlags::OVERRIDE) {
                    return Some(modifier_must_precede_modifier(
                        p,
                        modifier.as_text_range(),
                        self.get_first_range_unchecked(ModifierKind::Override),
                    ));
                }
            }
            ModifierKind::Accessor => {
                if preceding_modifiers.contains(ModifierFlags::ACCESSOR) {
                    return Some(modifier_already_seen(
                        p,
                        modifier.as_text_range(),
                        self.get_first_range_unchecked(ModifierKind::Accessor),
                    ));
                }
                // test_err ts ts_class_member_accessor_readonly_precedence
                // class A {
                //     readonly accessor foo: number = 1;
                // }
                else if preceding_modifiers.contains(ModifierFlags::READONLY) {
                    return Some(modifier_must_precede_modifier(
                        p,
                        modifier.as_text_range(),
                        self.get_first_range_unchecked(ModifierKind::Readonly),
                    ));
                } else if preceding_modifiers.contains(ModifierFlags::OVERRIDE) {
                    return Some(modifier_cannot_be_used_with_modifier(
                        p,
                        modifier.as_text_range(),
                        self.get_first_range_unchecked(ModifierKind::Override),
                    ));
                } else if !matches!(
                    member_kind,
                    JS_PROPERTY_CLASS_MEMBER | TS_PROPERTY_SIGNATURE_CLASS_MEMBER
                ) {
                    return Some(p.err_builder(
                        "'accessor' modifier is only allowed on properties.",
                        modifier.as_text_range(),
                    ));
                }
            }
            ModifierKind::Override => {
                if preceding_modifiers.contains(ModifierFlags::OVERRIDE) {
                    return Some(modifier_already_seen(
                        p,
                        modifier.as_text_range(),
                        self.get_first_range_unchecked(ModifierKind::Override),
                    ));
                } else if preceding_modifiers.contains(ModifierFlags::READONLY) {
                    return Some(modifier_must_precede_modifier(
                        p,
                        modifier.as_text_range(),
                        self.get_first_range_unchecked(ModifierKind::Readonly),
                    ));
                }
            }
        }

        None
    }
}

// test ts decorator
// // class expressions
// let a = @decorator class {};
// let b = @decorator @functionDecorator(1,2,3) class {};
// let c = @first @second class Foo {}
// // class declarations
// @decorator class Foo {};
// @decorator @functionDecorator(1,2,3) class Bar {};
// @first @second class Baz {}
// // abstract class declarations
// @decorator abstract class Foo {};
// @decorator @functionDecorator(1,2,3) abstract class Bar {};
// @first @second abstract class Baz {}
// // exported class declarations
// export @decorator class Foo {};
// export @decorator @functionDecorator(1,2,3) class Bar {};
// export @first @second class Baz {}
// @decorator
// export class Foo { }
// @first.field @second @(() => decorator)()
// export class Bar {}
// @before
// export @after class Foo { }
// @before.field @before @(() => decorator)()
// export @after.field @after @(() => decorator)() class Bar {}

// test ts decorator_class_not_top_level
// if (a) {
//   @dec class MyClass {}
// }
// function foo() {
//   @dec class MyClass {}
// }

// test ts ts_decorator_call_expression_with_arrow
// export class Foo {
//  @Decorator((val) => val)
//  badField!: number
// }

// test_err ts decorator
// @'dsads' class MyClass {}
// @1 class MyClass {}
// @++1 class MyClass {}
// @[] in 1 class MyClass {}
// @[] class MyClass {}
// @() => {} class MyClass {}
// @1 == 2 ? true : false class MyClass {}
// @await fn class MyClass {}
// @function(){} class MyClass {}
// @obj instanceof Object class MyClass {}
// @1 === 2 class MyClass {}
// @new Object() class MyClass {}
// @{} class MyClass {}
// @a++ class MyClass {}
// @a,b class MyClass {}
// @`${d}foo` class MyClass {}
// @obj as MyType class MyClass {}
// @<MyType>obj class MyClass {}
// @obj satisfies MyType class MyClass {}
// @obj! class MyClass {}
pub(crate) fn parse_decorators(p: &mut JsParser) -> ParsedSyntax {
    if !p.at(T![@]) {
        return Absent;
    }

    let decorators = p.start();
    let mut progress = ParserProgress::default();

    while p.at(T![@]) {
        progress.assert_progressing(p);

        parse_decorator(p).ok();
    }

    Present(decorators.complete(p, JS_DECORATOR_LIST))
}

pub(crate) fn parse_parameter_decorators(p: &mut JsParser) -> ParsedSyntax {
    let decorator_list = parse_decorators(p);

    if p.options().should_parse_parameter_decorators() {
        decorator_list
    } else {
        decorator_list
            .add_diagnostic_if_present(p, parameter_decorators_not_allowed)
            .map(|mut decorator_list| {
                decorator_list.change_to_bogus(p);
                decorator_list
            })
            .into()
    }
}

pub(crate) fn empty_decorator_list(p: &mut JsParser) -> ParsedSyntax {
    let m = p.start();
    Present(m.complete(p, JS_DECORATOR_LIST))
}

fn parse_decorator(p: &mut JsParser) -> ParsedSyntax {
    if !p.at(T![@]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![@]);
    if let Some(mut complete_marker) =
        parse_lhs_expr(p, ExpressionContext::default().and_in_decorator(true))
            .or_add_diagnostic(p, expected_expression)
    {
        if !matches!(
            complete_marker.kind(p),
            JS_PARENTHESIZED_EXPRESSION
                | JS_CALL_EXPRESSION
                | JS_STATIC_MEMBER_EXPRESSION
                | JS_IDENTIFIER_EXPRESSION
        ) {
            p.error(invalid_decorator_error(p, complete_marker.range(p)));
            complete_marker.change_to_bogus(p);
        }
    }

    Present(m.complete(p, JS_DECORATOR))
}
