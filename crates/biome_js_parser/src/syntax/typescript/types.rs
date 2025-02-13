use std::ops::{BitOr, Sub};

use crate::parser::{RecoveryError, RecoveryResult};
use crate::prelude::*;
use crate::state::{EnterType, SignatureFlags};
use crate::syntax::expr::{
    is_at_binary_operator, is_at_expression, is_at_identifier, is_nth_at_identifier,
    is_nth_at_identifier_or_keyword, parse_big_int_literal_expression, parse_identifier,
    parse_literal_expression, parse_name, parse_number_literal_expression,
    parse_reference_identifier, parse_template_elements, ExpressionContext,
};
use crate::syntax::function::{
    parse_formal_parameter, parse_parameter_list, skip_parameter_start, ParameterContext,
};
use crate::syntax::js_parse_error::{
    decorators_not_allowed, expected_identifier, expected_object_member_name, expected_parameter,
    expected_parameters, expected_property_or_signature, modifier_already_seen,
    modifier_must_precede_modifier,
};
use crate::syntax::metavariable::parse_metavariable;
use crate::syntax::module::ImportAssertionList;
use crate::syntax::object::{
    is_at_object_member_name, is_nth_at_type_member_name, parse_object_member_name,
};
use crate::syntax::stmt::optional_semi;
use crate::syntax::typescript::try_parse;
use crate::syntax::typescript::ts_parse_error::{
    expected_ts_type, expected_ts_type_parameter, infer_not_allowed,
    ts_const_modifier_cannot_appear_on_a_type_parameter,
    ts_in_out_modifier_cannot_appear_on_a_type_parameter,
};
use biome_parser::parse_lists::{ParseNodeList, ParseSeparatedList};
use enumflags2::{bitflags, make_bitflags, BitFlags};
use smallvec::SmallVec;

use crate::lexer::{JsLexContext, JsReLexContext};
use crate::span::Span;
use crate::syntax::class::parse_decorators;
use crate::JsSyntaxFeature::TypeScript;
use crate::{Absent, JsParser, ParseRecoveryTokenSet, ParsedSyntax, Present};
use biome_js_syntax::JsSyntaxKind::TS_TYPE_ANNOTATION;
use biome_js_syntax::T;
use biome_js_syntax::{JsSyntaxKind::*, *};

use super::ts_parse_error::expected_ts_import_type_with_arguments;
use super::{expect_ts_index_signature_member, is_at_ts_index_signature_member, MemberParent};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[bitflags]
#[repr(u8)]
enum ContextFlag {
    DisallowConditionalTypes = 1 << 0,
    AllowInOutModifier = 1 << 1,
    AllowConstModifier = 1 << 2,
    InConditionalExtends = 1 << 3,
    TypeOrInterfaceDeclaration = 1 << 4,
}

/// Context tracking state that applies to the parsing of all types
#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct TypeContext(BitFlags<ContextFlag>);

impl TypeContext {
    /// Whether conditional types `extends string ? string : number` are allowed in the current context.
    ///
    /// By default, conditional types are allowed.
    pub const DISALLOW_CONDITIONAL_TYPES: Self =
        Self(make_bitflags!(ContextFlag::{DisallowConditionalTypes}));

    /// Whether 'in' and 'out' modifiers are allowed in the current context.
    ///
    /// By default, 'in' and 'out' modifiers are not allowed.
    pub const ALLOW_IN_OUT_MODIFIER: Self = Self(make_bitflags!(ContextFlag::{AllowInOutModifier}));

    /// Whether 'const' modifier is allowed in the current context.
    ///
    /// By default, 'const' modifier is not allowed.
    pub const ALLOW_CONST_MODIFIER: Self = Self(make_bitflags!(ContextFlag::{AllowConstModifier}));

    /// Whether the parser is inside a conditional extends
    pub const IN_CONDITIONAL_EXTENDS: Self =
        Self(make_bitflags!(ContextFlag::{InConditionalExtends}));

    /// Whether the current context is within a type or interface declaration
    pub const TYPE_OR_INTERFACE_DECLARATION: Self =
        Self(make_bitflags!(ContextFlag::{TypeOrInterfaceDeclaration}));

    pub fn contains(&self, other: impl Into<TypeContext>) -> bool {
        self.0.contains(other.into().0)
    }

    pub(crate) fn and_allow_conditional_types(self, allow: bool) -> Self {
        self.and(TypeContext::DISALLOW_CONDITIONAL_TYPES, !allow)
    }

    pub(crate) fn and_allow_in_out_modifier(self, allow: bool) -> Self {
        self.and(TypeContext::ALLOW_IN_OUT_MODIFIER, allow)
    }

    pub(crate) fn and_allow_const_modifier(self, allow: bool) -> Self {
        self.and(TypeContext::ALLOW_CONST_MODIFIER, allow)
    }

    pub(crate) fn and_in_conditional_extends(self, allow: bool) -> Self {
        self.and(TypeContext::IN_CONDITIONAL_EXTENDS, allow)
    }

    pub(crate) fn and_type_or_interface_declaration(self, allow: bool) -> Self {
        self.and(TypeContext::TYPE_OR_INTERFACE_DECLARATION, allow)
    }

    pub(crate) fn is_conditional_type_allowed(&self) -> bool {
        !self.contains(TypeContext::DISALLOW_CONDITIONAL_TYPES)
    }

    pub(crate) fn is_in_out_modifier_allowed(&self) -> bool {
        self.contains(TypeContext::ALLOW_IN_OUT_MODIFIER)
    }

    pub(crate) fn is_const_modifier_allowed(&self) -> bool {
        self.contains(TypeContext::ALLOW_CONST_MODIFIER)
    }

    pub(crate) fn in_conditional_extends(&self) -> bool {
        self.contains(TypeContext::IN_CONDITIONAL_EXTENDS)
    }

    pub(crate) fn is_in_type_or_interface_declaration(&self) -> bool {
        self.contains(TypeContext::TYPE_OR_INTERFACE_DECLARATION)
    }

    /// Adds the `flag` if `set` is `true`, otherwise removes the `flag`
    fn and(self, flag: TypeContext, set: bool) -> Self {
        if set {
            self | flag
        } else {
            self - flag
        }
    }
}

impl BitOr for TypeContext {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl Sub for TypeContext {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 & !rhs.0)
    }
}

pub(crate) fn is_reserved_type_name(name: &str) -> bool {
    matches!(
        name,
        "string"
            | "null"
            | "number"
            | "object"
            | "any"
            | "unknown"
            | "boolean"
            | "bigint"
            | "symbol"
            | "void"
            | "never"
            | "undefined"
    )
}

pub(crate) fn is_reserved_module_name(name: &str) -> bool {
    name.len() == 4 && matches!(name, "void" | "null")
}

pub(crate) fn parse_ts_type_annotation(p: &mut JsParser, context: TypeContext) -> ParsedSyntax {
    if !p.at(T![:]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![:]);
    parse_ts_type(p, context).or_add_diagnostic(p, expected_ts_type);
    Present(m.complete(p, TS_TYPE_ANNOTATION))
}

// test ts ts_return_type_annotation
// type A = (a) => a is string;
// type B = { test(a): a is string }
// type C = { (a): a is string }
// const a = { test(x): x is string { return typeof x === "string" } }
// class D { test(x): x is string { return typeof x === "string"; } }
pub(crate) fn parse_ts_return_type_annotation(
    p: &mut JsParser,
    context: TypeContext,
) -> ParsedSyntax {
    if !p.at(T![:]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![:]);
    parse_ts_return_type(p, context).or_add_diagnostic(p, expected_ts_type);
    Present(m.complete(p, TS_RETURN_TYPE_ANNOTATION))
}

fn parse_ts_call_signature(p: &mut JsParser, context: TypeContext) {
    parse_ts_type_parameters(p, context).ok();
    parse_parameter_list(
        p,
        ParameterContext::Declaration,
        context,
        SignatureFlags::empty(),
    )
    .or_add_diagnostic(p, expected_parameters);
    parse_ts_return_type_annotation(p, context).ok();
}

fn parse_ts_type_parameter_name(p: &mut JsParser) -> ParsedSyntax {
    parse_identifier(p, TS_TYPE_PARAMETER_NAME)
}

// test ts ts_type_parameters
// type A<X extends string, Y = number, Z extends string | number = number> = { x: X, y: Y, z: Z }
// type A<> = {}
// interface A<> {}

// test_err ts ts_type_parameters_incomplete
// type A<T
pub(crate) fn parse_ts_type_parameters(p: &mut JsParser, context: TypeContext) -> ParsedSyntax {
    if !is_nth_at_ts_type_parameters(p, 0) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![<]);

    if p.at(T![>]) && !context.is_in_type_or_interface_declaration() {
        p.error(expected_ts_type_parameter(p, p.cur_range()));
    }

    TsTypeParameterList(context).parse_list(p);
    p.expect(T![>]);

    Present(m.complete(p, TS_TYPE_PARAMETERS))
}

struct TsTypeParameterList(TypeContext);

impl ParseSeparatedList for TsTypeParameterList {
    type Kind = JsSyntaxKind;
    type Parser<'source> = JsParser<'source>;

    const LIST_KIND: Self::Kind = TS_TYPE_PARAMETER_LIST;

    fn parse_element(&mut self, p: &mut JsParser) -> ParsedSyntax {
        parse_ts_type_parameter(p, self.0)
    }

