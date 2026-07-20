use crate::prelude::*;
use biome_formatter::{format_args, write};
use biome_yaml_syntax::{YamlFlowMapping, YamlFlowMappingFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlFlowMapping;
impl FormatNodeRule<YamlFlowMapping> for FormatYamlFlowMapping {
    fn fmt_fields(&self, node: &YamlFlowMapping, f: &mut YamlFormatter) -> FormatResult<()> {
        let YamlFlowMappingFields {
            l_curly_token,
            entries,
            r_curly_token,
        } = node.as_fields();
        let should_expand = f.comments().has_comments(node.syntax());

        write!(
            f,
            [group(&format_args![
                l_curly_token.format(),
                should_expand.then_some(expand_parent()),
                soft_block_indent_with_maybe_space(&entries.format(), !entries.is_empty()),
                r_curly_token.format()
            ])]
        )
    }
}
