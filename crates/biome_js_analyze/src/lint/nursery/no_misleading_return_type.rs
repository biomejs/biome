use biome_analyze::{Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsBinding, AnyJsExpression, AnyJsFunction, AnyJsFunctionBody, AnyJsGetter,
    AnyTsCastExpression, AnyTsIdentifierBinding, AnyTsType, JsArrowFunctionExpression,
    JsClassDeclaration, JsConstructorClassMember, JsFunctionBody, JsFunctionDeclaration,
    JsFunctionExpression, JsGetterClassMember, JsGetterObjectMember, JsMethodClassMember,
    JsMethodObjectMember, JsReturnStatement, JsSetterClassMember, JsSetterObjectMember,
    JsSyntaxNode, JsVariableDeclarator, JsVariableStatement, TsAsExpression,
    TsInterfaceDeclaration, TsMethodSignatureClassMember, TsReferenceType,
    TsTypeAliasDeclaration, TsTypeAssertionExpression,
};
use biome_js_type_info::{Literal, Type, TypeData};
use biome_rowan::{AstNode, TextRange, TokenText, declare_node_union};
use biome_rule_options::no_misleading_return_type::NoMisleadingReturnTypeOptions;

use crate::services::typed::Typed;

declare_lint_rule! {
    /// Detect return type annotations that are misleadingly wider than what
    /// the implementation actually returns.
    ///
    /// Reports when a function's explicit return type annotation is wider than
    /// what TypeScript would infer from the implementation, hiding precise types
    /// from callers.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic,file=invalid.ts
    /// function getStatus(b: boolean): string { if (b) return "loading"; return "idle"; }
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=invalid2.ts
    /// function getCode(ok: boolean): number { if (ok) return 200; return 404; }
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=invalid3.ts
    /// class Foo { getStatus(b: boolean): string { if (b) return "loading"; return "idle"; } }
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=invalid4.ts
    /// const obj = { getMode(b: boolean): string { if (b) return "dark"; return "light"; } };
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=invalid5.ts
    /// function makeData(): object { return { retry: true }; }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// function getStatus() { return "loading"; }
    /// ```
    ///
    /// ```ts
    /// function run(): void { return; }
    /// ```
    ///
    /// ```ts
    /// class Foo { greet(): string { return "hello"; } }
    /// ```
    pub NoMisleadingReturnType {
        version: "2.4.11",
        name: "noMisleadingReturnType",
        language: "ts",
        recommended: false,
        domains: &[RuleDomain::Types],
        issue_number: Some("9810"),
    }
}

declare_node_union! {
    pub AnyFunctionLikeWithReturnType =
        AnyJsFunction
        | JsMethodClassMember
        | JsMethodObjectMember
        | JsGetterClassMember
        | JsGetterObjectMember
}

pub struct RuleState {
    annotation_range: TextRange,
    returns: Vec<Type>,
}

/// Maximum iterations for type graph traversal to guard against infinite loops on cyclic types.
const MAX_TYPE_TRAVERSAL_ITERATIONS: usize = 50;

impl Rule for NoMisleadingReturnType {
    type Query = Typed<AnyFunctionLikeWithReturnType>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = NoMisleadingReturnTypeOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        match node {
            AnyFunctionLikeWithReturnType::AnyJsFunction(func) => {
                run_for_function(ctx, func)
            }
            AnyFunctionLikeWithReturnType::JsMethodClassMember(method) => {
                if method.star_token().is_some() {
                    return None;
                }
                if is_class_method_overload_implementation(method) {
                    return None;
                }
                let annotation = method.return_type_annotation()?;
                let name = method.name().ok()?.as_js_literal_member_name()?.name().ok()?;
                let func_type = ctx.type_of_member(method.syntax(), name.text());
                run_for_member(ctx, annotation.range(), &func_type, method.async_token().is_some(), &method.body().ok()?)
            }
            AnyFunctionLikeWithReturnType::JsMethodObjectMember(method) => {
                if method.star_token().is_some() {
                    return None;
                }
                let annotation = method.return_type_annotation()?;
                let name = method.name().ok()?.as_js_literal_member_name()?.name().ok()?;
                let func_type = ctx.type_of_member(method.syntax(), name.text());
                run_for_member(ctx, annotation.range(), &func_type, method.async_token().is_some(), &method.body().ok()?)
            }
            AnyFunctionLikeWithReturnType::JsGetterClassMember(getter) => {
                let annotation = getter.return_type()?;
                let any_getter = AnyJsGetter::from(getter.clone());
                let name = any_getter.member_name()?;
                if any_getter.has_matching_setter(&name) {
                    return None;
                }
                let func_type = ctx.type_of_member(getter.syntax(), name.text());
                run_for_member(ctx, annotation.range(), &func_type, false, &getter.body().ok()?)
            }
            AnyFunctionLikeWithReturnType::JsGetterObjectMember(getter) => {
                let annotation = getter.return_type()?;
                let any_getter = AnyJsGetter::from(getter.clone());
                let name = any_getter.member_name()?;
                if any_getter.has_matching_setter(&name) {
                    return None;
                }
                let func_type = ctx.type_of_member(getter.syntax(), name.text());
                run_for_member(ctx, annotation.range(), &func_type, false, &getter.body().ok()?)
            }
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let diag = RuleDiagnostic::new(
            rule_category!(),
            state.annotation_range,
            markup! {
                "The return type annotation is wider than what the function actually returns."
            },
        )
        .note(markup! {
            "A wider return type hides the precise types that callers could rely on."
        });

        let diag = match build_inferred_description(&state.returns) {
            Some(desc) => diag.note(markup! {
                "Consider using "{desc}" as the return type."
            }),
            None => diag.note(markup! {
                "Narrow the return type to match what the function actually returns."
            }),
        };

        Some(diag)
    }

}

/// Looks for sibling function declarations with the same name but no body,
/// which indicates this function is the implementation of an overload set.
/// Overload signatures are parsed as `TsDeclareFunctionDeclaration` or as
/// `AnyJsFunction` with `body().is_err()`.
fn is_overload_implementation(node: &AnyJsFunction) -> bool {
    let name = node
        .binding()
        .and_then(|b| b.as_js_identifier_binding().cloned())
        .and_then(|id| id.name_token().ok())
        .map(|t| t.token_text_trimmed());
    let Some(name) = name else { return false };

    let Some(parent) = node.syntax().parent() else {
        return false;
    };
    parent.children().any(|sibling| {
        if sibling == *node.syntax() {
            return false;
        }
        if let Some(decl) =
            biome_js_syntax::TsDeclareFunctionDeclaration::cast(sibling.clone())
        {
            return decl
                .id()
                .ok()
                .and_then(|id| id.as_js_identifier_binding().cloned())
                .and_then(|id| id.name_token().ok())
                .is_some_and(|t| t.token_text_trimmed() == name);
        }
        AnyJsFunction::cast(sibling).is_some_and(|sib_fn| {
            sib_fn.body().is_err()
                && sib_fn
                    .binding()
                    .and_then(|b| b.as_js_identifier_binding().cloned())
                    .and_then(|id| id.name_token().ok())
                    .is_some_and(|t| t.token_text_trimmed() == name)
        })
    })
}

