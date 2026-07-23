use biome_diagnostics::category;
use biome_formatter::comments::{
    CommentKind, CommentPlacement, CommentStyle, CommentTextPosition, Comments, DecoratedComment,
    SourceComment,
};
use biome_formatter::formatter::Formatter;
use biome_formatter::{FormatRefWithRule, FormatResult, FormatRule, write};
use biome_rowan::AstNode;
use biome_rowan::AstNodeList;
use biome_rowan::{SyntaxTriviaPieceComments, TextSize};
use biome_suppression::{SuppressionKind, parse_suppression_comment};
use biome_yaml_syntax::{
    AnyYamlMappingImplicitKey, YamlBlockInBlockNode, YamlBlockMapExplicitEntry,
    YamlBlockMapImplicitEntry, YamlDocument, YamlFlowJsonNode, YamlFlowMapExplicitEntry,
    YamlFlowYamlNode, YamlFoldedScalar, YamlLanguage, YamlLiteralScalar, YamlRoot, YamlSyntaxKind,
    YamlSyntaxNode, YamlSyntaxToken,
};

use crate::prelude::*;

pub type YamlComments = Comments<YamlLanguage>;

/// Whether any comment is attached to `node` or one of its descendants
pub(crate) fn subtree_has_comments(
    comments: &YamlComments,
    node: &biome_yaml_syntax::YamlSyntaxNode,
) -> bool {
    node.descendants().any(|node| comments.has_comments(&node))
}

#[derive(Default)]
pub struct FormatYamlLeadingComment;

impl FormatRule<SourceComment<YamlLanguage>> for FormatYamlLeadingComment {
    type Context = YamlFormatContext;

    fn fmt(
        &self,
        comment: &SourceComment<YamlLanguage>,
        f: &mut Formatter<Self::Context>,
    ) -> FormatResult<()> {
        write!(f, [comment.piece().as_piece()])
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Default)]
pub struct YamlCommentStyle;

impl CommentStyle for YamlCommentStyle {
    type Language = YamlLanguage;

    fn is_suppression(text: &str) -> bool {
        parse_suppression_comment(text)
            .filter_map(Result::ok)
            .filter(|suppression| suppression.kind == SuppressionKind::Classic)
            .flat_map(|suppression| suppression.categories)
            .any(|(key, ..)| key == category!("format"))
    }

    fn is_global_suppression(text: &str) -> bool {
        parse_suppression_comment(text)
            .filter_map(Result::ok)
            .filter(|suppression| suppression.kind == SuppressionKind::All)
            .flat_map(|suppression| suppression.categories)
            .any(|(key, ..)| key == category!("format"))
    }

    fn get_comment_kind(_comment: &SyntaxTriviaPieceComments<Self::Language>) -> CommentKind {
        CommentKind::Line
    }

    fn place_comment(
        &self,
        comment: DecoratedComment<Self::Language>,
    ) -> CommentPlacement<Self::Language> {
        handle_global_suppression(comment)
            .or_else(handle_document_comment)
            .or_else(handle_flow_map_explicit_entry_comment)
            .or_else(handle_block_map_explicit_entry_comment)
            .or_else(handle_middle_comment)
            .or_else(handle_flow_collection_open_comment)
            .or_else(handle_block_scalar_comment)
            .or_else(handle_own_line_comment)
            .or_else(handle_end_of_line_comment)
    }
}

/// Handles a middle comment, one sitting between a node's properties and its
/// content. It becomes a dangling comment of the node that owns the
/// properties, whose format rule keeps a single one on the properties' line
/// and moves a group onto their own lines above the content, as Prettier
/// does.
fn handle_middle_comment(
    comment: DecoratedComment<YamlLanguage>,
) -> CommentPlacement<YamlLanguage> {
    let range = comment.piece().text_range();
    for node in comment.enclosing_node().ancestors().take(5) {
        if let Some((start, end)) = middle_comment_region(&node)
            && start <= range.start()
            && range.end() <= end
        {
            return CommentPlacement::dangling(node, comment);
        }
    }
    CommentPlacement::Default(comment)
}

