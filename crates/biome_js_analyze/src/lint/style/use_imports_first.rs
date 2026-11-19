use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsModuleItem, JsModuleItemList};
use biome_rowan::{AstNode, AstNodeList, TextRange};
use biome_rule_options::use_imports_first::UseImportsFirstOptions;

declare_lint_rule! {
    /// Enforce that all imports appear at the top of the module.
    ///
    /// Import statements that appear after non-import statements are harder to
    /// find and may indicate disorganized code. Keeping all imports together at
    /// the top makes dependencies immediately visible.
    ///
    /// Directives such as `"use strict"` are always allowed before
    /// imports, since they are parsed separately from module items.
    ///
    /// This rule only applies to ES module `import` statements. CommonJS
    /// `require()` calls are not covered.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import { foo } from "foo";
    /// const bar = 1;
    /// import { baz } from "baz";
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import { foo } from "foo";
    /// import { bar } from "bar";
    /// const baz = 1;
    /// ```
    ///
    /// ```js
    /// "use strict";
    /// import { foo } from "foo";
    /// ```
    ///
    pub UseImportsFirst {
        version: "2.4.7",
        name: "useImportsFirst",
        language: "js",
        recommended: false,
        sources: &[RuleSource::EslintImport("first").same()],
    }
}

impl Rule for UseImportsFirst {
    type Query = Ast<JsModuleItemList>;
    type State = TextRange;
    type Signals = Vec<Self::State>;
    type Options = UseImportsFirstOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let items = ctx.query();
        let mut seen_non_import = false;
        let mut signals = Vec::new();

        for item in items.iter() {
            match item {
                AnyJsModuleItem::JsImport(_) => {
                    if seen_non_import {
                        signals.push(item.range());
                    }
                }
                _ => {
                    seen_non_import = true;
                }
            }
        }

        signals
    }

    fn diagnostic(_ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This import appears after a non-import statement."
                },
            )
            .note(markup! {
                "Scattering imports makes it harder to see the module's dependencies at a glance."
            })
            .note(markup! {
                "Move all import statements before any other statements."
            }),
        )
    }
}
