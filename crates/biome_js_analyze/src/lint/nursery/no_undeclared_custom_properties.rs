use crate::services::database::ResolvedImports;
use biome_analyze::{Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_css_parser::{CssParserOptions, parse_css};
use biome_css_semantic::semantic_model;
use biome_css_syntax::{
    CssFunction, decode_css_identifier, property_syntax::custom_property_name_from_var_function,
};
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_js_syntax::{AnyJsxAttributeValue, JsxAttribute, JsxElement};
use biome_languages::{CssFileSource, css::CssEmbeddingKind};
use biome_module_graph::{SymbolFromModuleInfo, css_property_definitions};
use biome_rowan::{AstNode, TextRange, TextSize, TokenText};
use biome_rule_options::no_undeclared_custom_properties::NoUndeclaredCustomPropertiesOptions;

declare_lint_rule! {
    /// Reports custom properties used with `var()` that have no visible declaration.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <div style="color: var(--text-color)" />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <div style="--text-color: blue; color: var(--text-color)" />
    /// ```
    ///
    pub NoUndeclaredCustomProperties {
        version: "2.5.11",
        name: "noUndeclaredCustomProperties",
        language: "js",
        recommended: false,
        domains: &[RuleDomain::Project],
    }
}

impl Rule for NoUndeclaredCustomProperties {
    type Query = ResolvedImports<JsxAttribute>;
    type State = UndeclaredCustomProperty;
    type Signals = Vec<Self::State>;
    type Options = NoUndeclaredCustomPropertiesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let Some((text, file_start)) = inline_style_text(ctx.query()) else {
            return Vec::new();
        };
        let db = ctx.db();
        let Some(module) = db.module_for_path(ctx.file_path()) else {
            return Vec::new();
        };
        let properties = inline_custom_properties(text.text());

        properties
            .references
            .into_iter()
            .filter_map(|(range, name)| {
                let decoded_name = decode_css_identifier(name.text());
                if properties.definitions.iter().any(|definition| {
                    decode_css_identifier(definition.text()) == decoded_name
                }) || ancestor_defines_custom_property(ctx.query(), decoded_name.as_ref())
                    || !css_property_definitions(
                    db,
                    SymbolFromModuleInfo::new(db, decoded_name.as_ref(), module),
                )
                .is_empty()
                {
                    return None;
                }
                Some(UndeclaredCustomProperty {
                    range: range.checked_add(file_start)?,
                    name,
                })
            })
            .collect()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(diagnostic(state.range, state.name.text()))
    }
}

pub struct UndeclaredCustomProperty {
    range: TextRange,
    name: TokenText,
}

fn diagnostic(range: TextRange, name: &str) -> RuleDiagnostic {
    RuleDiagnostic::new(
        rule_category!(),
        range,
        markup! { "The custom property " <Emphasis>{name}</Emphasis> " is not defined." },
    )
    .note(markup! {
        "An undeclared custom property can indicate a misspelled name or a missing declaration."
    })
    .note(markup! {
        "Define the custom property in an available stylesheet, rename it or remove it."
    })
}

fn inline_style_text(attribute: &JsxAttribute) -> Option<(TokenText, TextSize)> {
    if attribute
        .syntax()
        .ancestors()
        .find_map(AnyJsxElement::cast)
        .is_some_and(|element| element.is_custom_component())
    {
        return None;
    }
    if attribute.name_value_token().ok()?.text_trimmed() != "style" {
        return None;
    }
    let AnyJsxAttributeValue::JsxString(value) = attribute.initializer()?.value().ok()? else {
        return None;
    };
    let value_token = value.value_token().ok()?;
    let text = value.inner_string_text().ok()?;
    let content_range = text.source_range(value_token.text_range());
    Some((text, content_range.start()))
}

struct InlineCustomProperties {
    definitions: Vec<TokenText>,
    references: Vec<(TextRange, TokenText)>,
}

fn inline_custom_properties(value: &str) -> InlineCustomProperties {
    let root = parse_css(
        value,
        CssFileSource::css().with_embedding_kind(CssEmbeddingKind::HtmlStyleAttribute),
        CssParserOptions::default(),
    )
    .tree();
    let definitions = semantic_model(&root)
        .custom_property_declarations()
        .map(|declaration| declaration.name().clone())
        .collect();
    let references = root
        .syntax()
        .descendants()
        .filter_map(CssFunction::cast)
        .filter_map(|function| {
            let property = custom_property_name_from_var_function(&function)?;
            Some((
                property.text_trimmed_range(),
                property.token_text_trimmed(),
            ))
        })
        .collect();
    InlineCustomProperties {
        definitions,
        references,
    }
}

fn ancestor_defines_custom_property(attribute: &JsxAttribute, name: &str) -> bool {
    let current = attribute
        .syntax()
        .ancestors()
        .find_map(AnyJsxElement::cast);
    attribute
        .syntax()
        .ancestors()
        .filter_map(JsxElement::cast)
        .filter_map(|element| element.opening_element().ok())
        .filter(|element| {
            current
                .as_ref()
                .is_none_or(|current| current.syntax() != element.syntax())
        })
        .flat_map(|element| element.attributes())
        .filter_map(|attribute| attribute.as_jsx_attribute().cloned())
        .filter_map(|attribute| inline_style_text(&attribute))
        .flat_map(|(text, _)| inline_custom_properties(text.text()).definitions)
        .any(|definition| decode_css_identifier(definition.text()) == name)
}