fn run_for_function(
    ctx: &RuleContext<NoMisleadingReturnType>,
    node: &AnyJsFunction,
) -> Option<RuleState> {
    let annotation = node.return_type_annotation()?;
    let annotation_range = annotation.range();

    if node.is_generator() || is_overload_implementation(node) {
        return None;
    }

    let func_type = ctx.type_of_function(node);
    let is_async = node.async_token().is_some();
    let body = node.body().ok()?;

    run_for_member_with_body(ctx, annotation_range, &func_type, is_async, &body)
}

fn run_for_member(
    ctx: &RuleContext<NoMisleadingReturnType>,
    annotation_range: TextRange,
    func_type: &Type,
    is_async: bool,
    body: &JsFunctionBody,
) -> Option<RuleState> {
    run_for_member_with_body(
        ctx,
        annotation_range,
        func_type,
        is_async,
        &AnyJsFunctionBody::JsFunctionBody(body.clone()),
    )
}

fn run_for_member_with_body(
    ctx: &RuleContext<NoMisleadingReturnType>,
    annotation_range: TextRange,
    func_type: &Type,
    is_async: bool,
    body: &AnyJsFunctionBody,
) -> Option<RuleState> {
    let return_ty = extract_return_type(func_type)?;

    if is_escape_hatch(&return_ty) {
        return None;
    }

    let effective_return_ty = if is_async {
        unwrap_promise_inner(&return_ty)
    } else {
        return_ty.clone()
    };

    let info = collect_return_info(ctx, body);

    if info.types.is_empty() {
        return None;
    }

    if info.is_single_primitive_literal() {
        return None;
    }

    if info.all_opt_into_object() && matches!(&*effective_return_ty, TypeData::ObjectKeyword) {
        return None;
    }

    if matches!(&*effective_return_ty, TypeData::Boolean)
        && info.matches_boolean_value(true)
        && info.matches_boolean_value(false)
    {
        return None;
    }

    if info.types.iter().any(is_any_contaminated) {
        return None;
    }

    if includes_undefined(&effective_return_ty)
        && !info.types.iter().any(includes_undefined)
    {
        return None;
    }

    if info.types.iter().any(is_intersection_with_type_param) {
        return None;
    }

    if !info.has_any_const
        && is_only_property_literal_widening(&effective_return_ty, &info.types)
    {
        return None;
    }

    let is_misleading = if effective_return_ty.is_union() {
        is_union_wider_than_returns(&effective_return_ty, &info.types)
    } else if matches!(&*effective_return_ty, TypeData::ObjectKeyword) {
        // For `: object`, any narrower branch is enough; bare `{}` doesn't count.
        info.has_narrower_than_object
            || info
                .types
                .iter()
                .any(|inferred| is_wider_than(&effective_return_ty, inferred))
    } else {
        info.types
            .iter()
            .all(|inferred| is_wider_than(&effective_return_ty, inferred))
    };

    if !is_misleading {
        return None;
    }

    Some(RuleState {
        annotation_range,
        returns: info.types,
    })
}

fn is_class_method_overload_implementation(method: &JsMethodClassMember) -> bool {
    let name = method
        .name()
        .ok()
        .and_then(|member_name| member_name.as_js_literal_member_name().cloned())
        .and_then(|literal_name| literal_name.name().ok());
    let Some(name) = name else { return false };

    let Some(member_list) = method.syntax().parent() else {
        return false;
    };

    member_list
        .children()
        .filter_map(TsMethodSignatureClassMember::cast)
        .any(|signature| {
            signature
                .name()
                .ok()
                .and_then(|member_name| member_name.as_js_literal_member_name().cloned())
                .and_then(|literal_name| literal_name.name().ok())
                .is_some_and(|sibling_name| sibling_name == name)
        })
}

fn extract_return_type(func_type: &Type) -> Option<Type> {
    match &**func_type {
        TypeData::Function(function) => {
            let ty_ref = function.return_type.as_type()?;
            func_type.resolve(ty_ref)
        }
        _ => None,
    }
}

fn is_escape_hatch(ty: &Type) -> bool {
    matches!(
        &**ty,
        TypeData::AnyKeyword
            | TypeData::VoidKeyword
            | TypeData::UnknownKeyword
            | TypeData::NeverKeyword
            | TypeData::Unknown
            | TypeData::ThisKeyword
    )
}

/// For async functions the annotation is `Promise<T>`. We need `T` to compare
/// against the return expressions, which are not wrapped in `Promise`.
fn unwrap_promise_inner(return_ty: &Type) -> Type {
    if let TypeData::InstanceOf(instance) = &**return_ty
        && let Some(inner_ref) = instance.type_parameters.first()
            && let Some(inner) = return_ty.resolve(inner_ref)
                && !is_escape_hatch(&inner) {
                    return inner;
                }

    return_ty.clone()
}

fn includes_undefined(ty: &Type) -> bool {
    match &**ty {
        TypeData::Undefined | TypeData::VoidKeyword => true,
        TypeData::Union(_) => ty
            .flattened_union_variants()
            .any(|v| matches!(&*v, TypeData::Undefined | TypeData::VoidKeyword)),
        _ => false,
    }
}

fn is_any_contaminated(ty: &Type) -> bool {
    match &**ty {
        TypeData::AnyKeyword => true,
        TypeData::Union(_) => ty
            .flattened_union_variants()
            .any(|v| matches!(&*v, TypeData::AnyKeyword)),
        _ => false,
    }
}

fn is_intersection_with_type_param(ty: &Type) -> bool {
    match &**ty {
        TypeData::Intersection(intersection) => intersection.types().iter().any(|member_ref| {
            ty.resolve(member_ref)
                .is_some_and(|resolved| matches!(&*resolved, TypeData::Generic(_)))
        }),
        _ => false,
    }
}

fn is_literal_of_primitive(ty: &Type) -> bool {
    match &**ty {
        TypeData::Literal(lit) => lit.is_primitive(),
        // The type resolver may wrap a single literal in a Union for mutable
        // bindings.  Treat a one-element union of a primitive literal the same.
        TypeData::Union(_) => {
            let mut iter = ty.flattened_union_variants();
            matches!(
                (iter.next(), iter.next()),
                (Some(v), None) if matches!(&*v, TypeData::Literal(lit) if lit.is_primitive())
            )
        }
        _ => false,
    }
}

