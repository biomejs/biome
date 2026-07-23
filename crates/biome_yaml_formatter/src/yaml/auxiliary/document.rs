use crate::prelude::*;
use biome_formatter::trivia::format_dangling_comments;
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
        // necessarily go on their own line.
        let first_comment_lines_before = f
            .comments()
            .dangling_comments(node.syntax())
            .first()
            .map(|comment| comment.lines_before());
        if let Some(lines_before) = first_comment_lines_before {
            if lines_before == 0 {
                write!(f, [space()])?;
            } else {
                write!(f, [hard_line_break()])?;
            }
            write!(f, [format_dangling_comments(node.syntax())])?;
        }

        if let Some(document_node) = &document_node {
            if dashdashdash_token.is_some() {
                // A node with tag or anchor properties stays on the `---`
                // line if it started there, as Prettier does. A node without
                // properties always goes on its own line
                if document_node.has_properties() && get_lines_before(document_node.syntax()) == 0 {
                    write!(f, [space()])?;
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
        // Handled inside `fmt_fields` so they are printed next to the `---` marker
        Ok(())
    }
}
