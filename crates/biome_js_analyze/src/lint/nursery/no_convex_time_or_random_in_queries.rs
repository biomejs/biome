use std::ops::ControlFlow;

use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyFunctionLike, AnyJsCallArgument, AnyJsExpression, AnyJsFunction, AnyJsMemberExpression,
    AnyJsObjectMember, JsCallExpression, JsGetterClassMember, JsGetterObjectMember, JsLanguage,
    JsNewOrCallExpression, JsReferenceIdentifier, JsSetterClassMember, JsSetterObjectMember, JsSyntaxKind, JsSyntaxNode,
    binding_ext::AnyJsBindingDeclaration, global_identifier, static_value::StaticValue,
};
use biome_rowan::{AstNode, AstSeparatedList, SyntaxKindSet, WalkEvent};
use biome_rule_options::no_convex_time_or_random_in_queries::NoConvexTimeOrRandomInQueriesOptions;
use rustc_hash::FxHashSet;

use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow reading the current time or generating random values in Convex queries.
    ///
    /// Convex caches query results, so a query that reads the current time can return an
    /// outdated time, and a query that generates a random value keeps returning the same one.
    ///
    /// This rule checks `query()` and `internalQuery()` handlers, including the callbacks they
    /// pass and the functions in the same file they call, for calls such as `Date.now()`,
    /// `new Date()`, and `Math.random()`.
    ///
    /// Pass the time to the query as an argument instead, and generate random values in a
    /// mutation or an action. See the
    /// [Convex best practices](https://docs.convex.dev/understanding/best-practices/#date-in-queries).
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// export const listRecentPosts = query({
    ///     args: {},
    ///     handler: async (ctx) => {
    ///         const since = Date.now() - 24 * 60 * 60 * 1000;
    ///         return await ctx.db
    ///             .query("posts")
    ///             .withIndex("by_creation_time", (q) => q.gt("_creationTime", since))
    ///             .collect();
    ///     },
    /// });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// export const pickWinner = query({
    ///     args: {},
    ///     handler: async (ctx) => {
    ///         const entries = await ctx.db.query("entries").collect();
    ///         return entries[Math.floor(Math.random() * entries.length)];
    ///     },
    /// });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function isExpired(session) {
    ///     return session.expiresAt < Date.now();
    /// }
    ///
    /// export const getSession = query({
    ///     args: {},
    ///     handler: async (ctx, args) => {
    ///         const session = await ctx.db.get(args.sessionId);
    ///         return isExpired(session) ? null : session;
    ///     },
    /// });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// export const listRecentPosts = query({
    ///     args: { since: v.number() },
    ///     handler: async (ctx, args) => {
    ///         return await ctx.db
    ///             .query("posts")
    ///             .withIndex("by_creation_time", (q) => q.gt("_creationTime", args.since))
    ///             .collect();
    ///     },
    /// });
    ///
    /// export const formatDate = query({
    ///     args: { timestamp: v.number() },
    ///     handler: async (ctx, args) => new Date(args.timestamp).toISOString(),
    /// });
    ///
    /// export const createPost = mutation({
    ///     args: { body: v.string() },
    ///     handler: async (ctx, args) => {
    ///         await ctx.db.insert("posts", { body: args.body, publishedAt: Date.now() });
    ///     },
    /// });
    /// ```
    ///
    pub NoConvexTimeOrRandomInQueries {
        version: "next",
        name: "noConvexTimeOrRandomInQueries",
        language: "js",
        sources: &[RuleSource::EslintConvex("no-time-or-random-in-queries").same()],
        recommended: true,
        severity: Severity::Warning,
        domains: &[RuleDomain::Convex],
    }
}

