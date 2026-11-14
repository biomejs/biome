use crate::globals::{is_google_apps_script_global, is_js_global};

use crate::services::embedded::EmbeddedService;
use crate::services::semantic::Semantic;
use biome_analyze::RuleSource;
use biome_analyze::{Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsExpression, JsAssignmentOperator, JsExpressionStatement,
    JsIdentifierAssignment, JsLabeledStatement, JsModuleItemList, TextRange,
};
use biome_rowan::AstNode;
use biome_languages::JsFileSource;
use biome_rule_options::no_global_assign::NoGlobalAssignOptions;

declare_lint_rule! {
    /// Disallow assignments to native objects and read-only global variables.
    ///
    /// JavaScript's environments contain numerous built-in global variables, such as `window` in browsers and `process` in Node.js.
    /// Assigning values to these global variables can be problematic as it can override essential functionality.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// Object = null;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// window = {};
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// undefined = true;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// a = 0;
    /// ```
    ///
    /// ```js
    /// let window;
    /// window = {};
    /// ```
    ///
    /// In Svelte components, a top-level reactive statement such as `$: length = 2` declares a
    /// component variable, so it isn't reported.
    pub NoGlobalAssign {
        version: "1.5.0",
        name: "noGlobalAssign",
        language: "js",
        sources: &[
            RuleSource::Eslint("no-global-assign").same(),
            RuleSource::Eslint("no-native-reassign").same(),
        ],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for NoGlobalAssign {
    type Query = Semantic<JsIdentifierAssignment>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = NoGlobalAssignOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let assignment = ctx.query();
        if !ctx.model().is_unresolved_reference(assignment) {
            return None;
        }
        let token = assignment.name_token().ok()?;

        let source_type = ctx.source_type::<JsFileSource>();
        if source_type.as_embedding_kind().is_svelte_instance_script()
            && is_svelte_reactive_declaration(assignment)
        {
            return None;
        }

        // Only trust a cross-embed binding from non-source embeds like template
        // expressions. Source embeds (e.g. Vue's `<script>` and `<script setup>`)
        // don't necessarily see each other's bindings, so trusting it there
        // would hide genuine global assignments.
        if !ctx.source_type::<JsFileSource>().is_embedded_source() {
            let embedded = ctx
                .get_service::<EmbeddedService>()
                .expect("embedded service");
            if embedded.contains_binding(token.token_text_trimmed()) {
                return None;
            }
        }

        let name = token.text_trimmed();
        // Apps Script service globals (e.g. `SpreadsheetApp`) are read-only
        // globals too, so reassigning them in a `.gs` file is reported.
        let is_global_var = is_js_global(name)
            || (source_type.is_google_apps_script() && is_google_apps_script_global(name));
        is_global_var.then(|| token.text_trimmed_range())
    }

    fn diagnostic(_ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "A global variable should not be reassigned."
                },
            )
            .note(markup! {
                "Assigning to a global variable can override essential functionality."
            })
            .note(markup! {
                "Remove the assignment, or declare a local variable with a different name and assign to that instead."
            }),
        )
    }
}

/// Returns whether `assignment` is a target of a top-level Svelte reactive declaration, such as
/// `length` in `$: length = 2` or in `$: ({ length } = value)`.
///
/// Svelte declares a component variable for each target of such a statement, so the assignment
/// doesn't reach a global. Nothing is declared for an assignment in a block or another
/// expression, such as `$: { length = 2 }` or `$: x = (length = 2)`, or for a variable that is
/// only updated, such as `top` in `$: length = top++`.
fn is_svelte_reactive_declaration(assignment: &JsIdentifierAssignment) -> bool {
    // The nearest enclosing expression is the assignment when `assignment` is one of its targets,
    // and an update expression such as `top++` when it isn't.
    let Some(AnyJsExpression::JsAssignmentExpression(assignment_expression)) = assignment
        .syntax()
        .ancestors()
        .skip(1)
        .find_map(AnyJsExpression::cast)
    else {
        return false;
    };
    if assignment_expression.operator() != Ok(JsAssignmentOperator::Assign) {
        return false;
    }
    AnyJsExpression::from(assignment_expression)
        .outer_expression()
        .and_then(|expression| expression.syntax().parent())
        .and_then(JsExpressionStatement::cast)
        .and_then(|statement| statement.syntax().parent())
        .and_then(JsLabeledStatement::cast)
        .is_some_and(|labeled| {
            labeled
                .label_token()
                .is_ok_and(|label| label.text_trimmed() == "$")
                && labeled
                    .syntax()
                    .parent()
                    .is_some_and(|parent| JsModuleItemList::can_cast(parent.kind()))
        })
}
