//! Generated file, do not edit by hand, see `xtask/codegen`

#[doc = r" Reconstruct an AstNode from a SyntaxNode"]
#[doc = r""]
#[doc = r" This macros performs a match over the [kind](biome_rowan::SyntaxNode::kind)"]
#[doc = r" of the provided [biome_rowan::SyntaxNode] and constructs the appropriate"]
#[doc = r" AstNode type for it, then execute the provided expression over it."]
#[doc = r""]
#[doc = r" # Examples"]
#[doc = r""]
#[doc = r" ```ignore"]
#[doc = r" map_syntax_node!(syntax_node, node => node.format())"]
#[doc = r" ```"]
#[macro_export]
macro_rules! map_syntax_node {
    ($ node : expr , $ pattern : pat => $ body : expr) => {
        match $node {
            node => match $crate::MarkdownSyntaxNode::kind(&node) {
                $crate::MarkdownSyntaxKind::MD_BULLET_LIST_ITEM => {
                    let $pattern = unsafe { $crate::MdBulletListItem::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MD_DOCUMENT => {
                    let $pattern = unsafe { $crate::MdDocument::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MD_FENCED_CODE_BLOCK => {
                    let $pattern = unsafe { $crate::MdFencedCodeBlock::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MD_HARD_LINE => {
                    let $pattern = unsafe { $crate::MdHardLine::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MD_HASH => {
                    let $pattern = unsafe { $crate::MdHash::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MD_HEADER => {
                    let $pattern = unsafe { $crate::MdHeader::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MD_HTML_BLOCK => {
                    let $pattern = unsafe { $crate::MdHtmlBlock::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MD_INDENT => {
                    let $pattern = unsafe { $crate::MdIndent::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MD_INDENT_CODE_BLOCK => {
                    let $pattern = unsafe { $crate::MdIndentCodeBlock::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MD_INLINE_CODE => {
                    let $pattern = unsafe { $crate::MdInlineCode::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MD_INLINE_EMPHASIS => {
                    let $pattern = unsafe { $crate::MdInlineEmphasis::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MD_INLINE_IMAGE => {
                    let $pattern = unsafe { $crate::MdInlineImage::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MD_INLINE_LINK => {
                    let $pattern = unsafe { $crate::MdInlineLink::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MD_LINK_BLOCK => {
                    let $pattern = unsafe { $crate::MdLinkBlock::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MD_ORDER_LIST_ITEM => {
                    let $pattern = unsafe { $crate::MdOrderListItem::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MD_PARAGRAPH => {
                    let $pattern = unsafe { $crate::MdParagraph::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MD_QUOTE => {
                    let $pattern = unsafe { $crate::MdQuote::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MD_SETEXT_HEADER => {
                    let $pattern = unsafe { $crate::MdSetextHeader::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MD_SOFT_BREAK => {
                    let $pattern = unsafe { $crate::MdSoftBreak::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MD_TEXTUAL => {
                    let $pattern = unsafe { $crate::MdTextual::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MD_THEMATIC_BREAK_BLOCK => {
                    let $pattern = unsafe { $crate::MdThematicBreakBlock::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MD_BOGUS => {
                    let $pattern = unsafe { $crate::MdBogus::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MD_BLOCK_LIST => {
                    let $pattern = unsafe { $crate::MdBlockList::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MD_BULLET_LIST => {
                    let $pattern = unsafe { $crate::MdBulletList::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MD_HASH_LIST => {
                    let $pattern = unsafe { $crate::MdHashList::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MD_ORDER_LIST => {
                    let $pattern = unsafe { $crate::MdOrderList::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MD_PARAGRAPH_ITEM_LIST => {
                    let $pattern = unsafe { $crate::MdParagraphItemList::new_unchecked(node) };
                    $body
                }
                _ => unreachable!(),
            },
        }
    };
}
pub(crate) use map_syntax_node;
