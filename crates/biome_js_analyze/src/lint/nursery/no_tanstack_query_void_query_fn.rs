use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_control_flow::{ExceptionHandlerKind, InstructionKind, builder::ROOT_BLOCK_ID};
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsExpression, AnyJsFunction, AnyJsFunctionBody, JsComputedMemberExpression,
    JsExpressionStatement, JsFunctionBody, JsIdentifierExpression, JsLanguage,
    JsMethodObjectMember, JsPropertyObjectMember, JsReturnStatement,
    JsShorthandPropertyObjectMember, JsStaticMemberExpression, JsSwitchStatement,
};
use biome_js_type_info::InferredType;
use biome_rowan::{AstNode, NodeOrToken, SyntaxKindSet, TextRange, declare_node_union};
use biome_rule_options::no_tanstack_query_void_query_fn::NoTanstackQueryVoidQueryFnOptions;
use roaring::RoaringBitmap;

use crate::services::control_flow::{AnyJsControlFlowRoot, control_flow_graph};
use crate::services::typed::Typed;

declare_lint_rule! {
    /// Disallow TanStack Query functions that don't return any data.
    ///
    /// A query function is the function that you give to TanStack Query as `queryFn`, for
    /// example in the options of `useQuery`. TanStack Query saves the value that this function
    /// returns as the data of the query. If the function returns a promise, TanStack Query waits
    /// for the promise and saves its result instead.
    ///
    /// TanStack Query doesn't accept `undefined` as data. When a query function returns
    /// `undefined`, the query fails with an error. This usually happens because a `return`
    /// statement is missing, or because the function returns a value in some cases but not in
    /// others.
    ///
    /// This rule checks every object property named `queryFn`. It reports the property when the
    /// function can return `undefined`, or a promise whose result is `undefined`.
    ///
    /// When the function is written in the property itself, the rule checks every `return`
    /// statement and whether the end of the function can be reached. When the property refers to
    /// a function that is defined somewhere else, the rule only reports it if the function never
    /// returns any data.
    ///
    /// If a query has no data to return, return `null` instead.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic,file=invalid-missing-return.js
    /// import { useQuery } from "@tanstack/react-query";
    ///
    /// useQuery({
    ///     queryKey: ["todos"],
    ///     queryFn: async () => {
    ///         await fetch("/api/todos");
    ///     },
    /// });
    /// ```
    ///
    /// ```js,expect_diagnostic,file=invalid-missing-return-in-one-path.js
    /// import { useQuery } from "@tanstack/react-query";
    ///
    /// useQuery({
    ///     queryKey: ["todos"],
    ///     queryFn: async () => {
    ///         const response = await fetch("/api/todos");
    ///         if (response.ok) {
    ///             return response.json();
    ///         }
    ///     },
    /// });
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=invalid-void-function.ts
    /// import { queryOptions } from "@tanstack/react-query";
    ///
    /// async function refreshTodos(): Promise<void> {
    ///     await fetch("/api/todos/refresh");
    /// }
    ///
    /// const options = queryOptions({
    ///     queryKey: ["todos"],
    ///     queryFn: refreshTodos,
    /// });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts,file=valid.ts
    /// import { queryOptions, useQuery } from "@tanstack/react-query";
    ///
    /// useQuery({
    ///     queryKey: ["todos"],
    ///     queryFn: async () => {
    ///         const response = await fetch("/api/todos");
    ///         return response.json();
    ///     },
    /// });
    ///
    /// useQuery({
    ///     queryKey: ["selected-todo"],
    ///     queryFn: () => null,
    /// });
    ///
    /// async function countTodos(): Promise<number> {
    ///     return 3;
    /// }
    ///
    /// const options = queryOptions({
    ///     queryKey: ["todo-count"],
    ///     queryFn: countTodos,
    /// });
    /// ```
    ///
    pub NoTanstackQueryVoidQueryFn {
        version: "2.6.5",
        name: "noTanstackQueryVoidQueryFn",
        language: "js",
        recommended: true,
        sources: &[RuleSource::EslintTanstackQuery("no-void-query-fn").same()],
        severity: Severity::Error,
        domains: &[RuleDomain::TanstackQuery, RuleDomain::Types],
    }
}

declare_node_union! {
    pub AnyQueryFnMember = JsPropertyObjectMember | JsMethodObjectMember | JsShorthandPropertyObjectMember
}

impl Rule for NoTanstackQueryVoidQueryFn {
    type Query = Typed<AnyQueryFnMember>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoTanstackQueryVoidQueryFnOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let member = ctx.query();
        member.name_range_if_query_fn()?;

