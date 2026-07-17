use crate::services::typed::Typed;
use std::collections::HashSet;

use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsAssignment, AnyJsAssignmentPattern, AnyJsExpression, AnyJsTemplateElement,
    JsAssignmentExpression, JsAssignmentOperator, JsBinaryExpression, JsBinaryOperator,
    JsCallExpression, JsTemplateExpression, global_identifier,
};
use biome_js_type_info::{
    ImportSymbol, Literal, ResolvedTypeData, ResolverId, Type, TypeData, TypeId, TypeReference,
};
use biome_rowan::{AstNode, AstSeparatedList, TextRange, declare_node_union};
use biome_rule_options::no_base_to_string::NoBaseToStringOptions;

const DEFAULT_IGNORED_TYPE_NAMES: &[&str] = &["Error", "RegExp", "URL", "URLSearchParams"];

declare_lint_rule! {
    /// Require stringification to avoid values that only use the default object representation.
    ///
    /// JavaScript coerces values to strings in several places, such as `String(value)`,
    /// `value.toString()`, string concatenation, template interpolation, and `Array#join()`.
    /// When the value only inherits the default object stringification, that often produces
    /// `"[object Object]"` instead of something intentionally readable.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic,file=invalid-string.ts
    /// const value: {} = {};
    /// String(value);
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=invalid-template.ts
    /// const value: {} = {};
    /// `${value}`;
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=invalid-join.ts
    /// const values: {}[] = [{}];
    /// values.join(",");
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// String(1);
    /// ```
    ///
    /// ```ts
    /// class CustomToString {
    ///     toString() {
    ///         return "ok";
    ///     }
    /// }
    ///
    /// `${new CustomToString()}`;
    /// ```
    pub NoBaseToString {
        version: "2.4.15",
        name: "noBaseToString",
        language: "js",
        recommended: false,
        sources: &[RuleSource::EslintTypeScript("no-base-to-string").same()],
        domains: &[RuleDomain::Types],
    }
}

declare_node_union! {
    pub AnyNoBaseToStringQuery =
        JsAssignmentExpression
        | JsBinaryExpression
        | JsCallExpression
        | JsTemplateExpression
}

#[derive(Clone, Copy, Debug)]
pub struct RuleState {
    kind: DiagnosticKind,
    certainty: Usefulness,
    range: TextRange,
}

