//! Editorial policy for `noMisleadingReturnType`.
//!
//! This module decides which wider relations are misleading, when syntax
//! evidence suppresses a diagnostic, and how replacement types are rendered.

use std::iter::FusedIterator;

use biome_analyze::context::RuleContext;
use biome_js_syntax::{
    AnyJsExpression, AnyJsFunction, AnyJsFunctionBody, AnyJsGetter, AnyTsCastExpression, AnyTsType,
    JsArrowFunctionExpression, JsConstructorClassMember, JsFunctionBody, JsFunctionDeclaration,
    JsFunctionExpression, JsGetterClassMember, JsGetterObjectMember, JsIdentifierExpression,
    JsLogicalOperator, JsMethodClassMember, JsMethodObjectMember, JsReturnStatement,
    JsSetterClassMember, JsSetterObjectMember, JsSyntaxNode, JsVariableStatement,
    TsDeclareFunctionDeclaration, TsMethodSignatureClassMember,
};
use biome_js_type_info::{
    InferredType, NarrowedTypeCandidates, ReturnTypeRelation, ReturnTypeVerdict,
    resolved::{InferredLiteralValue, InferredTypeData},
};
use biome_rowan::{AstNode, TextRange, declare_node_union};
use smallvec::SmallVec;

use super::{AnyFunctionLikeWithReturnType, NoMisleadingReturnType, RuleState};

/// Maximum iterations for expression traversal to guard against infinite loops.
const MAX_EXPRESSION_TRAVERSAL_ITERATIONS: usize = 200;
const MAX_RETURN_TYPE_DESCRIPTION_LENGTH: usize = 80;
const RETURN_TYPE_SEPARATOR: &str = " | ";