        let gives_undefined = match member {
            AnyQueryFnMember::JsPropertyObjectMember(property) => {
                let value = property.value().ok()?.omit_parentheses();
                if let Some(function) = AnyJsFunction::cast_ref(value.syntax()) {
                    if function.is_generator() {
                        return None;
                    }
                    if function.return_type_annotation().is_some() {
                        let return_type = ctx
                            .return_type_of_function(&function)
                            .or_else(|| ctx.type_of_expression(&value)?.callable_return_type())?;
                        results_in_undefined(return_type)
                    } else {
                        match function.body().ok()? {
                            AnyJsFunctionBody::AnyJsExpression(expression) => {
                                value_results_in_undefined(ctx, &expression)
                            }
                            AnyJsFunctionBody::JsFunctionBody(body) => {
                                body_gives_undefined(ctx, &function.clone().into(), &body)?
                            }
                        }
                    }
                } else {
                    let return_type = ctx.type_of_expression(&value)?.callable_return_type()?;
                    always_results_in_undefined(return_type)
                }
            }
            AnyQueryFnMember::JsMethodObjectMember(method) => {
                if method.star_token().is_some() {
                    return None;
                }
                if method.return_type_annotation().is_some() {
                    results_in_undefined(ctx.return_type_of_member(method.syntax(), QUERY_FN)?)
                } else {
                    body_gives_undefined(ctx, &method.clone().into(), &method.body().ok()?)?
                }
            }
            AnyQueryFnMember::JsShorthandPropertyObjectMember(shorthand) => {
                let return_type = ctx
                    .type_of_named_value(shorthand.range(), QUERY_FN)?
                    .callable_return_type()?;
                always_results_in_undefined(return_type)
            }
        };

        gives_undefined.then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let range = ctx.query().name_range_if_query_fn()?;

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This query function can return "<Emphasis>"undefined"</Emphasis>"."
                },
            )
            .note(markup! {
                "TanStack Query doesn't accept "<Emphasis>"undefined"</Emphasis>" as the data of a query. The query fails with an error instead of saving a result."
            })
            .note(markup! {
                "Return the data in every case. If there is no data to return, return "<Emphasis>"null"</Emphasis>" instead."
            }),
        )
    }
}

/// The name of the option that holds the query function.
const QUERY_FN: &str = "queryFn";

impl AnyQueryFnMember {
    /// Returns the range of the member's name if the member is named `queryFn`.
    ///
    /// A quoted name such as `"queryFn"` counts too. A computed name such as `[key]` doesn't.
    fn name_range_if_query_fn(&self) -> Option<TextRange> {
        let name = match self {
            Self::JsPropertyObjectMember(property) => property.name().ok()?,
            Self::JsMethodObjectMember(method) => method.name().ok()?,
            Self::JsShorthandPropertyObjectMember(shorthand) => {
                let name = shorthand.name().ok()?;
                let is_query_fn = name.value_token().ok()?.text_trimmed() == QUERY_FN;
                return is_query_fn.then(|| name.range());
            }
        };
        let literal_name = name.as_js_literal_member_name()?;

        (literal_name.name().ok()?.text() == QUERY_FN).then(|| literal_name.range())
    }
}

/// Expressions whose type can be narrowed by an earlier check, such as `if (!value) throw`.
///
/// The inferred type of these expressions is the declared type, which doesn't take such checks
/// into account.
const NARROWABLE_EXPRESSIONS: SyntaxKindSet<JsLanguage> = JsIdentifierExpression::KIND_SET
    .union(JsStaticMemberExpression::KIND_SET)
    .union(JsComputedMemberExpression::KIND_SET);

/// Returns whether a value of type `ty` can give TanStack Query `undefined` as its data.
///
/// TanStack Query awaits the value, so `Promise<void>` and `Promise<Data | undefined>` count as
/// well as `void` and `Data | undefined`. Returns `false` when the type can't be inferred
/// conclusively.
fn results_in_undefined(ty: InferredType) -> bool {
    ty.promise_inner_type()
        .unwrap_or(ty)
        .has_undefined_variant()
        == Some(true)
}

/// Returns whether a value of type `ty` always gives TanStack Query `undefined` as its data.
///
/// Unlike [`results_in_undefined`], a type that only includes `undefined`, such as
/// `Data | undefined`, doesn't count. This is the check for types that can be wider than the
/// actual value: the type of an expression that an earlier check can narrow, and the return
/// type of a function that is defined somewhere else, which can be inferred from such
/// expressions.
fn always_results_in_undefined(ty: InferredType) -> bool {
    ty.promise_inner_type().unwrap_or(ty).is_void_like() == Some(true)
}

