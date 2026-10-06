use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::AnyJsImportLike;
use biome_rowan::TextRange;
use biome_rule_options::no_react_native_deep_imports::NoReactNativeDeepImportsOptions;

declare_lint_rule! {
    /// Disallow deep imports from the `react-native` package.
    ///
    /// Deep imports reach into React Native's internal file structure,
    /// which is not part of the public API. Internal paths can change
    /// between versions without warning, breaking code that depends on them.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import View from "react-native/Libraries/Components/View/View";
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const Platform = require("react-native/Libraries/Utilities/Platform");
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const View = require("react-native/Libraries/Components/View/View");
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import("react-native/Libraries/Utilities/Platform");
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import { View } from "react-native";
    /// ```
    ///
    /// ```js
    /// const { Platform } = require("react-native");
    /// ```
    ///
    pub NoReactNativeDeepImports {
        version: "2.4.13",
        name: "noReactNativeDeepImports",
        language: "js",
        sources: &[RuleSource::EslintReactNative("no-deep-imports").same()],
        domains: &[RuleDomain::ReactNative],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for NoReactNativeDeepImports {
    type Query = Ast<AnyJsImportLike>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = NoReactNativeDeepImportsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let module_name_token = node.module_name_token()?;
        let import_path = node.inner_string_text()?;
        let import_path = import_path.text();

        if let Some(rest) = import_path.strip_prefix("react-native/")
            && !rest.is_empty()
        {
            return Some(module_name_token.text_trimmed_range());
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "Deep imports from "<Emphasis>"react-native"</Emphasis>" are not allowed."
                },
            )
            .note(markup! {
                "React Native's internal file structure is not part of the public API and may change between versions without warning."
            })
            .note(markup! {
                "Import from the top-level "<Emphasis>"react-native"</Emphasis>" entry point instead."
            }),
        )
    }
}
