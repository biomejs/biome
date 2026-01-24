use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_before_interactive_script_outside_document::NoBeforeInteractiveScriptOutsideDocumentOptions;

use crate::{
    nextjs::{NextUtility, is_next_import},
    services::semantic::Semantic,
};

declare_lint_rule! {
    /// Prevent usage of `next/script`'s `beforeInteractive` strategy outside of `pages/_document.js` in a Next.js project.
    ///
    /// Next.js provides a `next/script` component to optimize the loading of third-party scripts. Using the `beforeInteractive`
    /// strategy allows scripts to be preloaded before any first-party code. `beforeInteractive` scripts must be placed in `pages/_document.js`.
    ///
    /// This rule checks for any usage of the `beforeInteractive` scripts outside of these files.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// // pages/index.jsx
    /// import Script from 'next/script'
    ///
    /// export default function Index() {
    ///   return (
    ///     <div>
    ///       <Script
    ///         src="https://example.com/script.js"
    ///         strategy="beforeInteractive"
    ///       ></Script>
    ///     </div>
    ///   )
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx,ignore
    /// // pages/_document.jsx
    /// import { Html, Head, Main, NextScript } from 'next/document'
    /// import Script from 'next/script'
    ///
    /// export default function Document() {
    ///     return (
    ///         <Html>
    ///             <Head />
    ///             <body>
    ///                 <Main />
    ///                 <NextScript />
    ///                 <Script
    ///                   src="https://example.com/script.js"
    ///                   strategy="beforeInteractive"
    ///                 ></Script>
    ///             </body>
    ///         </Html>
    ///     )
    /// }
    /// ```
    ///
    pub NoBeforeInteractiveScriptOutsideDocument {
        version: "2.3.11",
        name: "noBeforeInteractiveScriptOutsideDocument",
        language: "jsx",
        sources: &[RuleSource::EslintNext("no-before-interactive-script-outside-document").same()],
        recommended: false,
        severity: Severity::Warning,
        domains: &[RuleDomain::Next],
    }
}

impl Rule for NoBeforeInteractiveScriptOutsideDocument {
    type Query = Semantic<AnyJsxElement>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = NoBeforeInteractiveScriptOutsideDocumentOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let is_in_app_dir = ctx
            .file_path()
            .ancestors()
            .any(|a| a.file_name().is_some_and(|f| f == "app" && a.is_dir()));
        // should not run in app dir
        if is_in_app_dir {
            return None;
        }

        let jsx_element = ctx.query();

        let semantic_model = ctx.model();
        let reference = jsx_element.name().ok()?;
        let reference = reference.as_jsx_reference_identifier()?;
        let binding = semantic_model.binding(reference)?;
        if !is_next_import(&binding, NextUtility::Script) {
            return None;
        }

        let strategy_attribute = jsx_element.find_attribute_by_name("strategy")?;
        let strategy_attribute_value = strategy_attribute.as_static_value()?;
        let strategy_attribute_value = strategy_attribute_value.text();
        if strategy_attribute_value != "beforeInteractive" {
            return None;
        }

        let path = ctx.file_path();

        let file_name = path.file_stem()?;

        // pages/_document.(js|ts|jsx|tsx)
        let is_in_pages_dir = path.parent()?.file_name().is_some_and(|f| f == "pages");
        if is_in_pages_dir && file_name == "_document" {
            return None;
        }

        Some(jsx_element.syntax().text_range_with_trivia())
    }

    fn diagnostic(_: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            range,
            markup! {
                "Don't use "<Emphasis>"next/script"</Emphasis>" component with the `"<Emphasis>"beforeInteractive"</Emphasis>"` strategy outside of "<Emphasis>"pages/_document.js"</Emphasis>"."
            },
        ).note(markup! {
            "See the "<Hyperlink href="https://nextjs.org/docs/messages/no-before-interactive-script-outside-document">"Next.js docs"</Hyperlink>" for more details."
        }))
    }
}