/// Evaluates the queried function-like node and returns a diagnostic state only
/// when its declared return type is provably wider than its returns.
pub(super) fn run(ctx: &RuleContext<NoMisleadingReturnType>) -> Option<RuleState> {
    let node = ctx.query();

    match node {
        AnyFunctionLikeWithReturnType::AnyJsFunction(func) => run_for_function(ctx, func),
        AnyFunctionLikeWithReturnType::JsMethodClassMember(method) => {
            if method.star_token().is_some() {
                return None;
            }
            if is_class_method_overload_implementation(method) {
                return None;
            }
            let annotation = method.return_type_annotation()?;
            let name = method
                .name()
                .ok()?
                .as_js_literal_member_name()?
                .name()
                .ok()?;
            let return_type = ctx.inferred_return_type_of_member(method.syntax(), name.text())?;
            run_for_member(
                ctx,
                annotation.range(),
                return_type,
                method.async_token().is_some(),
                prefers_inferred_suggestion(annotation.syntax()),
                &method.body().ok()?,
            )
        }
        AnyFunctionLikeWithReturnType::JsMethodObjectMember(method) => {
            if method.star_token().is_some() {
                return None;
            }
            let annotation = method.return_type_annotation()?;
            let name = method
                .name()
                .ok()?
                .as_js_literal_member_name()?
                .name()
                .ok()?;
            let return_type = ctx.inferred_return_type_of_member(method.syntax(), name.text())?;
            run_for_member(
                ctx,
                annotation.range(),
                return_type,
                method.async_token().is_some(),
                prefers_inferred_suggestion(annotation.syntax()),
                &method.body().ok()?,
            )
        }
        AnyFunctionLikeWithReturnType::JsGetterClassMember(getter) => {
            let annotation = getter.return_type()?;
            let any_getter = AnyJsGetter::from(getter.clone());
            let name = any_getter.member_name()?;
            if any_getter.has_matching_setter(&name) {
                return None;
            }
            let return_type = ctx.inferred_return_type_of_member(getter.syntax(), name.text())?;
            run_for_member(
                ctx,
                annotation.range(),
                return_type,
                false,
                prefers_inferred_suggestion(annotation.syntax()),
                &getter.body().ok()?,
            )
        }
        AnyFunctionLikeWithReturnType::JsGetterObjectMember(getter) => {
            let annotation = getter.return_type()?;
            let any_getter = AnyJsGetter::from(getter.clone());
            let name = any_getter.member_name()?;
            if any_getter.has_matching_setter(&name) {
                return None;
            }
            let return_type = ctx.inferred_return_type_of_member(getter.syntax(), name.text())?;
            run_for_member(
                ctx,
                annotation.range(),
                return_type,
                false,
                prefers_inferred_suggestion(annotation.syntax()),
                &getter.body().ok()?,
            )
        }
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
        if let Some(decl) = TsDeclareFunctionDeclaration::cast(sibling.clone()) {
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
    let prefer_inferred_suggestion = prefers_inferred_suggestion(annotation.syntax());

    if node.is_generator() || is_overload_implementation(node) {
        return None;
    }

    let return_type = ctx.inferred_return_type_of_function(node)?;
    let is_async = node.async_token().is_some();
    let body = node.body().ok()?;

    run_for_member_with_body(
        ctx,
        annotation_range,
        return_type,
        is_async,
        prefer_inferred_suggestion,
        &body,
    )
}

fn run_for_member<'db>(
    ctx: &'db RuleContext<NoMisleadingReturnType>,
    annotation_range: TextRange,
    return_type: InferredType<'db>,
    is_async: bool,
    prefer_inferred_suggestion: bool,
    body: &JsFunctionBody,
) -> Option<RuleState> {
    run_for_member_with_body(
        ctx,
        annotation_range,
        return_type,
        is_async,
        prefer_inferred_suggestion,
        &AnyJsFunctionBody::JsFunctionBody(body.clone()),
    )
}

/// Compares a member's declared type with the return evidence collected from
/// its body.
///
/// Async declarations compare their `Promise` argument when it is suitable for
/// relation analysis. Missing inference suppresses the diagnostic.
fn run_for_member_with_body<'db>(
    ctx: &'db RuleContext<NoMisleadingReturnType>,
    annotation_range: TextRange,
    return_type: InferredType<'db>,
    is_async: bool,
    prefer_inferred_suggestion: bool,
    body: &AnyJsFunctionBody,
) -> Option<RuleState> {
    let info = collect_return_info(ctx, body);
    if !return_type.is_inferred() || info.has_uninferred_return {
        return None;
    }
    let declared = if is_async {
        return_type
            .promise_inner_type()
            .filter(|inner| !inner.is_return_type_relation_escape_hatch())
            .unwrap_or(return_type)
    } else {
        return_type
    };
    let relation = declared.compare_declared_return_type(&info.types);
    let suggestion = misleading_suggestion(&relation, &info, prefer_inferred_suggestion)?;
    Some(RuleState {
        annotation_range,
        suggestion,
    })
}

/// Applies diagnostic policy to a computed return-type relation.
///
/// The outer `None` suppresses the diagnostic. `Some(None)` emits a diagnostic
/// without a replacement type, while `Some(Some(_))` includes a rendered
/// replacement.
fn misleading_suggestion(
    relation: &ReturnTypeRelation<'_>,
    evidence: &ReturnTypeEvidence<'_>,
    prefer_inferred_suggestion: bool,
) -> Option<Option<String>> {
    let declared = relation.declared();
    if relation.declared_is_escape_hatch()
        || relation.inferred_is_empty()
        || relation.has_any_contaminated_inferred()
        || relation.declared_union_contains_unknown()
        || relation.has_undefined_mismatch()
        || relation.inferred_has_generic_intersection()
    {
        return None;
    }
    if relation.has_single_primitive_literal_return()
        && !evidence.has_any_const
        && !evidence.has_pinning_assertion
        && !matches!(declared, InferredTypeData::Union(_))
    {
        return None;
    }
    if !evidence.has_any_const
        && evidence.object_wide_casts == relation.inferred().len()
        && matches!(declared, InferredTypeData::ObjectKeyword)
    {
        return None;
    }
    if !evidence.has_any_const && relation.is_only_property_literal_widening()? {
        return None;
    }

    let is_misleading = if matches!(declared, InferredTypeData::ObjectKeyword) {
        !relation.includes_object_return()
            && evidence.object_wide_casts == 0
            && (evidence.has_narrower_than_object || relation.object_has_wider_return()?)
    } else {
        matches!(relation.verdict(), ReturnTypeVerdict::Wider)
    };
    if !is_misleading {
        return None;
    }

    let inferred = || render_inferred(relation);
    let suggestion = if evidence.has_any_const || prefer_inferred_suggestion {
        inferred()
    } else {
        match relation.narrowed() {
            NarrowedTypeCandidates::Available(types) => {
                render_narrowed(relation, types).or_else(inferred)
            }
            NarrowedTypeCandidates::Unavailable => inferred(),
            NarrowedTypeCandidates::Indeterminate => return None,
        }
    };
    Some(suggestion)
}

