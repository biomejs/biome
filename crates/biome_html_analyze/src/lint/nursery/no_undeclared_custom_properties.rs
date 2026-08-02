use crate::services::module_graph::HtmlModuleGraph;
use biome_analyze::{Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_css_parser::{CssParserOptions, parse_css};
use biome_css_syntax::{
    CssFunction, CssGenericProperty, decode_css_identifier,
    property_syntax::custom_property_name_from_var_function,
};
use biome_html_syntax::{AnyHtmlAttributeInitializer, HtmlAttribute, HtmlElement, HtmlRoot};
use biome_languages::CssFileSource;
use biome_module_graph::{SymbolFromModuleInfo, css_property_definitions};
use biome_rowan::{AstNode, AstNodeList, TextRange, TextSize, TokenText};
use biome_rule_options::no_undeclared_custom_properties::NoUndeclaredCustomPropertiesOptions;
use biome_string_case::StrOnlyExtension;
use std::collections::HashMap;

declare_lint_rule! {
    /// Reports custom properties used in inline styles that have no visible definition.
    ///
    /// This rule checks static `style` attributes against custom properties defined in `<style>`
    /// blocks, linked stylesheets, and styles available from parent components.
    ///
    /// Locally scoped styles in Vue, Svelte, and Astro are visible only within their component.
    /// Global styles can provide custom properties to imported child components.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,ignore
    /// <div style="background: var(--custom-bg)"></div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html,ignore
    /// <style>:root { --custom-bg: blue; }</style>
    /// <div style="background: var(--custom-bg)"></div>
    /// ```
    ///
    pub NoUndeclaredCustomProperties {
        version: "next",
        name: "noUndeclaredCustomProperties",
        language: "html",
        recommended: false,
        domains: &[RuleDomain::Project],
    }
}

impl Rule for NoUndeclaredCustomProperties {
    type Query = HtmlModuleGraph<HtmlRoot>;
    type State = UndeclaredCustomProperty;
    type Signals = Vec<Self::State>;
    type Options = NoUndeclaredCustomPropertiesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let db = ctx.db();
        let Some(module) = db.module_for_path(ctx.file_path()) else {
            return Vec::new();
        };

        let styles = ctx
            .query()
            .syntax()
            .descendants()
            .filter_map(HtmlAttribute::cast)
            .filter_map(|attribute| {
                let (text, file_start) = inline_style_text(&attribute)?;
                Some(StyleAttribute {
                    attribute,
                    file_start,
                    properties: inline_custom_properties(text.text()),
                })
            })
            .collect::<Vec<_>>();
        let style_indices = styles
            .iter()
            .enumerate()
            .map(|(index, style)| (style.attribute.range(), index))
            .collect::<HashMap<_, _>>();

        styles
            .iter()
            .flat_map(|style| {
                style.properties.references.iter().filter_map(|(range, name)| {
                    let decoded_name = decode_css_identifier(name.text());
                    if style
                        .properties
                        .definitions
                        .iter()
                        .any(|definition| decode_css_identifier(definition.text()) == decoded_name)
                        || ancestor_inline_defines(
                            &style.attribute,
                            &decoded_name,
                            &styles,
                            &style_indices,
                        )
                    {
                        return None;
                    }
                    css_property_definitions(
                        db,
                        SymbolFromModuleInfo::new(db, decoded_name.as_ref(), module),
                    )
                    .is_empty()
                    .then_some(UndeclaredCustomProperty {
                        range: range.checked_add(style.file_start)?,
                        name: name.clone(),
                    })
                })
            })
            .collect()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let name = state.name();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! { "The custom property " <Emphasis>{name}</Emphasis> " is not defined." },
            )
            .note(markup! {
                "An undefined custom property uses its fallback when present; without one, the declaration is invalid."
            })
            .note(markup! {
                "Define the custom property in an available stylesheet or correct its name."
            }),
        )
    }
}

struct StyleAttribute {
    attribute: HtmlAttribute,
    file_start: TextSize,
    properties: InlineCustomProperties,
}

fn ancestor_inline_defines(
    attribute: &HtmlAttribute,
    name: &str,
    styles: &[StyleAttribute],
    style_indices: &HashMap<TextRange, usize>,
) -> bool {
    attribute
        .syntax()
        .ancestors()
        .filter_map(HtmlElement::cast)
        .skip(1)
        .filter_map(|element| element.opening_element().ok())
        .flat_map(|opening| opening.attributes().iter())
        .filter_map(|attribute| attribute.as_html_attribute().cloned())
        .filter_map(|attribute| style_indices.get(&attribute.range()))
        .any(|index| {
            styles[*index]
                .properties
                .definitions
                .iter()
                .any(|definition| decode_css_identifier(definition.text()).as_ref() == name)
        })
}

pub struct UndeclaredCustomProperty {
    range: TextRange,
    name: TokenText,
}

impl UndeclaredCustomProperty {
    fn name(&self) -> &str {
        self.name.text()
    }
}

fn inline_style_text(attribute: &HtmlAttribute) -> Option<(TokenText, TextSize)> {
    let name = attribute.name().ok()?.value_token().ok()?;
    if name.text_trimmed().to_lowercase_cow() != "style" {
        return None;
    }
    let AnyHtmlAttributeInitializer::HtmlString(value) = attribute.initializer()?.value().ok()? else {
        return None;
    };
    let token = value.value_token().ok()?;
    let quote_offset = matches!(token.text_trimmed().as_bytes().first(), Some(b'\'' | b'"'))
        .then_some(TextSize::from(1))
        .unwrap_or_default();
    let start = token.text_trimmed_range().start().checked_add(quote_offset)?;
    Some((value.inner_string_text().ok()?, start))
}

struct InlineCustomProperties {
    definitions: Vec<TokenText>,
    references: Vec<(TextRange, TokenText)>,
}

fn inline_custom_properties(value: &str) -> InlineCustomProperties {
    const PREFIX: &str = "a{";
    let prefix_len = TextSize::of(PREFIX);
    let value_len = TextSize::of(value);
    let mut source = String::with_capacity(PREFIX.len() + value.len() + 1);
    source.push_str(PREFIX);
    source.push_str(value);
    source.push('}');

    let root = parse_css(
        &source,
        CssFileSource::css(),
        CssParserOptions::default(),
    )
    .tree();
    let definitions = root
        .syntax()
        .descendants()
        .filter_map(CssGenericProperty::cast)
        .filter_map(|property| {
            property
                .name()
                .ok()?
                .as_any_css_dashed_identifier()?
                .as_css_dashed_identifier()?
                .value_token()
                .ok()
                .map(|token| token.token_text_trimmed())
        })
        .collect();
    let references = root
        .syntax()
        .descendants()
        .filter_map(CssFunction::cast)
        .filter_map(|function| {
            let property = custom_property_name_from_var_function(&function)?;
            let range = property.range().checked_sub(prefix_len)?;
            if range.end() > value_len {
                return None;
            }
            let name = property.value_token().ok()?.token_text_trimmed();
            Some((range, name))
        })
        .collect();
    InlineCustomProperties {
        definitions,
        references,
    }
}
