use crate::prelude::*;
use crate::separated::FormatAstSeparatedListExtension;
use biome_formatter::separated::TrailingSeparator;
use biome_yaml_syntax::YamlFlowSequenceEntryList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlFlowSequenceEntryList;
impl FormatRule<YamlFlowSequenceEntryList> for FormatYamlFlowSequenceEntryList {
    type Context = YamlFormatContext;
    fn fmt(&self, node: &YamlFlowSequenceEntryList, f: &mut YamlFormatter) -> FormatResult<()> {
        f.join_with(&soft_line_break_or_space())
            .entries(node.format_separated(",", TrailingSeparator::Omit))
            .finish()
    }
}
