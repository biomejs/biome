use crate::prelude::*;
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

        if bom_token.is_some()
            || directives.len() > 0
            || dashdashdash_token.is_some()
            || dotdotdot_token.is_some()
        {
            // TODO: Implement formatting for YAML document markers, directives, BOMs, and end markers.
            return format_verbatim_node(node.syntax()).fmt(f);
        }

        write!(f, [document_node.format()])
    }
}
