use biome_analyze::{Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsMemberExpression, JsCallArguments, JsCallExpression, JsReferenceIdentifier, JsSyntaxKind,
};
use biome_rowan::{AstNode, AstSeparatedList};
use biome_rule_options::no_drizzle_db_object_inside_transaction::NoDrizzleDbObjectInsideTransactionOptions;

use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow using a Drizzle ORM instance inside its own transaction callback.
    ///
    /// Inside `db.transaction(async (tx) => { ... })`, queries must use the transaction object `tx`.
    /// Queries made with `db` run outside the transaction: they aren't rolled back when the
    /// transaction fails, they don't see its uncommitted changes, and they can wait forever for
    /// rows that the transaction has locked.
    ///
    /// ## Options
    ///
    /// Use the `drizzleObjectName` option to specify the variable names that represent Drizzle
    /// ORM instances. This option has no default value, and the rule reports nothing until it's set.
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "drizzleObjectName": ["db"]
    ///   }
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic,use_options
    /// await db.transaction(async (tx) => {
    ///     await db.insert(users).values({ name: "John" });
    /// });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js,use_options
    /// await db.transaction(async (tx) => {
    ///     await tx.insert(users).values({ name: "John" });
    /// });
    /// ```
    ///
    pub NoDrizzleDbObjectInsideTransaction {
        version: "next",
        name: "noDrizzleDbObjectInsideTransaction",
        language: "js",
        recommended: false,
        severity: Severity::Information,
        domains: &[RuleDomain::Drizzle],
    }
}

impl Rule for NoDrizzleDbObjectInsideTransaction {
    type Query = Semantic<JsReferenceIdentifier>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoDrizzleDbObjectInsideTransactionOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let drizzle_object_name = ctx.options().drizzle_object_name.as_deref()?;
        let reference = ctx.query();
        let name = reference.value_token().ok()?;
        let name = name.text_trimmed();

        if !drizzle_object_name.iter().any(|n| n.as_ref() == name) {
            return None;
        }

        // Excludes type positions such as `typeof db`, which don't run queries.
        if !matches!(
            reference.syntax().parent()?.kind(),
            JsSyntaxKind::JS_IDENTIFIER_EXPRESSION
                | JsSyntaxKind::JS_SHORTHAND_PROPERTY_OBJECT_MEMBER
        ) {
            return None;
        }

        let model = ctx.model();
        reference
            .syntax()
            .ancestors()
            .filter_map(JsCallArguments::cast)
            .any(|arguments| {
                // Only an inline callback runs inside the transaction. A trailing config argument,
                // or an expression that builds the callback, runs before the transaction starts.
                let in_callback =
                    arguments
                        .args()
                        .first()
                        .and_then(Result::ok)
                        .is_some_and(|callback| {
                            matches!(
                                callback.syntax().kind(),
                                JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION
                                    | JsSyntaxKind::JS_FUNCTION_EXPRESSION
                            ) && callback.range().contains_range(reference.range())
                        });
                in_callback
                    && transaction_object(&arguments).is_some_and(|object| {
                        object.has_name(name) && model.binding(&object) == model.binding(reference)
                    })
            })
            .then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let name = node.value_token().ok()?;
        let name = name.text_trimmed();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    <Emphasis>{name}</Emphasis>" is used inside "<Emphasis>{name}".transaction()"</Emphasis>"."
                },
            )
            .note(markup! {
                "Queries made with "<Emphasis>{name}</Emphasis>" run outside the transaction, so they aren't rolled back with it and can deadlock waiting for its locks."
            })
            .note(markup! {
                "Use the transaction object passed to the callback instead."
            }),
        )
    }
}

/// Returns `db` when `arguments` belong to a call such as `db.transaction(...)`.
fn transaction_object(arguments: &JsCallArguments) -> Option<JsReferenceIdentifier> {
    let call = arguments.parent::<JsCallExpression>()?;
    let callee = AnyJsMemberExpression::cast(call.callee().ok()?.into_syntax())?;
    if callee.member_name()?.text() != "transaction" {
        return None;
    }
    callee
        .object()
        .ok()?
        .omit_parentheses()
        .as_js_reference_identifier()
}
