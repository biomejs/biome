use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource,
    RuleSourceKind,
};
use biome_console::markup;
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_js_syntax::{JsxChildList, JsxElement};
use biome_rowan::{AstNode, AstNodeList};

declare_lint_rule! {
    /// Prevent usage of `<img>` element in a Next.js project.
    ///
    /// Using the `<img>` element can result in slower Largest Contentful Paint (LCP)
    /// and higher bandwidth usage, as it lacks the optimizations provided by the `<Image />`
    /// component from `next/image`. Next.js's `<Image />` automatically optimizes images
    /// by serving responsive sizes and using modern formats, improving performance and reducing bandwidth.
    ///
    /// If your project is self-hosted, ensure that you have sufficient storage and have
    /// installed the `sharp` package to support optimized images. When deploying to managed
    /// hosting providers, be aware of potential additional costs or usage.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <img alt="Foo" />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <div>
    ///   <img alt="Foo" />
    /// </div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <img />
    /// ```
    ///
    /// ```jsx
    /// <Image src="https://example.com/hero.jpg" />
    /// ```
    ///
    /// ```jsx
    /// <picture>
    ///   <source srcSet="https://example.com/hero.avif" type="image/avif" />
    ///   <source srcSet="https://example.com/hero.webp" type="image/webp" />
    ///   <img src="https://example.com/hero.jpg" />
    /// </picture>
    /// ```
    ///
    pub NoImgElement {
        version: "1.9.4",
        name: "noImgElement",
        language: "jsx",
        sources: &[RuleSource::EslintNext("no-img-element")],
        source_kind: RuleSourceKind::SameLogic,
        recommended: true,
        domains: &[RuleDomain::Next],
    }
}

impl Rule for NoImgElement {
    type Query = Ast<AnyJsxElement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        if node.name().ok()?.name_value_token().ok()?.text_trimmed() != "img"
            || node.attributes().is_empty()
        {
            return None;
        }

        if let AnyJsxElement::JsxSelfClosingElement(jsx) = &node {
            let Some(parent) = jsx.parent::<JsxChildList>() else {
                return Some(());
            };
            let Some(parent) = parent.parent::<JsxElement>() else {
                return Some(());
            };
            let Some(opening_element) = parent.opening_element().ok() else {
                return Some(());
            };
            let name = opening_element.name().ok()?.name_value_token().ok()?;

            if name.text_trimmed() == "picture" {
                return None;
            }
        }

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "Don't use "<Emphasis>"<img>"</Emphasis>" element."
                },
            )
            .note(markup! {
                "Using the "<Emphasis>"<img>"</Emphasis>" can lead to slower LCP and higher bandwidth. Consider using "<Emphasis>"<Image />"</Emphasis>" from "<Emphasis>"next/image"</Emphasis>" to automatically optimize images."
            })
        )
    }
}
