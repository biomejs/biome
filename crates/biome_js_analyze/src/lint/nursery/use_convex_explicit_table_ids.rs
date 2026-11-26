use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, JsCallExpression, JsStaticMemberExpression,
};
use biome_rowan::{AstNode, AstSeparatedList};
use biome_rule_options::use_convex_explicit_table_ids::UseConvexExplicitTableIdsOptions;

use crate::services::manifest::Manifest;

declare_lint_rule! {
    /// Require the table name as the first argument of Convex database calls.
    ///
    /// Since `convex` 1.31.0, `db.get()`, `db.patch()`, `db.replace()`, and `db.delete()` accept
    /// the table name before the document ID. Passing it stops an ID with the wrong type from
    /// touching a document in another table, and Convex plans to deprecate calls without it.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const message = await ctx.db.get(messageId);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// await ctx.db.patch(messageId, { text: "updated" });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// await ctx.db.replace(messageId, { text: "replaced", author: "Alice" });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// await ctx.db.delete(messageId);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const file = await ctx.db.system.get(storageId);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const message = await ctx.db.get("messages", messageId);
    /// await ctx.db.patch("messages", messageId, { text: "updated" });
    /// await ctx.db.replace("messages", messageId, { text: "replaced", author: "Alice" });
    /// await ctx.db.delete("messages", messageId);
    /// const file = await ctx.db.system.get("_storage", storageId);
    /// ```
    ///
    pub UseConvexExplicitTableIds {
        version: "next",
        name: "useConvexExplicitTableIds",
        language: "js",
        sources: &[RuleSource::EslintConvex("explicit-table-ids").inspired()],
        recommended: true,
        severity: Severity::Warning,
        domains: &[RuleDomain::Convex],
    }
}

impl Rule for UseConvexExplicitTableIds {
    type Query = Manifest<JsCallExpression>;
    type State = DatabaseCall;
    type Signals = Option<Self::State>;
    type Options = UseConvexExplicitTableIdsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        let callee = call.callee().ok()?.omit_type_wrappers()?;
        let callee = callee.as_js_static_member_expression()?;
        let member = callee.member().ok()?;
        let method = match member.as_js_name()?.value_token().ok()?.text_trimmed() {
            "get" => DatabaseMethod::Get,
            "patch" => DatabaseMethod::Patch,
            "replace" => DatabaseMethod::Replace,
            "delete" => DatabaseMethod::Delete,
            _ => return None,
        };
        let object = DatabaseObject::from_expression(callee.object().ok()?)?;
        // `db.system` only reads documents.
        if object == DatabaseObject::System && method != DatabaseMethod::Get {
            return None;
        }

        let arguments = call.arguments().ok()?.args();
        // A spread argument can hold any number of values, including the table name.
        if arguments.len() != method.argument_count_without_table_name()
            || arguments
                .iter()
                .any(|argument| matches!(argument, Ok(AnyJsCallArgument::JsSpread(_))))
        {
            return None;
        }

        // A `convex` version range that includes 1.31.0 or later can install a version that
        // accepts the table name, so only a range entirely below it turns the rule off.
        if ctx
            .manifest
            .as_deref()
            .is_some_and(|manifest| manifest.dependency_excludes_range("convex", ">=1.31.0"))
        {
            return None;
        }

        Some(DatabaseCall { object, method })
    }

    fn diagnostic(ctx: &RuleContext<Self>, call: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "This "<Emphasis>{call.object}"."{call.method}"()"</Emphasis>" call doesn't include the table name."
                },
            )
            .note(markup! {
                "Without the table name, an ID with the wrong type can read or change a document in a different table than the one you expect. Convex also plans to deprecate calls without a table name."
            })
            .note(markup! {
                "Pass the name of the table as the first argument, for example "<Emphasis>{CallExample(*call)}</Emphasis>"."
            }),
        )
    }
}

/// A Convex database call that leaves out the table name.
#[derive(Clone, Copy)]
pub struct DatabaseCall {
    object: DatabaseObject,
    method: DatabaseMethod,
}

/// The Convex database methods that accept an optional table name as their first argument.
#[derive(Clone, Copy, Eq, PartialEq)]
enum DatabaseMethod {
    Get,
    Patch,
    Replace,
    Delete,
}

impl DatabaseMethod {
    /// Returns the number of arguments that the method takes when the call leaves out the
    /// table name.
    const fn argument_count_without_table_name(self) -> usize {
        match self {
            Self::Get | Self::Delete => 1,
            Self::Patch | Self::Replace => 2,
        }
    }
}

impl biome_console::fmt::Display for DatabaseMethod {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::Get => "get",
            Self::Patch => "patch",
            Self::Replace => "replace",
            Self::Delete => "delete",
        })
    }
}

/// The Convex database object that a method is called on.
#[derive(Clone, Copy, Eq, PartialEq)]
enum DatabaseObject {
    /// `<object>.db`, which accesses the tables of the app.
    Db,
    /// `<object>.db.system`, which reads the system tables, such as `_storage`.
    System,
}

impl DatabaseObject {
    /// Returns the database object for an expression of the form `<object>.db` or
    /// `<object>.db.system`, ignoring parentheses and TypeScript type wrappers such as `as` and
    /// `satisfies`.
    fn from_expression(expression: AnyJsExpression) -> Option<Self> {
        let expression = expression.omit_type_wrappers()?;
        let member = expression.as_js_static_member_expression()?;
        if has_member_name(member, "db") {
            return Some(Self::Db);
        }
        if !has_member_name(member, "system") {
            return None;
        }
        let object = member.object().ok()?.omit_type_wrappers()?;
        has_member_name(object.as_js_static_member_expression()?, "db").then_some(Self::System)
    }
}

impl biome_console::fmt::Display for DatabaseObject {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::Db => "db",
            Self::System => "db.system",
        })
    }
}

/// Writes an example of the call that includes the table name, such as
/// `db.get("messages", id)`.
struct CallExample(DatabaseCall);

impl biome_console::fmt::Display for CallExample {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        let DatabaseCall { object, method } = self.0;
        biome_console::fmt::Display::fmt(&object, fmt)?;
        fmt.write_str(".")?;
        biome_console::fmt::Display::fmt(&method, fmt)?;
        fmt.write_str(match object {
            DatabaseObject::Db => "(\"messages\", id",
            DatabaseObject::System => "(\"_storage\", id",
        })?;
        fmt.write_str(match method {
            DatabaseMethod::Get | DatabaseMethod::Delete => ")",
            DatabaseMethod::Patch | DatabaseMethod::Replace => ", value)",
        })
    }
}

/// Returns `true` when the property accessed by `member` is `name`.
fn has_member_name(member: &JsStaticMemberExpression, name: &str) -> bool {
    member.member().is_ok_and(|member| {
        member
            .as_js_name()
            .and_then(|member| member.value_token().ok())
            .is_some_and(|token| token.text_trimmed() == name)
    })
}
