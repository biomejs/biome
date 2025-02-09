use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource,
    RuleSourceKind,
};
use biome_console::markup;
use biome_js_syntax::{JsFileSource, JsImport};
use biome_rowan::AstNode;

declare_lint_rule! {
    /// Prevents importing `next/document` outside of `pages/_document.jsx` in Next.js projects.
    ///
    /// The `next/document` module is intended for customizing the document structure globally in Next.js.
    /// Importing it outside of `pages/_document.js` can cause unexpected behavior and break certain features of the framework.
    ///
    /// ## Examples
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// import { Document, Html } from 'next/document'
    ///
    /// export default class MyDocument extends Document {
    ///   render() {
    ///     return (
    ///       <Html lang="en">
    ///         {/* */}
    ///       </Html>
    ///     )
    ///   }
    /// }
    /// ```
    ///
    pub NoDocumentImportInPage {
        version: "1.9.4",
        name: "noDocumentImportInPage",
        language: "jsx",
        sources: &[RuleSource::EslintNext("no-document-import-in-page")],
        source_kind: RuleSourceKind::SameLogic,
        recommended: true,
        domains: &[RuleDomain::Next],
    }
}

impl Rule for NoDocumentImportInPage {
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

        if module_name != "next/document" {
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
        let parent_name = path.parent()?.file_stem()?;

        if parent_name == "_document" || file_name == "_document" {
            return None;
        }

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "Don't use "<Emphasis>"next/document"</Emphasis>" outside of pages/_document.jsx to avoid unexpected behaviors."
                },
            )
            .note(markup! {
                "Only import "<Emphasis>"next/document"</Emphasis>" within "<Emphasis>"pages/_document.jsx"</Emphasis>" to customize the global document structure."
            })
        )
    }
}
