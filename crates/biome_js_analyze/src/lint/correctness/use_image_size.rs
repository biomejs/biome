use biome_analyze::{Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_rowan::AstNode;
use biome_rule_options::use_image_size::UseImageSizeOptions;

declare_lint_rule! {
    /// Enforces that `<img>` elements have both width and height attributes.
    ///
    /// This rule ensures that `<img>` elements have `width` and `height` attributes.
    ///
    /// Images without specified width and height can cause layout shifts as the browser does not know how much space to reserve for them, leading to a poor user experience.
    /// It's recommended to always include these attributes to prevent such issues.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <img src="/image.png"/>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <img src="/static/images/portrait-01.webp"/>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <img src="/image.png" width="200"/>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <img src="/image.png" height="200"/>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <img width="200" height="600" src="/static/images/portrait-01.webp" />
    /// ```
    ///
    /// ```jsx
    /// <img width="100" height="100" src="https://example.com/image.png" />
    /// ```
    pub UseImageSize {
        version: "2.1.4",
        name: "useImageSize",
        language: "jsx",
        sources: &[RuleSource::EslintQwik("jsx-img").same()],
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Qwik],
    }
}

impl Rule for UseImageSize {
    type Query = Ast<AnyJsxElement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseImageSizeOptions;

    fn run(ctx: &biome_analyze::context::RuleContext<Self>) -> Self::Signals {
        let element = ctx.query();
        let tag_token = element.name().ok()?.name_value_token().ok()?;
        if tag_token.text_trimmed() != "img" {
            return None;
        }
        let has_width = element.find_attribute_by_name("width").is_some();
        let has_height = element.find_attribute_by_name("height").is_some();
        if !has_width || !has_height {
            return Some(());
        }
        None
    }

    fn diagnostic(
        ctx: &biome_analyze::context::RuleContext<Self>,
        _state: &Self::State,
    ) -> Option<RuleDiagnostic> {
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            ctx.query().range(),
            markup! {
                "Missing "<Emphasis>"width"</Emphasis>" or "<Emphasis>"height"</Emphasis>" attribute on "<Emphasis>"img"</Emphasis>" element."
            },
        )
        .note(markup! {
            "Without explicit dimensions, images cause layout shifts (CLS) when loading, harming user experience and SEO."
        })
        .note(markup! {
            "Learn why this matters: "
            <Hyperlink href="https://web.dev/optimize-cls/#images-without-dimensions">"web.dev: Image Dimensions"</Hyperlink>", "
            <Hyperlink href="https://developer.mozilla.org/en-US/docs/Web/HTML/Element/img#attributes">"MDN: img attributes"</Hyperlink>
        });
        Some(diagnostic)
    }
}