impl Rule for NoConvexTimeOrRandomInQueries {
    type Query = Semantic<JsCallExpression>;
    type State = NondeterministicCall;
    type Signals = Box<[Self::State]>;
    type Options = NoConvexTimeOrRandomInQueriesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let model = ctx.model();
        let Some((handler, reference)) = query_handler(ctx.query(), model) else {
            return Box::default();
        };
        // A handler shared by several registrations is only checked for the first one, so
        // each call in it is reported once.
        if reference.is_some_and(|reference| is_registered_earlier(&reference, &handler, model)) {
            return Box::default();
        }
        let mut found = Vec::new();
        for_each_call(&handler, |call| {
            if let Some(source) = NondeterministicSource::from_call(&call, model) {
                found.push(NondeterministicCall {
                    read: call.clone(),
                    call,
                    source,
                });
                return ControlFlow::<()>::Continue(());
            }
            let read = called_functions(&call, model)
                .find_map(|function| first_read(function, &handler, model));
            if let Some((read, source)) = read {
                found.push(NondeterministicCall { call, read, source });
            }
            ControlFlow::Continue(())
        });
        found.into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let NondeterministicCall { call, read, source } = state;
        let is_direct = call == read;
        let action = match (source.is_random(), is_direct) {
            (false, true) => "reads the current time",
            (false, false) => "calls a function that reads the current time",
            (true, true) => "generates a random value",
            (true, false) => "calls a function that generates a random value",
        };
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            call.range(),
            markup! {
                "This query "{action}" with "<Emphasis>{source}</Emphasis>"."
            },
        );
        let diagnostic = if is_direct {
            diagnostic
        } else {
            diagnostic.detail(
                read.range(),
                markup! { <Emphasis>{source}</Emphasis>" is called here." },
            )
        };
        Some(if source.is_random() {
            diagnostic
                .note(markup! {
                    "Convex caches the result of a query, so the query keeps returning the same value until the data it reads changes, instead of a new random value each time."
                })
                .note(markup! {
                    "Generate random values in a mutation or an action instead."
                })
        } else {
            diagnostic
                .note(markup! {
                    "Convex caches the result of a query and reuses it until the data the query reads changes, so the time in a cached result can be outdated. Reading the time can also make Convex discard cached results more often than needed."
                })
                .note(markup! {
                    "Pass the time to the query as an argument, rounded to a coarse unit such as a minute, or have a mutation or a scheduled function store time-based state in the database."
                })
        })
    }
}

/// A call in a query handler that reads the current time or generates a random value, either
/// directly or through a function it calls.
pub struct NondeterministicCall {
    /// The call in the handler that the diagnostic reports.
    call: JsNewOrCallExpression,
    /// The call that reads the time or generates a random value: `call` itself, or a call in a
    /// function that `call` runs.
    read: JsNewOrCallExpression,
    /// What `read` calls.
    source: NondeterministicSource,
}

/// A built-in that reads the current time or generates a random value.
#[derive(Clone, Copy)]
pub enum NondeterministicSource {
    DateCall,
    NewDate,
    DateNow,
    PerformanceNow,
    TemporalNowInstant,
    TemporalNowPlainDateIso,
    TemporalNowPlainDateTimeIso,
    TemporalNowPlainTimeIso,
    TemporalNowZonedDateTimeIso,
    MathRandom,
    CryptoRandomUuid,
    CryptoGetRandomValues,
}

