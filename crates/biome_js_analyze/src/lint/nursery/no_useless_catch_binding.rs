use crate::JsRuleAction;
use crate::lint::correctness::no_unused_variables::is_unused;
use crate::services::semantic::Semantic;
use biome_analyze::{FixKind, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_js_syntax::{
    JsCatchClause, JsCatchClauseFields, JsCatchDeclaration, binding_ext::AnyJsIdentifierBinding,
};
use biome_rowan::{AstNode, BatchMutationExt, trim_leading_trivia_pieces};
use biome_rule_options::no_useless_catch_binding::NoUselessCatchBindingOptions;

declare_lint_rule! {
    /// Disallow unused catch bindings.
    ///
    /// This rule disallows unnecessary catch bindings in accordance with ECMAScript 2019.
    /// See also: the ECMAScript 2019 “optional catch binding” feature in the language specification.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// try {
    ///     // Do something
    /// } catch (unused) {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// try {
    ///     // Do something
    /// } catch ({ unused }) {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// try {
    ///     // Do something
    /// } catch ({ unused1, unused2 }) {}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// try {
    ///     // Do something
    /// } catch (used) {
    ///     console.error(used);
    /// }
    /// ```
    ///
    /// ```js
    /// try {
    ///     // Do something
    /// } catch ({ used }) {
    ///     console.error(used);
    /// }
    /// ```
    ///
    /// ```js
    /// try {
    ///     // Do something
    /// } catch ({ used, unused }) {
    ///     console.error(used);
    /// }
    /// ```
    ///
    /// ```js
    /// try {
    ///     // Do something
    /// } catch {}
    /// ```
    ///
    pub NoUselessCatchBinding {
        version: "2.2.3",
        name: "noUselessCatchBinding",
        language: "js",
        recommended: false,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoUselessCatchBinding {
    type Query = Semantic<JsCatchDeclaration>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoUselessCatchBindingOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let model = ctx.model();
        let catch_declaration = ctx.query();
        let catch_binding = catch_declaration.binding().ok()?;

        let all_unused = catch_binding
            .syntax()
            .descendants()
            .filter_map(AnyJsIdentifierBinding::cast)
            .all(|ident| is_unused(model, &ident));
        if all_unused {
            return Some(());
        }
        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let catch_declaration = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                catch_declaration.range(),
                markup! {
                    "This "<Emphasis>"catch binding"</Emphasis>" is unused."
                },
            )
            .note(markup! {
                "Since ECMAScript 2019, catch bindings are optional; you can omit the catch binding if you don't need it."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        let node = ctx.query();

        let catch_clause = node.syntax().parent()?;
        let JsCatchClauseFields {
            catch_token,
            declaration,
            ..
        } = JsCatchClause::cast(catch_clause)?.as_fields();

        let catch_token = catch_token.ok()?;
        let declaration = declaration?;
        let catch_token_replacement =
            if let Some(trivia) = declaration.syntax().last_trailing_trivia() {
                catch_token
                    .clone()
                    .append_trivia_pieces(trim_leading_trivia_pieces(trivia.pieces()))
            } else {
                catch_token.clone()
            };
        mutation.remove_node(declaration);
        mutation.replace_token_discard_trivia(catch_token, catch_token_replacement);

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the catch binding." }.to_owned(),
            mutation,
        ))
    }
}