    fn is_at_list_end(&self, p: &mut JsParser) -> bool {
        p.at(T![>])
    }

    fn recover(&mut self, p: &mut JsParser, parsed_element: ParsedSyntax) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(
                TS_BOGUS_TYPE,
                token_set![T![>], T![,], T![ident], T![yield], T![await]],
            )
            .enable_recovery_on_line_break(),
            expected_ts_type_parameter,
        )
    }

    fn separating_element_kind(&mut self) -> JsSyntaxKind {
        T![,]
    }

    fn allow_trailing_separating_element(&self) -> bool {
        true
    }
}

// test_err ts type_parameter_modifier
// export default function foo<in T>() {}
// export function foo<out T>() {}
// export function foo1<in T>() {}
// export function foo2<out T>() {}
// let foo: Foo<in T>
// let foo: Foo<out T>
// declare function foo<in T>()
// declare function foo<out T>()
// declare let foo: Foo<in T>
// declare let foo: Foo<out T>
// foo = function <in T>() {}
// foo = function <out T>() {}
// class Foo { foo<in T>(): T {} }
// class Foo { foo<out T>(): T {} }
// foo = { foo<in T>(): T {} };
// foo = { foo<out T>(): T {} };
// <in T>() => {};
// <out T>() => {};
// <in T, out T>() => {};
// let x: <in T>() => {};
// let x: <out T>() => {};
// let x: <in T, out T>() => {};
// let x: new <in T>() => {};
// let x: new <out T>() => {};
// let x: new <in T, out T>() => {};
// let x: { y<in T>(): any };
// let x: { y<out T>(): any };
// let x: { y<in T, out T>(): any };
// type Foo<i\\u006E T> = {}
// type Foo<ou\\u0074 T> = {}
// type Foo<in in> = {}
// type Foo<out in> = {}
// type Foo<out in T> = {}
// type Foo<public T> = {}
// type Foo<innn T> = {}
// type Foo<in out in T> = {}
// type Foo<in out out T> = {}
// function foo<in T>() {}
// function foo<out T>() {}
// type Foo<const U> = {};

// test tsx type_parameter_modifier_tsx
// <in T></in>;
// <out T></out>;
// <const T></const>;
// <in out T></in>;
// <out in T></out>;
// <in T extends={true}></in>;
// <out T extends={true}></out>;
// <in out T extends={true}></in>;

// test ts type_parameter_modifier
// type Foo<in T> = {}
// type Foo<out> = {}
// type Foo<out T> = {}
// type Foo<in out> = {}
// type Foo<out out> = {}
// type Foo<in out out> = {}
// type Foo<in X, out Y> = {}
// type Foo<out X, in Y> = {}
// type Foo<out X, out Y extends keyof X> = {}
// class Foo<in T> {}
// class Foo<out T> {}
// export default class Foo<in T> {}
// class Foo<out T> {}
// interface Foo<in T> {}
// interface Foo<out T> {}
// declare class Foo<in T> {}
// declare class Foo<out T> {}
// declare interface Foo<in T> {}
// declare interface Foo<out T> {}
// function a<const T>() {}
// function b<const T extends U>() {}
// function c<T, const U>() {}
// declare function d<const T>();
// <T>() => {};
// <const T>() => {};
// (function <const T>() {});
// (function <const T extends U>() {});
// (function <T, const U>() {});
// class A<const T> {}
// class B<const T extends U> {}
// class C<T, const U> {}
// class D<in const T> {}
// class E<const in T> {}
// class F<in const out T> {}
// (class <const T> {});
// (class <const T extends U> {});
// (class <T, const U> {});
// (class <in const T> {});
// (class <const in T> {});
// class _ {
//   method<const T>() {}
//   method<const T extends U>() {}
//   method<T, const U>() {}
// }
// declare module a {
//   function test<const T>(): T;
// }
// const obj = {
//   a<const T>(b: any): b is T { return true; }
// }

