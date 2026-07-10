use std::io;

use biome_analyze::{Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule};
use biome_console::{
    fmt::{Display, Formatter},
    markup,
};
use biome_js_syntax::{
    AnyJsExpression, AnyJsFunction, AnyJsFunctionBody, AnyJsGetter, AnyTsCastExpression, AnyTsType,
    JsArrowFunctionExpression, JsConstructorClassMember, JsFunctionBody, JsFunctionDeclaration,
    JsFunctionExpression, JsGetterClassMember, JsGetterObjectMember, JsIdentifierExpression,
    JsLogicalOperator, JsMethodClassMember, JsMethodObjectMember, JsReturnStatement,
    JsSetterClassMember, JsSetterObjectMember, JsSyntaxNode, JsVariableStatement, TsAsExpression,
    TsDeclareFunctionDeclaration, TsMethodSignatureClassMember, TsTypeAssertionExpression,
};
use biome_js_type_info::{InferredType, ReturnTypeEvidence};
use biome_rowan::{AstNode, TextRange, declare_node_union};
use biome_rule_options::no_misleading_return_type::NoMisleadingReturnTypeOptions;
use smallvec::SmallVec;
use std::iter::FusedIterator;

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
    ///
    /// ## Known limitations
    ///
    /// - Suggested replacement types are only shown when their textual
    ///   representation is up to 80 characters long. Longer unions fall back to
    ///   a generic note without the specific suggestion.
    /// - When a return uses a type assertion such as `as T`, the rule does
    ///   not flag the return unless it can prove that `T` is narrower than
    ///   `object`. Trusted cases include `unknown`, `any`, `typeof` queries,
    ///   conditional types, generic type parameters, and types the rule
    ///   cannot resolve. Intersections (`A & B`) are trusted when every
    ///   member is or when any member is `any`; unions (`A | B`) when at
    ///   least one is.
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
    suggestion: Option<String>,
}

/// Maximum iterations for expression traversal to guard against infinite loops.
const MAX_EXPRESSION_TRAVERSAL_ITERATIONS: usize = 200;

impl Display for RuleState {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> io::Result<()> {
        if let Some(suggestion) = &self.suggestion {
            formatter.write_str(suggestion)
        } else {
            Ok(())
        }
    }
}

impl Rule for NoMisleadingReturnType {
    type Query = Typed<AnyFunctionLikeWithReturnType>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = NoMisleadingReturnTypeOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
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
                let return_type =
                    ctx.inferred_return_type_of_member(method.syntax(), name.text())?;
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
                let return_type =
                    ctx.inferred_return_type_of_member(method.syntax(), name.text())?;
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
                let return_type =
                    ctx.inferred_return_type_of_member(getter.syntax(), name.text())?;
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
                let return_type =
                    ctx.inferred_return_type_of_member(getter.syntax(), name.text())?;
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

        let diag = if state.suggestion.is_some() {
            diag.note(markup! {
                "Consider using "{state}" as the return type."
            })
        } else {
            diag.note(markup! {
                "Narrow the return type to match what the function actually returns."
            })
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

fn run_for_member_with_body<'db>(
    ctx: &'db RuleContext<NoMisleadingReturnType>,
    annotation_range: TextRange,
    return_type: InferredType<'db>,
    is_async: bool,
    prefer_inferred_suggestion: bool,
    body: &AnyJsFunctionBody,
) -> Option<RuleState> {
    let info = collect_return_info(ctx, body);
    let analysis = return_type.check_misleading_return_type(
        &info.types,
        ReturnTypeEvidence {
            has_any_const: info.has_any_const,
            object_wide_casts: info.object_wide_casts,
            has_narrower_than_object: info.has_narrower_than_object,
            has_pinning_assertion: info.has_pinning_assertion,
            prefer_inferred_suggestion,
        },
        is_async,
    )?;
    Some(RuleState {
        annotation_range,
        suggestion: analysis.suggestion,
    })
}

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
struct ReturnInfo<'db> {
    types: Vec<InferredType<'db>>,
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

/// Walks the function body and populates a [`ReturnInfo`].
fn collect_return_info<'db>(
    ctx: &'db RuleContext<NoMisleadingReturnType>,
    body: &AnyJsFunctionBody,
) -> ReturnInfo<'db> {
    let mut info = ReturnInfo::default();

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
            if let Some(ty) = infer_expression_type(ctx, expr) {
                info.types.push(ty);
            }
        }
    }
    info
}

fn collect_block_returns<'db>(
    ctx: &'db RuleContext<NoMisleadingReturnType>,
    block: &JsFunctionBody,
    info: &mut ReturnInfo<'db>,
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
            } else {
                if has_object_wide_assertion(ctx, &expr) {
                    info.object_wide_casts += 1;
                } else if has_narrow_cast(ctx, &expr) || expression_reveals_narrow_object(&expr) {
                    info.has_narrower_than_object = true;
                }
            }
            info.has_pinning_assertion |= is_pinned_by_assertion(&expr);
            if let Some(ty) = infer_expression_type(ctx, &expr) {
                info.types.push(ty);
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

fn has_const_assertion(expr: &AnyJsExpression) -> bool {
    let mut current = expr.clone();
    loop {
        match &current {
            AnyJsExpression::TsAsExpression(e) => return is_const_type_assertion(e),
            AnyJsExpression::TsTypeAssertionExpression(e) => {
                return is_const_angle_bracket_assertion(e);
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
            AnyJsExpression::TsAsExpression(e) => return is_const_type_assertion(e),
            AnyJsExpression::TsTypeAssertionExpression(e) => {
                return is_const_angle_bracket_assertion(e);
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

fn is_nested_function_like(node: &JsSyntaxNode) -> bool {
    AnyNestedFunctionLike::can_cast(node.kind())
}
