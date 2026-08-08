use crate::services::module_graph::HtmlModuleGraph;
use biome_analyze::{Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_css_parser::{CssParserOptions, parse_css};
use biome_css_semantic::semantic_model;
use biome_css_syntax::{
    CssFunction, decode_css_identifier, property_syntax::custom_property_name_from_var_function,
};
use biome_html_syntax::{AnyHtmlAttributeInitializer, HtmlAttribute};
use biome_languages::{CssFileSource, css::CssEmbeddingKind};
use biome_module_graph::{SymbolFromModuleInfo, css_property_definitions};
use biome_rowan::{AstNode, TextRange, TextSize, TokenText};
use biome_rule_options::no_undeclared_custom_properties::NoUndeclaredCustomPropertiesOptions;
use biome_string_case::StrOnlyExtension;

declare_lint_rule! {
    /// Reports custom properties used in static HTML `style` attributes that have no declaration.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <div style="color: var(--text-color)"></div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <style>:root { --text-color: blue; }</style>
    /// <div style="color: var(--text-color)"></div>
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
    type Query = HtmlModuleGraph<HtmlAttribute>;
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
                if properties
                    .definitions
                    .iter()
                    .any(|definition| decode_css_identifier(definition.text()) == decoded_name)
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

fn inline_style_text(attribute: &HtmlAttribute) -> Option<(TokenText, TextSize)> {
    let name = attribute.name().ok()?.value_token().ok()?;
    if name.text_trimmed().to_lowercase_cow() != "style" {
        return None;
    }
    let AnyHtmlAttributeInitializer::HtmlString(value) = attribute.initializer()?.value().ok()?
    else {
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
            Some((property.text_trimmed_range(), property.token_text_trimmed()))
        })
        .collect();
    InlineCustomProperties {
        definitions,
        references,
    }
}