/// The source range between `node`'s properties and its content, in which a
/// comment is a middle comment of the node. For a block node whose
/// properties the parser flattened onto the first key of its mapping, the
/// region reaches down to the start of the key's own line.
fn middle_comment_region(node: &YamlSyntaxNode) -> Option<(TextSize, TextSize)> {
    match node.kind() {
        YamlSyntaxKind::YAML_FLOW_YAML_NODE => {
            let node = YamlFlowYamlNode::cast_ref(node)?;
            let skipped = AnyYamlMappingImplicitKey::YamlFlowYamlNode(node.clone())
                .enclosing_mapping_property_count();
            let last = node.properties().iter().skip(skipped).last()?;
            let content = node.content()?;
            Some((last.range().end(), content.range().start()))
        }
        YamlSyntaxKind::YAML_FLOW_JSON_NODE => {
            let node = YamlFlowJsonNode::cast_ref(node)?;
            let skipped = AnyYamlMappingImplicitKey::YamlFlowJsonNode(node.clone())
                .enclosing_mapping_property_count();
            let last = node.properties().iter().skip(skipped).last()?;
            let content = node.content().ok()?;
            Some((last.range().end(), content.range().start()))
        }
        YamlSyntaxKind::YAML_BLOCK_IN_BLOCK_NODE => {
            let node = YamlBlockInBlockNode::cast_ref(node)?;
            if let Some((properties, count)) = node.properties_on_first_key() {
                let last = properties.iter().nth(count - 1)?;
                // The key's own line, where the mapping's content begins
                let rest_start = properties
                    .iter()
                    .nth(count)
                    .map(|property| property.range().start())
                    .or_else(|| {
                        let key = last.syntax().parent()?.parent()?;
                        match AnyYamlMappingImplicitKey::cast(key)? {
                            AnyYamlMappingImplicitKey::YamlFlowYamlNode(node) => {
                                Some(node.content()?.range().start())
                            }
                            AnyYamlMappingImplicitKey::YamlFlowJsonNode(node) => {
                                Some(node.content().ok()?.range().start())
                            }
                            AnyYamlMappingImplicitKey::YamlAliasNode(_) => None,
                        }
                    })?;
                Some((last.range().end(), rest_start))
            } else {
                let last = node.properties().iter().last()?;
                let content = node.content().ok()?;
                Some((last.range().end(), content.range().start()))
            }
        }
        _ => None,
    }
}

/// Handles a comment right after the opening bracket of a flow collection
/// that sits on the line of a mapping key:
///
/// ```yaml
/// key: [ # comment
///   1, 2]
/// ```
///
/// Prettier treats it as ending the key's line: the comment becomes a
/// trailing comment of the key, and the collection moves to its own line
/// below. A comment in a collection that starts on its own line stays
/// inside it.
fn handle_flow_collection_open_comment(
    comment: DecoratedComment<YamlLanguage>,
) -> CommentPlacement<YamlLanguage> {
    if comment.text_position() != CommentTextPosition::EndOfLine
        || comment.preceding_node().is_some()
    {
        return CommentPlacement::Default(comment);
    }

    let enclosing = comment.enclosing_node();
    if !matches!(
        enclosing.kind(),
        YamlSyntaxKind::YAML_FLOW_SEQUENCE | YamlSyntaxKind::YAML_FLOW_MAPPING
    ) {
        return CommentPlacement::Default(comment);
    }

    let Some(entry) = enclosing
        .ancestors()
        .find(|ancestor| ancestor.kind() == YamlSyntaxKind::YAML_BLOCK_MAP_IMPLICIT_ENTRY)
        .and_then(YamlBlockMapImplicitEntry::cast)
    else {
        return CommentPlacement::Default(comment);
    };
    let Some(value) = entry.value() else {
        return CommentPlacement::Default(comment);
    };
    let Some(key) = entry.key() else {
        return CommentPlacement::Default(comment);
    };

    // The collection has to start on the key's line; the entry must also be
    // the direct parent of the collection, not of some enclosing entry
    let on_key_line = value
        .syntax()
        .descendants()
        .any(|descendant| descendant == *enclosing)
        && crate::utils::lines_before_through_end_tokens(value.syntax()) == 0;
    if !on_key_line {
        return CommentPlacement::Default(comment);
    }

    CommentPlacement::trailing(key.syntax().clone(), comment)
}