fn literal_text(relation: &ReturnTypeRelation<'_>, ty: InferredTypeData<'_>) -> Option<String> {
    let InferredTypeData::Literal(literal) = ty else {
        return None;
    };
    match literal.literal(relation.db()) {
        InferredLiteralValue::String(value) => Some(format!("\"{}\"", value.as_str())),
        InferredLiteralValue::Number(value) => Some(value.as_str().to_string()),
        InferredLiteralValue::Boolean(value) => Some(value.as_bool().to_string()),
        InferredLiteralValue::BigInt(_)
        | InferredLiteralValue::Object(_)
        | InferredLiteralValue::RegExp(_)
        | InferredLiteralValue::Template(_) => None,
    }
}

fn renderable_variant(
    relation: &ReturnTypeRelation<'_>,
    ty: InferredTypeData<'_>,
) -> Option<String> {
    match ty {
        InferredTypeData::String => Some("string".into()),
        InferredTypeData::Number => Some("number".into()),
        InferredTypeData::Boolean => Some("boolean".into()),
        InferredTypeData::BigInt => Some("bigint".into()),
        InferredTypeData::Literal(_) => literal_text(relation, ty),
        _ => None,
    }
}

fn clean_literal_text(text: &str) -> bool {
    !text.contains("...") && !text.contains("__internal") && !text.contains("typeof import(")
}

/// Joins type fragments only when every fragment is suitable for source text
/// and the result fits the diagnostic length limit.
fn join_description(parts: &[String]) -> Option<String> {
    if parts.is_empty() || parts.iter().any(|part| !clean_literal_text(part)) {
        return None;
    }
    let description = parts.join(RETURN_TYPE_SEPARATOR);
    (description.len() <= MAX_RETURN_TYPE_DESCRIPTION_LENGTH).then_some(description)
}

/// Renders inferred returns when every return is a supported literal.
fn render_inferred(relation: &ReturnTypeRelation<'_>) -> Option<String> {
    let parts = relation
        .inferred()
        .iter()
        .map(|ty| literal_text(relation, *ty))
        .collect::<Option<Vec<_>>>()?;
    join_description(&parts)
}

