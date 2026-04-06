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
use biome_js_type_info::{Literal, ResolvedTypeData, ResolverId, Type, TypeData, TypeId};
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
    /// This rule reports stringification sites that are known to use that default object
    /// formatting. It supports the upstream `ignoredTypeNames` option, but intentionally does
    /// not implement `checkUnknown`.
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
        version: "next",
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

enum AnalysisTask {
    Eval(Type, AnalysisMode),
    Aggregate(AggregateKind, usize),
    Leave((ResolverId, TypeId)),
}

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

fn run_builtin_string_call(
    ctx: &RuleContext<NoBaseToString>,
    node: &JsCallExpression,
) -> Option<RuleState> {
    let callee = node.callee().ok()?;
    let (reference, name) = global_identifier(&callee.clone().omit_parentheses())?;
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

fn run_binary_expression(
    ctx: &RuleContext<NoBaseToString>,
    node: &JsBinaryExpression,
) -> Option<RuleState> {
    if node.operator().ok()? != JsBinaryOperator::Plus {
        return None;
    }

    let left = node.left().ok()?;
    let right = node.right().ok()?;

    if ctx.type_of_expression(&left).is_string_or_string_literal() {
        return check_expression(ctx, &right, DiagnosticKind::BaseToString);
    }

    if ctx.type_of_expression(&right).is_string_or_string_literal() {
        return check_expression(ctx, &left, DiagnosticKind::BaseToString);
    }

    None
}

fn run_assignment_expression(
    ctx: &RuleContext<NoBaseToString>,
    node: &JsAssignmentExpression,
) -> Option<RuleState> {
    if node.operator().ok()? != JsAssignmentOperator::AddAssign {
        return None;
    }

    let left = node.left().ok()?;
    let left_ty = type_of_assignment_target(ctx, node, &left)?;
    if !left_ty.is_string_or_string_literal() {
        return None;
    }

    check_expression(ctx, &node.right().ok()?, DiagnosticKind::BaseToString)
}

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
    assignment: &JsAssignmentExpression,
    target: &AnyJsAssignment,
) -> Option<Type> {
    match target {
        AnyJsAssignment::JsIdentifierAssignment(identifier) => {
            let name = identifier.name_token().ok()?;
            Some(ctx.type_of_named_value(assignment.range(), name.text_trimmed()))
        }
        AnyJsAssignment::JsParenthesizedAssignment(parenthesized) => {
            type_of_assignment(ctx, assignment, &parenthesized.assignment().ok()?)
        }
        AnyJsAssignment::TsAsAssignment(ts_as) => {
            type_of_assignment(ctx, assignment, &ts_as.assignment().ok()?)
        }
        AnyJsAssignment::TsNonNullAssertionAssignment(non_null) => {
            type_of_assignment(ctx, assignment, &non_null.assignment().ok()?)
        }
        AnyJsAssignment::TsSatisfiesAssignment(satisfies) => {
            type_of_assignment(ctx, assignment, &satisfies.assignment().ok()?)
        }
        AnyJsAssignment::TsTypeAssertionAssignment(assertion) => {
            type_of_assignment(ctx, assignment, &assertion.assignment().ok()?)
        }
        AnyJsAssignment::JsBogusAssignment(_)
        | AnyJsAssignment::JsComputedMemberAssignment(_)
        | AnyJsAssignment::JsStaticMemberAssignment(_) => None,
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

    (certainty != Usefulness::Always).then_some(RuleState {
        kind,
        certainty,
        range: expression.range(),
    })
}

fn is_literal_expression(expression: &AnyJsExpression) -> bool {
    matches!(
        expression.clone().omit_parentheses(),
        AnyJsExpression::AnyJsLiteralExpression(_) | AnyJsExpression::JsArrayExpression(_)
    )
}

/// Computes whether stringification is always useful, sometimes useful, or never useful
/// without recursion.
///
/// The analysis walks nested unions, intersections, tuples, arrays, aliases, and
/// inheritance edges with an explicit worklist.
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

                let union_variants: Vec<_> = ty.flattened_union_variants().collect();
                if !union_variants.is_empty() {
                    active_types.insert(key);
                    tasks.push(AnalysisTask::Leave(key));
                    tasks.push(AnalysisTask::Aggregate(
                        AggregateKind::Union,
                        union_variants.len(),
                    ));
                    for variant in union_variants.into_iter().rev() {
                        tasks.push(AnalysisTask::Eval(variant, mode));
                    }
                    continue;
                }

                let Some(raw) = ty.resolved_data().map(ResolvedTypeData::as_raw_data) else {
                    results.push(Usefulness::Always);
                    continue;
                };

                if let TypeData::Intersection(intersection) = raw {
                    let variants: Vec<_> = intersection
                        .types()
                        .iter()
                        .filter_map(|reference| ty.resolve(reference))
                        .collect();

                    if variants.is_empty() {
                        results.push(Usefulness::Never);
                    } else {
                        active_types.insert(key);
                        tasks.push(AnalysisTask::Leave(key));
                        tasks.push(AnalysisTask::Aggregate(
                            AggregateKind::Intersection,
                            variants.len(),
                        ));
                        for variant in variants.into_iter().rev() {
                            tasks.push(AnalysisTask::Eval(variant, mode));
                        }
                    }
                    continue;
                }

                if let Some(tuple_elements) = tuple_element_types(&ty) {
                    if tuple_elements.is_empty() {
                        results.push(Usefulness::Always);
                    } else {
                        active_types.insert(key);
                        tasks.push(AnalysisTask::Leave(key));
                        tasks.push(AnalysisTask::Aggregate(
                            AggregateKind::Tuple,
                            tuple_elements.len(),
                        ));
                        for element in tuple_elements.into_iter().rev() {
                            tasks.push(AnalysisTask::Eval(element, AnalysisMode::ToString));
                        }
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
                let values = results.split_off(start);
                let combined = match kind {
                    AggregateKind::Intersection => combine_intersection(values.into_iter()),
                    AggregateKind::Tuple => combine_tuple(values.into_iter()),
                    AggregateKind::Union => combine_union(values.into_iter()),
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

fn tuple_element_types(ty: &Type) -> Option<Vec<Type>> {
    let raw = ty.resolved_data().map(ResolvedTypeData::as_raw_data)?;
    let TypeData::Tuple(tuple) = raw else {
        return None;
    };

    Some(
        tuple
            .elements()
            .iter()
            .filter_map(|element| ty.resolve(&element.ty))
            .collect(),
    )
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

fn combine_union(certainties: impl Iterator<Item = Usefulness>) -> Usefulness {
    let certainties: Vec<_> = certainties.collect();
    if certainties.is_empty() {
        return Usefulness::Always;
    }

    if certainties
        .iter()
        .all(|certainty| *certainty == Usefulness::Never)
    {
        Usefulness::Never
    } else if certainties
        .iter()
        .all(|certainty| *certainty == Usefulness::Always)
    {
        Usefulness::Always
    } else {
        Usefulness::Sometimes
    }
}

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
                let bases: Vec<_> = interface
                    .extends
                    .iter()
                    .filter_map(|reference| current.resolve(reference))
                    .collect();
                if bases.is_empty() {
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
    let is_ignored_name = |name: &str| {
        options.ignored_type_names.as_deref().map_or_else(
            || DEFAULT_IGNORED_TYPE_NAMES.contains(&name),
            |ignored| ignored.iter().any(|candidate| candidate.as_ref() == name),
        )
    };

    let is_ignored_key = |key: &str| {
        options.ignored_type_names.as_deref().map_or_else(
            || {
                DEFAULT_IGNORED_TYPE_NAMES
                    .iter()
                    .any(|ignored| key.contains(ignored))
            },
            |ignored| {
                ignored
                    .iter()
                    .any(|candidate| key.contains(candidate.as_ref()))
            },
        )
    };

    let mut stack = vec![ty.clone()];

    while let Some(current) = stack.pop() {
        let key = current.to_string();
        let Some(visited_key) = visited_type(&current) else {
            continue;
        };

        if !visited.insert(visited_key) {
            continue;
        }

        if type_name(&current)
            .or_else(|| extract_identifier(&key))
            .is_some_and(is_ignored_name)
            || is_ignored_key(&key)
        {
            return true;
        }

        match current.resolved_data().map(ResolvedTypeData::as_raw_data) {
            Some(TypeData::Class(class)) => {
                if let Some(base) = class
                    .extends
                    .as_ref()
                    .and_then(|reference| current.resolve(reference))
                {
                    stack.push(base);
                }
            }
            Some(TypeData::Generic(generic)) if generic.constraint.is_known() => {
                if let Some(constraint) = current.resolve(&generic.constraint) {
                    stack.push(constraint);
                }
            }
            Some(TypeData::InstanceOf(instance)) => {
                if let Some(base) = current.resolve(&instance.ty) {
                    stack.push(base);
                }
            }
            Some(TypeData::Interface(interface)) => {
                stack.extend(
                    interface
                        .extends
                        .iter()
                        .filter_map(|reference| current.resolve(reference)),
                );
            }
            Some(TypeData::MergedReference(reference)) => {
                if let Some(resolved) = reference
                    .ty
                    .as_ref()
                    .and_then(|reference| current.resolve(reference))
                {
                    stack.push(resolved);
                }
            }
            Some(TypeData::Reference(reference)) => {
                if let Some(resolved) = current.resolve(reference) {
                    stack.push(resolved);
                }
            }
            Some(TypeData::TypeofValue(value)) => {
                if let Some(resolved) = current.resolve(&value.ty) {
                    stack.push(resolved);
                }
            }
            _ => {}
        }
    }

    false
}

fn visited_type(ty: &Type) -> Option<(ResolverId, TypeId)> {
    Some((ty.resolved_data()?.resolver_id(), ty.id()))
}

fn type_name(ty: &Type) -> Option<&str> {
    let raw = ty.resolved_data().map(ResolvedTypeData::as_raw_data)?;
    match raw {
        TypeData::Class(class) => class.name.as_ref().map(|name| name.text()),
        TypeData::Generic(generic) => Some(generic.name.text()),
        TypeData::Interface(interface) => Some(interface.name.text()),
        TypeData::Literal(literal) => match literal.as_ref() {
            Literal::RegExp(_) => Some("RegExp"),
            _ => None,
        },
        TypeData::TypeofValue(value) => Some(value.identifier.text()),
        _ => None,
    }
}

fn extract_identifier(text: &str) -> Option<&str> {
    let text = text
        .strip_prefix("instanceof ")
        .or_else(|| text.strip_prefix("typeof "))
        .unwrap_or(text);

    let end = text
        .find(|character: char| {
            !(character.is_ascii_alphanumeric() || character == '_' || character == '$')
        })
        .unwrap_or(text.len());

    (!text.is_empty() && end > 0).then_some(&text[..end])
}