impl NondeterministicSource {
    /// Returns the built-in that `call` calls, if it's one of the built-ins this rule reports
    /// and no local variable shadows it.
    fn from_call(call: &JsNewOrCallExpression, model: &SemanticModel) -> Option<Self> {
        let callee = call.callee().ok()?.omit_type_wrappers()?;
        if let JsNewOrCallExpression::JsNewExpression(new) = call {
            // `new Date(value)` creates a date from `value` instead of reading the clock.
            let has_arguments = new
                .arguments()
                .is_some_and(|arguments| !arguments.args().is_empty());
            return (!has_arguments && global_name(&callee, model)?.text() == "Date")
                .then_some(Self::NewDate);
        }
        if global_name(&callee, model).is_some_and(|name| name.text() == "Date") {
            return Some(Self::DateCall);
        }

        let member = AnyJsMemberExpression::cast(callee.into_syntax())?;
        let method = member.member_name()?;
        let object = member.object().ok()?.omit_type_wrappers()?;
        if let Some(object_name) = global_name(&object, model) {
            return match (object_name.text(), method.text()) {
                ("Date", "now") => Some(Self::DateNow),
                ("performance", "now") => Some(Self::PerformanceNow),
                ("Math", "random") => Some(Self::MathRandom),
                ("crypto", "randomUUID") => Some(Self::CryptoRandomUuid),
                ("crypto", "getRandomValues") => Some(Self::CryptoGetRandomValues),
                _ => None,
            };
        }

        // `Temporal.Now.instant()` and the other `Temporal.Now` methods.
        let namespace = AnyJsMemberExpression::cast(object.into_syntax())?;
        if namespace.member_name()?.text() != "Now"
            || global_name(&namespace.object().ok()?.omit_type_wrappers()?, model)?.text()
                != "Temporal"
        {
            return None;
        }
        match method.text() {
            "instant" => Some(Self::TemporalNowInstant),
            "plainDateISO" => Some(Self::TemporalNowPlainDateIso),
            "plainDateTimeISO" => Some(Self::TemporalNowPlainDateTimeIso),
            "plainTimeISO" => Some(Self::TemporalNowPlainTimeIso),
            "zonedDateTimeISO" => Some(Self::TemporalNowZonedDateTimeIso),
            _ => None,
        }
    }

    const fn is_random(self) -> bool {
        matches!(
            self,
            Self::MathRandom | Self::CryptoRandomUuid | Self::CryptoGetRandomValues
        )
    }
}

impl biome_console::fmt::Display for NondeterministicSource {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::DateCall => "Date()",
            Self::NewDate => "new Date()",
            Self::DateNow => "Date.now()",
            Self::PerformanceNow => "performance.now()",
            Self::TemporalNowInstant => "Temporal.Now.instant()",
            Self::TemporalNowPlainDateIso => "Temporal.Now.plainDateISO()",
            Self::TemporalNowPlainDateTimeIso => "Temporal.Now.plainDateTimeISO()",
            Self::TemporalNowPlainTimeIso => "Temporal.Now.plainTimeISO()",
            Self::TemporalNowZonedDateTimeIso => "Temporal.Now.zonedDateTimeISO()",
            Self::MathRandom => "Math.random()",
            Self::CryptoRandomUuid => "crypto.randomUUID()",
            Self::CryptoGetRandomValues => "crypto.getRandomValues()",
        })
    }
}

/// Returns the handler of the Convex query that `call` registers, as in
/// `query({ handler: async (ctx) => {} })` or `internalQuery(async (ctx) => {})`.
///
/// The handler can be written inline or be the name of a function declared in the same file.
/// In the second case, the name in `call` is also returned.
fn query_handler(
    call: &JsCallExpression,
    model: &SemanticModel,
) -> Option<(AnyFunctionLike, Option<JsReferenceIdentifier>)> {
    let callee = call.callee().ok()?.omit_type_wrappers()?;
    let callee = callee.as_js_reference_identifier()?.value_token().ok()?;
    if !matches!(callee.text_trimmed(), "query" | "internalQuery") {
        return None;
    }
    let arguments = call.arguments().ok()?.args();
    if arguments.len() != 1 {
        return None;
    }
    let AnyJsCallArgument::AnyJsExpression(argument) = arguments.first()?.ok()? else {
        return None;
    };
    let argument = argument.omit_type_wrappers()?;
    let AnyJsExpression::JsObjectExpression(object) = argument else {
        return function_value(&argument, model);
    };
    let handler = object
        .members()
        .iter()
        .flatten()
        .find(|member| member.name().is_some_and(|name| name.text() == "handler"))?;
    match handler {
        AnyJsObjectMember::JsMethodObjectMember(method) => Some((method.into(), None)),
        AnyJsObjectMember::JsPropertyObjectMember(property) => {
            function_value(&property.value().ok()?, model)
        }
        AnyJsObjectMember::JsShorthandPropertyObjectMember(shorthand) => {
            let reference = shorthand.name().ok()?;
            Some((local_function(&reference, model)?, Some(reference)))
        }
        _ => None,
    }
}

