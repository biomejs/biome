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
                $crate::MarkdownSyntaxKind::MARKDOWN_BREAK_BLOCK => {
                    let $pattern = unsafe { $crate::MarkdownBreakBlock::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MARKDOWN_BULLET_LIST_ITEM => {
                    let $pattern = unsafe { $crate::MarkdownBulletListItem::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MARKDOWN_DOCUMENT => {
                    let $pattern = unsafe { $crate::MarkdownDocument::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MARKDOWN_FENCED_CODE_BLOCK => {
                    let $pattern = unsafe { $crate::MarkdownFencedCodeBlock::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MARKDOWN_HARD_LINE => {
                    let $pattern = unsafe { $crate::MarkdownHardLine::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MARKDOWN_HASH => {
                    let $pattern = unsafe { $crate::MarkdownHash::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MARKDOWN_HEADER => {
                    let $pattern = unsafe { $crate::MarkdownHeader::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MARKDOWN_HTML_BLOCK => {
                    let $pattern = unsafe { $crate::MarkdownHtmlBlock::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MARKDOWN_INDENT => {
                    let $pattern = unsafe { $crate::MarkdownIndent::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MARKDOWN_INDENT_CODE_BLOCK => {
                    let $pattern = unsafe { $crate::MarkdownIndentCodeBlock::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MARKDOWN_INLINE_CODE => {
                    let $pattern = unsafe { $crate::MarkdownInlineCode::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MARKDOWN_INLINE_EMPHASIS => {
                    let $pattern = unsafe { $crate::MarkdownInlineEmphasis::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MARKDOWN_INLINE_IMAGE => {
                    let $pattern = unsafe { $crate::MarkdownInlineImage::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MARKDOWN_INLINE_LINK => {
                    let $pattern = unsafe { $crate::MarkdownInlineLink::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MARKDOWN_LINK_BLOCK => {
                    let $pattern = unsafe { $crate::MarkdownLinkBlock::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MARKDOWN_ORDER_LIST_ITEM => {
                    let $pattern = unsafe { $crate::MarkdownOrderListItem::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MARKDOWN_PARAGRAPH => {
                    let $pattern = unsafe { $crate::MarkdownParagraph::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MARKDOWN_QUOTE => {
                    let $pattern = unsafe { $crate::MarkdownQuote::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MARKDOWN_SETEXT_HEADER => {
                    let $pattern = unsafe { $crate::MarkdownSetextHeader::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MARKDOWN_SOFT_BREAK => {
                    let $pattern = unsafe { $crate::MarkdownSoftBreak::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MARKDOWN_TEXTUAL => {
                    let $pattern = unsafe { $crate::MarkdownTextual::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MARKDOWN_BOGUS => {
                    let $pattern = unsafe { $crate::MarkdownBogus::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MARKDOWN_BLOCK_LIST => {
                    let $pattern = unsafe { $crate::MarkdownBlockList::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MARKDOWN_BULLET_LIST => {
                    let $pattern = unsafe { $crate::MarkdownBulletList::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MARKDOWN_HASH_LIST => {
                    let $pattern = unsafe { $crate::MarkdownHashList::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MARKDOWN_ORDER_LIST => {
                    let $pattern = unsafe { $crate::MarkdownOrderList::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MARKDOWN_PARAGRAPH_ITEM_LIST => {
                    let $pattern =
                        unsafe { $crate::MarkdownParagraphItemList::new_unchecked(node) };
                    $body
                }
                _ => unreachable!(),
            },
        }
    };
}
pub(crate) use map_syntax_node;
