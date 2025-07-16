use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_rowan::AstNode;
use biome_rule_options::use_jsx_anchor_href::UseJsxAnchorHrefOptions;

declare_lint_rule! {
    /// Require `href` attribute for `<a>` elements in JSX.
    ///
    /// This rule is intended for use in Qwik applications to ensure `<a>` elements have an `href` attribute.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <a>Link</a>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <a target="_blank">External</a>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <a href="/home">Home</a>
    /// ```
    ///
    /// ```jsx
    /// <a href="https://example.com" target="_blank">External</a>
    /// ```
    pub UseJsxAnchorHref {
        version: "next",
        name: "useJsxAnchorHref",
        language: "jsx",
        sources: &[RuleSource::EslintQwik("jsx-a").inspired()],
        recommended: true,
        severity: Severity::Warning,
        fix_kind: FixKind::None,
        domains: &[RuleDomain::Qwik],
    }
}

pub enum JsxAnchorHrefDiagnosticKind {
    MissingHref,
}

impl Rule for UseJsxAnchorHref {
    type Query = Ast<AnyJsxElement>;
    type State = (biome_rowan::TextRange, JsxAnchorHrefDiagnosticKind);
    type Signals = Option<Self::State>;
    type Options = UseJsxAnchorHrefOptions;

    fn run(ctx: &biome_analyze::context::RuleContext<Self>) -> Self::Signals {
        let element = ctx.query();
        let tag_token = element.name().ok()?.name_value_token().ok()?;
        if tag_token.text_trimmed() != "a" {
            return None;
        }
        let has_href = element.find_attribute_by_name("href").is_some();
        if !has_href {
            return Some((element.range(), JsxAnchorHrefDiagnosticKind::MissingHref));
        }
        None
    }

    fn diagnostic(
        _: &biome_analyze::context::RuleContext<Self>,
        (range, kind): &Self::State,
    ) -> Option<RuleDiagnostic> {
        match kind {
            JsxAnchorHrefDiagnosticKind::MissingHref => Some(RuleDiagnostic::new(
                rule_category!(),
                range,
                markup!(
                    "<Emphasis>a</Emphasis> elements must have an <Emphasis>href</Emphasis> attribute."
                ),
            )),
        }
    }
}