/// Returns the function that `expression` evaluates to, if it's written inline or is the
/// name of a function declared in the same file. In the second case, the name is also returned.
fn function_value(
    expression: &AnyJsExpression,
    model: &SemanticModel,
) -> Option<(AnyFunctionLike, Option<JsReferenceIdentifier>)> {
    let expression = expression.clone().omit_type_wrappers()?;
    if let Some(function) = AnyJsFunction::cast_ref(expression.syntax()) {
        return Some((function.into(), None));
    }
    let reference = expression.as_js_reference_identifier()?;
    Some((local_function(&reference, model)?, Some(reference)))
}

/// Returns `true` if a `query()` or `internalQuery()` call before `reference` also registers
/// `handler`, where `reference` is the name that a registration refers to `handler` by.
fn is_registered_earlier(
    reference: &JsReferenceIdentifier,
    handler: &AnyFunctionLike,
    model: &SemanticModel,
) -> bool {
    let Some(binding) = model.binding(reference) else {
        return false;
    };
    binding.all_references().any(|other| {
        other.range_start() < reference.range().start()
            && other
                .syntax()
                .ancestors()
                .find_map(JsCallExpression::cast)
                .and_then(|call| query_handler(&call, model))
                .is_some_and(|(other_handler, _)| &other_handler == handler)
    })
}

/// Returns the function declared in the same file that `reference` refers to, as in
/// `function helper() {}` or `const helper = () => {}`.
///
/// Returns `None` when something assigns another value to the variable, because the call
/// might then run a different function.
fn local_function(
    reference: &JsReferenceIdentifier,
    model: &SemanticModel,
) -> Option<AnyFunctionLike> {
    let binding = model.binding(reference)?;
    if binding.all_writes().next().is_some() {
        return None;
    }
    let function: AnyJsFunction = match binding.tree().declaration()? {
        AnyJsBindingDeclaration::JsFunctionDeclaration(function) => function.into(),
        AnyJsBindingDeclaration::JsFunctionExpression(function) => function.into(),
        AnyJsBindingDeclaration::JsFunctionExportDefaultDeclaration(function) => function.into(),
        AnyJsBindingDeclaration::JsVariableDeclarator(declarator) => {
            // Skip destructuring, as in `const { helper } = () => {}`.
            declarator.id().ok()?.as_any_js_binding()?;
            let initializer = declarator.initializer()?.expression().ok()?;
            AnyJsFunction::cast(initializer.omit_type_wrappers()?.into_syntax())?
        }
        _ => return None,
    };
    Some(function.into())
}

/// Returns the functions declared in the same file that `call` runs: its callee, as in
/// `helper()`, and the functions passed to it by name, as in `items.map(helper)`.
///
/// Functions written inline in `call` aren't returned, because [for_each_call] already visits
/// the calls inside them.
fn called_functions(
    call: &JsNewOrCallExpression,
    model: &SemanticModel,
) -> impl Iterator<Item = AnyFunctionLike> {
    let callee = match call {
        JsNewOrCallExpression::JsCallExpression(call) => call.callee().ok(),
        // Constructors aren't followed, but the callbacks passed to them are.
        JsNewOrCallExpression::JsNewExpression(_) => None,
    };
    let arguments = call
        .arguments()
        .into_iter()
        .flat_map(|arguments| arguments.args())
        .filter_map(|argument| match argument.ok()? {
            AnyJsCallArgument::AnyJsExpression(argument) => Some(argument),
            _ => None,
        });
    callee.into_iter().chain(arguments).filter_map(|expression| {
        local_function(
            &expression.omit_type_wrappers()?.as_js_reference_identifier()?,
            model,
        )
    })
}

