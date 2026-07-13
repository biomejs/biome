use crate::CssFormatContext;
#[cfg(debug_assertions)]
use crate::CssFormatter;
#[cfg(debug_assertions)]
use biome_css_syntax::CssSyntaxToken;
use biome_css_syntax::{
    AnyCssQueryFeatureName, AnyCssSelectorIdentifier, CssContainerScrollStateQueryInParens,
    CssIdentifier, CssIfMediaTest, CssLanguage, CssQualifiedRule, CssSyntaxKind, CssSyntaxNode,
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
/// - Use [`CssCase::Preserve`] for author or externally owned text, including
///   custom identifiers, Sass names and interpolation, framework names, and
///   unknown future syntax, such as `--Brand`, `$Theme`, or `#{$Name}`.
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

/// Lowercases `INITIAL`, `INHERIT`, `UNSET`, and `REVERT` in value position.
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

/// Matches identifiers beginning with `--`.
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

    if is_dashed_identifier(name) || is_front_matter_selector(name) {
        CssCase::Preserve
    } else {
        CssCase::Lowercase
    }
}

/// Matches YAML front matter recovered as a selector beginning with `---`.
fn is_front_matter_selector(name: &CssIdentifier) -> bool {
    name.syntax()
        .ancestors()
        .find_map(CssQualifiedRule::cast)
        .is_some_and(|rule| rule.prelude().syntax().text_trimmed().starts_with("---"))
}
