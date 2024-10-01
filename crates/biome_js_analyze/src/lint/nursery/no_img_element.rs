use biome_analyze::RuleSourceKind;
use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_js_syntax::{
    JsSyntaxToken, JsxAttributeList, JsxChildList, JsxElement, JsxOpeningElement,
    JsxSelfClosingElement,
};
use biome_rowan::{declare_node_union, AstNode, AstNodeList, TextRange};

declare_lint_rule! {
    /// Prevent usage of `<img>` element in a Next.js project.
    ///
    /// Using the `<img>` element can result in slower Largest Contentful Paint (LCP)
    /// and higher bandwidth usage, as it lacks the optimizations provided by the `<Image />`
    /// component from `next/image`. Next.js's `<Image />` automatically optimizes images
    /// by serving responsive sizes and using modern formats, improving performance and reducing bandwidth.
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
        version: "next",
        name: "noImgElement",
        language: "jsx",
        sources: &[RuleSource::EslintNext("no-img-element")],
        source_kind: RuleSourceKind::SameLogic,
        recommended: false,
    }
}

declare_node_union! {
    pub NoImgElementQuery = JsxOpeningElement | JsxSelfClosingElement
}

impl Rule for NoImgElement {
    type Query = Ast<NoImgElementQuery>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        node.is_target_valid()?.then(|| node.range())
    }

    fn diagnostic(_: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        return Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "Using "<Emphasis>"<img>"</Emphasis>" could result in slower LCP and higher bandwidth."
                },
            )
            .note(markup! { "Consider using "<Emphasis>"<Image />"</Emphasis>" from "<Emphasis>"next/image"</Emphasis>" to automatically optimize images." })
            .note(markup! { "This may incur additional usage or cost from your provider." })
        );
    }
}

impl NoImgElementQuery {
    fn range(&self) -> TextRange {
        match self {
            NoImgElementQuery::JsxOpeningElement(jsx) => jsx.range(),
            NoImgElementQuery::JsxSelfClosingElement(jsx) => jsx.range(),
        }
    }

    fn name(&self) -> Option<JsSyntaxToken> {
        match self {
            NoImgElementQuery::JsxOpeningElement(jsx) => jsx.name().ok()?.name_value_token(),
            NoImgElementQuery::JsxSelfClosingElement(jsx) => jsx.name().ok()?.name_value_token(),
        }
    }

    fn attributes(&self) -> JsxAttributeList {
        match self {
            NoImgElementQuery::JsxOpeningElement(jsx) => jsx.attributes(),
            NoImgElementQuery::JsxSelfClosingElement(jsx) => jsx.attributes(),
        }
    }

    fn get_grandparent(&self) -> Option<JsxOpeningElement> {
        if let NoImgElementQuery::JsxSelfClosingElement(jsx) = self {
            return jsx
                .parent::<JsxChildList>()?
                .parent::<JsxElement>()?
                .opening_element()
                .ok();
        }
        None
    }

    fn is_target_valid(&self) -> Option<bool> {
        if self.name()?.text_trimmed() != "img" || self.attributes().is_empty() {
            return Some(false);
        }

        if let Some(grandparent) = self.get_grandparent() {
            let name = grandparent.name().ok()?.name_value_token();
            return Some(name.map_or(true, |name| name.text_trimmed() != "picture"));
        }

        Some(true)
    }
}
