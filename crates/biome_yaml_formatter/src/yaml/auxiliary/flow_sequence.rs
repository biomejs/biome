use crate::prelude::*;
use biome_formatter::write;
use biome_yaml_syntax::{AnyYamlFlowSequenceEntry, YamlFlowSequence, YamlFlowSequenceFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlFlowSequence;
impl FormatNodeRule<YamlFlowSequence> for FormatYamlFlowSequence {
    fn fmt_fields(&self, node: &YamlFlowSequence, f: &mut YamlFormatter) -> FormatResult<()> {
        let YamlFlowSequenceFields {
            l_brack_token,
            entries,
            r_brack_token,
        } = node.as_fields();
        let should_expand = entries
            .iter()
            .any(|entry| matches!(entry, Ok(AnyYamlFlowSequenceEntry::AnyYamlFlowMapEntry(_))));

        if entries.is_empty() {
            write!(f, [l_brack_token.format(), r_brack_token.format()])
        } else {
            write!(
                f,
                [
                    l_brack_token.format(),
                    group(&soft_block_indent_with_maybe_space(&entries.format(), false))
                        .should_expand(should_expand),
                    r_brack_token.format()
                ]
            )
        }
    }
}
