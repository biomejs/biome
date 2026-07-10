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
            node: document_node,
            dotdotdot_token,
        } = node.as_fields();

        write!(f, [bom_token.format(), directives.format(),])?;

        if let Some(dashdashdash_token) = dashdashdash_token {
            write!(f, [dashdashdash_token.format()])?;
        }

        write!(f, [document_node.format()])?;

        if let Some(dotdotdot_token) = dotdotdot_token {
            write!(f, [dotdotdot_token.format()])?;
        }

        Ok(())
    }
}
