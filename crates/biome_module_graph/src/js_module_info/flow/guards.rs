use super::builder::{CalleeReturn, declaration_return};
use super::subjects::{
    AnyPatternElement, array_element_key, assigned_subject, identifier_key, literal_key,
    member_name_key, subject_of,
};
use super::{
    FlowCall, FlowCallArgument, FlowCallKind, FlowCallReceiver, FlowCallRoot, FlowSubject,
    FlowSubjectId, FlowSubjectTable, PropertyKey, subject_root,
};
use crate::js_module_info::TsBindingReferenceExt;
use biome_js_control_flow::AnyJsControlFlowRoot;
use biome_js_semantic::{Binding, JsDeclarationKind, SemanticModel};
use biome_js_syntax::{
    AnyJsArrowFunctionParameters, AnyJsAssignment, AnyJsCallArgument, AnyJsClass, AnyJsClassMember,
    AnyJsClassMemberName, AnyJsExpression, AnyJsFormalParameter, AnyJsFunction, AnyJsInProperty,
    AnyJsLiteralExpression, AnyJsMemberExpression, AnyJsName, AnyJsObjectMember,
    AnyJsOptionalChainExpression, AnyJsParameter, AnyJsSwitchClause, AnyJsTemplateElement,
    AnyJsVariableDeclaration, AnyTsEnumMemberName, AnyTsName, AnyTsReturnType, AnyTsType,
    JsBinaryOperator, JsCallExpression, JsComputedMemberName, JsDecorator, JsIdentifierExpression,
    JsLanguage, JsLogicalOperator, JsModule, JsPrivateName, JsScript, JsStaticMemberExpression,
    JsSwitchStatement, JsSyntaxKind, JsSyntaxToken, JsTemplateExpression, JsThisExpression,
    JsUnaryOperator, JsVariableDeclarator, JsVariableKind, JsVariableStatement, T,
    TsDeclareStatement, TsEnumDeclaration, TsQualifiedName, binding_ext::AnyJsBindingDeclaration,
    inner_string_text, unescape_js_identifier,
};
use biome_js_type_info::{
    Function, FunctionParameter, RawPredicateTarget, RawTypeData, RawTypeId, ReturnType,
    TypeMemberKind, TypeReference, TypeReferenceQualifier, TypeofKind, is_raw_any,
    raw_may_be_nullish, raw_predicate_target,
};
use biome_rowan::{
    AstNode, AstNodeList, AstSeparatedList, SyntaxKind, SyntaxKindSet, Text, TextRange, TokenText,
    WalkEvent,
};
use rustc_hash::FxHashMap;
use std::borrow::Cow;

/// Limits the nested `!`, `&&`, and `||` operators decomposed in one
/// condition. Deeper operands keep the incoming type.
const MAX_CONDITION_DEPTH: usize = 32;
/// Limits the syntax steps spent decomposing one condition.
const MAX_CONDITION_STEPS: usize = 16_384;
/// Limits the identifiers and member accesses scanned for subjects when
/// decomposition stops early.
const MAX_FALLBACK_NODES: usize = 1024;
/// The expressions that the scan for subjects reads when decomposition stops
/// early: those with the shape of a subject.
const FALLBACK_KINDS: SyntaxKindSet<JsLanguage> =
    JsIdentifierExpression::KIND_SET.union(AnyJsMemberExpression::KIND_SET);
/// Limits the aliased conditions inlined into one another, as TypeScript
/// does. In `const c2 = c1; ...; const c6 = c5; if (c6)`, the test inlines
/// `c6` to `c2` and tests only the truthiness of `c1`.
const MAX_INLINED_ALIASES: usize = 5;

/// The result of a condition that a flow path has observed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum FlowOutcome {
    Truthy,
    Falsy,
    Nullish,
    NonNullish,
}

/// One step of a decomposed condition, stored in an arena per execution root.
///
/// Applying a guard to a subject's incoming type yields its type once the
/// condition holds. `Keep` and tests of other subjects return the incoming
/// type, while `Incomplete` makes it unknown.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum FlowGuard {
    /// The step cannot narrow any subject, so the incoming type is kept.
    Keep,
    /// Decomposition stopped at its work limit or at missing syntax, so the
    /// incoming type becomes unknown.
    Incomplete,
    /// A runtime test of `subject`, an id in the root's subject table.
    Test {
        subject: FlowSubjectId,
        test: FlowTest,
        positive: bool,
    },
    /// A runtime test of the property `key` of `object`, which narrows
    /// `object` itself when the property is a discriminant, as a test of
    /// `s.kind` narrows `s`. The same test of the property's own subject is a
    /// separate [`Self::Test`]. `test` is never [`FlowTest::Predicate`].
    ///
    /// `object` is a [`FlowSubject::Pattern`] for a test of one of its
    /// elements, as `kind === "c"` tests `{ kind, v }` in
    /// `const { kind, v } = s`.
    Property {
        object: FlowSubjectId,
        key: PropertyKey,
        test: FlowTest,
        positive: bool,
        /// Whether the test reads the property through an optional chain, as
        /// `s?.kind` does, which gives `undefined` when `s` is nullish.
        optional_chain: bool,
        /// Whether the test checks a value that a `const` read from the
        /// property before the condition, as in
        /// `const k = s.kind; if (k === "c")` or in an aliased condition such
        /// as `const ok = s.kind === "c"; if (ok)`. A write to the property
        /// between the two can make `object` differ from what the test shows,
        /// so collection replaces such a guard with [`Self::Keep`] when the
        /// root writes the property. A guard on a pattern is never aliased,
        /// because no write changes the values it destructured.
        aliased: bool,
    },
    /// Applies the guard at this index only to a subject whose path is
    /// readonly. A path is readonly when, at every step, each member of the
    /// object's declared type other than `null` and `undefined` declares the
    /// property itself as readonly. A runtime test of a property path inside
    /// an inlined aliased condition is wrapped this way, because TypeScript
    /// applies such a test to a path only when no assignment through the
    /// path can change it between the `const` and its test.
    ///
    /// Whether the path is readonly depends on resolved types, so inference
    /// decides it at each read, and the solve applies the wrapped guard only
    /// to a readonly path.
    ReadonlyPath(usize),
    /// The operands of `&&` or `||`. A sequential pair applies `right` to the
    /// result of `left`; otherwise both apply to the incoming type and their
    /// results are joined.
    Both {
        left: usize,
        right: usize,
        sequential: bool,
    },
    /// The guard at index `guard`, on the path where no case of the `switch`
    /// statement at index `switch` in [`RootFlow::switches`] matches. Gives
    /// `never` instead when that statement is exhaustive.
    ///
    /// A `switch` is exhaustive when every value that its discriminant can
    /// have at the statement matches a case, as in TypeScript. Then no value
    /// takes this path, so it adds nothing to any subject after the
    /// statement. Whether a statement is exhaustive depends on resolved
    /// types, so inference decides it at the read, and the solve applies
    /// `guard` unless inference found the statement exhaustive for that read.
    ///
    /// [`RootFlow::switches`]: super::RootFlow::switches
    Bypass { switch: u32, guard: usize },
    /// The guard at this index, which holds where no case of a `switch` on a
    /// value or on `typeof` of one matches. An incoming type with an
    /// `unknown` member stays as it is. A member that is a type parameter, or
    /// an intersection with one, stays as it is unless the guard leaves it no
    /// value, when it is removed. The guard narrows the other members.
    ///
    /// Other tests narrow `unknown` as `{} | null | undefined`, so
    /// `x !== null` gives `{} | undefined`, and a type parameter `T` to an
    /// intersection, so `x !== null` gives `T & ({} | undefined)`. Where no
    /// case of such a `switch` matches, TypeScript instead removes the
    /// members whose every value a case covers and keeps the others whole, so
    /// `switch (x) { case null: break; default: x }` reads `unknown` or `T`
    /// there. `switch (true)` tests each case as a condition, as an `if`
    /// does, so its guards need no such wrapper.
    Unmatched(usize),
    /// The call point at index `call` in [`RootFlow::calls`], whose result
    /// is truthy when `positive` is true and falsy otherwise. A statement
    /// call's guard is always positive, because it stands for the call
    /// returning.
    ///
    /// What the call narrows depends on its callee's signature, which
    /// inference resolves in the context that reads a subject and passes to
    /// the solve: a type predicate or an assertion narrows the argument or
    /// the receiver that it names, and an `asserts value` signature applies
    /// the guard of its argument decomposed as a condition that holds. A
    /// call without such a signature keeps the incoming type. A statement
    /// call whose signature never returns gives `never` when the solve's mask
    /// of ending calls has its bit, so the path through it adds nothing to
    /// the read.
    ///
    /// [`RootFlow::calls`]: super::RootFlow::calls
    Call { call: u32, positive: bool },
}

/// A runtime test whose result a guard assumes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum FlowTest {
    Truthy,
    Nullish,
    Typeof(TypeofKind),
    /// Strict equality with `null`.
    Null,
    /// Equality, strict (`===`) or loose (`==`), with the value of the
    /// expression at `value`, whose collected type is a primitive literal
    /// type. The expression is a literal such as `"a"` or `1`, `-1`, `+1`, or
    /// a template literal without substitutions. It is written in the
    /// comparison or as the initializer of a `const` that the comparison
    /// names.
    Literal {
        value: TextRange,
        strict: bool,
    },
    /// Equality with `undefined`; loose equality also matches `null`.
    Undefined {
        strict: bool,
    },
    /// Equality, strict or loose, with the member of an enum that this module
    /// declares whose name is at `member`. The member's collected type is the
    /// literal type of its value when collection computes the value, and the
    /// test narrows nothing otherwise.
    EnumMember {
        member: TextRange,
        strict: bool,
    },
    /// Equality, strict or loose, with the value of the imported variable
    /// declared at `binding`, or, when `member` is the range of a member's
    /// name, as `A` in `E.A`, with that member of the enum that the variable
    /// imports. Inference reads that value in the module that declares the
    /// variable. When the value isn't a unit value, a variable whose
    /// declaration has an explicit type narrows by that type, as
    /// [`Self::Compared`] does, and the test narrows nothing otherwise.
    Imported {
        binding: TextRange,
        member: Option<TextRange>,
        strict: bool,
    },
    /// Equality, strict or loose, with the value of the expression at
    /// `value`, which isn't a unit value, by the value's declared type. The
    /// expression is a variable or a parameter declared with a type
    /// annotation, a property path that starts at one, or an `as const`
    /// expression, whose declared type TypeScript gives it too.
    ///
    /// TypeScript narrows by the value's narrowed type at the comparison.
    /// The declared type is never narrower, so the true branch keeps at least
    /// TypeScript's members, and the false branch narrows only when the
    /// declared type is a single unit type, whose narrowed type is that value
    /// too.
    Compared {
        value: TextRange,
        strict: bool,
    },
    /// The receiver of an optional chain whose end a [`Self::Compared`] test
    /// finds equal to the value at `value`: a test that the receiver is
    /// nullish, which applies only when the value's declared type excludes
    /// `undefined`, and, for a loose comparison, `null`. Only the branch where
    /// the receiver is not nullish applies, as in `x?.v === y` with
    /// `y: string`.
    ComparedReceiver {
        value: TextRange,
        strict: bool,
    },
    /// A call to the type-guard function declared at `function`. Its type
    /// predicate states the tested type, as in `value is string` or
    /// `asserts value is Foo`.
    Predicate {
        function: TextRange,
        /// Whether every member of the tested type is an object type, as
        /// for `Foo` or `Foo | Bar`, so the test narrows only a type that
        /// has an object type or is `unknown` or `any`.
        object_target: bool,
    },
    /// The `in` operator, as in `"a" in value`. The range is the key, a string
    /// or number literal that spells the property's name, such as `"a"` or `0`
    /// but not `"\u0061"` or `0x0`. See [`literal_key`].
    In(TextRange),
    /// The `instanceof` operator, as in `value instanceof Date`, whose right
    /// operand, the constructor, is the expression at this range.
    Instanceof(TextRange),
    /// The `in` operator with a private name, as in `#brand in value`, which
    /// holds only for an object that the class declaring the name
    /// constructed, or for that class itself when the name is static.
    PrivateName {
        /// The range at which inference finds the type of the class: the
        /// name that a class declaration binds, or a class expression.
        class: TextRange,
        is_static: bool,
    },
    /// The equality of `value.constructor` with the constructor at this
    /// range, as in `value.constructor === Date`. Its guard is only ever
    /// positive, because TypeScript narrows only where the two are equal.
    Constructor(TextRange),
}

impl FlowTest {
    /// Returns whether the test can narrow a value of type `any`, as in
    /// TypeScript: `typeof` with a primitive kind or `"undefined"`,
    /// `instanceof`, `in` with a private name, a comparison of `constructor`,
    /// and a call to a type-guard function. Truthiness, equality, `in` with a
    /// key, and `typeof` with `"object"` or `"function"` keep `any` in both
    /// branches.
    pub(crate) fn narrows_any(self) -> bool {
        match self {
            Self::Typeof(kind) => !matches!(kind, TypeofKind::Object | TypeofKind::Function),
            Self::Instanceof(_)
            | Self::PrivateName { .. }
            | Self::Constructor(_)
            | Self::Predicate { .. } => true,
            Self::Truthy
            | Self::Nullish
            | Self::Null
            | Self::Literal { .. }
            | Self::Undefined { .. }
            | Self::EnumMember { .. }
            | Self::Imported { .. }
            | Self::Compared { .. }
            | Self::ComparedReceiver { .. }
            | Self::In(_) => false,
        }
    }

    /// Returns the value whose declared type inference resolves before the
    /// test applies: the expression of a [`Self::Compared`] or
    /// [`Self::ComparedReceiver`] test, or the variable that a
    /// [`Self::Imported`] test without a member compares with, which narrows
    /// by its declared type when it holds no unit value.
    pub(crate) fn compared_value(self) -> Option<FlowComparedValue> {
        match self {
            Self::Compared { value, .. } | Self::ComparedReceiver { value, .. } => {
                Some(FlowComparedValue::Expression(value))
            }
            Self::Imported {
                binding,
                member: None,
                ..
            } => Some(FlowComparedValue::Import(binding)),
            Self::Truthy
            | Self::Nullish
            | Self::Typeof(_)
            | Self::Null
            | Self::Literal { .. }
            | Self::Undefined { .. }
            | Self::EnumMember { .. }
            | Self::Imported { .. }
            | Self::Predicate { .. }
            | Self::In(_)
            | Self::Instanceof(_)
            | Self::PrivateName { .. }
            | Self::Constructor(_) => None,
        }
    }
}

/// A value that a comparison narrows by its declared type, which inference
/// resolves in the context that reads the narrowed subject.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum FlowComparedValue {
    /// The expression at this range, typed without narrowing.
    Expression(TextRange),
    /// The variable that the import declared at this range binds, typed by
    /// the declaration that it names when that declaration has an explicit
    /// type.
    Import(TextRange),
}

impl FlowComparedValue {
    /// Returns the range that identifies the value in the solve: the range of
    /// the expression or of the import.
    pub(crate) fn range(self) -> TextRange {
        match self {
            Self::Expression(range) | Self::Import(range) => range,
        }
    }
}

/// What a guard's return annotation states about one of its parameters.
#[derive(Clone, Copy)]
pub(crate) struct GuardContract<'a> {
    pub(crate) parameter: &'a Text,
    /// The type that the tested argument has when the contract holds, or
    /// `None` for `asserts value`, which only requires a truthy argument.
    pub(crate) target: Option<&'a TypeReference>,
    /// Whether a call returns only when the contract holds. Otherwise, the
    /// call's result reports whether it holds.
    pub(crate) asserts: bool,
}

/// Returns the signature of the guard declared at `function` and the contract
/// that its return annotation states.
///
/// Returns `None` unless the guard's collected type is one signature that is
/// synchronous, is not generic, and returns a type predicate or an assertion.
pub(crate) fn guard_signature<'a>(
    raw_types: &'a [RawTypeData],
    raw_binding_types: &FxHashMap<TextRange, TypeReference>,
    function: TextRange,
) -> Option<(&'a Function, GuardContract<'a>)> {
    let TypeReference::Resolved(RawTypeId::Local(id)) = raw_binding_types.get(&function)? else {
        return None;
    };
    // An overload set is a callable object, not a single function.
    let RawTypeData::Function(signature) = raw_types.get(id.index())? else {
        return None;
    };
    if signature.is_async || !signature.type_parameters.is_empty() {
        return None;
    }
    let contract = match &signature.return_type {
        ReturnType::Predicate(predicate) => GuardContract {
            parameter: &predicate.parameter_name,
            target: Some(&predicate.ty),
            asserts: false,
        },
        ReturnType::Asserts(assertion) => GuardContract {
            parameter: &assertion.parameter_name,
            // Collection records a bare `asserts value` as `Conditional`, which
            // no type annotation names.
            target: (!matches!(
                &assertion.ty,
                TypeReference::Resolved(RawTypeId::Local(id))
                    if matches!(raw_types.get(id.index()), Some(RawTypeData::Conditional))
            ))
            .then_some(&assertion.ty),
            asserts: true,
        },
        ReturnType::Type(_) | ReturnType::InferredPredicate(_) => return None,
    };
    Some((signature, contract))
}

/// Selects the argument that a call to a local type guard tests, and records
/// the call points of the root under construction.
///
/// A type guard's return annotation states a contract about one of its
/// parameters. With a type predicate, such as `value is string`, the call's
/// result reports whether the argument has the type. With an assertion, such
/// as `asserts value is string` or `asserts value`, the call returns only when
/// the contract holds. The guard's collected signature and its declaration
/// select the tested argument. Collection never infers the guard's body.
///
/// A call of a local function narrows according to its collected signature.
/// Any other call whose callee is a dotted name is a call point (see
/// [`FlowCall`]), and inference resolves its callee.
pub(super) struct GuardFunctions<'a> {
    raw_types: &'a [RawTypeData],
    raw_binding_types: &'a FxHashMap<TextRange, TypeReference>,
    /// Maps each function binding to its guard, or to `None` when the
    /// function is not a supported guard.
    guards: FxHashMap<TextRange, Option<Guard>>,
    /// The call points of the root under construction, in the order they are
    /// first met, each with the subjects that it may narrow.
    calls: Vec<(FlowCall, Box<[FlowSubjectId]>)>,
    /// The index of each call point in `calls`, keyed by the range of its
    /// call. A call met again, in a second build of a finally block or a
    /// second decomposition of a condition, keeps its index.
    call_ids: FxHashMap<TextRange, u32>,
}

/// A supported guard function.
#[derive(Clone, Copy)]
struct Guard {
    /// The index of the tested argument.
    argument: usize,
    kind: GuardKind,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum GuardKind {
    /// `value is T`: the call's result reports whether the argument has type
    /// `T`. `object_target` tells whether every member of `T` is an object
    /// type, as for [`FlowTest::Predicate`].
    Predicate { object_target: bool },
    /// `asserts value is T`: the call returns only when the argument has type
    /// `T`.
    TypeAssertion { object_target: bool },
    /// `asserts value`: the call returns only when the argument is truthy.
    TruthyAssertion,
}

impl<'a> GuardFunctions<'a> {
    pub(super) fn new(
        raw_types: &'a [RawTypeData],
        raw_binding_types: &'a FxHashMap<TextRange, TypeReference>,
    ) -> Self {
        Self {
            raw_types,
            raw_binding_types,
            guards: FxHashMap::default(),
            calls: Vec::new(),
            call_ids: FxHashMap::default(),
        }
    }

    /// Returns the call points recorded since the last call, each with the
    /// subjects that it may narrow, and starts an empty list for the next
    /// root.
    pub(super) fn take_calls(&mut self) -> Vec<(FlowCall, Box<[FlowSubjectId]>)> {
        self.call_ids.clear();
        std::mem::take(&mut self.calls)
    }

    /// Returns the index of the call point `call`, which may narrow
    /// `subjects`, adding it on first use, or `None` when the root has used
    /// every index.
    fn add_call(&mut self, call: FlowCall, subjects: Box<[FlowSubjectId]>) -> Option<u32> {
        if let Some(id) = self.call_ids.get(&call.range) {
            return Some(*id);
        }
        let id = u32::try_from(self.calls.len()).ok()?;
        self.call_ids.insert(call.range, id);
        self.calls.push((call, subjects));
        Some(id)
    }

    /// Returns whether the variable declared at `binding` has the type `any`
    /// written in its annotation, as the raw tables show without resolving
    /// anything. An alias of `any` and an import don't count.
    fn declares_any(&self, binding: TextRange) -> bool {
        self.raw_binding_types
            .get(&binding)
            .is_some_and(|reference| is_raw_any(reference, self.raw_types))
    }

    /// Returns whether `call`, a statement call, can establish a condition
    /// when it returns, judged from syntax alone: it calls a local assertion
    /// function, or its callee passes TypeScript's explicit-type rule, as
    /// [`statement_callee`] checks. A root that only statement calls select
    /// needs flow only when one of them can.
    pub(super) fn keeps_statement_call(
        &mut self,
        call: &JsCallExpression,
        model: &SemanticModel,
    ) -> bool {
        let mut remaining = MAX_CONDITION_STEPS;
        if self.calls_local_function(call, model) {
            return self
                .guard_call(call, model, &mut remaining)
                .is_some_and(|(guard, ..)| !matches!(guard.kind, GuardKind::Predicate { .. }));
        }
        call.callee()
            .ok()
            .and_then(|callee| omit_parentheses(callee, &mut remaining))
            .is_some_and(|callee| statement_callee(&callee, model, &mut remaining).is_some())
    }

    /// Returns whether `call`, a statement call of a local function, never
    /// returns, judging from the function's declaration.
    ///
    /// A function declaration or a `declare function` never returns when its
    /// return type annotation is `never`, written in place or through type
    /// aliases that the module declares, and its collected type is one
    /// signature that is not generic (see [`Self::local_return`]).
    /// TypeScript ends the path only when the `never` comes from an
    /// annotation. A local assertion function with `asserts value` never
    /// returns when its tested argument is always false (see
    /// [`is_false_expression`]). Inference decides a call of any other
    /// callee as a call point.
    pub(super) fn local_call_ends(
        &mut self,
        call: &JsCallExpression,
        model: &SemanticModel,
    ) -> bool {
        if !self.calls_local_function(call, model) || call.type_arguments().is_some() {
            return false;
        }
        let mut remaining = MAX_CONDITION_STEPS;
        if let Some((guard, _, argument)) = self.guard_call(call, model, &mut remaining) {
            return guard.kind == GuardKind::TruthyAssertion
                && is_false_expression(&argument, &mut remaining);
        }
        self.local_return(call, model) == AnnotatedReturn::Never
    }

    /// Returns what the return type annotation of the local function that
    /// `call` calls tells about whether the call returns, as
    /// [`annotated_return`] reads it, when the function is a function
    /// declaration or a `declare function` whose collected type is one
    /// signature that is not generic. Returns [`AnnotatedReturn::Other`] for
    /// any other callee.
    fn local_return(&self, call: &JsCallExpression, model: &SemanticModel) -> AnnotatedReturn {
        let Some(AnyJsExpression::JsIdentifierExpression(callee)) =
            call.callee().ok().map(AnyJsExpression::omit_parentheses)
        else {
            return AnnotatedReturn::Other;
        };
        let Some(function) = callee.name().ok().and_then(|name| model.binding(&name)) else {
            return AnnotatedReturn::Other;
        };
        let annotation = match function.tree().declaration() {
            Some(AnyJsBindingDeclaration::JsFunctionDeclaration(declaration)) => {
                declaration.return_type_annotation()
            }
            Some(AnyJsBindingDeclaration::TsDeclareFunctionDeclaration(declaration)) => {
                declaration.return_type_annotation()
            }
            _ => return AnnotatedReturn::Other,
        };
        // An overload set is a callable object, not a single function, and
        // TypeScript resolves which of its signatures a call takes.
        let is_single_signature = matches!(
            self.raw_binding_types.get(&function.range()),
            Some(TypeReference::Resolved(RawTypeId::Local(id)))
                if matches!(
                    self.raw_types.get(id.index()),
                    Some(RawTypeData::Function(signature))
                        if signature.type_parameters.is_empty()
                )
        );
        match annotation.and_then(|annotation| annotation.ty().ok()) {
            Some(AnyTsReturnType::AnyTsType(ty)) if is_single_signature => {
                annotated_return(ty, model)
            }
            _ => AnnotatedReturn::Other,
        }
    }

    /// Returns whether `call` calls a function whose collected signature
    /// decides whether the call narrows: a function declaration or a
    /// `declare function` of the module, or a variable without a type
    /// annotation that an arrow or function expression initializes.
    /// [`Self::guard`] reads that signature. Such a call is never a call
    /// point, because a call point would resolve the same signature, except
    /// in a condition when the signature leaves the type predicate to
    /// inference (see [`Self::calls_inferred_predicate`]), and in a statement
    /// when its return type annotation names a type that only inference can
    /// resolve (see [`AnnotatedReturn::Undecided`]).
    ///
    /// A function whose collected type needs the call's arguments to select
    /// its signature, as [`Self::selects_signature_by_arguments`] checks, is
    /// no such function, so its calls are call points.
    fn calls_local_function(&self, call: &JsCallExpression, model: &SemanticModel) -> bool {
        let Some(AnyJsExpression::JsIdentifierExpression(callee)) =
            call.callee().ok().map(AnyJsExpression::omit_parentheses)
        else {
            return false;
        };
        let Some(function) = callee.name().ok().and_then(|name| model.binding(&name)) else {
            return false;
        };
        if function.is_imported() || self.selects_signature_by_arguments(function.range()) {
            return false;
        }
        match function.tree().declaration() {
            Some(
                AnyJsBindingDeclaration::JsFunctionDeclaration(_)
                | AnyJsBindingDeclaration::TsDeclareFunctionDeclaration(_),
            ) => true,
            Some(AnyJsBindingDeclaration::JsVariableDeclarator(declarator)) => {
                declarator.variable_annotation().is_none()
                    && matches!(
                        declarator
                            .initializer()
                            .and_then(|initializer| initializer.expression().ok()),
                        Some(
                            AnyJsExpression::JsArrowFunctionExpression(_)
                                | AnyJsExpression::JsFunctionExpression(_)
                        )
                    )
            }
            _ => false,
        }
    }

    /// Returns whether the collected type of the local function declared at
    /// `function` is generic or has several overloads, and some signature
    /// returns a type predicate or an assertion, or has the return type
    /// annotation `never`. TypeScript then selects the signature of a call,
    /// and infers its type arguments, from the types of the arguments, so
    /// only inference can tell what the call does.
    fn selects_signature_by_arguments(&self, function: TextRange) -> bool {
        let Some(TypeReference::Resolved(RawTypeId::Local(id))) =
            self.raw_binding_types.get(&function)
        else {
            return false;
        };
        match self.raw_types.get(id.index()) {
            Some(RawTypeData::Function(signature)) => {
                !signature.type_parameters.is_empty() && self.has_effect_return(signature)
            }
            Some(RawTypeData::Object(object)) => object.members.iter().any(|member| {
                member.kind == TypeMemberKind::CallSignature
                    && matches!(
                        &member.ty,
                        TypeReference::Resolved(RawTypeId::Local(id))
                            if matches!(
                                self.raw_types.get(id.index()),
                                Some(RawTypeData::Function(signature))
                                    if self.has_effect_return(signature)
                            )
                    )
            }),
            _ => false,
        }
    }

