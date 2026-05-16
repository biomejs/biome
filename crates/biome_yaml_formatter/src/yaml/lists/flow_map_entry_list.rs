use crate::prelude::*;
use crate::separated::FormatAstSeparatedListExtension;
use biome_formatter::separated::TrailingSeparator;
use biome_yaml_syntax::YamlFlowMapEntryList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlFlowMapEntryList;
impl FormatRule<YamlFlowMapEntryList> for FormatYamlFlowMapEntryList {
    type Context = YamlFormatContext;
    fn fmt(&self, node: &YamlFlowMapEntryList, f: &mut YamlFormatter) -> FormatResult<()> {
        f.join_with(&soft_line_break_or_space())
            .entries(node.format_separated(",", TrailingSeparator::Omit))
            .finish()
    }
}