/// Checks whether annotation differs from returns only by property-level
/// literal widening that contextual typing would handle.
fn is_only_property_literal_widening(annotation: &Type, returns: &[Type]) -> bool {
    returns.iter().all(|inferred| {
        let mut stack: Vec<(Type, Type)> = vec![(annotation.clone(), inferred.clone())];
        let mut has_widening = false;
        let mut iterations = 0usize;

        while let Some((annotated, inferred)) = stack.pop() {
            iterations += 1;
            if iterations > MAX_TYPE_TRAVERSAL_ITERATIONS {
                return false;
            }

            if let TypeData::Tuple(annotated_tuple) = &*annotated {
                let TypeData::Tuple(inferred_tuple) = &*inferred else {
                    return false;
                };
                let annotated_elements = annotated_tuple.elements();
                let inferred_elements = inferred_tuple.elements();
                if annotated_elements.len() != inferred_elements.len()
                    || annotated_elements.is_empty()
                {
                    return false;
                }
                for (annotated_element, inferred_element) in
                    annotated_elements.iter().zip(inferred_elements.iter())
                {
                    match (
                        annotated.resolve(&annotated_element.ty),
                        inferred.resolve(&inferred_element.ty),
                    ) {
                        (Some(annotated_type), Some(inferred_type)) => {
                            if types_match(&annotated_type, &inferred_type) {
                                continue;
                            }
                            if is_base_type_of_literal(&annotated_type, &inferred_type) {
                                has_widening = true;
                            } else {
                                stack.push((annotated_type, inferred_type));
                            }
                        }
                        _ => return false,
                    }
                }
                continue;
            }

            let TypeData::Object(annotated_object) = &*annotated else {
                return false;
            };
            if annotated_object.members.is_empty() {
                return false;
            }

            let inferred_members = match &*inferred {
                TypeData::Object(object) => &object.members,
                TypeData::Literal(literal) => match literal.as_ref() {
                    Literal::Object(object_literal) => object_literal.members(),
                    _ => return false,
                },
                _ => return false,
            };
            if inferred_members.is_empty() {
                return false;
            }

            let annotated_index_signature = annotated_object.members.iter().find(|member| {
                matches!(
                    member.kind,
                    biome_js_type_info::TypeMemberKind::IndexSignature(_)
                )
            });
            if let Some(index_signature_member) = annotated_index_signature
                && let Some(index_signature_value_type) =
                    annotated.resolve(&index_signature_member.ty)
            {
                let mut index_signature_has_widening = false;
                let all_inferred_covered = inferred_members.iter().all(|inferred_member| {
                    if let Some(inferred_type) = inferred.resolve(&inferred_member.ty) {
                        if types_match(&index_signature_value_type, &inferred_type) {
                            return true;
                        }
                        if is_base_type_of_literal(&index_signature_value_type, &inferred_type) {
                            index_signature_has_widening = true;
                            return true;
                        }
                    }
                    false
                });
                if !(all_inferred_covered && index_signature_has_widening) {
                    return false;
                }
                has_widening = true;
                continue;
            }

            for annotated_member in annotated_object.members.iter() {
                let annotated_name = match &annotated_member.kind {
                    biome_js_type_info::TypeMemberKind::Named(name)
                    | biome_js_type_info::TypeMemberKind::NamedOptional(name) => name,
                    _ => continue,
                };
                let Some(inferred_member) = inferred_members
                    .iter()
                    .find(|member| member.kind.has_name(annotated_name))
                else {
                    return false;
                };
                match (
                    annotated.resolve(&annotated_member.ty),
                    inferred.resolve(&inferred_member.ty),
                ) {
                    (Some(annotated_type), Some(inferred_type)) => {
                        if types_match(&annotated_type, &inferred_type) {
                            continue;
                        }
                        if is_base_type_of_literal(&annotated_type, &inferred_type) {
                            has_widening = true;
                        } else {
                            stack.push((annotated_type, inferred_type));
                        }
                    }
                    _ => return false,
                }
            }
        }

        has_widening
    })
}

fn is_base_type_of_literal(base: &Type, literal: &Type) -> bool {
    match (&**base, &**literal) {
        (TypeData::String, TypeData::Literal(lit)) => {
            matches!(lit.as_ref(), Literal::String(_) | Literal::Template(_))
        }
        (TypeData::Number, TypeData::Literal(lit)) => matches!(lit.as_ref(), Literal::Number(_)),
        (TypeData::Boolean, TypeData::Literal(lit)) => {
            matches!(lit.as_ref(), Literal::Boolean(_))
        }
        (TypeData::BigInt, TypeData::Literal(lit)) => matches!(lit.as_ref(), Literal::BigInt(_)),
        _ => false,
    }
}

/// Builds a string like `"loading" | "idle"` for the diagnostic note.
fn build_inferred_description(returns: &[Type]) -> Option<String> {
    let mut result = String::new();
    for ty in returns {
        match &**ty {
            TypeData::Literal(lit) => {
                if !result.is_empty() {
                    result.push_str(" | ");
                }
                match lit.as_ref() {
                    Literal::String(s) => {
                        result.push('"');
                        result.push_str(s.as_str());
                        result.push('"');
                    }
                    Literal::Number(n) => result.push_str(n.as_str()),
                    Literal::Boolean(b) => {
                        result.push_str(if b.as_bool() { "true" } else { "false" })
                    }
                    _ => return None,
                }
            }
            _ => return None,
        }
    }

    if result.is_empty() {
        return None;
    }

    // Skip values that would look confusing in a diagnostic (e.g. "...").
    if result.contains("...") || result.contains("__internal") || result.contains("typeof import(") {
        return None;
    }

    // Skip overly long descriptions.
    if result.len() > 80 {
        return None;
    }

    Some(result)
}

/// Per-body accumulator for the misleading-return check.
#[derive(Default)]
struct ReturnInfo {
    types: Vec<Type>,
    has_any_const: bool,
    /// Count of return expressions that explicitly cast or assert to the
    /// `object` keyword (or an equivalent target such as an alias to `object`).
    object_keyword_casts: usize,
    /// Whether any return expression reveals structure narrower than the
    /// TypeScript `object` keyword, such as members, tuples, functions, or
    /// class instances.
    has_narrower_than_object: bool,
}

impl ReturnInfo {
    /// Single-return whose inferred type is a primitive literal and no `as const`.
    fn is_single_primitive_literal(&self) -> bool {
        self.types.len() == 1 && !self.has_any_const && is_literal_of_primitive(&self.types[0])
    }

