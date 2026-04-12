use biome_analyze::{Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsExpression, AnyJsFunction, AnyJsFunctionBody, JsFunctionBody, JsGetterClassMember,
    JsGetterObjectMember, JsLiteralMemberName, JsMethodClassMember, JsMethodObjectMember,
    JsReturnStatement, JsSyntaxKind, JsSyntaxNode, JsVariableDeclarator, JsVariableStatement,
    TsAsExpression, TsMethodSignatureClassMember, TsTypeAssertionExpression,
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
                let name = literal_member_name_text(method.name().ok()?.as_js_literal_member_name()?)?;
                let func_type = ctx.type_of_member(method.syntax(), name.text());
                run_for_member(ctx, annotation.range(), &func_type, method.async_token().is_some(), &method.body().ok()?)
            }
            AnyFunctionLikeWithReturnType::JsMethodObjectMember(method) => {
                if method.star_token().is_some() {
                    return None;
                }
                let annotation = method.return_type_annotation()?;
                let name = literal_member_name_text(method.name().ok()?.as_js_literal_member_name()?)?;
                let func_type = ctx.type_of_member(method.syntax(), name.text());
                run_for_member(ctx, annotation.range(), &func_type, method.async_token().is_some(), &method.body().ok()?)
            }
            AnyFunctionLikeWithReturnType::JsGetterClassMember(getter) => {
                let annotation = getter.return_type()?;
                let name = literal_member_name_text(getter.name().ok()?.as_js_literal_member_name()?)?;
                let func_type = ctx.type_of_member(getter.syntax(), name.text());
                run_for_member(ctx, annotation.range(), &func_type, false, &getter.body().ok()?)
            }
            AnyFunctionLikeWithReturnType::JsGetterObjectMember(getter) => {
                let annotation = getter.return_type()?;
                let name = literal_member_name_text(getter.name().ok()?.as_js_literal_member_name()?)?;
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

    let (returns, has_any_const_return) = collect_return_info(ctx, body);

    if returns.is_empty() {
        return None;
    }

    if returns.len() == 1 && !has_any_const_return && is_literal_of_primitive(&returns[0]) {
        return None;
    }

    if matches!(&*effective_return_ty, TypeData::Boolean)
        && returns.iter().any(|ty| matches!(&**ty, TypeData::Literal(lit) if matches!(lit.as_ref(), Literal::Boolean(b) if b.as_bool())))
        && returns.iter().any(|ty| matches!(&**ty, TypeData::Literal(lit) if matches!(lit.as_ref(), Literal::Boolean(b) if !b.as_bool())))
    {
        return None;
    }

    if returns.iter().any(is_any_contaminated) {
        return None;
    }

    if includes_undefined(&effective_return_ty)
        && !returns.iter().any(includes_undefined)
    {
        return None;
    }

    if returns.iter().any(is_intersection_with_type_param) {
        return None;
    }

    if !has_any_const_return
        && is_only_property_literal_widening(&effective_return_ty, &returns)
    {
        return None;
    }

    let is_misleading = if effective_return_ty.is_union() {
        is_union_wider_than_returns(&effective_return_ty, &returns)
    } else {
        returns
            .iter()
            .all(|inferred| is_wider_than(&effective_return_ty, inferred))
    };

    if !is_misleading {
        return None;
    }

    Some(RuleState {
        annotation_range,
        returns,
    })
}

fn literal_member_name_text(name: &JsLiteralMemberName) -> Option<TokenText> {
    Some(name.value().ok()?.token_text_trimmed())
}

