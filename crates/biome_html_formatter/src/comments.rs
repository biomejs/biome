use biome_formatter::{
    FormatRule,
    comments::{
        CommentKind, CommentPlacement, CommentStyle, Comments, DecoratedComment, SourceComment,
    },
    prelude::*,
    write,
};
use biome_html_syntax::{HtmlClosingElement, HtmlLanguage, HtmlOpeningElement, HtmlSyntaxKind};
use biome_rowan::{AstNode, SyntaxNodeCast, SyntaxTriviaPieceComments};

use crate::context::HtmlFormatContext;

pub type HtmlComments = Comments<HtmlLanguage>;

#[derive(Default)]
pub struct FormatHtmlComment;

impl FormatRule<SourceComment<HtmlLanguage>> for FormatHtmlComment {
    type Context = HtmlFormatContext;

    fn fmt(
        &self,
        comment: &SourceComment<HtmlLanguage>,
        f: &mut Formatter<Self::Context>,
    ) -> FormatResult<()> {
        write!(f, [comment.piece().as_piece()])
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Default)]
pub struct HtmlCommentStyle;

impl CommentStyle for HtmlCommentStyle {
    type Language = HtmlLanguage;

    fn get_comment_kind(comment: &SyntaxTriviaPieceComments<HtmlLanguage>) -> CommentKind {
        // Svelte/Vue files can have JS-style `//` line comments inside tags.
        // These must be treated as line comments so the formatter uses `line_suffix`,
        // which forces a newline after them — preventing `>` from being swallowed.
        // HTML `<!-- -->` comments and `/* */` block comments are both block-style.
        if comment.text().starts_with("//") {
            CommentKind::Line
        } else {
            CommentKind::Block
        }
    }

    /// This allows us to override which comments are associated with which nodes.
    ///
    /// While every comment is directly attached to a **syntax token**, Biome actually builds a map of comments to **syntax nodes** separately. This map lives in [`HtmlComments`]. This is so that we can easily look up comments that are associated with a specific node. It's part of how suppression comments are handled.
    ///
    /// This method specifically, however, lets us fine tune which comments are associated with which nodes. This is useful when the default heuristic fails.
    fn place_comment(
        &self,
        comment: DecoratedComment<Self::Language>,
    ) -> CommentPlacement<Self::Language> {
        // Fix trailing comments that are right before EOF being assigned to the wrong node.
        //
        // The issue is demonstrated in the example below.
        // ```html
        // Foo
        //
        // <!-- This comment gets assigned to the text node, despite it being actually attached to the EOF token. -->
        // ```
        //
        // Only a comment that sits on its own line is moved. One that ends
        // the line of the element before it is already printed by the
        // element list, which reads it out of that element's trailing
        // trivia; moving it here as well would print it twice.
        if let Some(token) = comment.following_token()
            && token.kind() == HtmlSyntaxKind::EOF
            && !comment.text_position().is_end_of_line()
        {
            return CommentPlacement::trailing(comment.enclosing_node().clone(), comment);
        }

        // Move comments placed between opening and closing tags to be dangling comments of the opening tag
        // (if empty element) or trailing comments of the previous sibling.
        // This MUST be checked before the same-line check below to avoid incorrectly making
        // these comments leading comments of the closing element.
        //
        // For whitespace-sensitive elements like <span><!-- comment --></span>, we don't want
        // the comment to become a leading comment of </span> because that would add unwanted
        // formatting (like spaces) between the comment and the closing tag.
        if let Some(_closing_tag) = comment
            .following_node()
            .and_then(|node| node.clone().cast::<HtmlClosingElement>())
        {
            if let Some(_preceding_opening_tag) = comment
                .preceding_node()
                .and_then(|node| node.clone().cast::<HtmlOpeningElement>())
            {
                return CommentPlacement::dangling(
                    comment.preceding_node().unwrap().clone(),
                    comment,
                );
            } else {
                return CommentPlacement::trailing(
                    comment.preceding_node().unwrap().clone(),
                    comment,
                );
            }
        }

        // A comment on the same line as the last child of a Svelte block is in the trailing trivia
        // of the child's last token. The element list of the block's children prints it after that
        // child, so it must be attached to the child, which leaves it to the list. Otherwise, the
        // `{:...}` or `{/...}` node after it prints it again.
        //
        // ```svelte
        // {#if a}
        //   <span>a</span><!-- this comment is attached to the `<span>` element -->
        // {/if}
        // ```
        let token = comment.piece().as_piece().token();
        if let Some(following_token) = comment.following_token()
            && matches!(
                following_token.kind(),
                HtmlSyntaxKind::SV_CURLY_COLON | HtmlSyntaxKind::SV_CURLY_SLASH
            )
            && token != *following_token
            && let Some(child) = token.ancestors().find(|node| {
                node.parent()
                    .is_some_and(|parent| parent.kind() == HtmlSyntaxKind::HTML_ELEMENT_LIST)
            })
            && child.last_token().as_ref() == Some(&token)
        {
            return CommentPlacement::trailing(child, comment);
        }

        // Other comments before the `{:` or `{/` token that ends the children of a Svelte block
        // are also formatted by the children's element list, so they are attached to the node that
        // starts with that token, which skips its leading comments.
        //
        // ```svelte
        // {#if a}
        //   <span>a</span>
        //   <!-- this comment is attached to the `{/if}` node -->
        // {/if}
        // ```
        if let Some(token) = comment.following_token()
            && matches!(
                token.kind(),
                HtmlSyntaxKind::SV_CURLY_COLON | HtmlSyntaxKind::SV_CURLY_SLASH
            )
            && let Some(parent) = token.parent()
        {
            return CommentPlacement::leading(parent, comment);
        }

        // Attach comments between attributes to the following attribute as leading comments.
        // This is required for suppression comments (e.g. `// biome-ignore format: reason`)
        // to work on attributes:
        //
        // ```svelte
        // <div
        //   // biome-ignore format: reason
        //   class="foo"
        // >
        // ```
        //
        // Without this rule, the comment would be a trailing comment of the preceding attribute
        // (or a dangling comment of the opening element if it's the first attribute), and
        // `is_suppressed(class_attr)` would return false.
        //
        // NOTE: Do NOT use `comment.kind().is_line()` here to detect `//` comments — the
        // comment kind is determined after `place_comment()` runs in the pipeline, so
        // `kind()` always returns `Block` at this point. Instead, inspect the text directly.
        if matches!(
            comment.enclosing_node().kind(),
            HtmlSyntaxKind::HTML_OPENING_ELEMENT | HtmlSyntaxKind::HTML_SELF_CLOSING_ELEMENT
        ) && let Some(following_node) = comment.following_node()
        {
            // Only re-attach for attribute-level following nodes.
            // The closing tag case is already handled by the check above.
            if !HtmlClosingElement::can_cast(following_node.kind()) {
                return CommentPlacement::leading(following_node.clone(), comment);
            }
        }

        // Fix trailing comments that should actually be leading comments for the next node.
        // ```html
        // 123<!--biome-ignore format: prettier ignore-->456
        // ```
        // This fix will ensure that the ignore comment is assigned to the 456 node instead of the 123 node.
        if let Some(following_node) = comment.following_node()
            && comment.text_position().is_same_line()
        {
            return CommentPlacement::leading(following_node.clone(), comment);
        }

        CommentPlacement::Default(comment)
    }
}
