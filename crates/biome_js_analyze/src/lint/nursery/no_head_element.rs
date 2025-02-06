use biome_analyze::RuleSourceKind;
use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource,
};
use biome_console::markup;
use biome_js_syntax::JsxOpeningElement;
use biome_rowan::AstNode;
use biome_rowan::TextRange;

declare_lint_rule! {
    /// Prevent usage of `<head>` element in a Next.js project.
    ///
    /// Next.js provides a specialized `<Head />` component from `next/head` that manages
    /// the `<head>` tag for optimal server-side rendering, client-side navigation, and
    /// automatic deduplication of tags such as `<meta>` and `<title>`.
    ///
    /// This rule only checks files that are outside of the [`app/` directory](https://nextjs.org/docs/app), as it's typically
    /// handled differently in Next.js.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    /// ```jsx,expect_diagnostic
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
        version: "1.9.4",
        name: "noHeadElement",
        language: "jsx",
        sources: &[RuleSource::EslintNext("no-head-element")],
        source_kind: RuleSourceKind::SameLogic,
        recommended: true,
        domains: &[RuleDomain::Next],
    }
}

impl Rule for NoHeadElement {
    type Query = Ast<JsxOpeningElement>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let element = ctx.query();
        let name = element.name().ok()?.name_value_token().ok()?;

        if name.text_trimmed() == "head" {
            let is_in_app_dir = ctx
                .file_path()
                .ancestors()
                .any(|a| a.file_name().is_some_and(|f| f == "app" && a.is_dir()));

            if !is_in_app_dir {
                return Some(element.syntax().text_range_with_trivia());
            }
        }

        None
    }

    fn diagnostic(_: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            range,
            markup! { "Don't use "<Emphasis>"<head>"</Emphasis>" element." },
        ).note(markup! {
            "Using the "<Emphasis>"<head>"</Emphasis>" element can cause unexpected behavior in a Next.js application. Use "<Emphasis>"<Head />"</Emphasis>" from "<Emphasis>"next/head"</Emphasis>" instead."
        }))
    }
}
