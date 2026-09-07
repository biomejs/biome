use crate::model::SemanticModel;
use crate::semantic_model;
use biome_css_syntax::selector_ext::AnyCssPseudoClassFunctionSelector;
use biome_css_syntax::{
    AnyCssRoot, CssComplexSelector, CssCompoundSelector, CssNestedQualifiedRule,
    CssPseudoClassIdentifier, CssQualifiedRule, CssSyntaxNode, decode_css_identifier,
};
use biome_parser::AnyParsedSource;
use biome_rowan::{AstNode, AstNodeList, TextRange, TokenText};

/// The name and source range of a custom property definition.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CssPropertyDefinition {
    name: TokenText,
    range: TextRange,
    globally_scoped: bool,
}

impl CssPropertyDefinition {
    pub fn matches(&self, name: &str) -> bool {
        decode_css_identifier(self.name.text()) == decode_css_identifier(name)
    }

    pub fn name_token(&self) -> &TokenText {
        &self.name
    }

    pub fn range(&self) -> TextRange {
        self.range
    }

    pub fn is_globally_scoped(&self) -> bool {
        self.globally_scoped
    }
}

fn collect_property_definitions(model: &SemanticModel) -> Vec<CssPropertyDefinition> {
    let mut definitions = model
        .custom_property_declarations()
        .map(|declaration| {
            let property = declaration.property();
            CssPropertyDefinition {
                name: declaration.name().clone(),
                range: declaration.range(),
                globally_scoped: is_globally_scoped(property.syntax()),
            }
        })
        .chain(
            model
                .global_custom_variables()
                .at_property_registration_candidates()
                .map(|property| CssPropertyDefinition {
                    name: property.name().clone(),
                    range: property.range(),
                    globally_scoped: true,
                }),
        )
        .collect::<Vec<_>>();
    definitions.sort_unstable_by_key(|definition| definition.range.start());
    definitions
}

fn is_globally_scoped(property: &CssSyntaxNode) -> bool {
    let mut containing_rules = property.ancestors().filter_map(|ancestor| {
        if let Some(rule) = CssQualifiedRule::cast(ancestor.clone()) {
            Some(has_standalone_global_selector(rule.prelude().syntax()))
        } else {
            CssNestedQualifiedRule::cast(ancestor)
                .map(|rule| has_standalone_global_selector(rule.prelude().syntax()))
        }
    });
    containing_rules
        .next()
        .is_some_and(|first| first && containing_rules.all(|is_global| is_global))
}

fn has_standalone_global_selector(prelude: &CssSyntaxNode) -> bool {
    prelude
        .descendants()
        .filter_map(AnyCssPseudoClassFunctionSelector::cast)
        .any(|selector| selector.is_global_pseudo() && is_standalone_selector(selector.syntax()))
        || prelude
            .descendants()
            .filter_map(CssPseudoClassIdentifier::cast)
            .any(|selector| {
                is_global_pseudo_identifier(&selector) && is_standalone_selector(selector.syntax())
            })
}

fn is_standalone_selector(selector: &CssSyntaxNode) -> bool {
    if selector.ancestors().any(|ancestor| {
        ancestor != *selector && AnyCssPseudoClassFunctionSelector::can_cast(ancestor.kind())
    }) {
        return false;
    }
    let Some(compound) = selector.ancestors().find_map(CssCompoundSelector::cast) else {
        return false;
    };
    compound.nesting_selectors().is_empty()
        && compound.simple_selector().is_none()
        && compound.sub_selectors().len() == 1
        && !compound
            .syntax()
            .ancestors()
            .any(|ancestor| CssComplexSelector::can_cast(ancestor.kind()))
}

fn is_global_pseudo_identifier(selector: &CssPseudoClassIdentifier) -> bool {
    selector
        .name()
        .ok()
        .and_then(|name| name.as_css_identifier().cloned())
        .and_then(|name| name.value_token().ok())
        .is_some_and(|token| {
            decode_css_identifier(token.text_trimmed()).eq_ignore_ascii_case("global")
        })
}

pub fn css_property_definitions(root: &AnyCssRoot) -> Vec<CssPropertyDefinition> {
    collect_property_definitions(&semantic_model(root))
}

pub fn css_semantic_model(parse: &AnyParsedSource) -> SemanticModel {
    semantic_model(&parse.tree())
}

#[cfg(test)]
mod tests {
    use super::{collect_property_definitions, css_semantic_model};
    use biome_css_parser::{CssParserOptions, parse_css};
    use biome_languages::css::CssFileSource;
    use biome_parser::{AnyParsedSource, ParsedSnippet};
    use biome_rowan::{TextRange, TextSize};

    fn parsed_source(source: &str) -> AnyParsedSource {
        AnyParsedSource::ParsedSource(
            parse_css(source, CssFileSource::css(), CssParserOptions::default()).into(),
        )
    }

    fn parsed_snippet(source: &str) -> AnyParsedSource {
        let empty_range = TextRange::new(TextSize::default(), TextSize::default());
        AnyParsedSource::ParsedSnippet(ParsedSnippet {
            parsed: parse_css(source, CssFileSource::css(), CssParserOptions::default()).into(),
            element_range: empty_range,
            content_range: empty_range,
            content_offset: TextSize::default(),
            document_source_index: Some(0),
        })
    }

    #[test]
    fn parsed_source_helper_builds_model() {
        let source = "p { color: red; }";
        let parse = parse_css(source, CssFileSource::css(), CssParserOptions::default());
        let expected = crate::semantic_model(&parse.tree());

        assert_eq!(css_semantic_model(&parsed_source(source)), expected);
    }

    #[test]
    fn parsed_snippet_helper_builds_model() {
        let source = "p { color: red; }";
        let parse = parse_css(source, CssFileSource::css(), CssParserOptions::default());
        let expected = crate::semantic_model(&parse.tree());

        assert_eq!(css_semantic_model(&parsed_snippet(source)), expected);
    }

    #[test]
    fn property_definition_ranges_track_parse_changes() {
        let source = "@property --value { syntax: '<color>'; inherits: true; initial-value: red; }";
        let source_start =
            collect_property_definitions(&css_semantic_model(&parsed_source(source)))[0]
                .range()
                .start();
        let snippet_start =
            collect_property_definitions(&css_semantic_model(&parsed_snippet(source)))[0]
                .range()
                .start();
        let updated = format!("\n{source}");

        assert_eq!(
            collect_property_definitions(&css_semantic_model(&parsed_source(&updated)))[0]
                .range()
                .start(),
            source_start + TextSize::from(1)
        );
        assert_eq!(
            collect_property_definitions(&css_semantic_model(&parsed_snippet(&updated)))[0]
                .range()
                .start(),
            snippet_start + TextSize::from(1)
        );
    }

    #[test]
    fn escaped_global_selectors_are_globally_scoped() {
        let parse = parse_css(
            r#":g\6c obal(:root) { --function: red; }
:g\6c obal { --identifier: blue; }"#,
            CssFileSource::css(),
            CssParserOptions::default().allow_css_modules(),
        );
        let model = crate::semantic_model(&parse.tree());
        let definitions = collect_property_definitions(&model);

        assert_eq!(definitions.len(), 2);
        assert!(
            definitions
                .iter()
                .all(|definition| definition.globally_scoped)
        );
    }

    #[test]
    fn incomplete_property_value_does_not_panic() {
        let model = css_semantic_model(&parsed_source(".incomplete {\n  height:\n}\n"));
        assert_eq!(model.rules().len(), 1);
    }
}
