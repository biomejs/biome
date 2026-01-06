use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_rowan::{AstNode, AstNodeList, TextRange};
use biome_rule_options::no_html_link_for_pages::NoHtmlLinkForPagesOptions;

declare_lint_rule! {
    /// Prevent usage of `<a>` elements to navigate to internal Next.js pages.
    ///
    /// Using `<a>` elements instead of `next/link` for internal navigation can cause unnecessary full-page reloads.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// export const Page = () => {
    ///   return (
    ///     <div>
    ///       <a href='/about'>About</a>
    ///     </div>
    ///   );
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// import Link from "next/link";
    ///
    /// export const Page = () => {
    ///   return (
    ///     <div>
    ///       <Link href="/about">About</Link>
    ///     </div>
    ///   );
    /// }
    /// ```
    ///
    pub NoHtmlLinkForPages {
        version: "next",
        name: "noHtmlLinkForPages",
        language: "jsx",
        sources: &[RuleSource::EslintNext("no-html-link-for-pages").same()],
        recommended: false,
        severity: Severity::Warning,
        domains: &[RuleDomain::Next],
    }
}

impl Rule for NoHtmlLinkForPages {
    type Query = Ast<AnyJsxElement>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = NoHtmlLinkForPagesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let jsx_element = ctx.query();

        let element_name = jsx_element.name().ok()?.name_value_token().ok()?;
        if element_name.text_trimmed() != "a" {
            return None;
        }

        if jsx_element.attributes().is_empty() {
            return None;
        }

        // Skip when download attribute is present
        if let Some(_) = jsx_element.find_attribute_by_name("download") {
            return None;
        }

        // Should not enforce when target="_blank" present
        if let Some(target) = jsx_element.find_attribute_by_name("target")
            && let Some(target_value) = target.as_static_value()
            && target_value.text().trim() == "_blank"
        {
            return None;
        }

        let href_attribute = jsx_element.find_attribute_by_name("href")?;
        let href_value = href_attribute.as_static_value()?;
        let href_value = href_value.text();
        if href_value.is_empty() {
            return None;
        }

        if is_internal_link(href_value) {
            return Some(jsx_element.range());
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let jsx_element = ctx.query();
        let href_attribute = jsx_element.find_attribute_by_name("href")?;
        let href_value = href_attribute.as_static_value()?;
        let href_value = href_value.text();

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state,
                markup! {
                    ""<Emphasis>"<a>"</Emphasis>" element has an internal link to "<Emphasis>{href_value}</Emphasis>"."
                },
            )
            .note(markup! {
                ""<Emphasis>"<a>"</Emphasis>" elements for internal navigation can cause unnecessary full-page reloads. Use "<Emphasis>"next/link"</Emphasis>" component instead."
            })
            .note(markup! {
            "See the "<Hyperlink href="https://nextjs.org/docs/messages/no-html-link-for-pages">"Next.js docs"</Hyperlink>" for more details."
        }),
        )
    }
}

fn is_internal_link(href: &str) -> bool {
    let href = href.trim();
    if href.is_empty() {
        return false;
    }

    if href.starts_with("http://") || href.starts_with("https://") || href.starts_with("//") {
        return false;
    }

    // Skip other protocols
    if href.starts_with("mailto:")
        || href.starts_with("tel:")
        || href.starts_with("ftp:")
        || href.starts_with("file:")
    {
        return false;
    }

    // Skip if it appears to be a public file (e.g. .pdf)
    // Internal links in Next.js do not contain file extensions basically
    if let Some(last_segment) = href.split("/").last()
        && last_segment.contains(".")
    {
        return false;
    }

    href.starts_with('/') || href.starts_with("./") || href.starts_with("../")
}
