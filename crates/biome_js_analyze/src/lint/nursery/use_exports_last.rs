use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsModuleItem, JsModuleItemList};
use biome_rowan::{AstNode, AstNodeList, TextRange};

declare_lint_rule! {
    /// Require that all exports are declared after all non-export statements.
    ///
    /// Enforces that export statements are placed at the end of the module, after all other statements.
    ///
    /// Source: https://github.com/import-js/eslint-plugin-import/blob/main/docs/rules/exports-last.md
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// export const a = 1;
    /// const b = 2;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const a = 1;
    /// export const b = 2;
    /// ```
    ///
    /// ```js
    /// const a = 1;
    /// export { a };
    /// ```
    ///
    pub UseExportsLast {
        version: "next",
        name: "useExportsLast",
        language: "js",
        recommended: false,
        sources: &[RuleSource::EslintImport("exports-last")],
    }
}

impl Rule for UseExportsLast {
    type Query = Ast<JsModuleItemList>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let items = ctx.query();
        let mut last_export = None;

        for item in items.iter() {
            if matches!(item, AnyJsModuleItem::JsExport(_)) {
                last_export = Some(item.range());
            } else if last_export.is_some() {
                return last_export;
            }
        }
        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "Export statements should appear at the end of the file."
                },
            )
            .note(markup! {
                "Move this export to the end of the file."
            }),
        )
    }
}
