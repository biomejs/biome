use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsExpression, AnyJsMemberExpression, JsCallExpression, JsReferenceIdentifier,
    binding_ext::AnyJsBindingDeclaration,
};
use biome_rowan::AstNode;
use biome_rule_options::no_convex_query_filter::NoConvexQueryFilterOptions;

use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow calling `.filter()` on a Convex database query.
    ///
    /// When a Convex query uses `.filter()`, the database still reads every document in the
    /// table (or in the range selected by an index), and only then drops the ones that don't
    /// match. As the table grows, the query gets slower and uses more of your database bandwidth.
    ///
    /// Use `.withIndex()` with an index on the fields you want to match, so the database only
    /// reads the matching documents. When an index isn't worth adding, collect the results and
    /// filter them in your own code: this reads the same documents as `.filter()`, but makes the
    /// cost visible in the code.
    ///
    /// The rule recognizes queries that start with `ctx.db.query()`, `db.query()`, or
    /// `db.privateSystem.query()`, including queries stored in a `const` variable.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const messages = await ctx.db
    ///     .query("messages")
    ///     .filter((q) => q.eq(q.field("author"), args.author))
    ///     .collect();
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const query = ctx.db.query("messages");
    /// const messages = await query.filter((q) => q.eq(q.field("author"), args.author)).collect();
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const messages = await ctx.db
    ///     .query("messages")
    ///     .withIndex("by_author", (q) => q.eq("author", args.author))
    ///     .collect();
    ///
    /// const allMessages = await ctx.db.query("messages").collect();
    /// const authorMessages = allMessages.filter((message) => message.author === args.author);
    /// ```
    ///
    pub NoConvexQueryFilter {
        version: "next",
        name: "noConvexQueryFilter",
        language: "js",
        sources: &[RuleSource::EslintConvex("no-filter-in-query").same()],
        recommended: true,
        severity: Severity::Warning,
        domains: &[RuleDomain::Convex],
    }
}

impl Rule for NoConvexQueryFilter {
    type Query = Semantic<JsCallExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoConvexQueryFilterOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let callee = filter_callee(ctx.query())?;
        let receiver = callee.object().ok()?;
        is_convex_query(receiver, ctx.model()).then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let member_name = filter_callee(ctx.query())?.member_name()?;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                member_name.range(),
                markup! {
                    "Avoid calling "<Emphasis>".filter()"</Emphasis>" on a Convex database query."
                },
            )
            .note(markup! {
                "The database reads every document in the query before "<Emphasis>".filter()"</Emphasis>" drops the ones that don't match, so the query gets slower as the table grows."
            })
            .note(markup! {
                "Use "<Emphasis>".withIndex()"</Emphasis>" to read only the matching documents, or collect the results and filter them in your code."
            }),
        )
    }
}

/// Returns the callee of `call` when the call is `<object>.filter(...)`.
fn filter_callee(call: &JsCallExpression) -> Option<AnyJsMemberExpression> {
    let callee = AnyJsMemberExpression::cast(call.callee().ok()?.into_syntax())?;
    (callee.member_name()?.text() == "filter").then_some(callee)
}

/// Returns whether `expression` evaluates to a Convex query builder, such as
/// `ctx.db.query("table")` or `db.query("table").withIndex(...).order("desc")`.
///
/// A reference to a `const` variable is followed to its initializer when the declaration comes
/// before the reference. Each step moves to an earlier position in the file, so the walk
/// terminates even when declarations refer to each other.
///
/// Returns `false` when the chain ends with a method that runs the query, such as `.collect()`,
/// because those return documents instead of a query builder.
fn is_convex_query(expression: AnyJsExpression, model: &SemanticModel) -> bool {
    let mut current = expression;
    loop {
        let Some(expression) = current.omit_type_wrappers() else {
            return false;
        };
        current = match expression {
            AnyJsExpression::JsCallExpression(call) => {
                let Some(callee) = call
                    .callee()
                    .ok()
                    .and_then(|callee| AnyJsMemberExpression::cast(callee.into_syntax()))
                else {
                    return false;
                };
                let Some(method) = callee.member_name() else {
                    return false;
                };
                let Ok(object) = callee.object() else {
                    return false;
                };
                match method.text() {
                    "query" => return is_database(object),
                    "collect" | "first" | "paginate" | "take" | "unique" => return false,
                    _ => object,
                }
            }
            AnyJsExpression::JsIdentifierExpression(identifier) => {
                let Some(initializer) = identifier
                    .name()
                    .ok()
                    .and_then(|reference| const_initializer(&reference, model))
                else {
                    return false;
                };
                initializer
            }
            _ => return false,
        };
    }
}

/// Returns the initializer of the `const` variable that `reference` points to, when that
/// variable is declared without destructuring and before `reference`.
fn const_initializer(
    reference: &JsReferenceIdentifier,
    model: &SemanticModel,
) -> Option<AnyJsExpression> {
    let binding = model.binding(reference)?;
    let AnyJsBindingDeclaration::JsVariableDeclarator(declarator) = binding.tree().declaration()?
    else {
        return None;
    };
    if !declarator.declaration()?.is_const()
        || declarator.range().end() > reference.range().start()
    {
        return None;
    }
    declarator.initializer()?.expression().ok()
}

/// Returns whether `object` is the receiver of a database `.query()` call: `db`, `<object>.db`,
/// `db.privateSystem`, or `<object>.db.privateSystem`.
fn is_database(object: AnyJsExpression) -> bool {
    let Some(object) = object.omit_type_wrappers() else {
        return false;
    };
    if let Some(member) = AnyJsMemberExpression::cast_ref(object.syntax())
        && member
            .member_name()
            .is_some_and(|name| name.text() == "privateSystem")
    {
        return member
            .object()
            .ok()
            .and_then(AnyJsExpression::omit_type_wrappers)
            .is_some_and(|inner| is_db_name(&inner));
    }
    is_db_name(&object)
}

/// Returns whether `expression` is the identifier `db` or a member access ending in `.db`.
fn is_db_name(expression: &AnyJsExpression) -> bool {
    match expression {
        AnyJsExpression::JsIdentifierExpression(identifier) => identifier
            .name()
            .ok()
            .and_then(|name| name.value_token().ok())
            .is_some_and(|token| token.text_trimmed() == "db"),
        AnyJsExpression::JsStaticMemberExpression(_)
        | AnyJsExpression::JsComputedMemberExpression(_) => {
            AnyJsMemberExpression::cast_ref(expression.syntax())
                .and_then(|member| member.member_name())
                .is_some_and(|name| name.text() == "db")
        }
        _ => false,
    }
}
