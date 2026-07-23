use crate::prelude::*;
use crate::utils::ends_in_keep_chomped_scalar;
use biome_formatter::{FormatOptions, write};
use biome_rowan::{AstNode, AstNodeList};
use biome_yaml_syntax::{YamlRoot, YamlRootFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlRoot;
impl FormatNodeRule<YamlRoot> for FormatYamlRoot {
    fn fmt_fields(&self, node: &YamlRoot, f: &mut YamlFormatter) -> FormatResult<()> {
        let YamlRootFields {
            documents,
            eof_token,
        } = node.as_fields();

        write!(f, [documents.format()])?;

        // A keep-chomped block scalar owns every line break that follows
        // it and prints them itself, so when one closes the last document,
        // Prettier emits no final line break of its own — even after a
        // `...` marker or a trailing comment that follows the scalar
        let ends_in_keep_scalar = documents
            .iter()
            .last()
            .is_some_and(|document| ends_in_keep_chomped_scalar(document.syntax()));

        if f.options().trailing_newline().value() && !ends_in_keep_scalar {
            write!(f, [hard_line_break()])?;
        }

        write!(f, [format_removed(&eof_token?)])
    }

    fn fmt_leading_comments(&self, node: &YamlRoot, f: &mut YamlFormatter) -> FormatResult<()> {
        // The comments of a document without content lead the root. They
        // keep the blank lines between them, but the blank lines after the
        // last one are dropped, as Prettier does:
        //
        // ```yaml
        // # Comment
        // ```
        //
        // Cheap clone of an `Rc`, releasing the borrow on the formatter so
        // the comments can be written while iterating over them
        let comments = f.comments().clone();
        for (index, comment) in comments.leading_comments(node.syntax()).iter().enumerate() {
            if index > 0 {
                if comment.lines_before() > 1 {
                    write!(f, [empty_line()])?;
                } else {
                    write!(f, [hard_line_break()])?;
                }
            }
            write!(
                f,
                [biome_formatter::FormatRefWithRule::new(
                    comment,
                    crate::comments::FormatYamlLeadingComment
                )]
            )?;
            comment.mark_formatted();
        }
        Ok(())
    }
}
