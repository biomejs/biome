use std::sync::Arc;

use biome_analyze::{Rule, context::RuleContext};
use biome_aria::event_handlers::matches_event_handler;
use biome_js_syntax::{AnyJsxAttributeName, JsxAttribute, jsx_ext::AnyJsxElement};
use biome_languages::JsFileSource;
use biome_package::PackageJson;
use biome_rowan::AstNodeList;
use camino::Utf8PathBuf;

/// The spelling of the tab index attribute expected by the JSX framework of the analyzed file.
///
/// Astro templates use the HTML spelling. Elsewhere, the framework is detected from the
/// dependencies of the nearest `package.json`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum TabindexSpelling {
    /// `tabIndex`, the DOM property name used by React and the default for JSX.
    CamelCase,
    /// `tabindex`, the HTML attribute name used by Astro and Solid.
    ///
    /// HTML attribute names are case-insensitive and Solid 1 also accepts `tabIndex`, so both
    /// spellings are recognized.
    Lowercase,
}

impl TabindexSpelling {
    pub(crate) fn from_context<R: Rule + 'static>(ctx: &RuleContext<R>) -> Self {
        let is_astro_template = ctx
            .source_type::<JsFileSource>()
            .as_embedding_kind()
            .is_astro_template();
        let is_solid = || {
            ctx.get_service::<Option<(Utf8PathBuf, Arc<PackageJson>)>>()
                .and_then(Option::as_ref)
                .is_some_and(|(_, manifest)| manifest.contains_dependency("solid-js"))
        };
        if is_astro_template || is_solid() {
            Self::Lowercase
        } else {
            Self::CamelCase
        }
    }

    /// The attribute name to suggest in diagnostics and code actions.
    pub(crate) const fn attribute_name(self) -> &'static str {
        match self {
            Self::CamelCase => "tabIndex",
            Self::Lowercase => "tabindex",
        }
    }

    /// Returns the tab index attribute of `element`, if any.
    pub(crate) fn find_attribute(self, element: &AnyJsxElement) -> Option<JsxAttribute> {
        match self {
            Self::CamelCase => element.find_attribute_by_name("tabIndex"),
            Self::Lowercase => element
                .find_attribute_by_name("tabindex")
                .or_else(|| element.find_attribute_by_name("tabIndex")),
        }
    }
}

/// Check the element is hidden from screen reader.
///
/// Ref:
/// - https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Attributes/aria-hidden
/// - https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/hidden
/// - https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/v6.10.0/src/util/isHiddenFromScreenReader.js
pub(crate) fn is_hidden_from_screen_reader(element: &AnyJsxElement) -> bool {
    let is_aria_hidden = element.has_truthy_attribute("aria-hidden");
    if is_aria_hidden {
        return true;
    }

    match element.name_value_token().ok() {
        Some(name) if name.text_trimmed() == "input" => element
            .find_attribute_by_name("type")
            .and_then(|attribute| attribute.as_static_value())
            .and_then(|value| value.as_string_constant().map(|value| value == "hidden"))
            .unwrap_or_default(),
        _ => false,
    }
}

/// Check if the element is `contentEditable`
///
/// Ref:
/// - https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/contenteditable
/// - https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/v6.10.0/src/util/isContentEditable.js
pub(crate) fn is_content_editable(element: &AnyJsxElement) -> bool {
    element
        .find_attribute_by_name("contentEditable")
        .and_then(|attribute| attribute.as_static_value())
        .and_then(|value| value.as_string_constant().map(|value| value == "true"))
        .unwrap_or_default()
}

/// Check if the element contains event handler
pub fn has_event_handler(handler_types: &[&str], element: &AnyJsxElement) -> bool {
    element.attributes().iter().any(|attribute| {
        if let Some(jsx_attribute) = attribute.as_jsx_attribute()
            && let Ok(AnyJsxAttributeName::JsxName(name)) = jsx_attribute.name()
            && let Ok(value_token) = name.value_token()
            && matches_event_handler(handler_types, value_token.text_trimmed())
        {
            return !jsx_attribute.is_value_null_or_undefined();
        }

        false
    })
}