/// Returns the first call that reads the time or generates a random value when `function`
/// runs, searching `function` and the functions it calls in the same file, along with the
/// built-in it calls.
///
/// `handler` isn't searched again when `function` calls it back, because the rule already
/// reports the calls in the handler itself.
fn first_read(
    function: AnyFunctionLike,
    handler: &AnyFunctionLike,
    model: &SemanticModel,
) -> Option<(JsNewOrCallExpression, NondeterministicSource)> {
    let mut visited = FxHashSet::default();
    visited.insert(handler.clone());
    let mut pending = vec![function];
    while let Some(function) = pending.pop() {
        if !visited.insert(function.clone()) {
            continue;
        }
        let found = for_each_call(&function, |call| {
            if let Some(source) = NondeterministicSource::from_call(&call, model) {
                return ControlFlow::Break((call, source));
            }
            pending.extend(called_functions(&call, model));
            ControlFlow::Continue(())
        });
        if found.is_some() {
            return found;
        }
    }
    None
}

/// Calls `visit` with each call and `new` expression that runs when `function` runs, until
/// `visit` breaks, and returns the value it breaks with.
///
/// Nested functions are skipped unless they run right away: callbacks passed to a call, as in
/// `items.map((item) => item.id)`, and functions called right after they're created, as in
/// `(() => {})()`.
fn for_each_call<T>(
    function: &AnyFunctionLike,
    mut visit: impl FnMut(JsNewOrCallExpression) -> ControlFlow<T>,
) -> Option<T> {
    let body = function.body().ok()?;
    let mut preorder = body.syntax().preorder_of_kinds(CALL_SEARCH_KINDS);
    while let Some(event) = preorder.next() {
        let WalkEvent::Enter(node) = event else {
            continue;
        };
        if let Some(call) = JsNewOrCallExpression::cast_ref(&node) {
            if let ControlFlow::Break(value) = visit(call) {
                return Some(value);
            }
        } else if !runs_right_away(&node) {
            preorder.skip_subtree();
        }
    }
    None
}

/// Returns `true` if the nested function `node` runs when the code around it runs: it's passed
/// to a call or `new` expression, or it's the callee of a call.
fn runs_right_away(node: &JsSyntaxNode) -> bool {
    let Some(outer) = AnyJsExpression::cast_ref(node).and_then(|function| function.outer_expression())
    else {
        return false;
    };
    let Some(parent) = outer.syntax().parent() else {
        return false;
    };
    parent.kind() == JsSyntaxKind::JS_CALL_ARGUMENT_LIST
        || JsCallExpression::cast(parent)
            .and_then(|call| call.callee().ok())
            .is_some_and(|callee| callee == outer)
}

/// Returns the name of the global that `expression` refers to, as in `Date` or
/// `globalThis.Date`, if no local variable shadows it.
fn global_name(expression: &AnyJsExpression, model: &SemanticModel) -> Option<StaticValue> {
    let (reference, name) = global_identifier(&expression.as_any_global_identifier_expression()?)?;
    model.binding(&reference).is_none().then_some(name)
}

/// The nodes [for_each_call] stops at: calls and `new` expressions, and the functions whose
/// calls might not run with the code around them.
const CALL_SEARCH_KINDS: SyntaxKindSet<JsLanguage> = JsNewOrCallExpression::KIND_SET
    .union(AnyFunctionLike::KIND_SET)
    .union(JsGetterObjectMember::KIND_SET)
    .union(JsSetterObjectMember::KIND_SET)
    .union(JsGetterClassMember::KIND_SET)
    .union(JsSetterClassMember::KIND_SET);