    /// Returns whether `signature` returns a type predicate or an assertion,
    /// or has the return type annotation `never`, the signatures that
    /// TypeScript considers when it decides what a call does to the flow.
    ///
    /// A type predicate that TypeScript may infer from the function's body
    /// doesn't count here: a call of such a function in a condition is a
    /// call point (see [`Self::calls_inferred_predicate`]), and a statement
    /// call of it narrows nothing, because the function never asserts.
    fn has_effect_return(&self, signature: &Function) -> bool {
        match &signature.return_type {
            ReturnType::Predicate(_) | ReturnType::Asserts(_) => true,
            ReturnType::Type(TypeReference::Resolved(RawTypeId::Local(id))) => matches!(
                self.raw_types.get(id.index()),
                Some(RawTypeData::NeverKeyword)
            ),
            ReturnType::Type(_) | ReturnType::InferredPredicate(_) => false,
        }
    }

    /// Returns whether `call` calls a local function, as
    /// [`Self::calls_local_function`] finds it, whose collected signature
    /// leaves to inference whether the function has a type predicate that
    /// TypeScript infers from its body ([`ReturnType::InferredPredicate`]).
    /// Collection can't decide what such a call does, so a call of it in a
    /// condition is a call point.
    fn calls_inferred_predicate(&self, call: &JsCallExpression, model: &SemanticModel) -> bool {
        let Some(AnyJsExpression::JsIdentifierExpression(callee)) =
            call.callee().ok().map(AnyJsExpression::omit_parentheses)
        else {
            return false;
        };
        let Some(function) = callee.name().ok().and_then(|name| model.binding(&name)) else {
            return false;
        };
        matches!(
            self.raw_binding_types.get(&function.range()),
            Some(TypeReference::Resolved(RawTypeId::Local(id)))
                if matches!(
                    self.raw_types.get(id.index()),
                    Some(RawTypeData::Function(signature))
                        if matches!(signature.return_type, ReturnType::InferredPredicate(_))
                )
        )
    }

    /// Returns whether `object`, the object of a `?.`, may be `null` or
    /// `undefined` as far as the collected types tell.
    ///
    /// Returns false for `this` and for a variable whose collected type can't
    /// be nullish, such as `o` in `const o = { a: 1 }; o?.f(o)`. A test that
    /// such an object is not nullish narrows nothing. Any other expression,
    /// and a variable without a collected type, may be nullish.
    /// [`raw_may_be_nullish`] lists the types that can't be nullish.
    pub(super) fn may_be_nullish(&self, object: &AnyJsExpression, model: &SemanticModel) -> bool {
        match object {
            AnyJsExpression::JsThisExpression(_) => false,
            AnyJsExpression::JsIdentifierExpression(identifier) => identifier
                .name()
                .ok()
                .and_then(|name| model.binding(&name))
                .and_then(|binding| self.raw_binding_types.get(&binding.range()))
                .is_none_or(|reference| raw_may_be_nullish(reference, self.raw_types)),
            _ => true,
        }
    }

    /// Returns the argument that `call` tests, without parentheses, and the
    /// [`FlowTest::Predicate`] that the call applies to it.
    ///
    /// Returns `None` unless `call` calls a supported type predicate.
    fn tested_argument(
        &mut self,
        call: &JsCallExpression,
        model: &SemanticModel,
        remaining: &mut usize,
    ) -> Option<(AnyJsExpression, FlowTest)> {
        let (guard, function, argument) = self.guard_call(call, model, remaining)?;
        let GuardKind::Predicate { object_target } = guard.kind else {
            return None;
        };
        Some((
            argument,
            FlowTest::Predicate {
                function,
                object_target,
            },
        ))
    }

    /// Returns the guard that `call` calls, the range of its declaration, and
    /// the tested argument without parentheses.
    ///
    /// Returns `None` unless `call` calls a supported guard directly and passes
    /// the tested argument. A spread argument before or at the tested one is
    /// unsupported, because it can pass a different value to the tested
    /// parameter.
    fn guard_call(
        &mut self,
        call: &JsCallExpression,
        model: &SemanticModel,
        remaining: &mut usize,
    ) -> Option<(Guard, TextRange, AnyJsExpression)> {
        if call.is_optional_chain() || call.type_arguments().is_some() {
            return None;
        }
        let AnyJsExpression::JsIdentifierExpression(callee) =
            omit_parentheses(call.callee().ok()?, remaining)?
        else {
            return None;
        };
        let function = model.binding(&callee.name().ok()?)?;
        let range = function.range();
        let guard = match self.guards.get(&range) {
            Some(guard) => *guard,
            None => {
                let guard = self.guard(&function, model);
                self.guards.insert(range, guard);
                guard
            }
        }?;
        let mut tested = None;
        for (position, argument) in call.arguments().ok()?.args().iter().enumerate() {
            let AnyJsCallArgument::AnyJsExpression(argument) = argument.ok()? else {
                return None;
            };
            if position == guard.argument {
                tested = Some(argument);
                break;
            }
        }
        Some((guard, range, omit_parentheses(tested?, remaining)?))
    }

    /// Returns the guard that `function` declares.
    ///
    /// Returns `None` unless `function` is a supported guard. A supported guard
    /// is a local function declaration, or an arrow or function expression that
    /// initializes a variable and returns a type predicate, not an assertion.
    /// Nothing reassigns it, and it is not a generator.
    /// Its tested parameter is named and is not a rest parameter. That
    /// parameter has no default value, because a default replaces an
    /// `undefined` argument. A target of `any` or `unknown`, which narrows
    /// nothing, is not supported.
    fn guard(&self, function: &Binding, model: &SemanticModel) -> Option<Guard> {
        if function.is_imported()
            || !matches!(
                function.declaration_kind(),
                JsDeclarationKind::Function | JsDeclarationKind::Value
            )
        {
            return None;
        }
        // Most callees are not guards, and the raw signature rules them out
        // before the write check visits every reference.
        let (signature, contract) =
            guard_signature(self.raw_types, self.raw_binding_types, function.range())?;
        if function.all_writes().next().is_some() {
            return None;
        }
        let kind = match contract.target {
            Some(target) => {
                let object_target = self.is_object_target(target, model)?;
                if contract.asserts {
                    GuardKind::TypeAssertion { object_target }
                } else {
                    GuardKind::Predicate { object_target }
                }
            }
            None => GuardKind::TruthyAssertion,
        };
        let tested = parameter_name(contract.parameter)?;
        // A `this` parameter types the receiver, so it takes no argument.
        let receiver = usize::from(matches!(
            signature.parameters.first(),
            Some(FunctionParameter::Named(parameter)) if parameter.name.text() == "this"
        ));
        let mut matching = signature
            .parameters
            .iter()
            .enumerate()
            .skip(receiver)
            .filter(|(_, parameter)| match parameter {
                FunctionParameter::Named(parameter) => {
                    parameter_name(&parameter.name).is_some_and(|name| name == tested)
                }
                FunctionParameter::Pattern(_) => false,
            });
        let (index, parameter) = matching.next()?;
        if matching.next().is_some()
            || matches!(parameter, FunctionParameter::Named(parameter) if parameter.is_rest)
        {
            return None;
        }
        let guard = Guard {
            argument: index - receiver,
            kind,
        };

        let parameters = match function.tree().declaration()? {
            AnyJsBindingDeclaration::JsFunctionDeclaration(declaration) => {
                if declaration.star_token().is_some() {
                    return None;
                }
                declaration.parameters().ok()?
            }
            AnyJsBindingDeclaration::TsDeclareFunctionDeclaration(declaration) => {
                declaration.parameters().ok()?
            }
            AnyJsBindingDeclaration::JsVariableDeclarator(declarator) => {
                // TypeScript ignores an assertion whose callee is a variable
                // without a type annotation (TS2775). An annotated variable
                // never gets here, because `guard_signature` does not accept
                // its collected type.
                if contract.asserts {
                    return None;
                }
                match declarator.initializer()?.expression().ok()? {
                    AnyJsExpression::JsArrowFunctionExpression(arrow) => {
                        match arrow.parameters().ok()? {
                            // A parameter without parentheses has no default value.
                            AnyJsArrowFunctionParameters::AnyJsBinding(_) => return Some(guard),
                            AnyJsArrowFunctionParameters::JsParameters(parameters) => parameters,
                        }
                    }
                    AnyJsExpression::JsFunctionExpression(expression) => {
                        if expression.star_token().is_some() {
                            return None;
                        }
                        expression.parameters().ok()?
                    }
                    _ => return None,
                }
            }
            _ => return None,
        };
        // The collected signature comes from these parameters, so `index` also
        // counts a `this` parameter here.
        let AnyJsParameter::AnyJsFormalParameter(AnyJsFormalParameter::JsFormalParameter(
            parameter,
        )) = parameters.items().iter().nth(index)?.ok()?
        else {
            return None;
        };
        parameter.initializer().is_none().then_some(guard)
    }

    /// Returns whether every member of `target`, the raw target of a type
    /// predicate, is an object type, as far as raw tables and the scopes that
    /// declare its names tell. Returns `None` when `target` is `any` or
    /// `unknown`, or has such a member, and `Some(false)` when `target` has
    /// more type references than [`MAX_TARGET_STEPS`] allows visiting.
    ///
    /// A local type alias is followed to the type it names, and a local
    /// interface or class is an object type. An imported name isn't taken as
    /// an object type, because it can name an alias of a primitive in another
    /// module. Other local names, such as enums and type parameters, aren't
    /// either. A name that no scope declares, such as a global type, and a
    /// name qualified by a namespace are taken as object types. The result
    /// decides only which declared types the test applies to, never what the
    /// test gives. So when such a name is an alias of a primitive, as the
    /// global `PropertyKey` is, the only effect is that a variable without an
    /// object type keeps its declared type.
    fn is_object_target(&self, target: &TypeReference, model: &SemanticModel) -> Option<bool> {
        let mut pending = vec![target];
        let mut is_object = true;
        for _ in 0..MAX_TARGET_STEPS {
            let Some(target) = pending.pop() else {
                return Some(is_object);
            };
            let qualifier = match raw_predicate_target(target, self.raw_types) {
                RawPredicateTarget::Object => continue,
                RawPredicateTarget::Other => {
                    is_object = false;
                    continue;
                }
                RawPredicateTarget::Top => return None,
                RawPredicateTarget::Union(members) => {
                    pending.extend(members);
                    continue;
                }
                RawPredicateTarget::Named(qualifier) => qualifier,
            };
            let Some(binding) = named_binding(qualifier, model) else {
                continue;
            };
            if binding.is_imported() {
                is_object = false;
                continue;
            }
            match binding.tree().declaration() {
                Some(AnyJsBindingDeclaration::TsTypeAliasDeclaration(_)) => {
                    match self.raw_binding_types.get(&binding.range()) {
                        Some(aliased) => pending.push(aliased),
                        None => is_object = false,
                    }
                }
                Some(
                    AnyJsBindingDeclaration::TsInterfaceDeclaration(_)
                    | AnyJsBindingDeclaration::JsClassDeclaration(_)
                    | AnyJsBindingDeclaration::JsClassExportDefaultDeclaration(_),
                ) => {}
                _ => is_object = false,
            }
        }
        Some(false)
    }
}

/// Limits the type references, including the chain of type aliases, visited
/// in the target of a type predicate.
const MAX_TARGET_STEPS: usize = 16;

/// Returns the binding that `qualifier` names, or `None` for a name qualified
/// by a namespace and for a name that no enclosing scope declares.
fn named_binding(qualifier: &TypeReferenceQualifier, model: &SemanticModel) -> Option<Binding> {
    let mut path = qualifier.path.iter();
    let name = path.next()?;
    if path.next().is_some() {
        return None;
    }
    let mut scope = model.scope_from_id(qualifier.scope_id);
    loop {
        if let Some(binding) = scope
            .get_binding_reference(name.text())
            .and_then(|reference| reference.get_binding_id_for_qualifier(qualifier))
            .and_then(|id| model.binding_by_id(id))
        {
            return Some(binding);
        }
        scope = scope.parent()?;
    }
}

/// Decomposes the condition that a statement call `call` establishes when it
/// returns, or returns `None` when it narrows nothing.
///
/// A statement call is the expression of an expression statement, or an
/// operand of the comma operator anywhere, without parentheses around it.
/// These are the calls that TypeScript adds to the flow. For example,
/// `(assert(x));` is none, while `(assert(x), 0);` and
/// `return (assert(x), x);` have one.
///
/// A call of a local assertion function applies its contract here, as
/// [`GuardFunctions::guard`] reads it. `asserts value is T` tests an argument
/// that is a subject against the guard's target, and `asserts value`
/// decomposes the argument as a condition that holds, so
/// `assert(value !== null)` excludes `null` from `value`. A call of another
/// local function narrows nothing, and is a call point only when its return
/// type annotation names a type that only inference can resolve, which may
/// be `never` (see [`AnnotatedReturn::Undecided`]). Any other call whose
/// callee passes TypeScript's explicit-type rule, as [`statement_callee`]
/// checks, is a call point. Its condition mentions every subject that it may
/// narrow.
pub(super) fn decompose_statement_call(
    call: &JsCallExpression,
    model: &SemanticModel,
    functions: &mut GuardFunctions,
    subjects: &mut FlowSubjectTable,
    guards: &mut Vec<FlowGuard>,
) -> Option<DecomposedCondition> {
    if !functions.calls_local_function(call, model) {
        return decompose_call_point(call, model, functions, subjects, guards);
    }
    let mut remaining = MAX_CONDITION_STEPS;
    let Some((guard, function, argument)) = functions.guard_call(call, model, &mut remaining)
    else {
        if functions.local_return(call, model) == AnnotatedReturn::Undecided {
            return decompose_call_point(call, model, functions, subjects, guards);
        }
        return None;
    };
    match guard.kind {
        GuardKind::Predicate { .. } => None,
        GuardKind::TruthyAssertion => Some(decompose_condition(
            &argument,
            FlowOutcome::Truthy,
            model,
            functions,
            subjects,
            guards,
        )),
        GuardKind::TypeAssertion { object_target } => {
            chain_receivers(&argument, &mut remaining)?;
            let subject = subject_of(&argument, model, subjects)?;
            let any_root = AnyRoot::new(subjects, functions, subject);
            if any_root == AnyRoot::Path {
                return None;
            }
            guards.push(FlowGuard::Test {
                subject,
                test: FlowTest::Predicate {
                    function,
                    object_target,
                },
                positive: true,
            });
            // As for `instanceof`, a target whose members are all object
            // types narrows only a type with an object type, `unknown`, or
            // `any`, and a variable whose annotation is `any` is an ordinary
            // subject.
            let (mentions, parents) = if object_target && any_root == AnyRoot::None {
                (Vec::new(), vec![subject])
            } else {
                (vec![subject], Vec::new())
            };
            Some(DecomposedCondition {
                guard: guards.len() - 1,
                mentions,
                instance_tested: parents.clone(),
                parents,
                call_mentions: Vec::new(),
            })
        }
    }
}

/// Decomposes `call`, a statement call, as a call point, whose effect
/// inference resolves (see [`Decomposer::statement_call_point`]).
fn decompose_call_point(
    call: &JsCallExpression,
    model: &SemanticModel,
    functions: &mut GuardFunctions,
    subjects: &mut FlowSubjectTable,
    guards: &mut Vec<FlowGuard>,
) -> Option<DecomposedCondition> {
    let mut decomposer = Decomposer::new(model, functions, subjects, guards);
    let guard = decomposer.statement_call_point(call)?;
    Some(decomposer.finish(guard, std::iter::empty()))
}

/// Limits the parentheses and type aliases that [`annotated_return`] follows
/// in one return type annotation.
const MAX_RETURN_ALIAS_STEPS: usize = 16;

/// What the return type annotation of a function tells, from the syntax of
/// one module, about whether a call of the function returns.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum AnnotatedReturn {
    /// The annotation is `never`, written in place or through a chain of
    /// type aliases that the module declares, so a call never returns.
    Never,
    /// The annotation names an import, or a member of one such as
    /// `types.Never`, in place or at the end of a chain of the module's type
    /// aliases. Only inference can resolve it.
    Undecided,
    /// Any other annotation. A call returns as far as the syntax tells, even
    /// where TypeScript resolves the annotation to `never`, as it does for
    /// `G<string>` with `type G<T> = never`, a union of `never` types, a
    /// member of a namespace that the module declares, or a type alias that
    /// a declaration file declares globally. A name that no scope declares
    /// gets no call point, because inference resolves it only to a type of
    /// the lib or to an interface that merges into one, never to `never`.
    Other,
}

/// Reads `ty`, the return type annotation of a function, as
/// [`AnnotatedReturn`] describes, following parentheses and the type aliases
/// without type parameters that the module declares. A chain longer than
/// [`MAX_RETURN_ALIAS_STEPS`], such as one of aliases that name each other,
/// is [`AnnotatedReturn::Other`].
fn annotated_return(mut ty: AnyTsType, model: &SemanticModel) -> AnnotatedReturn {
    for _ in 0..MAX_RETURN_ALIAS_STEPS {
        let reference = match ty {
            AnyTsType::TsNeverType(_) => return AnnotatedReturn::Never,
            AnyTsType::TsParenthesizedType(parenthesized) => match parenthesized.ty() {
                Ok(inner) => {
                    ty = inner;
                    continue;
                }
                Err(_) => return AnnotatedReturn::Other,
            },
            AnyTsType::TsReferenceType(reference) if reference.type_arguments().is_none() => {
                reference
            }
            _ => return AnnotatedReturn::Other,
        };
        let name = match reference.name() {
            Ok(AnyTsName::JsReferenceIdentifier(name)) => name,
            Ok(AnyTsName::TsQualifiedName(qualified)) => {
                return if qualifies_import(qualified, model) {
                    AnnotatedReturn::Undecided
                } else {
                    AnnotatedReturn::Other
                };
            }
            Err(_) => return AnnotatedReturn::Other,
        };
        let Some(binding) = model.binding(&name) else {
            return AnnotatedReturn::Other;
        };
        if binding.is_imported() {
            return AnnotatedReturn::Undecided;
        }
        ty = match binding.tree().declaration() {
            Some(AnyJsBindingDeclaration::TsTypeAliasDeclaration(alias))
                if alias.type_parameters().is_none() =>
            {
                match alias.ty() {
                    Ok(aliased) => aliased,
                    Err(_) => return AnnotatedReturn::Other,
                }
            }
            _ => return AnnotatedReturn::Other,
        };
    }
    AnnotatedReturn::Other
}

/// Returns whether the name that `qualified`, a qualified type name such as
/// `types.Never`, starts at is an import.
fn qualifies_import(qualified: TsQualifiedName, model: &SemanticModel) -> bool {
    let mut left = qualified.left();
    for _ in 0..MAX_RETURN_ALIAS_STEPS {
        match left {
            Ok(AnyTsName::JsReferenceIdentifier(name)) => {
                return model
                    .binding(&name)
                    .is_some_and(|binding| binding.is_imported());
            }
            Ok(AnyTsName::TsQualifiedName(qualified)) => left = qualified.left(),
            Err(_) => return false,
        }
    }
    false
}

/// Returns the decoded parameter `name`.
///
/// Returns `None` for `this`, the receiver, and for a name with a malformed
/// escape.
fn parameter_name(name: &Text) -> Option<Cow<'_, str>> {
    let name = unescape_js_identifier(name.text());
    (name != "this" && !name.contains('\\')).then_some(name)
}

/// A condition decomposed into the arena.
#[derive(Debug)]
pub(super) struct DecomposedCondition {
    pub(super) guard: usize,
    /// Subjects whose type the condition may narrow.
    ///
    /// When decomposition stops early, this conservatively lists every
    /// variable and property path read directly in the condition, except a
    /// variable whose annotation is `any` and the paths on one. It also
    /// lists every variable read directly in an inlined initializer when the
    /// variable cannot change between the alias and its test. See
    /// [`narrows_through_aliases`] for the variables that qualify.
    pub(super) mentions: Vec<FlowSubjectId>,
    /// Subjects whose type the condition may narrow only through these tests,
    /// unless `mentions` also lists them:
    ///
    /// - [`FlowGuard::Property`], such as `s` in `s.kind === "c"`.
    /// - [`FlowTest::In`], such as `x` in `"a" in x`, and
    ///   [`FlowTest::PrivateName`], such as `x` in `#p in x`.
    /// - [`FlowTest::Instanceof`], such as `x` in `x instanceof Date`, and
    ///   [`FlowTest::Constructor`], such as `x` in `x.constructor === Date`.
    /// - A [`FlowTest::Predicate`] whose target has only object types, such as
    ///   `x` in `isFoo(x)`.
    ///
    /// The first narrows only a union with two object types, and the others
    /// narrow only a type with an object type, `unknown`, or `any`. A
    /// variable whose annotation is `any` is never listed here (see
    /// [`AnyRoot`]). When decomposition stops early, this also lists the
    /// pattern of every variable in `mentions` that is an element of a
    /// destructuring pattern that a test of the variable narrows.
    pub(super) parents: Vec<FlowSubjectId>,
    /// The subjects in `parents` that a test other than one of a property
    /// checks.
    pub(super) instance_tested: Vec<FlowSubjectId>,
    /// Subjects that call points may narrow. A subject that `mentions` or
    /// `parents` also lists counts as mentioned by them instead. Whether a
    /// call point narrows depends on its callee's signature, which only
    /// inference resolves. So the reads that only these subjects' call
    /// points reach keep their declared types without a solve when no call
    /// point narrows.
    pub(super) call_mentions: Vec<FlowSubjectId>,
}

impl DecomposedCondition {
    /// Returns the condition that holds when this condition holds and then
    /// `next` holds, as `&&` combines its operands. Its guard applies `next`
    /// to the result of this guard.
    pub(super) fn then(mut self, next: Self, guards: &mut Vec<FlowGuard>) -> Self {
        guards.push(FlowGuard::Both {
            left: self.guard,
            right: next.guard,
            sequential: true,
        });
        self.guard = guards.len() - 1;
        self.mentions.extend(next.mentions);
        self.parents.extend(next.parents);
        self.instance_tested.extend(next.instance_tested);
        self.call_mentions.extend(next.call_mentions);
        self
    }
}

/// Decomposes `expression`, assuming `outcome`, into guards appended to
/// `guards`. The tests in those guards refer to subjects by their ids in
/// `subjects`.
///
/// A truthiness test of a `const` without a type annotation, such as `ok` in
/// `const ok = x !== null; if (ok) x`, is an aliased condition: it also
/// applies the condition that initializes the `const`, as TypeScript does,
/// to the subjects that [`narrows_through_aliases`] accepts, and to the
/// property paths that [`narrows_through_aliases_when_readonly`] accepts when
/// they are readonly. Aliases inline into one another up to
/// [`MAX_INLINED_ALIASES`] levels.
pub(super) fn decompose_condition(
    expression: &AnyJsExpression,
    outcome: FlowOutcome,
    model: &SemanticModel,
    functions: &mut GuardFunctions,
    subjects: &mut FlowSubjectTable,
    guards: &mut Vec<FlowGuard>,
) -> DecomposedCondition {
    let mut decomposer = Decomposer::new(model, functions, subjects, guards);
    let guard = decomposer.guard(expression.clone(), outcome, 0);
    decomposer.finish(guard, [expression])
}

/// Decomposes the test that a logical assignment applies to its target
/// `target`, assuming `outcome`, into guards appended to `guards`. The test
/// matches TypeScript's: `x ??= y` evaluates `y` where `x` is nullish,
/// `x ||= y` where `x` is falsy, and `x &&= y` where `x` is truthy.
///
/// The guard tests the subject that the expression with the same spelling as
/// the target reads. For a named property, it also tests the object that
/// holds the property, so `s.kind ||= "c"` narrows `s` as `s.kind || "c"`
/// does. A target is never part of an optional chain, so the guard tests no
/// receiver. A target that is no subject, such as `o[k]`, gives
/// [`FlowGuard::Keep`].
pub(super) fn decompose_assignment_target(
    target: &AnyJsAssignment,
    outcome: FlowOutcome,
    model: &SemanticModel,
    functions: &mut GuardFunctions,
    subjects: &mut FlowSubjectTable,
    guards: &mut Vec<FlowGuard>,
) -> DecomposedCondition {
    let mut decomposer = Decomposer::new(model, functions, subjects, guards);
    let (test, positive) = match outcome {
        FlowOutcome::Truthy => (FlowTest::Truthy, true),
        FlowOutcome::Falsy => (FlowTest::Truthy, false),
        FlowOutcome::Nullish => (FlowTest::Nullish, true),
        FlowOutcome::NonNullish => (FlowTest::Nullish, false),
    };
    let guard = match assigned_subject(target, model, decomposer.subjects) {
        Some(subject) => decomposer.target_test(subject, test, positive),
        None => FlowGuard::Keep,
    };
    let guard = decomposer.push(guard);
    decomposer.finish(guard, std::iter::empty())
}

/// How the cases of a `switch` statement narrow, which depends on its
/// discriminant, as in TypeScript.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum SwitchKind {
    /// The discriminant has the shape of a subject, or is a call in an
    /// optional chain, as in `switch (x)`, `switch (s.kind)`, or
    /// `switch (o?.f())`. A case narrows as `discriminant === test` does.
    Value,
    /// The discriminant is `typeof` of such an operand, and every case test
    /// is a string literal or a template literal without a tag and without
    /// substitutions, as in `switch (typeof x) { case "string": }` or
    /// `` case `string`: ``. A case narrows as `typeof operand === test`
    /// narrows the operand and the receivers of its optional chain, but not
    /// the object that holds it or the pattern that declares it, as in
    /// TypeScript. A single case of any other kind, such as `case k:`, makes
    /// the statement [`Self::Opaque`], as in TypeScript.
    Typeof,
    /// The discriminant is the literal `true` itself, without parentheses. A
    /// case narrows as a condition does when it is truthy.
    True,
    /// Any other discriminant, such as `x!`, `(true)`, or `f()`, whose cases
    /// narrow nothing.
    Opaque,
}

impl SwitchKind {
    /// Returns how the cases of `statement` narrow.
    pub(super) fn new(statement: &JsSwitchStatement) -> Self {
        let Ok(discriminant) = statement.discriminant() else {
            return Self::Opaque;
        };
        // TypeScript checks for a `true` discriminant without skipping
        // parentheses, so `(true)` narrows nothing.
        if let AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsBooleanLiteralExpression(literal),
        ) = &discriminant
        {
            return if literal
                .value_token()
                .is_ok_and(|token| token.kind() == T![true])
            {
                Self::True
            } else {
                Self::Opaque
            };
        }
        let mut remaining = MAX_CONDITION_STEPS;
        let Some(discriminant) = omit_parentheses(discriminant, &mut remaining) else {
            return Self::Opaque;
        };
        if let Some(operand) = typeof_operand(&discriminant, &mut remaining) {
            let only_strings = statement.cases().iter().all(|clause| match clause {
                AnyJsSwitchClause::JsCaseClause(clause) => match clause.test() {
                    Ok(AnyJsExpression::AnyJsLiteralExpression(
                        AnyJsLiteralExpression::JsStringLiteralExpression(_),
                    )) => true,
                    Ok(AnyJsExpression::JsTemplateExpression(template)) => {
                        is_plain_template(&template)
                    }
                    _ => false,
                },
                AnyJsSwitchClause::JsDefaultClause(_) => true,
            });
            return if only_strings && has_operand_shape(&operand) == Some(true) {
                Self::Typeof
            } else {
                Self::Opaque
            };
        }
        if has_operand_shape(&discriminant) == Some(true) {
            Self::Value
        } else {
            Self::Opaque
        }
    }
}

