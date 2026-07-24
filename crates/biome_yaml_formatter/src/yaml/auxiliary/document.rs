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

        if !directives.is_empty() {
            write!(f, [directives.format(), hard_line_break()])?;
        }

        if let Some(dashdashdash_token) = &dashdashdash_token {
            write!(f, [dashdashdash_token.format()])?;
        }

        // Dangling comments sit between the `---` marker and the content, or
        // in a document without content. A first comment on the same line as
        // the marker stays there; being line comments, all the following ones
        // necessarily go on their own line, keeping the blank lines that
        // separate them.
        let comments = f.comments().clone();
        if let Some((first, rest)) = comments.dangling_comments(node.syntax()).split_first() {
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
                // A node with tag or anchor properties stays on the `---`
                // line if it started there; a node without properties always
                // goes on its own line
                if document_node.has_properties() && get_lines_before(document_node.syntax()) == 0 {
                    write!(f, [space()])?;
                } else if comments.has_dangling_comments(node.syntax())
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
        // The dangling comments sit between the `---` marker and the
        // content, or close a document without content:
        //
        // ```yaml
        // --- # comment
        // content
        // ```
        //
        // They are printed with `format_dangling_comment`, one at a time,
        // by `fmt_fields` next to the marker; the default implementation
        // would print them again after the content
        Ok(())
    }
}