    /// Every return carries an explicit `object`-keyword cast (and no `as const`).
    fn all_opt_into_object(&self) -> bool {
        !self.has_any_const && self.object_keyword_casts == self.types.len()
    }

    /// Whether any return type matches the boolean literal `value`.
    fn matches_boolean_value(&self, value: bool) -> bool {
        self.types.iter().any(|ty| ty.is_boolean_literal(value))
    }
}

/// Walks the function body and populates a [`ReturnInfo`].
fn collect_return_info(
    ctx: &RuleContext<NoMisleadingReturnType>,
    body: &AnyJsFunctionBody,
) -> ReturnInfo {
    let mut info = ReturnInfo::default();

    match body {
        AnyJsFunctionBody::JsFunctionBody(block) => collect_block_returns(ctx, block, &mut info),
        AnyJsFunctionBody::AnyJsExpression(expr) => {
            if has_const_assertion(expr) {
                info.has_any_const = true;
            } else if has_object_keyword_assertion(expr) {
                info.object_keyword_casts += 1;
            } else if has_narrow_cast(expr) || object_literal_has_any_member(expr) {
                info.has_narrower_than_object = true;
            }
            info.types.push(infer_expression_type(ctx, expr));
        }
    }

    info
}

fn collect_block_returns(
    ctx: &RuleContext<NoMisleadingReturnType>,
    block: &JsFunctionBody,
    info: &mut ReturnInfo,
) {
    for node in block
        .syntax()
        .pruned_descendents(|n| !is_nested_function_like(n))
    {
        let Some(ret) = JsReturnStatement::cast(node) else {
            continue;
        };
        if let Some(arg) = ret.argument()
            && let Some(expr) = AnyJsExpression::cast(arg.syntax().clone())
        {
            if has_const_assertion(&expr) {
                info.has_any_const = true;
            } else if has_object_keyword_assertion(&expr) {
                info.object_keyword_casts += 1;
            } else if has_narrow_cast(&expr) || object_literal_has_any_member(&expr) {
                info.has_narrower_than_object = true;
            }
            info.types.push(infer_expression_type(ctx, &expr));
        }
    }
}

/// Whether the return is an object literal with any member. Bare `{}` is
/// equivalent to `object`, so it returns `false`; spreads count because
/// type-info drops them today.
fn object_literal_has_any_member(expression: &AnyJsExpression) -> bool {
    let mut current = expression.clone();
    loop {
        match current {
            AnyJsExpression::JsParenthesizedExpression(paren) => match paren.expression() {
                Ok(inner) => current = inner,
                Err(_) => return false,
            },
            AnyJsExpression::JsObjectExpression(object) => {
                return object.members().into_iter().next().is_some();
            }
            _ => return false,
        }
    }
}

/// Whether the expression has a cast whose target is strictly narrower
/// than `object`. Walks parens and ternary branches, mirroring
/// [`has_object_keyword_assertion`].
fn has_narrow_cast(expression: &AnyJsExpression) -> bool {
    let mut stack = vec![expression.clone()];
    while let Some(current) = stack.pop() {
        if let Some(cast) = AnyTsCastExpression::cast(current.syntax().clone()) {
            let Some(cast_type) = cast.cast_type() else {
                continue;
            };
            if !cast_target_trustworthy(&cast_type, &cast) {
                return true;
            }
            continue;
        }
        match current {
            AnyJsExpression::JsParenthesizedExpression(paren) => {
                if let Ok(inner) = paren.expression() {
                    stack.push(inner);
                }
            }
            AnyJsExpression::JsConditionalExpression(conditional) => {
                if let Ok(consequent) = conditional.consequent() {
                    stack.push(consequent);
                }
                if let Ok(alternate) = conditional.alternate() {
                    stack.push(alternate);
                }
            }
            _ => {}
        }
    }
    false
}

/// Whether the expression is a type assertion that opts into `object` widening.
/// Ternary branches must each carry their own trustworthy cast.
fn has_object_keyword_assertion(expression: &AnyJsExpression) -> bool {
    let mut stack = vec![expression.clone()];
    while let Some(current_expression) = stack.pop() {
        if let Some(cast) = AnyTsCastExpression::cast(current_expression.syntax().clone()) {
            let Some(cast_type) = cast.cast_type() else {
                return false;
            };
            if !cast_target_trustworthy(&cast_type, &cast) {
                return false;
            }
            continue;
        }
        match current_expression {
            AnyJsExpression::JsParenthesizedExpression(parenthesized_expression) => {
                let Ok(inner_expression) = parenthesized_expression.expression() else {
                    return false;
                };
                stack.push(inner_expression);
            }
            AnyJsExpression::JsConditionalExpression(conditional_expression) => {
                let Ok(consequent) = conditional_expression.consequent() else {
                    return false;
                };
                let Ok(alternate) = conditional_expression.alternate() else {
                    return false;
                };
                stack.push(consequent);
                stack.push(alternate);
            }
            _ => return false,
        }
    }
    true
}

/// Whether the cast target reduces to the `object` keyword. Atomic targets
/// decide inline; compound shapes delegate to [`walk_compound_cast_target`].
fn cast_target_trustworthy(cast_target: &AnyTsType, anchor: &AnyTsCastExpression) -> bool {
    match cast_target.clone().omit_parentheses() {
        AnyTsType::TsNonPrimitiveType(_)
        | AnyTsType::TsUnknownType(_)
        | AnyTsType::TsAnyType(_) => true,
        AnyTsType::TsObjectType(object_type) => {
            object_type.members().into_iter().next().is_none()
        }
        unwrapped @ (AnyTsType::TsReferenceType(_)
        | AnyTsType::TsIntersectionType(_)
        | AnyTsType::TsUnionType(_)
        | AnyTsType::TsConditionalType(_)) => walk_compound_cast_target(unwrapped, anchor),
        _ => false,
    }
}