/// Returns the operand of `expression`, without parentheses, when
/// `expression` is a `typeof` expression.
fn typeof_operand(expression: &AnyJsExpression, remaining: &mut usize) -> Option<AnyJsExpression> {
    let AnyJsExpression::JsUnaryExpression(unary) = expression else {
        return None;
    };
    if unary.operator().ok()? != JsUnaryOperator::Typeof {
        return None;
    }
    omit_parentheses(unary.argument().ok()?, remaining)
}

/// Returns the expression whose type decides whether a `switch` statement
/// of `kind` with the discriminant `discriminant` is exhaustive, without
/// parentheses: the discriminant of a [`SwitchKind::Value`] statement, or the
/// operand of a [`SwitchKind::Typeof`] one.
///
/// Returns `None` for other kinds, and for a `typeof` discriminant in
/// parentheses. TypeScript checks such a discriminant as a string value
/// instead, so `switch ((typeof x))` is exhaustive only with a case for every
/// kind.
pub(super) fn exhaustive_operand(
    discriminant: &AnyJsExpression,
    kind: SwitchKind,
) -> Option<AnyJsExpression> {
    let mut remaining = MAX_CONDITION_STEPS;
    match kind {
        SwitchKind::Value => omit_parentheses(discriminant.clone(), &mut remaining),
        SwitchKind::Typeof => typeof_operand(discriminant, &mut remaining),
        SwitchKind::True | SwitchKind::Opaque => None,
    }
}

/// Returns whether TypeScript gives the case test `test` of a `switch`
/// statement of `kind` a unit type that a case of Biome also tests.
/// TypeScript finds a statement exhaustive only when every case has a unit
/// type. For [`SwitchKind::Value`], the test must be a unit value, such as
/// `"a"`, `-1`, `null`, `undefined`, or a `const` that holds one. For
/// [`SwitchKind::Typeof`], it must be a string literal or a template literal
/// without substitutions that names a kind of value, as `"string"` does.
///
/// Returns true for an imported variable and for a member of an enum because
/// only inference can read their values, so inference finds the statement
/// exhaustive only when they are unit values. A local variable named
/// `undefined` counts only when it holds a unit value, as a variable of any
/// other name does.
pub(super) fn is_unit_case(
    discriminant: &AnyJsExpression,
    test: &AnyJsExpression,
    kind: SwitchKind,
    model: &SemanticModel,
) -> bool {
    if !matches!(kind, SwitchKind::Value | SwitchKind::Typeof) {
        return false;
    }
    let mut remaining = MAX_CONDITION_STEPS;
    let (Some(discriminant), Some(test)) = (
        omit_parentheses(discriminant.clone(), &mut remaining),
        omit_parentheses(test.clone(), &mut remaining),
    ) else {
        return false;
    };
    matches!(
        comparison_step(&discriminant, &test, true, true, model, &mut remaining),
        Some(ConditionStep::Guard {
            guard: SyntaxGuard::Null
                | SyntaxGuard::Literal { .. }
                | SyntaxGuard::Undefined { .. }
                | SyntaxGuard::EnumMember { .. }
                | SyntaxGuard::Imported { .. }
                | SyntaxGuard::Typeof(_),
            ..
        })
    )
}

/// One case of a `switch` statement, assumed to match or not.
#[derive(Clone, Copy)]
pub(super) struct SwitchCase<'a> {
    pub(super) discriminant: &'a AnyJsExpression,
    pub(super) test: &'a AnyJsExpression,
    /// How the cases of the statement narrow.
    pub(super) kind: SwitchKind,
    /// Whether the case is assumed to match.
    pub(super) matched: bool,
}

/// Decomposes the test of `case` into guards appended to `guards`, assuming
/// that the case matched or, when `case.matched` is false, that it did not.
///
/// A case of [`SwitchKind::Value`] and [`SwitchKind::Typeof`] statements
/// narrows as `discriminant === test` and `typeof operand === test` do, and
/// fails to match as `!==` does. As in TypeScript, a case that doesn't match
/// narrows no receiver of an optional chain, so `default` after
/// `case undefined:` keeps `undefined` in `o` for `switch (o?.k)`. A case of
/// [`SwitchKind::True`] narrows as its test does when it is truthy. It fails
/// to match as the test does when it is falsy only when every truthy value
/// of the test is `true` ([`is_boolean_valued`]). Otherwise a case that
/// fails to match narrows nothing, because a case such as `case x:` also
/// fails when `x` is an object, which is truthy. TypeScript takes the test
/// as falsy anyway.
///
/// Returns `None` when the case narrows nothing: for a
/// [`SwitchKind::Opaque`] statement, a case that fails to match and isn't
/// boolean, and a case test that isn't a unit value, such as a call or a
/// variable with a type annotation. TypeScript narrows by both of those.
pub(super) fn decompose_switch_case(
    case: SwitchCase,
    model: &SemanticModel,
    functions: &mut GuardFunctions,
    subjects: &mut FlowSubjectTable,
    guards: &mut Vec<FlowGuard>,
) -> Option<DecomposedCondition> {
    let SwitchCase {
        discriminant,
        test,
        kind,
        matched,
    } = case;
    let outcome = if matched {
        FlowOutcome::Truthy
    } else {
        FlowOutcome::Falsy
    };
    match kind {
        SwitchKind::Opaque => None,
        SwitchKind::True => {
            let mut remaining = MAX_CONDITION_STEPS;
            (matched || is_boolean_valued(test, &mut remaining))
                .then(|| decompose_condition(test, outcome, model, functions, subjects, guards))
        }
        SwitchKind::Value | SwitchKind::Typeof => {
            let mut decomposer = Decomposer::new(model, functions, subjects, guards);
            decomposer.shape_guards = kind == SwitchKind::Value;
            let step = omit_parentheses(discriminant.clone(), &mut decomposer.remaining)
                .zip(omit_parentheses(test.clone(), &mut decomposer.remaining))
                .and_then(|(discriminant, test)| {
                    comparison_step(
                        &discriminant,
                        &test,
                        true,
                        matched,
                        model,
                        &mut decomposer.remaining,
                    )
                });
            let guard = match step {
                Some(ConditionStep::Unsupported) => return None,
                Some(ConditionStep::Guard {
                    operand,
                    guard,
                    positive,
                    receivers,
                }) => decomposer.step_guard(
                    ConditionStep::Guard {
                        operand,
                        guard,
                        positive,
                        receivers: receivers && matched,
                    },
                    outcome,
                    0,
                ),
                Some(step) => decomposer.step_guard(step, outcome, 0),
                None => {
                    decomposer.complete = false;
                    decomposer.push(FlowGuard::Incomplete)
                }
            };
            Some(decomposer.finish(guard, [discriminant, test]))
        }
    }
}

/// Returns whether every value of `expression` that is truthy is `true`, so
/// that a case of `switch (true)` whose test is `expression` fails to match
/// only when the test is falsy. See [`is_boolean_when`].
fn is_boolean_valued(expression: &AnyJsExpression, remaining: &mut usize) -> bool {
    is_boolean_when(expression, true, remaining)
}

/// Returns whether every value of `expression` that is truthy, when `truthy`
/// is true, or falsy otherwise, is the boolean `truthy`.
///
/// This holds for comparisons, `instanceof`, `in`, `!`, calls, and these in
/// parentheses, which give booleans. A call counts because only a call to a
/// type-guard function narrows, and such a function returns a boolean.
/// `a && b` gives `a` when `a` is falsy and `b` otherwise, so its truthy
/// values are those of `b` and its falsy values those of both. `a || b`
/// gives `a` when `a` is truthy and `b` otherwise, the other way around.
#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "Other expressions may give a value other than a boolean."
)]
fn is_boolean_when(expression: &AnyJsExpression, truthy: bool, remaining: &mut usize) -> bool {
    let Some(left) = remaining.checked_sub(1) else {
        return false;
    };
    *remaining = left;
    let Some(expression) = omit_parentheses(expression.clone(), remaining) else {
        return false;
    };
    match expression {
        AnyJsExpression::JsBinaryExpression(binary) => binary.operator().is_ok_and(|operator| {
            matches!(
                operator,
                JsBinaryOperator::LessThan
                    | JsBinaryOperator::GreaterThan
                    | JsBinaryOperator::LessThanOrEqual
                    | JsBinaryOperator::GreaterThanOrEqual
                    | JsBinaryOperator::Equality
                    | JsBinaryOperator::StrictEquality
                    | JsBinaryOperator::Inequality
                    | JsBinaryOperator::StrictInequality
            )
        }),
        AnyJsExpression::JsInstanceofExpression(_)
        | AnyJsExpression::JsInExpression(_)
        | AnyJsExpression::JsCallExpression(_) => true,
        AnyJsExpression::JsUnaryExpression(unary) => {
            unary.operator().ok() == Some(JsUnaryOperator::LogicalNot)
        }
        AnyJsExpression::JsLogicalExpression(logical) => {
            // Whether the left operand also gives such values: the falsy
            // ones of `&&` and the truthy ones of `||`.
            let both = match logical.operator() {
                Ok(JsLogicalOperator::LogicalAnd) => !truthy,
                Ok(JsLogicalOperator::LogicalOr) => truthy,
                Ok(JsLogicalOperator::NullishCoalescing) | Err(_) => return false,
            };
            logical
                .right()
                .is_ok_and(|right| is_boolean_when(&right, truthy, remaining))
                && (!both
                    || logical
                        .left()
                        .is_ok_and(|left| is_boolean_when(&left, truthy, remaining)))
        }
        _ => false,
    }
}

/// Decomposes the condition that the body of a `for...in` loop assumes about
/// the loop's expression `expression` into guards appended to `guards`. The
/// body assumes it only when the loop's head declares a variable, such as
/// `const k`, `let k`, or `var k`.
///
/// As in TypeScript, the condition tests that the expression is not nullish,
/// and that no receiver of its optional chain is nullish either, so
/// `for (const k in o?.v)` narrows both `o?.v` and `o`. The condition adds no
/// property or pattern guard, so `for (const k in s.kind)` narrows `s.kind`
/// but not `s`, which matches TypeScript. Parentheses are skipped, but `o!`
/// tests nothing, where TypeScript narrows `o`.
pub(super) fn decompose_for_in(
    expression: &AnyJsExpression,
    model: &SemanticModel,
    functions: &mut GuardFunctions,
    subjects: &mut FlowSubjectTable,
    guards: &mut Vec<FlowGuard>,
) -> DecomposedCondition {
    let mut decomposer = Decomposer::new(model, functions, subjects, guards);
    decomposer.shape_guards = false;
    let step =
        omit_parentheses(expression.clone(), &mut decomposer.remaining).and_then(|operand| {
            Some(if has_operand_shape(&operand)? {
                ConditionStep::Guard {
                    operand,
                    guard: SyntaxGuard::Nullish,
                    positive: false,
                    receivers: true,
                }
            } else {
                ConditionStep::Unsupported
            })
        });
    let guard = match step {
        Some(step) => decomposer.step_guard(step, FlowOutcome::NonNullish, 0),
        None => {
            decomposer.complete = false;
            decomposer.push(FlowGuard::Incomplete)
        }
    };
    decomposer.finish(guard, [expression])
}

struct Decomposer<'a, 'raw> {
    model: &'a SemanticModel,
    functions: &'a mut GuardFunctions<'raw>,
    subjects: &'a mut FlowSubjectTable,
    guards: &'a mut Vec<FlowGuard>,
    remaining: usize,
    mentions: Vec<FlowSubjectId>,
    /// The objects of the [`FlowGuard::Property`] guards and the subjects of
    /// the tests of the shape of a value decomposed so far: `in`,
    /// `instanceof`, a comparison of `constructor`, and object
    /// [`FlowTest::Predicate`] tests.
    parents: Vec<FlowSubjectId>,
    /// The subjects in `parents` of the tests other than
    /// [`FlowGuard::Property`] decomposed so far.
    instance_tested: Vec<FlowSubjectId>,
    /// The subjects that the call points decomposed so far may narrow.
    call_mentions: Vec<FlowSubjectId>,
    complete: bool,
    /// The number of aliased conditions whose initializers enclose the
    /// expression being decomposed.
    aliases: usize,
    /// Every initializer that an aliased condition has inlined.
    initializers: Vec<AnyJsExpression>,
    /// Whether a runtime test of a property or of an element of a pattern
    /// also narrows the object or the pattern. False for the cases of
    /// `switch (typeof x)`, which narrow only the operand of `typeof`, and for
    /// the expression of a `for...in` loop. See [`Self::property_guard`] and
    /// [`Self::pattern_guard`].
    shape_guards: bool,
}

impl<'a, 'raw> Decomposer<'a, 'raw> {
    fn new(
        model: &'a SemanticModel,
        functions: &'a mut GuardFunctions<'raw>,
        subjects: &'a mut FlowSubjectTable,
        guards: &'a mut Vec<FlowGuard>,
    ) -> Self {
        Self {
            model,
            functions,
            subjects,
            guards,
            remaining: MAX_CONDITION_STEPS,
            mentions: Vec::new(),
            parents: Vec::new(),
            instance_tested: Vec::new(),
            call_mentions: Vec::new(),
            complete: true,
            aliases: 0,
            initializers: Vec::new(),
            shape_guards: true,
        }
    }

