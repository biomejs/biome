use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource,
    RuleSourceKind,
};
use biome_console::markup;
use biome_js_syntax::{JsFileSource, JsImport};
use biome_rowan::AstNode;
use std::path::MAIN_SEPARATOR;

declare_lint_rule! {
    /// Prevent using the `next/head` module in `pages/_document.js` on Next.js projects.
    ///
    /// Importing `next/head` within the custom `pages/_document.js` file can cause
    /// unexpected behavior in your application. The `next/head` component is designed
    /// to be used at the page level, and when used in the custom document it can interfere
    /// with the global document structure, which leads to issues with rendering and SEO.
    ///
    /// To modify `<head>` elements across all pages, you should use the `<Head />`
    /// component from the `next/document` module.
    ///
    /// ## Examples
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// // pages/_document.js
    /// import Document, { Html, Head, Main, NextScript } from "next/document";
    ///
    /// class MyDocument extends Document {
    ///   static async getInitialProps(ctx) {
    ///     //...
    ///   }
    ///
    ///   render() {
    ///     return (
    ///       <Html>
    ///         <Head></Head>
    ///       </Html>
    ///     );
    ///   }
    /// }
    ///
    /// export default MyDocument;
    /// ```
    ///
    pub NoHeadImportInDocument {
        version: "1.9.4",
        name: "noHeadImportInDocument",
        language: "jsx",
        sources: &[RuleSource::EslintNext("no-head-import-in-document")],
        source_kind: RuleSourceKind::SameLogic,
        recommended: true,
        domains: &[RuleDomain::Next],
    }
}

impl Rule for NoHeadImportInDocument {
    type Query = Ast<JsImport>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !ctx.source_type::<JsFileSource>().is_jsx() {
            return None;
        }

        let import = ctx.query();
        let import_source = import.import_clause().ok()?.source().ok()?;
        let module_name = import_source.inner_string_text().ok()?;

        if module_name != "next/head" {
            return None;
        }

        let path = ctx.file_path();

        if !path
            .ancestors()
            .filter_map(|a| a.file_name())
            .any(|f| f == "pages")
        {
            return None;
        }

        let file_name = path.file_stem()?;

        // pages/_document.(jsx|tsx)
        if file_name == "_document" {
            return Some(());
        }

        let parent_name = path.parent()?.file_stem()?;

        // pages/_document/index.(jsx|tsx)
        if parent_name == "_document" && file_name == "index" {
            return Some(());
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let path = ctx.file_path().as_str().split("pages").nth(1)?;
        let path = if cfg!(debug_assertions) {
            path.replace(MAIN_SEPARATOR, "/")
        } else {
            path.to_string()
        };

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "Don't use "<Emphasis>"next/head"</Emphasis>" in pages"{path}""
                },
            )
            .note(markup! {
                "Using the "<Emphasis>"next/head"</Emphasis>" in document pages can cause unexpected issues. Use "<Emphasis>"<Head />"</Emphasis>" from "<Emphasis>"next/document"</Emphasis>" instead."
            })
        )
    }
}
