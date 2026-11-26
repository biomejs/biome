use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsExpression, JsCallExpression, JsStaticMemberExpression, JsSyntaxToken};
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_convex_collect_in_query::NoConvexCollectInQueryOptions;

declare_lint_rule! {
    /// Disallow loading every result of a Convex database query with `.collect()`.
    ///
    /// As a table grows, `.collect()` returns more and more documents, until the query fails
    /// at the [Convex query limits](https://docs.convex.dev/production/state/limits).
    /// Use `.take()` or `.paginate()` instead.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const messages = await ctx.db.query("messages").collect();
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const messages = await ctx.db
    ///     .query("messages")
    ///     .withIndex("by_channel", (q) => q.eq("channel", channel))
    ///     .order("desc")
    ///     .collect();
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const latest = await ctx.db.query("messages").order("desc").take(10);
    /// const page = await ctx.db.query("messages").paginate(paginationOpts);
    /// const first = await ctx.db.query("messages").first();
    /// ```
    ///
    pub NoConvexCollectInQuery {
        version: "next",
        name: "noConvexCollectInQuery",
        language: "js",
        sources: &[RuleSource::EslintConvex("no-collect-in-query").same()],
        recommended: false,
        severity: Severity::Warning,
        domains: &[RuleDomain::Convex],
    }
}

impl Rule for NoConvexCollectInQuery {
    type Query = Ast<JsCallExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoConvexCollectInQueryOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let callee = ctx.query().callee().ok()?.omit_parentheses();
        let member = callee.as_js_static_member_expression()?;
        if member_name(member)?.text_trimmed() != "collect" {
            return None;
        }
        is_database_query_chain(member.object().ok()?).then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let call = ctx.query();
        let callee = call.callee().ok()?.omit_parentheses();
        let member = callee.as_js_static_member_expression()?.member().ok()?;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                TextRange::new(member.range().start(), call.range().end()),
                markup! {
                    "This database query loads every matching document with "<Emphasis>".collect()"</Emphasis>"."
                },
            )
            .note(markup! {
                "A query that returns many documents uses a lot of bandwidth, makes mutations more likely to conflict, and fails once it reaches the Convex query limits."
            })
            .note(markup! {
                "Use "<Emphasis>".take()"</Emphasis>" to load only the first few documents, or "<Emphasis>".paginate()"</Emphasis>" to load them one page at a time. If this query always returns a small number of documents, suppress this diagnostic with a "<Emphasis>"biome-ignore"</Emphasis>" comment."
            }),
        )
    }
}

/// Returns `true` when `expression` is a chain of method calls that starts with a call to
/// `query()` on a Convex database, such as `ctx.db.query("messages").withIndex(...)`.
///
/// The chain stops matching at a method that runs the query, such as `.first()`, because its
/// result is a document or a list of documents rather than a query.
fn is_database_query_chain(expression: AnyJsExpression) -> bool {
    let mut expression = expression;
    loop {
        let Some(member) = omit_wrappers(expression)
            .as_ref()
            .and_then(AnyJsExpression::as_js_call_expression)
            .and_then(|call| call.callee().ok())
            .and_then(|callee| callee.omit_parentheses().as_js_static_member_expression().cloned())
        else {
            return false;
        };
        let Some(name) = member_name(&member) else {
            return false;
        };
        match name.text_trimmed() {
            "query" => return member.object().is_ok_and(is_database),
            "collect" | "take" | "first" | "unique" | "paginate" => return false,
            _ => match member.object() {
                Ok(object) => expression = object,
                Err(_) => return false,
            },
        }
    }
}

/// Returns `true` for an expression that looks like a Convex database: `db`, `ctx.db`,
/// `db.privateSystem`, or `ctx.db.privateSystem`.
fn is_database(expression: AnyJsExpression) -> bool {
    let Some(expression) = omit_wrappers(expression) else {
        return false;
    };
    if is_named_db(&expression) {
        return true;
    }
    expression
        .as_js_static_member_expression()
        .filter(|member| member_name(member).is_some_and(|name| name.text_trimmed() == "privateSystem"))
        .and_then(|member| member.object().ok())
        .and_then(omit_wrappers)
        .is_some_and(|object| is_named_db(&object))
}

/// Returns `true` for the identifier `db` or a property access that ends with `.db`.
fn is_named_db(expression: &AnyJsExpression) -> bool {
    match expression {
        AnyJsExpression::JsIdentifierExpression(identifier) => identifier
            .name()
            .and_then(|name| name.value_token())
            .is_ok_and(|token| token.text_trimmed() == "db"),
        AnyJsExpression::JsStaticMemberExpression(member) => {
            member_name(member).is_some_and(|name| name.text_trimmed() == "db")
        }
        _ => false,
    }
}

/// Returns the name token of a non-private property access, such as `query` in `db.query`.
fn member_name(member: &JsStaticMemberExpression) -> Option<JsSyntaxToken> {
    member.member().ok()?.as_js_name()?.value_token().ok()
}

/// Removes parentheses, TypeScript type wrappers such as `as` and `!`, and `await` from
/// `expression`, because they don't change which object a method is called on.
fn omit_wrappers(expression: AnyJsExpression) -> Option<AnyJsExpression> {
    let mut expression = expression;
    loop {
        expression = match expression.omit_type_wrappers()? {
            AnyJsExpression::JsAwaitExpression(await_expression) => {
                await_expression.argument().ok()?
            }
            expression => return Some(expression),
        };
    }
}