    /// Returns the condition whose guard is `guard`, with the subjects that
    /// decomposition found.
    ///
    /// When decomposition stopped early, the condition mentions every
    /// variable and property path that `scanned` read directly instead,
    /// except a variable whose annotation is `any` and the paths on one (see
    /// [`AnyRoot`]). It also mentions the variables that inlined initializers
    /// read when an aliased condition can narrow them
    /// ([`narrows_through_aliases`]).
    fn finish<'e>(
        self,
        guard: usize,
        scanned: impl IntoIterator<Item = &'e AnyJsExpression>,
    ) -> DecomposedCondition {
        let Self {
            model,
            functions,
            subjects,
            mut parents,
            instance_tested,
            call_mentions,
            complete,
            mentions,
            initializers,
            ..
        } = self;
        if complete {
            return DecomposedCondition {
                guard,
                mentions,
                parents,
                instance_tested,
                call_mentions,
            };
        }
        let direct = scanned
            .into_iter()
            .flat_map(|expression| expression.syntax().preorder_of_kinds(FALLBACK_KINDS))
            .map(|event| (event, false));
        let inlined = initializers
            .iter()
            .flat_map(|initializer| initializer.syntax().preorder_of_kinds(FALLBACK_KINDS))
            .map(|event| (event, true));
        let entered = direct
            .chain(inlined)
            .filter_map(|(event, inlined)| match event {
                WalkEvent::Enter(node) => Some((node, inlined)),
                WalkEvent::Leave(_) => None,
            });
        let mut mentions = Vec::new();
        for (node, inlined) in entered.take(MAX_FALLBACK_NODES) {
            let Some(expression) = AnyJsExpression::cast(node) else {
                continue;
            };
            let Some(subject) = subject_of(&expression, model, subjects) else {
                continue;
            };
            if inlined && !narrows_through_aliases(subjects, subject, model) {
                continue;
            }
            // An `any` variable keeps its type, rather than becoming
            // unknown, when the tests that narrow it can't be told apart.
            if AnyRoot::new(subjects, functions, subject) != AnyRoot::None {
                continue;
            }
            mentions.push(subject);
            // A test of an element of a pattern also narrows the pattern.
            if let Some(&FlowSubject::Binding(range)) = subjects.as_slice().get(subject as usize)
                && let Some((pattern, _)) = subjects.pattern(range, model)
            {
                parents.push(pattern);
            }
        }
        DecomposedCondition {
            guard,
            mentions,
            parents,
            instance_tested,
            call_mentions: Vec::new(),
        }
    }

    fn guard(&mut self, expression: AnyJsExpression, outcome: FlowOutcome, depth: usize) -> usize {
        if depth >= MAX_CONDITION_DEPTH {
            self.complete = false;
            return self.push(FlowGuard::Keep);
        }
        let Some(step) = condition_step(expression, outcome, self.model, &mut self.remaining)
        else {
            self.complete = false;
            return self.push(FlowGuard::Incomplete);
        };
        self.step_guard(step, outcome, depth)
    }

    /// Returns the index of the guard of `step`, one step of a condition at
    /// `depth` assumed to have `outcome`, with its operands decomposed.
    fn step_guard(&mut self, step: ConditionStep, outcome: FlowOutcome, depth: usize) -> usize {
        let guard = match step {
            ConditionStep::Unsupported => FlowGuard::Keep,
            ConditionStep::Guard {
                operand,
                guard,
                positive,
                receivers,
            } => {
                let initializer = if matches!(guard, SyntaxGuard::Truthy) {
                    self.aliased_condition(&operand)
                } else {
                    None
                };
                let test = self.test(&operand, guard, positive, receivers);
                match initializer {
                    Some(initializer) => {
                        let left = self.push(test);
                        self.aliases += 1;
                        let right = self.guard(initializer, outcome, depth + 1);
                        self.aliases -= 1;
                        FlowGuard::Both {
                            left,
                            right,
                            sequential: true,
                        }
                    }
                    None => test,
                }
            }
            ConditionStep::Call { call, positive } => {
                let guard =
                    match self
                        .functions
                        .tested_argument(&call, self.model, &mut self.remaining)
                    {
                        Some((subject, test)) => {
                            self.test(&subject, SyntaxGuard::Predicate(test), positive, false)
                        }
                        None => self.condition_call_point(&call, positive),
                    };
                // A truthy result shows that an optional call ran.
                if positive
                    && let Some(receivers) = chain_receivers(&call.into(), &mut self.remaining)
                {
                    self.receiver_tests(receivers, guard)
                } else {
                    guard
                }
            }
            ConditionStep::Negated { argument, outcome } => {
                return self.guard(argument, outcome, depth + 1);
            }
            ConditionStep::Logical {
                left,
                right,
                outcome,
                sequential,
            } => {
                let left = self.guard(left, outcome, depth + 1);
                let right = self.guard(right, outcome, depth + 1);
                FlowGuard::Both {
                    left,
                    right,
                    sequential,
                }
            }
            ConditionStep::Boolean {
                operand,
                outcome: assumed,
                assumption,
                test,
            } => {
                let Some(test) = test else {
                    return if assumption.holds(&operand, &mut self.remaining) {
                        self.guard(operand, assumed, depth + 1)
                    } else {
                        self.push(FlowGuard::Keep)
                    };
                };
                let left = self.step_guard(*test, outcome, depth);
                let Some(initializer) = self
                    .alias_initializer(&operand)
                    .filter(|initializer| assumption.holds(initializer, &mut self.remaining))
                else {
                    return left;
                };
                self.initializers.push(initializer.clone());
                self.aliases += 1;
                let right = self.guard(initializer, assumed, depth + 1);
                self.aliases -= 1;
                FlowGuard::Both {
                    left,
                    right,
                    sequential: true,
                }
            }
            ConditionStep::Both { left, right } => {
                // A subject compared with itself, as in `x === x`, learns
                // nothing.
                if let (
                    ConditionStep::Guard { operand: first, .. },
                    ConditionStep::Guard {
                        operand: second, ..
                    },
                ) = (left.as_ref(), right.as_ref())
                    && subject_of(first, self.model, self.subjects).is_some_and(|subject| {
                        subject_of(second, self.model, self.subjects) == Some(subject)
                    })
                {
                    return self.push(FlowGuard::Keep);
                }
                let left = self.step_guard(*left, outcome, depth);
                let right = self.step_guard(*right, outcome, depth);
                FlowGuard::Both {
                    left,
                    right,
                    sequential: true,
                }
            }
        };
        self.push(guard)
    }

    /// Returns the guard for `guard` applied to `operand`: a runtime test of
    /// `operand` when it is a subject. When `receivers` holds, the guard
    /// first tests that each receiver of the optional chain around `operand`
    /// is not nullish, which a comparison by a declared type tests only when
    /// that type excludes `undefined` ([`FlowTest::ComparedReceiver`]).
    ///
    /// Returns [`FlowGuard::Keep`] when the chain around `operand` starts at a
    /// parenthesized optional chain, as in `(o?.v)?.w`, and when finding its
    /// receivers runs out of work or meets missing syntax. A test that can't
    /// narrow a variable whose annotation is `any`, or a property path on
    /// one, mentions nothing (see [`AnyRoot`]).
    fn test(
        &mut self,
        operand: &AnyJsExpression,
        guard: SyntaxGuard,
        positive: bool,
        receivers: bool,
    ) -> FlowGuard {
        // The receivers of a chain whose end equals a value typed by its
        // declaration are not nullish only when that type excludes
        // `undefined`, which inference decides.
        let receiver_test = match guard {
            SyntaxGuard::Compared { value, strict } => FlowTest::ComparedReceiver { value, strict },
            _ => FlowTest::Nullish,
        };
        let test = match guard {
            SyntaxGuard::Truthy => Some(FlowTest::Truthy),
            SyntaxGuard::Nullish => Some(FlowTest::Nullish),
            SyntaxGuard::Typeof(kind) => Some(FlowTest::Typeof(kind)),
            SyntaxGuard::Null => Some(FlowTest::Null),
            SyntaxGuard::Literal { value, strict } => Some(FlowTest::Literal { value, strict }),
            SyntaxGuard::Undefined { strict } => Some(FlowTest::Undefined { strict }),
            SyntaxGuard::EnumMember { member, strict } => {
                Some(FlowTest::EnumMember { member, strict })
            }
            SyntaxGuard::Compared { value, strict } => Some(FlowTest::Compared { value, strict }),
            SyntaxGuard::Imported {
                binding,
                member,
                strict,
            } => Some(FlowTest::Imported {
                binding,
                member,
                strict,
            }),
            SyntaxGuard::Predicate(test) => Some(test),
            SyntaxGuard::In(key) => Some(FlowTest::In(key)),
            SyntaxGuard::Instanceof(constructor) => Some(FlowTest::Instanceof(constructor)),
            SyntaxGuard::PrivateName { class, is_static } => {
                Some(FlowTest::PrivateName { class, is_static })
            }
            SyntaxGuard::Constructor(constructor) => Some(FlowTest::Constructor(constructor)),
        };
        let Some(chain) = chain_receivers(operand, &mut self.remaining) else {
            return FlowGuard::Keep;
        };
        let tested =
            test.and_then(|test| Some((test, subject_of(operand, self.model, self.subjects)?)));
        // A test that can't narrow `any` still applies to a variable declared
        // `any`, because an earlier test can narrow it to a type that this one
        // narrows further, as `x === "a"` narrows `x` after
        // `typeof x === "string"`, and a `switch` on `typeof x` needs all its
        // cases. It mentions nothing, so it makes no read a candidate.
        let narrows = tested.is_none_or(|(test, subject)| match self.any_root(subject) {
            AnyRoot::None => true,
            AnyRoot::Variable => test.narrows_any(),
            AnyRoot::Path => false,
        });
        let mut guard = match tested {
            Some((test, subject)) => match self.testable(subject) {
                Testable::Never => FlowGuard::Keep,
                testable => {
                    // `in`, `instanceof`, a comparison of `constructor`, and a
                    // type-guard function whose target has only object types
                    // narrow only a type with an object type, `unknown`, or
                    // `any`. A variable declared `any` is an ordinary
                    // subject, because its annotation already shows that
                    // these tests narrow it.
                    match test {
                        _ if !narrows => {}
                        FlowTest::In(_)
                        | FlowTest::Instanceof(_)
                        | FlowTest::PrivateName { .. }
                        | FlowTest::Constructor(_)
                        | FlowTest::Predicate {
                            object_target: true,
                            ..
                        } if self.any_root(subject) == AnyRoot::None => {
                            self.parents.push(subject);
                            self.instance_tested.push(subject);
                        }
                        _ => self.mentions.push(subject),
                    }
                    let guard = FlowGuard::Test {
                        subject,
                        test,
                        positive,
                    };
                    self.applied_when(guard, testable)
                }
            },
            None => FlowGuard::Keep,
        };
        if let Some((test, subject)) = tested
            && self.shape_guards
            && narrows
        {
            let property = self.property_guard(Some(operand), subject, test, positive);
            let pattern = self.pattern_guard(subject, test, positive);
            for property in [property, pattern].into_iter().flatten() {
                let left = self.push(guard);
                let right = self.push(property);
                guard = FlowGuard::Both {
                    left,
                    right,
                    sequential: true,
                };
            }
        }
        if receivers {
            self.receiver_tests_by(chain, guard, receiver_test)
        } else {
            guard
        }
    }

    /// Returns the guard of `call`, a call whose result a condition tests,
    /// where the result is truthy when `positive` is true.
    ///
    /// The guard is [`FlowGuard::Call`] when the callee is a dotted name that
    /// is no local function, as [`GuardFunctions::calls_local_function`]
    /// checks, or a local function that may have a type predicate inferred
    /// from its body ([`GuardFunctions::calls_inferred_predicate`]), and the
    /// call passes a subject as an argument or as the object of its callee,
    /// as TypeScript requires. Otherwise the guard is
    /// [`FlowGuard::Keep`]. So is the guard of a falsy optional call, which
    /// TypeScript doesn't narrow by, and of a call that starts at a
    /// parenthesized optional chain, which narrows nothing for any test (see
    /// [`chain_receivers`]).
    ///
    /// Inside an inlined aliased condition, only subjects that
    /// [`narrows_through_aliases`] accepts count.
    fn condition_call_point(&mut self, call: &JsCallExpression, positive: bool) -> FlowGuard {
        if (!positive && call.is_optional_chain())
            || (self.functions.calls_local_function(call, self.model)
                && !self.functions.calls_inferred_predicate(call, self.model))
            || chain_receivers(&call.clone().into(), &mut self.remaining).is_none()
        {
            return FlowGuard::Keep;
        }
        let Some(callee) = call
            .callee()
            .ok()
            .and_then(|callee| omit_parentheses(callee, &mut self.remaining))
        else {
            return FlowGuard::Keep;
        };
        if !is_dotted_name(&callee, &mut self.remaining) {
            return FlowGuard::Keep;
        }
        let Some((arguments, receiver)) = self.call_operands(call, &callee, false) else {
            return FlowGuard::Keep;
        };
        let narrowed: Box<[_]> = arguments
            .iter()
            .filter_map(|argument| argument.subject)
            .chain(receiver.map(|receiver| receiver.subject))
            .collect();
        if narrowed.is_empty() {
            return FlowGuard::Keep;
        }
        let call = FlowCall {
            range: call.range(),
            kind: FlowCallKind::Condition,
            arguments,
            receiver,
        };
        self.call_mentions.extend(narrowed.iter().copied());
        match self.functions.add_call(call, narrowed) {
            Some(call) => FlowGuard::Call { call, positive },
            None => FlowGuard::Keep,
        }
    }

    /// Returns the index of the guard of `call`, a statement call, when it is
    /// a call point, after decomposing each of its arguments as a condition
    /// that holds.
    ///
    /// The call mentions every subject that it may narrow: its subject
    /// arguments, the subject of the object of its callee, and the subjects
    /// that the decomposed arguments mention. A call that mentions none is a
    /// call point too, because its signature can end the path. Returns
    /// `None` when its callee fails TypeScript's explicit-type rule (see
    /// [`statement_callee`]), and when an argument can't be decomposed
    /// completely.
    fn statement_call_point(&mut self, call: &JsCallExpression) -> Option<usize> {
        let callee = omit_parentheses(call.callee().ok()?, &mut self.remaining)?;
        let (root, steps, annotated) = statement_callee(&callee, self.model, &mut self.remaining)?;
        let (arguments, receiver) = self.call_operands(call, &callee, true)?;
        if !self.complete {
            return None;
        }
        let mut narrowed: Vec<_> = std::mem::take(&mut self.mentions);
        narrowed.append(&mut self.parents);
        narrowed.append(&mut self.call_mentions);
        narrowed.extend(
            arguments
                .iter()
                .filter_map(|argument| argument.subject)
                .chain(receiver.map(|receiver| receiver.subject)),
        );
        narrowed.sort_unstable();
        narrowed.dedup();
        self.instance_tested.clear();
        // A call that narrows no subject can still end the path.
        let call = FlowCall {
            range: call.range(),
            kind: FlowCallKind::Statement {
                root,
                steps,
                annotated,
            },
            arguments,
            receiver,
        };
        let call = self.functions.add_call(call, narrowed.clone().into())?;
        self.call_mentions = narrowed;
        Some(self.push(FlowGuard::Call {
            call,
            positive: true,
        }))
    }

    /// Returns the arguments of `call`, up to its first spread argument, and
    /// the object of `callee`, its callee without parentheses, when it is a
    /// subject, as [`FlowCall`] records them. For a statement call
    /// (`statement` is true), each argument is also decomposed as a condition
    /// that holds.
    ///
    /// Returns `None` when an argument is missing.
    fn call_operands(
        &mut self,
        call: &JsCallExpression,
        callee: &AnyJsExpression,
        statement: bool,
    ) -> Option<(Box<[FlowCallArgument]>, Option<FlowCallReceiver>)> {
        let mut arguments = Vec::new();
        for argument in call.arguments().ok()?.args().iter() {
            let AnyJsCallArgument::AnyJsExpression(argument) = argument.ok()? else {
                break;
            };
            let range = argument.clone().omit_parentheses().range();
            let subject = self.narrowed_operand(&argument);
            let is_false = statement && is_false_expression(&argument, &mut self.remaining);
            let condition = statement.then(|| self.guard(argument, FlowOutcome::Truthy, 0));
            arguments.push(FlowCallArgument {
                range,
                subject,
                condition,
                is_false,
            });
        }
        let receiver = match callee {
            AnyJsExpression::JsStaticMemberExpression(member) => {
                let object = member.object().ok()?;
                self.narrowed_operand(&object)
                    .map(|subject| FlowCallReceiver {
                        subject,
                        range: object.omit_parentheses().range(),
                    })
            }
            _ => None,
        };
        Some((arguments.into_boxed_slice(), receiver))
    }

    /// Returns the subject of `operand`, an argument or the object of a
    /// callee, when a call point may narrow it. Inside an inlined aliased
    /// condition, only subjects that [`narrows_through_aliases`] accepts
    /// count. A subject read through a chain that starts at a parenthesized
    /// optional chain narrows nothing, as for any other test (see
    /// [`chain_receivers`]).
    fn narrowed_operand(&mut self, operand: &AnyJsExpression) -> Option<FlowSubjectId> {
        chain_receivers(operand, &mut self.remaining)?;
        let subject = subject_of(operand, self.model, self.subjects)?;
        (self.testable(subject) == Testable::Always && self.any_root(subject) != AnyRoot::Path)
            .then_some(subject)
    }

    /// Returns the guard for `test` applied to `subject`, the target of a
    /// logical assignment. When the target is a named property of an object
    /// that a test may narrow, the guard also applies the test to that object,
    /// as `s.kind ||= "c"` applies it to `s`. [`Self::property_guard`] builds
    /// the guard on the object.
    fn target_test(&mut self, subject: FlowSubjectId, test: FlowTest, positive: bool) -> FlowGuard {
        if self.any_root(subject) != AnyRoot::None && !test.narrows_any() {
            return FlowGuard::Keep;
        }
        self.mentions.push(subject);
        let guard = FlowGuard::Test {
            subject,
            test,
            positive,
        };
        match self.property_guard(None, subject, test, positive) {
            Some(property) => {
                let left = self.push(guard);
                let right = self.push(property);
                FlowGuard::Both {
                    left,
                    right,
                    sequential: true,
                }
            }
            None => guard,
        }
    }

    /// Returns the guard that a runtime test of `operand`, whose subject is
    /// `subject`, applies to the object that holds the tested property, as
    /// `s.kind === "c"` applies to `s`. The property is `subject` itself when
    /// it is a named property, or the property whose value a variable holds,
    /// as [`Self::held_property`] describes, so `k === "c"` also applies to
    /// `s` after `const k = s.kind`. The object must be a subject other than
    /// `this` that a test may narrow. Inside an inlined aliased condition, a
    /// guard on a property path is wrapped in [`FlowGuard::ReadonlyPath`].
    ///
    /// `operand` is `None` for the target of a logical assignment, such as
    /// `s.kind` in `s.kind ||= "c"`. Such a target never reads its property
    /// through an optional chain. A variable target holds no property, because
    /// only a `const` holds one.
    ///
    /// Returns `None` for a call to a type-guard function, for `in`, for
    /// `instanceof`, and for a comparison of `constructor`, because
    /// TypeScript never narrows `s` through `isC(s.kind)`, `"a" in s.kind`,
    /// `s.kind instanceof C`, or `s.kind.constructor === C`. Only the object
    /// that holds the property narrows: `o.a.kind` narrows `o.a`, not `o`.
    fn property_guard(
        &mut self,
        operand: Option<&AnyJsExpression>,
        subject: FlowSubjectId,
        test: FlowTest,
        positive: bool,
    ) -> Option<FlowGuard> {
        if matches!(
            test,
            FlowTest::Predicate { .. }
                | FlowTest::In(_)
                | FlowTest::Instanceof(_)
                | FlowTest::PrivateName { .. }
                | FlowTest::Constructor(_)
        ) {
            return None;
        }
        let (object, key, optional_chain, aliased) =
            match self.subjects.as_slice().get(subject as usize)? {
                FlowSubject::Member {
                    object,
                    key: key @ PropertyKey::Name(_),
                } => (
                    *object,
                    key.clone(),
                    operand.is_some_and(is_optional_chain),
                    self.aliases > 0,
                ),
                FlowSubject::Binding(_) => {
                    let (object, key, optional_chain) = self.held_property(operand?)?;
                    (object, key, optional_chain, true)
                }
                FlowSubject::This | FlowSubject::Member { .. } | FlowSubject::Pattern(_) => {
                    return None;
                }
            };
        if matches!(
            self.subjects.as_slice().get(object as usize)?,
            FlowSubject::This
        ) || self.any_root(object) != AnyRoot::None
        {
            return None;
        }
        let testable = self.testable(object);
        if testable == Testable::Never {
            return None;
        }
        self.parents.push(object);
        let guard = FlowGuard::Property {
            object,
            key,
            test,
            positive,
            optional_chain,
            aliased,
        };
        Some(self.applied_when(guard, testable))
    }

    /// Returns the guard that a runtime test of the variable `subject`
    /// applies to the destructuring pattern that holds the variable as an
    /// element, as `kind === "c"` applies to `{ kind, v }` in
    /// `const { kind, v } = s`.
    ///
    /// Returns `None` for a call to a type-guard function, for `in`, for
    /// `instanceof`, and for a comparison of `constructor`, because TypeScript
    /// never narrows a pattern through these tests of one of its elements.
    /// Inside an inlined aliased condition, every such pattern can be tested,
    /// because its values don't change after the destructuring.
    ///
    /// [`subjects::narrowing_pattern`]: super::subjects::narrowing_pattern
    fn pattern_guard(
        &mut self,
        subject: FlowSubjectId,
        test: FlowTest,
        positive: bool,
    ) -> Option<FlowGuard> {
        if matches!(
            test,
            FlowTest::Predicate { .. }
                | FlowTest::In(_)
                | FlowTest::Instanceof(_)
                | FlowTest::PrivateName { .. }
                | FlowTest::Constructor(_)
        ) {
            return None;
        }
        let &FlowSubject::Binding(range) = self.subjects.as_slice().get(subject as usize)? else {
            return None;
        };
        let (pattern, key) = self.subjects.pattern(range, self.model)?;
        self.parents.push(pattern);
        Some(FlowGuard::Property {
            object: pattern,
            key,
            test,
            positive,
            optional_chain: false,
            aliased: false,
        })
    }

    /// Returns the object and the key of the property that `operand` holds
    /// when `operand` names a `const` that holds a property of a subject, as
    /// `k` holds `s.kind` after `const k = s.kind`. The returned flag tells
    /// whether the `const` reads the property through an optional chain.
    ///
    /// The accepted declarations match those through which TypeScript narrows
    /// the object of a discriminant, except where noted below. The `const` is
    /// not exported and has no type annotation, as for an aliased condition,
    /// and it is declared in one of these ways:
    ///
    /// - Its initializer is a member access whose object is a subject, as in
    ///   `const k = s.kind`, `s?.kind`, `s["kind"]`, or `t[0]`.
    /// - It is a property or an element of a destructuring pattern that the
    ///   declarator binds directly, without a default, as in
    ///   `const { kind } = s`, `const { kind: k } = s`, or `const [k] = t`,
    ///   and the declarator's initializer is a subject, as `s` or `w.s` is.
    ///   Such a `const` never reads its property through an optional chain.
    ///
    /// A rest element holds nothing. TypeScript takes `ok` in
    /// `const { ...ok } = r` as `r.ok`, although `ok` holds an object.
    ///
    /// Parentheses around the initializer don't matter. `s!.kind`, `s!`, and
    /// `s as S` are not subjects, so they hold nothing, although TypeScript
    /// accepts `s!.kind`. A chain that starts at a parenthesized optional
    /// chain, such as `(s?.a)?.kind`, holds nothing either, because a test of
    /// such a chain narrows nothing. See [`chain_receivers`]. For a pattern,
    /// the path of the property is added to the subject table, so that
    /// collection can find writes to it.
    fn held_property(
        &mut self,
        operand: &AnyJsExpression,
    ) -> Option<(FlowSubjectId, PropertyKey, bool)> {
        let AnyJsExpression::JsIdentifierExpression(identifier) = operand else {
            return None;
        };
        let binding = self.model.binding(&identifier.name().ok()?)?;
        // As for aliased conditions, TypeScript holds nothing in an
        // `export const`, and Biome also skips a `const` exported by a
        // specifier, which TypeScript accepts.
        if binding.is_exported() {
            return None;
        }
        let (declarator, key) = match binding.tree().declaration()? {
            AnyJsBindingDeclaration::JsVariableDeclarator(declarator) => (declarator, None),
            AnyJsBindingDeclaration::JsObjectBindingPatternShorthandProperty(property) => {
                if property.init().is_some() {
                    return None;
                }
                let key = identifier_key(&binding.tree().name_token().ok()?)?;
                (pattern_declarator(&property.into())?, Some(key))
            }
            AnyJsBindingDeclaration::JsObjectBindingPatternProperty(property) => {
                if property.init().is_some() {
                    return None;
                }
                let key = member_name_key(&property.member().ok()?)?;
                (pattern_declarator(&property.into())?, Some(key))
            }
            AnyJsBindingDeclaration::JsArrayBindingPatternElement(element) => {
                if element.init().is_some() {
                    return None;
                }
                // Holes count, so `k` holds `t[1]` in `const [, k] = t`.
                let key = array_element_key(&element)?;
                (pattern_declarator(&element.into())?, Some(key))
            }
            _ => return None,
        };
        let initializer = omit_parentheses(const_initializer(&declarator)?, &mut self.remaining)?;
        chain_receivers(&initializer, &mut self.remaining)?;
        match key {
            None => {
                if !matches!(
                    initializer,
                    AnyJsExpression::JsStaticMemberExpression(_)
                        | AnyJsExpression::JsComputedMemberExpression(_)
                ) {
                    return None;
                }
                let subject = subject_of(&initializer, self.model, self.subjects)?;
                let FlowSubject::Member {
                    object,
                    key: key @ PropertyKey::Name(_),
                } = self.subjects.as_slice().get(subject as usize)?
                else {
                    return None;
                };
                Some((*object, key.clone(), is_optional_chain(&initializer)))
            }
            Some(key) => {
                if !has_subject_shape(&initializer)? {
                    return None;
                }
                let object = subject_of(&initializer, self.model, self.subjects)?;
                self.subjects.intern(FlowSubject::Member {
                    object,
                    key: key.clone(),
                })?;
                Some((object, key, false))
            }
        }
    }

    /// Returns `guard` preceded by a test that each of `receivers` is not
    /// nullish, combined as `&&` combines its operands. The innermost
    /// receiver, which `receivers` lists last, is tested first. A receiver
    /// that is not a subject adds no test.
    fn receiver_tests(&mut self, receivers: Vec<AnyJsExpression>, guard: FlowGuard) -> FlowGuard {
        self.receiver_tests_by(receivers, guard, FlowTest::Nullish)
    }

    /// Returns `guard` preceded by the negative `test` of each of
    /// `receivers`, as [`Self::receiver_tests`] does with [`FlowTest::Nullish`].
    fn receiver_tests_by(
        &mut self,
        receivers: Vec<AnyJsExpression>,
        guard: FlowGuard,
        test: FlowTest,
    ) -> FlowGuard {
        let mut guard = guard;
        for receiver in receivers {
            let Some(subject) = subject_of(&receiver, self.model, self.subjects) else {
                continue;
            };
            let testable = self.testable(subject);
            if testable == Testable::Never || self.any_root(subject) != AnyRoot::None {
                continue;
            }
            self.mentions.push(subject);
            let test = FlowGuard::Test {
                subject,
                test,
                positive: false,
            };
            let test = self.applied_when(test, testable);
            let left = self.push(test);
            let right = self.push(guard);
            guard = FlowGuard::Both {
                left,
                right,
                sequential: true,
            };
        }
        guard
    }

    /// Returns the initializer that a truthiness test of `operand` inlines
    /// when `operand` names an aliased condition: a `const` that is not
    /// exported, has no type annotation, and binds an identifier, not a
    /// pattern, to an initializer. TypeScript inlines nothing else, though it
    /// also accepts a `const` exported by an `export { ... }` list.
    ///
    /// Returns `None` once [`MAX_INLINED_ALIASES`] initializers enclose
    /// `operand`, as TypeScript stops there too.
    fn aliased_condition(&mut self, operand: &AnyJsExpression) -> Option<AnyJsExpression> {
        let initializer = self.alias_initializer(operand)?;
        self.initializers.push(initializer.clone());
        Some(initializer)
    }

    /// Returns the initializer that [`Self::aliased_condition`] inlines for
    /// `operand`, without recording it.
    fn alias_initializer(&self, operand: &AnyJsExpression) -> Option<AnyJsExpression> {
        if self.aliases >= MAX_INLINED_ALIASES {
            return None;
        }
        let AnyJsExpression::JsIdentifierExpression(identifier) = operand else {
            return None;
        };
        let binding = self.model.binding(&identifier.name().ok()?)?;
        // TypeScript never inlines an `export const`.
        if binding.is_exported() {
            return None;
        }
        let AnyJsBindingDeclaration::JsVariableDeclarator(declarator) =
            binding.tree().declaration()?
        else {
            return None;
        };
        const_initializer(&declarator)
    }

    /// Returns whether `subject` is a variable whose annotation is `any`, or
    /// a property path that starts at one (see [`AnyRoot`]).
    fn any_root(&self, subject: FlowSubjectId) -> AnyRoot {
        AnyRoot::new(self.subjects, self.functions, subject)
    }

    /// Returns whether a test may narrow `subject`: always outside inlined
    /// initializers, and inside one, always for the subjects that
    /// [`narrows_through_aliases`] accepts and only when the path is readonly
    /// for those that [`narrows_through_aliases_when_readonly`] accepts.
    fn testable(&self, subject: FlowSubjectId) -> Testable {
        if self.aliases == 0 || narrows_through_aliases(self.subjects, subject, self.model) {
            Testable::Always
        } else if narrows_through_aliases_when_readonly(self.subjects, subject, self.model) {
            Testable::WhenReadonly
        } else {
            Testable::Never
        }
    }

    /// Returns `guard`, wrapped in [`FlowGuard::ReadonlyPath`] when it may
    /// narrow its subject only when the subject's path is readonly.
    fn applied_when(&mut self, guard: FlowGuard, testable: Testable) -> FlowGuard {
        match testable {
            Testable::WhenReadonly => FlowGuard::ReadonlyPath(self.push(guard)),
            Testable::Always | Testable::Never => guard,
        }
    }

    fn push(&mut self, guard: FlowGuard) -> usize {
        self.guards.push(guard);
        self.guards.len() - 1
    }
}

/// How a subject relates to a variable whose annotation is `any`, as
/// [`Decomposer::any_root`] decides it from the raw tables.
///
/// Only some runtime tests narrow `any` (see [`FlowTest::narrows_any`]), so
/// only those mention such a variable. TypeScript types a property of `any`
/// as `any` without narrowing it, so no test mentions a property path on such
/// a variable, and no property guard narrows the variable.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AnyRoot {
    /// The subject's variable has another annotation or none, or the subject
    /// is `this` or a pattern, or a path on one.
    None,
    /// The subject is a variable whose annotation is `any`.
    Variable,
    /// The subject is a property path on a variable whose annotation is
    /// `any`.
    Path,
}

impl AnyRoot {
    /// Returns how `subject`, an id in `subjects`, relates to a variable whose
    /// annotation is `any`, which `functions` reads from the raw tables.
    fn new(
        subjects: &FlowSubjectTable,
        functions: &GuardFunctions,
        subject: FlowSubjectId,
    ) -> Self {
        let subjects = subjects.as_slice();
        let mut root = subject;
        // A table stores an object before its members, so this ends.
        while let Some(FlowSubject::Member { object, .. }) = subjects.get(root as usize) {
            root = *object;
        }
        match subjects.get(root as usize) {
            Some(FlowSubject::Binding(range)) if functions.declares_any(*range) => {
                if root == subject {
                    Self::Variable
                } else {
                    Self::Path
                }
            }
            _ => Self::None,
        }
    }
}

/// Whether a runtime test may narrow a subject, as [`Decomposer::testable`]
/// decides it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Testable {
    Never,
    Always,
    /// Only when the subject's path is readonly, as
    /// [`FlowGuard::ReadonlyPath`] describes.
    WhenReadonly,
}

/// Returns whether the condition inlined from an aliased condition may narrow
/// `subject`, an id in `subjects`.
///
/// TypeScript applies an inlined condition only to a reference whose value
/// cannot change between the declaration of the alias and its test: a
/// variable that is never written and is declared by `const`, as a parameter
/// or a catch variable, or by `let` unless the `let` statement is at the top
/// level of a script, where other scripts can write it. Biome cannot tell a
/// script from a module, so it rejects a `let` statement at the top level of
/// any file.
///
/// Biome also rejects every exported variable, where TypeScript accepts a
/// `const` exported by an `export { ... }` list. A destructuring pattern is
/// accepted, as in TypeScript, because the table stores only patterns declared
/// by a `const` or by a parameter whose variables are never written. `this` is
/// rejected, because no read of `this` itself narrows. Every property path is
/// rejected too, because it narrows only when it is readonly; see
/// [`narrows_through_aliases_when_readonly`].
fn narrows_through_aliases(
    subjects: &FlowSubjectTable,
    subject: FlowSubjectId,
    model: &SemanticModel,
) -> bool {
    match subjects.as_slice().get(subject as usize) {
        Some(FlowSubject::Binding(range)) => is_constant_binding(*range, model),
        // The table stores only patterns whose values can't change.
        Some(FlowSubject::Pattern(_)) => true,
        Some(FlowSubject::This | FlowSubject::Member { .. }) | None => false,
    }
}

/// Returns whether the condition inlined from an aliased condition may narrow
/// the property path `subject`, an id in `subjects`, when the path is
/// readonly, as [`FlowGuard::ReadonlyPath`] describes.
///
/// TypeScript applies an inlined condition to a property access only when the
/// property is readonly and the object is itself a reference whose value
/// cannot change. So the path must start at a variable that
/// [`narrows_through_aliases`] accepts, or at `this`. The table stores `this`
/// only for a `this` in the root's own code, so `this` qualifies only for an
/// aliased condition that the root declares. Biome therefore rejects two
/// cases that TypeScript accepts: an aliased condition that an enclosing
/// method declares and an arrow function inside it tests, and one that reads
/// another function's `this`.
fn narrows_through_aliases_when_readonly(
    subjects: &FlowSubjectTable,
    subject: FlowSubjectId,
    model: &SemanticModel,
) -> bool {
    if !matches!(
        subjects.as_slice().get(subject as usize),
        Some(FlowSubject::Member { .. })
    ) {
        return false;
    }
    match subject_root(subjects.as_slice(), subject) {
        Some(FlowSubject::This) => true,
        Some(FlowSubject::Binding(range)) => is_constant_binding(*range, model),
        Some(FlowSubject::Pattern(_) | FlowSubject::Member { .. }) | None => false,
    }
}

/// Returns whether the variable declared at `range` keeps its value between
/// an aliased condition and its test, as [`narrows_through_aliases`]
/// describes.
#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "Only parameters, catch variables, and `const` and `let` declarations qualify."
)]
fn is_constant_binding(range: TextRange, model: &SemanticModel) -> bool {
    let Some(binding) = model.as_binding_by_range(range) else {
        return false;
    };
    if binding.is_exported() || binding.all_writes().next().is_some() {
        return false;
    }
    let Some(declaration) = binding.tree().declaration() else {
        return false;
    };
    match declaration
        .parent_binding_pattern_declaration()
        .unwrap_or(declaration)
    {
        AnyJsBindingDeclaration::JsFormalParameter(_)
        | AnyJsBindingDeclaration::JsRestParameter(_)
        | AnyJsBindingDeclaration::JsArrowFunctionExpression(_)
        | AnyJsBindingDeclaration::TsPropertyParameter(_)
        | AnyJsBindingDeclaration::JsCatchDeclaration(_) => true,
        AnyJsBindingDeclaration::JsVariableDeclarator(declarator) => {
            let Some(declaration) = declarator.declaration() else {
                return false;
            };
            match declaration.variable_kind() {
                Ok(JsVariableKind::Const) => true,
                Ok(JsVariableKind::Let) => !is_top_level_statement(&declaration),
                Ok(JsVariableKind::Var | JsVariableKind::Using) | Err(_) => false,
            }
        }
        _ => false,
    }
}

/// Returns the initializer of `declarator` when it declares a `const` without
/// a type annotation, which TypeScript requires of an aliased condition and of
/// a `const` that holds a property.
fn const_initializer(declarator: &JsVariableDeclarator) -> Option<AnyJsExpression> {
    if declarator.variable_annotation().is_some()
        || declarator.declaration()?.variable_kind().ok()? != JsVariableKind::Const
    {
        return None;
    }
    declarator.initializer()?.expression().ok()
}

/// Returns the declarator whose binding is the pattern that holds `element`,
/// or `None` when that pattern is nested in another one or belongs to a
/// parameter.
fn pattern_declarator(element: &AnyPatternElement) -> Option<JsVariableDeclarator> {
    // An element's parent is its list, inside the pattern.
    JsVariableDeclarator::cast(element.syntax().grand_parent()?.parent()?)
}

/// Returns whether `expression` is part of an optional chain, as `s?.kind`
/// and `w?.s.kind` are.
fn is_optional_chain(expression: &AnyJsExpression) -> bool {
    AnyJsOptionalChainExpression::cast_ref(expression.syntax())
        .is_some_and(|chain| chain.is_optional_chain())
}

/// Returns whether `declaration` is a variable statement directly in the body
/// of a module or script, not in a block, a loop head, or a namespace.
pub(super) fn is_top_level_statement(declaration: &AnyJsVariableDeclaration) -> bool {
    declaration
        .parent::<JsVariableStatement>()
        .and_then(|statement| statement.syntax().grand_parent())
        .is_some_and(|body| FILE_KINDS.matches(body.kind()))
}

/// Modules and scripts, whose item lists hold the statements at the top level
/// of a file.
const FILE_KINDS: SyntaxKindSet<JsLanguage> = JsModule::KIND_SET.union(JsScript::KIND_SET);

enum SyntaxGuard {
    Truthy,
    Nullish,
    Typeof(TypeofKind),
    /// Strict equality with `null`.
    Null,
    /// Equality with the value at `value`, a [`UnitValue::Literal`].
    Literal {
        value: TextRange,
        strict: bool,
    },
    /// Equality with `undefined`, the global variable or the value of `void`.
    Undefined {
        strict: bool,
    },
    /// Equality with the member of an enum whose name is at `member`, a
    /// [`UnitValue::EnumMember`].
    EnumMember {
        member: TextRange,
        strict: bool,
    },
    /// Equality with the imported variable declared at `binding`, or with
    /// its member whose name is at `member`, a [`UnitValue::Imported`].
    Imported {
        binding: TextRange,
        member: Option<TextRange>,
        strict: bool,
    },
    /// Equality with the value at `value`, which a [`FlowTest::Compared`]
    /// test narrows by its declared type.
    Compared {
        value: TextRange,
        strict: bool,
    },
    /// A call to a type-guard function, with the test it applies, a
    /// [`FlowTest::Predicate`].
    Predicate(FlowTest),
    /// The `in` operator whose key is the literal at this range.
    In(TextRange),
    /// The `instanceof` operator whose right operand is at this range.
    Instanceof(TextRange),
    /// The `in` operator with a private name, a [`FlowTest::PrivateName`].
    PrivateName {
        class: TextRange,
        is_static: bool,
    },
    /// The equality of `constructor` with the value at this range, a
    /// [`FlowTest::Constructor`].
    Constructor(TextRange),
}

/// One syntax step, with child expressions left undecomposed.
enum ConditionStep {
    Unsupported,
    /// A runtime test of `operand`, an expression with the shape of a subject
    /// or a call in an optional chain.
    Guard {
        operand: AnyJsExpression,
        guard: SyntaxGuard,
        positive: bool,
        /// Whether the guard also tests that the receivers of the optional
        /// chain around `operand` are not nullish.
        receivers: bool,
    },
    /// A call that may report a type guard's test.
    Call {
        call: JsCallExpression,
        positive: bool,
    },
    Negated {
        argument: AnyJsExpression,
        outcome: FlowOutcome,
    },
    Logical {
        left: AnyJsExpression,
        right: AnyJsExpression,
        outcome: FlowOutcome,
        sequential: bool,
    },
    /// A comparison of `operand` with a boolean literal, as in
    /// `(typeof x === "string") === true`, which holds where `operand` has
    /// `outcome` when `assumption` holds for it. `test` is the comparison's
    /// own test of `operand` when `operand` is a variable, whose aliased
    /// condition the comparison also inlines.
    Boolean {
        operand: AnyJsExpression,
        outcome: FlowOutcome,
        assumption: BooleanAssumption,
        test: Option<Box<Self>>,
    },
    /// Two steps that hold together, as the tests of both operands of a
    /// comparison of two subjects do.
    Both {
        left: Box<Self>,
        right: Box<Self>,
    },
}

/// What a comparison with a boolean literal requires of the compared
/// expression before it decomposes the expression as truthy or falsy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum BooleanAssumption {
    /// Nothing: `c === true` and `c == true` hold only for a truthy `c`, and
    /// `c === false` only for a falsy `c`.
    Always,
    /// Every truthy value of the expression is `true`
    /// ([`is_boolean_valued`]), so a value that isn't `true`, or that
    /// loosely equals `false`, is falsy.
    TruthyIsTrue,
    /// Every falsy value of the expression is `false`, so a value that isn't
    /// `false` is truthy.
    FalsyIsFalse,
}