/// Renders narrowed union candidates when every candidate has a stable source
/// representation.
fn render_narrowed(
    relation: &ReturnTypeRelation<'_>,
    types: &[InferredTypeData<'_>],
) -> Option<String> {
    let parts = types
        .iter()
        .map(|ty| renderable_variant(relation, *ty))
        .collect::<Option<Vec<_>>>()?;
    join_description(&parts)
}

/// Returns whether the annotation mixes a primitive with a literal of the same
/// primitive family.
///
/// Narrowing such an annotation by removing variants would preserve a
/// redundant primitive, so diagnostics prefer the inferred literal set.
fn prefers_inferred_suggestion(annotation: &JsSyntaxNode) -> bool {
    let mut primitives = [false; 4];
    let mut literals = [false; 4];
    for ty in annotation.descendants().filter_map(AnyTsType::cast) {
        match ty {
            AnyTsType::TsStringType(_) => primitives[0] = true,
            AnyTsType::TsNumberType(_) => primitives[1] = true,
            AnyTsType::TsBooleanType(_) => primitives[2] = true,
            AnyTsType::TsBigintType(_) => primitives[3] = true,
            AnyTsType::TsStringLiteralType(_) | AnyTsType::TsTemplateLiteralType(_) => {
                literals[0] = true;
            }
            AnyTsType::TsNumberLiteralType(_) => literals[1] = true,
            AnyTsType::TsBooleanLiteralType(_) => literals[2] = true,
            AnyTsType::TsBigintLiteralType(_) => literals[3] = true,
            _ => {}
        }
    }
    primitives
        .iter()
        .zip(literals)
        .any(|(primitive, literal)| *primitive && literal)
}

/// Returns whether a class method has a sibling overload signature with the
/// same literal name.
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

/// Per-body accumulator for the misleading-return check.
#[derive(Default)]
struct ReturnTypeEvidence<'db> {
    types: Vec<InferredType<'db>>,
    has_uninferred_return: bool,
    has_any_const: bool,
    /// Count of return expressions with an assertion target treated as at least
    /// as wide as `object`.
    object_wide_casts: usize,
    /// Whether any return expression reveals structure narrower than the
    /// TypeScript `object` keyword, such as members, tuples, functions, or
    /// class instances.
    has_narrower_than_object: bool,
    /// Whether a return pins its type with a non-`const` `as`/`<>` assertion,
    /// so its literal does not widen.
    has_pinning_assertion: bool,
}

/// Walks the function body and populates [`ReturnTypeEvidence`].
fn collect_return_info<'db>(
    ctx: &'db RuleContext<NoMisleadingReturnType>,
    body: &AnyJsFunctionBody,
) -> ReturnTypeEvidence<'db> {
    let mut info = ReturnTypeEvidence::default();

    match body {
        AnyJsFunctionBody::JsFunctionBody(block) => {
            collect_block_returns(ctx, block, &mut info);
        }
        AnyJsFunctionBody::AnyJsExpression(expr) => {
            if has_const_assertion(expr) {
                info.has_any_const = true;
            } else {
                if has_object_wide_assertion(ctx, expr) {
                    info.object_wide_casts += 1;
                } else if has_narrow_cast(ctx, expr) || expression_reveals_narrow_object(expr) {
                    info.has_narrower_than_object = true;
                }
            }
            info.has_pinning_assertion |= is_pinned_by_assertion(expr);
            match infer_expression_type(ctx, expr) {
                Some(ty) => info.types.push(ty),
                None => info.has_uninferred_return = true,
            }
        }
    }
    info
}

/// Adds return evidence from the current function body while pruning nested
/// function-like nodes.
fn collect_block_returns<'db>(
    ctx: &'db RuleContext<NoMisleadingReturnType>,
    block: &JsFunctionBody,
    info: &mut ReturnTypeEvidence<'db>,
) {
    for node in block
        .syntax()
        .pruned_descendents(|n| !AnyNestedFunctionLike::can_cast(n.kind()))
    {
        let Some(ret) = JsReturnStatement::cast(node) else {
            continue;
        };
        if let Some(arg) = ret.argument()
            && let Some(expr) = AnyJsExpression::cast(arg.syntax().clone())
        {
            if has_const_assertion(&expr) {
                info.has_any_const = true;
            } else {
                if has_object_wide_assertion(ctx, &expr) {
                    info.object_wide_casts += 1;
                } else if has_narrow_cast(ctx, &expr) || expression_reveals_narrow_object(&expr) {
                    info.has_narrower_than_object = true;
                }
            }
            info.has_pinning_assertion |= is_pinned_by_assertion(&expr);
            match infer_expression_type(ctx, &expr) {
                Some(ty) => info.types.push(ty),
                None => info.has_uninferred_return = true,
            }
        }
    }
}

/// Whether the return expression has a narrow object shape hidden behind
/// `: object`. Bare `{}` and empty classes are not flagged; spreads count
/// because type-info drops them today.
fn expression_reveals_narrow_object(expression: &AnyJsExpression) -> bool {
    NonTransparentLeaves::new(expression, LogicalTraversal::All).any(|leaf| match leaf {
        AnyJsExpression::JsObjectExpression(object) => {
            object.members().into_iter().next().is_some()
        }
        _ => false,
    })
}

