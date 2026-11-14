use super::subjects::subject_of;
use super::{FlowSubject, FlowSubjectId, FlowSubjectTable, PropertyKey};
use crate::js_module_info::TsBindingReferenceExt;
use biome_js_semantic::{Binding, JsDeclarationKind, SemanticModel};
use biome_js_syntax::{
    AnyJsArrowFunctionParameters, AnyJsCallArgument, AnyJsExpression, AnyJsFormalParameter,
    AnyJsLiteralExpression, AnyJsOptionalChainExpression, AnyJsParameter, AnyJsVariableDeclaration,
    JsBinaryOperator, JsCallExpression, JsLogicalOperator, JsReferenceIdentifier, JsSyntaxKind,
    JsUnaryOperator, JsVariableKind, binding_ext::AnyJsBindingDeclaration, unescape_js_identifier,
};
use biome_js_type_info::{
    Function, FunctionParameter, RawPrimitiveTarget, RawTypeData, RawTypeId, ReturnType,
    TypeReference, TypeReferenceQualifier, TypeofKind, raw_primitive_target,
};
use biome_rowan::{AstNode, AstSeparatedList, SyntaxKind, Text, TextRange};
use rustc_hash::FxHashMap;
use std::borrow::Cow;

/// Limits the nested `!`, `&&`, and `||` operators decomposed in one
/// condition. Deeper operands keep the incoming type.
const MAX_CONDITION_DEPTH: usize = 32;
/// Limits the syntax steps spent decomposing one condition.
const MAX_CONDITION_STEPS: usize = 16_384;
/// Limits the descendants scanned for subjects when decomposition stops early.
const MAX_FALLBACK_NODES: usize = 1024;
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
    /// The step cannot refine any subject, so the incoming type is kept.
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
    Property {
        object: FlowSubjectId,
        key: PropertyKey,
        test: FlowTest,
        positive: bool,
        /// Whether the test reads the property through an optional chain, as
        /// `s?.kind` does, which gives `undefined` when `s` is nullish.
        optional_chain: bool,
    },
    /// The operands of `&&` or `||`. A sequential pair applies `right` to the
    /// result of `left`; otherwise both apply to the incoming type and their
    /// results are joined.
    Both {
        left: usize,
        right: usize,
        sequential: bool,
    },
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
    /// A call to the guard declared at this range. The guard's return
    /// annotation states the tested type, as in `value is string` or
    /// `asserts value is string`.
    Predicate(TextRange),
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
    /// `T`.
    Predicate,
    /// `asserts value is T`: the call returns only when the argument has type
    /// `T`.
    TypeAssertion,
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

    /// Returns the argument that `call` tests, without parentheses, with the
    /// range of the type predicate it calls.
    ///
    /// Returns `None` unless `call` calls a supported type predicate.
    fn tested_argument(
        &mut self,
        call: &JsCallExpression,
        model: &SemanticModel,
        remaining: &mut usize,
    ) -> Option<(AnyJsExpression, TextRange)> {
        let (guard, function, argument) = self.guard_call(call, model, remaining)?;
        (guard.kind == GuardKind::Predicate).then_some((argument, function))
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
    /// `undefined` argument. A target type must be able to resolve to a
    /// primitive type or a primitive literal, the only targets that narrow.
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
            Some(target) if !self.may_resolve_to_primitive(target, model) => return None,
            Some(_) if contract.asserts => GuardKind::TypeAssertion,
            Some(_) => GuardKind::Predicate,
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

    /// Returns whether the raw guard `target` can resolve to a primitive type
    /// or a primitive literal.
    ///
    /// A name continues with the type that a local type alias names. An
    /// imported name can name an alias in another module, so it returns true.
    /// Other names, such as interfaces, classes, enums, and global types, and
    /// names qualified by a namespace return false. A `typeof` query returns
    /// true, because only inference tells the type of the value it names.
    fn may_resolve_to_primitive(&self, target: &TypeReference, model: &SemanticModel) -> bool {
        let mut target = target;
        for _ in 0..MAX_ALIAS_STEPS {
            let qualifier = match raw_primitive_target(target, self.raw_types) {
                RawPrimitiveTarget::Primitive | RawPrimitiveTarget::Typeof => return true,
                RawPrimitiveTarget::Other => return false,
                RawPrimitiveTarget::Named(qualifier) => qualifier,
            };
            let Some(binding) = named_binding(qualifier, model) else {
                return false;
            };
            if binding.is_imported() {
                return true;
            }
            if !matches!(
                binding.tree().declaration(),
                Some(AnyJsBindingDeclaration::TsTypeAliasDeclaration(_))
            ) {
                return false;
            }
            let Some(aliased) = self.raw_binding_types.get(&binding.range()) else {
                return false;
            };
            target = aliased;
        }
        false
    }
}