/// Iterative walker for compound cast targets. Intersections require every
/// member trustworthy, unions require one. Bounded by
/// [`MAX_TYPE_TRAVERSAL_ITERATIONS`]; on timeout we trust — tsc rejects
/// structurally incompatible casts anyway.
fn walk_compound_cast_target(root: AnyTsType, anchor: &AnyTsCastExpression) -> bool {
    enum Task {
        Visit(AnyTsType),
        /// AND the top `N` results (intersection).
        AllOf(usize),
        /// OR the top `N` results (union).
        AnyOf(usize),
    }

    let mut tasks: Vec<Task> = vec![Task::Visit(root)];
    let mut results: Vec<bool> = Vec::new();
    let mut iterations: usize = 0;

    while let Some(task) = tasks.pop() {
        iterations += 1;
        if iterations > MAX_TYPE_TRAVERSAL_ITERATIONS {
            return true;
        }
        match task {
            Task::Visit(ty) => match ty.omit_parentheses() {
                AnyTsType::TsNonPrimitiveType(_)
                | AnyTsType::TsUnknownType(_)
                | AnyTsType::TsAnyType(_) => results.push(true),
                AnyTsType::TsObjectType(object_type) => {
                    results.push(object_type.members().into_iter().next().is_none());
                }
                AnyTsType::TsReferenceType(reference_type) => {
                    let Some(name) = reference_type_name(&reference_type) else {
                        results.push(true);
                        continue;
                    };
                    match find_named_type_declaration(&name, anchor) {
                        Some(FoundDeclaration::TypeAlias(body)) => {
                            tasks.push(Task::Visit(body));
                        }
                        Some(FoundDeclaration::ObjectEquivalentNominal) => results.push(true),
                        Some(FoundDeclaration::NarrowNominal) => results.push(false),
                        None => results.push(true),
                    }
                }
                AnyTsType::TsIntersectionType(intersection) => {
                    let members: Vec<_> = intersection
                        .types()
                        .into_iter()
                        .filter_map(|member_result| member_result.ok())
                        .collect();
                    if members.is_empty() {
                        results.push(false);
                    } else {
                        tasks.push(Task::AllOf(members.len()));
                        for member in members {
                            tasks.push(Task::Visit(member));
                        }
                    }
                }
                AnyTsType::TsUnionType(union_type) => {
                    let members: Vec<_> = union_type
                        .types()
                        .into_iter()
                        .filter_map(|member_result| member_result.ok())
                        .collect();
                    if members.is_empty() {
                        results.push(false);
                    } else {
                        tasks.push(Task::AnyOf(members.len()));
                        for member in members {
                            tasks.push(Task::Visit(member));
                        }
                    }
                }
                AnyTsType::TsConditionalType(conditional) => {
                    // Branch is unknowable statically — both must trust.
                    match (conditional.true_type(), conditional.false_type()) {
                        (Ok(true_branch), Ok(false_branch)) => {
                            tasks.push(Task::AllOf(2));
                            tasks.push(Task::Visit(false_branch));
                            tasks.push(Task::Visit(true_branch));
                        }
                        _ => results.push(false),
                    }
                }
                _ => results.push(false),
            },
            Task::AllOf(count) => {
                let split = results.len().saturating_sub(count);
                let all = results.drain(split..).all(|result| result);
                results.push(all);
            }
            Task::AnyOf(count) => {
                let split = results.len().saturating_sub(count);
                let any = results.drain(split..).any(|result| result);
                results.push(any);
            }
        }
    }
    results.pop().unwrap_or(true)
}

/// Shape of a named declaration.
enum FoundDeclaration {
    TypeAlias(AnyTsType),
    /// Empty class or interface with no base types — equivalent to `object`.
    ObjectEquivalentNominal,
    /// Class or interface that narrows `object` (has members or base types).
    NarrowNominal,
}

declare_node_union! {
    AnyNamedTypeDecl =
        TsTypeAliasDeclaration
        | JsClassDeclaration
        | TsInterfaceDeclaration
}

impl AnyNamedTypeDecl {
    /// Whether this declaration's binding name matches `name`.
    fn matches_name(&self, name: &TokenText) -> bool {
        let token = match self {
            Self::TsTypeAliasDeclaration(alias) => {
                let Ok(binding) = alias.binding_identifier() else {
                    return false;
                };
                let AnyTsIdentifierBinding::TsIdentifierBinding(binding) = binding else {
                    return false;
                };
                binding.name_token().ok()
            }
            Self::TsInterfaceDeclaration(interface) => {
                let Ok(binding) = interface.id() else {
                    return false;
                };
                let AnyTsIdentifierBinding::TsIdentifierBinding(binding) = binding else {
                    return false;
                };
                binding.name_token().ok()
            }
            Self::JsClassDeclaration(class) => {
                let Ok(binding) = class.id() else {
                    return false;
                };
                let AnyJsBinding::JsIdentifierBinding(binding) = binding else {
                    return false;
                };
                binding.name_token().ok()
            }
        };
        token.is_some_and(|token| token.token_text_trimmed() == *name)
    }
}

/// Finds the matching type alias, class, or interface declaration reachable
/// by walking `anchor`'s ancestors. Same-file only.
fn find_named_type_declaration(
    name: &TokenText,
    anchor: &AnyTsCastExpression,
) -> Option<FoundDeclaration> {
    for ancestor in anchor.syntax().ancestors() {
        let found = ancestor
            .children()
            .filter_map(AnyNamedTypeDecl::cast)
            .find(|declaration| declaration.matches_name(name));
        if let Some(declaration) = found {
            return match declaration {
                AnyNamedTypeDecl::TsTypeAliasDeclaration(alias) => {
                    alias.ty().ok().map(FoundDeclaration::TypeAlias)
                }
                AnyNamedTypeDecl::JsClassDeclaration(class) => {
                    Some(if is_empty_class(&class) {
                        FoundDeclaration::ObjectEquivalentNominal
                    } else {
                        FoundDeclaration::NarrowNominal
                    })
                }
                AnyNamedTypeDecl::TsInterfaceDeclaration(interface) => {
                    Some(if is_empty_interface(&interface) {
                        FoundDeclaration::ObjectEquivalentNominal
                    } else {
                        FoundDeclaration::NarrowNominal
                    })
                }
            };
        }
    }
    None
}

/// Whether the class has no members, no `extends`, and no `implements`.
fn is_empty_class(class: &JsClassDeclaration) -> bool {
    class.members().into_iter().next().is_none()
        && class.extends_clause().is_none()
        && class.implements_clause().is_none()
}

/// Whether the interface has no members and no `extends` clause.
fn is_empty_interface(interface: &TsInterfaceDeclaration) -> bool {
    interface.members().into_iter().next().is_none() && interface.extends_clause().is_none()
}

/// Extracts the textual name of a reference type expression.
fn reference_type_name(reference_type: &TsReferenceType) -> Option<TokenText> {
    reference_type
        .name()
        .ok()?
        .as_js_reference_identifier()?
        .value_token()
        .ok()
        .map(|token| token.token_text_trimmed())
}

/// Gets the type of a return expression. For identifiers bound to an
/// `as const` initializer, walks the AST to find the original literal type
/// since `type_of_expression` would return the widened type.
fn infer_expression_type(
    ctx: &RuleContext<NoMisleadingReturnType>,
    expr: &AnyJsExpression,
) -> Type {
    let inner = unwrap_type_wrappers(expr);

    if let AnyJsExpression::JsIdentifierExpression(ref id_expr) = inner
        && let Some(init_type) = resolve_identifier_initializer_type(ctx, id_expr) {
            return init_type;
        }

    ctx.type_of_expression(&inner)
}

