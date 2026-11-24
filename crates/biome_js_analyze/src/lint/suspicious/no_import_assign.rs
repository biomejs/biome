use crate::services::embedded::EmbeddedService;
use crate::services::semantic::Semantic;
use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::JsIdentifierAssignment;
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_import_assign::NoImportAssignOptions;

declare_lint_rule! {
    /// Disallow assigning a new value to an imported variable.
    ///
    /// An import is a read-only reference to a value exported by another module. Reassigning the
    /// imported name does not update that module and causes an error. Copy the value to a local
    /// variable when it needs to change.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import x from "y";
    /// x = 1;
    /// ```
    /// ```js,expect_diagnostic
    /// import y from "y";
    /// [y] = 1;
    /// ```
    /// ```js,expect_diagnostic
    /// import z from "y";
    /// ({ z } = 1);
    /// ```
    /// ```js,expect_diagnostic
    /// import a from "y";
    /// [...a] = 1;
    /// ```
    /// ```js,expect_diagnostic
    /// import b from "y";
    /// ({ ...b } = 1);
    /// ```
    /// ```js,expect_diagnostic
    /// import c from "y";
    /// for (c in y) {};
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import d from "y";
    /// d += 1;
    /// ```
    /// ```js,expect_diagnostic
    /// import * as e from "y";
    /// e = 1;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import value from "y";
    /// let localValue = value;
    /// localValue = 1;
    /// ```
    pub NoImportAssign {
        version: "1.0.0",
        name: "noImportAssign",
        language: "js",
        sources: &[RuleSource::Eslint("no-import-assign").same()],
        recommended: true,
        severity: Severity::Error,
    }
}

pub enum ImportDeclaration {
    Local(TextRange),
    OtherSnippet,
}

impl Rule for NoImportAssign {
    type Query = Semantic<JsIdentifierAssignment>;
    type State = ImportDeclaration;
    type Signals = Option<Self::State>;
    type Options = NoImportAssignOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let assignment = ctx.query();
        if let Some(binding) = ctx.model().binding(assignment) {
            return binding
                .is_imported()
                .then(|| ImportDeclaration::Local(binding.range()));
        }

        ctx.get_service::<EmbeddedService>()?
            .svelte_declaration_kind(assignment.range())?
            .is_import_declaration()
            .then_some(ImportDeclaration::OtherSnippet)
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let assignment = ctx.query();
        let name = assignment.syntax().text_trimmed();
        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            assignment.syntax().text_trimmed_range(),
            markup! {
                "The imported variable "<Emphasis>{name.to_string()}</Emphasis>" is read-only"
            },
        )
        .note(markup! {"Use a local variable instead of reassigning an import."});
        if let ImportDeclaration::Local(range) = state {
            diagnostic = diagnostic.detail(
                range,
                markup! {
                    "The variable is imported here"
                },
            );
        }
        Some(diagnostic)
    }
}