fn parse_ts_type_parameter(p: &mut JsParser, context: TypeContext) -> ParsedSyntax {
    let m = p.start();
    parse_ts_type_parameter_modifiers(p, context).ok();

    let name = parse_ts_type_parameter_name(p);
    parse_ts_type_constraint_clause(p, context).ok();
    parse_ts_default_type_clause(p).ok();

    if name.is_absent() {
        m.abandon(p);
        Absent
    } else {
        Present(m.complete(p, TS_TYPE_PARAMETER))
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
enum TypeParameterModifierKind {
    In,
    Out,
    Const,
}

/// Stores the range of a parsed modifier with its kind
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
struct TypeParameterModifier {
    kind: TypeParameterModifierKind,
    range: TextRange,
}

impl TypeParameterModifier {
    const fn as_syntax_kind(&self) -> JsSyntaxKind {
        match self.kind {
            TypeParameterModifierKind::In => TS_IN_MODIFIER,
            TypeParameterModifierKind::Out => TS_OUT_MODIFIER,
            TypeParameterModifierKind::Const => TS_CONST_MODIFIER,
        }
    }
}

#[derive(Debug, Default)]
struct ClassMemberModifierList(SmallVec<[TypeParameterModifier; 3]>);

impl ClassMemberModifierList {
    fn add_modifier(&mut self, modifier: TypeParameterModifier) {
        self.0.push(modifier);
    }

    fn find(&self, modifier_kind: &TypeParameterModifierKind) -> Option<&TypeParameterModifier> {
        self.0
            .iter()
            .find(|predicate| predicate.kind == *modifier_kind)
    }
}

pub(crate) fn is_nth_at_type_parameter_modifier(p: &mut JsParser, n: usize) -> bool {
    match p.nth(n) {
        T![in] | T![out] | T![const] => !p.nth_at(n + 1, T![,]) && !p.nth_at(n + 1, T![>]),
        _ => false,
    }
}

fn parse_ts_type_parameter_modifiers(p: &mut JsParser, context: TypeContext) -> ParsedSyntax {
    let list = p.start();
    let mut modifiers = ClassMemberModifierList::default();

    while is_nth_at_type_parameter_modifier(p, 0) {
        let modifier_kind = match p.cur() {
            T![in] => TypeParameterModifierKind::In,
            T![out] => TypeParameterModifierKind::Out,
            T![const] => TypeParameterModifierKind::Const,
            _ => unreachable!("keywords that are not 'in', 'out' and 'const' are checked earlier"),
        };
        let m = p.start();
        let text_range = p.cur_range();
        p.bump_any();

        if matches!(
            modifier_kind,
            TypeParameterModifierKind::In | TypeParameterModifierKind::Out,
        ) && !context.is_in_out_modifier_allowed()
        {
            p.error(ts_in_out_modifier_cannot_appear_on_a_type_parameter(
                p, text_range,
            ));
            m.abandon(p);
            continue;
        }

        if matches!(modifier_kind, TypeParameterModifierKind::Const)
            && !context.is_const_modifier_allowed()
        {
            p.error(ts_const_modifier_cannot_appear_on_a_type_parameter(
                p, text_range,
            ));
            m.abandon(p);
            continue;
        }

        // check for duplicate modifiers
        if let Some(existing_modifier) = modifiers.find(&modifier_kind) {
            p.error(modifier_already_seen(
                p,
                text_range,
                existing_modifier.range,
            ));
            m.abandon(p);
            continue;
        }

        // check for modifier precedence
        if let Some(ts_out_modifier) = modifiers.find(&TypeParameterModifierKind::Out) {
            if modifier_kind == TypeParameterModifierKind::In {
                p.error(modifier_must_precede_modifier(
                    p,
                    text_range,
                    ts_out_modifier.range,
                ));
                m.abandon(p);
                continue;
            }
        }

        let modifier = TypeParameterModifier {
            kind: modifier_kind,
            range: text_range,
        };
        modifiers.add_modifier(modifier);
        m.complete(p, modifier.as_syntax_kind());
    }

    Present(list.complete(p, TS_TYPE_PARAMETER_MODIFIER_LIST))
}

// test ts ts_type_constraint_clause
// type A<X extends number> = X;
// type B<X extends number | string> = { a: X }
fn parse_ts_type_constraint_clause(p: &mut JsParser, context: TypeContext) -> ParsedSyntax {
    if !p.at(T![extends]) {
        return Absent;
    }

    let m = p.start();
    p.expect(T![extends]);

    parse_ts_type(p, context).or_add_diagnostic(p, expected_ts_type);
    Present(m.complete(p, TS_TYPE_CONSTRAINT_CLAUSE))
}

// test ts ts_default_type_clause
// type A<X=string> = X;
// type B<X extends number | string = string> = { a: X }
fn parse_ts_default_type_clause(p: &mut JsParser) -> ParsedSyntax {
    if !p.at(T![=]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![=]);
    parse_ts_type(p, TypeContext::default()).or_add_diagnostic(p, expected_ts_type);
    Present(m.complete(p, TS_DEFAULT_TYPE_CLAUSE))
}

fn is_nth_at_ts_type_parameters(p: &mut JsParser, n: usize) -> bool {
    p.nth_at(n, T![<])
}

#[inline(always)]
pub(crate) fn parse_ts_type(p: &mut JsParser, context: TypeContext) -> ParsedSyntax {
    p.with_state(EnterType, |p| {
        if is_at_constructor_type(p) {
            return parse_ts_constructor_type(p, context);
        }

        if is_at_function_type(p) {
            return parse_ts_function_type(p, context);
        }

        let left = parse_ts_union_type_or_higher(p, context);

        // test ts ts_conditional_type_call_signature_lhs
        // type X<V> = V extends (...args: any[]) => any ? (...args: Parameters<V>) => void : Function;
        if context.is_conditional_type_allowed() {
            left.map(|left| {
                // test ts ts_conditional_type
                // type A = number;
                // type B = string extends number ? string : number;
                // type C = A extends (B extends A ? number : string) ? void : number;
                // type D<T> = T extends [infer S extends string, ...unknown[]] ? S : never;
                // type E<U, T> = T extends (infer U extends number ? U : T ) ? U : T
                // type F<T> = T extends { [P in infer U extends keyof T ? 1 : 0]: 1; } ? 1 : 0;
                // type G<T> = T extends [unknown, infer S extends string] ? S : never;
                // type H = A extends () => B extends C ? D : E ? F : G;
                // type J<T> = T extends ((...a: any[]) => infer R extends string) ? R : never;
                // type Equals = A extends (x: B extends C ? D : E) => 0 ? F : G;
                // type Curry<F extends ((...args: any) => any)> =
                //     <T extends any[]>(...args: Tools.Cast<Tools.Cast<T, Gaps<Parameters<F>>>, any[]>) =>
                //          GapsOf<T, Parameters<F>> extends [any, ...any[]]
                //          ? Curry<(...args: GapsOf<T, Parameters<F>> extends infer G ? Tools.Cast<G, any[]> : never) => ReturnType<F>>
                //          : ReturnType<F>;
                // interface GapsOfWorker<T1 extends any[], T2 extends any[], TN extends any[] = [], I extends any[] = []> {
                //     0: GapsOf<T1, T2, GapOf<T1, T2, TN, I> extends infer G ? Tools.Cast<G, any[]> : never, Tools.Next<I>>;
                //     1: Tools.Concat<TN, Tools.Drop<Tools.Pos<I>, T2> extends infer D ? Tools.Cast<D, any[]> : never>;
                // }
                if !p.has_preceding_line_break() && p.at(T![extends]) {
                    let m = left.precede(p);
                    p.expect(T![extends]);

                    parse_ts_type(
                        p,
                        context
                            .and_allow_conditional_types(false)
                            .and_in_conditional_extends(true),
                    )
                    .or_add_diagnostic(p, expected_ts_type);
                    p.expect(T![?]);
                    parse_ts_type(p, context).or_add_diagnostic(p, expected_ts_type);
                    p.expect(T![:]);
                    parse_ts_type(p, context).or_add_diagnostic(p, expected_ts_type);
                    m.complete(p, TS_CONDITIONAL_TYPE)
                } else {
                    left
                }
            })
        } else {
            left
        }
    })
}

// test ts ts_union_type
// type A = string | number;
// type B = | A | void | null;
// type C = A & C | C;
fn parse_ts_union_type_or_higher(p: &mut JsParser, context: TypeContext) -> ParsedSyntax {
    parse_ts_union_or_intersection_type(p, IntersectionOrUnionType::Union, context)
}

// test ts ts_intersection_type
// type A = string & number;
// type B = & A & void & null;
fn parse_ts_intersection_type_or_higher(p: &mut JsParser, context: TypeContext) -> ParsedSyntax {
    parse_ts_union_or_intersection_type(p, IntersectionOrUnionType::Intersection, context)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum IntersectionOrUnionType {
    Intersection,
    Union,
}

impl IntersectionOrUnionType {
    #[inline]
    fn operator(&self) -> JsSyntaxKind {
        match self {
            IntersectionOrUnionType::Union => T![|],
            IntersectionOrUnionType::Intersection => T![&],
        }
    }

    #[inline]
    fn list_kind(&self) -> JsSyntaxKind {
        match self {
            IntersectionOrUnionType::Union => TS_UNION_TYPE_VARIANT_LIST,
            IntersectionOrUnionType::Intersection => TS_INTERSECTION_TYPE_ELEMENT_LIST,
        }
    }

    #[inline]
    fn kind(&self) -> JsSyntaxKind {
        match self {
            IntersectionOrUnionType::Union => TS_UNION_TYPE,
            IntersectionOrUnionType::Intersection => TS_INTERSECTION_TYPE,
        }
    }

    #[inline]
    fn parse_element(&self, p: &mut JsParser, context: TypeContext) -> ParsedSyntax {
        match self {
            IntersectionOrUnionType::Union => parse_ts_intersection_type_or_higher(p, context),
            IntersectionOrUnionType::Intersection => parse_ts_primary_type(p, context),
        }
    }
}

#[inline]
fn parse_ts_union_or_intersection_type(
    p: &mut JsParser,
    ty_kind: IntersectionOrUnionType,
    context: TypeContext,
) -> ParsedSyntax {
    // Leading operator: `& A & B`
    if p.at(ty_kind.operator()) {
        let m = p.start();
        p.bump(ty_kind.operator());
        let list = p.start();
        ty_kind
            .parse_element(p, context)
            .or_add_diagnostic(p, expected_ts_type);

        eat_ts_union_or_intersection_type_elements(p, ty_kind, context);

        list.complete(p, ty_kind.list_kind());

        Present(m.complete(p, ty_kind.kind()))
    } else {
        let first = ty_kind.parse_element(p, context);

        if p.at(ty_kind.operator()) {
            let list = first.precede(p);

            eat_ts_union_or_intersection_type_elements(p, ty_kind, context);

            let completed_list = list.complete(p, ty_kind.list_kind());
            let m = completed_list.precede(p);
            Present(m.complete(p, ty_kind.kind()))
        } else {
            // Not a union or intersection type
            first
        }
    }
}

#[inline]
fn eat_ts_union_or_intersection_type_elements(
    p: &mut JsParser,
    ty_kind: IntersectionOrUnionType,
    context: TypeContext,
) {
    while p.at(ty_kind.operator()) {
        p.bump(ty_kind.operator());

        ty_kind
            .parse_element(p, context)
            .or_add_diagnostic(p, expected_ts_type);
    }
}

fn parse_ts_primary_type(p: &mut JsParser, context: TypeContext) -> ParsedSyntax {
    // test ts ts_inferred_type
    // type A = A extends infer B ? B : never;
    // type B = A extends { a: infer U; b: infer U} ? U : never;
    if p.at(T![infer]) {
        let m = p.start();
        p.expect(T![infer]);
        parse_ts_type_parameter_name(p).or_add_diagnostic(p, expected_identifier);
        try_parse_constraint_of_infer_type(p, context).ok();

        return if context.in_conditional_extends() {
            // test ts ts_infer_type_allowed
            // type Args<F> = F extends (...args: infer A) => void ? A : never;
            // type A = T extends import("test").C<infer P> ? P : never
            // type A = T extends typeof Array<infer P> ? P : never
            // type A = T extends { set(a: infer P): number } ? P : never
            // type A = T extends { get(): infer P } ? P : never
            // type A = T extends { method(this: infer P): number } ? P : never
            // type valid9<T> = T extends Array<infer R> ? R : any;
            // type ContentBetweenBrackets<S> = S extends `[${infer T}]` ? T : never;
            // type WithSelectors<S> = S extends { getState: () => infer T }
            //     ? S & { use: { [K in keyof T]: () => T[K] } }
            //     : never;
            // type A = MyType extends (OtherType extends infer T ? infer U : InnerFalse) ? OuterTrue : OuterFalse
            // type Join<T extends unknown[], D extends string> =
            //      T extends [] ? '' :
            //      T extends [string | number | boolean | bigint] ? `${T[0]}` :
            //      T extends [string | number | boolean | bigint, ...infer U] ? `${T[0]}${D}${Join<U, D>}` :
            //      string;
            // type MatchPair<S extends string> = S extends `[${infer A},${infer B}]` ? [A, B] : unknown;
            // type FirstTwoAndRest<S extends string> = S extends `${infer A}${infer B}${infer R}` ? [`${A}${B}`, R] : unknown;
            // type Trim<S extends string> =
            //      S extends `${infer T} ` ? Trim<T> :
            //      S;
            // type Foo<T> = T extends `*${infer S}*` ? S : never;
            // type Unpacked<T> = T extends (infer U)[] ? U :
            //     T extends (...args: any[]) => infer U ? U :
            //     T extends Promise<infer U> ? U :
            //     T;
            // type ArgumentType<T extends (x: any) => any> = T extends (a: infer A) => any ? A : any;
            // type X3<T> = T extends { a: (x: infer U) => void, b: (x: infer U) => void } ? U : never;
            // type X1<T extends { x: any, y: any }> = T extends { x: infer X, y: infer Y } ? [X, Y] : any;
            // type T62<T> = U extends (infer U)[] ? U : U;
            // type T63<T> = T extends ((infer A) extends infer B ? infer C : infer D) ? string : number;
            // type T75<T> = T extends T74<infer U, infer U> ? T70<U> | T72<U> | T74<U, U> : never;
            // type Jsonified<T> = T extends string | number | boolean | null ? T
            //     : T extends undefined | Function ? never
            //     : T extends { toJSON(): infer R } ? R
            //     : T extends object ? JsonifiedObject<T>
            // : "what is this";
            // type T1 = F1 extends (...args: (infer T)) => void ? T : never;
            // type T2 = F1 extends (args: [...(infer T)]) => void ? T : never;
            // type T3<T> = T extends IsNumber<(infer N)> ? true : false;
            // type T4 = F1 extends (...args: ((infer T))) => void ? T : never;
            // type T5 = F1 extends (args: [...((infer T))]) => void ? T : never;
            // type T6<T> = T extends IsNumber<((infer N))> ? true : false;
            // type T7 = F1 extends (...args: ((((infer T))))) => void ? T : never;
            // type T8 = F1 extends (args: [...((((infer T))))]) => void ? T : never;
            // type T9<T> = T extends IsNumber<((((infer N))))> ? true : false;
            // type Prepend<E, T extends any[]> =
            //     ((head: E, ...args: T) => any) extends ((...args: infer U) => any)
            //     ? U
            //     : T;
            Present(m.complete(p, TS_INFER_TYPE))
        } else {
            // test_err ts ts_infer_type_not_allowed
            // type WithSelectors<S> = S extends { getState: () => infer T }
            //   ? { use: { [K in keyof infer /*error*/ T]: () => T[K] } }
            //   : never;
            // type TV1 = `${infer X}`;
            // type T61<T> = (infer A) extends infer B ? infer C : infer D;
            // type A = {a: infer T}
            // type A = () => infer T;
            // let s: (infer string)[] = symbol();
            // let s: unique (infer string);
            // let s: [number, ...infer string]
            // let s: [(infer string)?]
            // let s: (infer string)[a]
            // let s: a[(infer string)]
            let infer_type = m.complete(p, TS_BOGUS_TYPE);
            p.error(infer_not_allowed(p, infer_type.range(p)));

            Present(infer_type)
        };
    }

    // test ts ts_type_operator
    // type A = { x: string, y: number };
    // type B = keyof A;
    // type C = readonly string[];
    // const d: unique symbol = Symbol();
    let is_type_operator = matches!(p.cur(), T![unique] | T![keyof] | T![readonly]);
    if is_type_operator {
        let m = p.start();
        p.bump_any();
        parse_ts_primary_type(p, context).or_add_diagnostic(p, expected_ts_type);
        return Present(m.complete(p, TS_TYPE_OPERATOR_TYPE));
    }

    parse_postfix_type_or_higher(p, context.and_allow_conditional_types(true))
}

fn try_parse_constraint_of_infer_type(p: &mut JsParser, context: TypeContext) -> ParsedSyntax {
    if !p.at(T![extends]) {
        return Absent;
    }

    try_parse(p, |p| {
        let parsed = parse_ts_type_constraint_clause(p, context.and_allow_conditional_types(false))
            .expect("Type constraint clause because parser is positioned at expect clause");

        // Rewind if conditional types are allowed, and the parser is at the `?` token because
        // this should instead be parsed as a conditional type.
        if context.is_conditional_type_allowed() && p.at(T![?]) {
            Err(())
        } else {
            Ok(Present(parsed))
        }
    })
    .unwrap_or(Absent)
}

fn parse_postfix_type_or_higher(p: &mut JsParser, context: TypeContext) -> ParsedSyntax {
    parse_ts_non_array_type(p, context).map(|primary_type| {
        let mut left = primary_type;

        while p.at(T!['[']) && !p.has_preceding_line_break() {
            let m = left.precede(p);
            p.bump(T!['[']);

            left = if parse_ts_type(p, context).is_present() {
                // test ts ts_indexed_access_type
                // type A = string[number];
                // type B = string[number][number][number][];
                p.expect(T![']']);
                m.complete(p, TS_INDEXED_ACCESS_TYPE)
            } else {
                // test ts ts_array_type
                // type A = string[];
                // type B = { a: number } [];
                p.expect(T![']']);
                m.complete(p, TS_ARRAY_TYPE)
            }
        }

        left
    })
}

fn parse_ts_non_array_type(p: &mut JsParser, context: TypeContext) -> ParsedSyntax {
    // test ts ts_predefined_type
    // type A = any
    // type B = number;
    // type C = object;
    // type D = boolean;
    // type E = bigint;
    // type F = string;
    // type G = symbol;
    // type H = void;
    // type I = undefined;
    // type J = null;
    // type K = never
    match p.cur() {
        T!['('] => parse_ts_parenthesized_type(p, context),
        T!['{'] => {
            if is_at_start_of_mapped_type(p) {
                parse_ts_mapped_type(p, context)
            } else {
                parse_ts_object_type(p, context)
            }
        }
        T!['['] => parse_ts_tuple_type(p, context),
        T![void] => {
            let m = p.start();
            p.expect(T![void]);
            Present(m.complete(p, TS_VOID_TYPE))
        }
        JS_NUMBER_LITERAL | JS_STRING_LITERAL | TRUE_KW | FALSE_KW | T![null] => {
            parse_ts_literal_type(p)
        }
        BACKTICK => parse_ts_template_literal_type(p, context),
        T![-] if p.nth_at(1, JS_NUMBER_LITERAL) => parse_ts_literal_type(p),
        T![this] => parse_ts_this_type(p),
        T![typeof] => {
            if p.nth_at(1, T![import]) {
                parse_ts_import_type(p, context)
            } else {
                parse_ts_typeof_type(p, context)
            }
        }
        T![import] => parse_ts_import_type(p, context),
        t if t.is_metavariable() => parse_metavariable(p),
        _ => {
            if !p.nth_at(1, T![.]) {
                let mapping = match p.cur() {
                    T![any] => Some(TS_ANY_TYPE),
                    T![unknown] => Some(TS_UNKNOWN_TYPE),
                    T![number] => Some(TS_NUMBER_TYPE),
                    T![object] => Some(TS_NON_PRIMITIVE_TYPE),
                    T![boolean] => Some(TS_BOOLEAN_TYPE),
                    T![bigint] => Some(TS_BIGINT_TYPE),
                    T![string] => Some(TS_STRING_TYPE),
                    T![symbol] => Some(TS_SYMBOL_TYPE),
                    T![undefined] => Some(TS_UNDEFINED_TYPE),
                    T![never] => Some(TS_NEVER_TYPE),
                    _ => None,
                };

                if let Some(literal_type_kind) = mapping {
                    let m = p.start();
                    p.bump_any();
                    return Present(m.complete(p, literal_type_kind));
                }
            }

            parse_ts_reference_type(p, context)
        }
    }
}

// test ts ts_reference_type
// type A = object;
// type B = string;
// type C = A;
// type D = B.a;
// type E = D.c.b.a;
pub(crate) fn parse_ts_reference_type(p: &mut JsParser, context: TypeContext) -> ParsedSyntax {
    parse_ts_name(p).map(|name| {
        let m = name.precede(p);

        if !p.has_preceding_line_break() {
            parse_ts_type_arguments(p, context).ok();
        }

        m.complete(p, TS_REFERENCE_TYPE)
    })
}

pub(crate) fn parse_ts_name(p: &mut JsParser) -> ParsedSyntax {
    let mut left = if p.cur().is_non_contextual_keyword() && !p.cur().is_future_reserved_keyword() {
        let m = p.start();
        p.bump_remap(T![ident]);
        Present(m.complete(p, JS_REFERENCE_IDENTIFIER))
    } else {
        parse_reference_identifier(p)
    };

    while p.at(T![.]) {
        let m = left.precede_or_add_diagnostic(p, expected_identifier);
        p.bump(T![.]);
        parse_name(p).or_add_diagnostic(p, expected_identifier);
        left = Present(m.complete(p, TS_QUALIFIED_NAME));
    }

    left
}

// test ts ts_typeof_type
// let a = "test";
// type B = typeof a;
// type T21 = typeof Array<string>;
// type A<U> = InstanceType<typeof Array<U>>;

// test tsx ts_typeof_type2
// type X = typeof Array
// <div>a</div>;
fn parse_ts_typeof_type(p: &mut JsParser, context: TypeContext) -> ParsedSyntax {
    if !p.at(T![typeof]) {
        return Absent;
    }

    let m = p.start();
    p.expect(T![typeof]);
    parse_ts_name(p).or_add_diagnostic(p, expected_identifier);
    if !p.has_preceding_line_break() {
        parse_ts_type_arguments(p, context).ok();
    }

    Present(m.complete(p, TS_TYPEOF_TYPE))
}

// test ts ts_this_type
// class A {
//     method() {
//         type A = this;
//     }
//     predicate(): this is string {
//         return typeof this === "string"
//     }
// }
fn parse_ts_this_type(p: &mut JsParser) -> ParsedSyntax {
    if !p.at(T![this]) {
        return Absent;
    }

    let m = p.start();
    p.expect(T![this]);
    Present(m.complete(p, TS_THIS_TYPE))
}

// test ts ts_parenthesized_type
// type A = (string)
fn parse_ts_parenthesized_type(p: &mut JsParser, context: TypeContext) -> ParsedSyntax {
    if !p.at(T!['(']) {
        return Absent;
    }

    let m = p.start();
    p.bump(T!['(']);
    parse_ts_type(p, context).or_add_diagnostic(p, expected_ts_type);
    p.expect(T![')']);
    Present(m.complete(p, TS_PARENTHESIZED_TYPE))
}

fn is_at_start_of_mapped_type(p: &mut JsParser) -> bool {
    if !p.at(T!['{']) {
        return false;
    }

    if p.nth_at(1, T![+]) || p.nth_at(1, T![-]) {
        return p.nth_at(2, T![readonly]);
    }

    let mut offset = 1;

    if p.nth_at(offset, T![readonly]) {
        offset += 1;
    }

    p.nth_at(offset, T!['['])
        && (is_nth_at_identifier(p, offset + 1) || p.nth(offset + 1).is_keyword())
        && p.nth_at(offset + 2, T![in])
}

// test ts issue_2790
// var x: {
//   readonly [A in keyof B]?: any;
// };

// test ts ts_mapped_type
// type A = { [test in "a" | "b"] }
// type OptionsFlags<Type> = {
//   [Property in keyof Type]: boolean;
// };
// type CreateMutable<Type> = {
// 	-readonly [Property in keyof Type]: Type[Property];
// };
// type Concrete<Type> = {
//   [Property in keyof Type]-?: Type[Property]
// };
// type Getters<Type> = {
//     [Property in keyof Type as `get${Capitalize<string & Property>}`]: () => Type[Property]
// };
fn parse_ts_mapped_type(p: &mut JsParser, context: TypeContext) -> ParsedSyntax {
    if !p.at(T!['{']) {
        return Absent;
    }

    let m = p.start();
    p.bump(T!['{']);
    parse_ts_mapped_type_readonly_modifier_clause(p).ok();
    p.expect(T!['[']);
    parse_ts_type_parameter_name(p).or_add_diagnostic(p, expected_ts_type_parameter);
    p.expect(T![in]);
    parse_ts_type(p, context).or_add_diagnostic(p, expected_ts_type);
    parse_ts_mapped_type_as_clause(p, context).ok();
    p.expect(T![']']);
    parse_ts_mapped_type_optional_modifier_clause(p).ok();
    parse_ts_type_annotation(p, context).ok();
    p.eat(T![;]);
    p.expect(T!['}']);

    Present(m.complete(p, TS_MAPPED_TYPE))
}

fn parse_ts_mapped_type_as_clause(p: &mut JsParser, context: TypeContext) -> ParsedSyntax {
    if !p.at(T![as]) {
        return Absent;
    }

    let m = p.start();
    p.bump_remap(T![as]);
    parse_ts_type(p, context).or_add_diagnostic(p, expected_ts_type);
    Present(m.complete(p, TS_MAPPED_TYPE_AS_CLAUSE))
}

fn parse_ts_mapped_type_readonly_modifier_clause(p: &mut JsParser) -> ParsedSyntax {
    if p.at(T![readonly]) {
        let m = p.start();
        p.expect(T![readonly]);
        Present(m.complete(p, TS_MAPPED_TYPE_READONLY_MODIFIER_CLAUSE))
    } else if p.at(T![+]) || p.at(T![-]) {
        let m = p.start();
        p.bump_any();
        p.expect(T![readonly]);
        Present(m.complete(p, TS_MAPPED_TYPE_READONLY_MODIFIER_CLAUSE))
    } else {
        Absent
    }
}

fn parse_ts_mapped_type_optional_modifier_clause(p: &mut JsParser) -> ParsedSyntax {
    match p.cur() {
        T![?] => {
            let m = p.start();
            p.bump(T![?]);
            Present(m.complete(p, TS_MAPPED_TYPE_OPTIONAL_MODIFIER_CLAUSE))
        }
        T![-] | T![+] => {
            let m = p.start();
            p.bump_any();
            p.expect(T![?]);
            Present(m.complete(p, TS_MAPPED_TYPE_OPTIONAL_MODIFIER_CLAUSE))
        }
        _ => Absent,
    }
}

// test ts ts_import_type
// type A = typeof import("test");
// type B = import("test");
// type C = typeof import("test").a.b.c.d.e.f;
// type D = import("test")<string>;
// type E = import("test").C<string>;
// type F = typeof import("test", { with: { "resolution-mode": "import" } });
// type G = import("test", { with: { "resolution-mode": "import" } }).TypeFromImport;
// type H = import("test", { with: { "resolution-mode": "import" } })<string>;
// type I = import("test", { with: { "resolution-mode": "require" } }).C<string>;
// type J = typeof import("test", { with: { "resolution-mode": "require" } }).a.b.c.d.e.f;
fn parse_ts_import_type(p: &mut JsParser, context: TypeContext) -> ParsedSyntax {
    if !p.at(T![typeof]) && !p.at(T![import]) {
        return Absent;
    }

    let m = p.start();
    p.eat(T![typeof]);
    p.expect(T![import]);

    parse_ts_import_type_arguments(p, context)
        .or_add_diagnostic(p, expected_ts_import_type_with_arguments);

    if p.at(T![.]) {
        let qualifier = p.start();
        p.bump(T![.]);
        parse_ts_name(p).or_add_diagnostic(p, expected_identifier);
        qualifier.complete(p, TS_IMPORT_TYPE_QUALIFIER);
    }

    parse_ts_type_arguments(p, context).ok();

    Present(m.complete(p, TS_IMPORT_TYPE))
}

// test ts ts_object_type
// type A = { a: string, b: number };
// type B = { a: string; b: number };
// type C = { a: string, b: number; c: string };
// type D = {
// 	a: string
//  b: number
// }
fn parse_ts_object_type(p: &mut JsParser, context: TypeContext) -> ParsedSyntax {
    if !p.at(T!['{']) {
        return Absent;
    }

    let m = p.start();
    p.bump(T!['{']);
    TypeMembers::new(context).parse_list(p);
    p.expect(T!['}']);
    Present(m.complete(p, TS_OBJECT_TYPE))
}

#[derive(Debug, Default)]
pub(crate) struct TypeMembers {
    context: TypeContext,
}

impl TypeMembers {
    pub fn new(context: TypeContext) -> Self {
        Self { context }
    }
}

impl ParseNodeList for TypeMembers {
    type Kind = JsSyntaxKind;
    type Parser<'source> = JsParser<'source>;

    const LIST_KIND: Self::Kind = TS_TYPE_MEMBER_LIST;

    fn parse_element(&mut self, p: &mut JsParser) -> ParsedSyntax {
        parse_ts_type_member(p, self.context)
    }

    fn is_at_list_end(&self, p: &mut JsParser) -> bool {
        p.at(T!['}'])
    }

    fn recover(&mut self, p: &mut JsParser, member: ParsedSyntax) -> RecoveryResult {
        member.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(JS_BOGUS, token_set![T!['}'], T![,], T![;]])
                .enable_recovery_on_line_break(),
            expected_property_or_signature,
        )
    }
}

fn parse_ts_type_member(p: &mut JsParser, context: TypeContext) -> ParsedSyntax {
    // test ts ts_index_signature_member
    // type A = { [a: number]: string }
    // type B = { readonly [a: number]: string }
    // // not an index signature
    // type C = { [a]: string }
    // type D = { readonly [a]: string }
    if is_at_ts_index_signature_member(p) {
        let m = p.start();
        return Present(expect_ts_index_signature_member(
            p,
            m,
            MemberParent::TypeOrInterface,
            context,
        ));
    }

    match p.cur() {
        T!['('] | T![<] => parse_ts_call_signature_type_member(p, context),
        T![new] if is_at_ts_construct_signature_type_member(p) => {
            parse_ts_construct_signature_type_member(p, context)
        }
        T![get] if is_nth_at_type_member_name(p, 1) => {
            parse_ts_getter_signature_type_member(p, context)
        }
        T![set] if is_nth_at_type_member_name(p, 1) => {
            parse_ts_setter_signature_type_member(p, context)
        }
        _ => parse_ts_property_or_method_signature_type_member(p, context),
    }
}

// test ts ts_property_or_method_signature_member
// type A = { a?: string; b?(): number }
// type B = { a: string, b(): number }
// type C = { m(a: string, b: number, c: string): any }
// type D = { readonly: string, readonly a: number }
// type E = { m<A, B>(a: A, b: B): never }
// type F = { m<const A>(a: A): never }
fn parse_ts_property_or_method_signature_type_member(
    p: &mut JsParser,
    context: TypeContext,
) -> ParsedSyntax {
    if !is_at_object_member_name(p) {
        return Absent;
    }

    let m = p.start();
    let readonly_range = if p.at(T![readonly]) && is_nth_at_type_member_name(p, 1) {
        let range = p.cur_range();
        p.expect(T![readonly]);
        Some(range)
    } else {
        None
    };

    parse_object_member_name(p).unwrap();

    p.eat(T![?]);

    if p.at(T!['(']) || p.at(T![<]) {
        parse_ts_call_signature(p, context.and_allow_const_modifier(true));
        parse_ts_type_member_semi(p);
        let method = m.complete(p, TS_METHOD_SIGNATURE_TYPE_MEMBER);

        if let Some(readonly_range) = readonly_range {
            p.error(p.err_builder(
                "readonly modifier can only appear on a property or signature declaration",
                readonly_range,
            ));
        }

        Present(method)
    } else {
        parse_ts_type_annotation(p, context).ok();
        parse_ts_type_member_semi(p);
        Present(m.complete(p, TS_PROPERTY_SIGNATURE_TYPE_MEMBER))
    }
}

// test ts ts_call_signature_member
// type A = { (): string; }
// type B = { (a, b, c): number }
// type C = { <A, B>(a: A, b: B): number }
// type D = { <const A>(a: A): number }
fn parse_ts_call_signature_type_member(p: &mut JsParser, context: TypeContext) -> ParsedSyntax {
    if !(p.at(T!['(']) || p.at(T![<])) {
        return Absent;
    }

    let m = p.start();
    parse_ts_call_signature(p, context.and_allow_const_modifier(true));
    parse_ts_type_member_semi(p);
    Present(m.complete(p, TS_CALL_SIGNATURE_TYPE_MEMBER))
}

// test ts ts_construct_signature_member
// type A = { new (): string; }
// type B = { new (a: string, b: number) }
// type C = { new <A, B>(a: A, b: B): string }
// type D = { new <const T>(a: T, b: B): string }

// test_err ts ts_construct_signature_member_err
// type C = { new <>(a: A, b: B): string }
fn parse_ts_construct_signature_type_member(
    p: &mut JsParser,
    context: TypeContext,
) -> ParsedSyntax {
    if !is_at_ts_construct_signature_type_member(p) {
        return Absent;
    }

    let m = p.start();
    p.expect(T![new]);
    parse_ts_type_parameters(
        p,
        context
            .and_allow_const_modifier(true)
            .and_allow_in_out_modifier(true),
    )
    .ok();
    parse_parameter_list(
        p,
        ParameterContext::Declaration,
        context,
        SignatureFlags::empty(),
    )
    .or_add_diagnostic(p, expected_parameters);
    parse_ts_type_annotation(p, context).ok();
    parse_ts_type_member_semi(p);

    Present(m.complete(p, TS_CONSTRUCT_SIGNATURE_TYPE_MEMBER))
}

// test ts ts_getter_signature_member
// type A = { get a(): number }
// type B = { get a() }
// // members that look like getters but aren't getters
// type C = { get(): number }
// type D = { get: number }
// type E = { get }
fn parse_ts_getter_signature_type_member(p: &mut JsParser, context: TypeContext) -> ParsedSyntax {
    if !p.at(T![get]) {
        return Absent;
    }

    let m = p.start();
    p.expect(T![get]);
    parse_object_member_name(p).or_add_diagnostic(p, expected_object_member_name);
    p.expect(T!['(']);
    p.expect(T![')']);
    parse_ts_type_annotation(p, context).ok();
    parse_ts_type_member_semi(p);
    Present(m.complete(p, TS_GETTER_SIGNATURE_TYPE_MEMBER))
}

// test ts ts_setter_signature_member
// type A = { set a(b: number) }
// type B = { set a(b) }
// // members that look like setters but aren't setters
// type C = { set(a) }
// type D = { set: number }
// type E = { set }
// type F = { set(b: number,) }
// type G = {set a(b,)}
// type H = {set(a, ) }
fn parse_ts_setter_signature_type_member(p: &mut JsParser, context: TypeContext) -> ParsedSyntax {
    if !p.at(T![set]) {
        return Absent;
    }

    let m = p.start();
    p.expect(T![set]);
    parse_object_member_name(p).or_add_diagnostic(p, expected_object_member_name);
    p.expect(T!['(']);

    // test_err ts ts_decorator_setter_signature
    // type A = { set a(@dec b: number) }
    // type B = { set a(@dec(val) b) }
    // type AA = { set a(@dec b: number) }
    // type BB = { set a(@dec(val) b) }
    let decorator_list = parse_decorators(p)
        .add_diagnostic_if_present(p, decorators_not_allowed)
        .map(|mut decorator_list| {
            decorator_list.change_to_bogus(p);
            decorator_list
        })
        .into();

    parse_formal_parameter(
        p,
        decorator_list,
        ParameterContext::Setter,
        ExpressionContext::default(),
        context,
    )
    .or_add_diagnostic(p, expected_parameter);

    if p.at(T![,]) {
        p.bump_any();
    }

    p.expect(T![')']);
    parse_ts_type_member_semi(p);
    Present(m.complete(p, TS_SETTER_SIGNATURE_TYPE_MEMBER))
}

// test ts ts_tuple_type
// type A = [string, number, any]
// type B = [a: string, b: number, c: any]
// type C = [a: string, b: number, ...c: any[]]
// type D = [a?: string]
// type E = [...string[]]
// type F = [string?]
//
// test_err ts ts_tuple_type_incomplete
// type A = [string,
fn parse_ts_tuple_type(p: &mut JsParser, context: TypeContext) -> ParsedSyntax {
    if !p.at(T!['[']) {
        return Absent;
    }

    let m = p.start();
    p.bump(T!['[']);
    TsTupleTypeElementList(context).parse_list(p);
    p.expect(T![']']);

    Present(m.complete(p, TS_TUPLE_TYPE))
}

struct TsTupleTypeElementList(TypeContext);

impl ParseSeparatedList for TsTupleTypeElementList {
    type Kind = JsSyntaxKind;
    type Parser<'source> = JsParser<'source>;

    const LIST_KIND: Self::Kind = TS_TUPLE_TYPE_ELEMENT_LIST;

    fn parse_element(&mut self, p: &mut JsParser) -> ParsedSyntax {
        if is_at_named_tuple_type_element(p) {
            let m = p.start();
            let has_ellipsis = p.eat(T![...]);
            parse_name(p).or_add_diagnostic(p, expected_identifier);
            let has_question_mark = p.eat(T![?]);
            p.bump(T![:]);
            parse_ts_type(p, self.0).or_add_diagnostic(p, expected_ts_type);

            let mut syntax = m.complete(p, TS_NAMED_TUPLE_TYPE_ELEMENT);

            // test_err ts ts_tuple_type_cannot_be_optional_and_rest
            // type A = [ ...name?: string[] ]
            if has_ellipsis && has_question_mark {
                let err = p.err_builder(
                    "A tuple member cannot be both optional and rest.",
                    syntax.range(p).as_range(),
                );
                p.error(err);
                syntax.change_to_bogus(p);
            }

            return Present(syntax);
        }

        if p.at(T![...]) {
            let m = p.start();
            p.bump(T![...]);
            parse_ts_type(p, self.0).or_add_diagnostic(p, expected_ts_type);
            return Present(m.complete(p, TS_REST_TUPLE_TYPE_ELEMENT));
        }

        let ty = parse_ts_type(p, self.0);

        if p.at(T![?]) {
            let m = ty.precede_or_add_diagnostic(p, expected_ts_type);
            p.bump(T![?]);
            return Present(m.complete(p, TS_OPTIONAL_TUPLE_TYPE_ELEMENT));
        }

        ty
    }

    fn is_at_list_end(&self, p: &mut JsParser) -> bool {
        p.at(T![']'])
    }

    fn recover(&mut self, p: &mut JsParser, parsed_element: ParsedSyntax) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(
                TS_BOGUS_TYPE,
                token_set![
                    T![']'],
                    T![...],
                    T![ident],
                    T!['['],
                    T!['{'],
                    T![void],
                    T![null]
                ],
            ),
            expected_ts_type,
        )
    }

    fn separating_element_kind(&mut self) -> JsSyntaxKind {
        T![,]
    }

    fn allow_trailing_separating_element(&self) -> bool {
        true
    }
}

