//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
use biome_markdown_syntax::{
    MarkdownSyntaxElement as SyntaxElement, MarkdownSyntaxNode as SyntaxNode,
    MarkdownSyntaxToken as SyntaxToken, *,
};
use biome_rowan::AstNode;
pub fn markdown_document(
    value: AnyMarkdownBlock,
    eof_token: SyntaxToken,
) -> MarkdownDocumentBuilder {
    MarkdownDocumentBuilder {
        value,
        eof_token,
        bom_token: None,
    }
}
pub struct MarkdownDocumentBuilder {
    value: AnyMarkdownBlock,
    eof_token: SyntaxToken,
    bom_token: Option<SyntaxToken>,
}
impl MarkdownDocumentBuilder {
    pub fn with_bom_token(mut self, bom_token: SyntaxToken) -> Self {
        self.bom_token = Some(bom_token);
        self
    }
    pub fn build(self) -> MarkdownDocument {
        MarkdownDocument::unwrap_cast(SyntaxNode::new_detached(
            MarkdownSyntaxKind::MARKDOWN_DOCUMENT,
            [
                self.bom_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.value.into_syntax())),
                Some(SyntaxElement::Token(self.eof_token)),
            ],
        ))
    }
}
pub fn markdown_h1(hash_token: SyntaxToken, markdown_h2: MarkdownH2) -> MarkdownH1Builder {
    MarkdownH1Builder {
        hash_token,
        markdown_h2,
        hash_token: None,
        markdown_paragraph: None,
    }
}
pub struct MarkdownH1Builder {
    hash_token: SyntaxToken,
    markdown_h2: MarkdownH2,
    hash_token: Option<SyntaxToken>,
    markdown_paragraph: Option<MarkdownParagraph>,
}
impl MarkdownH1Builder {
    pub fn with_hash_token(mut self, hash_token: SyntaxToken) -> Self {
        self.hash_token = Some(hash_token);
        self
    }
    pub fn with_markdown_paragraph(mut self, markdown_paragraph: MarkdownParagraph) -> Self {
        self.markdown_paragraph = Some(markdown_paragraph);
        self
    }
    pub fn build(self) -> MarkdownH1 {
        MarkdownH1::unwrap_cast(SyntaxNode::new_detached(
            MarkdownSyntaxKind::MARKDOWN_H1,
            [
                Some(SyntaxElement::Token(self.hash_token)),
                Some(SyntaxElement::Node(self.markdown_h2.into_syntax())),
                self.hash_token.map(|token| SyntaxElement::Token(token)),
                self.markdown_paragraph
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn markdown_h2(hash_token: SyntaxToken, markdown_h3: MarkdownH3) -> MarkdownH2Builder {
    MarkdownH2Builder {
        hash_token,
        markdown_h3,
        hash_token: None,
        markdown_paragraph: None,
    }
}
pub struct MarkdownH2Builder {
    hash_token: SyntaxToken,
    markdown_h3: MarkdownH3,
    hash_token: Option<SyntaxToken>,
    markdown_paragraph: Option<MarkdownParagraph>,
}
impl MarkdownH2Builder {
    pub fn with_hash_token(mut self, hash_token: SyntaxToken) -> Self {
        self.hash_token = Some(hash_token);
        self
    }
    pub fn with_markdown_paragraph(mut self, markdown_paragraph: MarkdownParagraph) -> Self {
        self.markdown_paragraph = Some(markdown_paragraph);
        self
    }
    pub fn build(self) -> MarkdownH2 {
        MarkdownH2::unwrap_cast(SyntaxNode::new_detached(
            MarkdownSyntaxKind::MARKDOWN_H2,
            [
                Some(SyntaxElement::Token(self.hash_token)),
                Some(SyntaxElement::Node(self.markdown_h3.into_syntax())),
                self.hash_token.map(|token| SyntaxElement::Token(token)),
                self.markdown_paragraph
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn markdown_h3(hash_token: SyntaxToken, markdown_h4: MarkdownH4) -> MarkdownH3Builder {
    MarkdownH3Builder {
        hash_token,
        markdown_h4,
        hash_token: None,
        markdown_paragraph: None,
    }
}
pub struct MarkdownH3Builder {
    hash_token: SyntaxToken,
    markdown_h4: MarkdownH4,
    hash_token: Option<SyntaxToken>,
    markdown_paragraph: Option<MarkdownParagraph>,
}
impl MarkdownH3Builder {
    pub fn with_hash_token(mut self, hash_token: SyntaxToken) -> Self {
        self.hash_token = Some(hash_token);
        self
    }
    pub fn with_markdown_paragraph(mut self, markdown_paragraph: MarkdownParagraph) -> Self {
        self.markdown_paragraph = Some(markdown_paragraph);
        self
    }
    pub fn build(self) -> MarkdownH3 {
        MarkdownH3::unwrap_cast(SyntaxNode::new_detached(
            MarkdownSyntaxKind::MARKDOWN_H3,
            [
                Some(SyntaxElement::Token(self.hash_token)),
                Some(SyntaxElement::Node(self.markdown_h4.into_syntax())),
                self.hash_token.map(|token| SyntaxElement::Token(token)),
                self.markdown_paragraph
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn markdown_h4(hash_token: SyntaxToken, markdown_h5: MarkdownH5) -> MarkdownH4Builder {
    MarkdownH4Builder {
        hash_token,
        markdown_h5,
        hash_token: None,
        markdown_paragraph: None,
    }
}
pub struct MarkdownH4Builder {
    hash_token: SyntaxToken,
    markdown_h5: MarkdownH5,
    hash_token: Option<SyntaxToken>,
    markdown_paragraph: Option<MarkdownParagraph>,
}
impl MarkdownH4Builder {
    pub fn with_hash_token(mut self, hash_token: SyntaxToken) -> Self {
        self.hash_token = Some(hash_token);
        self
    }
    pub fn with_markdown_paragraph(mut self, markdown_paragraph: MarkdownParagraph) -> Self {
        self.markdown_paragraph = Some(markdown_paragraph);
        self
    }
    pub fn build(self) -> MarkdownH4 {
        MarkdownH4::unwrap_cast(SyntaxNode::new_detached(
            MarkdownSyntaxKind::MARKDOWN_H4,
            [
                Some(SyntaxElement::Token(self.hash_token)),
                Some(SyntaxElement::Node(self.markdown_h5.into_syntax())),
                self.hash_token.map(|token| SyntaxElement::Token(token)),
                self.markdown_paragraph
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn markdown_h5(hash_token: SyntaxToken, markdown_h6: MarkdownH6) -> MarkdownH5Builder {
    MarkdownH5Builder {
        hash_token,
        markdown_h6,
        hash_token: None,
        markdown_paragraph: None,
    }
}
pub struct MarkdownH5Builder {
    hash_token: SyntaxToken,
    markdown_h6: MarkdownH6,
    hash_token: Option<SyntaxToken>,
    markdown_paragraph: Option<MarkdownParagraph>,
}
impl MarkdownH5Builder {
    pub fn with_hash_token(mut self, hash_token: SyntaxToken) -> Self {
        self.hash_token = Some(hash_token);
        self
    }
    pub fn with_markdown_paragraph(mut self, markdown_paragraph: MarkdownParagraph) -> Self {
        self.markdown_paragraph = Some(markdown_paragraph);
        self
    }
    pub fn build(self) -> MarkdownH5 {
        MarkdownH5::unwrap_cast(SyntaxNode::new_detached(
            MarkdownSyntaxKind::MARKDOWN_H5,
            [
                Some(SyntaxElement::Token(self.hash_token)),
                Some(SyntaxElement::Node(self.markdown_h6.into_syntax())),
                self.hash_token.map(|token| SyntaxElement::Token(token)),
                self.markdown_paragraph
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn markdown_h6() -> MarkdownH6Builder {
    MarkdownH6Builder {
        markdown_paragraph: None,
    }
}
pub struct MarkdownH6Builder {
    markdown_paragraph: Option<MarkdownParagraph>,
}
impl MarkdownH6Builder {
    pub fn with_markdown_paragraph(mut self, markdown_paragraph: MarkdownParagraph) -> Self {
        self.markdown_paragraph = Some(markdown_paragraph);
        self
    }
    pub fn build(self) -> MarkdownH6 {
        MarkdownH6::unwrap_cast(SyntaxNode::new_detached(
            MarkdownSyntaxKind::MARKDOWN_H6,
            [self
                .markdown_paragraph
                .map(|token| SyntaxElement::Node(token.into_syntax()))],
        ))
    }
}
pub fn markdown_header(hash_token: SyntaxToken, markdown_h1: MarkdownH1) -> MarkdownHeaderBuilder {
    MarkdownHeaderBuilder {
        hash_token,
        markdown_h1,
        hash_token: None,
    }
}
pub struct MarkdownHeaderBuilder {
    hash_token: SyntaxToken,
    markdown_h1: MarkdownH1,
    hash_token: Option<SyntaxToken>,
}
impl MarkdownHeaderBuilder {
    pub fn with_hash_token(mut self, hash_token: SyntaxToken) -> Self {
        self.hash_token = Some(hash_token);
        self
    }
    pub fn build(self) -> MarkdownHeader {
        MarkdownHeader::unwrap_cast(SyntaxNode::new_detached(
            MarkdownSyntaxKind::MARKDOWN_HEADER,
            [
                Some(SyntaxElement::Token(self.hash_token)),
                Some(SyntaxElement::Node(self.markdown_h1.into_syntax())),
                self.hash_token.map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn markdown_paragraph(markdown_string: MarkdownString) -> MarkdownParagraph {
    MarkdownParagraph::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MARKDOWN_PARAGRAPH,
        [Some(SyntaxElement::Node(markdown_string.into_syntax()))],
    ))
}
pub fn markdown_string(value_token: SyntaxToken) -> MarkdownString {
    MarkdownString::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MARKDOWN_STRING,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn markdown_list<I>(items: I) -> MarkdownList
where
    I: IntoIterator<Item = MarkdownString>,
    I::IntoIter: ExactSizeIterator,
{
    MarkdownList::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MARKDOWN_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn markdown_bogus<I>(slots: I) -> MarkdownBogus
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    MarkdownBogus::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MARKDOWN_BOGUS,
        slots,
    ))
}
