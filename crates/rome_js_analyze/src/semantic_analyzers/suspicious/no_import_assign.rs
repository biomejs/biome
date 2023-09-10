use crate::semantic_services::Semantic;
use biome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use biome_console::markup;
use rome_js_semantic::ReferencesExtensions;
use rome_js_syntax::{
    JsDefaultImportSpecifier, JsIdentifierAssignment, JsIdentifierBinding, JsImportDefaultClause,
    JsImportNamespaceClause, JsNamedImportSpecifier, JsNamespaceImportSpecifier,
    JsShorthandNamedImportSpecifier,
};

use biome_rowan::{declare_node_union, AstNode};

declare_rule! {
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
    /// ({ z } = 1); /// ```
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
    pub(crate) NoImportAssign {
        version: "1.0.0",
        name: "noImportAssign",
        recommended: true,
    }
}

impl Rule for NoImportAssign {
    type Query = Semantic<AnyJsImportLike>;
    /// The first element of the tuple is the invalid `JsIdentifierAssignment`, the second element of the tuple is the imported `JsIdentifierBinding`.
    type State = (JsIdentifierAssignment, JsIdentifierBinding);
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Vec<Self::State> {
        let label_statement = ctx.query();
        let mut invalid_assign_list = vec![];
        let local_name_binding = match label_statement {
            // `import xx from 'y'`
            AnyJsImportLike::JsImportDefaultClause(clause) => clause.local_name().ok(),
            // `import * as xxx from 'y'`
            AnyJsImportLike::JsImportNamespaceClause(clause) => clause.local_name().ok(),
            // `import {x as xx} from 'y'`
            //          ^^^^^^^
            AnyJsImportLike::JsNamedImportSpecifier(specifier) => specifier.local_name().ok(),
            // `import {x} from 'y'`
            //          ^
            AnyJsImportLike::JsShorthandNamedImportSpecifier(specifier) => {
                specifier.local_name().ok()
            }
            // `import a, * as b from 'y'`
            //            ^^^^^^
            AnyJsImportLike::JsNamespaceImportSpecifier(specifier) => specifier.local_name().ok(),
            // `import a, * as b from 'y'`
            //         ^
            AnyJsImportLike::JsDefaultImportSpecifier(specifier) => specifier.local_name().ok(),
        };
        local_name_binding
            .and_then(|binding| {
                let ident_binding = binding.as_js_identifier_binding()?;
                let model = ctx.model();
                for reference in ident_binding.all_writes(model) {
                    invalid_assign_list.push((
                        JsIdentifierAssignment::cast(reference.syntax().clone())?,
                        ident_binding.clone(),
                    ));
                }
                Some(invalid_assign_list)
            })
            .unwrap_or_default()
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

declare_node_union! {
    pub(crate) AnyJsImportLike = JsImportDefaultClause | JsImportNamespaceClause | JsNamedImportSpecifier | JsShorthandNamedImportSpecifier | JsNamespaceImportSpecifier | JsDefaultImportSpecifier
}