fn is_at_named_tuple_type_element(p: &mut JsParser) -> bool {
    let offset = usize::from(p.at(T![...]));

    // a:
    let is_colon = p.nth_at(offset + 1, T![:]);
    // a?:
    let is_question_colon = p.nth_at(offset + 1, T![?]) && p.nth_at(offset + 2, T![:]);

    is_nth_at_identifier_or_keyword(p, offset) && (is_colon || is_question_colon)
}

// test ts ts_literal_type
// type A = 5;
// type B = -5;
// type C = 5n;
// type D = -5n;
// type E = "abvcd";
// type F = true;
// type G = false;
// type H = null;
fn parse_ts_literal_type(p: &mut JsParser) -> ParsedSyntax {
    if p.cur() == JS_REGEX_LITERAL {
        return Absent;
    }

    if p.at(T![-]) && p.nth_at(1, JS_NUMBER_LITERAL) {
        let m = p.start();
        p.bump(T![-]);

        let number_expr = parse_number_literal_expression(p)
            .or_else(|| parse_big_int_literal_expression(p))
            .unwrap();

        let type_kind = match number_expr.kind(p) {
            JS_NUMBER_LITERAL_EXPRESSION => TS_NUMBER_LITERAL_TYPE,
            JS_BIGINT_LITERAL_EXPRESSION => TS_BIGINT_LITERAL_TYPE,
            _ => unreachable!(),
        };

        // Inline the number or big int literal into the number/big int literal type
        number_expr.undo_completion(p).abandon(p);

        return Present(m.complete(p, type_kind));
    }

    parse_literal_expression(p).map(|mut expression| {
        let type_kind = match expression.kind(p) {
            JS_NUMBER_LITERAL_EXPRESSION => TS_NUMBER_LITERAL_TYPE,
            JS_BIGINT_LITERAL_EXPRESSION => TS_BIGINT_LITERAL_TYPE,
            JS_NULL_LITERAL_EXPRESSION => TS_NULL_LITERAL_TYPE,
            JS_BOOLEAN_LITERAL_EXPRESSION => TS_BOOLEAN_LITERAL_TYPE,
            JS_STRING_LITERAL_EXPRESSION => TS_STRING_LITERAL_TYPE,
            kind => unreachable!("Not a valid kind {:?}", kind),
        };

        expression.change_kind(p, type_kind);
        expression
    })
}

