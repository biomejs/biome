use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_rowan::AstNode;
use biome_rule_options::use_jsx_img::UseJsxImgOptions;

declare_lint_rule! {
    /// For performance reasons, always provide width and height attributes for `<img>` elements; it will help to prevent layout shifts.
    ///
    /// This rule is intended for use in Qwik applications to ensure `<img>` elements have width and height attributes.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <img src="/image.png">
    /// <img src="/static/images/portrait-01.webp">
    /// <img src="/image.png" width="200">
    /// <img src="/image.png" height="200">
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <img width="200" height="600" src="/static/images/portrait-01.webp">
    /// ```
    pub UseJsxImg {
        version: "1.0.0",
        name: "useJsxImg",
        language: "js",
        sources: &[RuleSource::EslintQwik("jsx-img").inspired()],
        recommended: true,
        severity: Severity::Warning,
        fix_kind: FixKind::None,
        domains: &[RuleDomain::Qwik],
    }
}

pub enum JsxImgDiagnosticKind {
    MissingWidthOrHeight,
}

impl Rule for UseJsxImg {
    type Query = Ast<AnyJsxElement>;
    type State = (biome_rowan::TextRange, JsxImgDiagnosticKind);
    type Signals = Option<Self::State>;
    type Options = UseJsxImgOptions;

    fn run(ctx: &biome_analyze::context::RuleContext<Self>) -> Self::Signals {
        let element = ctx.query();
        let tag_token = element.name().ok()?.name_value_token().ok()?;
        if tag_token.text_trimmed() != "img" {
            return None;
        }
        let has_width = element.find_attribute_by_name("width").is_some();
        let has_height = element.find_attribute_by_name("height").is_some();
        if !has_width || !has_height {
            return Some((element.range(), JsxImgDiagnosticKind::MissingWidthOrHeight));
        }
        None
    }

    fn diagnostic(
        _: &biome_analyze::context::RuleContext<Self>,
        (range, kind): &Self::State,
    ) -> Option<RuleDiagnostic> {
        match kind {
            JsxImgDiagnosticKind::MissingWidthOrHeight => Some(RuleDiagnostic::new(
                rule_category!(),
                range,
                markup!(
                    "For performance reasons, always provide width and height attributes for <Emphasis>img</Emphasis> elements, it will help to prevent layout shifts."
                ),
            )),
        }
    }
}
