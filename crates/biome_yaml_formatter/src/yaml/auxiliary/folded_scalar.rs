use crate::prelude::*;
use crate::utils::block_scalar::format_block_scalar;
use biome_yaml_syntax::YamlFoldedScalar;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlFoldedScalar;
impl FormatNodeRule<YamlFoldedScalar> for FormatYamlFoldedScalar {
    fn fmt_fields(&self, node: &YamlFoldedScalar, f: &mut YamlFormatter) -> FormatResult<()> {
        let fields = node.as_fields();
        format_block_scalar(&fields.r_angle_token?, &fields.headers, &fields.content?, f)
    }
}