// test ts ts_template_literal_type
// type A = `abcd`
// type B = `a${A}`
// type C<X extends string> = `c${X}`
//
// test_err ts ts_template_literal_error
// type A = "a";
// type B = "b"
// type C = `${A B}bcd`
// type D = `${A B`
fn parse_ts_template_literal_type(p: &mut JsParser, context: TypeContext) -> ParsedSyntax {
    if !p.at(BACKTICK) {
        return Absent;
    }

    let m = p.start();
    p.bump_with_context(BACKTICK, JsLexContext::TemplateElement { tagged: false });

    let elements = p.start();
    parse_template_elements(
        p,
        TS_TEMPLATE_CHUNK_ELEMENT,
        TS_TEMPLATE_ELEMENT,
        false,
        |p| parse_ts_type(p, context).or_add_diagnostic(p, expected_ts_type),
    );
    elements.complete(p, TS_TEMPLATE_ELEMENT_LIST);
    p.expect(BACKTICK);
    Present(m.complete(p, TS_TEMPLATE_LITERAL_TYPE))
}

fn is_at_ts_construct_signature_type_member(p: &mut JsParser) -> bool {
    p.at(T![new]) && (p.nth_at(1, T!['(']) || is_nth_at_ts_type_parameters(p, 1))
}

// test ts ts_constructor_type
// type A = new(a: string, b: number) => string;
// type B = abstract new(a: string, b: number) => string;
// type C = new<A, B>(a: A, b: B) => string;
// type D = abstract new<A, B>(a: A, b: B) => string;

