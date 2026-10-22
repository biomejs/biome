use crate::comments::preserved_lines_before;
use crate::prelude::*;
use biome_formatter::trivia::format_dangling_comment;
use biome_formatter::write;
use biome_rowan::AstNode;
use biome_yaml_syntax::{YamlDocument, YamlDocumentFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlDocument;

impl FormatNodeRule<YamlDocument> for FormatYamlDocument {
    fn fmt_fields(&self, node: &YamlDocument, f: &mut YamlFormatter) -> FormatResult<()> {
        let YamlDocumentFields {
            bom_token,
            directives,
            dashdashdash_token,
            node: document_node,
            dotdotdot_token,
        } = node.as_fields();

        if let Some(bom_token) = bom_token {
            write!(f, [bom_token.format()])?;
        }

        // The dangling comments before the `---` marker close the directives;
        // the others follow the marker
        let comments = f.comments().clone();
        let dangling = comments.dangling_comments(node.syntax());
        let (head_comments, body_comments) =
            match &dashdashdash_token {
                Some(token) => {
                    let marker_start = token.text_trimmed_range().start();
                    dangling.split_at(dangling.partition_point(|comment| {
                        comment.piece().text_range().start() < marker_start
                    }))
                }
                None => (&[][..], dangling),
            };

        if let Some(last_directive) = directives.last() {
            // As in Prettier, the last directive of the last document is the
            // last node of the file, after which no blank line is kept. A
            // comment on a line of its own after the `...` marker is a
            // document of its own for Prettier
            let ends_file = !comments.has_trailing_comments(last_directive.syntax())
                && node.syntax().next_sibling().is_none()
                && !comments
                    .trailing_comments(node.syntax())
                    .iter()
                    .any(|comment| comment.lines_before() > 0);

            let head_id = f.group_id("directives");
            let head = format_with(|f| {
                write!(f, [directives.format()])?;
                for (index, comment) in head_comments.iter().enumerate() {
                    if comment.lines_before() > 1 && !(index == 0 && ends_file) {
                        write!(f, [empty_line()])?;
                    } else {
                        write!(f, [hard_line_break()])?;
                    }
                    write!(f, [format_dangling_comment(comment)])?;
                }
                Ok(())
            });
            write!(
                f,
                [group(&head).with_group_id(Some(head_id)), hard_line_break()]
            )?;

            // A blank line before the `---` marker is kept as in Prettier,
            // where it is a `softline` in the group of the directives: it
            // only prints when the group breaks, which it does when it holds
            // more than one line. A literal line break completes the hard
            // line break above to a blank line without expanding the group
            let lines_before_marker = dashdashdash_token.as_ref().map_or(0, |token| {
                token
                    .leading_trivia()
                    .pieces()
                    .rev()
                    .take_while(|piece| !piece.is_comments())
                    .filter(|piece| piece.is_newline())
                    .count()
            });
            if lines_before_marker > 1 && !(head_comments.is_empty() && ends_file) {
                write!(
                    f,
                    [if_group_breaks(&literal_line_break_without_parent())
                        .with_group_id(Some(head_id))]
                )?;
            }
        }

        if let Some(dashdashdash_token) = &dashdashdash_token {
            write!(f, [dashdashdash_token.format()])?;
        }

        // The dangling comments after the `---` marker sit between the marker
        // and the content, or in a document without content. A first comment
        // on the same line as the marker stays there; being line comments,
        // all the following ones necessarily go on their own line, keeping
        // the blank lines that separate them.
        if let Some((first, rest)) = body_comments.split_first() {
            if first.lines_before() == 0 {
                write!(f, [space()])?;
            } else {
                write!(f, [hard_line_break()])?;
            }
            write!(f, [format_dangling_comment(first)])?;

            for comment in rest {
                if comment.lines_before() > 1 {
                    write!(f, [empty_line()])?;
                } else {
                    write!(f, [hard_line_break()])?;
                }
                write!(f, [format_dangling_comment(comment)])?;
            }
        }

        if let Some(document_node) = &document_node {
            if dashdashdash_token.is_some() {
                // The content always goes on its own line below the marker,
                // even when it started on the marker's line
                if !body_comments.is_empty()
                    && preserved_lines_before(&comments, document_node.syntax()) > 1
                {
                    // A blank line separating the content from the comments
                    // above it is kept
                    write!(f, [empty_line()])?;
                } else {
                    write!(f, [hard_line_break()])?;
                }
            }
            write!(f, [document_node.format()])?;
        }

        if let Some(dotdotdot_token) = dotdotdot_token {
            write!(f, [hard_line_break(), dotdotdot_token.format()])?;
        }

        Ok(())
    }

    fn fmt_dangling_comments(&self, _: &YamlDocument, _: &mut YamlFormatter) -> FormatResult<()> {
        // The dangling comments close the directives, sit between the `---`
        // marker and the content, or close a document without content:
        //
        // ```yaml
        // %YAML 1.2
        // # comment
        // --- # comment
        // content
        // ```
        //
        // They are printed with `format_dangling_comment`, one at a time,
        // by `fmt_fields` around the marker; the default implementation
        // would print them again after the content
        Ok(())
    }
}