/// Returns `true` when the expression contains a cast whose target is
/// strictly narrower than the `object` keyword.
fn has_narrow_cast(
    ctx: &RuleContext<NoMisleadingReturnType>,
    expression: &AnyJsExpression,
) -> bool {
    NonTransparentLeaves::new(expression, LogicalTraversal::All).any(|leaf| {
        let cast = match &leaf {
            AnyJsExpression::TsAsExpression(expression) => {
                AnyTsCastExpression::from(expression.clone())
            }
            AnyJsExpression::TsTypeAssertionExpression(expression) => {
                AnyTsCastExpression::from(expression.clone())
            }
            _ => return false,
        };
        !cast_target_at_least_object_wide(ctx, &leaf, &cast)
    })
}

/// Whether some non-transparent leaf reached through the fallback walk has a
/// type assertion that opts into `object` widening. `as const` is excluded
/// because it narrows rather than widens.
fn has_object_wide_assertion(
    ctx: &RuleContext<NoMisleadingReturnType>,
    expression: &AnyJsExpression,
) -> bool {
    let mut leaves = NonTransparentLeaves::new(expression, LogicalTraversal::FallbackOnly);
    let any_wide = leaves.by_ref().any(|leaf| {
        let cast = match &leaf {
            AnyJsExpression::TsAsExpression(expression) => {
                AnyTsCastExpression::from(expression.clone())
            }
            AnyJsExpression::TsTypeAssertionExpression(expression) => {
                AnyTsCastExpression::from(expression.clone())
            }
            _ => return false,
        };
        let cast_type = cast.cast_type();
        if is_const_reference_type(&cast_type) {
            return false;
        }
        if cast_type.is_none() {
            return false;
        }
        cast_target_at_least_object_wide(ctx, &leaf, &cast)
    });
    any_wide || leaves.cap_exceeded()
}

#[derive(Clone, Copy)]
enum LogicalTraversal {
    /// Walk every logical operand.
    All,
    /// Walk `||`/`??`, but treat `&&` as opaque.
    FallbackOnly,
}

/// Iterator yielding the non-transparent expression leaves reachable from a
/// root expression. Parentheses, ternaries, `satisfies`, non-null assertions,
/// sequences, `await`, logical expressions, and const identifier initializers
/// are walked according to [`LogicalTraversal`].
struct NonTransparentLeaves {
    stack: SmallVec<[AnyJsExpression; 4]>,
    iterations: usize,
    logical_mode: LogicalTraversal,
    cap_exceeded: bool,
}

impl NonTransparentLeaves {
    /// Creates a bounded leaf traversal using the requested logical-expression
    /// policy.
    fn new(expression: &AnyJsExpression, logical_mode: LogicalTraversal) -> Self {
        let mut stack: SmallVec<[AnyJsExpression; 4]> = SmallVec::new();
        stack.push(expression.clone().omit_parentheses());
        Self {
            stack,
            iterations: 0,
            logical_mode,
            cap_exceeded: false,
        }
    }

    /// `true` if iteration stopped because the cap was hit.
    fn cap_exceeded(&self) -> bool {
        self.cap_exceeded
    }

    /// Push the transparent operand(s) of `current` onto the stack. Returns
    /// `true` when `current` is fully transparent.
    fn push_transparent_operand(&mut self, current: &AnyJsExpression) -> bool {
        match current {
            AnyJsExpression::TsNonNullAssertionExpression(expression) => {
                if let Ok(inner) = expression.expression() {
                    self.stack.push(inner.omit_parentheses());
                }
                true
            }
            AnyJsExpression::JsSequenceExpression(expression) => {
                if let Ok(inner) = expression.right() {
                    self.stack.push(inner.omit_parentheses());
                }
                true
            }
            AnyJsExpression::JsAwaitExpression(expression) => {
                if let Ok(inner) = expression.argument() {
                    self.stack.push(inner.omit_parentheses());
                }
                true
            }
            AnyJsExpression::JsLogicalExpression(expression) => {
                if matches!(self.logical_mode, LogicalTraversal::FallbackOnly)
                    && matches!(expression.operator(), Ok(JsLogicalOperator::LogicalAnd))
                {
                    return false;
                }
                if let Ok(left) = expression.left() {
                    self.stack.push(left.omit_parentheses());
                }
                if let Ok(right) = expression.right() {
                    self.stack.push(right.omit_parentheses());
                }
                true
            }
            AnyJsExpression::JsIdentifierExpression(identifier) => {
                if let Some(init) = resolve_const_identifier_initializer_expression(identifier) {
                    self.stack.push(init.omit_parentheses());
                    return true;
                }
                false
            }
            _ => false,
        }
    }
}

