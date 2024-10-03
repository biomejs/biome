use biome_analyze::{context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::{AnyJsModuleItem, JsModuleItemList, JsSyntaxNode};
use biome_rowan::AstNode;
use biome_rowan::AstNodeList;

declare_lint_rule! {
    /// Require that all exports are declared after all non-export statements.
    ///
    /// Enforces that export statements are placed at the end of the module, after all other statements.
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
    }
}

impl Rule for UseExportsLast {
    type Query = Ast<JsModuleItemList>;
    type State = JsSyntaxNode;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let items = ctx.query();
        let mut seen_export = false;

        for item in items.iter() {
            if is_export_declaration(&item) {
                seen_export = true;
            } else if seen_export {
                // Found a non-export statement after an export statement
                return Some(item.syntax().clone());
            }
        }
        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, node: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.text_range(),
                markup! {
                    "All exports should be declared after all non-export statements."
                },
            )
            .note(markup! {
                "Move this statement before the export statements to keep all exports at the end of the module."
            }),
        )
    }
}

fn is_export_declaration(item: &AnyJsModuleItem) -> bool {
    matches!(item, AnyJsModuleItem::JsExport(_))
}