/// Formats the middle comments of a node, its dangling comments sitting
/// between its properties and its content. A single comment joins the
/// properties' line; a group goes onto its own lines below. The caller
/// prints the line break that separates the comments from the content.
pub(crate) struct FormatMiddleComments<'a> {
    node: &'a YamlSyntaxNode,
}

impl<'a> FormatMiddleComments<'a> {
    pub(crate) fn new(node: &'a YamlSyntaxNode) -> Self {
        Self { node }
    }
}

impl Format<YamlFormatContext> for FormatMiddleComments<'_> {
    fn fmt(&self, f: &mut Formatter<YamlFormatContext>) -> FormatResult<()> {
        // Cheap clone of an `Rc`, releasing the borrow on the formatter so
        // the comments can be written while iterating over them
        let comments = f.comments().clone();
        let dangling = comments.dangling_comments(self.node);
        let single = dangling.len() == 1;

        for comment in dangling {
            if single {
                write!(f, [space()])?;
            } else {
                write!(f, [hard_line_break()])?;
            }
            write!(
                f,
                [FormatRefWithRule::new(comment, FormatYamlLeadingComment)]
            )?;
            comment.mark_formatted();
        }
        Ok(())
    }
}

/// Handles the comments in the head of an explicit block mapping entry
/// (`? key : value`): next to the `?`, between the key and the `:`, and on
/// the line of the `:` before the value. They are made dangling comments of
/// the entry, whose format rule prints each at the position it came from:
///
/// ```yaml
/// ? key
///   # comment
/// : value
/// ```
fn handle_block_map_explicit_entry_comment(
    comment: DecoratedComment<YamlLanguage>,
) -> CommentPlacement<YamlLanguage> {
    let Some(entry) = YamlBlockMapExplicitEntry::cast_ref(comment.enclosing_node()) else {
        return CommentPlacement::Default(comment);
    };
    let comment_start = comment.piece().text_range().start();

    if let Some(colon) = entry.colon_token() {
        if comment_start < colon.text_trimmed_range().start() {
            return CommentPlacement::dangling(entry.syntax().clone(), comment);
        }
        if comment.text_position() == CommentTextPosition::EndOfLine
            && entry
                .value()
                .is_some_and(|value| comment_start < value.range().start())
        {
            return CommentPlacement::dangling(entry.syntax().clone(), comment);
        }
    } else if entry.value().is_none() {
        return CommentPlacement::dangling(entry.syntax().clone(), comment);
    }

    CommentPlacement::Default(comment)
}

