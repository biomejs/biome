use crate::prelude::*;
use biome_formatter::trivia::format_dangling_comments;
use biome_formatter::write;
use biome_rowan::AstNode;
use biome_yaml_syntax::{YamlLiteralScalar, YamlLiteralScalarFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlLiteralScalar;
impl FormatNodeRule<YamlLiteralScalar> for FormatYamlLiteralScalar {
    fn fmt_fields(&self, node: &YamlLiteralScalar, f: &mut YamlFormatter) -> FormatResult<()> {
        let YamlLiteralScalarFields {
            bitwise_or_token,
            headers,
            content,
        } = node.as_fields();

        write!(f, [bitwise_or_token.format(), headers.format()])?;

        // The comment on the header line
        if f.comments().has_dangling_comments(node.syntax()) {
            write!(f, [space(), format_dangling_comments(node.syntax())])?;
        }

        write!(f, [content?.format()])
    }

    fn fmt_dangling_comments(
        &self,
        _: &YamlLiteralScalar,
        _: &mut YamlFormatter,
    ) -> FormatResult<()> {
        // The dangling comment sits on the header line:
        //
        // ```yaml
        // key: | # comment
        //   content
        // ```
        //
        // It is printed inside `fmt_fields` right after the header; the
        // default implementation would print it again after the content
        Ok(())
    }
}