impl BooleanAssumption {
    /// Returns whether the assumption holds for `expression`.
    fn holds(self, expression: &AnyJsExpression, remaining: &mut usize) -> bool {
        match self {
            Self::Always => true,
            Self::TruthyIsTrue => is_boolean_when(expression, true, remaining),
            Self::FalsyIsFalse => is_boolean_when(expression, false, remaining),
        }
    }
}

/// Decomposes one condition step, spending work on the step and on removing
/// parentheses. Missing syntax and exhausted work return `None`, not
/// `Unsupported`.
#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "Only supported narrowing syntax is decomposed."
)]
fn condition_step(
    expression: AnyJsExpression,
    outcome: FlowOutcome,
    model: &SemanticModel,
    remaining: &mut usize,
) -> Option<ConditionStep> {
    *remaining = remaining.checked_sub(1)?;
    let expression = omit_parentheses(expression, remaining)?;
    let positive = matches!(outcome, FlowOutcome::Truthy | FlowOutcome::Nullish);
    if has_subject_shape(&expression)? {
        let guard = match outcome {
            FlowOutcome::Truthy | FlowOutcome::Falsy => SyntaxGuard::Truthy,
            FlowOutcome::Nullish | FlowOutcome::NonNullish => SyntaxGuard::Nullish,
        };
        return Some(ConditionStep::Guard {
            operand: expression,
            guard,
            positive,
            // TypeScript narrows no receiver for either outcome of the left
            // operand of `??`, although a value that is not nullish shows that
            // the chain ran.
            receivers: outcome == FlowOutcome::Truthy,
        });
    }
    if matches!(outcome, FlowOutcome::Nullish | FlowOutcome::NonNullish) {
        return Some(ConditionStep::Unsupported);
    }
    match expression {
        AnyJsExpression::JsCallExpression(call) => Some(ConditionStep::Call { call, positive }),
        AnyJsExpression::JsUnaryExpression(unary) => {
            let operator = unary.operator().ok()?;
            let argument = unary.argument().ok()?;
            if operator != JsUnaryOperator::LogicalNot {
                return Some(ConditionStep::Unsupported);
            }
            Some(ConditionStep::Negated {
                argument,
                outcome: if positive {
                    FlowOutcome::Falsy
                } else {
                    FlowOutcome::Truthy
                },
            })
        }
        AnyJsExpression::JsLogicalExpression(logical) => {
            let left = logical.left().ok()?;
            let right = logical.right().ok()?;
            let operator = logical.operator().ok()?;
            if operator == JsLogicalOperator::NullishCoalescing {
                return Some(ConditionStep::Unsupported);
            }
            Some(ConditionStep::Logical {
                left,
                right,
                outcome,
                sequential: (operator == JsLogicalOperator::LogicalAnd) == positive,
            })
        }
        AnyJsExpression::JsInExpression(expression) => {
            let object = omit_parentheses(expression.object().ok()?, remaining)?;
            let guard = match expression.property().ok()? {
                AnyJsInProperty::JsPrivateName(name) => {
                    match private_name_class(&name, remaining) {
                        Some((class, is_static)) => SyntaxGuard::PrivateName { class, is_static },
                        None => return Some(ConditionStep::Unsupported),
                    }
                }
                AnyJsInProperty::AnyJsExpression(key) => {
                    let key = omit_parentheses(key, remaining)?;
                    if literal_key(&key).is_none() {
                        return Some(ConditionStep::Unsupported);
                    }
                    SyntaxGuard::In(key.range())
                }
            };
            if !has_subject_shape(&object)? {
                return Some(ConditionStep::Unsupported);
            }
            Some(ConditionStep::Guard {
                operand: object,
                guard,
                positive,
                // `in` throws on `null` and `undefined`, but TypeScript
                // narrows no receiver of an optional chain through it.
                receivers: false,
            })
        }
        AnyJsExpression::JsInstanceofExpression(expression) => {
            let left = omit_parentheses(expression.left().ok()?, remaining)?;
            let right = omit_parentheses(expression.right().ok()?, remaining)?;
            if !has_operand_shape(&left)? {
                return Some(ConditionStep::Unsupported);
            }
            Some(ConditionStep::Guard {
                operand: left,
                guard: SyntaxGuard::Instanceof(right.range()),
                positive,
                // As in TypeScript, an instance is never nullish, so the
                // receivers of an optional chain narrow in the true branch.
                receivers: positive,
            })
        }
        AnyJsExpression::JsBinaryExpression(binary) => {
            let left = binary.left().ok()?;
            let right = binary.right().ok()?;
            let operator = binary.operator().ok()?;
            let equal = match operator {
                JsBinaryOperator::Equality | JsBinaryOperator::StrictEquality => positive,
                JsBinaryOperator::Inequality | JsBinaryOperator::StrictInequality => !positive,
                _ => return Some(ConditionStep::Unsupported),
            };
            let strict = matches!(
                operator,
                JsBinaryOperator::StrictEquality | JsBinaryOperator::StrictInequality
            );
            let left = omit_parentheses(left, remaining)?;
            let right = omit_parentheses(right, remaining)?;
            for (subject, value) in [(&left, &right), (&right, &left)] {
                if let Some(step) = constructor_step(subject, value, equal, remaining) {
                    return Some(step);
                }
            }
            for (operand, value) in [(&left, &right), (&right, &left)] {
                if let Some(step) =
                    boolean_comparison_step(operand, value, strict, equal, model, remaining)?
                {
                    return Some(step);
                }
            }
            for (subject, value) in [(&left, &right), (&right, &left)] {
                let step = comparison_step(subject, value, strict, equal, model, remaining)?;
                if !matches!(step, ConditionStep::Unsupported) {
                    return Some(step);
                }
            }
            typed_comparison_step(&left, &right, strict, equal, model, remaining)
        }
        _ => Some(ConditionStep::Unsupported),
    }
}

/// Decomposes the comparison of `operand` with `value`, both without
/// parentheses, when `value` is the literal `true` or `false`, as
/// `operand === value` when `strict` is true and as `operand == value`
/// otherwise, assuming that they are equal when `positive` is true. Returns
/// `Some(None)` when `value` is no boolean literal or `operand` has the shape
/// of a subject other than a variable, which [`comparison_step`] tests as a
/// unit value.
///
/// As in TypeScript, the comparison holds where `operand` is truthy or falsy,
/// so `(typeof x === "string") === true` narrows `x` as
/// `typeof x === "string"` does. Each branch assumes only what the comparison
/// shows:
///
/// - `=== true` and `== true` hold only for a truthy value, and `=== false`
///   only for `false`, so these branches decompose `operand` unconditionally.
/// - A value that isn't `true`, or that loosely equals `false`, is falsy only
///   when every truthy value of `operand` is `true`.
/// - A value that isn't `false`, strictly or loosely, is truthy only when
///   every falsy value of `operand` is `false`.
///
/// TypeScript assumes the outcome in every branch, which is wrong for an
/// object, which is truthy but not `true`, and for `0` or `null`, which are
/// falsy but not `false`.
///
/// A variable is tested as a unit value, and when it names an aliased
/// condition, the comparison also inlines its initializer under the same
/// rules. TypeScript narrows nothing when `operand`, without parentheses and
/// `!`, is `&&`, `||`, or `??`, so such a comparison is
/// [`ConditionStep::Unsupported`].
fn boolean_comparison_step(
    operand: &AnyJsExpression,
    value: &AnyJsExpression,
    strict: bool,
    positive: bool,
    model: &SemanticModel,
    remaining: &mut usize,
) -> Option<Option<ConditionStep>> {
    let AnyJsExpression::AnyJsLiteralExpression(
        AnyJsLiteralExpression::JsBooleanLiteralExpression(literal),
    ) = value
    else {
        return Some(None);
    };
    let is_true = literal.value_token().ok()?.kind() == T![true];
    let (outcome, assumption) = match (strict, is_true, positive) {
        (_, true, true) => (FlowOutcome::Truthy, BooleanAssumption::Always),
        (_, true, false) | (false, false, true) => {
            (FlowOutcome::Falsy, BooleanAssumption::TruthyIsTrue)
        }
        (true, false, true) => (FlowOutcome::Falsy, BooleanAssumption::Always),
        (_, false, false) => (FlowOutcome::Truthy, BooleanAssumption::FalsyIsFalse),
    };
    if let AnyJsExpression::JsIdentifierExpression(_) = operand {
        let test = comparison_step(operand, value, strict, positive, model, remaining)?;
        return Some(Some(ConditionStep::Boolean {
            operand: operand.clone(),
            outcome,
            assumption,
            test: Some(Box::new(test)),
        }));
    }
    if has_operand_shape(operand)? {
        return Some(None);
    }
    let mut inner = operand.clone();
    loop {
        *remaining = remaining.checked_sub(1)?;
        inner = match omit_parentheses(inner, remaining)? {
            AnyJsExpression::JsUnaryExpression(unary)
                if unary.operator().ok()? == JsUnaryOperator::LogicalNot =>
            {
                unary.argument().ok()?
            }
            AnyJsExpression::JsLogicalExpression(_) => {
                return Some(Some(ConditionStep::Unsupported));
            }
            _ => break,
        };
    }
    Some(Some(ConditionStep::Boolean {
        operand: operand.clone(),
        outcome,
        assumption,
        test: None,
    }))
}

/// Decomposes the comparison of `left` with `right`, both without
/// parentheses, by the declared type of each operand, when neither is a unit
/// value: each operand that can be the operand of a comparison is tested
/// when the other has a declared type that TypeScript gives it too
/// ([`has_declared_type`]). Both tests hold together, so `x === y` tests `x`
/// by the declared type of `y` and `y` by that of `x`. The comparison is
/// strict when `strict` is true, and the operands are equal when `positive`
/// is true.
///
/// Returns [`ConditionStep::Unsupported`] when no operand is tested, as for
/// a value whose type Biome infers, because an inferred type can be
/// narrower than TypeScript's.
fn typed_comparison_step(
    left: &AnyJsExpression,
    right: &AnyJsExpression,
    strict: bool,
    positive: bool,
    model: &SemanticModel,
    remaining: &mut usize,
) -> Option<ConditionStep> {
    let mut steps = Vec::new();
    for (operand, value) in [(left, right), (right, left)] {
        if has_operand_shape(operand)? && has_declared_type(value, model, remaining)? {
            steps.push(ConditionStep::Guard {
                operand: operand.clone(),
                guard: SyntaxGuard::Compared {
                    value: value.range(),
                    strict,
                },
                positive,
                // Where the operands are equal, the receivers of an optional
                // chain are not nullish when the value can't be `undefined`,
                // which inference decides from its type.
                receivers: positive,
            });
        }
    }
    let right = steps.pop();
    Some(match (steps.pop(), right) {
        (Some(left), Some(right)) => ConditionStep::Both {
            left: Box::new(left),
            right: Box::new(right),
        },
        (None, Some(step)) | (Some(step), None) => step,
        (None, None) => ConditionStep::Unsupported,
    })
}

/// Returns whether `value`, an expression without parentheses, has a declared
/// type that TypeScript gives it too: a variable or a parameter declared with
/// a type annotation, a property path that starts at one, such as `o.k` with
/// `o: { k: "a" }`, or an `as const` expression.
///
/// Other values, such as a variable without an annotation or a call, have
/// types that Biome infers, which can be narrower than TypeScript's. So does
/// a variable named `undefined` or `globalThis`, whose reads inference types
/// as the globals of those names whatever they refer to. An imported
/// variable is a unit value of its own ([`UnitValue::Imported`]), whose
/// declaration inference checks. Returns `None` when the syntax is
/// incomplete or the walk runs out of work.
#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "Only annotated variables, their paths, and `as const` have declared types."
)]
fn has_declared_type(
    value: &AnyJsExpression,
    model: &SemanticModel,
    remaining: &mut usize,
) -> Option<bool> {
    match value {
        AnyJsExpression::TsAsExpression(expression) => {
            return Some(expression.ty().is_ok_and(|ty| is_const_type(&ty)));
        }
        AnyJsExpression::TsTypeAssertionExpression(expression) => {
            return Some(expression.ty().is_ok_and(|ty| is_const_type(&ty)));
        }
        _ => {}
    }
    if !has_subject_shape(value)? {
        return Some(false);
    }
    let mut current = value.clone();
    loop {
        *remaining = remaining.checked_sub(1)?;
        current = match current {
            AnyJsExpression::JsIdentifierExpression(identifier) => {
                let reference = identifier.name().ok()?;
                if matches!(
                    reference.value_token().ok()?.text_trimmed(),
                    "undefined" | "globalThis"
                ) {
                    return Some(false);
                }
                let Some(binding) = model.binding(&reference) else {
                    return Some(false);
                };
                return Some(match binding.tree().declaration() {
                    Some(AnyJsBindingDeclaration::JsVariableDeclarator(declarator)) => {
                        declarator.variable_annotation().is_some()
                    }
                    Some(AnyJsBindingDeclaration::JsFormalParameter(parameter)) => {
                        parameter.type_annotation().is_some()
                    }
                    Some(AnyJsBindingDeclaration::JsRestParameter(parameter)) => {
                        parameter.type_annotation().is_some()
                    }
                    _ => false,
                });
            }
            AnyJsExpression::JsStaticMemberExpression(member) => {
                omit_parentheses(member.object().ok()?, remaining)?
            }
            AnyJsExpression::JsComputedMemberExpression(member) => {
                omit_parentheses(member.object().ok()?, remaining)?
            }
            _ => return Some(false),
        };
    }
}

/// Returns whether `ty` is the `const` of an `as const` or `<const>`
/// assertion.
fn is_const_type(ty: &AnyTsType) -> bool {
    let Some(reference) = ty.as_ts_reference_type() else {
        return false;
    };
    reference.type_arguments().is_none()
        && reference.name().is_ok_and(|name| {
            name.as_js_reference_identifier()
                .and_then(|identifier| identifier.value_token().ok())
                .is_some_and(|token| token.text_trimmed() == "const")
        })
}

/// Decomposes the comparison of `subject` with `value`, both without
/// parentheses, as `subject === value` when `strict` is true and as
/// `subject == value` otherwise, assuming that they are equal when
/// `positive` is true.
///
/// `subject` is tested when it can be the operand of a comparison and `value`
/// is a unit value, such as `"a"`, `-1`, `null`, `undefined`, or a `const`
/// that holds one. When `subject` is `typeof` of such an operand, the operand
/// is tested when `value` names a kind of value, as `"string"` and
/// `` `string` `` do. Other comparisons are [`ConditionStep::Unsupported`]
/// here; [`typed_comparison_step`] tests them by declared types.
#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "Comparisons only support subjects and primitive guards."
)]
fn comparison_step(
    subject: &AnyJsExpression,
    value: &AnyJsExpression,
    strict: bool,
    positive: bool,
    model: &SemanticModel,
    remaining: &mut usize,
) -> Option<ConditionStep> {
    let (operand, guard) = match subject {
        subject if has_operand_shape(subject)? => {
            if let AnyJsExpression::AnyJsLiteralExpression(literal) = value {
                literal.value_token().ok()?;
            }
            let Some(unit) = unit_value(value.clone(), model, remaining) else {
                return Some(ConditionStep::Unsupported);
            };
            let guard = match (unit, strict) {
                (UnitValue::Null, true) => SyntaxGuard::Null,
                (UnitValue::Null, false) => SyntaxGuard::Nullish,
                (UnitValue::Undefined, strict) => SyntaxGuard::Undefined { strict },
                (UnitValue::Literal(value), strict) => SyntaxGuard::Literal { value, strict },
                (UnitValue::EnumMember(member), strict) => {
                    SyntaxGuard::EnumMember { member, strict }
                }
                (UnitValue::Imported { binding, member }, strict) => SyntaxGuard::Imported {
                    binding,
                    member,
                    strict,
                },
            };
            (subject.clone(), guard)
        }
        AnyJsExpression::JsUnaryExpression(unary) => {
            if unary.operator().ok()? != JsUnaryOperator::Typeof {
                return Some(ConditionStep::Unsupported);
            }
            let argument = unary.argument().ok()?;
            let value = match value {
                AnyJsExpression::AnyJsLiteralExpression(
                    AnyJsLiteralExpression::JsStringLiteralExpression(literal),
                ) => literal.inner_string_text().ok()?,
                AnyJsExpression::JsTemplateExpression(template) => {
                    match plain_template_text(template) {
                        Some(text) => text,
                        None => return Some(ConditionStep::Unsupported),
                    }
                }
                _ => return Some(ConditionStep::Unsupported),
            };
            let kind = match value.text() {
                "undefined" => TypeofKind::Undefined,
                "object" => TypeofKind::Object,
                "boolean" => TypeofKind::Boolean,
                "number" => TypeofKind::Number,
                "bigint" => TypeofKind::BigInt,
                "string" => TypeofKind::String,
                "symbol" => TypeofKind::Symbol,
                "function" => TypeofKind::Function,
                _ => return Some(ConditionStep::Unsupported),
            };
            let argument = omit_parentheses(argument, remaining)?;
            if !has_operand_shape(&argument)? {
                return Some(ConditionStep::Unsupported);
            }
            (argument, SyntaxGuard::Typeof(kind))
        }
        _ => return Some(ConditionStep::Unsupported),
    };
    // TypeScript narrows the receivers of an optional chain in the branch
    // where the operand cannot be `undefined`: where it equals a primitive
    // literal, a member of an enum, which is a number or a string, or,
    // strictly, `null`, where it differs from `undefined` or, loosely, from
    // `null`, and where its `typeof` is not `"undefined"`. Collection can't
    // tell whether an imported value is `undefined`, `null`, or a primitive
    // literal, so a comparison with one narrows no receiver.
    let receivers = match &guard {
        SyntaxGuard::Null | SyntaxGuard::Literal { .. } | SyntaxGuard::EnumMember { .. } => {
            positive
        }
        SyntaxGuard::Nullish | SyntaxGuard::Undefined { .. } => !positive,
        SyntaxGuard::Typeof(kind) => positive == (*kind != TypeofKind::Undefined),
        SyntaxGuard::Truthy
        | SyntaxGuard::Imported { .. }
        | SyntaxGuard::Compared { .. }
        | SyntaxGuard::Predicate(_)
        | SyntaxGuard::In(_)
        | SyntaxGuard::Instanceof(_)
        | SyntaxGuard::PrivateName { .. }
        | SyntaxGuard::Constructor(_) => false,
    };
    Some(ConditionStep::Guard {
        operand,
        guard,
        positive,
        receivers,
    })
}

/// Decomposes the comparison of `constructor`, the read of the property
/// `constructor` of an operand, with `value`, a dotted name such as `Date`
/// or `ns.Foo`, assuming that they are equal when `positive` is true, as
/// in `x.constructor === Date` or `Date == x.constructor`.
///
/// Returns `None` when `constructor` doesn't read `constructor` or `value`
/// is not a dotted name, so the comparison is decomposed otherwise. In the
/// branch where the two are equal, the step tests the operand, and the
/// receivers of its optional chain as not nullish, as in
/// `o?.v.constructor === Foo`. In the other branch, which TypeScript doesn't
/// narrow, the step is [`ConditionStep::Unsupported`].
fn constructor_step(
    constructor: &AnyJsExpression,
    value: &AnyJsExpression,
    positive: bool,
    remaining: &mut usize,
) -> Option<ConditionStep> {
    let AnyJsExpression::JsStaticMemberExpression(member) = constructor else {
        return None;
    };
    let operand = constructor_operand(member, remaining)?;
    if !is_dotted_name(value, remaining) {
        return None;
    }
    if !positive {
        return Some(ConditionStep::Unsupported);
    }
    Some(ConditionStep::Guard {
        operand,
        guard: SyntaxGuard::Constructor(value.range()),
        positive,
        receivers: true,
    })
}

/// Returns the object of `member`, without parentheses, when `member` reads
/// the property `constructor` of an expression with the shape of an
/// operand, as `x.constructor` and `x?.constructor` do.
fn constructor_operand(
    member: &JsStaticMemberExpression,
    remaining: &mut usize,
) -> Option<AnyJsExpression> {
    let AnyJsName::JsName(name) = member.member().ok()? else {
        return None;
    };
    if name.value_token().ok()?.text_trimmed() != "constructor" {
        return None;
    }
    let object = omit_parentheses(member.object().ok()?, remaining)?;
    has_operand_shape(&object)?.then_some(object)
}

/// Returns the range at which inference finds the type of the class that
/// declares the private name `name`, and whether the name is static.
///
/// The class is the innermost one around `name` that declares a member with
/// its name, as JavaScript resolves a private name. Its range is the name
/// that a class declaration binds, or the class expression itself. Returns
/// `None` when no class around `name` declares it, when that class is an
/// anonymous default export, and when the walk runs out of work.
fn private_name_class(name: &JsPrivateName, remaining: &mut usize) -> Option<(TextRange, bool)> {
    let token = name.value_token().ok()?;
    let text = token.text_trimmed();
    for class in name.syntax().ancestors().filter_map(AnyJsClass::cast) {
        for member in class.members() {
            *remaining = remaining.checked_sub(1)?;
            let Ok(Some(AnyJsClassMemberName::JsPrivateClassMemberName(member_name))) =
                member.name()
            else {
                continue;
            };
            if member_name.id_token().ok()?.text_trimmed() != text {
                continue;
            }
            let range = match &class {
                AnyJsClass::JsClassDeclaration(declaration) => declaration.id().ok()?.range(),
                AnyJsClass::JsClassExportDefaultDeclaration(declaration) => {
                    declaration.id()?.range()
                }
                AnyJsClass::JsClassExpression(expression) => expression.range(),
            };
            return Some((range, is_static_member(&member)));
        }
    }
    None
}

/// A value of a unit type, a type with a single value such as `"a"` or
/// `null`. A comparison with a unit value narrows the other operand, as in
/// TypeScript. Where the operands are equal, the other operand keeps the
/// members that can equal the value. Where they differ, it loses the members
/// that are the value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UnitValue {
    Null,
    /// `undefined`: the global variable, or the value of `void`, which is
    /// `undefined` whatever its operand is.
    Undefined,
    /// The value of the expression at this range, whose collected type is a
    /// primitive literal type: a string, number, bigint, or boolean literal,
    /// `-` or `+` directly before a number literal, or a template literal
    /// without a tag and without substitutions. TypeScript gives `-(1)` the
    /// type `number`, so an operand in parentheses doesn't count. Biome gives
    /// `-1n` the type `bigint`, where TypeScript gives `-1n`, so it doesn't
    /// count either.
    Literal(TextRange),
    /// The value of the member of an enum that this module declares whose
    /// name is at this range, read as `E.A` or `E["A"]`. Only inference can
    /// tell whether the member has a unit value, from the literal type that
    /// collection gives it when it computes its value.
    EnumMember(TextRange),
    /// The value of the variable declared at `binding`, which a named or a
    /// default import binds, or, when `member` is the range of a member's
    /// name, the value of that member of the enum that the variable imports,
    /// as `A` in `E.A`. Only inference can tell whether it is a unit value,
    /// because the value is written in another module.
    Imported {
        binding: TextRange,
        member: Option<TextRange>,
    },
}

/// Limits the `const` declarations followed to find the value that a
/// comparison names.
const MAX_HELD_VALUES: usize = 16;

/// Returns the unit value of `expression`, or `None` when it has none that
/// Biome compares with.
///
/// A literal, `null`, `undefined`, and `void` have a unit value. A `const`
/// without a type annotation and without destructuring holds the unit value of
/// its initializer, because TypeScript gives such a `const` the type of its
/// initializer. So `K` holds `"a"` after `const K = "a"`, also through such
/// `const`s that name one another, at most 16 of them. A variable that a named
/// or a default import binds is a [`UnitValue::Imported`]. A member of an
/// enum, read as `E.A` or `E["A"]` where `E` names an enum declaration of this
/// module, is a [`UnitValue::EnumMember`], and one where `E` names an import
/// is a [`UnitValue::Imported`] with the member's name.
///
/// Other variables have no unit value, such as a `let`, whose literal type
/// widens, and a variable with a type annotation. Other expressions have none
/// either, such as a call or `1 === 1`, because Biome may infer their types
/// narrower than TypeScript does.
pub(crate) fn unit_value(
    expression: AnyJsExpression,
    model: &SemanticModel,
    remaining: &mut usize,
) -> Option<UnitValue> {
    let mut expression = omit_parentheses(expression, remaining)?;
    for _ in 0..MAX_HELD_VALUES {
        let identifier = match &expression {
            AnyJsExpression::JsIdentifierExpression(identifier) => identifier,
            AnyJsExpression::JsStaticMemberExpression(_)
            | AnyJsExpression::JsComputedMemberExpression(_) => {
                return enum_member_unit_value(&expression, model);
            }
            _ => return literal_unit_value(&expression),
        };
        let reference = identifier.name().ok()?;
        let Some(binding) = model.binding(&reference) else {
            let is_undefined = reference.value_token().ok()?.text_trimmed() == "undefined";
            return is_undefined.then_some(UnitValue::Undefined);
        };
        match held_value(&binding)? {
            HeldValue::Imported => {
                return Some(UnitValue::Imported {
                    binding: binding.range(),
                    member: None,
                });
            }
            HeldValue::Initializer(initializer) => {
                expression = omit_parentheses(initializer, remaining)?;
            }
        }
    }
    None
}

/// Returns the unit value of `expression` when it reads a member of an enum
/// through an identifier that names the enum, as `E.A` or `E["A"]` does:
/// [`UnitValue::EnumMember`] when the identifier names an enum declaration of
/// this module that has a member of that name, and [`UnitValue::Imported`]
/// with the member's name when an import binds the identifier, because only
/// inference can tell that the import is an enum.
///
/// Returns `None` for other member accesses, including an optional chain such
/// as `E?.A`, and for a member declared with a computed name, as `["A"]`,
/// which has no binding of its own, so the comparison keeps the type.
#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "Only enum declarations and value imports have members that are unit values."
)]
pub(crate) fn enum_member_unit_value(
    expression: &AnyJsExpression,
    model: &SemanticModel,
) -> Option<UnitValue> {
    let (object, name) = match expression {
        AnyJsExpression::JsStaticMemberExpression(member) if !member.is_optional_chain() => (
            member.object().ok()?,
            member.member().ok()?.as_js_name()?.value_token().ok()?,
        ),
        AnyJsExpression::JsComputedMemberExpression(member) if !member.is_optional_chain() => {
            let AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsStringLiteralExpression(key),
            ) = member.member().ok()?
            else {
                return None;
            };
            (member.object().ok()?, key.value_token().ok()?)
        }
        _ => return None,
    };
    let AnyJsExpression::JsIdentifierExpression(identifier) = object else {
        return None;
    };
    let binding = model.binding(&identifier.name().ok()?)?;
    match binding.tree().declaration()? {
        AnyJsBindingDeclaration::TsEnumDeclaration(declaration) => {
            enum_member_name_range(&declaration, member_key(&name).text())
                .map(UnitValue::EnumMember)
        }
        AnyJsBindingDeclaration::JsNamedImportSpecifier(_)
        | AnyJsBindingDeclaration::JsShorthandNamedImportSpecifier(_)
        | AnyJsBindingDeclaration::JsDefaultImportSpecifier(_) => Some(UnitValue::Imported {
            binding: binding.range(),
            member: Some(name.text_trimmed_range()),
        }),
        _ => None,
    }
}

/// Returns the name that `token` gives a member when it follows `.`, or the
/// text of the string literal `token` when it is the key of an element
/// access, as `A` in `E.A` and in `E["A"]`.
pub(crate) fn member_key(token: &JsSyntaxToken) -> TokenText {
    if token.kind() == JsSyntaxKind::JS_STRING_LITERAL {
        inner_string_text(token)
    } else {
        token.token_text_trimmed()
    }
}