/// Limits the chain of type aliases that a guard target follows.
const MAX_ALIAS_STEPS: usize = 16;

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
        GuardKind::Predicate => None,
        GuardKind::TruthyAssertion => Some(decompose_condition(
            &argument,
            FlowOutcome::Truthy,
            model,
            functions,
            subjects,
            guards,
        )),
        GuardKind::TypeAssertion => {
            chain_receivers(&argument, &mut remaining)?;
            let subject = subject_of(&argument, model, subjects)?;
            guards.push(FlowGuard::Test {
                subject,
                test: FlowTest::Predicate(function),
                positive: true,
            });
            Some(DecomposedCondition {
                guard: guards.len() - 1,
                mentions: vec![subject],
                parents: Vec::new(),
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
pub(super) struct DecomposedCondition {
    pub(super) guard: usize,
    /// Subjects whose type the condition may refine.
    ///
    /// When decomposition stops early, this conservatively lists every
    /// variable and property path read directly in the condition, and every
    /// subject read directly in an inlined initializer that
    /// [`narrows_through_aliases`] accepts.
    pub(super) mentions: Vec<FlowSubjectId>,
    /// Subjects whose type the condition may refine only through
    /// [`FlowGuard::Property`], such as `s` in `s.kind === "c"`, unless
    /// `mentions` also lists them.
    pub(super) parents: Vec<FlowSubjectId>,
}

/// Decomposes `expression`, assuming `outcome`, into guards appended to
/// `guards`, whose tests refer to subjects by their ids in `subjects`.
///
/// A truthiness test of a `const` without a type annotation, such as `ok` in
/// `const ok = x !== null; if (ok) x`, is an aliased condition: it also
/// applies the condition that initializes the `const`, as TypeScript does,
/// to the subjects that [`narrows_through_aliases`] accepts. Aliases inline
/// into one another up to [`MAX_INLINED_ALIASES`] levels.
pub(super) fn decompose_condition(
    expression: &AnyJsExpression,
    outcome: FlowOutcome,
    model: &SemanticModel,
    functions: &mut GuardFunctions,
    subjects: &mut FlowSubjectTable,
    guards: &mut Vec<FlowGuard>,
) -> DecomposedCondition {
    let mut decomposer = Decomposer {
        model,
        functions,
        subjects,
        guards,
        remaining: MAX_CONDITION_STEPS,
        mentions: Vec::new(),
        parents: Vec::new(),
        complete: true,
        aliases: 0,
        initializers: Vec::new(),
    };
    let guard = decomposer.guard(expression.clone(), outcome, 0);
    let parents = std::mem::take(&mut decomposer.parents);
    let mentions = if decomposer.complete {
        decomposer.mentions
    } else {
        let Decomposer {
            subjects,
            initializers,
            ..
        } = decomposer;
        let direct = expression.syntax().descendants().map(|node| (node, false));
        let inlined = initializers
            .iter()
            .flat_map(|initializer| initializer.syntax().descendants())
            .map(|node| (node, true));
        direct
            .chain(inlined)
            .take(MAX_FALLBACK_NODES)
            .filter_map(|(node, inlined)| {
                let expression = AnyJsExpression::cast(node)?;
                if !matches!(
                    expression,
                    AnyJsExpression::JsIdentifierExpression(_)
                        | AnyJsExpression::JsStaticMemberExpression(_)
                        | AnyJsExpression::JsComputedMemberExpression(_)
                ) {
                    return None;
                }
                let subject = subject_of(&expression, model, subjects)?;
                (!inlined || narrows_through_aliases(subjects, subject, model)).then_some(subject)
            })
            .collect()
    };
    DecomposedCondition {
        guard,
        mentions,
        parents,
    }
}

struct Decomposer<'a, 'raw> {
    model: &'a SemanticModel,
    functions: &'a mut GuardFunctions<'raw>,
    subjects: &'a mut FlowSubjectTable,
    guards: &'a mut Vec<FlowGuard>,
    remaining: usize,
    mentions: Vec<FlowSubjectId>,
    /// The objects of the [`FlowGuard::Property`] guards decomposed so far.
    parents: Vec<FlowSubjectId>,
    complete: bool,
    /// The number of aliased conditions whose initializers enclose the
    /// expression being decomposed.
    aliases: usize,
    /// Every initializer that an aliased condition has inlined.
    initializers: Vec<AnyJsExpression>,
}

impl Decomposer<'_, '_> {
    fn guard(&mut self, expression: AnyJsExpression, outcome: FlowOutcome, depth: usize) -> usize {
        if depth >= MAX_CONDITION_DEPTH {
            self.complete = false;
            return self.push(FlowGuard::Keep);
        }
        let Some(step) = condition_step(expression, outcome, &mut self.remaining) else {
            self.complete = false;
            return self.push(FlowGuard::Incomplete);
        };
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
                        Some((subject, function)) => {
                            self.test(&subject, SyntaxGuard::Predicate(function), positive, false)
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
    /// `operand` when it is a subject, combined with the tests that
    /// [`Self::receiver_tests`] adds when `receivers` holds.
    ///
    /// Returns [`FlowGuard::Keep`] when [`chain_receivers`] returns `None`
    /// for `operand`.
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
            SyntaxGuard::Predicate(function) => Some(FlowTest::Predicate(function)),
        };
        let Some(chain) = chain_receivers(operand, &mut self.remaining) else {
            return FlowGuard::Keep;
        };
        let tested =
            test.and_then(|test| Some((test, subject_of(operand, self.model, self.subjects)?)));
        let mut guard = match tested {
            Some((test, subject)) if self.may_test(subject) => {
                self.mentions.push(subject);
                FlowGuard::Test {
                    subject,
                    test,
                    positive,
                }
            }
            _ => FlowGuard::Keep,
        };
        if let Some((test, subject)) = tested
            && let Some(property) = self.property_guard(operand, subject, test, positive)
        {
            let left = self.push(guard);
            let right = self.push(property);
            guard = FlowGuard::Both {
                left,
                right,
                sequential: true,
            };
        }
        if receivers {
            self.receiver_tests(chain, guard)
        } else {
            guard
        }
    }

    /// Returns the guard that a runtime test of `operand`, whose subject is
    /// `subject`, applies to the object that holds it, as `s.kind === "c"`
    /// applies to `s`, when `subject` is a named property of a subject other
    /// than `this` that a test may narrow.
    ///
    /// Returns `None` for a call to a type-guard function, because
    /// TypeScript never narrows `s` through `isC(s.kind)`. Only the object
    /// that holds the property narrows: `o.a.kind` narrows `o.a`, not `o`.
    fn property_guard(
        &mut self,
        operand: &AnyJsExpression,
        subject: FlowSubjectId,
        test: FlowTest,
        positive: bool,
    ) -> Option<FlowGuard> {
        if matches!(test, FlowTest::Predicate(_)) {
            return None;
        }
        let subjects = self.subjects.as_slice();
        let FlowSubject::Member {
            object,
            key: key @ PropertyKey::Name(_),
        } = subjects.get(subject as usize)?
        else {
            return None;
        };
        let (object, key) = (*object, key.clone());
        if matches!(subjects.get(object as usize)?, FlowSubject::This) || !self.may_test(object) {
            return None;
        }
        self.parents.push(object);
        Some(FlowGuard::Property {
            object,
            key,
            test,
            positive,
            optional_chain: AnyJsOptionalChainExpression::cast_ref(operand.syntax())
                .is_some_and(|chain| chain.is_optional_chain()),
        })
    }

    /// Returns `guard` combined, as `&&` combines its operands, with a test
    /// that each of `receivers` that is a subject is not nullish.
    fn receiver_tests(&mut self, receivers: Vec<AnyJsExpression>, guard: FlowGuard) -> FlowGuard {
        let mut guard = guard;
        for receiver in receivers {
            let Some(subject) = subject_of(&receiver, self.model, self.subjects)
                .filter(|subject| self.may_test(*subject))
            else {
                continue;
            };
            self.mentions.push(subject);
            let left = self.push(FlowGuard::Test {
                subject,
                test: FlowTest::Nullish,
                positive: false,
            });
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
    /// pattern, to an initializer. TypeScript inlines nothing else.
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
        // TypeScript gives an `export const` a local symbol that is not a
        // `const`, so it never inlines one. Biome also skips a `const`
        // exported by a specifier, which TypeScript inlines.
        if binding.is_exported() {
            return None;
        }
        let AnyJsBindingDeclaration::JsVariableDeclarator(declarator) =
            binding.tree().declaration()?
        else {
            return None;
        };
        if declarator.variable_annotation().is_some()
            || declarator.declaration()?.variable_kind().ok()? != JsVariableKind::Const
        {
            return None;
        }
        let initializer = declarator.initializer()?.expression().ok()?;
        self.initializers.push(initializer.clone());
        Some(initializer)
    }

    /// Returns whether a test may narrow `subject`, which holds outside
    /// inlined initializers and, inside one, for the subjects that
    /// [`narrows_through_aliases`] accepts.
    fn may_test(&self, subject: FlowSubjectId) -> bool {
        self.aliases == 0 || narrows_through_aliases(self.subjects, subject, self.model)
    }

    fn push(&mut self, guard: FlowGuard) -> usize {
        self.guards.push(guard);
        self.guards.len() - 1
    }
}

/// Returns whether the condition inlined from an aliased condition may narrow
/// `subject`, an id in `subjects`.
///
/// TypeScript applies an inlined condition only to a reference whose value
/// cannot change between the declaration of the alias and its test
/// (`isConstantReference`): a variable that is never written and is declared
/// by `const`, as a parameter or a catch variable, or by `let` unless the
/// `let` statement is at the top level of a script, where other scripts can
/// write it. Biome cannot tell a script from a module, so it rejects a `let`
/// statement at the top level of any file.
///
/// Biome also rejects every exported variable, `this`, and every property
/// path, where TypeScript accepts a `const` exported by an `export { ... }`
/// list, `this`, and a readonly property. An alias declared in an enclosing
/// function would read another `this` than the root's.
#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "Only parameters, catch variables, and `const` and `let` declarations qualify."
)]
fn narrows_through_aliases(
    subjects: &FlowSubjectTable,
    subject: FlowSubjectId,
    model: &SemanticModel,
) -> bool {
    let Some(FlowSubject::Binding(range)) = subjects.as_slice().get(subject as usize) else {
        return false;
    };
    let Some(binding) = model.as_binding_by_range(*range) else {
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

/// Returns whether `declaration` is a variable statement directly in the body
/// of a module or script, not in a block, a loop head, or a namespace.
fn is_top_level_statement(declaration: &AnyJsVariableDeclaration) -> bool {
    declaration
        .syntax()
        .parent()
        .filter(|statement| statement.kind() == JsSyntaxKind::JS_VARIABLE_STATEMENT)
        .and_then(|statement| statement.grand_parent())
        .is_some_and(|body| {
            matches!(
                body.kind(),
                JsSyntaxKind::JS_MODULE | JsSyntaxKind::JS_SCRIPT
            )
        })
}

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
    /// A call to the guard function declared at this range.
    Predicate(TextRange),
}

/// One syntax step, with child expressions left undecomposed.
enum ConditionStep {
    Unsupported,
    /// A runtime test of `operand`, an expression that [`has_operand_shape`]
    /// accepts.
    Guard {
        operand: AnyJsExpression,
        guard: SyntaxGuard,
        positive: bool,
        /// Whether the result shows that `operand` is not `undefined`, so the
        /// receivers of an optional chain are not nullish either.
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
        SyntaxGuard::Truthy | SyntaxGuard::Predicate(_) => false,
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
/// Returns `None` when [`has_subject_shape`] does.
fn has_operand_shape(expression: &AnyJsExpression) -> Option<bool> {
    Some(
        has_subject_shape(expression)?
            || matches!(expression, AnyJsExpression::JsCallExpression(call) if call.is_optional_chain()),
    )
}

/// Returns whether `expression` has the shape of a subject: an identifier or a
/// member access, which [`subject_of`] accepts when its object is a subject
/// too and, for an element access, its key is a literal.
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
/// narrows `o.v` there, and the paths that extend it, but not `o`, except in
/// the truthiness of a condition. A read through `o?.v` then adds `undefined`
/// back from `o`, while a read in Biome keeps the narrowed type of its path.
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

    /// A decomposed `if` test, with the semantic model and the subject table
    /// that name its subjects.
    struct Decomposed {
        guards: Vec<FlowGuard>,
        condition: DecomposedCondition,
        model: SemanticModel,
        subjects: Vec<FlowSubject>,
    }

    /// Decomposes the `if` test `source` in a function with the parameters
    /// `value` and `other`.
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

    /// Decomposes the `if` test `source` and names the mentioned subjects.
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
    /// guard that they wrap, without the [`FlowGuard::Property`] that a test
    /// of a property adds to it.
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
            ("value?.a instanceof other", Truthy, vec![]),
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

    #[test]
    fn tests_of_properties_of_this_add_no_property_guard() {
        let program = "class K { kind: string; m() { if (this.kind === 'a') {} } }";
        let parsed = parse(program, JsFileSource::ts(), JsParserOptions::default());
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
        assert!(condition.parents.is_empty());
        assert!(
            guards
                .iter()
                .all(|guard| !matches!(guard, FlowGuard::Property { .. })),
            "{guards:?}"
        );
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
            // A receiver can narrow when its property path cannot.
            (
                "function f(value) { const ok = value?.a; if (ok) {} }",
                vec!["ok", "value"],
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
                vec!["ok"],
            ),
            (
                "function f(value) { const ok = value['a']; if (ok) {} }",
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
