use crate::prelude::*;
use biome_formatter::write;
use biome_yaml_syntax::{YamlAliasNode, YamlAliasNodeFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlAliasNode;
impl FormatNodeRule<YamlAliasNode> for FormatYamlAliasNode {
    fn fmt_fields(&self, node: &YamlAliasNode, f: &mut YamlFormatter) -> FormatResult<()> {
        let YamlAliasNodeFields { value_token } = node.as_fields();

        write!(f, [value_token.format()])
    }
}