/// Returns the range of the name of the member of `declaration` named
/// `name`, which is also the range of the member's binding, or `None` when
/// the enum has no member of that name written as an identifier or a string
/// literal.
pub(crate) fn enum_member_name_range(
    declaration: &TsEnumDeclaration,
    name: &str,
) -> Option<TextRange> {
    declaration.members().iter().find_map(|member| {
        let AnyTsEnumMemberName::TsLiteralEnumMemberName(member_name) = member.ok()?.name().ok()?
        else {
            return None;
        };
        let token = member_name.value().ok()?;
        (member_key(&token).text() == name).then(|| member_name.range())
    })
}

/// Returns the unit value that `binding` holds, which is also the unit value
/// of a read of `binding`, or `None` when it holds none that Biome compares
/// with.
pub(crate) fn binding_unit_value(binding: &Binding, model: &SemanticModel) -> Option<UnitValue> {
    match held_value(binding)? {
        HeldValue::Imported => Some(UnitValue::Imported {
            binding: binding.range(),
            member: None,
        }),
        HeldValue::Initializer(initializer) => {
            let mut remaining = MAX_CONDITION_STEPS;
            unit_value(initializer, model, &mut remaining)
        }
    }
}

/// Where the value of a variable that a comparison names comes from.
enum HeldValue {
    /// A named or a default import binds the variable.
    Imported,
    /// The variable is a `const` without a type annotation, declared with
    /// this initializer.
    Initializer(AnyJsExpression),
}

#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "Only `const` declarators and value imports hold a value that a comparison names."
)]
fn held_value(binding: &Binding) -> Option<HeldValue> {
    match binding.tree().declaration()? {
        AnyJsBindingDeclaration::JsVariableDeclarator(declarator) => {
            const_initializer(&declarator).map(HeldValue::Initializer)
        }
        AnyJsBindingDeclaration::JsNamedImportSpecifier(_)
        | AnyJsBindingDeclaration::JsShorthandNamedImportSpecifier(_)
        | AnyJsBindingDeclaration::JsDefaultImportSpecifier(_) => Some(HeldValue::Imported),
        _ => None,
    }
}

/// Returns the unit value of `expression` without following variables: `null`,
/// a string, number, bigint, or boolean literal, `-` or `+` directly before a
/// number literal, a template literal without a tag and without
/// substitutions, or `void`.
#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "Only literals and `void` have a unit value written in place."
)]
fn literal_unit_value(expression: &AnyJsExpression) -> Option<UnitValue> {
    match expression {
        AnyJsExpression::AnyJsLiteralExpression(literal) => {
            literal.value_token().ok()?;
            match literal {
                AnyJsLiteralExpression::JsNullLiteralExpression(_) => Some(UnitValue::Null),
                AnyJsLiteralExpression::JsBooleanLiteralExpression(_)
                | AnyJsLiteralExpression::JsStringLiteralExpression(_)
                | AnyJsLiteralExpression::JsNumberLiteralExpression(_)
                | AnyJsLiteralExpression::JsBigintLiteralExpression(_) => {
                    Some(UnitValue::Literal(literal.range()))
                }
                AnyJsLiteralExpression::JsRegexLiteralExpression(_) => None,
            }
        }
        AnyJsExpression::JsUnaryExpression(unary) => {
            let operator = unary.operator().ok()?;
            if operator == JsUnaryOperator::Void {
                return Some(UnitValue::Undefined);
            }
            let AnyJsExpression::AnyJsLiteralExpression(argument) = unary.argument().ok()? else {
                return None;
            };
            argument.value_token().ok()?;
            (matches!(operator, JsUnaryOperator::Minus | JsUnaryOperator::Plus)
                && matches!(
                    argument,
                    AnyJsLiteralExpression::JsNumberLiteralExpression(_)
                ))
            .then(|| UnitValue::Literal(unary.range()))
        }
        AnyJsExpression::JsTemplateExpression(template) => {
            is_plain_template(template).then(|| UnitValue::Literal(template.range()))
        }
        _ => None,
    }
}

/// Returns whether `template` has no tag and no substitutions, as in
/// `` `a` ``, so its value is the text between its backticks.
fn is_plain_template(template: &JsTemplateExpression) -> bool {
    let mut elements = template.elements().iter();
    template.tag().is_none()
        && match (elements.next(), elements.next()) {
            (None, _) => true,
            (Some(AnyJsTemplateElement::JsTemplateChunkElement(_)), None) => true,
            (Some(_), _) => false,
        }
}

/// Returns the text between the backticks of `template` when it has no tag,
/// no substitutions, and some text, as `string` for `` `string` ``. Escapes
/// stay as they are written, so a text with an escape names no kind of value.
fn plain_template_text(template: &JsTemplateExpression) -> Option<TokenText> {
    if !is_plain_template(template) {
        return None;
    }
    let AnyJsTemplateElement::JsTemplateChunkElement(chunk) = template.elements().iter().next()?
    else {
        return None;
    };
    Some(chunk.template_chunk_token().ok()?.token_text_trimmed())
}

/// Returns whether `expression` can be the operand of a comparison: an
/// expression with the shape of a subject, or a call in an optional chain,
/// such as `o?.f()`, whose receivers the comparison can narrow.
///
/// Returns `None` when the identifier, the object, or the member is missing.
fn has_operand_shape(expression: &AnyJsExpression) -> Option<bool> {
    Some(
        has_subject_shape(expression)?
            || matches!(expression, AnyJsExpression::JsCallExpression(call) if call.is_optional_chain()),
    )
}

/// Returns whether `expression` has the shape of a subject: an identifier or a
/// member access. A member access is a subject only when its object is one too
/// and, for an element access, its key spells the property's name, which
/// [`subject_of`] checks.
///
/// Returns `None` when the identifier, the object, or the member is missing.
#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "Only identifiers and member accesses can be subjects."
)]
fn has_subject_shape(expression: &AnyJsExpression) -> Option<bool> {
    match expression {
        AnyJsExpression::JsIdentifierExpression(identifier) => {
            identifier.name().ok()?.value_token().ok()?;
            Some(true)
        }
        AnyJsExpression::JsStaticMemberExpression(member) => {
            member.object().ok()?;
            member.member().ok()?;
            Some(true)
        }
        AnyJsExpression::JsComputedMemberExpression(member) => {
            member.object().ok()?;
            member.member().ok()?;
            Some(true)
        }
        _ => Some(false),
    }
}

/// Returns the receivers of `operand` when it is part of an optional chain:
/// the objects and callees that the chain evaluates before `operand`,
/// outermost first, down to the object of its innermost `?.`. They are `o.f`
/// and `o` in `o?.f()`, and `a.b?.c` and `a.b`, but not `a`, in `a.b?.c.d`.
/// The chain gives `undefined` when one of them is nullish. An expression
/// outside an optional chain has none.
///
/// Parentheses end a chain, so `(o)?.v` has the receiver `(o)`, and
/// `(o?.v).w` is not part of an optional chain.
///
/// Returns `None` when the chain starts at a parenthesized optional chain, as
/// `(o?.v)?.w` and `(o?.v).w` do, and when the walk runs out of work or meets
/// missing syntax. A test of such an operand narrows nothing. TypeScript
/// narrows `o.v` there, and the paths that extend it, but not `o`, so its
/// reads through `o?.v` add `undefined` back from `o`. A read in Biome keeps
/// the narrowed type of its path, so narrowing `o.v` alone would leave that
/// `undefined` out. When the test is the truthiness of a condition,
/// TypeScript narrows `o` as well, and Biome is deliberately wider.
fn chain_receivers(
    operand: &AnyJsExpression,
    remaining: &mut usize,
) -> Option<Vec<AnyJsExpression>> {
    let mut receivers = Vec::new();
    // The number of receivers down to the object of the innermost `?.`.
    let mut in_chain = 0;
    let mut expression = operand.clone();
    while let Some(link) = AnyJsOptionalChainExpression::cast_ref(expression.syntax()) {
        *remaining = remaining.checked_sub(1)?;
        let object = link.object().ok()?;
        receivers.push(object.clone());
        if link.is_optional() {
            in_chain = receivers.len();
        }
        expression = object;
    }
    if matches!(expression, AnyJsExpression::JsParenthesizedExpression(_))
        && omit_parentheses(expression, remaining)?.is_optional_chain()
    {
        return None;
    }
    receivers.truncate(in_chain);
    Some(receivers)
}

/// Returns whether `callee` is a dotted name, which TypeScript requires of the
/// callee of a call point: an identifier or `this`, followed by property
/// accesses with `.` or `?.`, in parentheses or not, such as `a.b.c`,
/// `(a).b`, or `this.m`.
///
/// Returns false when the walk runs out of work.
fn is_dotted_name(callee: &AnyJsExpression, remaining: &mut usize) -> bool {
    let mut expression = callee.clone();
    loop {
        let Some(left) = remaining.checked_sub(1) else {
            return false;
        };
        *remaining = left;
        expression = match expression {
            AnyJsExpression::JsIdentifierExpression(_) | AnyJsExpression::JsThisExpression(_) => {
                return true;
            }
            AnyJsExpression::JsParenthesizedExpression(parenthesized) => {
                match parenthesized.expression() {
                    Ok(expression) => expression,
                    Err(_) => return false,
                }
            }
            AnyJsExpression::JsStaticMemberExpression(member) => match member.object() {
                Ok(object) => object,
                Err(_) => return false,
            },
            _ => return false,
        };
    }
}

/// Returns how the reading context confirms that `callee`, the callee of a
/// statement call without parentheses, follows TypeScript's explicit-type
/// rule, with the ranges of its member accesses and whether the return type
/// of a callee that is a name itself comes from an annotation. These are the
/// fields of [`FlowCallKind::Statement`].
///
/// The callee must be a dotted name, as [`is_dotted_name`] describes, and the
/// name that it starts at must have an explicit type, as [`explicit_name`]
/// checks for a variable. `this` has one in a method, an accessor, a
/// constructor, or a field of a class, unless it is `static`, and in a
/// function with a `this` parameter, as [`explicit_this`] checks. Returns
/// `None` otherwise, because TypeScript then ignores an assertion (TS2775).
fn statement_callee(
    callee: &AnyJsExpression,
    model: &SemanticModel,
    remaining: &mut usize,
) -> Option<(FlowCallRoot, Box<[TextRange]>, bool)> {
    let mut steps = Vec::new();
    let mut expression = callee.clone();
    let mut annotated = false;
    let root = loop {
        *remaining = remaining.checked_sub(1)?;
        expression = match expression {
            AnyJsExpression::JsParenthesizedExpression(parenthesized) => {
                parenthesized.expression().ok()?
            }
            AnyJsExpression::JsStaticMemberExpression(member) => {
                steps.push(member.range());
                member.object().ok()?
            }
            AnyJsExpression::JsIdentifierExpression(identifier) => {
                let root = explicit_name(&identifier, !steps.is_empty(), model)?;
                // A name that no scope declares is a global, which the lib or
                // a declaration file declares with annotations.
                annotated = steps.is_empty()
                    && identifier
                        .name()
                        .ok()
                        .and_then(|name| model.binding(&name))
                        .is_none_or(|binding| {
                            declaration_return(&binding) == CalleeReturn::Annotated
                        });
                break root;
            }
            AnyJsExpression::JsThisExpression(this) => {
                if !explicit_this(&this) {
                    return None;
                }
                break FlowCallRoot::Explicit;
            }
            _ => return None,
        };
    };
    steps.reverse();
    Some((root, steps.into_boxed_slice(), annotated))
}

/// Returns whether `expression`, an argument of an assertion, is always
/// false, as TypeScript reads it: the literal `false`, `a && b` when either
/// operand is, and `a || b` when both are, also inside parentheses. An
/// `asserts value` signature never returns for such an argument. Returns
/// false when the walk runs out of work.
pub(super) fn is_false_expression(expression: &AnyJsExpression, remaining: &mut usize) -> bool {
    let Some(left) = remaining.checked_sub(1) else {
        return false;
    };
    *remaining = left;
    match expression {
        AnyJsExpression::JsParenthesizedExpression(parenthesized) => parenthesized
            .expression()
            .is_ok_and(|inner| is_false_expression(&inner, remaining)),
        AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsBooleanLiteralExpression(literal),
        ) => literal
            .value_token()
            .is_ok_and(|token| token.kind() == T![false]),
        AnyJsExpression::JsLogicalExpression(logical) => {
            let (Ok(left), Ok(right)) = (logical.left(), logical.right()) else {
                return false;
            };
            match logical.operator() {
                Ok(JsLogicalOperator::LogicalAnd) => {
                    is_false_expression(&left, remaining) || is_false_expression(&right, remaining)
                }
                Ok(JsLogicalOperator::LogicalOr) => {
                    is_false_expression(&left, remaining) && is_false_expression(&right, remaining)
                }
                Ok(JsLogicalOperator::NullishCoalescing) | Err(_) => false,
            }
        }
        _ => false,
    }
}

/// Returns how the reading context confirms that the variable that
/// `identifier` reads has an explicit type, or `None` when it has none.
/// TypeScript requires an explicit type of the name that the callee of a
/// statement call starts at. `has_steps` tells whether the callee accesses
/// members of the variable.
///
/// A function, a class, and a variable or a parameter with a type annotation
/// have one. So does a namespace that `declare` declares, because all its
/// members have explicit types, and a global, which the lib or a declaration
/// file declares. An import, including a namespace import, has one when the
/// declaration that it names has one, which the reading context checks (see
/// [`FlowCallRoot::Import`]). A variable without a type annotation, an
/// element of a destructuring pattern, and a namespace that can declare
/// members without types have none.
fn explicit_name(
    identifier: &JsIdentifierExpression,
    has_steps: bool,
    model: &SemanticModel,
) -> Option<FlowCallRoot> {
    let Some(binding) = model.binding(&identifier.name().ok()?) else {
        return Some(FlowCallRoot::Explicit);
    };
    let explicit = match binding.tree().declaration()? {
        AnyJsBindingDeclaration::JsNamedImportSpecifier(_)
        | AnyJsBindingDeclaration::JsShorthandNamedImportSpecifier(_)
        | AnyJsBindingDeclaration::JsDefaultImportSpecifier(_) => {
            return Some(FlowCallRoot::Import(binding.range()));
        }
        AnyJsBindingDeclaration::JsNamespaceImportSpecifier(_) => {
            return has_steps.then(|| FlowCallRoot::Import(binding.range()));
        }
        AnyJsBindingDeclaration::JsVariableDeclarator(declarator) => {
            declarator.variable_annotation().is_some()
        }
        AnyJsBindingDeclaration::JsFormalParameter(parameter) => {
            parameter.type_annotation().is_some()
        }
        AnyJsBindingDeclaration::JsRestParameter(parameter) => {
            parameter.type_annotation().is_some()
        }
        AnyJsBindingDeclaration::TsPropertyParameter(parameter) => {
            matches!(
                parameter.formal_parameter(),
                Ok(AnyJsFormalParameter::JsFormalParameter(parameter))
                    if parameter.type_annotation().is_some()
            )
        }
        AnyJsBindingDeclaration::JsFunctionDeclaration(_)
        | AnyJsBindingDeclaration::TsDeclareFunctionDeclaration(_)
        | AnyJsBindingDeclaration::JsFunctionExportDefaultDeclaration(_)
        | AnyJsBindingDeclaration::TsDeclareFunctionExportDefaultDeclaration(_)
        | AnyJsBindingDeclaration::JsClassDeclaration(_)
        | AnyJsBindingDeclaration::JsClassExportDefaultDeclaration(_) => true,
        AnyJsBindingDeclaration::TsModuleDeclaration(module) => {
            return (has_steps && module.parent::<TsDeclareStatement>().is_some())
                .then_some(FlowCallRoot::Namespace);
        }
        _ => false,
    };
    explicit.then_some(FlowCallRoot::Explicit)
}

/// Returns whether TypeScript gives the value that `this` reads an explicit
/// type, and Biome gives it the same type. That holds in a method, an
/// accessor, a constructor, or a field of a class, looking through arrow
/// functions, and in a function with a `this` parameter. It doesn't hold in
/// a `static` member or a static block, where Biome types `this` as an
/// instance, nor in an object literal method, where TypeScript's `this` has
/// no explicit type.
fn explicit_this(this: &JsThisExpression) -> bool {
    for node in this.syntax().ancestors().skip(1) {
        // The decorators and computed keys of a member run where the class is
        // created, with the `this` of the code around it.
        if JsDecorator::can_cast(node.kind()) || JsComputedMemberName::can_cast(node.kind()) {
            return false;
        }
        if let Some(member) = AnyJsClassMember::cast_ref(&node) {
            return !is_static_member(&member);
        }
        match AnyJsFunction::cast_ref(&node) {
            Some(AnyJsFunction::JsArrowFunctionExpression(_)) => {}
            Some(function) => {
                return function.parameters().is_ok_and(|parameters| {
                    matches!(
                        parameters,
                        AnyJsArrowFunctionParameters::JsParameters(parameters)
                            if matches!(
                                parameters.items().first(),
                                Some(Ok(AnyJsParameter::TsThisParameter(_)))
                            )
                    )
                });
            }
            None => {}
        }
        if AnyJsObjectMember::can_cast(node.kind()) || AnyJsControlFlowRoot::can_cast(node.kind()) {
            return false;
        }
    }
    false
}

/// Returns whether `member` is a `static` member of its class, or a static
/// block, whose `this` is the class itself.
fn is_static_member(member: &AnyJsClassMember) -> bool {
    match member {
        AnyJsClassMember::JsStaticInitializationBlockClassMember(_) => true,
        AnyJsClassMember::JsMethodClassMember(member) => member
            .modifiers()
            .iter()
            .any(|modifier| modifier.as_js_static_modifier().is_some()),
        AnyJsClassMember::JsGetterClassMember(member) => member
            .modifiers()
            .iter()
            .any(|modifier| modifier.as_js_static_modifier().is_some()),
        AnyJsClassMember::JsSetterClassMember(member) => member
            .modifiers()
            .iter()
            .any(|modifier| modifier.as_js_static_modifier().is_some()),
        AnyJsClassMember::JsPropertyClassMember(member) => member
            .modifiers()
            .iter()
            .any(|modifier| modifier.as_js_static_modifier().is_some()),
        _ => false,
    }
}

fn omit_parentheses(
    mut expression: AnyJsExpression,
    remaining: &mut usize,
) -> Option<AnyJsExpression> {
    while let AnyJsExpression::JsParenthesizedExpression(parenthesized) = expression {
        *remaining = remaining.checked_sub(1)?;
        parenthesized.l_paren_token().ok()?;
        parenthesized.r_paren_token().ok()?;
        expression = parenthesized.expression().ok()?;
    }
    let kind = expression.syntax().kind();
    (!kind.is_bogus() && !kind.is_metavariable()).then_some(expression)
}

#[cfg(test)]
mod tests {
    use super::super::{FlowSubject, PropertyKey};
    use super::*;
    use biome_js_parser::{JsParserOptions, parse};
    use biome_js_semantic::{SemanticModelOptions, semantic_model};
    use biome_js_syntax::JsIfStatement;
    use biome_languages::JsFileSource;

    /// A decomposed `if` condition, with the semantic model and the subject
    /// table that name its subjects.
    struct Decomposed {
        guards: Vec<FlowGuard>,
        condition: DecomposedCondition,
        model: SemanticModel,
        subjects: Vec<FlowSubject>,
    }

    /// Decomposes the `if` condition `source` in a function with the
    /// parameters `value` and `other`.
    fn decomposed(source: &str, outcome: FlowOutcome) -> Decomposed {
        decomposed_last(
            &format!("function f(value, other) {{ if ({source}) {{}} }}"),
            outcome,
        )
    }

    /// Decomposes the test of the last `if` statement in the module `program`.
    fn decomposed_last(program: &str, outcome: FlowOutcome) -> Decomposed {
        let parsed = parse(program, JsFileSource::ts(), JsParserOptions::default());
        let model = semantic_model(&parsed.tree(), SemanticModelOptions::default());
        let test = parsed
            .syntax()
            .descendants()
            .filter_map(JsIfStatement::cast)
            .last()
            .unwrap()
            .test()
            .unwrap();
        let mut guards = Vec::new();
        let raw_binding_types = FxHashMap::default();
        let mut functions = GuardFunctions::new(&[], &raw_binding_types);
        let mut subjects = FlowSubjectTable::default();
        let condition = decompose_condition(
            &test,
            outcome,
            &model,
            &mut functions,
            &mut subjects,
            &mut guards,
        );
        Decomposed {
            guards,
            condition,
            model,
            subjects: subjects.into_subjects(),
        }
    }

    /// Returns the kind of the first `switch` statement in `source`.
    fn switch_kind(source: &str) -> SwitchKind {
        let parsed = parse(source, JsFileSource::ts(), JsParserOptions::default());
        assert!(!parsed.has_errors(), "{source}");
        let statement = parsed
            .syntax()
            .descendants()
            .find_map(JsSwitchStatement::cast)
            .unwrap();
        SwitchKind::new(&statement)
    }

    #[test]
    fn switch_kinds_follow_the_discriminant() {
        for (source, kind) in [
            ("switch (x) { case 1: }", SwitchKind::Value),
            ("switch ((x)) { case 1: }", SwitchKind::Value),
            ("switch (o.k) { case 1: }", SwitchKind::Value),
            ("switch (o?.f()) { case 1: }", SwitchKind::Value),
            (
                "switch (typeof x) { case \"a\": default: }",
                SwitchKind::Typeof,
            ),
            ("switch ((typeof (x))) { case \"a\": }", SwitchKind::Typeof),
            ("switch (typeof x) { case k: }", SwitchKind::Opaque),
            ("switch (typeof x) { case (\"a\"): }", SwitchKind::Opaque),
            ("switch (typeof x) { case `a`: }", SwitchKind::Typeof),
            ("switch (typeof x) { case `${a}`: }", SwitchKind::Opaque),
            ("switch (typeof f()) { case \"a\": }", SwitchKind::Opaque),
            ("switch (true) { case x: }", SwitchKind::True),
            ("switch ((true)) { case x: }", SwitchKind::Opaque),
            ("switch (false) { case x: }", SwitchKind::Opaque),
            ("switch (x!) { case 1: }", SwitchKind::Opaque),
            ("switch (f()) { case 1: }", SwitchKind::Opaque),
        ] {
            assert_eq!(switch_kind(source), kind, "{source}");
        }
    }

    #[test]
    fn boolean_valued_expressions_are_true_whenever_they_are_truthy() {
        for (source, expected) in [
            ("a === b", true),
            ("a < b", true),
            ("a instanceof B", true),
            ("\"a\" in b", true),
            ("!a", true),
            ("f(a)", true),
            ("(a !== null)", true),
            ("a && b === 1", true),
            ("a === 1 || b === 2", true),
            ("a", false),
            ("a && b", false),
            ("a === 1 || b", false),
            ("a ?? b === 1", false),
            ("a + b", false),
        ] {
            let parsed = parse(source, JsFileSource::ts(), JsParserOptions::default());
            let expression = parsed
                .syntax()
                .descendants()
                .find_map(AnyJsExpression::cast)
                .unwrap();
            let mut remaining = MAX_CONDITION_STEPS;
            assert_eq!(
                is_boolean_valued(&expression, &mut remaining),
                expected,
                "{source}"
            );
        }
    }

    /// Decomposes the `if` condition `source` and names the mentioned subjects.
    fn decompose(source: &str, outcome: FlowOutcome) -> (Vec<FlowGuard>, Vec<String>) {
        let decomposed = decomposed(source, outcome);
        let names = decomposed
            .condition
            .mentions
            .iter()
            .map(|subject| subject_name(&decomposed.model, &decomposed.subjects, *subject))
            .collect();
        (decomposed.guards, names)
    }

    /// Returns the name of the variable that `subject` starts at, or `this`,
    /// followed by the names along its path, as in `value.length` or
    /// `this.#a`.
    fn subject_name(
        model: &SemanticModel,
        subjects: &[FlowSubject],
        subject: FlowSubjectId,
    ) -> String {
        match &subjects[subject as usize] {
            FlowSubject::Binding(range) => model
                .as_binding_by_range(*range)
                .unwrap()
                .tree()
                .name_token()
                .unwrap()
                .text_trimmed()
                .to_owned(),
            FlowSubject::This => "this".to_owned(),
            FlowSubject::Pattern(range) => model
                .root()
                .syntax()
                .covering_element(*range)
                .into_node()
                .unwrap()
                .text_trimmed()
                .to_string(),
            FlowSubject::Member {
                object,
                key: PropertyKey::Name(name),
            } => format!("{}.{name}", subject_name(model, subjects, *object)),
            FlowSubject::Member {
                object,
                key: PropertyKey::Private(name),
            } => format!("{}.#{name}", subject_name(model, subjects, *object)),
        }
    }

    fn mentioned_names(source: &str, outcome: FlowOutcome) -> Vec<String> {
        decompose(source, outcome).1
    }

    /// Names the subjects that the test of the last `if` statement in the
    /// module `program` mentions.
    fn mentioned_names_in(program: &str, outcome: FlowOutcome) -> Vec<String> {
        let decomposed = decomposed_last(program, outcome);
        decomposed
            .condition
            .mentions
            .iter()
            .map(|subject| subject_name(&decomposed.model, &decomposed.subjects, *subject))
            .collect()
    }

    /// Decomposes the `if` test `source` and names the subjects that its
    /// receiver tests check, outermost guard first, followed by the main
    /// guard that they wrap. The returned main guard leaves out the
    /// [`FlowGuard::Property`] guard that a test of a property adds.
    fn receiver_names(source: &str, outcome: FlowOutcome) -> (Vec<String>, FlowGuard) {
        let Decomposed {
            guards,
            condition,
            model,
            subjects,
        } = decomposed(source, outcome);
        let mut names = Vec::new();
        let mut guard = &guards[condition.guard];
        while let FlowGuard::Both {
            left,
            right,
            sequential: true,
        } = guard
        {
            if matches!(guards[*right], FlowGuard::Property { .. }) {
                guard = &guards[*left];
                break;
            }
            let FlowGuard::Test {
                subject,
                test: FlowTest::Nullish,
                positive: false,
            } = &guards[*left]
            else {
                break;
            };
            names.push(subject_name(&model, &subjects, *subject));
            guard = &guards[*right];
        }
        (names, guard.clone())
    }

