//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::redundant_closure)]
use biome_markdown_syntax::{
    MarkdownSyntaxElement as SyntaxElement, MarkdownSyntaxNode as SyntaxNode,
    MarkdownSyntaxToken as SyntaxToken, *,
};
use biome_rowan::AstNode;
pub fn md_bullet_list_item(md_bullet_list: MdBulletList) -> MdBulletListItem {
    MdBulletListItem::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MD_BULLET_LIST_ITEM,
        [Some(SyntaxElement::Node(md_bullet_list.into_syntax()))],
    ))
}
pub fn md_document(value: MdBlockList, eof_token: SyntaxToken) -> MdDocumentBuilder {
    MdDocumentBuilder {
        value,
        eof_token,
        bom_token: None,
    }
}
pub struct MdDocumentBuilder {
    value: MdBlockList,
    eof_token: SyntaxToken,
    bom_token: Option<SyntaxToken>,
}
impl MdDocumentBuilder {
    pub fn with_bom_token(mut self, bom_token: SyntaxToken) -> Self {
        self.bom_token = Some(bom_token);
        self
    }
    pub fn build(self) -> MdDocument {
        MdDocument::unwrap_cast(SyntaxNode::new_detached(
            MarkdownSyntaxKind::MD_DOCUMENT,
            [
                self.bom_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.value.into_syntax())),
                Some(SyntaxElement::Token(self.eof_token)),
            ],
        ))
    }
}
pub fn md_fenced_code_block(md_textual: MdTextual) -> MdFencedCodeBlock {
    MdFencedCodeBlock::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MD_FENCED_CODE_BLOCK,
        [Some(SyntaxElement::Node(md_textual.into_syntax()))],
    ))
}
pub fn md_hard_line(value_token: SyntaxToken) -> MdHardLine {
    MdHardLine::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MD_HARD_LINE,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn md_hash(hash_token: SyntaxToken) -> MdHash {
    MdHash::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MD_HASH,
        [Some(SyntaxElement::Token(hash_token))],
    ))
}
pub fn md_header(before: MdHashList, after: MdHashList) -> MdHeaderBuilder {
    MdHeaderBuilder {
        before,
        after,
        md_paragraph: None,
    }
}
pub struct MdHeaderBuilder {
    before: MdHashList,
    after: MdHashList,
    md_paragraph: Option<MdParagraph>,
}
impl MdHeaderBuilder {
    pub fn with_md_paragraph(mut self, md_paragraph: MdParagraph) -> Self {
        self.md_paragraph = Some(md_paragraph);
        self
    }
    pub fn build(self) -> MdHeader {
        MdHeader::unwrap_cast(SyntaxNode::new_detached(
            MarkdownSyntaxKind::MD_HEADER,
            [
                Some(SyntaxElement::Node(self.before.into_syntax())),
                self.md_paragraph
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.after.into_syntax())),
            ],
        ))
    }
}
pub fn md_html_block(md_textual: MdTextual) -> MdHtmlBlock {
    MdHtmlBlock::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MD_HTML_BLOCK,
        [Some(SyntaxElement::Node(md_textual.into_syntax()))],
    ))
}
pub fn md_indent(value_token: SyntaxToken) -> MdIndent {
    MdIndent::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MD_INDENT,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn md_indent_code_block(md_textual: MdTextual) -> MdIndentCodeBlock {
    MdIndentCodeBlock::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MD_INDENT_CODE_BLOCK,
        [Some(SyntaxElement::Node(md_textual.into_syntax()))],
    ))
}
pub fn md_inline_code(md_textual: MdTextual) -> MdInlineCode {
    MdInlineCode::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MD_INLINE_CODE,
        [Some(SyntaxElement::Node(md_textual.into_syntax()))],
    ))
}
pub fn md_inline_emphasis(md_textual: MdTextual) -> MdInlineEmphasis {
    MdInlineEmphasis::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MD_INLINE_EMPHASIS,
        [Some(SyntaxElement::Node(md_textual.into_syntax()))],
    ))
}
pub fn md_inline_image(alt: MdTextual, src: MdTextual) -> MdInlineImageBuilder {
    MdInlineImageBuilder {
        alt,
        src,
        title: None,
    }
}
pub struct MdInlineImageBuilder {
    alt: MdTextual,
    src: MdTextual,
    title: Option<MdTextual>,
}
impl MdInlineImageBuilder {
    pub fn with_title(mut self, title: MdTextual) -> Self {
        self.title = Some(title);
        self
    }
    pub fn build(self) -> MdInlineImage {
        MdInlineImage::unwrap_cast(SyntaxNode::new_detached(
            MarkdownSyntaxKind::MD_INLINE_IMAGE,
            [
                Some(SyntaxElement::Node(self.alt.into_syntax())),
                Some(SyntaxElement::Node(self.src.into_syntax())),
                self.title
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn md_inline_link(label: MdTextual, url: MdTextual) -> MdInlineLinkBuilder {
    MdInlineLinkBuilder {
        label,
        url,
        title: None,
    }
}
pub struct MdInlineLinkBuilder {
    label: MdTextual,
    url: MdTextual,
    title: Option<MdTextual>,
}
impl MdInlineLinkBuilder {
    pub fn with_title(mut self, title: MdTextual) -> Self {
        self.title = Some(title);
        self
    }
    pub fn build(self) -> MdInlineLink {
        MdInlineLink::unwrap_cast(SyntaxNode::new_detached(
            MarkdownSyntaxKind::MD_INLINE_LINK,
            [
                Some(SyntaxElement::Node(self.label.into_syntax())),
                Some(SyntaxElement::Node(self.url.into_syntax())),
                self.title
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn md_link_block(label: MdTextual, url: MdTextual) -> MdLinkBlockBuilder {
    MdLinkBlockBuilder {
        label,
        url,
        title: None,
    }
}
pub struct MdLinkBlockBuilder {
    label: MdTextual,
    url: MdTextual,
    title: Option<MdTextual>,
}
impl MdLinkBlockBuilder {
    pub fn with_title(mut self, title: MdTextual) -> Self {
        self.title = Some(title);
        self
    }
    pub fn build(self) -> MdLinkBlock {
        MdLinkBlock::unwrap_cast(SyntaxNode::new_detached(
            MarkdownSyntaxKind::MD_LINK_BLOCK,
            [
                Some(SyntaxElement::Node(self.label.into_syntax())),
                Some(SyntaxElement::Node(self.url.into_syntax())),
                self.title
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn md_order_list_item(md_bullet_list: MdBulletList) -> MdOrderListItem {
    MdOrderListItem::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MD_ORDER_LIST_ITEM,
        [Some(SyntaxElement::Node(md_bullet_list.into_syntax()))],
    ))
}
pub fn md_paragraph(md_paragraph_item_list: MdParagraphItemList) -> MdParagraph {
    MdParagraph::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MD_PARAGRAPH,
        [Some(SyntaxElement::Node(
            md_paragraph_item_list.into_syntax(),
        ))],
    ))
}
pub fn md_quote(any_md_block: AnyMdBlock) -> MdQuote {
    MdQuote::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MD_QUOTE,
        [Some(SyntaxElement::Node(any_md_block.into_syntax()))],
    ))
}
pub fn md_setext_header(md_paragraph: MdParagraph) -> MdSetextHeader {
    MdSetextHeader::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MD_SETEXT_HEADER,
        [Some(SyntaxElement::Node(md_paragraph.into_syntax()))],
    ))
}
pub fn md_soft_break(value_token: SyntaxToken) -> MdSoftBreak {
    MdSoftBreak::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MD_SOFT_BREAK,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn md_textual(value_token: SyntaxToken) -> MdTextual {
    MdTextual::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MD_TEXTUAL,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn md_thematic_break_block(value_token: SyntaxToken) -> MdThematicBreakBlock {
    MdThematicBreakBlock::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MD_THEMATIC_BREAK_BLOCK,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn md_block_list<I>(items: I) -> MdBlockList
where
    I: IntoIterator<Item = AnyMdBlock>,
    I::IntoIter: ExactSizeIterator,
{
    MdBlockList::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MD_BLOCK_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn md_bullet_list<I>(items: I) -> MdBulletList
where
    I: IntoIterator<Item = AnyCodeBlock>,
    I::IntoIter: ExactSizeIterator,
{
    MdBulletList::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MD_BULLET_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn md_hash_list<I>(items: I) -> MdHashList
where
    I: IntoIterator<Item = MdHash>,
    I::IntoIter: ExactSizeIterator,
{
    MdHashList::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MD_HASH_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn md_order_list<I>(items: I) -> MdOrderList
where
    I: IntoIterator<Item = AnyCodeBlock>,
    I::IntoIter: ExactSizeIterator,
{
    MdOrderList::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MD_ORDER_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn md_paragraph_item_list<I>(items: I) -> MdParagraphItemList
where
    I: IntoIterator<Item = AnyMdInline>,
    I::IntoIter: ExactSizeIterator,
{
    MdParagraphItemList::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MD_PARAGRAPH_ITEM_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn md_bogus<I>(slots: I) -> MdBogus
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    MdBogus::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MD_BOGUS,
        slots,
    ))
}
