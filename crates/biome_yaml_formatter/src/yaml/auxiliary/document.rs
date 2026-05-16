use crate::prelude::*;
use biome_formatter::write;
use biome_yaml_syntax::{YamlDocument, YamlDocumentFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlDocument;
impl FormatNodeRule<YamlDocument> for FormatYamlDocument {
    fn fmt_fields(&self, node: &YamlDocument, f: &mut YamlFormatter) -> FormatResult<()> {
        let YamlDocumentFields {
            bom_token,
            directives,
            dashdashdash_token,
            node,
            dotdotdot_token,
        } = node.as_fields();
        let has_document_start = dashdashdash_token.is_some();

        write!(f, [bom_token.format(), directives.format()])?;

        if let Some(token) = dashdashdash_token {
            write!(f, [token.format()])?;

            if node.is_some() {
                write!(f, [hard_line_break()])?;
            }
        }

        write!(f, [node.format()])?;

        if let Some(token) = dotdotdot_token {
            if node.is_some() || has_document_start {
                write!(f, [hard_line_break()])?;
            }

            write!(f, [token.format()])?;
        }

        Ok(())
    }
}
