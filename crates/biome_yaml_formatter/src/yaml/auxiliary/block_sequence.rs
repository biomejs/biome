use crate::prelude::*;
use biome_formatter::write;
use biome_yaml_syntax::{YamlBlockSequence, YamlBlockSequenceFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlBlockSequence;
impl FormatNodeRule<YamlBlockSequence> for FormatYamlBlockSequence {
    fn fmt_fields(&self, node: &YamlBlockSequence, f: &mut YamlFormatter) -> FormatResult<()> {
        let YamlBlockSequenceFields {
            sequence_start_token,
            entries,
            sequence_end_token,
        } = node.as_fields();

        write!(
            f,
            [
                format_removed(&sequence_start_token?), // always empty, ignored
                entries.format(),
                format_removed(&sequence_end_token?), // always empty, ignored
            ]
        )
    }
}