/// Handles a comment on its own line that is indented deeper than the line
/// following it. Such a comment belongs to the structure it is indented
/// under, not to the node after it:
///
/// ```yaml
/// parent:
///   one: 1
///   # comment
/// next: 2
/// ```
///
/// The comment attaches to the deepest entry that precedes it without being
/// indented deeper than the comment itself: as a trailing comment when the
/// two are equally indented (the comment reads as a sibling of the entry),
/// and otherwise as a dangling comment, which the entry prints one level
/// deeper, in its value slot:
///
/// ```yaml
/// key:
///   # comment
/// ```
fn handle_own_line_comment(
    comment: DecoratedComment<YamlLanguage>,
) -> CommentPlacement<YamlLanguage> {
    if comment.text_position() != CommentTextPosition::OwnLine
        || YamlCommentStyle::is_suppression(comment.piece().text())
    {
        return CommentPlacement::Default(comment);
    }
    let Some(preceding) = comment.preceding_node() else {
        return CommentPlacement::Default(comment);
    };

    let token = comment.piece().as_piece().token();
    let comment_start = comment.piece().text_range().start();
    let mut comment_column = source_column(&token, comment_start);

    // The comments above this one in the same uninterrupted block of
    // comment lines cap how deep it can attach, so the block is never torn
    // apart and printed in a different order:
    //
    // ```yaml
    // a:
    //   b:
    //  #a
    //   #still a, not b
    // ```
    let mut line_column = None;
    for piece in token.leading_trivia().pieces() {
        if piece.text_range().start() >= comment_start {
            break;
        }
        if piece.is_newline() {
            line_column = Some(0);
        } else if piece.is_whitespace() {
            if let Some(column) = &mut line_column {
                *column += piece.text().len();
            }
        } else if piece.is_comments() {
            if let Some(column) = line_column {
                comment_column = comment_column.min(column);
            }
            line_column = None;
        }
    }

    // A comment that isn't indented deeper than the node following it leads
    // that node
    if let Some(following) = comment.following_node() {
        let following_column = following
            .first_token()
            .map(|token| source_column(&token, token.text_trimmed_range().start()));
        if following_column.is_none_or(|column| comment_column <= column) {
            return CommentPlacement::Default(comment);
        }
    }

    // The entries the comment can attach to sit along the last-child chain
    // of the preceding node, each one nested and indented deeper than the
    // one before
    let mut best = None;
    let mut current = Some(preceding.clone());
    while let Some(node) = current {
        if matches!(
            node.kind(),
            YamlSyntaxKind::YAML_BLOCK_MAP_IMPLICIT_ENTRY
                | YamlSyntaxKind::YAML_BLOCK_MAP_EXPLICIT_ENTRY
                | YamlSyntaxKind::YAML_BLOCK_SEQUENCE_ENTRY
        ) && let Some(token) = node.first_token()
        {
            let column = source_column(&token, token.text_trimmed_range().start());
            if column <= comment_column {
                best = Some((node.clone(), column));
            }
        }
        current = node.children().last();
    }

    match best {
        Some((entry, column)) if column < comment_column => {
            CommentPlacement::dangling(entry, comment)
        }
        Some((entry, _)) => CommentPlacement::trailing(entry, comment),
        None => CommentPlacement::Default(comment),
    }
}

/// The column at which the text at `offset` starts in the source, computed
/// by walking backward to the closest line break. `offset` must lie within
/// the span of `token`, trivia included
pub(crate) fn source_column(token: &YamlSyntaxToken, offset: TextSize) -> usize {
    /// Adds the width of a segment preceding the offset to the column;
    /// `true` when the segment contains the line break the column counts
    /// from, which ends the walk
    fn measure(text: &str, column: &mut usize) -> bool {
        match text.rfind(['\n', '\r']) {
            Some(index) => {
                *column += text.len() - index - 1;
                true
            }
            None => {
                *column += text.len();
                false
            }
        }
    }

    let mut column = 0;
    let mut current = Some(token.clone());
    while let Some(token) = current {
        let leading: Vec<_> = token.leading_trivia().pieces().collect();
        let trailing: Vec<_> = token.trailing_trivia().pieces().collect();

        for piece in trailing.iter().rev() {
            if piece.text_range().end() <= offset && measure(piece.text(), &mut column) {
                return column;
            }
        }
        if token.text_trimmed_range().end() <= offset && measure(token.text_trimmed(), &mut column)
        {
            return column;
        }
        for piece in leading.iter().rev() {
            if piece.text_range().end() <= offset && measure(piece.text(), &mut column) {
                return column;
            }
        }

        current = token.prev_token();
    }
    column
}

