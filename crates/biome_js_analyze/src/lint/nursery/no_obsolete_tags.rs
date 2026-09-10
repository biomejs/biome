use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{JsSyntaxToken, jsx_ext::AnyJsxElement};
use biome_rule_options::no_obsolete_tags::NoObsoleteTagsOptions;
use biome_string_case::StrLikeExtension;

declare_lint_rule! {
    /// Disallow obsolete HTML elements.
    ///
    /// Obsolete elements are no longer part of conforming HTML. Use standard HTML
    /// elements for structure and semantics, and CSS for presentation.
    ///
    /// This rule disallows `acronym`, `applet`, `basefont`, `bgsound`, `big`, `blink`,
    /// `center`, `dir`, `font`, `frame`, `frameset`, `isindex`, `keygen`, `listing`,
    /// `marquee`, `menuitem`, `multicol`, `nextid`, `nobr`, `noembed`, `noframes`,
    /// `plaintext`, `rb`, `rtc`, `spacer`, `strike`, `tt`, and `xmp`.
    ///
    /// This rule checks native JSX elements without case sensitivity. Component names
    /// such as `<Font />`, member expressions such as `<UI.font />`, and namespaced
    /// names are ignored.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <font color="red">Warning</font>;
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <acronym title="World Wide Web">WWW</acronym>;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <span className="warning">Warning</span>;
    /// <abbr title="World Wide Web">WWW</abbr>;
    /// <Font />;
    /// <UI.font />;
    /// ```
    ///
    /// ## Resources
    ///
    /// - [HTML specification: Non-conforming features](https://html.spec.whatwg.org/multipage/obsolete.html#non-conforming-features)
    ///
    pub NoObsoleteTags {
        version: "next",
        name: "noObsoleteTags",
        language: "jsx",
        recommended: false,
        severity: Severity::Warning,
        sources: &[RuleSource::HtmlEslint("no-obsolete-tags").inspired()],
    }
}

const OBSOLETE_ELEMENTS: [&str; 28] = [
    "acronym",
    "applet",
    "basefont",
    "bgsound",
    "big",
    "blink",
    "center",
    "dir",
    "font",
    "frame",
    "frameset",
    "isindex",
    "keygen",
    "listing",
    "marquee",
    "menuitem",
    "multicol",
    "nextid",
    "nobr",
    "noembed",
    "noframes",
    "plaintext",
    "rb",
    "rtc",
    "spacer",
    "strike",
    "tt",
    "xmp",
];

impl Rule for NoObsoleteTags {
    type Query = Ast<AnyJsxElement>;
    type State = JsSyntaxToken;
    type Signals = Option<Self::State>;
    type Options = NoObsoleteTagsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let name = ctx.query().name().ok()?.as_jsx_name()?.value_token().ok()?;
        let is_obsolete = OBSOLETE_ELEMENTS
            .binary_search(&name.text_trimmed().to_ascii_lowercase_cow().as_ref())
            .is_ok();
        is_obsolete.then_some(name)
    }

    fn diagnostic(_ctx: &RuleContext<Self>, name: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                name.text_trimmed_range(),
                markup! {
                    "The "<Emphasis>{name.text_trimmed()}</Emphasis>" element is obsolete."
                },
            )
            .note(markup! {
                "Obsolete elements are no longer part of conforming HTML."
            })
            .note(markup! {
                "Use standard HTML elements for structure and semantics, and CSS for presentation."
            }),
        )
    }
}

#[test]
fn obsolete_elements_are_sorted() {
    assert!(OBSOLETE_ELEMENTS.is_sorted_by(|left, right| left < right));
}
