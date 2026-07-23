use biome_formatter::format_element::tag::VerbatimKind;
use biome_formatter::formatter::Formatter;
use biome_formatter::prelude::{Tag, text};
use biome_formatter::trivia::{
    format_leading_comments_from_slice, format_trailing_comments_from_slice,
};
use biome_formatter::{
    Buffer, CstFormatContext, Format, FormatContext, FormatElement, FormatResult, LINE_TERMINATORS,
    normalize_newlines,
};
use biome_rowan::{Direction, SyntaxElement, TextRange};
use biome_yaml_syntax::YamlSyntaxNode;

use crate::context::YamlFormatContext;

/// "Formats" a node according to its original formatting in the source text. Being able to format
/// a node "as is" is useful if a node contains syntax errors. Formatting a node with syntax errors
/// has the risk that Biome misinterprets the structure of the code and formatting it could
/// "mess up" the developers, yet incomplete, work or accidentally introduce new syntax errors.
///
/// You may be inclined to call `node.text` directly. However, using `text` doesn't track the nodes
/// nor its children source mapping information, resulting in incorrect source maps for this subtree.
///
/// These nodes and tokens get tracked as [VerbatimKind::Verbatim], useful to understand
/// if these nodes still need to have their own implementation.
///
/// No handwritten formatting rule calls this anymore, but it stays around
/// because `just gen-formatter` generates implementations for new syntax
/// nodes that call it (as `format_verbatim_node`, via the prelude)
#[expect(dead_code)]
pub fn format_yaml_verbatim_node(node: &YamlSyntaxNode) -> FormatYamlVerbatimNode<'_> {
    FormatYamlVerbatimNode {
        node,
        kind: VerbatimKind::Verbatim {
            length: node.text_range_with_trivia().len(),
        },
        // This variant is used from within `FormatNodeRule::fmt_fields`, where
        // `FormatNodeRule::fmt` already formats the node's leading and trailing
        // comments. Formatting them here as well would print them twice.
        format_comments: false,
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct FormatYamlVerbatimNode<'node> {
    node: &'node YamlSyntaxNode,
    kind: VerbatimKind,
    format_comments: bool,
}

