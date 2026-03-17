use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{JsCallExpression, JsStaticMemberExpression};
use biome_rowan::AstNode;
use biome_rule_options::no_drizzle_delete_without_where::NoDrizzleDeleteWithoutWhereOptions;

use crate::frameworks::drizzle::{get_identifier_name, has_where_in_chain};

declare_lint_rule! {
    /// Require `.where()` to be called when using `.delete()` with Drizzle ORM.
    ///
    /// Without a `.where()` clause, a `delete` statement will delete **all rows** from the table.
    /// This rule requires explicitly calling `.where()` to prevent accidental data loss.
    ///
    /// ## Options
    ///
    /// Use the `drizzleObjectName` option to specify the variable names that represent Drizzle
    /// ORM instances.
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
    /// await db.delete(users);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js,use_options
    /// await db.delete(users).where(eq(users.id, 1));
    /// ```
    ///
    pub NoDrizzleDeleteWithoutWhere {
        version: "next",
        name: "noDrizzleDeleteWithoutWhere",
        language: "js",
        sources: &[RuleSource::EslintDrizzle("enforce-delete-with-where").same()],
        recommended: false,
        domains: &[RuleDomain::Drizzle],
    }
}

impl Rule for NoDrizzleDeleteWithoutWhere {
    type Query = Ast<JsCallExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoDrizzleDeleteWithoutWhereOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call_expr = ctx.query();
        let options = ctx.options();

        if options.drizzle_object_name.is_empty() {
            return None;
        }

        let callee = call_expr.callee().ok()?;
        let member_expr = JsStaticMemberExpression::cast_ref(callee.syntax())?;

        let method_name = member_expr
            .member()
            .ok()?
            .as_js_name()?
            .value_token()
            .ok()?
            .token_text_trimmed();

        if method_name != "delete" {
            return None;
        }

        let object = member_expr.object().ok()?;
        let object_name = get_identifier_name(&object)?;

        if !options
            .drizzle_object_name
            .iter()
            .any(|n| n.as_str() == object_name.text())
        {
            return None;
        }

        if has_where_in_chain(call_expr.syntax()) {
            return None;
        }

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    <Emphasis>".delete()"</Emphasis>" is used without "<Emphasis>".where()"</Emphasis>". This will delete all rows in the table."
                },
            )
            .note(markup! {
                "Add a "<Emphasis>".where()"</Emphasis>" clause to delete only the intended rows, or use "<Emphasis>".where(sql`1=1`)"</Emphasis>" to explicitly delete all rows."
            }),
        )
    }
}
