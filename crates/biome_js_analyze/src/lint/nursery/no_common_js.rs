use biome_analyze::{context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_js_syntax::{
    global_identifier, JsCallExpression, JsReferenceIdentifier, JsStaticMemberAssignment,
};
use biome_rowan::{declare_node_union, AstNode};

use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow use of CommonJs module system in favor of ESM style imports.
    ///
    /// ESM-style `import`s are modern alternative to CommonJS `require` imports. Supported by all modern browsers and Node.js versions.
    /// Tooling can more easily statically analyze and tree-shake ESM `import`s compared to CommonJs.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// require('node:fs');
    /// ```
    /// ```js,expect_diagnostic
    /// module.exports = { a: 'b' }
    /// ```
    /// ```js,expect_diagnostic
    /// exports.a = 'b';
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import fs from 'node:fs';
    /// ```
    /// ```js
    /// import('node:fs')
    /// ```
    /// ```js
    /// export const a = 'b';
    /// ```
    /// ```js
    /// export default { a: 'b' };
    /// ```
    ///
    /// ## Caveats
    ///
    /// Rule is automatically disabled inside `.cjs` and `.cts` files, because they are explicitly CommonJs files.
    ///
    /// This rule could be helpful if you are migrating from CommonJs to ESM,
    /// but if you wish to continue using CommonJs, you can safely disable it.
    ///
    pub NoCommonJs {
        version: "1.9.0",
        name: "noCommonJs",
        language: "js",
        sources: &[
            RuleSource::EslintTypeScript("no-require-imports"),
            RuleSource::EslintImport("no-commonjs"),
        ],
        recommended: false,
    }
}

impl Rule for NoCommonJs {
    type Query = Semantic<CommonJsImportExport>;
    type State = bool;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let file_ext = ctx.file_path().extension();
        // cjs and cts files can only use CommonJs modules
        if file_ext.is_some_and(|file_ext| matches!(file_ext.as_bytes(), b"cjs" | b"cts")) {
            return None;
        }

        let expression = ctx.query();
        let (reference, is_export) = match expression {
            CommonJsImportExport::JsCallExpression(node) => {
                Some((is_require_call_expression(node)?, false))
            }
            CommonJsImportExport::JsStaticMemberAssignment(node) => {
                Some((is_common_js_exports(node)?, true))
            }
        }?;

        if ctx.model().binding(&reference).is_none() {
            return Some(is_export);
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let (es_name, common_js_name) = match state {
            true => ("export", "module.exports"),
            false => ("import", "require"),
        };

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Use ESM "<Emphasis>{es_name}</Emphasis>"s instead of "<Emphasis>{common_js_name}</Emphasis>"."
                },
            )
            .note(markup! {
                "ESM-style "<Emphasis>{es_name}</Emphasis>" statements are more easily statically analyzable and tree-shakable compared to CommonJs "<Emphasis>{common_js_name}</Emphasis>"."
            }),
        )
    }
}

declare_node_union! {
    pub CommonJsImportExport = JsStaticMemberAssignment | JsCallExpression
}

fn is_require_call_expression(node: &JsCallExpression) -> Option<JsReferenceIdentifier> {
    let callee = node.callee().ok()?;
    let (reference, name) = global_identifier(&callee.omit_parentheses())?;

    if name.text() == "require" {
        return Some(reference);
    }

    None
}

fn is_common_js_exports(node: &JsStaticMemberAssignment) -> Option<JsReferenceIdentifier> {
    let object = node.object().ok()?;
    let (reference, name) = global_identifier(&object.omit_parentheses())?;
    let object_name = name.text();

    // exports.*
    if object_name == "exports" {
        return Some(reference);
    }

    // module.exports.*
    if object_name != "module" {
        return None;
    }

    let value_token = node.member().ok()?.value_token().ok()?;
    let member_name = value_token.text_trimmed();
    if member_name == "exports" {
        return Some(reference);
    }

    None
}
