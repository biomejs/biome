use crate::prelude::*;
use crate::utils::block_scalar::format_block_scalar;
use biome_yaml_syntax::YamlLiteralScalar;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlLiteralScalar;
impl FormatNodeRule<YamlLiteralScalar> for FormatYamlLiteralScalar {
    fn fmt_fields(&self, node: &YamlLiteralScalar, f: &mut YamlFormatter) -> FormatResult<()> {
        let fields = node.as_fields();
        format_block_scalar(
            &fields.bitwise_or_token?,
            &fields.headers,
            &fields.content?,
            f,
        )
    }
}
