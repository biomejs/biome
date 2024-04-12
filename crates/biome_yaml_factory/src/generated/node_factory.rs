//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
use biome_rowan::AstNode;
use biome_yaml_syntax::{
    YamlSyntaxElement as SyntaxElement, YamlSyntaxNode as SyntaxNode,
    YamlSyntaxToken as SyntaxToken, *,
};
pub fn yaml_alias(star_token: SyntaxToken, label_token: SyntaxToken) -> YamlAlias {
    YamlAlias::unwrap_cast(SyntaxNode::new_detached(
        YamlSyntaxKind::YAML_ALIAS,
        [
            Some(SyntaxElement::Token(star_token)),
            Some(SyntaxElement::Token(label_token)),
        ],
    ))
}
pub fn yaml_comment(hash_token: SyntaxToken, text_token: SyntaxToken) -> YamlComment {
    YamlComment::unwrap_cast(SyntaxNode::new_detached(
        YamlSyntaxKind::YAML_COMMENT,
        [
            Some(SyntaxElement::Token(hash_token)),
            Some(SyntaxElement::Token(text_token)),
        ],
    ))
}
pub fn yaml_document() -> YamlDocumentBuilder {
    YamlDocumentBuilder { content: None }
}
pub struct YamlDocumentBuilder {
    content: Option<AnyYamlNode>,
}
impl YamlDocumentBuilder {
    pub fn with_content(mut self, content: AnyYamlNode) -> Self {
        self.content = Some(content);
        self
    }
    pub fn build(self) -> YamlDocument {
        YamlDocument::unwrap_cast(SyntaxNode::new_detached(
            YamlSyntaxKind::YAML_DOCUMENT,
            [self
                .content
                .map(|token| SyntaxElement::Node(token.into_syntax()))],
        ))
    }
}
pub fn yaml_mapping(
    l_curly_token: SyntaxToken,
    entries: YamlMappingEntries,
    r_curly_token: SyntaxToken,
) -> YamlMapping {
    YamlMapping::unwrap_cast(SyntaxNode::new_detached(
        YamlSyntaxKind::YAML_MAPPING,
        [
            Some(SyntaxElement::Token(l_curly_token)),
            Some(SyntaxElement::Node(entries.into_syntax())),
            Some(SyntaxElement::Token(r_curly_token)),
        ],
    ))
}
pub fn yaml_mapping_entry(
    key: YamlScalar,
    colon_token: SyntaxToken,
    value: AnyYamlNode,
) -> YamlMappingEntry {
    YamlMappingEntry::unwrap_cast(SyntaxNode::new_detached(
        YamlSyntaxKind::YAML_MAPPING_ENTRY,
        [
            Some(SyntaxElement::Node(key.into_syntax())),
            Some(SyntaxElement::Token(colon_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn yaml_root(documents: YamlDocumentList, eof_token: SyntaxToken) -> YamlRootBuilder {
    YamlRootBuilder {
        documents,
        eof_token,
        bom_token: None,
    }
}
pub struct YamlRootBuilder {
    documents: YamlDocumentList,
    eof_token: SyntaxToken,
    bom_token: Option<SyntaxToken>,
}
impl YamlRootBuilder {
    pub fn with_bom_token(mut self, bom_token: SyntaxToken) -> Self {
        self.bom_token = Some(bom_token);
        self
    }
    pub fn build(self) -> YamlRoot {
        YamlRoot::unwrap_cast(SyntaxNode::new_detached(
            YamlSyntaxKind::YAML_ROOT,
            [
                self.bom_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.documents.into_syntax())),
                Some(SyntaxElement::Token(self.eof_token)),
            ],
        ))
    }
}
pub fn yaml_scalar(SCALAR_VALUE_token: SyntaxToken) -> YamlScalar {
    YamlScalar::unwrap_cast(SyntaxNode::new_detached(
        YamlSyntaxKind::YAML_SCALAR,
        [Some(SyntaxElement::Token(SCALAR_VALUE_token))],
    ))
}
pub fn yaml_sequence(
    l_brack_token: SyntaxToken,
    elements: YamlSequenceElements,
    r_brack_token: SyntaxToken,
) -> YamlSequence {
    YamlSequence::unwrap_cast(SyntaxNode::new_detached(
        YamlSyntaxKind::YAML_SEQUENCE,
        [
            Some(SyntaxElement::Token(l_brack_token)),
            Some(SyntaxElement::Node(elements.into_syntax())),
            Some(SyntaxElement::Token(r_brack_token)),
        ],
    ))
}
pub fn yaml_document_list<I>(items: I) -> YamlDocumentList
where
    I: IntoIterator<Item = YamlDocument>,
    I::IntoIter: ExactSizeIterator,
{
    YamlDocumentList::unwrap_cast(SyntaxNode::new_detached(
        YamlSyntaxKind::YAML_DOCUMENT_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn yaml_mapping_entries<I, S>(items: I, separators: S) -> YamlMappingEntries
where
    I: IntoIterator<Item = YamlMappingEntry>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = YamlSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    YamlMappingEntries::unwrap_cast(SyntaxNode::new_detached(
        YamlSyntaxKind::YAML_MAPPING_ENTRIES,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn yaml_sequence_elements<I, S>(items: I, separators: S) -> YamlSequenceElements
where
    I: IntoIterator<Item = AnyYamlNode>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = YamlSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    YamlSequenceElements::unwrap_cast(SyntaxNode::new_detached(
        YamlSyntaxKind::YAML_SEQUENCE_ELEMENTS,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn yaml_bogus<I>(slots: I) -> YamlBogus
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    YamlBogus::unwrap_cast(SyntaxNode::new_detached(YamlSyntaxKind::YAML_BOGUS, slots))
}