// test_err ts ts_constructor_type_err
// type C = new<>(a: A, b: B) => string;
// type D = abstract new<>(a: A, b: B) => string;
fn parse_ts_constructor_type(p: &mut JsParser, context: TypeContext) -> ParsedSyntax {
    if !is_at_constructor_type(p) {
        return Absent;
    }

    let m = p.start();
    p.eat(T![abstract]);
    p.expect(T![new]);

    parse_ts_type_parameters(p, context).ok();
    parse_parameter_list(
        p,
        ParameterContext::Declaration,
        context,
        SignatureFlags::empty(),
    )
    .or_add_diagnostic(p, expected_parameters);
    p.expect(T![=>]);
    parse_ts_type(p, context).or_add_diagnostic(p, expected_ts_type);
    Present(m.complete(p, TS_CONSTRUCTOR_TYPE))
}

fn parse_ts_import_type_assertion(p: &mut JsParser) -> ParsedSyntax {
    if !p.at(T![assert]) && !p.at(T![with]) {
        return Absent;
    }

    let m = p.start();
    match p.cur() {
        T![assert] => {
            p.expect(T![assert]);
        }
        T![with] => {
            p.expect(T![with]);
        }
        _ => {
            m.abandon(p);
            return Absent;
        }
    };

    // bump assert or with
    p.expect(T![:]);
    p.expect(T!['{']);
    ImportAssertionList::default().parse_list(p);

    p.expect(T!['}']);

    Present(m.complete(p, TS_IMPORT_TYPE_ASSERTION))
}

