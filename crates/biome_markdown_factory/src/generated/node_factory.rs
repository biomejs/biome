//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::redundant_closure)]
use biome_markdown_syntax::{
    MarkdownSyntaxElement as SyntaxElement, MarkdownSyntaxNode as SyntaxNode,
    MarkdownSyntaxToken as SyntaxToken, *,
};
use biome_rowan::AstNode;
pub fn md_autolink(
    l_angle_token: SyntaxToken,
    value: MdInlineItemList,
    r_angle_token: SyntaxToken,
) -> MdAutolink {
    MdAutolink::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MD_AUTOLINK,
        [
            Some(SyntaxElement::Token(l_angle_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
            Some(SyntaxElement::Token(r_angle_token)),
        ],
    ))
}
pub fn md_bullet(bullet_token: SyntaxToken, content: MdBlockList) -> MdBullet {
    MdBullet::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MD_BULLET,
        [
            Some(SyntaxElement::Token(bullet_token)),
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
pub fn md_entity_reference(value_token: SyntaxToken) -> MdEntityReference {
    MdEntityReference::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MD_ENTITY_REFERENCE,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn md_fenced_code_block(
    l_fence_token: SyntaxToken,
    code_list: MdCodeNameList,
    content: MdInlineItemList,
    r_fence_token: SyntaxToken,
) -> MdFencedCodeBlock {
    MdFencedCodeBlock::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MD_FENCED_CODE_BLOCK,
        [
            Some(SyntaxElement::Token(l_fence_token)),
            Some(SyntaxElement::Node(code_list.into_syntax())),
            Some(SyntaxElement::Node(content.into_syntax())),
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
pub fn md_html_block(content: MdInlineItemList) -> MdHtmlBlock {
    MdHtmlBlock::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MD_HTML_BLOCK,
        [Some(SyntaxElement::Node(content.into_syntax()))],
    ))
}
pub fn md_indent(value_token: SyntaxToken) -> MdIndent {
    MdIndent::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MD_INDENT,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn md_indent_code_block(content: MdInlineItemList) -> MdIndentCodeBlock {
    MdIndentCodeBlock::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MD_INDENT_CODE_BLOCK,
        [Some(SyntaxElement::Node(content.into_syntax()))],
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
pub fn md_inline_html(value: MdInlineItemList) -> MdInlineHtml {
    MdInlineHtml::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MD_INLINE_HTML,
        [Some(SyntaxElement::Node(value.into_syntax()))],
    ))
}
pub fn md_inline_image(
    excl_token: SyntaxToken,
    l_brack_token: SyntaxToken,
    alt: MdInlineItemList,
    r_brack_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    destination: MdInlineItemList,
    r_paren_token: SyntaxToken,
) -> MdInlineImageBuilder {
    MdInlineImageBuilder {
        excl_token,
        l_brack_token,
        alt,
        r_brack_token,
        l_paren_token,
        destination,
        r_paren_token,
        title: None,
    }
}
pub struct MdInlineImageBuilder {
    excl_token: SyntaxToken,
    l_brack_token: SyntaxToken,
    alt: MdInlineItemList,
    r_brack_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    destination: MdInlineItemList,
    r_paren_token: SyntaxToken,
    title: Option<MdLinkTitle>,
}
impl MdInlineImageBuilder {
    pub fn with_title(mut self, title: MdLinkTitle) -> Self {
        self.title = Some(title);
        self
    }
    pub fn build(self) -> MdInlineImage {
        MdInlineImage::unwrap_cast(SyntaxNode::new_detached(
            MarkdownSyntaxKind::MD_INLINE_IMAGE,
            [
                Some(SyntaxElement::Token(self.excl_token)),
                Some(SyntaxElement::Token(self.l_brack_token)),
                Some(SyntaxElement::Node(self.alt.into_syntax())),
                Some(SyntaxElement::Token(self.r_brack_token)),
                Some(SyntaxElement::Token(self.l_paren_token)),
                Some(SyntaxElement::Node(self.destination.into_syntax())),
                self.title
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.r_paren_token)),
            ],
        ))
    }
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
    destination: MdInlineItemList,
    r_paren_token: SyntaxToken,
) -> MdInlineLinkBuilder {
    MdInlineLinkBuilder {
        l_brack_token,
        text,
        r_brack_token,
        l_paren_token,
        destination,
        r_paren_token,
        title: None,
    }
}
pub struct MdInlineLinkBuilder {
    l_brack_token: SyntaxToken,
    text: MdInlineItemList,
    r_brack_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    destination: MdInlineItemList,
    r_paren_token: SyntaxToken,
    title: Option<MdLinkTitle>,
}
impl MdInlineLinkBuilder {
    pub fn with_title(mut self, title: MdLinkTitle) -> Self {
        self.title = Some(title);
        self
    }
    pub fn build(self) -> MdInlineLink {
        MdInlineLink::unwrap_cast(SyntaxNode::new_detached(
            MarkdownSyntaxKind::MD_INLINE_LINK,
            [
                Some(SyntaxElement::Token(self.l_brack_token)),
                Some(SyntaxElement::Node(self.text.into_syntax())),
                Some(SyntaxElement::Token(self.r_brack_token)),
                Some(SyntaxElement::Token(self.l_paren_token)),
                Some(SyntaxElement::Node(self.destination.into_syntax())),
                self.title
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.r_paren_token)),
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
pub fn md_link_destination(content: MdInlineItemList) -> MdLinkDestination {
    MdLinkDestination::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MD_LINK_DESTINATION,
        [Some(SyntaxElement::Node(content.into_syntax()))],
    ))
}
pub fn md_link_label(content: MdInlineItemList) -> MdLinkLabel {
    MdLinkLabel::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MD_LINK_LABEL,
        [Some(SyntaxElement::Node(content.into_syntax()))],
    ))
}
pub fn md_link_reference_definition(
    l_brack_token: SyntaxToken,
    label: MdLinkLabel,
    r_brack_token: SyntaxToken,
    colon_token: SyntaxToken,
    destination: MdLinkDestination,
) -> MdLinkReferenceDefinitionBuilder {
    MdLinkReferenceDefinitionBuilder {
        l_brack_token,
        label,
        r_brack_token,
        colon_token,
        destination,
        title: None,
    }
}
pub struct MdLinkReferenceDefinitionBuilder {
    l_brack_token: SyntaxToken,
    label: MdLinkLabel,
    r_brack_token: SyntaxToken,
    colon_token: SyntaxToken,
    destination: MdLinkDestination,
    title: Option<MdLinkTitle>,
}
impl MdLinkReferenceDefinitionBuilder {
    pub fn with_title(mut self, title: MdLinkTitle) -> Self {
        self.title = Some(title);
        self
    }
    pub fn build(self) -> MdLinkReferenceDefinition {
        MdLinkReferenceDefinition::unwrap_cast(SyntaxNode::new_detached(
            MarkdownSyntaxKind::MD_LINK_REFERENCE_DEFINITION,
            [
                Some(SyntaxElement::Token(self.l_brack_token)),
                Some(SyntaxElement::Node(self.label.into_syntax())),
                Some(SyntaxElement::Token(self.r_brack_token)),
                Some(SyntaxElement::Token(self.colon_token)),
                Some(SyntaxElement::Node(self.destination.into_syntax())),
                self.title
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn md_link_title(content: MdInlineItemList) -> MdLinkTitle {
    MdLinkTitle::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MD_LINK_TITLE,
        [Some(SyntaxElement::Node(content.into_syntax()))],
    ))
}
pub fn md_newline(value_token: SyntaxToken) -> MdNewline {
    MdNewline::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MD_NEWLINE,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn md_ordered_list_item(md_bullet_list: MdBulletList) -> MdOrderedListItem {
    MdOrderedListItem::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MD_ORDERED_LIST_ITEM,
        [Some(SyntaxElement::Node(md_bullet_list.into_syntax()))],
    ))
}
pub fn md_paragraph(list: MdInlineItemList) -> MdParagraphBuilder {
    MdParagraphBuilder {
        list,
        hard_line: None,
    }
}
pub struct MdParagraphBuilder {
    list: MdInlineItemList,
    hard_line: Option<MdHardLine>,
}
impl MdParagraphBuilder {
    pub fn with_hard_line(mut self, hard_line: MdHardLine) -> Self {
        self.hard_line = Some(hard_line);
        self
    }
    pub fn build(self) -> MdParagraph {
        MdParagraph::unwrap_cast(SyntaxNode::new_detached(
            MarkdownSyntaxKind::MD_PARAGRAPH,
            [
                Some(SyntaxElement::Node(self.list.into_syntax())),
                self.hard_line
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn md_quote(prefix: MdQuotePrefix, content: MdBlockList) -> MdQuote {
    MdQuote::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MD_QUOTE,
        [
            Some(SyntaxElement::Node(prefix.into_syntax())),
            Some(SyntaxElement::Node(content.into_syntax())),
        ],
    ))
}
pub fn md_quote_indent(md_quote_pre_marker_indent_token: SyntaxToken) -> MdQuoteIndent {
    MdQuoteIndent::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MD_QUOTE_INDENT,
        [Some(SyntaxElement::Token(md_quote_pre_marker_indent_token))],
    ))
}
pub fn md_quote_prefix(
    pre_marker_indent: MdQuoteIndentList,
    marker_token: SyntaxToken,
) -> MdQuotePrefixBuilder {
    MdQuotePrefixBuilder {
        pre_marker_indent,
        marker_token,
        post_marker_space_token: None,
    }
}
pub struct MdQuotePrefixBuilder {
    pre_marker_indent: MdQuoteIndentList,
    marker_token: SyntaxToken,
    post_marker_space_token: Option<SyntaxToken>,
}
impl MdQuotePrefixBuilder {
    pub fn with_post_marker_space_token(mut self, post_marker_space_token: SyntaxToken) -> Self {
        self.post_marker_space_token = Some(post_marker_space_token);
        self
    }
    pub fn build(self) -> MdQuotePrefix {
        MdQuotePrefix::unwrap_cast(SyntaxNode::new_detached(
            MarkdownSyntaxKind::MD_QUOTE_PREFIX,
            [
                Some(SyntaxElement::Node(self.pre_marker_indent.into_syntax())),
                Some(SyntaxElement::Token(self.marker_token)),
                self.post_marker_space_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn md_reference_image(
    excl_token: SyntaxToken,
    l_brack_token: SyntaxToken,
    alt: MdInlineItemList,
    r_brack_token: SyntaxToken,
) -> MdReferenceImageBuilder {
    MdReferenceImageBuilder {
        excl_token,
        l_brack_token,
        alt,
        r_brack_token,
        label: None,
    }
}
pub struct MdReferenceImageBuilder {
    excl_token: SyntaxToken,
    l_brack_token: SyntaxToken,
    alt: MdInlineItemList,
    r_brack_token: SyntaxToken,
    label: Option<MdReferenceLinkLabel>,
}
impl MdReferenceImageBuilder {
    pub fn with_label(mut self, label: MdReferenceLinkLabel) -> Self {
        self.label = Some(label);
        self
    }
    pub fn build(self) -> MdReferenceImage {
        MdReferenceImage::unwrap_cast(SyntaxNode::new_detached(
            MarkdownSyntaxKind::MD_REFERENCE_IMAGE,
            [
                Some(SyntaxElement::Token(self.excl_token)),
                Some(SyntaxElement::Token(self.l_brack_token)),
                Some(SyntaxElement::Node(self.alt.into_syntax())),
                Some(SyntaxElement::Token(self.r_brack_token)),
                self.label
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn md_reference_link(
    l_brack_token: SyntaxToken,
    text: MdInlineItemList,
    r_brack_token: SyntaxToken,
) -> MdReferenceLinkBuilder {
    MdReferenceLinkBuilder {
        l_brack_token,
        text,
        r_brack_token,
        label: None,
    }
}
pub struct MdReferenceLinkBuilder {
    l_brack_token: SyntaxToken,
    text: MdInlineItemList,
    r_brack_token: SyntaxToken,
    label: Option<MdReferenceLinkLabel>,
}
impl MdReferenceLinkBuilder {
    pub fn with_label(mut self, label: MdReferenceLinkLabel) -> Self {
        self.label = Some(label);
        self
    }
    pub fn build(self) -> MdReferenceLink {
        MdReferenceLink::unwrap_cast(SyntaxNode::new_detached(
            MarkdownSyntaxKind::MD_REFERENCE_LINK,
            [
                Some(SyntaxElement::Token(self.l_brack_token)),
                Some(SyntaxElement::Node(self.text.into_syntax())),
                Some(SyntaxElement::Token(self.r_brack_token)),
                self.label
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn md_reference_link_label(
    l_brack_token: SyntaxToken,
    label: MdInlineItemList,
    r_brack_token: SyntaxToken,
) -> MdReferenceLinkLabel {
    MdReferenceLinkLabel::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MD_REFERENCE_LINK_LABEL,
        [
            Some(SyntaxElement::Token(l_brack_token)),
            Some(SyntaxElement::Node(label.into_syntax())),
            Some(SyntaxElement::Token(r_brack_token)),
        ],
    ))
}
pub fn md_setext_header(content: MdInlineItemList, underline_token: SyntaxToken) -> MdSetextHeader {
    MdSetextHeader::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MD_SETEXT_HEADER,
        [
            Some(SyntaxElement::Node(content.into_syntax())),
            Some(SyntaxElement::Token(underline_token)),
        ],
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
pub fn md_code_name_list<I>(items: I) -> MdCodeNameList
where
    I: IntoIterator<Item = MdTextual>,
    I::IntoIter: ExactSizeIterator,
{
    MdCodeNameList::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MD_CODE_NAME_LIST,
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
pub fn md_quote_indent_list<I>(items: I) -> MdQuoteIndentList
where
    I: IntoIterator<Item = MdQuoteIndent>,
    I::IntoIter: ExactSizeIterator,
{
    MdQuoteIndentList::unwrap_cast(SyntaxNode::new_detached(
        MarkdownSyntaxKind::MD_QUOTE_INDENT_LIST,
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
