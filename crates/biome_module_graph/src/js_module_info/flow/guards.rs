use super::subjects::{
    AnyPatternElement, array_element_key, assigned_subject, identifier_key, literal_key,
    member_name_key, subject_of,
};
use super::{FlowSubject, FlowSubjectId, FlowSubjectTable, PropertyKey, subject_root};
use crate::js_module_info::TsBindingReferenceExt;
use biome_js_semantic::{Binding, JsDeclarationKind, SemanticModel};
use biome_js_syntax::{
    AnyJsArrowFunctionParameters, AnyJsAssignment, AnyJsCallArgument, AnyJsExpression,
    AnyJsFormalParameter, AnyJsInProperty, AnyJsLiteralExpression, AnyJsMemberExpression,
    AnyJsOptionalChainExpression, AnyJsParameter, AnyJsSwitchClause, AnyJsVariableDeclaration,
    JsBinaryOperator, JsCallExpression, JsIdentifierExpression, JsLanguage, JsLogicalOperator,
    JsModule, JsReferenceIdentifier, JsScript, JsSwitchStatement, JsUnaryOperator,
    JsVariableDeclarator, JsVariableKind, JsVariableStatement, T,
    binding_ext::AnyJsBindingDeclaration, unescape_js_identifier,
};
use biome_js_type_info::{
    Function, FunctionParameter, RawPredicateTarget, RawTypeData, RawTypeId, ReturnType,
    TypeReference, TypeReferenceQualifier, TypeofKind, raw_may_be_nullish, raw_predicate_target,
};
use biome_rowan::{
    AstNode, AstNodeList, AstSeparatedList, SyntaxKind, SyntaxKindSet, Text, TextRange, WalkEvent,
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
}

/// A runtime test whose result a guard assumes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum FlowTest {
    Truthy,
    Nullish,
    Typeof(TypeofKind),
    /// Strict equality with `null`.
    Null,
    /// Strict equality with the primitive literal expression at this range.
    Literal(TextRange),
    /// Equality with `undefined`; loose equality also matches `null`.
    Undefined {
        strict: bool,
    },
    /// A call to the type-guard function declared at `function`. Its type
    /// predicate states the tested type, as in `value is string` or
    /// `asserts value is Foo`.
    Predicate {
        function: TextRange,
        /// Whether every member of the tested type is an object type, as
        /// for `Foo` or `Foo | Bar`, so the test narrows only a type that
        /// has an object type or is `unknown`.
        object_target: bool,
    },
    /// The `in` operator, as in `"a" in value`. The range is the key, a string
    /// or number literal that spells the property's name, such as `"a"` or `0`
    /// but not `"\u0061"` or `0x0`. See [`literal_key`].
    In(TextRange),
    /// The `instanceof` operator, as in `value instanceof Date`, whose right
    /// operand, the constructor, is the expression at this range.
    Instanceof(TextRange),
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
        ReturnType::Type(_) => return None,
    };
    Some((signature, contract))
}

/// Selects the argument that a call to a local type guard tests.
///
/// A type guard's return annotation states a contract about one of its
/// parameters. With a type predicate, such as `value is string`, the call's
/// result reports whether the argument has the type. With an assertion, such
/// as `asserts value is string` or `asserts value`, the call returns only when
/// the contract holds. The guard's collected signature and its declaration
/// select the tested argument. Collection never infers the guard's body.
pub(super) struct GuardFunctions<'a> {
    raw_types: &'a [RawTypeData],
    raw_binding_types: &'a FxHashMap<TextRange, TypeReference>,
    /// Maps each function binding to its guard, or to `None` when the
    /// function is not a supported guard.
    guards: FxHashMap<TextRange, Option<Guard>>,
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
        }
    }

    /// Returns whether `object`, the object of a `?.`, may be `null` or
    /// `undefined` as far as the collected types tell. It is false for `this`
    /// and for a variable whose collected type [`raw_may_be_nullish`]
    /// rejects, as `o` in `const o = { a: 1 }; o?.f(o)`, where a test that
    /// the object is not nullish narrows nothing.
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

