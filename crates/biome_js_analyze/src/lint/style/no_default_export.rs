use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_js_syntax::AnyJsExportClause;
use biome_rowan::{AstNode, AstSeparatedList, TextRange};

declare_lint_rule! {
    /// Disallow default exports.
    ///
    /// Default exports cannot be easily discovered inside an editor:
    /// They cannot be suggested by the editor when the user tries to import a name.
    ///
    /// Also, default exports don't encourage consistency over a code base:
    /// the module that imports the default export must choose a name.
    /// It is likely that different modules use different names.
    ///
    /// Moreover, default exports encourage exporting an object that acts as a namespace.
    /// This is a legacy pattern used to mimic CommonJS modules.
    ///
    /// For all these reasons, a team may want to disallow default exports.
    ///
    /// Note that this rule disallows only default exports in EcmaScript Module.
    /// It ignores CommonJS default exports.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// export default function f() {};
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// export default class C {};
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// export default {
    ///     f() {},
    ///     g() {},
    /// };
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// export { X as default };
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// export function f () {};
    /// export class C {};
    /// export { default as X } from "mod";
    /// ```
    ///
    /// ```cjs
    /// module.exports = class {};
    /// ```
    ///
    pub NoDefaultExport {
        version: "1.4.0",
        name: "noDefaultExport",
        language: "js",
        sources: &[RuleSource::EslintImport("no-default-export")],
        recommended: false,
    }
}

impl Rule for NoDefaultExport {
    type Query = Ast<AnyJsExportClause>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let export_clause = ctx.query();
        match export_clause {
            AnyJsExportClause::JsExportDefaultDeclarationClause(clause) => {
                Some(clause.default_token().ok()?.text_trimmed_range())
            }
            AnyJsExportClause::JsExportDefaultExpressionClause(clause) => {
                Some(clause.default_token().ok()?.text_trimmed_range())
            }
            AnyJsExportClause::JsExportNamedClause(clause) => clause
                .specifiers()
                .iter()
                .filter_map(|x| x.ok()?.as_js_export_named_specifier()?.exported_name().ok())
                .find(|x| x.is_default())
                .map(|x| x.range()),
            AnyJsExportClause::JsExportNamedFromClause(clause) => clause
                .specifiers()
                .iter()
                .filter_map(|x| x.ok()?.export_as()?.exported_name().ok())
                .find(|x| x.is_default())
                .map(|x| x.range()),
            AnyJsExportClause::AnyJsDeclarationClause(_)
            | AnyJsExportClause::JsExportFromClause(_)
            | AnyJsExportClause::TsExportAsNamespaceClause(_)
            | AnyJsExportClause::TsExportAssignmentClause(_)
            | AnyJsExportClause::TsExportDeclareClause(_) => None,
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "Avoid "<Emphasis>"default"</Emphasis>" exports."
                },
            )
            .note(markup ! {
                "Default exports cannot be easily discovered inside an editor and don't encourage the use of consistent names through a code base."
            })
            .note(markup! {
                "Use a named export instead."
            }),
        )
    }
}
