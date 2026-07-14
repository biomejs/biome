use crate::CssFormatContext;
#[cfg(debug_assertions)]
use crate::CssFormatter;
#[cfg(debug_assertions)]
use biome_css_syntax::CssSyntaxToken;
use biome_css_syntax::{
    AnyCssQueryFeatureName, AnyCssSelectorIdentifier, AnyCssUnknownAtRuleName,
    CssContainerScrollStateQueryInParens, CssIdentifier, CssIfMediaTest, CssLanguage,
    CssSyntaxKind, CssSyntaxNode, CssUnknownAtRuleComponentList,
};
#[cfg(debug_assertions)]
use biome_formatter::Buffer;
use biome_formatter::{FormatScopedOptions, TextCase};
use biome_rowan::AstNode as _;

/// Casing policy for CSS tokens and identifiers.
///
/// Choose by ownership:
///
/// - Use [`CssCase::Lowercase`] for case-insensitive, syntax-owned names that
///   the formatter canonicalizes, such as `@IMPORT` or `:HOVER`.
/// - Use [`CssCase::Preserve`] for author-owned text, including custom
///   identifiers, Sass names and interpolation, such as `--Brand`, `$Theme`,
///   or `#{$Name}`.
/// - Leave [`CssCase::Auto`] only as the default that exposes a missing decision
///   in formatter tests. Do not select it explicitly at a formatting callsite.
pub(crate) type CssCase = TextCase;

impl<T> FormatScopedOptions<CssFormatContext, T> for CssCase
where
    T: biome_rowan::AstNode<Language = CssLanguage>,
{
    type Restore = Option<Self>;

    fn enter(&self, item: &T, context: &mut CssFormatContext) -> Self::Restore {
        debug_assert!(
            T::can_cast(CssSyntaxKind::CSS_IDENTIFIER),
            "CSS identifier case requires an identifier-capable node"
        );

        if CssIdentifier::can_cast(item.syntax().kind()) {
            Some(context.replace_identifier_case(*self))
        } else {
            None
        }
    }

    fn exit(&self, restore: Self::Restore, context: &mut CssFormatContext) {
        if let Some(previous) = restore {
            context.replace_identifier_case(previous);
        }
    }
}

#[cfg(debug_assertions)]
pub(crate) fn record_auto_identifier(node: &CssIdentifier, f: &mut CssFormatter) {
    let parent = node.syntax().parent().map(|parent| parent.kind());
    f.state_mut().record_audit_event(std::format!(
        "CSS formatter used an unclassified case policy: identifier `{}` under {parent:?}",
        node.syntax().text_trimmed()
    ));
}

#[cfg(debug_assertions)]
pub(crate) fn record_auto_contextual_token(token: &CssSyntaxToken, f: &mut CssFormatter) {
    if !token.kind().is_contextual_keyword() {
        return;
    }

    let parent = token.parent().map(|parent| parent.kind());
    f.state_mut().record_audit_event(std::format!(
        "CSS formatter used an unclassified case policy: token `{}` ({:?}) under {parent:?}",
        token.text_trimmed(),
        token.kind()
    ));
}

/// Lowercases CSS-wide keywords such as `INITIAL` and `REVERT` in values.
pub(crate) fn value_identifier_case(value: &CssIdentifier) -> CssCase {
    if is_lowercase_value_keyword(value) {
        CssCase::Lowercase
    } else {
        CssCase::Preserve
    }
}

/// Recognizes CSS-wide keywords normalized in value position, such as `INITIAL`.
fn is_lowercase_value_keyword(value: &CssIdentifier) -> bool {
    value.value_token().is_ok_and(|token| {
        let text = token.token_text_trimmed();

        text.eq_ignore_ascii_case("initial")
            || text.eq_ignore_ascii_case("inherit")
            || text.eq_ignore_ascii_case("unset")
            || text.eq_ignore_ascii_case("revert")
    })
}

/// Preserves custom, interpolated, and colon-prefixed unknown at-rule names.
pub(crate) fn unknown_at_rule_name_case(
    name: &AnyCssUnknownAtRuleName,
    components: Option<&CssUnknownAtRuleComponentList>,
) -> CssCase {
    match name.as_css_identifier() {
        None => CssCase::Preserve,
        Some(identifier)
            if is_dashed_identifier(identifier)
                || components.is_some_and(unknown_components_start_with_colon) =>
        {
            CssCase::Preserve
        }
        Some(_) => CssCase::Lowercase,
    }
}

fn unknown_components_start_with_colon(components: &CssUnknownAtRuleComponentList) -> bool {
    components
        .syntax()
        .first_token()
        .is_some_and(|token| token.kind() == CssSyntaxKind::COLON)
}

/// Lowercases standard query features such as `(MIN-WIDTH: 1PX)`.
///
/// Dashed names, `scroll-state((STUCK))`, and
/// `if(media(WIDTH > 1PX): ...)` preserve source casing.
pub(crate) fn query_feature_name_case(name: &AnyCssQueryFeatureName) -> CssCase {
    match name.as_css_identifier() {
        None => CssCase::Preserve,
        Some(identifier) if is_dashed_identifier(identifier) => CssCase::Preserve,
        Some(identifier) if is_preserved_query_feature_context(identifier.syntax()) => {
            CssCase::Preserve
        }
        Some(_) => CssCase::Lowercase,
    }
}

/// Matches `scroll-state()` and CSS `if(media())` query contexts.
fn is_preserved_query_feature_context(node: &CssSyntaxNode) -> bool {
    node.ancestors().any(|ancestor| {
        CssContainerScrollStateQueryInParens::can_cast(ancestor.kind())
            || CssIfMediaTest::can_cast(ancestor.kind())
    })
}

/// Matches identifiers beginning with a literal `--`.
fn is_dashed_identifier(identifier: &CssIdentifier) -> bool {
    identifier
        .value_token()
        .is_ok_and(|token| token.token_text_trimmed().starts_with("--"))
}

/// Returns the casing policy for pseudo names such as `:--STATE`.
pub(crate) fn pseudo_name_case(name: &AnyCssSelectorIdentifier) -> CssCase {
    let Some(name) = name.as_css_identifier() else {
        return CssCase::Preserve;
    };

    pseudo_identifier_case(name)
}

/// Returns the casing policy for identifier-backed pseudo names.
pub(crate) fn pseudo_identifier_case(name: &CssIdentifier) -> CssCase {
    if is_dashed_identifier(name) || has_hex_escape(name) {
        CssCase::Preserve
    } else {
        CssCase::Lowercase
    }
}

/// Prettier preserves pseudo names containing hexadecimal escapes.
fn has_hex_escape(name: &CssIdentifier) -> bool {
    name.value_token().is_ok_and(|token| {
        token
            .token_text_trimmed()
            .as_bytes()
            .windows(2)
            .any(|pair| pair[0] == b'\\' && pair[1].is_ascii_hexdigit())
    })
}