impl Format<YamlFormatContext> for FormatYamlVerbatimNode<'_> {
    fn fmt(&self, f: &mut Formatter<YamlFormatContext>) -> FormatResult<()> {
        fn source_range<Context>(f: &Formatter<Context>, range: TextRange) -> TextRange
        where
            Context: CstFormatContext,
        {
            f.context()
                .source_map()
                .map_or_else(|| range, |source_map| source_map.source_range(range))
        }

        let preserve_outer_trivia = self.node.parent().is_none();

        // The trimmed range of a node is its range without any of its leading or trailing trivia.
        // Except for nodes that used to be parenthesized, the range than covers the source from the
        // `(` to the `)` (the trimmed range of the parenthesized expression, not the inner expression)
        let verbatim_source_range = if preserve_outer_trivia {
            source_range(f, self.node.text_range_with_trivia())
        } else {
            f.context().source_map().map_or_else(
                || self.node.text_trimmed_range(),
                |source_map| source_map.trimmed_source_range(self.node),
            )
        };

        // Comments attached to a descendant node that are part of the printed
        // source range don't need any handling of their own. However, comments
        // placed on a descendant can physically sit in the trivia surrounding
        // the verbatim node, so they have to be printed here or they would be
        // dropped.
        let mut descendant_comments_before = Vec::new();
        let mut descendant_comments_after = Vec::new();
        for element in self.node.descendants_with_tokens(Direction::Next) {
            match element {
                SyntaxElement::Token(token) => f.state_mut().track_token(&token),
                SyntaxElement::Node(node) => {
                    let comments = f.context().comments();
                    comments.mark_suppression_checked(&node);

                    if node == *self.node {
                        // The verbatim node's own comments are handled below,
                        // or by the caller
                        continue;
                    }
                    for comment in comments.leading_dangling_trailing_comments(&node) {
                        if !preserve_outer_trivia {
                            let comment_range = source_range(f, comment.piece().text_range());
                            if comment_range.end() <= verbatim_source_range.start() {
                                descendant_comments_before.push(comment.clone());
                            } else if comment_range.start() >= verbatim_source_range.end() {
                                descendant_comments_after.push(comment.clone());
                            }
                        }
                        comment.mark_formatted();
                    }
                }
            }
        }

        let verbatim_text_start = if preserve_outer_trivia {
            self.node.text_range_with_trivia().start()
        } else {
            self.node.text_trimmed_range().start()
        };

        f.write_element(FormatElement::Tag(Tag::StartVerbatim(self.kind)))?;

        // Format all leading comments that are outside of the node's source range.
        {
            let comments = f.context().comments().clone();
            let leading_comments = comments.leading_comments(self.node);

            let outside_trimmed_range = leading_comments.partition_point(|comment| {
                comment.piece().text_range().end() <= verbatim_source_range.start()
            });

            let (outside_trimmed_range, in_trimmed_range) =
                leading_comments.split_at(outside_trimmed_range);

            if self.format_comments {
                biome_formatter::write!(
                    f,
                    [format_leading_comments_from_slice(outside_trimmed_range)]
                )?;
            }

            for comment in in_trimmed_range {
                comment.mark_formatted();
            }
        }

        descendant_comments_before.sort_by_key(|comment| comment.piece().text_range().start());
        biome_formatter::write!(
            f,
            [format_leading_comments_from_slice(
                &descendant_comments_before
            )]
        )?;

        // Find the first skipped token trivia, if any, and include it in the verbatim range because
        // the comments only format **up to** but not including skipped token trivia.
        let start_source = self
            .node
            .first_leading_trivia()
            .into_iter()
            .flat_map(|trivia| trivia.pieces())
            .filter(|trivia| trivia.is_skipped())
            .map(|trivia| source_range(f, trivia.text_range()).start())
            .take_while(|start| *start < verbatim_source_range.start())
            .next()
            .unwrap_or_else(|| verbatim_source_range.start());

        let original_source = f.context().source_map().map_or_else(
            || {
                if preserve_outer_trivia {
                    self.node.text_with_trivia().to_string()
                } else {
                    self.node.text_trimmed().to_string()
                }
            },
            |source_map| {
                source_map
                    .source()
                    .text_slice(verbatim_source_range.cover_offset(start_source))
                    .to_string()
            },
        );

        text(
            &normalize_newlines(&original_source, LINE_TERMINATORS),
            Some(verbatim_text_start),
        )
        .fmt(f)?;

        for comment in f.context().comments().dangling_comments(self.node) {
            comment.mark_formatted();
        }

        descendant_comments_after.sort_by_key(|comment| comment.piece().text_range().start());
        biome_formatter::write!(
            f,
            [format_trailing_comments_from_slice(
                &descendant_comments_after
            )]
        )?;

        // Format all trailing comments that are outside of the trimmed range.
        {
            let comments = f.context().comments().clone();

            let trailing_comments = comments.trailing_comments(self.node);

            let outside_trimmed_range_start = trailing_comments.partition_point(|comment| {
                source_range(f, comment.piece().text_range()).end() <= verbatim_source_range.end()
            });

            let (in_trimmed_range, outside_trimmed_range) =
                trailing_comments.split_at(outside_trimmed_range_start);

            for comment in in_trimmed_range {
                comment.mark_formatted();
            }

            if self.format_comments {
                biome_formatter::write!(
                    f,
                    [format_trailing_comments_from_slice(outside_trimmed_range)]
                )?;
            }
        }

        f.write_element(FormatElement::Tag(Tag::EndVerbatim))
    }
}

/// Formats bogus nodes. The difference between this method  and `format_verbatim` is that this method
/// doesn't track nodes/tokens as [VerbatimKind::Verbatim]. They are just printed as they are.
pub fn format_bogus_node(node: &YamlSyntaxNode) -> FormatYamlVerbatimNode<'_> {
    FormatYamlVerbatimNode {
        node,
        kind: VerbatimKind::Bogus,
        format_comments: true,
    }
}

/// Format a node having formatter suppression comment applied to it
pub fn format_suppressed_node(node: &YamlSyntaxNode) -> FormatYamlVerbatimNode<'_> {
    FormatYamlVerbatimNode {
        node,
        kind: VerbatimKind::Suppressed,
        format_comments: true,
    }
}