impl Iterator for NonTransparentLeaves {
    type Item = AnyJsExpression;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cap_exceeded {
            return None;
        }
        while let Some(current) = self.stack.pop() {
            self.iterations += 1;
            if self.iterations > MAX_EXPRESSION_TRAVERSAL_ITERATIONS {
                self.cap_exceeded = true;
                return None;
            }
            if self.push_transparent_operand(&current) {
                continue;
            }
            if let AnyJsExpression::JsConditionalExpression(conditional) = &current {
                if let Ok(consequent) = conditional.consequent() {
                    self.stack.push(consequent.omit_parentheses());
                }
                if let Ok(alternate) = conditional.alternate() {
                    self.stack.push(alternate.omit_parentheses());
                }
                continue;
            }
            if let AnyJsExpression::TsSatisfiesExpression(satisfies) = &current {
                if let Ok(inner) = satisfies.expression() {
                    self.stack.push(inner.omit_parentheses());
                }
                continue;
            }
            return Some(current);
        }
        None
    }
}

impl FusedIterator for NonTransparentLeaves {}

/// Resolves a local const identifier to the initializer visible at the use site.
fn resolve_const_identifier_initializer_expression(
    id_expr: &JsIdentifierExpression,
) -> Option<AnyJsExpression> {
    let name = id_expr
        .name()
        .ok()
        .and_then(|n| n.value_token().ok())
        .map(|t| t.token_text_trimmed())?;

    let mut current = id_expr.syntax().clone();
    while let Some(parent) = current.parent() {
        if let Some(init) = const_initializer_before_child(&parent, &current, name.text()) {
            return Some(init);
        }
        if JsFunctionBody::can_cast(parent.kind()) {
            break;
        }
        current = parent;
    }

    None
}

/// Finds the last matching const initializer before `child_on_path`.
fn const_initializer_before_child(
    parent: &JsSyntaxNode,
    child_on_path: &JsSyntaxNode,
    name: &str,
) -> Option<AnyJsExpression> {
    let mut initializer = None;
    for child in parent.children() {
        if child == *child_on_path {
            break;
        }
        if let Some(found) = const_initializer_from_child(&child, name) {
            initializer = Some(found);
        }
    }
    initializer
}

/// Extracts a matching const initializer from a variable statement.
fn const_initializer_from_child(child: &JsSyntaxNode, name: &str) -> Option<AnyJsExpression> {
    let statement = JsVariableStatement::cast(child.clone())?;
    let declaration = statement.declaration().ok()?;
    if declaration
        .kind()
        .ok()
        .is_none_or(|kind| kind.text_trimmed() != "const")
    {
        return None;
    }

    for declarator in declaration.declarators() {
        let Ok(declarator) = declarator else {
            continue;
        };
        let matches_name = declarator
            .id()
            .ok()
            .and_then(|id| id.as_any_js_binding().cloned())
            .and_then(|binding| binding.as_js_identifier_binding().cloned())
            .and_then(|binding| binding.name_token().ok())
            .is_some_and(|token| token.token_text_trimmed().text() == name);
        if matches_name {
            return declarator
                .initializer()
                .and_then(|initializer| initializer.expression().ok());
        }
    }

    None
}

