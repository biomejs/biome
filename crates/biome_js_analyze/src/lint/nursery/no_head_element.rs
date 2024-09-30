use biome_analyze::RuleSourceKind;
use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_js_syntax::JsxOpeningElement;
use biome_rowan::AstNode;
use biome_rowan::TextRange;

declare_lint_rule! {
    /// Prevent usage of `<head>` element.
    ///
    /// A `<head>` element was used to include page-level metadata, but this can
    /// cause unexpected behavior in a Next.js application. Use Next.js' built-in
    /// `next/head` component instead.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    /// ```jsx,expect_diagnostic
    /// // /pages/index.jsx
    /// function Index() {
    ///   return (
    ///     <head>
    ///       <title>Invalid</title>
    ///     </head>
    ///   )
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// // /pages/index.jsx
    /// import Head from 'next/head'
    ///
    /// function Index() {
    ///   return (
    ///     <Head>
    ///       <title>All good!</title>
    ///     </Head>
    ///   )
    /// }
    /// ```
    pub NoHeadElement {
        version: "next",
        name: "noHeadElement",
        language: "js",
        sources: &[RuleSource::EslintNext("no-head-element")],
        source_kind: RuleSourceKind::SameLogic,
        recommended: false,
    }
}

impl Rule for NoHeadElement {
    type Query = Ast<JsxOpeningElement>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let element = ctx.query();
        let name = element.name().ok()?.name_value_token()?;

        if name.text_trimmed() == "head" {
            let is_in_app_dir = ctx
                .file_path()
                .ancestors()
                .any(|a| a.file_name().map_or(false, |f| f == "app" && a.is_dir()));

            if !is_in_app_dir {
                return Some(element.syntax().text_range());
            }
        }

        None
    }

    fn diagnostic(_: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        return Some(RuleDiagnostic::new(
            rule_category!(),
            range,
            markup! {
                "Do not use "<Emphasis>"<head>"</Emphasis>" element. Use "<Emphasis>"<Head />"</Emphasis>" from "<Emphasis>"next/head"</Emphasis>" instead."
            },
        ));
    }
}