fn is_class_method_overload_implementation(method: &JsMethodClassMember) -> bool {
    let name = method
        .name()
        .ok()
        .and_then(|n| n.as_js_literal_member_name().cloned())
        .and_then(|n| n.value().ok())
        .map(|t| t.token_text_trimmed());
    let Some(name) = name else { return false };

    let Some(member_list) = method.syntax().parent() else {
        return false;
    };

    member_list.children().any(|child| {
        child.kind() == JsSyntaxKind::TS_METHOD_SIGNATURE_CLASS_MEMBER
            && TsMethodSignatureClassMember::cast(child)
                .and_then(|sig| sig.name().ok())
                .and_then(|n| n.as_js_literal_member_name().cloned())
                .and_then(|n| n.value().ok())
                .map(|t| t.token_text_trimmed())
                .is_some_and(|sig_name| sig_name == name)
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
    matches!(&**ty, TypeData::Literal(lit) if lit.is_primitive())
}

/// Checks whether annotation differs from returns only by property-level
/// literal widening that contextual typing would handle.
fn is_only_property_literal_widening(annotation: &Type, returns: &[Type]) -> bool {
    if let TypeData::Tuple(ann_tuple) = &**annotation {
        return returns.iter().all(|inferred| {
            let TypeData::Tuple(inf_tuple) = &**inferred else {
                return false;
            };
            let ann_elems = ann_tuple.elements();
            let inf_elems = inf_tuple.elements();
            if ann_elems.len() != inf_elems.len() || ann_elems.is_empty() {
                return false;
            }
            let mut has_widening = false;
            for (ann_elem, inf_elem) in ann_elems.iter().zip(inf_elems.iter()) {
                let ann_ty = annotation.resolve(&ann_elem.ty);
                let inf_ty = inferred.resolve(&inf_elem.ty);
                match (ann_ty, inf_ty) {
                    (Some(a), Some(b)) => {
                        if types_match(&a, &b) {
                            continue;
                        }
                        if is_base_type_of_literal(&a, &b) {
                            has_widening = true;
                        } else {
                            return false;
                        }
                    }
                    _ => return false,
                }
            }
            has_widening
        });
    }

    let (TypeData::Object(ann_obj), _) = (&**annotation, ()) else {
        return false;
    };

    if ann_obj.members.is_empty() {
        return false;
    }

    returns.iter().all(|inferred| {
        let inf_members = match &**inferred {
            TypeData::Object(obj) => &obj.members,
            TypeData::Literal(lit) => match lit.as_ref() {
                Literal::Object(obj_lit) => obj_lit.members(),
                _ => return false,
            },
            _ => return false,
        };

        if inf_members.is_empty() {
            return false;
        }

        let mut has_widening = false;

        let ann_index_sig = ann_obj.members.iter().find(|m| {
            matches!(m.kind, biome_js_type_info::TypeMemberKind::IndexSignature(_))
        });
        if let Some(sig_member) = ann_index_sig
            && let Some(sig_value_ty) = annotation.resolve(&sig_member.ty) {
                let mut sig_has_widening = false;
                let all_ok = inf_members.iter().all(|inf_m| {
                    if let Some(inf_ty) = annotation.resolve(&inf_m.ty) {
                        if types_match(&sig_value_ty, &inf_ty) {
                            return true;
                        }
                        if is_base_type_of_literal(&sig_value_ty, &inf_ty) {
                            sig_has_widening = true;
                            return true;
                        }
                    }
                    false
                });
                return all_ok && sig_has_widening;
            }

        for ann_member in ann_obj.members.iter() {
            let ann_name = match &ann_member.kind {
                biome_js_type_info::TypeMemberKind::Named(name) => name,
                _ => continue,
            };

            let inf_member = inf_members.iter().find(|m| m.kind.has_name(ann_name));
            let Some(inf_member) = inf_member else {
                return false;
            };

            let ann_ty = annotation.resolve(&ann_member.ty);
            let inf_ty = annotation.resolve(&inf_member.ty);

            match (ann_ty, inf_ty) {
                (Some(a), Some(b)) => {
                    if types_match(&a, &b) {
                        continue;
                    }
                    if is_base_type_of_literal(&a, &b) {
                        has_widening = true;
                    } else {
                        return false;
                    }
                }
                _ => return false,
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

/// Collects return types and tracks `as const` usage from a function body.
fn collect_return_info(
    ctx: &RuleContext<NoMisleadingReturnType>,
    body: &AnyJsFunctionBody,
) -> (Vec<Type>, bool) {
    let mut has_any_const = false;

    let types = match body {
        AnyJsFunctionBody::JsFunctionBody(block) => {
            collect_block_returns(ctx, block, &mut has_any_const)
        }
        AnyJsFunctionBody::AnyJsExpression(expr) => {
            if has_const_assertion(expr) {
                has_any_const = true;
            }
            vec![infer_expression_type(ctx, expr)]
        }
    };

    (types, has_any_const)
}

fn collect_block_returns(
    ctx: &RuleContext<NoMisleadingReturnType>,
    block: &JsFunctionBody,
    has_any_const: &mut bool,
) -> Vec<Type> {
    let mut returns = Vec::new();

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
                *has_any_const = true;
            }
            returns.push(infer_expression_type(ctx, &expr));
        }
    }

    returns
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

    let body_node = id_expr
        .syntax()
        .ancestors()
        .find(|ancestor| ancestor.kind() == JsSyntaxKind::JS_FUNCTION_BODY)?;
    let body = JsFunctionBody::cast(body_node)?;

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
        match &current {
            AnyJsExpression::TsAsExpression(e) => match e.expression() {
                Ok(inner) => current = inner,
                Err(_) => return current,
            },
            AnyJsExpression::TsSatisfiesExpression(e) => match e.expression() {
                Ok(inner) => current = inner,
                Err(_) => return current,
            },
            AnyJsExpression::TsTypeAssertionExpression(e) => match e.expression() {
                Ok(inner) => current = inner,
                Err(_) => return current,
            },
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

    let enclosing_body = id_expr
        .syntax()
        .ancestors()
        .find(|ancestor| ancestor.kind() == JsSyntaxKind::JS_FUNCTION_BODY);
    let Some(body_node) = enclosing_body else {
        return false;
    };
    let Some(body) = JsFunctionBody::cast(body_node) else {
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

fn is_nested_function_like(node: &JsSyntaxNode) -> bool {
    matches!(
        node.kind(),
        JsSyntaxKind::JS_FUNCTION_EXPRESSION
            | JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION
            | JsSyntaxKind::JS_FUNCTION_DECLARATION
            | JsSyntaxKind::JS_CONSTRUCTOR_CLASS_MEMBER
            | JsSyntaxKind::JS_METHOD_CLASS_MEMBER
            | JsSyntaxKind::JS_METHOD_OBJECT_MEMBER
            | JsSyntaxKind::JS_GETTER_CLASS_MEMBER
            | JsSyntaxKind::JS_GETTER_OBJECT_MEMBER
            | JsSyntaxKind::JS_SETTER_CLASS_MEMBER
            | JsSyntaxKind::JS_SETTER_OBJECT_MEMBER
    )
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
        if iterations > 50 {
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
            biome_js_type_info::TypeMemberKind::Named(name) => name,
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
            biome_js_type_info::TypeMemberKind::Named(name) => name,
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
            current
                .flattened_union_variants()
                .all(|v| types_match(annotated, &v) || is_nonunion_wider(annotated, &v))
                && current
                    .flattened_union_variants()
                    .any(|v| is_nonunion_wider(annotated, &v))
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
            | (TypeData::NeverKeyword, TypeData::NeverKeyword) => return true,

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
