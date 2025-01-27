use crate::services::semantic::Semantic;
use biome_analyze::{context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::ReferencesExtensions;
use biome_js_syntax::JsCatchClause;
use biome_rowan::{AstNode, TextRange};

declare_lint_rule! {
    /// Disallow reassigning exceptions in catch clauses.
    ///
    /// Assignment to a `catch` parameter can be misleading and confusing.
    /// It is often unintended and indicative of a programmer error.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// try {
    ///
    /// } catch (e) {
    ///   e;
    ///   e = 10;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// try {
    ///
    /// } catch (e) {
    ///   let e = 10;
    ///   e = 100;
    /// }
    /// ```
    pub NoCatchAssign {
        version: "1.0.0",
        name: "noCatchAssign",
        language: "js",
        sources: &[RuleSource::Eslint("no-ex-assign")],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for NoCatchAssign {
    // Why use [JsCatchClause] instead of [JsIdentifierAssignment] ?
    // Because this could reduce search range.
    // We only compare the declaration of [JsCatchClause] with all descent
    // [JsIdentifierAssignment] of its body.
    type Query = Semantic<JsCatchClause>;
    // The first element of `State` is the reassignment of catch parameter,
    // the second element of `State` is the declaration of catch clause.
    type State = (TextRange, TextRange);
    type Signals = Box<[Self::State]>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let catch_clause = ctx.query();
        let model = ctx.model();
        catch_clause
            .declaration()
            .and_then(|decl| {
                let catch_binding = decl.binding().ok()?;
                // Only [JsIdentifierBinding] is allowed to use `model.all_references` now,
                // so there's need to make sure this is a [JsIdentifierBinding].
                let identifier_binding = catch_binding
                    .as_any_js_binding()?
                    .as_js_identifier_binding()?;
                let catch_binding_syntax = catch_binding.syntax();
                let mut invalid_assignment = Vec::new();
                for reference in identifier_binding.all_writes(model) {
                    invalid_assignment.push((
                        reference.syntax().text_trimmed_range(),
                        catch_binding_syntax.text_trimmed_range(),
                    ));
                }

                Some(invalid_assignment)
            })
            .unwrap_or_default()
            .into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let (assignment, catch_binding_syntax) = state;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                assignment,
                markup! {
                    "Reassigning a "<Emphasis>"catch parameter"</Emphasis>" is confusing."
                },
            )
            .detail(
                catch_binding_syntax,
                markup! {
                    "The "<Emphasis>"catch parameter"</Emphasis>" is declared here:"
                },
            )
            .note("Use a local variable instead."),
        )
    }
}
