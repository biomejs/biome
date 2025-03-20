//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::redundant_closure)]
use biome_markdown_syntax::{
    MarkdownSyntaxElement as SyntaxElement, MarkdownSyntaxNode as SyntaxNode,
    MarkdownSyntaxToken as SyntaxToken, *,
};
use biome_rowan::AstNode;
pub fn md_bullet(
    bullet_token: SyntaxToken,
    space_token: SyntaxToken,
    content: MdInlineItemList,
) -> MdBullet {
    MdBullet::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MD_BULLET,
        [
            Some(SyntaxElement::Token(bullet_token)),
            Some(SyntaxElement::Token(space_token)),
            Some(SyntaxElement::Node(content.into_syntax())),
        ],
    ))
}
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
pub fn md_fenced_code_block(
    l_fence_token: SyntaxToken,
    code_list: MdCodeNameList,
    l_hard_line: MdHardLine,
    content: MdTextual,
    r_hard_line: MdHardLine,
    r_fence_token: SyntaxToken,
) -> MdFencedCodeBlock {
    MdFencedCodeBlock::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MD_FENCED_CODE_BLOCK,
        [
            Some(SyntaxElement::Token(l_fence_token)),
            Some(SyntaxElement::Node(code_list.into_syntax())),
            Some(SyntaxElement::Node(l_hard_line.into_syntax())),
            Some(SyntaxElement::Node(content.into_syntax())),
            Some(SyntaxElement::Node(r_hard_line.into_syntax())),
            Some(SyntaxElement::Token(r_fence_token)),
        ],
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
        content: None,
    }
}
pub struct MdHeaderBuilder {
    before: MdHashList,
    after: MdHashList,
    content: Option<MdParagraph>,
}
impl MdHeaderBuilder {
    pub fn with_content(mut self, content: MdParagraph) -> Self {
        self.content = Some(content);
        self
    }
    pub fn build(self) -> MdHeader {
        MdHeader::unwrap_cast(SyntaxNode::new_detached(
            MarkdownSyntaxKind::MD_HEADER,
            [
                Some(SyntaxElement::Node(self.before.into_syntax())),
                self.content
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
pub fn md_indent_code_block(lines: MdIndentedCodeLineList) -> MdIndentCodeBlock {
    MdIndentCodeBlock::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MD_INDENT_CODE_BLOCK,
        [Some(SyntaxElement::Node(lines.into_syntax()))],
    ))
}
pub fn md_indented_code_line(indentation: MdIndent, content: MdTextual) -> MdIndentedCodeLine {
    MdIndentedCodeLine::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MD_INDENTED_CODE_LINE,
        [
            Some(SyntaxElement::Node(indentation.into_syntax())),
            Some(SyntaxElement::Node(content.into_syntax())),
        ],
    ))
}
pub fn md_inline_code(
    l_tick_token: SyntaxToken,
    content: MdInlineItemList,
    r_tick_token: SyntaxToken,
) -> MdInlineCode {
    MdInlineCode::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MD_INLINE_CODE,
        [
            Some(SyntaxElement::Token(l_tick_token)),
            Some(SyntaxElement::Node(content.into_syntax())),
            Some(SyntaxElement::Token(r_tick_token)),
        ],
    ))
}
pub fn md_inline_emphasis(
    l_fence_token: SyntaxToken,
    content: MdInlineItemList,
    r_fence_token: SyntaxToken,
) -> MdInlineEmphasis {
    MdInlineEmphasis::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MD_INLINE_EMPHASIS,
        [
            Some(SyntaxElement::Token(l_fence_token)),
            Some(SyntaxElement::Node(content.into_syntax())),
            Some(SyntaxElement::Token(r_fence_token)),
        ],
    ))
}
pub fn md_inline_image(
    l_brack_token: SyntaxToken,
    excl_token: SyntaxToken,
    alt: MdInlineImageAlt,
    source: MdInlineImageSource,
    r_brack_token: SyntaxToken,
) -> MdInlineImageBuilder {
    MdInlineImageBuilder {
        l_brack_token,
        excl_token,
        alt,
        source,
        r_brack_token,
        link: None,
    }
}
pub struct MdInlineImageBuilder {
    l_brack_token: SyntaxToken,
    excl_token: SyntaxToken,
    alt: MdInlineImageAlt,
    source: MdInlineImageSource,
    r_brack_token: SyntaxToken,
    link: Option<MdInlineImageLink>,
}
impl MdInlineImageBuilder {
    pub fn with_link(mut self, link: MdInlineImageLink) -> Self {
        self.link = Some(link);
        self
    }
    pub fn build(self) -> MdInlineImage {
        MdInlineImage::unwrap_cast(SyntaxNode::new_detached(
            MarkdownSyntaxKind::MD_INLINE_IMAGE,
            [
                Some(SyntaxElement::Token(self.l_brack_token)),
                Some(SyntaxElement::Token(self.excl_token)),
                Some(SyntaxElement::Node(self.alt.into_syntax())),
                Some(SyntaxElement::Node(self.source.into_syntax())),
                Some(SyntaxElement::Token(self.r_brack_token)),
                self.link
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn md_inline_image_alt(
    l_brack_token: SyntaxToken,
    content: MdInlineItemList,
    r_brack_token: SyntaxToken,
) -> MdInlineImageAlt {
    MdInlineImageAlt::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MD_INLINE_IMAGE_ALT,
        [
            Some(SyntaxElement::Token(l_brack_token)),
            Some(SyntaxElement::Node(content.into_syntax())),
            Some(SyntaxElement::Token(r_brack_token)),
        ],
    ))
}
pub fn md_inline_image_link(
    l_paren_token: SyntaxToken,
    content: MdInlineItemList,
    r_paren_token: SyntaxToken,
) -> MdInlineImageLink {
    MdInlineImageLink::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MD_INLINE_IMAGE_LINK,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(content.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn md_inline_image_source(
    l_paren_token: SyntaxToken,
    content: MdInlineItemList,
    r_paren_token: SyntaxToken,
) -> MdInlineImageSource {
    MdInlineImageSource::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MD_INLINE_IMAGE_SOURCE,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(content.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn md_inline_italic(
    l_fence_token: SyntaxToken,
    content: MdInlineItemList,
    r_fence_token: SyntaxToken,
) -> MdInlineItalic {
    MdInlineItalic::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MD_INLINE_ITALIC,
        [
            Some(SyntaxElement::Token(l_fence_token)),
            Some(SyntaxElement::Node(content.into_syntax())),
            Some(SyntaxElement::Token(r_fence_token)),
        ],
    ))
}
pub fn md_inline_link(
    l_brack_token: SyntaxToken,
    text: MdInlineItemList,
    r_brack_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    source: MdInlineItemList,
    r_paren_token: SyntaxToken,
) -> MdInlineLink {
    MdInlineLink::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MD_INLINE_LINK,
        [
            Some(SyntaxElement::Token(l_brack_token)),
            Some(SyntaxElement::Node(text.into_syntax())),
            Some(SyntaxElement::Token(r_brack_token)),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(source.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
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
pub fn md_paragraph(list: MdInlineItemList, hard_line: MdHardLine) -> MdParagraph {
    MdParagraph::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MD_PARAGRAPH,
        [
            Some(SyntaxElement::Node(list.into_syntax())),
            Some(SyntaxElement::Node(hard_line.into_syntax())),
        ],
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
    I: IntoIterator<Item = MdBullet>,
    I::IntoIter: ExactSizeIterator,
{
    MdBulletList::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MD_BULLET_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn md_code_name_list<I, S>(items: I, separators: S) -> MdCodeNameList
where
    I: IntoIterator<Item = MdTextual>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = MarkdownSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    MdCodeNameList::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MD_CODE_NAME_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
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
pub fn md_indented_code_line_list<I>(items: I) -> MdIndentedCodeLineList
where
    I: IntoIterator<Item = MdIndentedCodeLine>,
    I::IntoIter: ExactSizeIterator,
{
    MdIndentedCodeLineList::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MD_INDENTED_CODE_LINE_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn md_inline_item_list<I>(items: I) -> MdInlineItemList
where
    I: IntoIterator<Item = AnyMdInline>,
    I::IntoIter: ExactSizeIterator,
{
    MdInlineItemList::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MD_INLINE_ITEM_LIST,
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
