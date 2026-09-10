use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::{HtmlSyntaxKind, HtmlSyntaxToken, T, element_ext::AnyHtmlTagElement};
use biome_parser::{TokenSet, token_set};
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
    /// Tag names are case-insensitive in HTML files. Custom components in template
    /// languages, such as `<Font />`, are not HTML elements and are ignored.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <font color="red">Warning</font>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <acronym title="World Wide Web">WWW</acronym>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <span class="warning">Warning</span>
    /// <abbr title="World Wide Web">WWW</abbr>
    /// ```
    ///
    /// ## Resources
    ///
    /// - [HTML specification: Non-conforming features](https://html.spec.whatwg.org/multipage/obsolete.html#non-conforming-features)
    ///
    pub NoObsoleteTags {
        version: "next",
        name: "noObsoleteTags",
        language: "html",
        recommended: false,
        severity: Severity::Warning,
        sources: &[RuleSource::HtmlEslint("no-obsolete-tags").same()],
    }
}

impl Rule for NoObsoleteTags {
    type Query = Ast<AnyHtmlTagElement>;
    type State = HtmlSyntaxToken;
    type Signals = Option<Self::State>;
    type Options = NoObsoleteTagsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let name = ctx.query().name_value_token()?;
        let is_obsolete = OBSOLETE_ELEMENTS.contains(name.kind())
            || (name.kind() == HtmlSyntaxKind::HTML_UNKNOWN_TAG
                && OBSOLETE_ELEMENTS_WITHOUT_KEYWORDS
                    .binary_search(&name.text_trimmed().to_ascii_lowercase_cow().as_ref())
                    .is_ok());
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

const OBSOLETE_ELEMENTS: TokenSet<HtmlSyntaxKind> = token_set!(
    T![acronym],
    T![basefont],
    T![bgsound],
    T![big],
    T![blink],
    T![center],
    T![dir],
    T![font],
    T![frame],
    T![frameset],
    T![keygen],
    T![marquee],
    T![menuitem],
    T![nobr],
    T![noembed],
    T![noframes],
    T![plaintext],
    T![rb],
    T![rtc],
    T![strike],
    T![tt],
    T![xmp]
);

const OBSOLETE_ELEMENTS_WITHOUT_KEYWORDS: [&str; 6] = [
    "applet", "isindex", "listing", "multicol", "nextid", "spacer",
];

#[cfg(test)]
mod tests {
    use super::OBSOLETE_ELEMENTS_WITHOUT_KEYWORDS;

    #[test]
    fn obsolete_elements_without_keywords_are_sorted() {
        assert!(OBSOLETE_ELEMENTS_WITHOUT_KEYWORDS.is_sorted_by(|left, right| left < right));
    }
}