impl Rule for NoBaseToString {
    type Query = Typed<AnyNoBaseToStringQuery>;
    type State = RuleState;
    type Signals = Box<[Self::State]>;
    type Options = NoBaseToStringOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        match ctx.query() {
            AnyNoBaseToStringQuery::JsCallExpression(call) => {
                run_call_expression(ctx, call).into_iter().collect()
            }
            AnyNoBaseToStringQuery::JsBinaryExpression(binary) => {
                run_binary_expression(ctx, binary).into_iter().collect()
            }
            AnyNoBaseToStringQuery::JsAssignmentExpression(assignment) => {
                run_assignment_expression(ctx, assignment)
                    .into_iter()
                    .collect()
            }
            AnyNoBaseToStringQuery::JsTemplateExpression(template) => {
                run_template_expression(ctx, template).into_iter().collect()
            }
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let certainty = match state.certainty {
            Usefulness::Always => return None,
            Usefulness::Sometimes => "may",
            Usefulness::Never => "will",
        };

        let diagnostic = match state.kind {
            DiagnosticKind::BaseToString => RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! {
                    "This expression "{certainty}" use the default object stringification format, such as "<Emphasis>"[object Object]"</Emphasis>", when stringified."
                },
            )
            .note(markup! {
                "Stringify a primitive value, define a custom stringification method, or serialize the object explicitly."
            }),
            DiagnosticKind::BaseArrayJoin => RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! {
                    "This "<Emphasis>"join()"</Emphasis>" call "{certainty}" stringify one or more elements with the default object format, such as "<Emphasis>"[object Object]"</Emphasis>"."
                },
            )
            .note(markup! {
                "Ensure every joined element has a useful string representation, or map the values before joining."
            }),
        };

        Some(diagnostic)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DiagnosticKind {
    BaseToString,
    BaseArrayJoin,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Usefulness {
    Always,
    Sometimes,
    Never,
}

type VisitedTypes = HashSet<(ResolverId, TypeId)>;

#[derive(Clone, Copy)]
enum AnalysisMode {
    Join,
    ToString,
}

/// Worklist entries used by `collect_certainty()` to traverse type graphs.
enum AnalysisTask {
    /// Analyze a type under the given stringification mode.
    Eval(Type, AnalysisMode),
    /// Combine the next `usize` collected child results using the given strategy.
    Aggregate(AggregateKind, usize),
    /// Remove a type from the active traversal set after finishing its children.
    Leave((ResolverId, TypeId)),
}

/// Describes how a group of child results should be combined.
#[derive(Clone, Copy)]
enum AggregateKind {
    Intersection,
    Tuple,
    Union,
}

fn run_call_expression(
    ctx: &RuleContext<NoBaseToString>,
    node: &JsCallExpression,
) -> Option<RuleState> {
    run_builtin_string_call(ctx, node).or_else(|| run_member_call(ctx, node))
}

/// Handle `String(value)` calls.
fn run_builtin_string_call(
    ctx: &RuleContext<NoBaseToString>,
    node: &JsCallExpression,
) -> Option<RuleState> {
    let callee = node.callee().ok()?;
    let (reference, name) = global_identifier(
        &callee
            .clone()
            .omit_parentheses()
            .as_any_global_identifier_expression()?,
    )?;
    if name.text() != "String" || ctx.has_binding(&reference) {
        return None;
    }

    let args = node.arguments().ok()?.args();
    if args.len() != 1 {
        return None;
    }

    let first_arg = args.first()?.ok()?;
    let arg = first_arg.as_any_js_expression()?;
    check_expression(ctx, arg, DiagnosticKind::BaseToString)
}

/// Handle `value.join()`, `value.toString()`, and `value.toLocaleString()` calls.
fn run_member_call(
    ctx: &RuleContext<NoBaseToString>,
    node: &JsCallExpression,
) -> Option<RuleState> {
    let callee = node.callee().ok()?.omit_parentheses();
    let member = callee.as_js_static_member_expression()?;
    let property = member.member().ok()?;
    let property_token = property.value_token().ok()?;
    let property_name = property_token.text_trimmed();
    let object = member.object().ok()?;

    match property_name {
        "join" => check_expression(ctx, &object, DiagnosticKind::BaseArrayJoin),
        "toString" | "toLocaleString" => {
            check_expression(ctx, &object, DiagnosticKind::BaseToString)
        }
        _ => None,
    }
}

/// Handle `value + ""` and `"" + value`.
fn run_binary_expression(
    ctx: &RuleContext<NoBaseToString>,
    node: &JsBinaryExpression,
) -> Option<RuleState> {
    if node.operator().ok()? != JsBinaryOperator::Plus {
        return None;
    }

    let left = node.left().ok()?;
    let right = node.right().ok()?;

    if is_string_like_type(&ctx.type_of_expression(&left)) {
        return check_expression(ctx, &right, DiagnosticKind::BaseToString);
    }

    if is_string_like_type(&ctx.type_of_expression(&right)) {
        return check_expression(ctx, &left, DiagnosticKind::BaseToString);
    }

    None
}

/// Handle `value += ""`.
fn run_assignment_expression(
    ctx: &RuleContext<NoBaseToString>,
    node: &JsAssignmentExpression,
) -> Option<RuleState> {
    if node.operator().ok()? != JsAssignmentOperator::AddAssign {
        return None;
    }

    let left = node.left().ok()?;
    let left_ty = type_of_assignment_target(ctx, node, &left)?;
    if !is_string_like_type(&left_ty) {
        return None;
    }

    check_expression(ctx, &node.right().ok()?, DiagnosticKind::BaseToString)
}

/// Handle template strings like `${value}`.
fn run_template_expression(
    ctx: &RuleContext<NoBaseToString>,
    node: &JsTemplateExpression,
) -> Option<RuleState> {
    if node.tag().is_some() {
        return None;
    }

    node.elements()
        .into_iter()
        .find_map(|element| match element {
            AnyJsTemplateElement::JsTemplateElement(template_element) => {
                template_element.expression().ok().and_then(|expression| {
                    check_expression(ctx, &expression, DiagnosticKind::BaseToString)
                })
            }
            AnyJsTemplateElement::JsTemplateChunkElement(_) => None,
        })
}

fn type_of_assignment_target(
    ctx: &RuleContext<NoBaseToString>,
    assignment: &JsAssignmentExpression,
    left: &AnyJsAssignmentPattern,
) -> Option<Type> {
    match left {
        AnyJsAssignmentPattern::AnyJsAssignment(assignment_target) => {
            type_of_assignment(ctx, assignment, assignment_target)
        }
        AnyJsAssignmentPattern::JsArrayAssignmentPattern(_)
        | AnyJsAssignmentPattern::JsObjectAssignmentPattern(_) => None,
    }
}

fn type_of_assignment(
    ctx: &RuleContext<NoBaseToString>,
    _assignment: &JsAssignmentExpression,
    target: &AnyJsAssignment,
) -> Option<Type> {
    let mut current = target.clone();

    loop {
        match current {
            AnyJsAssignment::JsIdentifierAssignment(identifier) => {
                let name = identifier.name_token().ok()?;
                return Some(
                    ctx.type_of_named_value(name.text_trimmed_range(), name.text_trimmed()),
                );
            }
            AnyJsAssignment::JsParenthesizedAssignment(parenthesized) => {
                current = parenthesized.assignment().ok()?;
            }
            AnyJsAssignment::TsAsAssignment(ts_as) => {
                current = ts_as.assignment().ok()?;
            }
            AnyJsAssignment::TsNonNullAssertionAssignment(non_null) => {
                current = non_null.assignment().ok()?;
            }
            AnyJsAssignment::TsSatisfiesAssignment(satisfies) => {
                current = satisfies.assignment().ok()?;
            }
            AnyJsAssignment::TsTypeAssertionAssignment(assertion) => {
                current = assertion.assignment().ok()?;
            }
            AnyJsAssignment::JsBogusAssignment(_)
            | AnyJsAssignment::JsComputedMemberAssignment(_)
            | AnyJsAssignment::JsStaticMemberAssignment(_) => return None,
        }
    }
}

fn check_expression(
    ctx: &RuleContext<NoBaseToString>,
    expression: &AnyJsExpression,
    kind: DiagnosticKind,
) -> Option<RuleState> {
    if is_literal_expression(expression) {
        return None;
    }

    let ty = ctx.type_of_expression(expression);
    let certainty = match kind {
        DiagnosticKind::BaseArrayJoin => collect_certainty(&ty, ctx.options(), AnalysisMode::Join),
        DiagnosticKind::BaseToString => {
            collect_certainty(&ty, ctx.options(), AnalysisMode::ToString)
        }
    };

    if certainty == Usefulness::Always {
        None
    } else {
        Some(RuleState {
            kind,
            certainty,
            range: expression.range(),
        })
    }
}

fn is_literal_expression(expression: &AnyJsExpression) -> bool {
    matches!(
        expression.clone().omit_parentheses(),
        AnyJsExpression::AnyJsLiteralExpression(_) | AnyJsExpression::JsArrayExpression(_)
    )
}

fn is_string_like_type(ty: &Type) -> bool {
    let mut saw_variant = false;
    let mut pending = vec![ty.clone()];

    while let Some(current) = pending.pop() {
        if current.is_union() {
            let mut variants = current.flattened_union_variants().peekable();
            if variants.peek().is_none() {
                return false;
            }

            saw_variant = true;
            pending.extend(variants);
            continue;
        }

        let Some(raw) = current.resolved_data().map(ResolvedTypeData::as_raw_data) else {
            return false;
        };

        match raw {
            TypeData::Generic(generic) if generic.constraint.is_known() => {
                let Some(constraint) = current.resolve(&generic.constraint) else {
                    return false;
                };

                pending.push(constraint);
            }
            TypeData::Generic(_) => return false,
            _ if current.is_string_or_string_literal() => saw_variant = true,
            _ => return false,
        }
    }

    saw_variant
}

/// Computes whether stringification is always useful, sometimes useful, or never useful. The core business logic for the rule.
fn collect_certainty(ty: &Type, options: &NoBaseToStringOptions, mode: AnalysisMode) -> Usefulness {
    let mut active_types = HashSet::new();
    let mut results = Vec::new();
    let mut tasks = vec![AnalysisTask::Eval(ty.clone(), mode)];

    while let Some(task) = tasks.pop() {
        match task {
            AnalysisTask::Eval(ty, mode) => {
                let Some(key) = visited_type(&ty) else {
                    results.push(Usefulness::Always);
                    continue;
                };

                if active_types.contains(&key) {
                    results.push(Usefulness::Always);
                    continue;
                }

                if let Some(raw) = ty.resolved_data().map(ResolvedTypeData::as_raw_data) {
                    match raw {
                        TypeData::Generic(generic) if generic.constraint.is_known() => {
                            if let Some(constraint) = ty.resolve(&generic.constraint) {
                                tasks.push(AnalysisTask::Eval(constraint, mode));
                            } else {
                                results.push(Usefulness::Always);
                            }
                            continue;
                        }
                        TypeData::Generic(_) => {
                            results.push(Usefulness::Always);
                            continue;
                        }
                        _ => {}
                    }
                }

                if matches!(mode, AnalysisMode::ToString)
                    && (is_ignored_type(&ty, options, &mut HashSet::new())
                        || is_primitive_or_otherwise_safe(&ty))
                {
                    results.push(Usefulness::Always);
                    continue;
                }

                if push_aggregate_tasks(
                    &mut tasks,
                    &mut active_types,
                    key,
                    AggregateKind::Union,
                    ty.flattened_union_variants(),
                    mode,
                ) {
                    continue;
                }

                let Some(raw) = ty.resolved_data().map(ResolvedTypeData::as_raw_data) else {
                    results.push(Usefulness::Always);
                    continue;
                };

                if let TypeData::Intersection(intersection) = raw {
                    if !push_aggregate_tasks(
                        &mut tasks,
                        &mut active_types,
                        key,
                        AggregateKind::Intersection,
                        intersection
                            .types()
                            .iter()
                            .filter_map(|reference| ty.resolve(reference)),
                        mode,
                    ) {
                        results.push(Usefulness::Never);
                    }
                    continue;
                }

                if let TypeData::Tuple(tuple) = raw {
                    if !push_aggregate_tasks(
                        &mut tasks,
                        &mut active_types,
                        key,
                        AggregateKind::Tuple,
                        tuple
                            .elements()
                            .iter()
                            .filter_map(|element| ty.resolve(&element.ty)),
                        AnalysisMode::ToString,
                    ) {
                        results.push(Usefulness::Always);
                    }
                    continue;
                }

                if let Some(element_ty) = array_element_type(&ty) {
                    active_types.insert(key);
                    tasks.push(AnalysisTask::Leave(key));
                    tasks.push(AnalysisTask::Eval(element_ty, AnalysisMode::ToString));
                    continue;
                }

                let certainty = match mode {
                    AnalysisMode::Join => Usefulness::Always,
                    AnalysisMode::ToString => {
                        match is_to_string_like_from_object(&ty, &mut HashSet::new()) {
                            Some(true) => Usefulness::Never,
                            Some(false) | None => Usefulness::Always,
                        }
                    }
                };
                results.push(certainty);
            }
            AnalysisTask::Aggregate(kind, count) => {
                let start = results.len().saturating_sub(count);
                let combined = match kind {
                    AggregateKind::Intersection => combine_intersection(results.drain(start..)),
                    AggregateKind::Tuple => combine_tuple(results.drain(start..)),
                    AggregateKind::Union => combine_union(results.drain(start..)),
                };
                results.push(combined);
            }
            AnalysisTask::Leave(key) => {
                active_types.remove(&key);
            }
        }
    }

    results.pop().unwrap_or(Usefulness::Always)
}

fn push_aggregate_tasks(
    tasks: &mut Vec<AnalysisTask>,
    active_types: &mut VisitedTypes,
    key: (ResolverId, TypeId),
    kind: AggregateKind,
    types: impl Iterator<Item = Type>,
    mode: AnalysisMode,
) -> bool {
    active_types.insert(key);
    tasks.push(AnalysisTask::Leave(key));
    let aggregate_index = tasks.len();
    tasks.push(AnalysisTask::Aggregate(kind, 0));

    let mut count = 0;
    for ty in types {
        tasks.push(AnalysisTask::Eval(ty, mode));
        count += 1;
    }

    if count == 0 {
        tasks.pop();
        tasks.pop();
        active_types.remove(&key);
        return false;
    }

    tasks[aggregate_index] = AnalysisTask::Aggregate(kind, count);
    true
}

fn array_element_type(ty: &Type) -> Option<Type> {
    if !ty.is_array_of(|_| true) {
        return None;
    }

    let TypeData::InstanceOf(instance) = ty.resolved_data().map(ResolvedTypeData::as_raw_data)?
    else {
        return None;
    };

    let element_ty = instance
        .type_parameters
        .first()
        .and_then(|reference| ty.resolve(reference))?;

    Some(element_ty)
}

/// Combine the certainties of union variants. If all variants have the same certainty, that is returned. Otherwise, `Sometimes` is returned.
fn combine_union(certainties: impl Iterator<Item = Usefulness>) -> Usefulness {
    let mut combined = None;

    for certainty in certainties {
        match combined {
            None => combined = Some(certainty),
            Some(existing) if existing == certainty => {}
            Some(_) => return Usefulness::Sometimes,
        }
    }

    combined.unwrap_or(Usefulness::Always)
}

/// Combine the certainties of intersection constituents. If any constituent is `Always`, the result is `Always`. Otherwise, the result is `Never`.
fn combine_intersection(certainties: impl Iterator<Item = Usefulness>) -> Usefulness {
    if certainties
        .into_iter()
        .any(|certainty| certainty == Usefulness::Always)
    {
        Usefulness::Always
    } else {
        Usefulness::Never
    }
}

/// Combine the certainties of tuple elements. If any element is `Never`, the result is `Never`. Otherwise, if any element is `Sometimes`, the result is `Sometimes`. Otherwise, the result is `Always`.
fn combine_tuple(certainties: impl Iterator<Item = Usefulness>) -> Usefulness {
    let mut saw_sometimes = false;

    for certainty in certainties {
        match certainty {
            Usefulness::Always => {}
            Usefulness::Sometimes => saw_sometimes = true,
            Usefulness::Never => return Usefulness::Never,
        }
    }

    if saw_sometimes {
        Usefulness::Sometimes
    } else {
        Usefulness::Always
    }
}

fn is_primitive_or_otherwise_safe(ty: &Type) -> bool {
    if ty.is_string_or_string_literal() || ty.is_number_or_number_literal() {
        return true;
    }

    let Some(raw) = ty.resolved_data().map(ResolvedTypeData::as_raw_data) else {
        // if we can't resolve the type, assume it's safe to avoid false positives
        return true;
    };

    match raw {
        TypeData::AnyKeyword
        | TypeData::BigInt
        | TypeData::Boolean
        | TypeData::Function(_)
        | TypeData::Null
        | TypeData::Number
        | TypeData::String
        | TypeData::Symbol
        | TypeData::Undefined
        | TypeData::Unknown
        | TypeData::UnknownKeyword
        | TypeData::NeverKeyword
        | TypeData::VoidKeyword => true,
        TypeData::Literal(literal) => matches!(
            literal.as_ref(),
            Literal::BigInt(_)
                | Literal::Boolean(_)
                | Literal::Number(_)
                | Literal::String(_)
                | Literal::Template(_)
        ),
        _ => false,
    }
}

/// Returns whether `ty` ultimately falls back to object-style stringification
/// like `[object Object]`, which is what the rule wants to flag.
///
/// `Some(true)` means the reachable type graph bottoms out in plain object-like
/// behavior without a custom `toString`-like member. `Some(false)` means a
/// custom stringification member was found. `None` means the type shape is not
/// one this rule can classify with confidence.
fn is_to_string_like_from_object(ty: &Type, visited: &mut VisitedTypes) -> Option<bool> {
    let mut stack = vec![ty.clone()];
    let mut saw_true = false;
    let mut saw_unknown = false;

    while let Some(current) = stack.pop() {
        let Some(key) = visited_type(&current) else {
            saw_unknown = true;
            continue;
        };

        if !visited.insert(key) {
            return Some(false);
        }

        let Some(raw) = current.resolved_data().map(ResolvedTypeData::as_raw_data) else {
            saw_unknown = true;
            continue;
        };

        if has_custom_stringification_member(raw) {
            return Some(false);
        }

        match raw {
            TypeData::Class(class) => {
                if let Some(base) = class
                    .extends
                    .as_ref()
                    .and_then(|reference| current.resolve(reference))
                {
                    stack.push(base);
                } else {
                    saw_true = true;
                }
            }
            TypeData::InstanceOf(instance) => {
                if let Some(base) = current.resolve(&instance.ty) {
                    stack.push(base);
                } else {
                    saw_true = true;
                }
            }
            TypeData::Interface(interface) => {
                let mut bases = interface
                    .extends
                    .iter()
                    .filter_map(|reference| current.resolve(reference))
                    .peekable();
                if bases.peek().is_none() {
                    saw_true = true;
                } else {
                    stack.extend(bases);
                }
            }
            TypeData::Literal(literal) => match literal.as_ref() {
                Literal::Object(object) => {
                    if has_custom_object_literal_member(object.members()) {
                        return Some(false);
                    } else {
                        saw_true = true;
                    }
                }
                Literal::RegExp(_) => saw_true = true,
                _ => return Some(false),
            },
            TypeData::MergedReference(reference) => {
                if let Some(resolved) = reference
                    .ty
                    .as_ref()
                    .and_then(|reference| current.resolve(reference))
                {
                    stack.push(resolved);
                } else {
                    saw_unknown = true;
                }
            }
            TypeData::Object(_) | TypeData::ObjectKeyword => saw_true = true,
            TypeData::Reference(reference) => {
                if let Some(resolved) = current.resolve(reference) {
                    stack.push(resolved);
                } else {
                    saw_unknown = true;
                }
            }
            TypeData::TypeofValue(value) => {
                if let Some(resolved) = current.resolve(&value.ty) {
                    stack.push(resolved);
                } else {
                    saw_unknown = true;
                }
            }
            _ => return None,
        }
    }

    if saw_true {
        Some(true)
    } else {
        let _ = saw_unknown;
        None
    }
}

fn has_custom_stringification_member(raw: &TypeData) -> bool {
    raw.own_members().any(|member| {
        member.has_name("toLocaleString")
            || member.has_name("toString")
            || member.has_name("valueOf")
    })
}

fn has_custom_object_literal_member(members: &[biome_js_type_info::TypeMember]) -> bool {
    members.iter().any(|member| {
        member.has_name("toLocaleString")
            || member.has_name("toString")
            || member.has_name("valueOf")
    })
}

fn is_ignored_type(ty: &Type, options: &NoBaseToStringOptions, visited: &mut VisitedTypes) -> bool {
    let mut stack = vec![ty.clone()];

    while let Some(current) = stack.pop() {
        let Some(raw) = current.resolved_data().map(ResolvedTypeData::as_raw_data) else {
            continue;
        };
        let Some(visited_key) = visited_type(&current) else {
            continue;
        };

        if !visited.insert(visited_key) {
            continue;
        }

        if matches_ignored_type_name(raw, options) {
            return true;
        }

        match raw {
            TypeData::Class(class) => {
                if let Some(base) = class
                    .extends
                    .as_ref()
                    .and_then(|reference| current.resolve(reference))
                {
                    stack.push(base);
                }
            }
            TypeData::Generic(generic) if generic.constraint.is_known() => {
                if let Some(constraint) = current.resolve(&generic.constraint) {
                    stack.push(constraint);
                }
            }
            TypeData::InstanceOf(instance) => {
                if let Some(base) = current.resolve(&instance.ty) {
                    stack.push(base);
                }
            }
            TypeData::Interface(interface) => {
                stack.extend(
                    interface
                        .extends
                        .iter()
                        .filter_map(|reference| current.resolve(reference)),
                );
            }
            TypeData::MergedReference(reference) => {
                if let Some(resolved) = reference
                    .ty
                    .as_ref()
                    .and_then(|reference| current.resolve(reference))
                {
                    stack.push(resolved);
                }
            }
            TypeData::Reference(reference) => {
                if let Some(resolved) = current.resolve(reference) {
                    stack.push(resolved);
                }
            }
            TypeData::TypeofType(reference) => {
                if let Some(resolved) = current.resolve(reference) {
                    stack.push(resolved);
                }
            }
            TypeData::TypeOperator(operator) => {
                if let Some(resolved) = current.resolve(&operator.ty) {
                    stack.push(resolved);
                }
            }
            TypeData::TypeofValue(value) => {
                if let Some(resolved) = current.resolve(&value.ty) {
                    stack.push(resolved);
                }
            }
            _ => {}
        }
    }

    false
}

fn matches_ignored_type_name(raw: &TypeData, options: &NoBaseToStringOptions) -> bool {
    match raw {
        TypeData::Class(class) => class
            .name
            .as_ref()
            .is_some_and(|name| is_ignored_type_name(name.text(), options)),
        TypeData::Generic(generic) => is_ignored_type_name(generic.name.text(), options),
        TypeData::InstanceOf(instance) => matches_ignored_type_reference(&instance.ty, options),
        TypeData::Interface(interface) => is_ignored_type_name(interface.name.text(), options),
        TypeData::Literal(literal) => {
            matches!(literal.as_ref(), Literal::RegExp(_))
                && is_ignored_type_name("RegExp", options)
        }
        TypeData::MergedReference(reference) => {
            reference
                .ty
                .as_ref()
                .is_some_and(|reference| matches_ignored_type_reference(reference, options))
                || reference
                    .value_ty
                    .as_ref()
                    .is_some_and(|reference| matches_ignored_type_reference(reference, options))
                || reference
                    .namespace_ty
                    .as_ref()
                    .is_some_and(|reference| matches_ignored_type_reference(reference, options))
        }
        TypeData::Reference(reference) => matches_ignored_type_reference(reference, options),
        TypeData::TypeofType(reference) => matches_ignored_type_reference(reference, options),
        TypeData::TypeOperator(operator) => matches_ignored_type_reference(&operator.ty, options),
        TypeData::TypeofValue(value) => is_ignored_type_name(value.identifier.text(), options),
        _ => false,
    }
}

fn matches_ignored_type_reference(
    reference: &TypeReference,
    options: &NoBaseToStringOptions,
) -> bool {
    let mut stack = vec![reference];

    while let Some(reference) = stack.pop() {
        match reference {
            TypeReference::Qualifier(qualifier) => {
                if qualifier
                    .path
                    .iter()
                    .any(|part| is_ignored_type_name(part.text(), options))
                {
                    return true;
                }

                stack.extend(qualifier.type_parameters.iter());
            }
            TypeReference::Import(import) => {
                if let ImportSymbol::Named(name) = &import.symbol
                    && is_ignored_type_name(name.text(), options)
                {
                    return true;
                }
            }
            TypeReference::Resolved(_) => {}
        }
    }

    false
}

fn is_ignored_type_name(name: &str, options: &NoBaseToStringOptions) -> bool {
    options.ignored_type_names.as_deref().map_or_else(
        || DEFAULT_IGNORED_TYPE_NAMES.contains(&name),
        |ignored| ignored.iter().any(|candidate| candidate.as_ref() == name),
    )
}

fn visited_type(ty: &Type) -> Option<(ResolverId, TypeId)> {
    Some((ty.resolved_data()?.resolver_id(), ty.id()))
}
