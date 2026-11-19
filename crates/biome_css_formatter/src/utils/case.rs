use crate::CssFormatContext;
#[cfg(debug_assertions)]
use crate::CssFormatter;
#[cfg(debug_assertions)]
use biome_css_syntax::CssSyntaxToken;
use biome_css_syntax::{
    AnyCssAtRule, AnyCssQueryFeatureName, AnyCssSelectorIdentifier, AnyCssUnknownAtRuleName,
    CssContainerScrollStateQueryInParens, CssDeclaration, CssFontFeatureValuesItem,
    CssGenericProperty, CssIdentifier, CssIfMediaTest, CssIfSupportsTest, CssImportSupports,
    CssLanguage, CssNestedQualifiedRule, CssPseudoClassFunctionIdentifier,
    CssPseudoClassIdentifier, CssPseudoClassSelector, CssQualifiedRule,
    CssSupportsFeatureDeclaration, CssSyntaxKind, CssSyntaxNode, CssUnknownAtRuleComponentList,
    CssUnknownBlockAtRule, TwPluginAtRule,
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
///   Escaped identifiers preserve their complete source spelling because
///   lowercasing only their unescaped characters would be incomplete.
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

        let identifier = CssIdentifier::cast_ref(item.syntax())?;
        let case = if *self == Self::Lowercase && identifier_has_escape(&identifier) {
            Self::Preserve
        } else {
            *self
        };

        Some(context.replace_identifier_case(case))
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

/// Returns whether an identifier's source spelling contains a CSS escape.
pub(crate) fn identifier_has_escape(identifier: &CssIdentifier) -> bool {
    identifier
        .value_token()
        .is_ok_and(|token| token.text_trimmed().contains('\\'))
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
    if is_dashed_identifier(name) {
        CssCase::Preserve
    } else {
        CssCase::Lowercase
    }
}

/// Matches declarations enclosed by CSS Modules `:import` or `:export` rules.
pub(crate) fn is_css_modules_import_export_declaration(property: &CssGenericProperty) -> bool {
    let Some(declaration) = property.parent::<CssDeclaration>() else {
        return false;
    };

    declaration
        .syntax()
        .ancestors()
        .find_map(CssQualifiedRule::cast)
        .is_some_and(|rule| is_css_modules_import_export_rule(&rule))
}

/// Matches declarations used by `@supports`, import `supports()`, and CSS `if()` tests.
pub(crate) fn is_supports_test_declaration(declaration: &CssDeclaration) -> bool {
    declaration
        .parent::<CssSupportsFeatureDeclaration>()
        .is_some()
        || declaration.parent::<CssImportSupports>().is_some()
        || declaration.parent::<CssIfSupportsTest>().is_some()
}

/// Matches opaque values such as `@plugin "x" { Option: INITIAL; }`.
pub(crate) fn is_author_owned_property_value(property: &CssGenericProperty) -> bool {
    for ancestor in property.syntax().ancestors() {
        if CssFontFeatureValuesItem::can_cast(ancestor.kind())
            || CssUnknownBlockAtRule::can_cast(ancestor.kind())
            || TwPluginAtRule::can_cast(ancestor.kind())
        {
            return true;
        }

        if let Some(rule) = CssQualifiedRule::cast(ancestor.clone()) {
            return is_css_modules_import_export_rule(&rule);
        }

        if AnyCssAtRule::can_cast(ancestor.kind())
            || CssNestedQualifiedRule::can_cast(ancestor.kind())
        {
            return false;
        }
    }

    false
}

fn is_css_modules_import_export_rule(rule: &CssQualifiedRule) -> bool {
    let prelude = rule.prelude();

    prelude
        .syntax()
        .descendants()
        .filter(is_css_modules_import_export_pseudo)
        .filter_map(|node| node.parent())
        .filter_map(CssPseudoClassSelector::cast)
        .any(|selector| {
            selector.syntax().text_trimmed_range() == prelude.syntax().text_trimmed_range()
        })
}

fn is_css_modules_import_export_pseudo(node: &CssSyntaxNode) -> bool {
    if let Some(pseudo) = CssPseudoClassIdentifier::cast(node.clone()) {
        return pseudo.name().is_ok_and(|name| {
            selector_identifier_text_eq(&name, "export")
                || selector_identifier_text_eq(&name, "import")
        });
    }

    CssPseudoClassFunctionIdentifier::cast(node.clone()).is_some_and(|pseudo| {
        pseudo.name().is_ok_and(|name| {
            identifier_text_eq(&name, "export") || identifier_text_eq(&name, "import")
        })
    })
}

fn selector_identifier_text_eq(name: &AnyCssSelectorIdentifier, expected: &str) -> bool {
    matches!(name, AnyCssSelectorIdentifier::CssIdentifier(identifier) if identifier_text_eq(identifier, expected))
}

fn identifier_text_eq(identifier: &CssIdentifier, expected: &str) -> bool {
    identifier
        .value_token()
        .is_ok_and(|token| token.token_text_trimmed() == expected)
}