/// Returns whether returning `value` from a query function can give TanStack Query `undefined`
/// as its data.
///
/// Both branches of a conditional expression are checked on their own. A branch that is one of
/// the [`NARROWABLE_EXPRESSIONS`] only counts when it is always `undefined`.
///
/// Returns `false` when the type of `value` can't be inferred conclusively.
///
/// ```ts
/// declare const cached: Data | undefined;
/// declare function find(): Data | undefined;
///
/// // `true`
/// find()
/// cached ? cached : undefined
/// // `false`: an earlier check can rule out `undefined`.
/// cached
/// ```
fn value_results_in_undefined(
    ctx: &RuleContext<NoTanstackQueryVoidQueryFn>,
    value: &AnyJsExpression,
) -> bool {
    let value = value.clone().omit_parentheses();
    if let Some(conditional) = value.as_js_conditional_expression() {
        return [conditional.consequent(), conditional.alternate()]
            .into_iter()
            .flatten()
            .any(|branch| value_results_in_undefined(ctx, &branch));
    }

    let Some(ty) = ctx.type_of_expression(&value) else {
        return false;
    };
    if NARROWABLE_EXPRESSIONS.matches(value.syntax().kind()) {
        always_results_in_undefined(ty)
    } else {
        results_in_undefined(ty)
    }
}

/// Returns whether `body`, the body of `function`, can give TanStack Query `undefined` as its
/// data, judging from the statements that can be reached.
///
/// The body gives `undefined` when one of the following can be reached:
///
/// - A `return` statement without a value.
/// - A `return` statement whose value [results in `undefined`](value_results_in_undefined).
/// - The end of the body, unless [`ending_depends_on_types`] says that the types can make the
///   end unreachable.
///
/// A body that always throws gives nothing, so the result is `false`.
///
/// Returns `None` when `function` has no control flow graph because of a syntax error.
///
/// ```js
/// // `true`: nothing is returned when `ok` is false.
/// async () => { if (ok) return data; }
/// // `false`
/// async () => { throw new Error("Not implemented"); }
/// ```
fn body_gives_undefined(
    ctx: &RuleContext<NoTanstackQueryVoidQueryFn>,
    function: &AnyJsControlFlowRoot,
    body: &JsFunctionBody,
) -> Option<bool> {
    let graph = control_flow_graph(ctx, function)?;

    let mut reaches_end = false;
    let mut block_stack = vec![ROOT_BLOCK_ID];
    let mut visited_blocks = RoaringBitmap::new();
    visited_blocks.insert(ROOT_BLOCK_ID.index());

    while let Some(block_id) = block_stack.pop() {
        let block = graph.get(block_id);
        for handler in block.exception_handlers.iter() {
            // A `finally` clause is entered through a jump when its `try` block completes.
            // Following its handler as well would make the code after the `try` statement
            // look reachable when the `try` block always returns.
            if handler.kind == ExceptionHandlerKind::Catch
                && visited_blocks.insert(handler.target.index())
            {
                block_stack.push(handler.target);
            }
        }
        for instruction in block.instructions.iter() {
            match instruction.kind {
                InstructionKind::Statement => {}
                InstructionKind::Jump {
                    conditional,
                    block: target,
                    ..
                } => {
                    if visited_blocks.insert(target.index()) {
                        block_stack.push(target);
                    }
                    if !conditional {
                        break;
                    }
                }
                InstructionKind::Return => {
                    match &instruction.node {
                        Some(NodeOrToken::Node(node)) => {
                            // The other statement that ends a path is `throw`, which doesn't
                            // give the query any data.
                            if let Some(statement) = JsReturnStatement::cast_ref(node) {
                                let gives_undefined = statement
                                    .argument()
                                    .is_none_or(|value| value_results_in_undefined(ctx, &value));
                                if gives_undefined {
                                    return Some(true);
                                }
                            }
                        }
                        // The return at the end of the body has no node.
                        _ => reaches_end = true,
                    }
                    break;
                }
            }
        }
    }

    Some(reaches_end && !ending_depends_on_types(ctx, body))
}

/// Returns whether the types in `body` can make the end of the body unreachable, even though
/// the statements alone suggest that it is reachable.
///
/// That is the case when `body` contains one of the following outside of nested functions:
///
/// - A `switch` statement, because its cases can cover every value of the tested type.
/// - A statement that calls a function whose return type is `never`.
///
/// ```ts
/// declare const kind: "a" | "b";
/// declare function fail(message: string): never;
///
/// // The end of both bodies is unreachable.
/// () => { switch (kind) { case "a": return 1; case "b": return 2; } }
/// () => { if (kind === "a") return 1; fail("Unknown kind"); }
/// ```
fn ending_depends_on_types(
    ctx: &RuleContext<NoTanstackQueryVoidQueryFn>,
    body: &JsFunctionBody,
) -> bool {
    body.syntax()
        .pruned_descendents(|node| !AnyJsControlFlowRoot::can_cast(node.kind()))
        .any(|node| {
            if JsSwitchStatement::can_cast(node.kind()) {
                return true;
            }
            JsExpressionStatement::cast(node)
                .and_then(|statement| statement.expression().ok())
                .filter(|expression| expression.as_js_call_expression().is_some())
                .and_then(|call| ctx.type_of_expression(&call))
                .is_some_and(|ty| ty.is_never())
        })
}