fn parse_ts_import_type_assertion_block(p: &mut JsParser) -> ParsedSyntax {
    if !p.at(T!['{']) {
        return Absent;
    }

    let m = p.start();

    if p.at(T!['{']) {
        p.bump(T!['{']);
        if p.at(T!['}']) {
            p.error(
                p.err_builder(
                    "Missing import type assertion keyword 'with'",
                    p.cur_range(),
                )
                .with_detail(p.cur_range(), "'with' expected."),
            );
        }
    }

    parse_ts_import_type_assertion(p).ok();

    p.expect(T!['}']);

    Present(m.complete(p, TS_IMPORT_TYPE_ASSERTION_BLOCK))
}

fn parse_ts_import_type_arguments(p: &mut JsParser, context: TypeContext) -> ParsedSyntax {
    if !p.at(T!['(']) {
        return Absent;
    }
    let m = p.start();
    p.bump(T!('('));
    parse_ts_type(p, context).or_add_diagnostic(p, expected_ts_type);
    if p.at(T![,]) {
        if p.nth_at(1, T![')']) {
            p.error(
                p.err_builder(
                    "ts import type may not have a trailing comma",
                    p.cur_range(),
                )
                .with_detail(p.cur_range(), "Remove the trailing comma here"),
            );
        }
        p.bump(T![,]);
        parse_ts_import_type_assertion_block(p).ok();
    }

    p.expect(T![')']);

    Present(m.complete(p, TS_IMPORT_TYPE_ARGUMENTS))
}

fn is_at_constructor_type(p: &mut JsParser) -> bool {
    p.at(T![new]) || (p.at(T![abstract]) && p.nth_at(1, T![new]))
}

/// Determines if the parser's currently located at a function type. Performs a lookahead of at most a single character.
fn is_at_function_type(p: &mut JsParser) -> bool {
    if p.at(T![<]) {
        // <
        return true;
    }

    if !p.at(T!['(']) {
        return false;
    }

    p.lookahead(|p| {
        p.bump(T!['(']);

        if p.at(T![')']) || p.at(T![...]) {
            // () not a valid parenthesized type
            // (... rest parameters are only valid in function types
            return true;
        }

        if skip_parameter_start(p) {
            // We're passed the start of the parameter. Now we can verify if anything indicates that
            // this is a parameter

            if p.at_ts(token_set![T![:], T![=], T![,], T![?]]) {
                return true;
            }

            return p.at(T![')']) && p.nth_at(1, T![=>]);
        }

        false
    })
}

// test ts ts_function_type
// type A = () => string;
// type B = (a: string) => string;
// type C = (b = "test") => string;
// type D = (c, d) => string
// type E = ([a]) => string
// type F = ({a}) => string
// type G = <A, B>(a: A, b: B) => string
// type H = (a: any) => a is string;
// type I = ({ a, b }?) => string;

// test_err ts ts_function_type_err
// type G = <>(a: A, b: B) => string
fn parse_ts_function_type(p: &mut JsParser, context: TypeContext) -> ParsedSyntax {
    if !p.at(T![<]) && !p.at(T!['(']) {
        return Absent;
    }

    let m = p.start();
    parse_ts_type_parameters(p, context.and_allow_const_modifier(true)).ok();
    parse_parameter_list(
        p,
        ParameterContext::Declaration,
        context.and_allow_conditional_types(true),
        SignatureFlags::empty(),
    )
    .or_add_diagnostic(p, expected_parameters);
    p.expect(T![=>]);
    parse_ts_return_type(p, context.and_allow_conditional_types(true))
        .or_add_diagnostic(p, expected_ts_type);

    Present(m.complete(p, TS_FUNCTION_TYPE))
}

// test ts ts_return_type_asi
// interface I {
//  foo(test: string): I
//  is(): boolean;
//  bar(test: string): I
//  asserts(): boolean;
// }
fn parse_ts_return_type(p: &mut JsParser, context: TypeContext) -> ParsedSyntax {
    let is_asserts_predicate =
        p.at(T![asserts]) && (is_nth_at_identifier(p, 1) || p.nth_at(1, T![this]));
    let is_is_predicate = (is_at_identifier(p) || p.at(T![this])) && p.nth_at(1, T![is]);

    if !p.has_nth_preceding_line_break(1) && (is_asserts_predicate || is_is_predicate) {
        parse_ts_type_predicate(p, context)
    } else {
        parse_ts_type(p, context)
    }
}

// test ts ts_type_predicate
// type A = (a) => a is string;
// type B = (a) => asserts a is string;
// type C = (a) => asserts a;
// type asserts = string;
// type D = () => asserts;
fn parse_ts_type_predicate(p: &mut JsParser, context: TypeContext) -> ParsedSyntax {
    let m = p.start();
    let is_asserts = p.eat(T![asserts]);

    parse_ts_this_type(p)
        .or_else(|| parse_reference_identifier(p))
        .unwrap();

    if is_asserts && p.at(T![is]) {
        let condition = p.start();
        p.expect(T![is]);
        parse_ts_type(p, context).or_add_diagnostic(p, expected_ts_type);
        condition.complete(p, TS_ASSERTS_CONDITION);
    } else if !is_asserts {
        p.expect(T![is]);
        parse_ts_type(p, context).or_add_diagnostic(p, expected_ts_type);
    }

    let kind = if is_asserts {
        TS_ASSERTS_RETURN_TYPE
    } else {
        TS_PREDICATE_RETURN_TYPE
    };

    Present(m.complete(p, kind))
}

// test ts ts_instantiation_expressions
// let f1 = fx<string>;
// let f2 = fx<string, number>;
// let f3 = fx['test']<string>;
// const a2 = f.g<number>;  // () => number
// const a5 = f['g']<number>;  // () => number
// const a7 = (f<number>)['g'];
// const a6 = f<number>['g'];  // type Error
// const b2 = f?.<number>();
// const b3 = f<number>?.();
// const b4 = f<number>?.<number>();  // Type Error, expected no type arguments
// const x1 = f<true>
// (true);
// const x2 = f<true>
// true;
// const x3 = f<true>;
// true;
// (f<T>)<K>;
// (f<T>)<K>();
// (f<T>)<K>?.();
// (a?.f<T>)<K>();
// new (a<T>)<K>();
// f<<T>() => T>?.();
// f?.<<T>() => T>();
// f<x> ? g<y> : h<z>;
// [f<x>];
// { f<x> }

// test ts ts_type_instantiation_expression
// type StringBox = Box<string>;

