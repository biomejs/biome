use crate::prelude::*;
use biome_formatter::{format_args, write};
use biome_yaml_syntax::{YamlFlowSequence, YamlFlowSequenceFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlFlowSequence;
impl FormatNodeRule<YamlFlowSequence> for FormatYamlFlowSequence {
    fn fmt_fields(&self, node: &YamlFlowSequence, f: &mut YamlFormatter) -> FormatResult<()> {
        let YamlFlowSequenceFields {
            l_brack_token,
            entries,
            r_brack_token,
        } = node.as_fields();
        let should_expand = f.comments().has_comments(node.syntax());

        write!(
            f,
            [group(&format_args![
                l_brack_token.format(),
                soft_block_indent(&entries.format()),
                r_brack_token.format()
            ])
            .should_expand(should_expand)]
        )
    }
}