/// The number of line breaks to keep in front of `node`. The leading
/// comments of the node keep the blank lines above them themselves, so the
/// count stops at the first one; but a comment that attached elsewhere
/// prints on the other side of the separation, so what matters then is the
/// blank line count after it
pub(crate) fn preserved_lines_before(comments: &YamlComments, node: &YamlSyntaxNode) -> usize {
    if comments.has_leading_comments(node) {
        return get_lines_before(node);
    }

    node.first_token().map_or(0, |token| {
        token
            .leading_trivia()
            .pieces()
            .rev()
            .take_while(|piece| !piece.is_comments() && !piece.is_skipped())
            .filter(|piece| piece.is_newline())
            .count()
    })
}

/// Formats the dangling comments of a block collection entry: the comments
/// sitting in the entry's value slot, indented deeper than the entry itself.
/// They are printed on their own lines one level deeper than the entry,
/// keeping any blank line that separates them from the content before them:
///
/// ```yaml
/// key:
///   # comment
/// ```
pub(crate) struct FormatEntryDanglingComments<'a> {
    node: &'a YamlSyntaxNode,
}

impl<'a> FormatEntryDanglingComments<'a> {
    pub(crate) fn new(node: &'a YamlSyntaxNode) -> Self {
        Self { node }
    }
}

impl Format<YamlFormatContext> for FormatEntryDanglingComments<'_> {
    fn fmt(&self, f: &mut Formatter<YamlFormatContext>) -> FormatResult<()> {
        // Cheap clone of an `Rc`, releasing the borrow on the formatter so
        // the comments can be written while iterating over them
        let comments = f.comments().clone();
        let dangling = comments.dangling_comments(self.node);
        if dangling.is_empty() {
            return Ok(());
        }

        write!(
            f,
            [indent(&FormatCommentsSlice {
                comments: dangling,
                inline_first: true
            })]
        )
    }
}

/// Formats a run of comments, each opened by the line break its position in
/// the source calls for: a blank line when one separated it from the content
/// before, otherwise a plain break. With `inline_first`, a first comment
/// that started on the line of the preceding content stays there, after a
/// space
pub(crate) struct FormatCommentsSlice<'a> {
    pub(crate) comments: &'a [SourceComment<YamlLanguage>],
    pub(crate) inline_first: bool,
}

impl Format<YamlFormatContext> for FormatCommentsSlice<'_> {
    fn fmt(&self, f: &mut Formatter<YamlFormatContext>) -> FormatResult<()> {
        for (index, comment) in self.comments.iter().enumerate() {
            if index == 0 && self.inline_first && comment.lines_before() == 0 {
                write!(f, [space()])?;
            } else if comment.lines_before() > 1 {
                write!(f, [empty_line()])?;
            } else {
                write!(f, [hard_line_break()])?;
            }
            write!(
                f,
                [FormatRefWithRule::new(comment, FormatYamlLeadingComment)]
            )?;
            comment.mark_formatted();
        }
        Ok(())
    }
}

/// Handles comments that are attached to the marker tokens (`---`, `...`) or
/// directives of a [YamlDocument].
fn handle_document_comment(
    comment: DecoratedComment<YamlLanguage>,
) -> CommentPlacement<YamlLanguage> {
    let Some(document) = YamlDocument::cast_ref(comment.enclosing_node()) else {
        return CommentPlacement::Default(comment);
    };
    let comment_start = comment.piece().text_range().start();

    // Comments following the `...` document end marker belong to the document,
    // so they are printed after the marker (which is then kept).
    if let Some(dotdotdot) = document.dotdotdot_token()
        && comment_start > dotdotdot.text_trimmed_range().start()
    {
        return CommentPlacement::trailing(document.syntax().clone(), comment);
    }

    if let Some(dashdashdash) = document.dashdashdash_token() {
        if comment_start < dashdashdash.text_trimmed_range().start() {
            // Comments between the last directive and the `---` marker stay
            // with the directive so they aren't moved after the marker.
            if let Some(directive) = document.directives().iter().last()
                && directive.range().end() <= comment_start
            {
                return CommentPlacement::trailing(directive.syntax().clone(), comment);
            }

            // Comments preceding the `---` marker of a document without
            // directives lead the whole document.
            return CommentPlacement::leading(document.syntax().clone(), comment);
        }

        // Comments between the `---` marker and the document content are
        // printed right after the marker.
        let before_content = document
            .node()
            .is_none_or(|content| comment_start < content.range().start());
        if before_content {
            return CommentPlacement::dangling(document.syntax().clone(), comment);
        }
    }

    CommentPlacement::Default(comment)
}