// test ts ts_instantiation_expressions_1
// class A {
//  constructor() {
//    f<T> super();
//  }
// }
// f<TemplateStringsArray>``;
// f<T>(1);
// f<T> ?? 1;
// f<T> || 1;
// f<T> && 1;
// f<T> | 1;
// f<T> ^ 1;
// f<T> & 1;
// f<T> == f<T>;
// f<T> != f<T>;
// f<T> === f<T>;
// f<T> !== f<T>;
// f<T> <= f<T>;
// f<T> instanceof f<T>;
// f<T> in {};
// f<T> as {};
// f<T> satisfies {};
// f<T> * f<T>;
// f<T> / f<T>;
// f<T> % f<T>;
// f<T> ** f<T>;
// f < T > +f<T>;
// f < T > -f<T>;
// f < T > this;
// f < T > null;
// f < T > true;
// f < T > false;
// f < T > 1;
// f < T > 123n;
// f < T > [];
// f < T > {};
// f < T > function test() {};
// f < T > class A {};
// f < T > new A();
// f<T> / 1;
// f < T > +1;
// f < T > -1;
// f < T > ~1;
// f < T > !1;
// f < T > someIdentifier;
// f < T > delete a[field];
// f < T > typeof MyClass;
// f < T > void a;
// f<T> <= 1;
// f < T > (await 1);
// f < T > import.meta;
// f < T > import("123");
// a < b >> c;
// f = h >>> 0 < j >>> 0;

// test ts ts_instantiation_expressions_new_line
// class A {
//  constructor() {
//    f<T>
//      super();
//  }
// }
// function *f() {
//     const f = f<T>
//     yield;
// }
// f<T>
// ?? 1;
// f<T>
// || 1;
// f<T>
// && 1;
// f<T>
// | 1;
// f<T>
// ^ 1;
// f<T>
// & 1;
// f<T>
// == f<T>;
// f<T>
// <= f<T>;
// f<T>
// != f<T>;
// f<T>
// === f<T>;
// f<T>
// !== f<T>;
// f<T>
// instanceof f<T>;
// f<T>
// in {};
// f<T>
// * f<T>;
// f<T>
// / f<T>;
// f<T>
// % f<T>;
// f<T>
// ** f<T>;
// f <T>
// +f<T>;
// f <T>
// -f<T>;
// f <T>
// this;
// f <T>
// null;
// f <T>
// true;
// f <T>
// false;
// f <T>
// 1;
// f <T>
// 123n;
// f <T>
// {};
// f <T>
// function test() {};
// f <T>
// class A {};
// f <T>
// new A();
// f<T>
// / 1;
// f <T>
// +1;
// f <T>
// -1;
// f <T>
// ~1;
// f <T>
// !1;
// f <T>
// someIdentifier;
// f <T>
// delete a[field];
// f <T>
// typeof MyClass;
// f <T>
// void a;
// f<T>
// <= 1;
// f <T>
// (await 1);
// f <T>
// import.meta;
// f <T>
// import("123");

// test ts ts_instantiation_expressions_asi
// const x5 = f<true>
// let yy = 0;
// const x6 = f<true>
// interface I {}
// let x10 = f<true>
// this.bar()
// let x11 = f<true>
// function bar() {}
// let x12 = f<true>
// class C {}
// let x13 = f<true>
// bar()
// let x14 = f<true>
// void bar()
// class C1 {
//     static specialFoo = f<string>
//     static bar = 123
// }
// class C2 {
//     public specialFoo = f<string>
//     public bar = 123
// }
// class C3 {
//     private specialFoo = f<string>
//     private bar = 123
// }
// class C4 {
//     protected specialFoo = f<string>
//     protected bar = 123
// }
// class C5 {
//     protected specialFoo = f<string>
//     #bar = 123
// }
// const Bar = Foo<string>
// const Baz = 123

// test_err ts ts_instantiation_expressions_1
// const a8 = f<number><number>;  // Relational operator error
// const b1 = f?.<number>;  // Error, `(` expected
// f<T> << f<T>;
// f<T> = g<K>;
// f<T> >> f<T>;
// f<T> >= f<T>;
// f<T> < f<T>;
// f<T> > f<T>;
// f<T> import<1>;
// f<T> yield;
// f<T> ++;
// f<T> --;
// f<T> /= 1;
// f<T> <= f<T>;
// f <T>
// [];
// f<T>
// as {};
// f<T>
// satisfies {};
// class C5 {
//     protected specialFoo = f<string> #bar = 123
// }
// for (const a = b.test<string> in []) {}

pub(crate) fn parse_ts_type_arguments_in_expression(
    p: &mut JsParser,
    context: ExpressionContext,
) -> ParsedSyntax {
    // Don't parse type arguments in JS because the syntax is ambiguous
    // https://github.com/microsoft/TypeScript/issues/36662

    // test js type_arguments_like_expression
    // ((0)<5>(6))

    if TypeScript.is_unsupported(p) || !matches!(p.cur(), T![<] | T![<<]) {
        return Absent;
    }

    // test ts ts_type_arguments_like_expression
    // 0 < (0 >= 1);
    try_parse(p, |p| {
        p.re_lex(JsReLexContext::TypeArgumentLessThan);
        let m = p.start();
        p.bump(T![<]);

        if p.at(T![>]) {
            p.error(expected_ts_type_parameter(p, p.cur_range()));
        }
        TypeArgumentsList::new(TypeContext::default(), false).parse_list(p);
        p.re_lex(JsReLexContext::BinaryOperator);
        p.expect(T![>]);
        let arguments = m.complete(p, TS_TYPE_ARGUMENTS);

        if p.last() == Some(T![>]) && can_follow_type_arguments_in_expr(p, context) {
            Ok(Present(arguments))
        } else {
            Err(())
        }
    })
    .unwrap_or(Absent)
}

fn can_follow_type_arguments_in_expr(p: &mut JsParser, context: ExpressionContext) -> bool {
    let cur_kind = p.cur();
    match cur_kind {
        // These tokens can follow a type argument list in a call expression.
        T!['('] | BACKTICK | EOF => true,
        // A type argument list followed by `<` never makes sense, and a type argument list followed
        // by `>` is ambiguous with a (re-scanned) `>>` operator, so we disqualify both. Also, in
        // this context, `+` and `-` are unary operators, not binary operators.
        T![<] | T![>] | T![+] | T![-] => false,
        // We favor the type argument list interpretation when it is immediately followed by
        // a line break, a binary operator, or something that can't start an expression.
        _ => {
            p.has_preceding_line_break()
                || is_at_binary_operator(p, context)
                || !is_at_expression(p)
        }
    }
}

pub(crate) fn parse_ts_type_arguments(p: &mut JsParser, context: TypeContext) -> ParsedSyntax {
    // test ts ts_type_arguments_left_shift
    // type A<T> = T;
    // type B = A<<C>(c: C) => undefined>;
    let current = p.re_lex(JsReLexContext::TypeArgumentLessThan);
    if current != T![<] {
        return Absent;
    }

    Present(parse_ts_type_arguments_impl(p, context, true))
}

// test_err ts type_arguments_incomplete
// func<T,
pub(crate) fn parse_ts_type_arguments_impl(
    p: &mut JsParser,
    context: TypeContext,
    recover_on_errors: bool,
) -> CompletedMarker {
    let m = p.start();
    p.bump(T![<]);

    if p.at(T![>]) {
        p.error(expected_ts_type_parameter(p, p.cur_range()));
    }
    TypeArgumentsList::new(context, recover_on_errors).parse_list(p);
    p.expect(T![>]);
    m.complete(p, TS_TYPE_ARGUMENTS)
}

struct TypeArgumentsList {
    context: TypeContext,
    recover_on_errors: bool,
}

impl TypeArgumentsList {
    pub(crate) fn new(context: TypeContext, recover_on_errors: bool) -> Self {
        Self {
            context,
            recover_on_errors,
        }
    }
}

impl ParseSeparatedList for TypeArgumentsList {
    type Kind = JsSyntaxKind;
    type Parser<'source> = JsParser<'source>;

    const LIST_KIND: Self::Kind = TS_TYPE_ARGUMENT_LIST;

    fn parse_element(&mut self, p: &mut JsParser) -> ParsedSyntax {
        parse_ts_type(p, self.context)
    }

    fn is_at_list_end(&self, p: &mut JsParser) -> bool {
        p.at(T![>])
    }

    fn recover(&mut self, p: &mut JsParser, parsed_element: ParsedSyntax) -> RecoveryResult {
        if parsed_element.is_absent() && !self.recover_on_errors {
            // test ts type_arguments_no_recovery
            // for (let i = 0 ; i < 3; ++i) {
            //     verify.completions({
            //         marker: `${i + 1}`,
            //         exact: [
            //             { name: "foo", replacementSpan: test.ranges()[i] },
            //             { name: "bar", replacementSpan: test.ranges()[i] },
            //         ]
            //     });
            // }

            // Parse conditional expression speculatively tries to parse a list of type arguments
            // The parser shouldn't perform error recovery in that case and simply bail out of parsing
            RecoveryResult::Err(RecoveryError::AlreadyRecovered)
        } else {
            parsed_element.or_recover_with_token_set(
                p,
                &ParseRecoveryTokenSet::new(
                    TS_BOGUS_TYPE,
                    token_set![T![>], T![,], T![ident], T![yield], T![await]],
                )
                .enable_recovery_on_line_break(),
                expected_ts_type_parameter,
            )
        }
    }

    fn separating_element_kind(&mut self) -> JsSyntaxKind {
        T![,]
    }

    fn allow_trailing_separating_element(&self) -> bool {
        false
    }
}

fn parse_ts_type_member_semi(p: &mut JsParser) {
    // type members can either be separated by a comma
    if p.eat(T![,]) {
        return;
    }

    // or a semicolon (possibly ASI)
    if !optional_semi(p) {
        let err = p
            .err_builder("';' expected'", p.cur_range())
            .with_hint("An explicit or implicit semicolon is expected here...");

        p.error(err);
    }
}
