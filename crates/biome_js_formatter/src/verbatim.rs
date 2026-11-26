use crate::context::JsFormatContext;
use biome_formatter::format_element::tag::VerbatimKind;
use biome_formatter::formatter::Formatter;
use biome_formatter::rowan::FormatElement;
use biome_formatter::rowan::prelude::{Tag, text};
use biome_formatter::trivia::{
    format_leading_comments_from_slice, format_trailing_comments_from_slice,
};
use biome_formatter::{
    Buffer, CstFormatContext, Format, FormatContext, FormatError, FormatResult, FormatWithRule,
    LINE_TERMINATORS, normalize_newlines,
};
use biome_js_syntax::{JsLanguage, JsSyntaxNode};
use biome_rowan::{AstNode, Direction, SyntaxElement};
use biome_text_size::TextRange;

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
pub fn format_js_verbatim_node(node: &JsSyntaxNode) -> FormatJsVerbatimNode<'_> {
    FormatJsVerbatimNode {
        node,
        kind: VerbatimKind::Verbatim {
            length: node.text_range_with_trivia().len(),
        },
        format_comments: true,
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct FormatJsVerbatimNode<'node> {
    node: &'node JsSyntaxNode,
    kind: VerbatimKind,
    format_comments: bool,
}

impl Format<JsFormatContext> for FormatJsVerbatimNode<'_> {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        fn source_range<Context>(f: &Formatter<Context>, range: TextRange) -> TextRange
        where
            Context: CstFormatContext,
        {
            f.context()
                .source_map()
                .map_or_else(|| range, |source_map| source_map.source_range(range))
        }

        let preserve_outer_trivia = self.node.parent().is_none();

        for element in self.node.descendants_with_tokens(Direction::Next) {
            match element {
                SyntaxElement::Token(token) => f.state_mut().track_token(&token),
                SyntaxElement::Node(node) => {
                    let comments = f.context().comments();
                    comments.mark_suppression_checked(&node);

                    for comment in comments.leading_dangling_trailing_comments(&node) {
                        comment.mark_formatted();
                    }
                }
            }
        }

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

        let verbatim_text_start = if preserve_outer_trivia {
            self.node.text_range_with_trivia().start()
        } else {
            self.node.text_trimmed_range().start()
        };

        f.write_element(FormatElement::Tag(Tag::StartVerbatim(self.kind)))?;

        // Format all leading comments that are outside of the node's source range.
        if self.format_comments {
            let comments = f.context().comments().clone();
            let leading_comments = comments.leading_comments(self.node);

            let outside_trimmed_range = leading_comments.partition_point(|comment| {
                comment.piece().text_range().end() <= verbatim_source_range.start()
            });

            let (outside_trimmed_range, in_trimmed_range) =
                leading_comments.split_at(outside_trimmed_range);

            biome_formatter::write!(
                f,
                [format_leading_comments_from_slice(outside_trimmed_range)]
            )?;

            for comment in in_trimmed_range {
                comment.mark_formatted();
            }
        }

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

        let original_range = verbatim_source_range.cover_offset(start_source);
        let original_source = match f.context().source_map() {
            Some(source_map) => source_map.source().text_slice(original_range).to_string(),
            // Without a source map, the tree is the source. Read the same range as the source map
            // branch from the tree's root, so the text includes the skipped token trivia that
            // `original_range` covers.
            None => {
                let root = self
                    .node
                    .ancestors()
                    .last()
                    .unwrap_or_else(|| self.node.clone());
                root.text_with_trivia()
                    .slice(original_range - root.text_range_with_trivia().start())
                    .to_string()
            }
        };

        text(
            &normalize_newlines(&original_source, LINE_TERMINATORS),
            Some(verbatim_text_start),
        )
        .fmt(f)?;

        for comment in f.context().comments().dangling_comments(self.node) {
            comment.mark_formatted();
        }

        // Format all trailing comments that are outside of the trimmed range.
        if self.format_comments {
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

            biome_formatter::write!(
                f,
                [format_trailing_comments_from_slice(outside_trimmed_range)]
            )?;
        }

        f.write_element(FormatElement::Tag(Tag::EndVerbatim))
    }
}

