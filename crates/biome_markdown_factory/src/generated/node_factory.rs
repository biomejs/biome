//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
use biome_markdown_syntax::{
    MarkdownSyntaxElement as SyntaxElement, MarkdownSyntaxNode as SyntaxNode,
    MarkdownSyntaxToken as SyntaxToken, *,
};
use biome_rowan::AstNode;
pub fn markdown_break_block(value_token: SyntaxToken) -> MarkdownBreakBlock {
    MarkdownBreakBlock::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MARKDOWN_BREAK_BLOCK,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn markdown_bullet_list_item(
    markdown_bullet_list: MarkdownBulletList,
) -> MarkdownBulletListItem {
    MarkdownBulletListItem::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MARKDOWN_BULLET_LIST_ITEM,
        [Some(SyntaxElement::Node(
            markdown_bullet_list.into_syntax(),
        ))],
    ))
}
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
pub fn markdown_fenced_code_block(markdown_textual: MarkdownTextual) -> MarkdownFencedCodeBlock {
    MarkdownFencedCodeBlock::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MARKDOWN_FENCED_CODE_BLOCK,
        [Some(SyntaxElement::Node(markdown_textual.into_syntax()))],
    ))
}
pub fn markdown_h1() -> MarkdownH1Builder {
    MarkdownH1Builder {
        markdown_paragraph: None,
    }
}
pub struct MarkdownH1Builder {
    markdown_paragraph: Option<MarkdownParagraph>,
}
impl MarkdownH1Builder {
    pub fn with_markdown_paragraph(mut self, markdown_paragraph: MarkdownParagraph) -> Self {
        self.markdown_paragraph = Some(markdown_paragraph);
        self
    }
    pub fn build(self) -> MarkdownH1 {
        MarkdownH1::unwrap_cast(SyntaxNode::new_detached(
            MarkdownSyntaxKind::MARKDOWN_H1,
            [self
                .markdown_paragraph
                .map(|token| SyntaxElement::Node(token.into_syntax()))],
        ))
    }
}
pub fn markdown_h2() -> MarkdownH2Builder {
    MarkdownH2Builder {
        markdown_paragraph: None,
    }
}
pub struct MarkdownH2Builder {
    markdown_paragraph: Option<MarkdownParagraph>,
}
impl MarkdownH2Builder {
    pub fn with_markdown_paragraph(mut self, markdown_paragraph: MarkdownParagraph) -> Self {
        self.markdown_paragraph = Some(markdown_paragraph);
        self
    }
    pub fn build(self) -> MarkdownH2 {
        MarkdownH2::unwrap_cast(SyntaxNode::new_detached(
            MarkdownSyntaxKind::MARKDOWN_H2,
            [self
                .markdown_paragraph
                .map(|token| SyntaxElement::Node(token.into_syntax()))],
        ))
    }
}
pub fn markdown_h3() -> MarkdownH3Builder {
    MarkdownH3Builder {
        markdown_paragraph: None,
    }
}
pub struct MarkdownH3Builder {
    markdown_paragraph: Option<MarkdownParagraph>,
}
impl MarkdownH3Builder {
    pub fn with_markdown_paragraph(mut self, markdown_paragraph: MarkdownParagraph) -> Self {
        self.markdown_paragraph = Some(markdown_paragraph);
        self
    }
    pub fn build(self) -> MarkdownH3 {
        MarkdownH3::unwrap_cast(SyntaxNode::new_detached(
            MarkdownSyntaxKind::MARKDOWN_H3,
            [self
                .markdown_paragraph
                .map(|token| SyntaxElement::Node(token.into_syntax()))],
        ))
    }
}
pub fn markdown_h4() -> MarkdownH4Builder {
    MarkdownH4Builder {
        markdown_paragraph: None,
    }
}
pub struct MarkdownH4Builder {
    markdown_paragraph: Option<MarkdownParagraph>,
}
impl MarkdownH4Builder {
    pub fn with_markdown_paragraph(mut self, markdown_paragraph: MarkdownParagraph) -> Self {
        self.markdown_paragraph = Some(markdown_paragraph);
        self
    }
    pub fn build(self) -> MarkdownH4 {
        MarkdownH4::unwrap_cast(SyntaxNode::new_detached(
            MarkdownSyntaxKind::MARKDOWN_H4,
            [self
                .markdown_paragraph
                .map(|token| SyntaxElement::Node(token.into_syntax()))],
        ))
    }
}
pub fn markdown_h5() -> MarkdownH5Builder {
    MarkdownH5Builder {
        markdown_paragraph: None,
    }
}
pub struct MarkdownH5Builder {
    markdown_paragraph: Option<MarkdownParagraph>,
}
impl MarkdownH5Builder {
    pub fn with_markdown_paragraph(mut self, markdown_paragraph: MarkdownParagraph) -> Self {
        self.markdown_paragraph = Some(markdown_paragraph);
        self
    }
    pub fn build(self) -> MarkdownH5 {
        MarkdownH5::unwrap_cast(SyntaxNode::new_detached(
            MarkdownSyntaxKind::MARKDOWN_H5,
            [self
                .markdown_paragraph
                .map(|token| SyntaxElement::Node(token.into_syntax()))],
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
pub fn markdown_html_block(markdown_textual: MarkdownTextual) -> MarkdownHTMLBlock {
    MarkdownHTMLBlock::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MARKDOWN_HTML_BLOCK,
        [Some(SyntaxElement::Node(markdown_textual.into_syntax()))],
    ))
}
pub fn markdown_hard_line(value_token: SyntaxToken) -> MarkdownHardLine {
    MarkdownHardLine::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MARKDOWN_HARD_LINE,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn markdown_indent(value_token: SyntaxToken) -> MarkdownIndent {
    MarkdownIndent::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MARKDOWN_INDENT,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn markdown_indent_code_block(markdown_textual: MarkdownTextual) -> MarkdownIndentCodeBlock {
    MarkdownIndentCodeBlock::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MARKDOWN_INDENT_CODE_BLOCK,
        [Some(SyntaxElement::Node(markdown_textual.into_syntax()))],
    ))
}
pub fn markdown_inline_code(markdown_textual: MarkdownTextual) -> MarkdownInlineCode {
    MarkdownInlineCode::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MARKDOWN_INLINE_CODE,
        [Some(SyntaxElement::Node(markdown_textual.into_syntax()))],
    ))
}
pub fn markdown_inline_emphasis(markdown_textual: MarkdownTextual) -> MarkdownInlineEmphasis {
    MarkdownInlineEmphasis::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MARKDOWN_INLINE_EMPHASIS,
        [Some(SyntaxElement::Node(markdown_textual.into_syntax()))],
    ))
}
pub fn markdown_inline_image(
    alt: MarkdownTextual,
    src: MarkdownTextual,
) -> MarkdownInlineImageBuilder {
    MarkdownInlineImageBuilder {
        alt,
        src,
        title: None,
    }
}
pub struct MarkdownInlineImageBuilder {
    alt: MarkdownTextual,
    src: MarkdownTextual,
    title: Option<MarkdownTextual>,
}
impl MarkdownInlineImageBuilder {
    pub fn with_title(mut self, title: MarkdownTextual) -> Self {
        self.title = Some(title);
        self
    }
    pub fn build(self) -> MarkdownInlineImage {
        MarkdownInlineImage::unwrap_cast(SyntaxNode::new_detached(
            MarkdownSyntaxKind::MARKDOWN_INLINE_IMAGE,
            [
                Some(SyntaxElement::Node(self.alt.into_syntax())),
                Some(SyntaxElement::Node(self.src.into_syntax())),
                self.title
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn markdown_inline_link(
    label: MarkdownTextual,
    url: MarkdownTextual,
) -> MarkdownInlineLinkBuilder {
    MarkdownInlineLinkBuilder {
        label,
        url,
        title: None,
    }
}
pub struct MarkdownInlineLinkBuilder {
    label: MarkdownTextual,
    url: MarkdownTextual,
    title: Option<MarkdownTextual>,
}
impl MarkdownInlineLinkBuilder {
    pub fn with_title(mut self, title: MarkdownTextual) -> Self {
        self.title = Some(title);
        self
    }
    pub fn build(self) -> MarkdownInlineLink {
        MarkdownInlineLink::unwrap_cast(SyntaxNode::new_detached(
            MarkdownSyntaxKind::MARKDOWN_INLINE_LINK,
            [
                Some(SyntaxElement::Node(self.label.into_syntax())),
                Some(SyntaxElement::Node(self.url.into_syntax())),
                self.title
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn markdown_link_block(
    label: MarkdownTextual,
    url: MarkdownTextual,
) -> MarkdownLinkBlockBuilder {
    MarkdownLinkBlockBuilder {
        label,
        url,
        title: None,
    }
}
pub struct MarkdownLinkBlockBuilder {
    label: MarkdownTextual,
    url: MarkdownTextual,
    title: Option<MarkdownTextual>,
}
impl MarkdownLinkBlockBuilder {
    pub fn with_title(mut self, title: MarkdownTextual) -> Self {
        self.title = Some(title);
        self
    }
    pub fn build(self) -> MarkdownLinkBlock {
        MarkdownLinkBlock::unwrap_cast(SyntaxNode::new_detached(
            MarkdownSyntaxKind::MARKDOWN_LINK_BLOCK,
            [
                Some(SyntaxElement::Node(self.label.into_syntax())),
                Some(SyntaxElement::Node(self.url.into_syntax())),
                self.title
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn markdown_order_list_item(markdown_bullet_list: MarkdownBulletList) -> MarkdownOrderListItem {
    MarkdownOrderListItem::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MARKDOWN_ORDER_LIST_ITEM,
        [Some(SyntaxElement::Node(
            markdown_bullet_list.into_syntax(),
        ))],
    ))
}
pub fn markdown_paragraph(
    markdown_paragraph_item_list: MarkdownParagraphItemList,
) -> MarkdownParagraph {
    MarkdownParagraph::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MARKDOWN_PARAGRAPH,
        [Some(SyntaxElement::Node(
            markdown_paragraph_item_list.into_syntax(),
        ))],
    ))
}
pub fn markdown_quote(any_markdown_block: AnyMarkdownBlock) -> MarkdownQuote {
    MarkdownQuote::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MARKDOWN_QUOTE,
        [Some(SyntaxElement::Node(any_markdown_block.into_syntax()))],
    ))
}
pub fn markdown_setext_h1(markdown_paragraph: MarkdownParagraph) -> MarkdownSetextH1 {
    MarkdownSetextH1::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MARKDOWN_SETEXT_H1,
        [Some(SyntaxElement::Node(markdown_paragraph.into_syntax()))],
    ))
}
pub fn markdown_setext_h2(markdown_paragraph: MarkdownParagraph) -> MarkdownSetextH2 {
    MarkdownSetextH2::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MARKDOWN_SETEXT_H2,
        [Some(SyntaxElement::Node(markdown_paragraph.into_syntax()))],
    ))
}
pub fn markdown_soft_break(value_token: SyntaxToken) -> MarkdownSoftBreak {
    MarkdownSoftBreak::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MARKDOWN_SOFT_BREAK,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn markdown_textual(value_token: SyntaxToken) -> MarkdownTextual {
    MarkdownTextual::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MARKDOWN_TEXTUAL,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn markdown_bullet_list<I>(items: I) -> MarkdownBulletList
where
    I: IntoIterator<Item = AnyCodeBlock>,
    I::IntoIter: ExactSizeIterator,
{
    MarkdownBulletList::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MARKDOWN_BULLET_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn markdown_order_list<I>(items: I) -> MarkdownOrderList
where
    I: IntoIterator<Item = AnyCodeBlock>,
    I::IntoIter: ExactSizeIterator,
{
    MarkdownOrderList::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MARKDOWN_ORDER_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn markdown_paragraph_item_list<I>(items: I) -> MarkdownParagraphItemList
where
    I: IntoIterator<Item = AnyMarkdownInline>,
    I::IntoIter: ExactSizeIterator,
{
    MarkdownParagraphItemList::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MARKDOWN_PARAGRAPH_ITEM_LIST,
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
