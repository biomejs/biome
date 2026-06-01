use crate::prelude::*;
use biome_formatter::{FormatOptions, write};
use biome_yaml_syntax::{AnyYamlDocument, YamlRoot, YamlRootFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlRoot;
impl FormatNodeRule<YamlRoot> for FormatYamlRoot {
    fn fmt_fields(&self, node: &YamlRoot, f: &mut YamlFormatter) -> FormatResult<()> {
        let YamlRootFields {
            documents,
            eof_token: _,
        } = node.as_fields();

        if node.syntax().text_trimmed().to_string().contains('#') {
            // TODO: Implement stable formatting for comments in YAML documents.
            return format_verbatim_node(node.syntax()).fmt(f);
        }

        if documents.iter().any(|document| {
            matches!(
                document,
                AnyYamlDocument::YamlDocument(document)
                    if document.bom_token().is_some()
                        || document.directives().len() > 0
                        || document.dashdashdash_token().is_some()
                        || document.dotdotdot_token().is_some()
            )
        }) {
            // TODO: Implement formatting for YAML document markers, directives, BOMs, and end markers.
            return format_verbatim_node(node.syntax()).fmt(f);
        }

        write!(f, [documents.format()])?;

        if f.options().trailing_newline().value() {
            write!(f, [hard_line_break()])
        } else {
            Ok(())
        }
    }
}