/// Formats bogus nodes. The difference between this method  and `format_verbatim` is that this method
/// doesn't track nodes/tokens as [VerbatimKind::Verbatim]. They are just printed as they are.
pub fn format_bogus_node(node: &JsSyntaxNode) -> FormatJsVerbatimNode<'_> {
    FormatJsVerbatimNode {
        node,
        kind: VerbatimKind::Bogus,
        format_comments: true,
    }
}

/// Format a node having formatter suppression comment applied to it
pub fn format_suppressed_node(node: &JsSyntaxNode) -> FormatJsVerbatimNode<'_> {
    FormatJsVerbatimNode {
        node,
        kind: VerbatimKind::Suppressed,
        format_comments: true,
    }
}

/// Format a node having formatter suppression comment applied to it
pub fn format_suppressed_node_skip_comments(node: &JsSyntaxNode) -> FormatJsVerbatimNode<'_> {
    FormatJsVerbatimNode {
        node,
        kind: VerbatimKind::Suppressed,
        format_comments: false,
    }
}

/// Formats an object using its [`Format`] implementation but falls back to printing the object as
/// it is in the source document if formatting it returns an [`FormatError::SyntaxError`].
pub const fn format_or_verbatim<F>(inner: F) -> FormatNodeOrVerbatim<F> {
    FormatNodeOrVerbatim { inner }
}

/// Formats a node or falls back to verbatim printing if formatting this node fails.
#[derive(Copy, Clone)]
pub struct FormatNodeOrVerbatim<F> {
    inner: F,
}

impl<F, Item> Format<JsFormatContext> for FormatNodeOrVerbatim<F>
where
    F: FormatWithRule<JsFormatContext, Item = Item>,
    Item: AstNode<Language = JsLanguage>,
{
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        let snapshot = Formatter::state_snapshot(f);

        match self.inner.fmt(f) {
            Ok(result) => Ok(result),

            Err(FormatError::SyntaxError) => {
                f.restore_state_snapshot(snapshot);

                // Lists that yield errors are formatted as they were suppressed nodes.
                // Doing so, the formatter formats the nodes/tokens as is.
                format_suppressed_node(self.inner.item().syntax()).fmt(f)
            }
            Err(err) => Err(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{JsFormatOptions, format_node};
    use biome_js_parser::{JsParserOptions, parse_module};
    use biome_js_syntax::JsSyntaxKind;
    use biome_languages::JsFileSource;
    use biome_rowan::{BatchMutation, Direction, TriviaPieceKind};

    #[test]
    fn keeps_leading_skipped_trivia_without_source_map() {
        // The file has no parentheses or logical expressions, so the transform changes nothing
        // and the formatter has no source map.
        let root = parse_module(
            "// biome-ignore format: test\na;",
            JsParserOptions::default(),
        )
        .syntax();
        let identifier = root
            .descendants_tokens(Direction::Next)
            .find(|token| token.kind() == JsSyntaxKind::IDENT)
            .unwrap();
        let with_skipped = identifier.with_leading_trivia([
            (
                TriviaPieceKind::SingleLineComment,
                "// biome-ignore format: test",
            ),
            (TriviaPieceKind::Newline, "\n"),
            (TriviaPieceKind::Skipped, "@"),
        ]);
        let mut mutation = BatchMutation::new(root);
        mutation.replace_token_discard_trivia(identifier, with_skipped);
        let root = mutation.commit();
        assert_eq!(
            root.text_with_trivia().to_string(),
            "// biome-ignore format: test\n@a;"
        );

        let formatted = format_node(
            JsFormatOptions::new(JsFileSource::js_module()),
            &root,
            Vec::new(),
        )
        .unwrap();

        assert_eq!(
            formatted.print().unwrap().as_code(),
            "// biome-ignore format: test\n@a;\n"
        );
    }
}