fn resolve_identifier_initializer_type(
    ctx: &RuleContext<NoMisleadingReturnType>,
    id_expr: &biome_js_syntax::JsIdentifierExpression,
) -> Option<Type> {
    let name = id_expr
        .name()
        .ok()
        .and_then(|n| n.value_token().ok())
        .map(|t| t.token_text_trimmed())?;

    let body = id_expr
        .syntax()
        .ancestors()
        .find_map(JsFunctionBody::cast)?;

    for stmt in body.statements() {
        let var_stmt = JsVariableStatement::cast(stmt.into_syntax());
        let Some(var_stmt) = var_stmt else { continue };
        let Ok(decl) = var_stmt.declaration() else {
            continue;
        };
        for declarator in decl.declarators() {
            let Ok(d) = declarator else { continue };
            let id_text = d
                .id()
                .ok()
                .and_then(|id| id.as_any_js_binding().cloned())
                .and_then(|b| b.as_js_identifier_binding().cloned())
                .and_then(|ib| ib.name_token().ok())
                .map(|t| t.token_text_trimmed());
            let Some(id_text) = id_text else { continue };
            if id_text != name {
                continue;
            }
            let init_expr = d
                .initializer()
                .and_then(|init| init.expression().ok())?;
            if !has_const_assertion(&init_expr) {
                continue;
            }
            let unwrapped = unwrap_type_wrappers(&init_expr);
            return Some(ctx.type_of_expression(&unwrapped));
        }
    }

    None
}

fn unwrap_type_wrappers(expr: &AnyJsExpression) -> AnyJsExpression {
    let mut current = expr.clone();
    loop {
        if let Some(cast) = AnyTsCastExpression::cast(current.syntax().clone()) {
            let Some(inner) = cast.inner_expression() else {
                return current;
            };
            current = inner;
            continue;
        }
        match &current {
            AnyJsExpression::JsParenthesizedExpression(e) => match e.expression() {
                Ok(inner) => current = inner,
                Err(_) => return current,
            },
            _ => return current,
        }
    }
}

fn has_const_assertion(expr: &AnyJsExpression) -> bool {
    let mut current = expr.clone();
    loop {
        match &current {
            AnyJsExpression::TsAsExpression(e) => return is_const_type_assertion(e),
            AnyJsExpression::TsTypeAssertionExpression(e) => {
                return is_const_angle_bracket_assertion(e)
            }
            AnyJsExpression::JsParenthesizedExpression(e) => match e.expression() {
                Ok(inner) => current = inner,
                Err(_) => return false,
            },
            AnyJsExpression::TsSatisfiesExpression(e) => match e.expression() {
                Ok(inner) => current = inner,
                Err(_) => return false,
            },
            AnyJsExpression::JsIdentifierExpression(id_expr) => {
                return identifier_refers_to_const_assertion(id_expr)
            }
            _ => return false,
        }
    }
}

fn identifier_refers_to_const_assertion(
    id_expr: &biome_js_syntax::JsIdentifierExpression,
) -> bool {
    let name = id_expr
        .name()
        .ok()
        .and_then(|n| n.value_token().ok())
        .map(|t| t.token_text_trimmed());
    let Some(name) = name else { return false };

    let Some(body) = id_expr
        .syntax()
        .ancestors()
        .find_map(JsFunctionBody::cast)
    else {
        return false;
    };

    body.statements().into_iter().any(|stmt| {
        let var_stmt = JsVariableStatement::cast(stmt.into_syntax());
        let Some(var_stmt) = var_stmt else { return false };
        let Ok(decl) = var_stmt.declaration() else {
            return false;
        };
        decl.declarators().into_iter().any(|declarator| {
            declarator
                .ok()
                .is_some_and(|d| declarator_matches_name_with_const(&d, &name))
        })
    })
}

fn declarator_matches_name_with_const(declarator: &JsVariableDeclarator, name: &TokenText) -> bool {
    let id_text = declarator
        .id()
        .ok()
        .and_then(|id| id.as_any_js_binding().cloned())
        .and_then(|b| b.as_js_identifier_binding().cloned())
        .and_then(|ib| ib.name_token().ok())
        .map(|t| t.token_text_trimmed());
    let Some(id_text) = id_text else { return false };

    if id_text != *name {
        return false;
    }

    // We already resolved the identifier to reach this declarator,
    // so there's no need to follow identifiers again.
    declarator
        .initializer()
        .and_then(|init| init.expression().ok())
        .is_some_and(|init_expr| init_has_direct_const_assertion(&init_expr))
}

/// Checks for `as const` on the expression itself, without following identifiers.
fn init_has_direct_const_assertion(expr: &AnyJsExpression) -> bool {
    let mut current = expr.clone();
    loop {
        match &current {
            AnyJsExpression::TsAsExpression(e) => return is_const_type_assertion(e),
            AnyJsExpression::TsTypeAssertionExpression(e) => {
                return is_const_angle_bracket_assertion(e)
            }
            AnyJsExpression::JsParenthesizedExpression(e) => match e.expression() {
                Ok(inner) => current = inner,
                Err(_) => return false,
            },
            AnyJsExpression::TsSatisfiesExpression(e) => match e.expression() {
                Ok(inner) => current = inner,
                Err(_) => return false,
            },
            _ => return false,
        }
    }
}

fn is_const_type_assertion(expr: &TsAsExpression) -> bool {
    is_const_reference_type(&expr.ty().ok())
}

fn is_const_angle_bracket_assertion(expr: &TsTypeAssertionExpression) -> bool {
    is_const_reference_type(&expr.ty().ok())
}

fn is_const_reference_type(ty: &Option<biome_js_syntax::AnyTsType>) -> bool {
    ty.as_ref()
        .and_then(|ty| ty.as_ts_reference_type())
        .and_then(|ref_ty| ref_ty.name().ok())
        .is_some_and(|name| {
            name.as_js_reference_identifier()
                .and_then(|id| id.value_token().ok())
                .is_some_and(|token| token.text_trimmed() == "const")
        })
}

declare_node_union! {
    AnyNestedFunctionLike =
        JsFunctionExpression
        | JsArrowFunctionExpression
        | JsFunctionDeclaration
        | JsConstructorClassMember
        | JsMethodClassMember
        | JsMethodObjectMember
        | JsGetterClassMember
        | JsGetterObjectMember
        | JsSetterClassMember
        | JsSetterObjectMember
}

fn is_nested_function_like(node: &JsSyntaxNode) -> bool {
    AnyNestedFunctionLike::can_cast(node.kind())
}

