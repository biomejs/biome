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
                $crate::MarkdownSyntaxKind::MARKDOWN_H1 => {
                    let $pattern = unsafe { $crate::MarkdownH1::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MARKDOWN_H2 => {
                    let $pattern = unsafe { $crate::MarkdownH2::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MARKDOWN_H3 => {
                    let $pattern = unsafe { $crate::MarkdownH3::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MARKDOWN_H4 => {
                    let $pattern = unsafe { $crate::MarkdownH4::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MARKDOWN_H5 => {
                    let $pattern = unsafe { $crate::MarkdownH5::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MARKDOWN_H6 => {
                    let $pattern = unsafe { $crate::MarkdownH6::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MARKDOWN_HTML_BLOCK => {
                    let $pattern = unsafe { $crate::MarkdownHTMLBlock::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MARKDOWN_HARD_LINE => {
                    let $pattern = unsafe { $crate::MarkdownHardLine::new_unchecked(node) };
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
                $crate::MarkdownSyntaxKind::MARKDOWN_MINUS => {
                    let $pattern = unsafe { $crate::MarkdownMinus::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MARKDOWN_MINUS_THEMATIC_BREAK_BLOCK => {
                    let $pattern =
                        unsafe { $crate::MarkdownMinusThematicBreakBlock::new_unchecked(node) };
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
                $crate::MarkdownSyntaxKind::MARKDOWN_SETEXT_H1 => {
                    let $pattern = unsafe { $crate::MarkdownSetextH1::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MARKDOWN_SETEXT_H2 => {
                    let $pattern = unsafe { $crate::MarkdownSetextH2::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MARKDOWN_SOFT_BREAK => {
                    let $pattern = unsafe { $crate::MarkdownSoftBreak::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MARKDOWN_STAR => {
                    let $pattern = unsafe { $crate::MarkdownStar::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MARKDOWN_STAR_THEMATIC_BREAK_BLOCK => {
                    let $pattern =
                        unsafe { $crate::MarkdownStarThematicBreakBlock::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MARKDOWN_TEXTUAL => {
                    let $pattern = unsafe { $crate::MarkdownTextual::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MARKDOWN_UNDERSCORE => {
                    let $pattern = unsafe { $crate::MarkdownUnderscore::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MARKDOWN_UNDERSCORE_THEMATIC_BREAK_BLOCK => {
                    let $pattern = unsafe {
                        $crate::MarkdownUnderscoreThematicBreakBlock::new_unchecked(node)
                    };
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
                $crate::MarkdownSyntaxKind::MARKDOWN_MINUS_LIST => {
                    let $pattern = unsafe { $crate::MarkdownMinusList::new_unchecked(node) };
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
                $crate::MarkdownSyntaxKind::MARKDOWN_STAR_LIST => {
                    let $pattern = unsafe { $crate::MarkdownStarList::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MARKDOWN_UNDERSCORE_LIST => {
                    let $pattern = unsafe { $crate::MarkdownUnderscoreList::new_unchecked(node) };
                    $body
                }
                _ => unreachable!(),
            },
        }
    };
}
pub(crate) use map_syntax_node;
