use crate::prelude::*;
use biome_formatter::write;
use biome_yaml_syntax::{YamlBlockSequence, YamlBlockSequenceFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlBlockSequence;
impl FormatNodeRule<YamlBlockSequence> for FormatYamlBlockSequence {
    fn fmt_fields(&self, node: &YamlBlockSequence, f: &mut YamlFormatter) -> FormatResult<()> {
        let YamlBlockSequenceFields {
            sequence_start_token: _,
            entries,
            sequence_end_token: _,
        } = node.as_fields();

        write!(f, [entries.format()])
    }
}
