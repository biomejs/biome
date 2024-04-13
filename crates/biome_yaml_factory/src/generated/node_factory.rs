//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
use biome_rowan::AstNode;
use biome_yaml_syntax::{
    YamlSyntaxElement as SyntaxElement, YamlSyntaxNode as SyntaxNode,
    YamlSyntaxToken as SyntaxToken, *,
};
pub fn yaml_document(content: YamlContentList) -> YamlDocument {
    YamlDocument::unwrap_cast(SyntaxNode::new_detached(
        YamlSyntaxKind::YAML_DOCUMENT,
        [Some(SyntaxElement::Node(content.into_syntax()))],
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
pub fn yaml_scalar(YAML_SCALAR_token: SyntaxToken) -> YamlScalar {
    YamlScalar::unwrap_cast(SyntaxNode::new_detached(
        YamlSyntaxKind::YAML_SCALAR,
        [Some(SyntaxElement::Token(YAML_SCALAR_token))],
    ))
}
pub fn yaml_content_list<I>(items: I) -> YamlContentList
where
    I: IntoIterator<Item = AnyYamlContent>,
    I::IntoIter: ExactSizeIterator,
{
    YamlContentList::unwrap_cast(SyntaxNode::new_detached(
        YamlSyntaxKind::YAML_CONTENT_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
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
pub fn yaml_bogus<I>(slots: I) -> YamlBogus
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    YamlBogus::unwrap_cast(SyntaxNode::new_detached(YamlSyntaxKind::YAML_BOGUS, slots))
}
pub fn yaml_bogus_value<I>(slots: I) -> YamlBogusValue
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    YamlBogusValue::unwrap_cast(SyntaxNode::new_detached(
        YamlSyntaxKind::YAML_BOGUS_VALUE,
        slots,
    ))
}
