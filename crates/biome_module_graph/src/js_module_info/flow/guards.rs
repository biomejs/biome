use biome_js_semantic::{Binding, JsDeclarationKind, SemanticModel};
use biome_js_syntax::{
    AnyJsArrowFunctionParameters, AnyJsCallArgument, AnyJsExpression, AnyJsFormalParameter,
    AnyJsLiteralExpression, AnyJsParameter, JsBinaryOperator, JsCallExpression,
    JsIdentifierExpression, JsLogicalOperator, JsReferenceIdentifier, JsUnaryOperator,
    binding_ext::AnyJsBindingDeclaration, unescape_js_identifier,
};
use biome_js_type_info::{
    Function, FunctionParameter, RawTypeData, RawTypeId, ReturnType, TypeReference, TypeofKind,
    may_resolve_to_raw_primitive,
};
use biome_rowan::{AstNode, AstSeparatedList, SyntaxKind, Text, TextRange};
use rustc_hash::FxHashMap;
use std::borrow::Cow;

/// Limits the nested `!`, `&&`, and `||` operators decomposed in one
/// condition. Deeper operands keep the incoming type.
const MAX_CONDITION_DEPTH: usize = 32;
/// Limits the syntax steps spent decomposing one condition.
const MAX_CONDITION_STEPS: usize = 16_384;
/// Limits the descendants scanned for variables when decomposition stops early.
const MAX_FALLBACK_NODES: usize = 1024;

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
/// Applying a guard to a variable's incoming type yields its type once the
/// condition holds. `Keep` and tests of other variables return the incoming
/// type, while `Incomplete` makes it unknown.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum FlowGuard {
    /// The step cannot refine any variable, so the incoming type is kept.
    Keep,
    /// Decomposition stopped at its work limit or at missing syntax, so the
    /// incoming type becomes unknown.
    Incomplete,
    /// A runtime test of the variable declared at `binding`.
    Test {
        binding: TextRange,
        test: FlowTest,
        positive: bool,
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
    /// Whether the module may read the global `eval` or the implicit
    /// `arguments` object. A direct `eval` can reassign any guard, so the
    /// module then has no supported guard.
    has_dynamic_scope: bool,
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
        has_dynamic_scope: bool,
    ) -> Self {
        Self {
            raw_types,
            raw_binding_types,
            has_dynamic_scope,
            guards: FxHashMap::default(),
        }
    }

    /// Returns the variable that `call` tests, with the range of the type
    /// predicate it calls.
    ///
    /// Returns `None` unless `call` calls a supported type predicate and passes
    /// an identifier as the tested argument.
    fn tested_variable(
        &mut self,
        call: &JsCallExpression,
        model: &SemanticModel,
        remaining: &mut usize,
    ) -> Option<(JsReferenceIdentifier, TextRange)> {
        let (guard, function, argument) = self.guard_call(call, model, remaining)?;
        if guard.kind != GuardKind::Predicate {
            return None;
        }
        let AnyJsExpression::JsIdentifierExpression(subject) = argument else {
            return None;
        };
        Some((subject.name().ok()?, function))
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
                let guard = self.guard(&function);
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
    /// initializes a variable. Nothing reassigns it, and it is not a generator.
    /// Its tested parameter is named and is not a rest parameter. That
    /// parameter has no default value, because a default replaces an
    /// `undefined` argument. A target type must be able to resolve to a
    /// primitive type or a primitive literal, the only targets that narrow.
    fn guard(&self, function: &Binding) -> Option<Guard> {
        if self.has_dynamic_scope
            || function.is_imported()
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
            Some(target) if !may_resolve_to_raw_primitive(target, self.raw_types) => return None,
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
}

/// Decomposes the condition that a standalone call `expression` establishes
/// when it returns, or returns `None` when `expression` does not call a
/// supported assertion.
///
/// `asserts value is T` tests an identifier argument against the guard's
/// target. `asserts value` decomposes the argument as a condition that holds,
/// so `assert(value !== null)` excludes `null` from `value`.
pub(super) fn decompose_assertion(
    expression: &AnyJsExpression,
    model: &SemanticModel,
    functions: &mut GuardFunctions,
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
            guards,
        )),
        GuardKind::TypeAssertion => {
            let AnyJsExpression::JsIdentifierExpression(subject) = argument else {
                return None;
            };
            let binding = model.binding(&subject.name().ok()?)?.range();
            guards.push(FlowGuard::Test {
                binding,
                test: FlowTest::Predicate(function),
                positive: true,
            });
            Some(DecomposedCondition {
                guard: guards.len() - 1,
                mentions: vec![binding],
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
    /// Bindings whose type the condition may refine.
    ///
    /// When decomposition stops early, this conservatively lists every
    /// variable read directly in the condition.
    pub(super) mentions: Vec<TextRange>,
}

/// Decomposes `expression`, assuming `outcome`, into guards appended to `guards`.
pub(super) fn decompose_condition(
    expression: &AnyJsExpression,
    outcome: FlowOutcome,
    model: &SemanticModel,
    functions: &mut GuardFunctions,
    guards: &mut Vec<FlowGuard>,
) -> DecomposedCondition {
    let mut decomposer = Decomposer {
        model,
        functions,
        guards,
        remaining: MAX_CONDITION_STEPS,
        mentions: Vec::new(),
        complete: true,
    };
    let guard = decomposer.guard(expression.clone(), outcome, 0);
    let mentions = if decomposer.complete {
        decomposer.mentions
    } else {
        expression
            .syntax()
            .descendants()
            .take(MAX_FALLBACK_NODES)
            .filter_map(JsIdentifierExpression::cast)
            .filter_map(|identifier| model.binding(&identifier.name().ok()?))
            .map(|binding| binding.range())
            .collect()
    };
    DecomposedCondition { guard, mentions }
}

struct Decomposer<'a, 'raw> {
    model: &'a SemanticModel,
    functions: &'a mut GuardFunctions<'raw>,
    guards: &'a mut Vec<FlowGuard>,
    remaining: usize,
    mentions: Vec<TextRange>,
    complete: bool,
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
                subject,
                guard,
                positive,
            } => self.test(&subject, guard, positive),
            ConditionStep::Call { call, positive } => {
                match self
                    .functions
                    .tested_variable(&call, self.model, &mut self.remaining)
                {
                    Some((subject, function)) => {
                        self.test(&subject, SyntaxGuard::Predicate(function), positive)
                    }
                    None => FlowGuard::Keep,
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

    fn test(
        &mut self,
        subject: &JsReferenceIdentifier,
        guard: SyntaxGuard,
        positive: bool,
    ) -> FlowGuard {
        let test = match guard {
            SyntaxGuard::Truthy => FlowTest::Truthy,
            SyntaxGuard::Nullish => FlowTest::Nullish,
            SyntaxGuard::Typeof(kind) => FlowTest::Typeof(kind),
            SyntaxGuard::Literal(AnyJsLiteralExpression::JsNullLiteralExpression(_)) => {
                FlowTest::Null
            }
            SyntaxGuard::Literal(literal) => FlowTest::Literal(literal.range()),
            // A local variable named `undefined` can hold any value.
            SyntaxGuard::Undefined { reference, .. }
                if self.model.binding(&reference).is_some() =>
            {
                return FlowGuard::Keep;
            }
            SyntaxGuard::Undefined { strict, .. } => FlowTest::Undefined { strict },
            SyntaxGuard::Predicate(function) => FlowTest::Predicate(function),
        };
        let Some(binding) = self.model.binding(subject) else {
            return FlowGuard::Keep;
        };
        self.mentions.push(binding.range());
        FlowGuard::Test {
            binding: binding.range(),
            test,
            positive,
        }
    }

    fn push(&mut self, guard: FlowGuard) -> usize {
        self.guards.push(guard);
        self.guards.len() - 1
    }
}

enum SyntaxGuard {
    Truthy,
    Nullish,
    Typeof(TypeofKind),
    Literal(AnyJsLiteralExpression),
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
    Guard {
        subject: JsReferenceIdentifier,
        guard: SyntaxGuard,
        positive: bool,
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
    if let AnyJsExpression::JsIdentifierExpression(identifier) = expression {
        let subject = identifier.name().ok()?;
        subject.value_token().ok()?;
        let guard = match outcome {
            FlowOutcome::Truthy | FlowOutcome::Falsy => SyntaxGuard::Truthy,
            FlowOutcome::Nullish | FlowOutcome::NonNullish => SyntaxGuard::Nullish,
        };
        return Some(ConditionStep::Guard {
            subject,
            guard,
            positive,
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
    reason = "Comparisons only support direct identifiers and primitive guards."
)]
fn comparison_step(
    subject: &AnyJsExpression,
    value: &AnyJsExpression,
    strict: bool,
    positive: bool,
    remaining: &mut usize,
) -> Option<ConditionStep> {
    let (subject, guard) = match subject {
        AnyJsExpression::JsIdentifierExpression(identifier) => {
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
            (identifier.name().ok()?, guard)
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
            let AnyJsExpression::JsIdentifierExpression(identifier) =
                omit_parentheses(argument, remaining)?
            else {
                return Some(ConditionStep::Unsupported);
            };
            (identifier.name().ok()?, SyntaxGuard::Typeof(kind))
        }
        _ => return Some(ConditionStep::Unsupported),
    };
    subject.value_token().ok()?;
    Some(ConditionStep::Guard {
        subject,
        guard,
        positive,
    })
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
    use super::*;
    use biome_js_parser::{JsParserOptions, parse};
    use biome_js_semantic::{SemanticModelOptions, semantic_model};
    use biome_js_syntax::JsIfStatement;
    use biome_languages::JsFileSource;

    /// Decomposes the `if` test `source` and names the mentioned bindings.
    fn decompose(source: &str, outcome: FlowOutcome) -> (Vec<FlowGuard>, Vec<String>) {
        let parsed = parse(
            &format!("function f(value, other) {{ if ({source}) {{}} }}"),
            JsFileSource::ts(),
            JsParserOptions::default(),
        );
        let model = semantic_model(&parsed.tree(), SemanticModelOptions::default());
        let test = parsed
            .syntax()
            .descendants()
            .find_map(JsIfStatement::cast)
            .unwrap()
            .test()
            .unwrap();
        let mut guards = Vec::new();
        let raw_binding_types = FxHashMap::default();
        let mut functions = GuardFunctions::new(&[], &raw_binding_types, false);
        let condition = decompose_condition(&test, outcome, &model, &mut functions, &mut guards);
        let names = condition
            .mentions
            .iter()
            .map(|range| {
                model
                    .as_binding_by_range(*range)
                    .unwrap()
                    .tree()
                    .name_token()
                    .unwrap()
                    .text_trimmed()
                    .to_owned()
            })
            .collect();
        (guards, names)
    }

    fn mentioned_names(source: &str, outcome: FlowOutcome) -> Vec<String> {
        decompose(source, outcome).1
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
            ("value.length", FlowOutcome::Truthy, vec![]),
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
        let source = format!("{}value && other.length", "!".repeat(MAX_CONDITION_DEPTH));
        assert_eq!(
            mentioned_names(&source, FlowOutcome::Truthy),
            ["value", "other"]
        );
    }
}
