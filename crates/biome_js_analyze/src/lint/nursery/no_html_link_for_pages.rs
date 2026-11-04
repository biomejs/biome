use crate::services::database::ResolvedImports;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_module_graph::next_pages;
use biome_package::PackageJson;
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_html_link_for_pages::NoHtmlLinkForPagesOptions;
use camino::Utf8PathBuf;
use std::sync::Arc;

declare_lint_rule! {
    /// Disallow `<a>` elements that navigate to a page of the Next.js app.
    ///
    /// An `<a>` element makes the browser load the whole document again. Use the
    /// `<Link />` component from `next/link` instead: it navigates on the client,
    /// keeps the state of shared layouts, and prefetches the page.
    ///
    /// The rule reads the pages from the `app` or `src/app` directory (`page` files) and from the
    /// `pages` or `src/pages` directory next to the closest `package.json`, so it works in
    /// monorepos without configuration. It reports only links whose `href` is a string that
    /// matches one of these pages, including dynamic segments such as `[slug]`,
    /// `[...slug]`, and `[[...slug]]`. API routes and links to other origins are ignored.
    ///
    /// Links with a `download` attribute or with `target="_blank"` are allowed, because `<Link />`
    /// cannot replace them.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```json,file=package.json
    /// {
    ///   "dependencies": { "next": "*" }
    /// }
    /// ```
    ///
    /// ```jsx,file=app/about/page.jsx
    /// export default function About() {
    ///     return <h1>About</h1>;
    /// }
    /// ```
    ///
    /// ```jsx,expect_diagnostic,file=app/page.jsx
    /// export default function Home() {
    ///     return <a href="/about">About</a>;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```json,file=package.json
    /// {
    ///   "dependencies": { "next": "*" }
    /// }
    /// ```
    ///
    /// ```jsx,file=app/about/page.jsx
    /// export default function About() {
    ///     return <h1>About</h1>;
    /// }
    /// ```
    ///
    /// ```jsx,file=app/page.jsx
    /// import Link from "next/link";
    ///
    /// export default function Home() {
    ///     return (
    ///         <>
    ///             <Link href="/about">About</Link>
    ///             <a href="https://nextjs.org/">Next.js</a>
    ///             <a href="/about" target="_blank">About, in a new tab</a>
    ///         </>
    ///     );
    /// }
    /// ```
    pub NoHtmlLinkForPages {
        version: "next",
        name: "noHtmlLinkForPages",
        language: "jsx",
        sources: &[RuleSource::EslintNext("no-html-link-for-pages").inspired()],
        domains: &[RuleDomain::Next, RuleDomain::Project],
        severity: Severity::Warning,
        recommended: false,
        issue_number: Some("7650"),
    }
}

pub struct HtmlLinkToPage {
    href_range: TextRange,
    path: Box<str>,
}

impl Rule for NoHtmlLinkForPages {
    type Query = ResolvedImports<AnyJsxElement>;
    type State = HtmlLinkToPage;
    type Signals = Option<Self::State>;
    type Options = NoHtmlLinkForPagesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let element = ctx.query();
        if !element.matches_name("a").ok()?
            || element.find_attribute_by_name("download").is_some()
            || element
                .find_attribute_by_name("target")
                .and_then(|target| target.as_static_value())
                .is_some_and(|target| target.text() == "_blank")
        {
            return None;
        }

        let href = element.find_attribute_by_name("href")?;
        let href_value = href.as_static_value()?;
        let path = path_of_internal_href(href_value.text())?;
        let (package_path, _) = ctx
            .get_service::<Option<(Utf8PathBuf, Arc<PackageJson>)>>()?
            .as_ref()?;
        let path_segments: Vec<&str> = path.split('/').filter(|segment| !segment.is_empty()).collect();
        let links_to_page = next_pages(ctx.db())
            .iter()
            .any(|page| page.project_path == *package_path && page.matches(&path_segments));

        links_to_page.then(|| HtmlLinkToPage {
            href_range: href.range(),
            path: path.into(),
        })
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.href_range,
                markup! {
                    "This "<Emphasis>"<a>"</Emphasis>" element navigates to "<Emphasis>{state.path.as_ref()}</Emphasis>", a page of this Next.js app."
                },
            )
            .note(markup! {
                "An "<Emphasis>"<a>"</Emphasis>" element reloads the whole document, so the client loses its state and downloads the page's scripts again."
            })
            .note(markup! {
                "Use "<Emphasis>"<Link />"</Emphasis>" from "<Emphasis>"next/link"</Emphasis>" instead, which navigates on the client and prefetches the page."
            }),
        )
    }
}

/// Strips the query and fragment, and rejects `href`s to other origins.
fn path_of_internal_href(href: &str) -> Option<&str> {
    if !href.starts_with('/') || href.starts_with("//") {
        return None;
    }
    let end = href.find(['?', '#']).unwrap_or(href.len());
    Some(&href[..end])
}