/// Decomposes the condition that a standalone call `expression` establishes
/// when it returns, or returns `None` when `expression` does not call a
/// supported assertion.
///
/// `asserts value is T` tests an argument that is a subject against the
/// guard's target. `asserts value` decomposes the argument as a condition that
/// holds, so `assert(value !== null)` excludes `null` from `value`.
pub(super) fn decompose_assertion(
    expression: &AnyJsExpression,
    model: &SemanticModel,
    functions: &mut GuardFunctions,
    subjects: &mut FlowSubjectTable,
    guards: &mut Vec<FlowGuard>,
) -> Option<DecomposedCondition> {
    let mut remaining = MAX_CONDITION_STEPS;
    let AnyJsExpression::JsCallExpression(call) =
        omit_parentheses(expression.clone(), &mut remaining)?
    else {
        return None;
    };
    let (guard, function, argument) = functions.guard_call(&call, model, &mut remaining)?;
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
            guards.push(FlowGuard::Test {
                subject,
                test: FlowTest::Predicate {
                    function,
                    object_target,
                },
                positive: true,
            });
            // As for `instanceof`, a target whose members are all object
            // types narrows only a type with an object type or `unknown`.
            let (mentions, parents) = if object_target {
                (Vec::new(), vec![subject])
            } else {
                (vec![subject], Vec::new())
            };
            Some(DecomposedCondition {
                guard: guards.len() - 1,
                mentions,
                instance_tested: parents.clone(),
                parents,
            })
        }
    }
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
    /// variable and property path read directly in the condition. It also
    /// lists every variable read directly in an inlined initializer when the
    /// variable cannot change between the alias and its test. See
    /// [`narrows_through_aliases`] for the variables that qualify.
    pub(super) mentions: Vec<FlowSubjectId>,
    /// Subjects whose type the condition may narrow only through these tests,
    /// unless `mentions` also lists them:
    ///
    /// - [`FlowGuard::Property`], such as `s` in `s.kind === "c"`.
    /// - [`FlowTest::In`], such as `x` in `"a" in x`.
    /// - [`FlowTest::Instanceof`], such as `x` in `x instanceof Date`.
    /// - A [`FlowTest::Predicate`] whose target has only object types, such as
    ///   `x` in `isFoo(x)`.
    ///
    /// The first two narrow only a union with two object types, and the
    /// others narrow only a type with an object type or `unknown`. When
    /// decomposition stops early, this also lists the pattern of every
    /// variable in `mentions` that is an element of a destructuring pattern
    /// that a test of the variable narrows.
    pub(super) parents: Vec<FlowSubjectId>,
    /// The subjects in `parents` that `instanceof` or a type-guard function
    /// whose target has only object types checks.
    pub(super) instance_tested: Vec<FlowSubjectId>,
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
/// `target`, assuming `outcome`, into guards appended to `guards`, as
/// TypeScript's `bindLogicalLikeExpression` does: `x ??= y` evaluates `y`
/// where `x` is nullish, `x ||= y` where it is falsy, and `x &&= y` where it
/// is truthy.
///
/// The guard tests the subject that [`assigned_subject`] gives the target, as
/// [`decompose_condition`] tests the expression with the same spelling, and
/// adds the same property guard for a named property, so `s.kind ||= "c"`
/// narrows `s` as `s.kind || "c"` does. A target is never part of an optional
/// chain, so the guard tests no receiver. A target that is no subject, as
/// `o[k]`, gives [`FlowGuard::Keep`].
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
    /// is a string literal, as in `switch (typeof x) { case "string": }`. A
    /// case narrows as `typeof operand === test` narrows the operand and the
    /// receivers of its optional chain, but not the object that holds it or
    /// the pattern that declares it, as in TypeScript. A single case of any
    /// other kind, such as `` case `string`: `` or `case k:`, makes the
    /// statement [`Self::Opaque`]. TypeScript also accepts a template
    /// literal there.
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
                AnyJsSwitchClause::JsCaseClause(clause) => matches!(
                    clause.test(),
                    Ok(AnyJsExpression::AnyJsLiteralExpression(
                        AnyJsLiteralExpression::JsStringLiteralExpression(_)
                    ))
                ),
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
/// type. Biome accepts a string, number, bigint, or boolean literal, `null`,
/// or `undefined` for [`SwitchKind::Value`], and a string literal that names
/// a kind of value, such as `"string"`, for [`SwitchKind::Typeof`].
///
/// Returns false for a local variable named `undefined`, which can hold any
/// value.
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
    match comparison_step(&discriminant, &test, true, true, &mut remaining) {
        Some(ConditionStep::Guard {
            guard: SyntaxGuard::Literal(_) | SyntaxGuard::Typeof(_),
            ..
        }) => true,
        Some(ConditionStep::Guard {
            guard: SyntaxGuard::Undefined { reference, .. },
            ..
        }) => model.binding(&reference).is_none(),
        _ => false,
    }
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
/// fails to match as `!==` does. A case that doesn't match narrows no
/// receiver of an optional chain, as in TypeScript, so `default` after
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
/// boolean, and a case test that a comparison can't use, such as `-1`, a
/// `const`, an enum member, or a template literal. TypeScript narrows by
/// those case tests too.
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
/// only when the test is falsy.
///
/// This holds for comparisons, `instanceof`, `in`, `!`, calls, and these in
/// parentheses, for `a && b` when it holds for `b`, and for `a || b` when it
/// holds for both. A call counts because only a call to a type-guard
/// function narrows, and such a function returns a boolean. A falsy `a`
/// makes `a && b` give `a`, which is never `true`, so `a` need not be a
/// boolean.
#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "Other expressions may give a truthy value other than `true`."
)]
fn is_boolean_valued(expression: &AnyJsExpression, remaining: &mut usize) -> bool {
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
        AnyJsExpression::JsLogicalExpression(logical) => match logical.operator() {
            Ok(JsLogicalOperator::LogicalAnd) => logical
                .right()
                .is_ok_and(|right| is_boolean_valued(&right, remaining)),
            Ok(JsLogicalOperator::LogicalOr) => {
                logical
                    .left()
                    .is_ok_and(|left| is_boolean_valued(&left, remaining))
                    && logical
                        .right()
                        .is_ok_and(|right| is_boolean_valued(&right, remaining))
            }
            Ok(JsLogicalOperator::NullishCoalescing) | Err(_) => false,
        },
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
    /// the [`FlowTest::In`], [`FlowTest::Instanceof`], and object
    /// [`FlowTest::Predicate`] tests decomposed so far.
    parents: Vec<FlowSubjectId>,
    /// The subjects of the [`FlowTest::Instanceof`] tests and of the
    /// [`FlowTest::Predicate`] tests whose targets have only object types
    /// decomposed so far.
    instance_tested: Vec<FlowSubjectId>,
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
    /// variable and property path that `scanned` read directly instead. It
    /// also mentions the variables that inlined initializers read when an
    /// aliased condition can narrow them ([`narrows_through_aliases`]).
    fn finish<'e>(
        self,
        guard: usize,
        scanned: impl IntoIterator<Item = &'e AnyJsExpression>,
    ) -> DecomposedCondition {
        let Self {
            model,
            subjects,
            mut parents,
            instance_tested,
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
        }
    }

    fn guard(&mut self, expression: AnyJsExpression, outcome: FlowOutcome, depth: usize) -> usize {
        if depth >= MAX_CONDITION_DEPTH {
            self.complete = false;
            return self.push(FlowGuard::Keep);
        }
        let Some(step) = condition_step(expression, outcome, &mut self.remaining) else {
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
                        None => FlowGuard::Keep,
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
        };
        self.push(guard)
    }

    /// Returns the guard for `guard` applied to `operand`: a runtime test of
    /// `operand` when it is a subject and `guard` is not
    /// [`SyntaxGuard::LooseLiteral`]. When `receivers` holds, the guard first
    /// tests that each receiver of the optional chain around `operand` is not
    /// nullish.
    ///
    /// Returns [`FlowGuard::Keep`] when the chain around `operand` starts at a
    /// parenthesized optional chain, as in `(o?.v)?.w`, and when finding its
    /// receivers runs out of work or meets missing syntax.
    fn test(
        &mut self,
        operand: &AnyJsExpression,
        guard: SyntaxGuard,
        positive: bool,
        receivers: bool,
    ) -> FlowGuard {
        let test = match guard {
            SyntaxGuard::Truthy => Some(FlowTest::Truthy),
            SyntaxGuard::Nullish => Some(FlowTest::Nullish),
            SyntaxGuard::Typeof(kind) => Some(FlowTest::Typeof(kind)),
            SyntaxGuard::Literal(AnyJsLiteralExpression::JsNullLiteralExpression(_)) => {
                Some(FlowTest::Null)
            }
            SyntaxGuard::Literal(literal) => Some(FlowTest::Literal(literal.range())),
            SyntaxGuard::LooseLiteral => None,
            // A local variable named `undefined` can hold any value.
            SyntaxGuard::Undefined { reference, .. }
                if self.model.binding(&reference).is_some() =>
            {
                return FlowGuard::Keep;
            }
            SyntaxGuard::Undefined { strict, .. } => Some(FlowTest::Undefined { strict }),
            SyntaxGuard::Predicate(test) => Some(test),
            SyntaxGuard::In(key) => Some(FlowTest::In(key)),
            SyntaxGuard::Instanceof(constructor) => Some(FlowTest::Instanceof(constructor)),
        };
        let Some(chain) = chain_receivers(operand, &mut self.remaining) else {
            return FlowGuard::Keep;
        };
        let tested =
            test.and_then(|test| Some((test, subject_of(operand, self.model, self.subjects)?)));
        let mut guard = match tested {
            Some((test, subject)) => match self.testable(subject) {
                Testable::Never => FlowGuard::Keep,
                testable => {
                    // Like a test of a property, which narrows its object,
                    // `in` narrows only a union with two object types.
                    // `instanceof` and a type-guard function whose target has
                    // only object types narrow only a type with an object
                    // type or `unknown`.
                    match test {
                        FlowTest::In(_) => self.parents.push(subject),
                        FlowTest::Instanceof(_)
                        | FlowTest::Predicate {
                            object_target: true,
                            ..
                        } => {
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
            self.receiver_tests(chain, guard)
        } else {
            guard
        }
    }

    /// Returns the guard for `test` applied to `subject`, the target of a
    /// logical assignment, combined with the guard that
    /// [`Self::property_guard`] gives its object.
    fn target_test(&mut self, subject: FlowSubjectId, test: FlowTest, positive: bool) -> FlowGuard {
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
    /// `operand` is `None` for the target of a logical assignment, as `s.kind`
    /// in `s.kind ||= "c"`, which reads its property without an optional
    /// chain and holds no property in a variable.
    ///
    /// Returns `None` for a call to a type-guard function, for `in`, and for
    /// `instanceof`, because TypeScript never narrows `s` through
    /// `isC(s.kind)`, `"a" in s.kind`, or `s.kind instanceof C`. Only the
    /// object that holds the property narrows: `o.a.kind` narrows `o.a`, not
    /// `o`.
    fn property_guard(
        &mut self,
        operand: Option<&AnyJsExpression>,
        subject: FlowSubjectId,
        test: FlowTest,
        positive: bool,
    ) -> Option<FlowGuard> {
        if matches!(
            test,
            FlowTest::Predicate { .. } | FlowTest::In(_) | FlowTest::Instanceof(_)
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
        ) {
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
    /// Returns `None` for a call to a type-guard function, for `in`, and for
    /// `instanceof`, because TypeScript never narrows a pattern through these
    /// tests of one of its elements. Inside an inlined aliased condition,
    /// every such pattern can be tested, because its values don't change
    /// after the destructuring.
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
            FlowTest::Predicate { .. } | FlowTest::In(_) | FlowTest::Instanceof(_)
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
        let mut guard = guard;
        for receiver in receivers {
            let Some(subject) = subject_of(&receiver, self.model, self.subjects) else {
                continue;
            };
            let testable = self.testable(subject);
            if testable == Testable::Never {
                continue;
            }
            self.mentions.push(subject);
            let test = FlowGuard::Test {
                subject,
                test: FlowTest::Nullish,
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
        let initializer = const_initializer(&declarator)?;
        self.initializers.push(initializer.clone());
        Some(initializer)
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
fn is_top_level_statement(declaration: &AnyJsVariableDeclaration) -> bool {
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
    Literal(AnyJsLiteralExpression),
    /// Loose equality with a primitive literal other than `null`, which
    /// narrows only the receivers of an optional chain.
    LooseLiteral,
    Undefined {
        reference: JsReferenceIdentifier,
        strict: bool,
    },
    /// A call to a type-guard function, with the test it applies, a
    /// [`FlowTest::Predicate`].
    Predicate(FlowTest),
    /// The `in` operator whose key is the literal at this range.
    In(TextRange),
    /// The `instanceof` operator whose right operand is at this range.
    Instanceof(TextRange),
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
            // A private name, as in `#p in value`, isn't decomposed.
            let AnyJsInProperty::AnyJsExpression(key) = expression.property().ok()? else {
                return Some(ConditionStep::Unsupported);
            };
            let key = omit_parentheses(key, remaining)?;
            let object = omit_parentheses(expression.object().ok()?, remaining)?;
            if literal_key(&key).is_none() || !has_subject_shape(&object)? {
                return Some(ConditionStep::Unsupported);
            }
            Some(ConditionStep::Guard {
                operand: object,
                guard: SyntaxGuard::In(key.range()),
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
                let step = comparison_step(subject, value, strict, equal, remaining)?;
                if !matches!(step, ConditionStep::Unsupported) {
                    return Some(step);
                }
            }
            Some(ConditionStep::Unsupported)
        }
        _ => Some(ConditionStep::Unsupported),
    }
}

#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "Comparisons only support subjects and primitive guards."
)]
fn comparison_step(
    subject: &AnyJsExpression,
    value: &AnyJsExpression,
    strict: bool,
    positive: bool,
    remaining: &mut usize,
) -> Option<ConditionStep> {
    let (operand, guard) = match subject {
        subject if has_operand_shape(subject)? => {
            let guard = match value {
                AnyJsExpression::AnyJsLiteralExpression(literal) => {
                    literal.value_token().ok()?;
                    match literal {
                        AnyJsLiteralExpression::JsNullLiteralExpression(_) if !strict => {
                            SyntaxGuard::Nullish
                        }
                        AnyJsLiteralExpression::JsNullLiteralExpression(_)
                        | AnyJsLiteralExpression::JsBooleanLiteralExpression(_)
                        | AnyJsLiteralExpression::JsStringLiteralExpression(_)
                        | AnyJsLiteralExpression::JsNumberLiteralExpression(_)
                        | AnyJsLiteralExpression::JsBigintLiteralExpression(_)
                            if strict =>
                        {
                            SyntaxGuard::Literal(literal.clone())
                        }
                        AnyJsLiteralExpression::JsBooleanLiteralExpression(_)
                        | AnyJsLiteralExpression::JsStringLiteralExpression(_)
                        | AnyJsLiteralExpression::JsNumberLiteralExpression(_)
                        | AnyJsLiteralExpression::JsBigintLiteralExpression(_) => {
                            SyntaxGuard::LooseLiteral
                        }
                        _ => return Some(ConditionStep::Unsupported),
                    }
                }
                AnyJsExpression::JsIdentifierExpression(identifier) => {
                    let reference = identifier.name().ok()?;
                    if reference.value_token().ok()?.text_trimmed() != "undefined" {
                        return Some(ConditionStep::Unsupported);
                    }
                    SyntaxGuard::Undefined { reference, strict }
                }
                _ => return Some(ConditionStep::Unsupported),
            };
            (subject.clone(), guard)
        }
        AnyJsExpression::JsUnaryExpression(unary) => {
            if unary.operator().ok()? != JsUnaryOperator::Typeof {
                return Some(ConditionStep::Unsupported);
            }
            let argument = unary.argument().ok()?;
            let AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsStringLiteralExpression(literal),
            ) = value
            else {
                return Some(ConditionStep::Unsupported);
            };
            let value = literal.inner_string_text().ok()?;
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
    // literal or, strictly, `null`, where it differs from `undefined` or,
    // loosely, from `null`, and where its `typeof` is not `"undefined"`.
    let receivers = match &guard {
        SyntaxGuard::Literal(_) | SyntaxGuard::LooseLiteral => positive,
        SyntaxGuard::Nullish | SyntaxGuard::Undefined { .. } => !positive,
        SyntaxGuard::Typeof(kind) => positive == (*kind != TypeofKind::Undefined),
        SyntaxGuard::Truthy
        | SyntaxGuard::Predicate(_)
        | SyntaxGuard::In(_)
        | SyntaxGuard::Instanceof(_) => false,
    };
    Some(ConditionStep::Guard {
        operand,
        guard,
        positive,
        receivers,
    })
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
            ("switch (typeof x) { case `a`: }", SwitchKind::Opaque),
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
            ("value?.a === -1", Truthy, vec![]),
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
                test: FlowTest::Literal(_),
                positive: true,
                ..
            }
        ));
        assert_eq!(
            mentioned_names("value?.a === 1", FlowOutcome::Truthy),
            ["value.a", "value"]
        );
        // Loose equality with a literal tests only the receivers.
        let (receivers, main) = receiver_names("value?.a == 1", FlowOutcome::Truthy);
        assert_eq!(receivers, ["value"]);
        assert_eq!(main, FlowGuard::Keep);
        assert_eq!(
            mentioned_names("value?.a == 1", FlowOutcome::Truthy),
            ["value"]
        );
        // A call that is not a type guard keeps its own result untested.
        let (receivers, main) = receiver_names("value?.f()", FlowOutcome::Truthy);
        assert_eq!(receivers, ["value", "value.f"]);
        assert_eq!(main, FlowGuard::Keep);
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
            // Loose equality with a literal narrows only the receivers of an
            // optional chain.
            ("value.kind == 'a'", Truthy, vec![]),
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
            ("const k = value.kind;", "k == 'a'", Truthy, vec![]),
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
            "class K { #p = 1; m(value) { if (#p in value) {} } }",
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
            ("value == 1", FlowOutcome::Truthy, vec![]),
            ("value === -1", FlowOutcome::Truthy, vec![]),
            ("value === void 0", FlowOutcome::Truthy, vec![]),
            ("value ?? other", FlowOutcome::Truthy, vec![]),
            ("value && other", FlowOutcome::Nullish, vec![]),
            ("global", FlowOutcome::Truthy, vec![]),
        ] {
            assert_eq!(mentioned_names(source, outcome), expected, "{source}");
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
            // Only truthiness tests the alias as a condition.
            ("ok", FlowOutcome::Nullish, vec!["ok"]),
            ("ok === true", Truthy, vec!["ok"]),
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
