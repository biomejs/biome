use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource, RuleSourceKind,
};
use biome_console::markup;
use biome_js_syntax::{JsFileSource, JsImport};
use biome_rowan::AstNode;
use std::path::MAIN_SEPARATOR;

declare_lint_rule! {
    /// Prevent usage of `next/head` in `pages/_document.js` on Next.js projects.
    ///
    /// Importing `next/head` within the custom `pages/_document.js` file can cause
    /// unexpected behavior in your application. The `next/head` component is designed
    /// to be used at the page-level, and when used in the custom document it can interfere
    /// with the global document structure and lead to issues with rendering and SEO.
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
        version: "next",
        name: "noHeadImportInDocument",
        language: "jsx",
        sources: &[RuleSource::EslintNext("no-head-import-in-document")],
        source_kind: RuleSourceKind::SameLogic,
        recommended: false,
    }
}

impl Rule for NoHeadImportInDocument {
    type Query = Ast<JsImport>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let import = ctx.query();
        let import_source = import.import_clause().ok()?.source().ok()?;
        let module_name = import_source.inner_string_text().ok()?;

        if !ctx.source_type::<JsFileSource>().is_jsx() || module_name != "next/head" {
            return None;
        }

        let path = ctx.file_path();

        if !path
            .ancestors()
            .any(|a| a.file_name().map_or(false, |f| f == "pages"))
        {
            return None;
        }

        let file_name = path.file_stem()?.to_str()?;

        // pages/_document.(jsx|tsx)
        if file_name == "_document" {
            return Some(());
        }

        let parent_name = path.parent()?.file_stem()?.to_str()?;

        // pages/_document/index.(jsx|tsx)
        if parent_name == "_document" && file_name == "index" {
            return Some(());
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let path = ctx.file_path().to_str()?.split("pages").nth(1)?;
        let normalized_path = path.replace(MAIN_SEPARATOR, "/");

        return Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "Don't use "<Emphasis>"next/head"</Emphasis>" in pages"{normalized_path}""
                },
            )
            .note(markup! {
                "Using the "<Emphasis>"next/head"</Emphasis>" in document pages can cause unexpected issues. Use "<Emphasis>"<Head />"</Emphasis>" from "<Emphasis>"next/document"</Emphasis>" instead."
            })
        );
    }
}