/// Follows generic constraints iteratively: `T extends U extends string` → `string`.
fn resolve_generic_chain(ty: &Type) -> Type {
    let mut current = ty.clone();
    let mut steps = 0u8;
    while let TypeData::Generic(generic) = &*current {
        if steps > 5 || !generic.constraint.is_known() {
            break;
        }
        match current.resolve(&generic.constraint) {
            Some(resolved) => {
                current = resolved;
                steps += 1;
            }
            None => break,
        }
    }
    current
}

/// Whether the inferred type reveals structure hidden by `: object`. Empty
/// object shapes don't count — they're equivalent to `object`.
fn is_strictly_narrower_than_object_keyword(inferred: &TypeData) -> bool {
    match inferred {
        TypeData::Object(obj) => !obj.members.is_empty(),
        TypeData::InstanceOf(_) | TypeData::Tuple(_) | TypeData::Function(_) => true,
        TypeData::Literal(lit) => match lit.as_ref() {
            Literal::RegExp(_) => true,
            Literal::Object(obj) => !obj.members().is_empty(),
            _ => false,
        },
        _ => false,
    }
}

/// Compares non-union type pairs using a work stack. Compound types
/// (Instance params, Object properties) are decomposed into sub-pairs
/// and pushed back onto the stack for further comparison.
fn is_nonunion_wider(annotated: &Type, inferred: &Type) -> bool {
    let mut stack: Vec<(Type, Type)> =
        vec![(annotated.clone(), resolve_generic_chain(inferred))];
    let mut found_wider = false;
    let mut iterations = 0usize;

    while let Some((ann, inf)) = stack.pop() {
        iterations += 1;
        if iterations > MAX_TYPE_TRAVERSAL_ITERATIONS {
            return false;
        }

        if is_base_type_of_literal(&ann, &inf) {
            found_wider = true;
            continue;
        }

        if types_match(&ann, &inf) {
            continue;
        }

        match (&*ann, &*inf) {
            (TypeData::ObjectKeyword, TypeData::InstanceOf(_)) => {
                found_wider = true;
            }

            (TypeData::InstanceOf(ann_inst), TypeData::InstanceOf(inf_inst)) => {
                let same_base = match (ann.resolve(&ann_inst.ty), inf.resolve(&inf_inst.ty)) {
                    (Some(a), Some(b)) => types_match(&a, &b),
                    _ => false,
                };
                if !same_base {
                    return false;
                }
                let ann_params = &ann_inst.type_parameters;
                let inf_params = &inf_inst.type_parameters;
                if ann_params.len() != inf_params.len() || ann_params.is_empty() {
                    return false;
                }
                for (ann_p, inf_p) in ann_params.iter().zip(inf_params.iter()) {
                    match (ann.resolve(ann_p), inf.resolve(inf_p)) {
                        (Some(a), Some(b)) => stack.push((a, resolve_generic_chain(&b))),
                        _ => return false,
                    }
                }
            }

            (TypeData::Object(ann_obj), TypeData::Object(inf_obj)) => {
                if !push_object_pairs(&ann, ann_obj, &inf, inf_obj, &mut stack) {
                    return false;
                }
            }

            (TypeData::Object(ann_obj), TypeData::Literal(lit)) => match lit.as_ref() {
                Literal::Object(inf_lit) => {
                    if !push_object_literal_pairs(&ann, ann_obj, inf_lit, &mut stack) {
                        return false;
                    }
                }
                _ => return false,
            },

            (TypeData::ObjectKeyword, inferred_data)
                if is_strictly_narrower_than_object_keyword(inferred_data) =>
            {
                found_wider = true;
            }

            (TypeData::Tuple(ann_tuple), TypeData::Tuple(inf_tuple)) => {
                let ann_elems = ann_tuple.elements();
                let inf_elems = inf_tuple.elements();
                if ann_elems.len() != inf_elems.len() || ann_elems.is_empty() {
                    return false;
                }
                for (ann_e, inf_e) in ann_elems.iter().zip(inf_elems.iter()) {
                    match (ann.resolve(&ann_e.ty), inf.resolve(&inf_e.ty)) {
                        (Some(a), Some(b)) => stack.push((a, resolve_generic_chain(&b))),
                        _ => return false,
                    }
                }
            }

            _ => return false,
        }
    }

    found_wider
}

/// Pushes property type pairs onto the work stack for pairwise comparison.
/// Also handles index signatures, which arise from `Record<K,V>` annotations.
fn push_object_pairs(
    annotated: &Type,
    ann_obj: &biome_js_type_info::Object,
    inferred: &Type,
    inf_obj: &biome_js_type_info::Object,
    stack: &mut Vec<(Type, Type)>,
) -> bool {
    if ann_obj.members.is_empty() || inf_obj.members.is_empty() {
        return false;
    }

    let ann_index_sig = ann_obj.members.iter().find(|m| {
        matches!(m.kind, biome_js_type_info::TypeMemberKind::IndexSignature(_))
    });
    if let Some(sig_member) = ann_index_sig
        && let Some(sig_value_ty) = annotated.resolve(&sig_member.ty)
    {
        for inf_m in inf_obj.members.iter() {
            match inferred.resolve(&inf_m.ty) {
                Some(inf_ty) => stack.push((sig_value_ty.clone(), resolve_generic_chain(&inf_ty))),
                None => return false,
            }
        }
        return true;
    }

    for ann_member in ann_obj.members.iter() {
        let ann_name = match &ann_member.kind {
            biome_js_type_info::TypeMemberKind::Named(name)
            | biome_js_type_info::TypeMemberKind::NamedOptional(name) => name,
            _ => continue,
        };
        let inf_member = inf_obj.members.iter().find(|m| m.kind.has_name(ann_name));
        let Some(inf_member) = inf_member else {
            return false;
        };
        match (annotated.resolve(&ann_member.ty), inferred.resolve(&inf_member.ty)) {
            (Some(a), Some(b)) => stack.push((a, resolve_generic_chain(&b))),
            _ => return false,
        }
    }

    true
}

fn push_object_literal_pairs(
    annotated: &Type,
    ann_obj: &biome_js_type_info::Object,
    inf_lit: &biome_js_type_info::ObjectLiteral,
    stack: &mut Vec<(Type, Type)>,
) -> bool {
    if ann_obj.members.is_empty() || inf_lit.members().is_empty() {
        return false;
    }

    for ann_member in ann_obj.members.iter() {
        let ann_name = match &ann_member.kind {
            biome_js_type_info::TypeMemberKind::Named(name)
            | biome_js_type_info::TypeMemberKind::NamedOptional(name) => name,
            _ => continue,
        };
        let inf_member = inf_lit.members().iter().find(|m| m.kind.has_name(ann_name));
        let Some(inf_member) = inf_member else {
            return false;
        };
        match (annotated.resolve(&ann_member.ty), annotated.resolve(&inf_member.ty)) {
            (Some(a), Some(b)) => stack.push((a, resolve_generic_chain(&b))),
            _ => return false,
        }
    }

    true
}

