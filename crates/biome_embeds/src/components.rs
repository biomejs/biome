use biome_db::ParsedSource;
use biome_html_syntax::element_ext::AnyHtmlTagElement;
use biome_html_syntax::{AnyHtmlComponentObjectName, AnyHtmlTagName, HtmlRoot};
use biome_languages::{DocumentFileSource, LanguageDb};
use biome_parser::AnyParse;
use biome_rowan::{AstNode, TextRange, TokenText};
use camino::Utf8Path;
use smallvec::SmallVec;

/// The segments of a component name, such as `Card` and `Root` for `Card.Root`.
///
/// Segments are stored for every component tag in a host document. Most
/// component names have one or two segments, such as `Button` or `Card.Root`,
/// so an inline capacity of two avoids a heap allocation for those tags.
/// Longer names, such as `UI.Card.Root`, spill to the heap.
pub type ComponentNameSegments = SmallVec<[TokenText; 2]>;

/// An opening or self-closing tag in a host document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct EmbeddedElement {
    /// The range of the tag in the host document.
    range: TextRange,
    /// The name segments, if the element is a component or a custom element.
    component: Option<ComponentNameSegments>,
}

/// Returns the name segments of `element` if it's a component, such as
/// `Button` or `Card.Root`, or a custom element, such as `my-button`.
pub fn component_name_segments(element: &AnyHtmlTagElement) -> Option<ComponentNameSegments> {
    if !element.is_custom_component()
        && !element
            .tag_name()
            .is_some_and(|name| name.text().contains('-'))
    {
        return None;
    }
    let mut object = match element.name().ok()? {
        AnyHtmlTagName::HtmlTagName(name) => AnyHtmlComponentObjectName::HtmlTagName(name),
        AnyHtmlTagName::HtmlComponentName(name) => {
            AnyHtmlComponentObjectName::HtmlComponentName(name)
        }
        AnyHtmlTagName::HtmlMemberName(name) => AnyHtmlComponentObjectName::HtmlMemberName(name),
    };
    let mut segments = SmallVec::new();
    loop {
        let token = match object {
            AnyHtmlComponentObjectName::HtmlMemberName(member) => {
                segments.push(
                    member
                        .member()
                        .ok()?
                        .value_token()
                        .ok()?
                        .token_text_trimmed(),
                );
                object = member.object().ok()?;
                continue;
            }
            AnyHtmlComponentObjectName::HtmlComponentName(name) => name.value_token().ok()?,
            AnyHtmlComponentObjectName::HtmlTagName(name) => name.value_token().ok()?,
        };
        segments.push(token.token_text_trimmed());
        segments.reverse();
        return Some(segments);
    }
}

/// Collects the tags of a host document in source order.
pub(crate) fn collect_embedded_elements(
    host_source: DocumentFileSource,
    host_parse: &AnyParse,
) -> Vec<EmbeddedElement> {
    if host_source.to_html_file_source().is_none() {
        return Vec::new();
    }
    let html_root: HtmlRoot = host_parse.tree();
    html_root
        .syntax()
        .descendants()
        .filter_map(AnyHtmlTagElement::cast)
        .map(|element| EmbeddedElement {
            range: element.range(),
            component: component_name_segments(&element),
        })
        .collect()
}

/// Returns the name segments of the component whose tag contains `range`.
///
/// `elements` must be in source order. Tags never overlap, so the only
/// candidate is the last tag that starts at or before `range`.
pub(crate) fn find_component_at(
    elements: &[EmbeddedElement],
    range: TextRange,
) -> Option<&ComponentNameSegments> {
    let index = elements.partition_point(|element| element.range.start() <= range.start());
    let element = elements.get(index.checked_sub(1)?)?;
    if element.range.contains_range(range) {
        element.component.as_ref()
    } else {
        None
    }
}

#[salsa::tracked(returns(ref))]
pub(crate) fn embedded_elements_from_source(
    db: &dyn LanguageDb,
    file: ParsedSource,
) -> Vec<EmbeddedElement> {
    let Some(host_source) = db.source_from_index(file.document_source_index(db)) else {
        return Vec::new();
    };
    collect_embedded_elements(host_source, file.parsed(db))
}

/// Returns the name segments of the component whose tag contains `range` in
/// the host document at `path`.
pub fn component_at(
    db: &dyn LanguageDb,
    path: &Utf8Path,
    range: TextRange,
) -> Option<ComponentNameSegments> {
    let file = db.parsed_source_for_path(path)?;
    find_component_at(embedded_elements_from_source(db, file), range).cloned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collect_embedded_data;
    use crate::testing::{TestDb, parse_vue_source};
    use biome_html_parser::{HtmlParserOptions, parse_html};
    use biome_languages::HtmlFileSource;
    use biome_rowan::TextSize;

    const SOURCE: &str = r#"<template><div class="a"></div><Card.Root class="b" /><my-button class="c"></my-button></template>"#;

    fn range_of(text: &str) -> TextRange {
        let start = SOURCE.find(text).unwrap();
        TextRange::at(
            TextSize::from(start as u32),
            TextSize::from(text.len() as u32),
        )
    }

    fn texts(segments: Option<ComponentNameSegments>) -> Option<Vec<String>> {
        segments.map(|segments| {
            segments
                .iter()
                .map(|text| text.text().to_string())
                .collect()
        })
    }

    #[test]
    fn component_at_finds_the_enclosing_component() {
        let db = TestDb::new();
        let path = parse_vue_source(&db, SOURCE);
        let component = |text| texts(component_at(&db, &path, range_of(text)));

        assert_eq!(component("\"a\""), None);
        assert_eq!(
            component("\"b\""),
            Some(vec!["Card".to_string(), "Root".to_string()])
        );
        assert_eq!(component("\"c\""), Some(vec!["my-button".to_string()]));
        assert_eq!(component("</div>"), None);
    }

    #[test]
    fn embedded_data_finds_the_enclosing_component() {
        let parse = parse_html(SOURCE, HtmlParserOptions::default().with_vue()).into();
        let data = collect_embedded_data(
            DocumentFileSource::Html(HtmlFileSource::vue()),
            &parse,
            Vec::new(),
        );
        let component = |text| texts(data.component_at(range_of(text)).cloned());

        assert_eq!(component("\"a\""), None);
        assert_eq!(
            component("\"b\""),
            Some(vec!["Card".to_string(), "Root".to_string()])
        );
        assert_eq!(component("\"c\""), Some(vec!["my-button".to_string()]));
        assert_eq!(component("</div>"), None);
    }
}
