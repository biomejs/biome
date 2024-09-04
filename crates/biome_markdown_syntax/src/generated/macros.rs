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
                $crate::MarkdownSyntaxKind::MARKDOWN_DOCUMENT => {
                    let $pattern = unsafe { $crate::MarkdownDocument::new_unchecked(node) };
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
                $crate::MarkdownSyntaxKind::MARKDOWN_HEADER => {
                    let $pattern = unsafe { $crate::MarkdownHeader::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MARKDOWN_PARAGRAPH => {
                    let $pattern = unsafe { $crate::MarkdownParagraph::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MARKDOWN_STRING => {
                    let $pattern = unsafe { $crate::MarkdownString::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MARKDOWN_BOGUS => {
                    let $pattern = unsafe { $crate::MarkdownBogus::new_unchecked(node) };
                    $body
                }
                $crate::MarkdownSyntaxKind::MARKDOWN_LIST => {
                    let $pattern = unsafe { $crate::MarkdownList::new_unchecked(node) };
                    $body
                }
                _ => unreachable!(),
            },
        }
    };
}
pub(crate) use map_syntax_node;