/// Handles comments between the key and the `:` of an explicit flow mapping
/// entry (`? key : value`). They are made dangling comments of the entry so
/// its format rule can print them on their own lines before the `:`, where
/// they attach the same way when reparsed.
fn handle_flow_map_explicit_entry_comment(
    comment: DecoratedComment<YamlLanguage>,
) -> CommentPlacement<YamlLanguage> {
    let Some(entry) = YamlFlowMapExplicitEntry::cast_ref(comment.enclosing_node()) else {
        return CommentPlacement::Default(comment);
    };

    if let Some(colon) = entry.colon_token()
        && comment.piece().text_range().start() < colon.text_trimmed_range().start()
    {
        return CommentPlacement::dangling(entry.syntax().clone(), comment);
    }

    CommentPlacement::Default(comment)
}

/// Handles the comment on the header line of a block scalar (`a: | # c`).
/// It is made a dangling comment of the scalar so its format rule can print
/// it right after the header indicators, before the content starts.
fn handle_block_scalar_comment(
    comment: DecoratedComment<YamlLanguage>,
) -> CommentPlacement<YamlLanguage> {
    let enclosing = comment.enclosing_node();
    let content = if let Some(scalar) = YamlLiteralScalar::cast_ref(enclosing) {
        scalar.content()
    } else if let Some(scalar) = YamlFoldedScalar::cast_ref(enclosing) {
        scalar.content()
    } else {
        return CommentPlacement::Default(comment);
    };

    let before_content =
        content.is_ok_and(|content| comment.piece().text_range().start() < content.range().start());
    if before_content {
        return CommentPlacement::dangling(enclosing.clone(), comment);
    }

    CommentPlacement::Default(comment)
}

fn handle_end_of_line_comment(
    comment: DecoratedComment<YamlLanguage>,
) -> CommentPlacement<YamlLanguage> {
    if comment.text_position() != CommentTextPosition::EndOfLine {
        return CommentPlacement::Default(comment);
    }

    if let Some(preceding_node) = comment.preceding_node() {
        // A comment on the line of an entry that ends without a value sits
        // in the entry's value slot. As a dangling comment the entry prints
        // it right after the colon, ahead of any value-slot comments on the
        // following lines; a trailing comment would be deferred past them:
        //
        // ```yaml
        // key: # comment
        //   # value-slot comment
        // ```
        if matches!(
            preceding_node.kind(),
            YamlSyntaxKind::YAML_BLOCK_MAP_IMPLICIT_ENTRY
                | YamlSyntaxKind::YAML_BLOCK_SEQUENCE_ENTRY
        ) {
            return CommentPlacement::dangling(preceding_node.clone(), comment);
        }

        return CommentPlacement::trailing(preceding_node.clone(), comment);
    }

    CommentPlacement::Default(comment)
}

fn handle_global_suppression(
    comment: DecoratedComment<YamlLanguage>,
) -> CommentPlacement<YamlLanguage> {
    let node = comment.enclosing_node();

    if node.text_range_with_trivia().start() == TextSize::from(0) {
        let has_global_suppression = node.first_leading_trivia().is_some_and(|trivia| {
            trivia
                .pieces()
                .filter(|piece| piece.is_comments())
                .any(|piece| YamlCommentStyle::is_global_suppression(piece.text()))
        });
        let root = node.ancestors().find_map(YamlRoot::cast);
        if let Some(root) = root
            && has_global_suppression
        {
            return CommentPlacement::leading(root.syntax().clone(), comment);
        }
    }

    CommentPlacement::Default(comment)
}