/// Checks whether `annotated` is strictly wider than `inferred`.
fn is_wider_than(annotated: &Type, inferred: &Type) -> bool {
    let current = resolve_generic_chain(inferred);

    match (&**annotated, &*current) {
        (TypeData::String, TypeData::String)
        | (TypeData::Number, TypeData::Number)
        | (TypeData::Boolean, TypeData::Boolean)
        | (TypeData::BigInt, TypeData::BigInt) => false,

        (TypeData::Union(_), _) => is_union_wider(annotated, &current),
        (_, TypeData::Union(_)) => {
            // When the annotation's base type already appears as a variant in the
            // inferred union, any literal subtypes are subsumed by it — the union
            // collapses to the base type (e.g., 0 | number = number).  In that
            // case the annotation is not wider than the inferred type.
            let (has_base_variant, all_subsumed, all_covered, any_wider) = current
                .flattened_union_variants()
                .fold(
                    (false, true, true, false),
                    |(has_base_variant, all_subsumed, all_covered, any_wider), v| {
                        let matches = types_match(annotated, &v);
                        let wider = is_nonunion_wider(annotated, &v);
                        (
                            has_base_variant || matches,
                            all_subsumed && (matches || is_base_type_of_literal(annotated, &v)),
                            all_covered && (matches || wider),
                            any_wider || wider,
                        )
                    },
                );
            if has_base_variant && all_subsumed {
                return false;
            }
            all_covered && any_wider
        }
        _ => is_nonunion_wider(annotated, &current),
    }
}

/// Checks whether a union annotation is wider than a set of return types.
fn is_union_wider_than_returns(annotated: &Type, returns: &[Type]) -> bool {
    let all_covered = returns.iter().all(|ret| {
        annotated
            .flattened_union_variants()
            .any(|ann_v| types_match(&ann_v, ret) || is_nonunion_wider(&ann_v, ret))
    });

    if !all_covered {
        return false;
    }

    let has_extra = annotated.flattened_union_variants().any(|ann_v| {
        !returns
            .iter()
            .any(|ret| types_match(&ann_v, ret) || is_nonunion_wider(&ann_v, ret))
    });

    let has_wider_variant = annotated
        .flattened_union_variants()
        .any(|ann_v| returns.iter().any(|ret| is_nonunion_wider(&ann_v, ret)));

    has_extra || has_wider_variant
}

/// Like `is_union_wider_than_returns` but for a single inferred type (used
/// inside `is_wider_than`). Also filters out generic variants whose
/// constraints are subsumed by other variants in the annotation union.
fn is_union_wider(annotated: &Type, inferred: &Type) -> bool {
    let all_inferred_covered = if let TypeData::Union(_) = &**inferred {
        inferred.flattened_union_variants().all(|inf_v| {
            annotated
                .flattened_union_variants()
                .any(|ann_v| types_match(&ann_v, &inf_v) || is_nonunion_wider(&ann_v, &inf_v))
        })
    } else {
        annotated
            .flattened_union_variants()
            .any(|ann_v| types_match(&ann_v, inferred) || is_nonunion_wider(&ann_v, inferred))
    };

    if !all_inferred_covered {
        return false;
    }

    let ann_variants: Vec<Type> = annotated.flattened_union_variants().collect();

    let inf_variants: Vec<Type> = match &**inferred {
        TypeData::Union(_) => inferred.flattened_union_variants().collect(),
        _ => vec![inferred.clone()],
    };

    ann_variants
        .iter()
        .filter(|ann_v| {
            if let TypeData::Generic(generic) = &***ann_v
                && generic.constraint.is_known()
                && let Some(constraint) = ann_v.resolve(&generic.constraint)
            {
                let subsumed = ann_variants.iter().any(|other| {
                    !std::ptr::eq(*ann_v as *const Type, other as *const Type)
                        && (types_match(other, &constraint)
                            || is_nonunion_wider(other, &constraint))
                });
                return !subsumed;
            }
            true
        })
        .any(|ann_v| {
            !inf_variants
                .iter()
                .any(|inf_v| types_match(ann_v, inf_v) || is_nonunion_wider(ann_v, inf_v))
        })
}

/// Checks structural equality between two types.
fn types_match(a: &Type, b: &Type) -> bool {
    let mut a = a.clone();
    let mut b = b.clone();
    loop {
        match (&*a, &*b) {
            (TypeData::String, TypeData::String)
            | (TypeData::Number, TypeData::Number)
            | (TypeData::Boolean, TypeData::Boolean)
            | (TypeData::BigInt, TypeData::BigInt)
            | (TypeData::Null, TypeData::Null)
            | (TypeData::Undefined, TypeData::Undefined)
            | (TypeData::VoidKeyword, TypeData::VoidKeyword)
            | (TypeData::NeverKeyword, TypeData::NeverKeyword)
            | (TypeData::ObjectKeyword, TypeData::ObjectKeyword) => return true,

            (TypeData::Literal(a_lit), TypeData::Literal(b_lit)) => return a_lit == b_lit,

            (TypeData::Generic(a_gen), TypeData::Generic(b_gen)) => {
                return a_gen.name == b_gen.name
            }

            (TypeData::InstanceOf(a_inst), TypeData::InstanceOf(b_inst))
                if a_inst.type_parameters.is_empty() && b_inst.type_parameters.is_empty() =>
            {
                match (a.resolve(&a_inst.ty), b.resolve(&b_inst.ty)) {
                    (Some(a_base), Some(b_base)) => {
                        a = a_base;
                        b = b_base;
                    }
                    _ => return false,
                }
            }

            (TypeData::Generic(a_gen), TypeData::InstanceOf(b_inst))
                if b_inst.type_parameters.is_empty() =>
            {
                if let Some(base) = b.resolve(&b_inst.ty)
                    && let TypeData::Generic(b_gen) = &*base
                {
                    return a_gen.name == b_gen.name;
                }
                return false;
            }
            (TypeData::InstanceOf(a_inst), TypeData::Generic(b_gen))
                if a_inst.type_parameters.is_empty() =>
            {
                if let Some(base) = a.resolve(&a_inst.ty)
                    && let TypeData::Generic(a_gen) = &*base
                {
                    return a_gen.name == b_gen.name;
                }
                return false;
            }

            _ => return false,
        }
    }
}