/// Returns `true` if the cast target is at least as wide as `object`;
/// `false` if it is known to be strictly narrower.
fn cast_target_at_least_object_wide(
    ctx: &RuleContext<NoMisleadingReturnType>,
    expression: &AnyJsExpression,
    _cast: &AnyTsCastExpression,
) -> bool {
    ctx.inferred_type_of_expression(expression)
        .is_none_or(InferredType::is_at_least_as_wide_as_object)
}

/// Gets the type of a return expression. For identifiers bound to an `as const`
/// initializer, uses the initializer's literal type instead of the widened binding.
fn infer_expression_type<'db>(
    ctx: &'db RuleContext<NoMisleadingReturnType>,
    expr: &AnyJsExpression,
) -> Option<InferredType<'db>> {
    let inner = unwrap_type_wrappers(expr);

    if let AnyJsExpression::JsIdentifierExpression(ref id_expr) = inner
        && let Some(init_type) = resolve_identifier_initializer_type(ctx, id_expr)
    {
        return Some(init_type);
    }

    ctx.inferred_type_of_expression(&inner)
}

/// Returns the initializer's literal type when the identifier resolves to a
/// visible `const` binding initialized with a direct const assertion.
fn resolve_identifier_initializer_type<'db>(
    ctx: &'db RuleContext<NoMisleadingReturnType>,
    id_expr: &JsIdentifierExpression,
) -> Option<InferredType<'db>> {
    let init_expr = resolve_const_identifier_initializer_expression(id_expr)?;
    if !init_has_direct_const_assertion(&init_expr) {
        return None;
    }
    let unwrapped = unwrap_type_wrappers(&init_expr);
    ctx.inferred_type_of_expression(&unwrapped)
}

/// Removes parentheses and type wrappers that preserve literal inference.
///
/// Widening `as T` and angle-bracket assertions remain intact so inference uses
/// their asserted type.
fn unwrap_type_wrappers(expr: &AnyJsExpression) -> AnyJsExpression {
    let mut current = expr.clone();
    loop {
        if let Some(cast) = AnyTsCastExpression::cast(current.syntax().clone()) {
            // Only `satisfies` and `as const` are transparent; keep a widening
            // `as T` so its target type is read, not the inner literal.
            let is_transparent = matches!(current, AnyJsExpression::TsSatisfiesExpression(_))
                || is_const_reference_type(&cast.cast_type());
            if is_transparent {
                let Some(inner) = cast.inner_expression() else {
                    return current;
                };
                current = inner;
                continue;
            }
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

/// Whether the return keeps a non-`const` `as`/`<>` cast after `satisfies` and
/// `as const` are unwrapped.
fn is_pinned_by_assertion(expr: &AnyJsExpression) -> bool {
    matches!(
        unwrap_type_wrappers(expr),
        AnyJsExpression::TsAsExpression(_) | AnyJsExpression::TsTypeAssertionExpression(_)
    )
}

/// Returns whether an expression or its visible const initializer is pinned by
/// a const assertion.
fn has_const_assertion(expr: &AnyJsExpression) -> bool {
    let mut current = expr.clone();
    loop {
        match &current {
            AnyJsExpression::TsAsExpression(e) => return is_const_reference_type(&e.ty().ok()),
            AnyJsExpression::TsTypeAssertionExpression(e) => {
                return is_const_reference_type(&e.ty().ok());
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
                return identifier_refers_to_const_assertion(id_expr);
            }
            _ => return false,
        }
    }
}

fn identifier_refers_to_const_assertion(id_expr: &JsIdentifierExpression) -> bool {
    resolve_const_identifier_initializer_expression(id_expr)
        .is_some_and(|init_expr| init_has_direct_const_assertion(&init_expr))
}

/// Checks for `as const` on the expression itself, without following identifiers.
fn init_has_direct_const_assertion(expr: &AnyJsExpression) -> bool {
    let mut current = expr.clone();
    loop {
        match &current {
            AnyJsExpression::TsAsExpression(e) => return is_const_reference_type(&e.ty().ok()),
            AnyJsExpression::TsTypeAssertionExpression(e) => {
                return is_const_reference_type(&e.ty().ok());
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

fn is_const_reference_type(ty: &Option<AnyTsType>) -> bool {
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
