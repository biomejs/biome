use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_css_syntax::CssMediaType;
use biome_rowan::AstNode;
use biome_rule_options::no_deprecated_media_type::NoDeprecatedMediaTypeOptions;
use biome_string_case::StrLikeExtension;

declare_lint_rule! {
    /// Disallow deprecated media types.
    ///
    /// Several media types defined in earlier specifications have been deprecated and should
    /// no longer be used. The deprecated media types are still recognized, but they match nothing.
    ///
    /// For details on media types, see the
    /// [Media Queries Level 5 specification](https://drafts.csswg.org/mediaqueries-5/#media-types).
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// @media tv {}
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// @media handheld and (min-width: 480px) {}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// @media screen {}
    /// ```
    ///
    /// ```css
    /// @media print and (min-resolution: 300dpi) {}
    /// ```
    ///
    /// ## Options
    ///
    /// ### `allow`
    ///
    /// Media types to allow (case-insensitive).
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "allow": ["tv", "speech"]
    ///   }
    /// }
    /// ```
    ///
    /// #### Valid
    ///
    /// ```css,use_options
    /// @media tv {}
    /// @media speech {}
    /// ```
    ///
    pub NoDeprecatedMediaType {
        version: "next",
        name: "noDeprecatedMediaType",
        language: "css",
        recommended: false,
        sources: &[RuleSource::Stylelint("media-type-no-deprecated").same()],
    }
}

impl Rule for NoDeprecatedMediaType {
    type Query = Ast<CssMediaType>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoDeprecatedMediaTypeOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let media_type = node.value().ok().and_then(|v| v.value_token().ok())?;
        let media_type = media_type.text_trimmed();

        // Check allow list from options
        if let Some(allow_list) = &ctx.options().allow {
            for allowed in allow_list {
                if media_type.eq_ignore_ascii_case(allowed) {
                    return None;
                }
            }
        }

        // FIXME: Optimize to avoid allocation
        let media_type = media_type.to_ascii_lowercase_cow();
        if DEPRECATED_MEDIA_TYPES
            .binary_search(&media_type.as_ref())
            .is_ok()
        {
            return Some(());
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let media_type = ctx
            .query()
            .value()
            .ok()
            .and_then(|v| v.value_token().ok())?;
        let media_type = media_type.text_trimmed();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "Unexpected deprecated media type: "<Emphasis>{ media_type }</Emphasis>
                },
            )
            .note(markup! {
                "Deprecated media types are recognized but match nothing; prefer using media features or recommended media types."
            })
            .footer_list(
                markup! {
                    "Recommended media types include:"
                },
                ["all", "print", "screen"],
            ),
        )
    }
}

const DEPRECATED_MEDIA_TYPES: [&str; 8] = [
    "aural",
    "braille",
    "embossed",
    "handheld",
    "projection",
    "speech",
    "tty",
    "tv",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builtin_list_is_sorted() {
        assert!(DEPRECATED_MEDIA_TYPES.is_sorted());
    }
}
