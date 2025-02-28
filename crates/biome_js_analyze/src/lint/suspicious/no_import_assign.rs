use crate::services::semantic::Semantic;
use biome_analyze::{context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::ReferencesExtensions;
use biome_js_syntax::{AnyJsImportSpecifier, JsIdentifierAssignment, JsIdentifierBinding};

use biome_rowan::AstNode;

declare_lint_rule! {
    ///  Disallow assigning to imported bindings
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
    pub NoImportAssign {
        version: "1.0.0",
        name: "noImportAssign",
        language: "js",
        sources: &[RuleSource::Eslint("no-import-assign")],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for NoImportAssign {
    type Query = Semantic<AnyJsImportSpecifier>;
    /// The first element of the tuple is the invalid `JsIdentifierAssignment`, the second element of the tuple is the imported `JsIdentifierBinding`.
    type State = (JsIdentifierAssignment, JsIdentifierBinding);
    type Signals = Box<[Self::State]>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let label_statement = ctx.query();
        let mut invalid_assign_list = Vec::new();
        let local_name_binding = match label_statement {
            // `import {x as xx} from 'y'`
            //          ^^^^^^^
            AnyJsImportSpecifier::JsNamedImportSpecifier(specifier) => specifier.local_name().ok(),
            // `import {x} from 'y'`
            //          ^
            AnyJsImportSpecifier::JsShorthandNamedImportSpecifier(specifier) => {
                specifier.local_name().ok()
            }
            // `import * as xxx from 'y'`
            //         ^^^^^^^^
            // `import a, * as b from 'y'`
            //            ^^^^^^
            AnyJsImportSpecifier::JsNamespaceImportSpecifier(specifier) => {
                specifier.local_name().ok()
            }
            // `import xx from 'y'`
            //         ^^
            // `import a, * as b from 'y'`
            //         ^
            AnyJsImportSpecifier::JsDefaultImportSpecifier(specifier) => {
                specifier.local_name().ok()
            }
        };
        local_name_binding
            .and_then(|binding| {
                let ident_binding = binding.as_js_identifier_binding()?;
                let model = ctx.model();
                for reference in ident_binding.all_writes(model) {
                    invalid_assign_list.push((
                        JsIdentifierAssignment::cast_ref(reference.syntax())?,
                        ident_binding.clone(),
                    ));
                }
                Some(invalid_assign_list)
            })
            .unwrap_or_default()
            .into_boxed_slice()
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let (invalid_assign, import_binding) = state;
        let name = invalid_assign.syntax().text_trimmed();

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                invalid_assign.syntax().text_trimmed_range(),
                markup! {
                    "The imported variable "<Emphasis>{name.to_string()}</Emphasis>" is read-only"
                },
            )
            .note(markup! {"Use a local variable instead of reassigning an import."})
            .detail(
                import_binding.syntax().text_trimmed_range(),
                markup! {
                    "The variable is imported here"
                },
            ),
        )
    }
}
