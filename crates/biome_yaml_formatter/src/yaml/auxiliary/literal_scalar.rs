use crate::prelude::*;
use crate::utils::block_scalar::{format_block_scalar, is_reindentable_block_scalar};
use biome_rowan::AstNode;
use biome_yaml_syntax::YamlLiteralScalar;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlLiteralScalar;
impl FormatNodeRule<YamlLiteralScalar> for FormatYamlLiteralScalar {
    fn fmt_fields(&self, node: &YamlLiteralScalar, f: &mut YamlFormatter) -> FormatResult<()> {
        let fields = node.as_fields();
        let content = fields.content?;
        if !is_reindentable_block_scalar(node.syntax(), &fields.headers, &content) {
            return format_verbatim_node(node.syntax()).fmt(f);
        }
        format_block_scalar(&fields.bitwise_or_token?, &fields.headers, &content, f)
    }
}