    /// Decomposes the `if` test `source` and names the objects of its
    /// [`FlowGuard::Property`] guards with their keys, as `value[kind]`,
    /// prefixed with `?` when the test reads the key through an optional
    /// chain.
    fn property_names(source: &str, outcome: FlowOutcome) -> Vec<String> {
        let Decomposed {
            guards,
            model,
            subjects,
            ..
        } = decomposed(source, outcome);
        guards
            .iter()
            .filter_map(|guard| match guard {
                FlowGuard::Property {
                    object,
                    key: PropertyKey::Name(key),
                    optional_chain,
                    ..
                } => Some(format!(
                    "{}{}[{key}]",
                    if *optional_chain { "?" } else { "" },
                    subject_name(&model, &subjects, *object)
                )),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn receiver_tests_follow_the_kind_of_test() {
        use FlowOutcome::{Falsy, NonNullish, Nullish, Truthy};
        for (source, outcome, expected) in [
            ("value?.a", Truthy, vec!["value"]),
            ("value?.a", Falsy, vec![]),
            ("!value?.a", Falsy, vec!["value"]),
            // TypeScript narrows no receiver for the outcomes of `??`.
            ("value?.a", Nullish, vec![]),
            ("value?.a", NonNullish, vec![]),
            ("value?.f()", Truthy, vec!["value", "value.f"]),
            ("value?.f()", Falsy, vec![]),
            ("value?.a === 1", Truthy, vec!["value"]),
            ("value?.a === 1", Falsy, vec![]),
            ("value?.a !== 'x'", Truthy, vec![]),
            ("value?.a !== 'x'", Falsy, vec!["value"]),
            ("value?.a == true", Truthy, vec!["value"]),
            ("value?.a != 1n", Truthy, vec![]),
            ("value?.a != 1n", Falsy, vec!["value"]),
            ("value?.a === null", Truthy, vec!["value"]),
            ("value?.a !== null", Truthy, vec![]),
            ("value?.a == null", Truthy, vec![]),
            ("null != value?.a", Truthy, vec!["value"]),
            ("value?.a === undefined", Truthy, vec![]),
            ("value?.a === undefined", Falsy, vec!["value"]),
            ("value?.a != undefined", Truthy, vec!["value"]),
            ("typeof value?.a === 'string'", Truthy, vec!["value"]),
            ("typeof value?.a === 'string'", Falsy, vec![]),
            ("typeof value?.a == 'undefined'", Truthy, vec![]),
            ("typeof value?.a !== 'undefined'", Truthy, vec!["value"]),
            ("value?.f() === true", Truthy, vec!["value", "value.f"]),
            (
                "typeof value?.f() === 'boolean'",
                Truthy,
                vec!["value", "value.f"],
            ),
            ("value?.a === other", Truthy, vec![]),
            ("value?.a === -1", Truthy, vec!["value"]),
            ("value?.a === -1", Falsy, vec![]),
            ("value?.a === `a`", Truthy, vec!["value"]),
            ("value?.a !== void 0", Truthy, vec!["value"]),
            ("value?.a === void 0", Truthy, vec![]),
            ("value?.a instanceof other", Truthy, vec!["value"]),
            ("value?.a instanceof other", Falsy, vec![]),
            ("!(value?.a instanceof other)", Falsy, vec!["value"]),
            ("value.a", Truthy, vec![]),
        ] {
            assert_eq!(receiver_names(source, outcome).0, expected, "{source}");
        }
    }

    #[test]
    fn receiver_tests_walk_the_optional_chain() {
        for (source, expected) in [
            ("value?.a?.b", vec!["value", "value.a"]),
            ("value?.a.b", vec!["value", "value.a"]),
            // `value.a` is evaluated before the chain starts.
            ("value.a?.b.c", vec!["value.a", "value.a.b"]),
            ("value?.f()?.a", vec!["value", "value.f"]),
            ("value()?.a", vec![]),
            ("value?.[other]", vec!["value"]),
            ("value?.[0]?.['a']", vec!["value", "value.0"]),
            // Parentheses end the chain.
            ("(value)?.a", vec!["value"]),
            ("((value).a)?.b", vec!["value.a"]),
            ("value?.a!.b", vec![]),
        ] {
            assert_eq!(
                receiver_names(source, FlowOutcome::Truthy).0,
                expected,
                "{source}"
            );
        }
    }

    #[test]
    fn receiver_tests_wrap_the_main_test() {
        let (receivers, main) = receiver_names("value?.a === 1", FlowOutcome::Truthy);
        assert_eq!(receivers, ["value"]);
        assert!(matches!(
            main,
            FlowGuard::Test {
                test: FlowTest::Literal { strict: true, .. },
                positive: true,
                ..
            }
        ));
        assert_eq!(
            mentioned_names("value?.a === 1", FlowOutcome::Truthy),
            ["value.a", "value"]
        );
        // Loose equality with a literal tests the end of the chain and its
        // receivers too.
        let (receivers, main) = receiver_names("value?.a == 1", FlowOutcome::Truthy);
        assert_eq!(receivers, ["value"]);
        assert!(matches!(
            main,
            FlowGuard::Test {
                test: FlowTest::Literal { strict: false, .. },
                positive: true,
                ..
            }
        ));
        assert_eq!(
            mentioned_names("value?.a == 1", FlowOutcome::Truthy),
            ["value.a", "value"]
        );
        // A call of a function that the module doesn't declare is a call
        // point, whose effect inference resolves.
        let (receivers, main) = receiver_names("value?.f()", FlowOutcome::Truthy);
        assert_eq!(receivers, ["value", "value.f"]);
        assert!(matches!(main, FlowGuard::Call { positive: true, .. }));
    }

    #[test]
    fn tests_of_named_properties_also_test_their_object() {
        use FlowOutcome::{Falsy, NonNullish, Truthy};
        for (source, outcome, expected) in [
            ("value.kind === 'a'", Truthy, vec!["value[kind]"]),
            ("value.kind !== 'a'", Falsy, vec!["value[kind]"]),
            ("value['kind'] === 1", Truthy, vec!["value[kind]"]),
            ("value[0] === 'a'", Truthy, vec!["value[0]"]),
            ("value.ok", Truthy, vec!["value[ok]"]),
            ("!value.ok", Truthy, vec!["value[ok]"]),
            ("value.ok", NonNullish, vec!["value[ok]"]),
            ("value.kind === null", Truthy, vec!["value[kind]"]),
            ("value.kind == null", Truthy, vec!["value[kind]"]),
            ("value.kind !== undefined", Truthy, vec!["value[kind]"]),
            (
                "typeof value.kind === 'string'",
                Truthy,
                vec!["value[kind]"],
            ),
            ("value.a.kind === 'a'", Truthy, vec!["value.a[kind]"]),
            ("value?.kind === 'a'", Truthy, vec!["?value[kind]"]),
            ("value?.a.kind === 'a'", Truthy, vec!["?value.a[kind]"]),
            ("(value).kind === 'a'", Truthy, vec!["value[kind]"]),
            (
                "value.kind === 'a' && other.ok",
                Truthy,
                vec!["value[kind]", "other[ok]"],
            ),
            ("value.kind == 'a'", Truthy, vec!["value[kind]"]),
            ("value.kind === other", Truthy, vec![]),
            ("value[other] === 'a'", Truthy, vec![]),
            ("this.kind === 'a'", Truthy, vec![]),
            ("value", Truthy, vec![]),
            ("check(value.kind)", Truthy, vec![]),
        ] {
            assert_eq!(property_names(source, outcome), expected, "{source}");
        }
        // The object is a parent, not a mention, unless a test checks it.
        let decomposed = decomposed("value.kind === 'a'", Truthy);
        let parents: Vec<_> = decomposed
            .condition
            .parents
            .iter()
            .map(|subject| subject_name(&decomposed.model, &decomposed.subjects, *subject))
            .collect();
        assert_eq!(parents, ["value"]);
        assert_eq!(
            mentioned_names("value.kind === 'a'", Truthy),
            ["value.kind"]
        );
        assert_eq!(
            mentioned_names("value?.kind === 'a'", Truthy),
            ["value.kind", "value"]
        );
    }

    /// Decomposes the test of the last `if` statement in the module
    /// `program` and names the objects of its [`FlowGuard::Property`] guards
    /// with their keys, as `value[kind]`. A name starts with `?` when the test
    /// reads the key through an optional chain, and ends with `~` when the
    /// guard's `aliased` field is true.
    fn property_names_in(program: &str, outcome: FlowOutcome) -> Vec<String> {
        let Decomposed {
            guards,
            model,
            subjects,
            ..
        } = decomposed_last(program, outcome);
        let readonly = readonly_guards(&guards);
        guards
            .iter()
            .enumerate()
            .filter_map(|(index, guard)| match guard {
                FlowGuard::Property {
                    object,
                    key: PropertyKey::Name(key),
                    optional_chain,
                    aliased,
                    ..
                } => Some(format!(
                    "{}{}{}[{key}]{}",
                    if readonly.contains(&index) {
                        "readonly "
                    } else {
                        ""
                    },
                    if *optional_chain { "?" } else { "" },
                    subject_name(&model, &subjects, *object),
                    if *aliased { "~" } else { "" },
                )),
                _ => None,
            })
            .collect()
    }

    /// Returns the indices of the guards that a [`FlowGuard::ReadonlyPath`]
    /// in `guards` wraps.
    fn readonly_guards(guards: &[FlowGuard]) -> Vec<usize> {
        guards
            .iter()
            .filter_map(|guard| match guard {
                FlowGuard::ReadonlyPath(inner) => Some(*inner),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn tests_of_destructured_elements_test_their_pattern() {
        for (program, expected) in [
            (
                "function f(value) { const { kind, v } = value; if (kind === 'a') {} }",
                vec!["value[kind]~", "{ kind, v }[kind]"],
            ),
            (
                "function f({ kind, v }) { if (kind === 'a') {} }",
                vec!["{ kind, v }[kind]"],
            ),
            (
                "function f({ kind: k, v }) { if (k === 'a') {} }",
                vec!["{ kind: k, v }[kind]"],
            ),
            (
                "function f(...[k, x]) { if (x === 1) {} }",
                vec!["[k, x][1]"],
            ),
            (
                "function f(t) { const [k, , x] = t; if (x === 'a') {} }",
                vec!["t[2]~", "[k, , x][2]"],
            ),
            (
                "function f(w) { const { a: { kind, v } } = w; if (kind === 'a') {} }",
                vec!["{ kind, v }[kind]"],
            ),
            (
                "function f(w) { const [{ kind, v }] = w; if (kind === 'a') {} }",
                vec!["{ kind, v }[kind]"],
            ),
            (
                "function f(value) { for (const { kind, v } of value) { if (kind === 'a') {} } }",
                vec!["{ kind, v }[kind]"],
            ),
            (
                "function f({ kind, v } = d) { if (kind === 'a') {} }",
                vec!["{ kind, v }[kind]"],
            ),
            // A pattern's values don't change, so its guard is never aliased.
            (
                "function f({ kind, v }) { const ok = kind === 'a'; if (ok) {} }",
                vec!["{ kind, v }[kind]"],
            ),
            (
                "function f({ kind, v }) { if (typeof kind === 'string' || !v) {} }",
                vec!["{ kind, v }[kind]", "{ kind, v }[v]"],
            ),
            // The pattern needs two elements without a default that are not
            // rest elements.
            (
                "function f(value) { const { kind, v = 1 } = value; if (kind === 'a') {} }",
                vec!["value[kind]~"],
            ),
            (
                "function f(value) { const { kind, ...rest } = value; if (kind === 'a') {} }",
                vec!["value[kind]~"],
            ),
            (
                "function f(value) { const { kind = 'a', v } = value; if (kind === 'a') {} }",
                vec![],
            ),
            (
                "function f(value) { let { kind, v } = value; if (kind === 'a') {} }",
                vec![],
            ),
            (
                "function f(value) { var { kind, v } = value; if (kind === 'a') {} }",
                vec![],
            ),
            (
                "function f({ kind, v }) { v = 1; if (kind === 'a') {} }",
                vec![],
            ),
            (
                "function f({ kind, inner: [v] }) { v = 1; if (kind === 'a') {} }",
                vec![],
            ),
            (
                "function f(w) { const { a: { kind, v } = d } = w; if (kind === 'a') {} }",
                vec![],
            ),
            (
                "function f() { try {} catch ({ kind, v }) { if (kind === 'a') {} } }",
                vec![],
            ),
        ] {
            assert_eq!(
                property_names_in(program, FlowOutcome::Truthy),
                expected,
                "{program}"
            );
        }
    }

    #[test]
    fn tests_of_constants_that_hold_a_property_test_its_object() {
        use FlowOutcome::{Falsy, Truthy};
        for (declaration, test, outcome, expected) in [
            (
                "const k = value.kind;",
                "k === 'a'",
                Truthy,
                vec!["value[kind]~"],
            ),
            (
                "const k = value.kind;",
                "k !== 'a'",
                Falsy,
                vec!["value[kind]~"],
            ),
            (
                "const k = value?.kind;",
                "k === 'a'",
                Truthy,
                vec!["?value[kind]~"],
            ),
            (
                "const k = value['kind'];",
                "k === 'a'",
                Truthy,
                vec!["value[kind]~"],
            ),
            (
                "const k = value[0];",
                "k === 'a'",
                Truthy,
                vec!["value[0]~"],
            ),
            (
                "const k = (value.kind);",
                "k === 'a'",
                Truthy,
                vec!["value[kind]~"],
            ),
            (
                "const k = (value).kind;",
                "k === 'a'",
                Truthy,
                vec!["value[kind]~"],
            ),
            (
                "const k = value.a.kind;",
                "k === 'a'",
                Truthy,
                vec!["value.a[kind]~"],
            ),
            (
                "const k = value?.a.kind;",
                "k === 'a'",
                Truthy,
                vec!["?value.a[kind]~"],
            ),
            (
                "const k = value.kind;",
                "typeof k === 'string'",
                Truthy,
                vec!["value[kind]~"],
            ),
            (
                "const k = value.kind;",
                "k === null",
                Truthy,
                vec!["value[kind]~"],
            ),
            (
                "const k = value.kind;",
                "k == undefined",
                Truthy,
                vec!["value[kind]~"],
            ),
            (
                "const k = value.kind;",
                "'a' === k",
                Truthy,
                vec!["value[kind]~"],
            ),
            (
                "const { kind } = value;",
                "kind === 'a'",
                Truthy,
                vec!["value[kind]~"],
            ),
            (
                "const { kind: k } = value;",
                "k === 'a'",
                Truthy,
                vec!["value[kind]~"],
            ),
            (
                "const { kind: k, other: o = 1 } = value;",
                "k === 'a'",
                Truthy,
                vec!["value[kind]~"],
            ),
            (
                "const { 'kind': k } = value;",
                "k === 'a'",
                Truthy,
                vec!["value[kind]~"],
            ),
            (
                "const { ['kind']: k } = value;",
                "k === 'a'",
                Truthy,
                vec!["value[kind]~"],
            ),
            (
                "const { 0: k } = value;",
                "k === 'a'",
                Truthy,
                vec!["value[0]~"],
            ),
            (
                "const { type: k } = value;",
                "k === 'a'",
                Truthy,
                vec!["value[type]~"],
            ),
            (
                "const { default: k } = value;",
                "k === 'a'",
                Truthy,
                vec!["value[default]~"],
            ),
            (
                "const { type } = value;",
                "type === 'a'",
                Truthy,
                vec!["value[type]~"],
            ),
            (
                "const { \\u006bind } = value;",
                "kind === 'a'",
                Truthy,
                vec!["value[kind]~"],
            ),
            (
                "const { \\u006bind: k } = value;",
                "k === 'a'",
                Truthy,
                vec!["value[kind]~"],
            ),
            (
                "const { kind } = (value);",
                "kind === 'a'",
                Truthy,
                vec!["value[kind]~"],
            ),
            (
                "const { kind } = value.a;",
                "kind === 'a'",
                Truthy,
                vec!["value.a[kind]~"],
            ),
            (
                "const { kind } = value['a'];",
                "kind === 'a'",
                Truthy,
                vec!["value.a[kind]~"],
            ),
            ("const [k] = value;", "k === 'a'", Truthy, vec!["value[0]~"]),
            (
                "const [, k] = value;",
                "k === 'a'",
                Truthy,
                vec!["value[1]~"],
            ),
            (
                "const [k, ...rest] = value;",
                "k === 'a'",
                Truthy,
                vec!["value[0]~"],
            ),
            // A truthiness test also inlines the initializer as an aliased
            // condition, whose test of the property adds a guard of its own.
            (
                "const k = value.kind;",
                "k",
                Truthy,
                vec!["value[kind]~", "value[kind]~"],
            ),
            (
                "const { kind } = value;",
                "kind",
                Truthy,
                vec!["value[kind]~"],
            ),
            // Nothing else holds a property.
            ("const k: string = value.kind;", "k === 'a'", Truthy, vec![]),
            ("let k = value.kind;", "k === 'a'", Truthy, vec![]),
            (
                "const k = value.kind as string;",
                "k === 'a'",
                Truthy,
                vec![],
            ),
            ("const k = value!.kind;", "k === 'a'", Truthy, vec![]),
            ("const k = value.#kind;", "k === 'a'", Truthy, vec![]),
            ("const k = value[other];", "k === 'a'", Truthy, vec![]),
            (
                "const k = value['\\u006bind'];",
                "k === 'a'",
                Truthy,
                vec![],
            ),
            ("const k = value[0x0];", "k === 'a'", Truthy, vec![]),
            ("const k = (value?.a)?.kind;", "k === 'a'", Truthy, vec![]),
            ("const k = this.kind;", "k === 'a'", Truthy, vec![]),
            ("const k = value;", "k === 'a'", Truthy, vec![]),
            (
                "const k = value.kind;",
                "k == 'a'",
                Truthy,
                vec!["value[kind]~"],
            ),
            ("const k = value.kind;", "check(k)", Truthy, vec![]),
            (
                "const { kind = 'a' } = value;",
                "kind === 'a'",
                Truthy,
                vec![],
            ),
            (
                "const { kind: k = 'a' } = value;",
                "k === 'a'",
                Truthy,
                vec![],
            ),
            ("const [k = 'a'] = value;", "k === 'a'", Truthy, vec![]),
            ("const { ...kind } = value;", "kind", Truthy, vec![]),
            ("const [...k] = value;", "k", Truthy, vec![]),
            (
                "const { a: { kind } } = value;",
                "kind === 'a'",
                Truthy,
                vec![],
            ),
            ("const [{ kind }] = value;", "kind === 'a'", Truthy, vec![]),
            (
                "const { kind }: { kind: string } = value;",
                "kind === 'a'",
                Truthy,
                vec![],
            ),
            ("let { kind } = value;", "kind === 'a'", Truthy, vec![]),
            ("const { kind } = value!;", "kind === 'a'", Truthy, vec![]),
            (
                "const { kind } = value as any;",
                "kind === 'a'",
                Truthy,
                vec![],
            ),
            (
                "const { kind } = check(value);",
                "kind === 'a'",
                Truthy,
                vec![],
            ),
            ("const { kind } = this;", "kind === 'a'", Truthy, vec![]),
            (
                "const { '\\u006bind': k } = value;",
                "k === 'a'",
                Truthy,
                vec![],
            ),
            ("const { 0x0: k } = value;", "k === 'a'", Truthy, vec![]),
            (
                "const { [`kind`]: k } = value;",
                "k === 'a'",
                Truthy,
                vec![],
            ),
        ] {
            let program = format!(
                "declare function check(v: unknown): v is string; \
                function f(value, other) {{ {declaration} if ({test}) {{}} }}"
            );
            assert_eq!(
                property_names_in(&program, outcome),
                expected,
                "{declaration} {test}"
            );
        }
        for program in [
            "const value = get(); export const k = value.kind; if (k === 'a') {}",
            "const value = get(); const k = value.kind; export { k }; if (k === 'a') {}",
            "const value = get(); export const { kind } = value; if (kind === 'a') {}",
            "function f(value) { for (const { kind } of value) { if (kind === 'a') {} } }",
            "function f({ kind }) { if (kind === 'a') {} }",
        ] {
            assert_eq!(
                property_names_in(program, Truthy),
                Vec::<String>::new(),
                "{program}"
            );
        }
    }

    #[test]
    fn held_properties_narrow_through_aliased_conditions_objects_that_cannot_change() {
        for (program, expected) in [
            (
                "function f(value) { const k = value.kind; const ok = k === 'a'; if (ok) {} }",
                vec!["value[kind]~"],
            ),
            (
                "function f(value) { const { kind } = value; const ok = kind === 'a'; if (ok) {} }",
                vec!["value[kind]~"],
            ),
            (
                "function f(value) { const ok = value.kind === 'a'; if (ok) {} }",
                vec!["value[kind]~"],
            ),
            // A property path can change between the `const` and the test
            // unless it is readonly.
            (
                "function f(value) { const k = value.a.kind; const ok = k === 'a'; if (ok) {} }",
                vec!["readonly value.a[kind]~"],
            ),
            (
                "function f(value) { const { kind } = value.a; const ok = kind === 'a'; if (ok) {} }",
                vec!["readonly value.a[kind]~"],
            ),
            // Outside aliased conditions, a property path holds its property.
            (
                "function f(value) { const k = value.a.kind; if (k === 'a') {} }",
                vec!["value.a[kind]~"],
            ),
            // A test of the property itself is not held.
            (
                "function f(value) { if (value.kind === 'a') {} }",
                vec!["value[kind]"],
            ),
        ] {
            assert_eq!(
                property_names_in(program, FlowOutcome::Truthy),
                expected,
                "{program}"
            );
        }
    }

    #[test]
    fn held_properties_of_patterns_are_subjects_for_the_write_scan() {
        let Decomposed {
            subjects, model, ..
        } = decomposed_last(
            "function f(value) { const { kind } = value; if (kind === 'a') {} }",
            FlowOutcome::Truthy,
        );
        let names: Vec<_> = (0..)
            .zip(&subjects)
            .map(|(id, _)| subject_name(&model, &subjects, id))
            .collect();
        assert!(names.contains(&"value.kind".to_owned()), "{names:?}");
    }

    /// Names the subjects in [`DecomposedCondition::parents`] of the
    /// decomposed condition.
    fn parent_names(decomposed: &Decomposed) -> Vec<String> {
        decomposed
            .condition
            .parents
            .iter()
            .map(|subject| subject_name(&decomposed.model, &decomposed.subjects, *subject))
            .collect()
    }

    #[test]
    fn in_tests_the_object_that_it_looks_into() {
        use FlowOutcome::{Falsy, Truthy};
        for (source, outcome, object, positive) in [
            (r#""a" in value"#, Truthy, "value", true),
            (r#""a" in value"#, Falsy, "value", false),
            (r#"!("a" in value)"#, Truthy, "value", false),
            ("0 in value", Truthy, "value", true),
            (r#"("a") in (value)"#, Truthy, "value", true),
            (r#""a" in value.v"#, Truthy, "value.v", true),
            // `in` narrows no receiver of an optional chain.
            (r#""a" in value?.v"#, Truthy, "value.v", true),
        ] {
            let decomposed = decomposed(source, outcome);
            // Like a test of a property, `in` narrows only a union with two
            // object types, so its subject is a parent. It narrows no object
            // that holds the tested value.
            assert!(decomposed.condition.mentions.is_empty(), "{source}");
            assert_eq!(parent_names(&decomposed), [object], "{source}");
            assert!(
                !decomposed
                    .guards
                    .iter()
                    .any(|guard| matches!(guard, FlowGuard::Property { .. })),
                "{source}"
            );
            let FlowGuard::Test {
                subject,
                test: FlowTest::In(key),
                positive: actual,
            } = decomposed.guards[decomposed.condition.guard]
            else {
                panic!("{source}: {:?}", decomposed.guards);
            };
            assert_eq!(
                subject_name(&decomposed.model, &decomposed.subjects, subject),
                object,
                "{source}"
            );
            assert_eq!(actual, positive, "{source}");
            let program = decomposed.model.root().syntax().text_trimmed().to_string();
            assert!(
                matches!(&program[key], r#""a""# | "0"),
                "{source}: {}",
                &program[key]
            );
        }
    }

    #[test]
    fn in_with_a_key_that_is_not_a_literal_name_tests_nothing() {
        for program in [
            r#"function f(value) { if (`a` in value) {} }"#,
            r#"function f(value) { if ("\u0061" in value) {} }"#,
            "function f(value) { if (0x0 in value) {} }",
            "function f(value) { if (1.0 in value) {} }",
            "function f(value) { if (-0 in value) {} }",
            r#"function f(value) { const k = "a"; if (k in value) {} }"#,
            r#"function f(value) { if ("a" in value!) {} }"#,
            r#"function f(value) { if ("a" in value.f()) {} }"#,
            // No class around the test declares `#q`.
            "class K { #p = 1; m(value) { if (#q in value) {} } }",
        ] {
            let decomposed = decomposed_last(program, FlowOutcome::Truthy);
            assert!(decomposed.condition.mentions.is_empty(), "{program}");
            assert!(decomposed.condition.parents.is_empty(), "{program}");
            assert_eq!(
                decomposed.guards[decomposed.condition.guard],
                FlowGuard::Keep,
                "{program}"
            );
        }
    }

    #[test]
    fn in_narrows_through_aliased_conditions() {
        let decomposed = decomposed_last(
            r#"function f(value) { const ok = "a" in value; if (ok) {} }"#,
            FlowOutcome::Truthy,
        );
        assert_eq!(parent_names(&decomposed), ["value"]);
    }

    #[test]
    fn instanceof_tests_its_left_operand() {
        use FlowOutcome::{Falsy, Truthy};
        for (source, outcome, subject, positive) in [
            ("value instanceof Date", Truthy, "value", true),
            ("value instanceof Date", Falsy, "value", false),
            ("!(value instanceof Date)", Truthy, "value", false),
            ("(value) instanceof (other)", Truthy, "value", true),
            ("value.v instanceof other.C", Truthy, "value.v", true),
            ("value[0] instanceof Date", Truthy, "value.0", true),
        ] {
            let decomposed = decomposed(source, outcome);
            // Like the object of a property test, the subject of `instanceof`
            // is a parent, because `instanceof` narrows only a type with an
            // object type or `unknown`. It narrows no object that holds the
            // tested value.
            assert!(decomposed.condition.mentions.is_empty(), "{source}");
            assert_eq!(parent_names(&decomposed), [subject], "{source}");
            let instance_tested: Vec<_> = decomposed
                .condition
                .instance_tested
                .iter()
                .map(|tested| subject_name(&decomposed.model, &decomposed.subjects, *tested))
                .collect();
            assert_eq!(instance_tested, [subject], "{source}");
            assert!(
                !decomposed
                    .guards
                    .iter()
                    .any(|guard| matches!(guard, FlowGuard::Property { .. })),
                "{source}"
            );
            let FlowGuard::Test {
                subject: tested,
                test: FlowTest::Instanceof(constructor),
                positive: actual,
            } = decomposed.guards[decomposed.condition.guard]
            else {
                panic!("{source}: {:?}", decomposed.guards);
            };
            assert_eq!(
                subject_name(&decomposed.model, &decomposed.subjects, tested),
                subject,
                "{source}"
            );
            assert_eq!(actual, positive, "{source}");
            let program = decomposed.model.root().syntax().text_trimmed().to_string();
            assert!(
                matches!(&program[constructor], "Date" | "other" | "other.C"),
                "{source}: {}",
                &program[constructor]
            );
        }
    }

    #[test]
    fn private_names_in_tests_test_the_class_that_declares_them() {
        for (program, class, is_static) in [
            (
                "class K { #p = 1; m(value) { if (#p in value) {} } }",
                "K",
                false,
            ),
            (
                "class K { static #p = 1; m(value) { if (#p in value) {} } }",
                "K",
                true,
            ),
            // The innermost class that declares the name is the one that the
            // name refers to.
            (
                "class K { #p = 1; m() { class L { #q = 1; n(value) { if (#p in value) {} } } } }",
                "K",
                false,
            ),
            (
                "const C = class { #p() {} m(value) { if (#p in value) {} } };",
                "class { #p() {} m(value) { if (#p in value) {} } }",
                false,
            ),
        ] {
            let decomposed = decomposed_last(program, FlowOutcome::Truthy);
            assert!(decomposed.condition.mentions.is_empty(), "{program}");
            assert_eq!(parent_names(&decomposed), ["value"], "{program}");
            let FlowGuard::Test {
                test:
                    FlowTest::PrivateName {
                        class: range,
                        is_static: actual,
                    },
                positive: true,
                ..
            } = decomposed.guards[decomposed.condition.guard]
            else {
                panic!("{program}: {:?}", decomposed.guards);
            };
            let source = decomposed.model.root().syntax().text_trimmed().to_string();
            assert_eq!(&source[range], class, "{program}");
            assert_eq!(actual, is_static, "{program}");
        }
    }

    #[test]
    fn constructor_comparisons_test_their_object_where_they_hold() {
        use FlowOutcome::{Falsy, Truthy};
        for (source, outcome, constructor) in [
            ("value.constructor === Date", Truthy, Some("Date")),
            ("value.constructor == other.C", Truthy, Some("other.C")),
            ("Date === value.constructor", Truthy, Some("Date")),
            ("value.constructor !== Date", Falsy, Some("Date")),
            ("value?.constructor === Date", Truthy, Some("Date")),
            // TypeScript narrows only where the comparison holds.
            ("value.constructor === Date", Falsy, None),
            ("value.constructor !== Date", Truthy, None),
        ] {
            let decomposed = decomposed(source, outcome);
            let guard = &decomposed.guards[decomposed.condition.guard];
            let Some(constructor) = constructor else {
                assert_eq!(*guard, FlowGuard::Keep, "{source}");
                continue;
            };
            assert_eq!(parent_names(&decomposed), ["value"], "{source}");
            let FlowGuard::Test {
                test: FlowTest::Constructor(range),
                positive: true,
                ..
            } = guard
            else {
                panic!("{source}: {:?}", decomposed.guards);
            };
            let program = decomposed.model.root().syntax().text_trimmed().to_string();
            assert_eq!(&program[*range], constructor, "{source}");
        }
        // A value that is not a dotted name, and a member other than
        // `constructor`, compare as any other value.
        for source in ["value.constructor === f()", "value.kind === Date"] {
            let decomposed = decomposed(source, Truthy);
            assert!(
                !decomposed.guards.iter().any(|guard| matches!(
                    guard,
                    FlowGuard::Test {
                        test: FlowTest::Constructor(_),
                        ..
                    }
                )),
                "{source}"
            );
        }
    }

    #[test]
    fn instanceof_of_an_expression_that_is_not_a_subject_tests_nothing() {
        for source in [
            "value.f() instanceof Date",
            "value! instanceof Date",
            "(value as object) instanceof Date",
            "value[other] instanceof Date",
        ] {
            let decomposed = decomposed(source, FlowOutcome::Truthy);
            assert!(decomposed.condition.mentions.is_empty(), "{source}");
            assert!(decomposed.condition.parents.is_empty(), "{source}");
            assert_eq!(
                decomposed.guards[decomposed.condition.guard],
                FlowGuard::Keep,
                "{source}"
            );
        }
    }

    #[test]
    fn instanceof_narrows_through_aliased_conditions() {
        let decomposed = decomposed_last(
            "function f(value) { const ok = value instanceof Date; if (ok) {} }",
            FlowOutcome::Truthy,
        );
        assert_eq!(parent_names(&decomposed), ["value"]);
    }

    #[test]
    fn tests_of_properties_of_this_add_no_property_guard() {
        for (body, expected) in [
            ("if (this.kind === 'a') {}", vec![]),
            ("const k = this.kind; if (k === 'a') {}", vec![]),
            ("const { kind } = this; if (kind === 'a') {}", vec![]),
            // A path on `this` is an object that a test can narrow.
            ("const k = this.s.kind; if (k === 'a') {}", vec!["this.s"]),
            (
                "const { kind } = this.s; if (kind === 'a') {}",
                vec!["this.s"],
            ),
        ] {
            let program = format!("class K {{ kind: string; s: any; m() {{ {body} }} }}");
            let parsed = parse(&program, JsFileSource::ts(), JsParserOptions::default());
            let model = semantic_model(&parsed.tree(), SemanticModelOptions::default());
            let test = parsed
                .syntax()
                .descendants()
                .find_map(JsIfStatement::cast)
                .unwrap()
                .test()
                .unwrap();
            let method = parsed
                .syntax()
                .descendants()
                .filter_map(biome_js_control_flow::AnyJsControlFlowRoot::cast)
                .find(super::super::subjects::this_can_narrow)
                .unwrap();
            let mut guards = Vec::new();
            let raw_binding_types = FxHashMap::default();
            let mut functions = GuardFunctions::new(&[], &raw_binding_types);
            let mut subjects = FlowSubjectTable::new(&method, true);
            let condition = decompose_condition(
                &test,
                FlowOutcome::Truthy,
                &model,
                &mut functions,
                &mut subjects,
                &mut guards,
            );
            let subjects = subjects.into_subjects();
            let parents: Vec<_> = condition
                .parents
                .iter()
                .map(|subject| subject_name(&model, &subjects, *subject))
                .collect();
            assert_eq!(parents, expected, "{body}");
            assert_eq!(
                guards
                    .iter()
                    .filter(|guard| matches!(guard, FlowGuard::Property { .. }))
                    .count(),
                expected.len(),
                "{body}: {guards:?}"
            );
        }
    }

    #[test]
    fn chains_that_start_at_a_parenthesized_chain_narrow_nothing() {
        for (source, outcome) in [
            ("(value?.a)?.b", FlowOutcome::Truthy),
            ("(value?.a)?.b?.c", FlowOutcome::Truthy),
            ("((value?.a))?.b === 1", FlowOutcome::Truthy),
            ("(value?.a).b !== undefined", FlowOutcome::Truthy),
            ("typeof (value?.f())?.b === 'string'", FlowOutcome::Truthy),
            ("(value?.f)()", FlowOutcome::Truthy),
        ] {
            let (guards, names) = decompose(source, outcome);
            assert_eq!(names, Vec::<String>::new(), "{source}");
            assert!(
                guards.iter().all(|guard| *guard == FlowGuard::Keep),
                "{source}: {guards:?}"
            );
        }
    }

    #[test]
    fn mentions_match_supported_condition_shapes() {
        for (source, outcome, expected) in [
            ("value", FlowOutcome::Truthy, vec!["value"]),
            ("(!((value)))", FlowOutcome::Falsy, vec!["value"]),
            (
                "typeof (value) !== 'number'",
                FlowOutcome::Truthy,
                vec!["value"],
            ),
            (
                "'string' === typeof value",
                FlowOutcome::Falsy,
                vec!["value"],
            ),
            ("(value) == (null)", FlowOutcome::Truthy, vec!["value"]),
            ("undefined != value", FlowOutcome::Truthy, vec!["value"]),
            ("value === 1", FlowOutcome::Falsy, vec!["value"]),
            (
                "value && other",
                FlowOutcome::Truthy,
                vec!["value", "other"],
            ),
            ("value || other", FlowOutcome::Falsy, vec!["value", "other"]),
            ("value", FlowOutcome::Nullish, vec!["value"]),
            ("value.length", FlowOutcome::Truthy, vec!["value.length"]),
            ("((value).a).b", FlowOutcome::Falsy, vec!["value.a.b"]),
            // The chain starts at a parenthesized optional chain.
            ("(value?.a).b", FlowOutcome::Falsy, vec![]),
            ("value.a", FlowOutcome::NonNullish, vec!["value.a"]),
            ("value.a !== null", FlowOutcome::Truthy, vec!["value.a"]),
            ("undefined == value.a", FlowOutcome::Falsy, vec!["value.a"]),
            ("value.a === 'x'", FlowOutcome::Truthy, vec!["value.a"]),
            (
                "typeof value.a === 'string'",
                FlowOutcome::Truthy,
                vec!["value.a"],
            ),
            (
                "value.a && other.b",
                FlowOutcome::Truthy,
                vec!["value.a", "other.b"],
            ),
            ("value['a']", FlowOutcome::Truthy, vec!["value.a"]),
            ("value?.[0] !== null", FlowOutcome::Truthy, vec!["value.0"]),
            (
                "typeof value['a'].b === 'string'",
                FlowOutcome::Truthy,
                vec!["value.a.b"],
            ),
            ("value[other]", FlowOutcome::Truthy, vec![]),
            ("value[0x0]", FlowOutcome::Truthy, vec![]),
            ("value[other] === null", FlowOutcome::Truthy, vec![]),
            ("this.a", FlowOutcome::Truthy, vec![]),
            ("value.f()", FlowOutcome::Truthy, vec![]),
            ("value!.a", FlowOutcome::Truthy, vec![]),
            ("global.a", FlowOutcome::Truthy, vec![]),
            ("value.a === other.b", FlowOutcome::Truthy, vec![]),
            ("check(value)", FlowOutcome::Truthy, vec![]),
            ("value === other", FlowOutcome::Truthy, vec![]),
            ("value == 1", FlowOutcome::Truthy, vec!["value"]),
            ("value === -1", FlowOutcome::Truthy, vec!["value"]),
            ("value === -(1)", FlowOutcome::Truthy, vec![]),
            ("value === void 0", FlowOutcome::Truthy, vec!["value"]),
            ("value === `a`", FlowOutcome::Truthy, vec!["value"]),
            ("value === `${other}`", FlowOutcome::Truthy, vec![]),
            ("value ?? other", FlowOutcome::Truthy, vec![]),
            ("value && other", FlowOutcome::Nullish, vec![]),
            ("global", FlowOutcome::Truthy, vec![]),
        ] {
            assert_eq!(mentioned_names(source, outcome), expected, "{source}");
        }
    }

    /// Decomposes the condition of the last `if` statement in `program` and
    /// names the value that its first runtime test compares with: `=` and the
    /// source of a [`FlowTest::Literal`], `null`, `undefined`, `==null` or
    /// `==undefined` for loose equality, `enum` and the name of a
    /// [`FlowTest::EnumMember`], `import` and the imported name, with the
    /// member's name after it, `typed` and the source of a
    /// [`FlowTest::Compared`], or `none` when the condition tests nothing.
    /// `==` before the source marks loose equality.
    fn compared_value(program: &str) -> String {
        let decomposed = decomposed_last(program, FlowOutcome::Truthy);
        let source = decomposed.model.root().syntax().to_string();
        let test = decomposed.guards.iter().find_map(|guard| match guard {
            FlowGuard::Test { test, .. } => Some(*test),
            _ => None,
        });
        match test {
            Some(FlowTest::Literal { value, strict }) => {
                format!("{}{}", if strict { "=" } else { "==" }, &source[value])
            }
            Some(FlowTest::Compared { value, strict }) => {
                format!("typed {}{}", if strict { "" } else { "==" }, &source[value])
            }
            Some(FlowTest::Null) => "null".to_owned(),
            Some(FlowTest::Nullish) => "==null".to_owned(),
            Some(FlowTest::Undefined { strict: true }) => "undefined".to_owned(),
            Some(FlowTest::Undefined { strict: false }) => "==undefined".to_owned(),
            Some(FlowTest::EnumMember { member, strict }) => {
                format!("enum {}{}", if strict { "" } else { "==" }, &source[member])
            }
            Some(FlowTest::Imported {
                binding,
                member,
                strict,
            }) => {
                format!(
                    "import {}{}{}",
                    if strict { "" } else { "==" },
                    &source[binding],
                    member.map_or(String::new(), |member| format!(".{}", &source[member]))
                )
            }
            Some(test) => format!("{test:?}"),
            None => "none".to_owned(),
        }
    }

    #[test]
    fn comparisons_test_the_other_operand_by_its_unit_value() {
        for (program, expected) in [
            ("function f(x) { if (x === -1) {} }", "=-1"),
            ("function f(x) { if (x === +1) {} }", "=+1"),
            ("function f(x) { if (x === `a`) {} }", "=`a`"),
            ("function f(x) { if (x === ``) {} }", "=``"),
            ("function f(x) { if (x === void f()) {} }", "undefined"),
            ("function f(x) { if (x == void 0) {} }", "==undefined"),
            ("const K = 'a'; function f(x) { if (x === K) {} }", "='a'"),
            (
                "const K = ('a'); function f(x) { if (x === (K)) {} }",
                "='a'",
            ),
            (
                "const K = -1; const L = K; function f(x) { if (L === x) {} }",
                "=-1",
            ),
            ("const K = null; function f(x) { if (x === K) {} }", "null"),
            ("const K = null; function f(x) { if (x == K) {} }", "==null"),
            (
                "const K = undefined; function f(x) { if (x === K) {} }",
                "undefined",
            ),
            (
                "const K = void 0; function f(x) { if (x != K) {} }",
                "==undefined",
            ),
            (
                "function f(x) { const undefined = 'a'; if (x === undefined) {} }",
                "='a'",
            ),
            (
                "import { K } from './k'; function f(x) { if (x === K) {} }",
                "import K",
            ),
            (
                "import { A as K } from './k'; function f(x) { if (x == K) {} }",
                "import ==K",
            ),
            (
                "import K from './k'; function f(x) { if (x === K) {} }",
                "import K",
            ),
            (
                "import { A } from './k'; const K = A; function f(x) { if (x === K) {} }",
                "import A",
            ),
            // A member of an enum is compared with by the member's name, and
            // inference reads its value from the member's collected type.
            ("enum E { A } function f(x) { if (x === E.A) {} }", "enum A"),
            (
                "enum E { 'A' } function f(x) { if (E['A'] !== x) {} }",
                "enum 'A'",
            ),
            (
                "enum E { A } const K = E.A; function f(x) { if (x === K) {} }",
                "enum A",
            ),
            (
                "import { E } from './e'; function f(x) { if (x === E.A) {} }",
                "import E.A",
            ),
            (
                "import { E } from './e'; function f(x) { if (x == E['A']) {} }",
                "import ==E.'A'",
            ),
            // `-(1)` has the type `number`, Biome gives `-1n` the type
            // `bigint`, and a `let` widens its literal. Biome doesn't compare
            // with the other values below.
            ("function f(x) { if (x === -(1)) {} }", "none"),
            ("function f(x) { if (x === -1n) {} }", "none"),
            ("function f(x) { if (x === `${x}`) {} }", "none"),
            ("function f(x) { if (x === tag`a`) {} }", "none"),
            ("const K = -(1); function f(x) { if (x === K) {} }", "none"),
            (
                "const K = 1 === 1; function f(x) { if (x === K) {} }",
                "none",
            ),
            ("let K = 'a'; function f(x) { if (x === K) {} }", "none"),
            (
                "const K: 'a' = 'a'; function f(x) { if (x === K) {} }",
                "typed K",
            ),
            (
                "const K = 'a' as const; function f(x) { if (x === K) {} }",
                "none",
            ),
            (
                "const { K } = { K: 'a' }; function f(x) { if (x === K) {} }",
                "none",
            ),
            ("function f(x, k: 'a') { if (x === k) {} }", "typed k"),
            (
                "function f(x, o: { k: 'a' }) { if (x == o.k) {} }",
                "typed ==o.k",
            ),
            (
                "function f(x) { if (x === ('a' as const)) {} }",
                "typed 'a' as const",
            ),
            (
                "function f(x) { const k = 'a' as const; if (x === k) {} }",
                "none",
            ),
            ("function f(x, k = 'a') { if (x === k) {} }", "none"),
            ("function f(x) { if (x === f()) {} }", "none"),
            (
                "import * as N from './k'; function f(x) { if (x === N) {} }",
                "none",
            ),
            // An optional chain, parentheses around the enum, a member that
            // the enum doesn't declare, a key that isn't a string literal,
            // and a member of an object that isn't an enum compare with
            // nothing.
            ("enum E { A } function f(x) { if (x === E?.A) {} }", "none"),
            ("enum E { A } function f(x) { if (x === (E).A) {} }", "none"),
            ("enum E { A } function f(x) { if (x === E.B) {} }", "none"),
            ("enum E { A } function f(x) { if (x === E[0]) {} }", "none"),
            (
                "const O = { A: 1 }; function f(x) { if (x === O.A) {} }",
                "none",
            ),
            (
                "enum E { A } function f(x) { if (x == E.A) {} }",
                "enum ==A",
            ),
            ("function f(x) { if (x == 'a') {} }", "=='a'"),
            ("function f(x) { if (x === NaN) {} }", "none"),
            (
                "const A = B; const B = A; function f(x) { if (x === A) {} }",
                "none",
            ),
        ] {
            assert_eq!(compared_value(program), expected, "{program}");
        }
    }

    #[test]
    fn missing_operands_make_the_guard_incomplete() {
        let (guards, names) = decompose("other === null || value ===", FlowOutcome::Truthy);
        assert!(guards.contains(&FlowGuard::Incomplete));
        assert_eq!(names, ["other", "value"]);
    }

    #[test]
    fn incomplete_decomposition_mentions_every_direct_read() {
        let source = format!(
            "{}value && other.length && other['a'] && other[value]",
            "!".repeat(MAX_CONDITION_DEPTH)
        );
        assert_eq!(
            mentioned_names(&source, FlowOutcome::Truthy),
            [
                "value",
                "other.length",
                "other",
                "other.a",
                "other",
                "other",
                "value"
            ]
        );
    }

    #[test]
    fn aliased_conditions_inline_their_initializers() {
        use FlowOutcome::{Falsy, Truthy};
        let alias = "function f(value, other) { const ok = value !== null && other;";
        for (test, outcome, expected) in [
            ("ok", Truthy, vec!["ok", "value", "other"]),
            ("ok", Falsy, vec!["ok", "value", "other"]),
            ("!ok", Truthy, vec!["ok", "value", "other"]),
            ("((ok))", Truthy, vec!["ok", "value", "other"]),
            ("ok && other", Truthy, vec!["ok", "value", "other", "other"]),
            ("ok || other", Falsy, vec!["ok", "value", "other", "other"]),
            // Only truthiness and comparisons with booleans test the alias
            // as a condition, the latter only where the comparison shows its
            // outcome.
            ("ok", FlowOutcome::Nullish, vec!["ok"]),
            ("ok === true", Truthy, vec!["ok", "value", "other"]),
            ("ok !== false", Truthy, vec!["ok"]),
            ("ok === false", Truthy, vec!["ok", "value", "other"]),
            ("ok === 1", Truthy, vec!["ok"]),
        ] {
            assert_eq!(
                mentioned_names_in(&format!("{alias} if ({test}) {{}} }}"), outcome),
                expected,
                "{test}"
            );
        }

        let Decomposed {
            guards, condition, ..
        } = decomposed_last("function f(value) { const ok = value; if (ok) {} }", Falsy);
        let FlowGuard::Both {
            left,
            right,
            sequential: true,
        } = guards[condition.guard]
        else {
            panic!("{guards:?}");
        };
        for guard in [left, right] {
            assert!(
                matches!(
                    guards[guard],
                    FlowGuard::Test {
                        test: FlowTest::Truthy,
                        positive: false,
                        ..
                    }
                ),
                "{guards:?}"
            );
        }
    }

    #[test]
    fn only_unannotated_const_variables_with_initializers_are_aliases() {
        for declaration in [
            "const ok: boolean = value !== null;",
            "let ok = value !== null;",
            "var ok = value !== null;",
            "const ok;",
            "const { ok } = { ok: value !== null };",
            "const [ok] = [value !== null];",
        ] {
            assert_eq!(
                mentioned_names_in(
                    &format!("function f(value) {{ {declaration} if (ok) {{}} }}"),
                    FlowOutcome::Truthy
                ),
                ["ok"],
                "{declaration}"
            );
        }
        for program in [
            "const value = get(); export const ok = value !== null; if (ok) {}",
            "const value = get(); const ok = value !== null; export { ok }; if (ok) {}",
        ] {
            assert_eq!(
                mentioned_names_in(program, FlowOutcome::Truthy),
                ["ok"],
                "{program}"
            );
        }
    }

    #[test]
    fn inlined_initializers_test_only_variables_that_cannot_change() {
        for (program, expected) in [
            (
                "function f(value) { const ok = value; if (ok) {} }",
                vec!["ok", "value"],
            ),
            (
                "const g = value => { const ok = value; if (ok) {} };",
                vec!["ok", "value"],
            ),
            (
                "function f({ a }) { const { b } = a; const ok = a && b; if (ok) {} }",
                vec!["ok", "a", "b"],
            ),
            (
                "function f() { let l = get(); const ok = l; if (ok) {} }",
                vec!["ok", "l"],
            ),
            (
                "try {} catch (error) { const ok = error; if (ok) {} }",
                vec!["ok", "error"],
            ),
            ("const c = get(); const ok = c; if (ok) {}", vec!["ok", "c"]),
            (
                "{ let l = get(); const ok = l; if (ok) {} }",
                vec!["ok", "l"],
            ),
            (
                "for (let l = get(); ; ) { const ok = l; if (ok) {} }",
                vec!["ok", "l"],
            ),
            (
                "class K { constructor(private value) { const ok = value; if (ok) {} } }",
                vec!["ok", "value"],
            ),
            // A property path narrows only when it is readonly, which
            // inference decides, while its receiver always narrows.
            (
                "function f(value) { const ok = value?.a; if (ok) {} }",
                vec!["ok", "value.a", "value"],
            ),
            // A top-level `let` of a script can change in another script.
            ("let l = get(); const ok = l; if (ok) {}", vec!["ok"]),
            (
                "export const c = get(); const ok = c; if (ok) {}",
                vec!["ok"],
            ),
            (
                "const c = get(); export { c }; const ok = c; if (ok) {}",
                vec!["ok"],
            ),
            (
                "function f() { let l = get(); l = get(); const ok = l; if (ok) {} }",
                vec!["ok"],
            ),
            (
                "function f(value) { const ok = value; if (ok) {} value = get(); }",
                vec!["ok"],
            ),
            (
                "function f() { var v = get(); const ok = v; if (ok) {} }",
                vec!["ok"],
            ),
            ("function g() {} const ok = g; if (ok) {}", vec!["ok"]),
            (
                "import { i } from \"i\"; const ok = i; if (ok) {}",
                vec!["ok"],
            ),
            (
                "function f(value) { const ok = value.a !== null; if (ok) {} }",
                vec!["ok", "value.a"],
            ),
            (
                "function f(value) { const ok = value['a']; if (ok) {} }",
                vec!["ok", "value.a"],
            ),
            // The path of a variable that can change never narrows.
            (
                "function f(value) { const ok = value.a; if (ok) {} value = get(); }",
                vec!["ok"],
            ),
            ("const ok = global; if (ok) {}", vec!["ok"]),
        ] {
            assert_eq!(
                mentioned_names_in(program, FlowOutcome::Truthy),
                expected,
                "{program}"
            );
        }
    }

    /// Decomposes the test of the last `if` statement in `program` and names
    /// what each [`FlowGuard::ReadonlyPath`] wraps: the subject of a runtime
    /// test, or the object and key of a property guard. `this` is a subject
    /// in the first method or arrow function of `program` whose `this` can
    /// narrow.
    fn readonly_names_in(program: &str) -> Vec<String> {
        let parsed = parse(program, JsFileSource::ts(), JsParserOptions::default());
        let model = semantic_model(&parsed.tree(), SemanticModelOptions::default());
        let test = parsed
            .syntax()
            .descendants()
            .filter_map(JsIfStatement::cast)
            .last()
            .unwrap()
            .test()
            .unwrap();
        let mut subjects = match parsed
            .syntax()
            .descendants()
            .filter_map(biome_js_control_flow::AnyJsControlFlowRoot::cast)
            .find(super::super::subjects::this_can_narrow)
        {
            Some(root) => FlowSubjectTable::new(&root, true),
            None => FlowSubjectTable::default(),
        };
        let mut guards = Vec::new();
        let raw_binding_types = FxHashMap::default();
        let mut functions = GuardFunctions::new(&[], &raw_binding_types);
        decompose_condition(
            &test,
            FlowOutcome::Truthy,
            &model,
            &mut functions,
            &mut subjects,
            &mut guards,
        );
        let subjects = subjects.into_subjects();
        readonly_guards(&guards)
            .into_iter()
            .map(|inner| match &guards[inner] {
                FlowGuard::Test { subject, .. } => subject_name(&model, &subjects, *subject),
                FlowGuard::Property {
                    object,
                    key: PropertyKey::Name(key),
                    ..
                } => format!("{}[{key}]", subject_name(&model, &subjects, *object)),
                guard => panic!("{program}: {guard:?}"),
            })
            .collect()
    }

    #[test]
    fn inlined_tests_of_paths_apply_only_to_readonly_paths() {
        for (program, expected) in [
            (
                "function f(value) { const ok = value.a !== null; if (ok) {} }",
                vec!["value.a"],
            ),
            (
                "function f(value) { const ok = typeof value.a.b === 'string'; if (ok) {} }",
                vec!["value.a.b", "value.a[b]"],
            ),
            // The receivers of an optional chain that are paths, and the
            // object of a property guard that is a path.
            (
                "function f(value) { const ok = value.a?.b; if (ok) {} }",
                vec!["value.a.b", "value.a[b]", "value.a"],
            ),
            (
                "function f(value) { const ok = value.a.kind === 'x'; if (ok) {} }",
                vec!["value.a.kind", "value.a[kind]"],
            ),
            (
                "function f(value) { const ok = 'x' in value.a; if (ok) {} }",
                vec!["value.a"],
            ),
            (
                "class K { m() { const ok = this.v !== null; if (ok) {} } }",
                vec!["this.v"],
            ),
            (
                "class K { m() { const g = () => { const ok = this.#v; if (ok) {} }; } }",
                vec!["this.#v"],
            ),
            // A `this` in an enclosing function can be another function's.
            (
                "function outer() { const ok = this.v; class K { m() { if (ok) {} } } }",
                vec![],
            ),
            // Variables, and tests outside aliased conditions, always narrow.
            (
                "function f(value) { const ok = value !== null; if (ok) {} }",
                vec![],
            ),
            ("function f(value) { if (value.a !== null) {} }", vec![]),
            // A path on a variable that can change never narrows.
            (
                "function f(value) { const ok = value.a; value = null; if (ok) {} }",
                vec![],
            ),
        ] {
            assert_eq!(readonly_names_in(program), expected, "{program}");
        }
    }

    #[test]
    fn aliased_conditions_inline_up_to_five_levels() {
        let chain = "const c1 = value !== null; const c2 = c1; const c3 = !!c2; \
            const c4 = c3; const c5 = c4 && other; const c6 = c5;";
        for (test, expected) in [
            ("c5", vec!["c5", "c4", "c3", "c2", "c1", "value", "other"]),
            ("c6", vec!["c6", "c5", "c4", "c3", "c2", "c1", "other"]),
        ] {
            assert_eq!(
                mentioned_names_in(
                    &format!("function f(value, other) {{ {chain} if ({test}) {{}} }}"),
                    FlowOutcome::Truthy
                ),
                expected,
                "{test}"
            );
        }
        // The limit counts the aliases that enclose a test, not every alias
        // that a condition inlines.
        assert_eq!(
            mentioned_names_in(
                "function f(value, other) { const a = value; const b = other; \
                const c = a && b; if (c && c) {} }",
                FlowOutcome::Truthy
            ),
            [
                "c", "a", "value", "b", "other", "c", "a", "value", "b", "other"
            ]
        );
    }

    #[test]
    fn incomplete_initializers_mention_the_subjects_they_may_narrow() {
        let program =
            "function f(value, other) { const ok = value.a === null || other ===; if (ok) {} }";
        assert!(
            decomposed_last(program, FlowOutcome::Truthy)
                .guards
                .contains(&FlowGuard::Incomplete)
        );
        // `value.a` is a property path, which an inlined test never narrows.
        assert_eq!(
            mentioned_names_in(program, FlowOutcome::Truthy),
            ["ok", "value", "other"]
        );
    }
}
