use crate::prelude::*;
use crate::utils::block_scalar::{FormatBlockScalar, is_reindentable_block_scalar};
use biome_rowan::AstNode;
use biome_yaml_syntax::YamlFoldedScalar;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlFoldedScalar;
impl FormatNodeRule<YamlFoldedScalar> for FormatYamlFoldedScalar {
    fn fmt_fields(&self, node: &YamlFoldedScalar, f: &mut YamlFormatter) -> FormatResult<()> {
        let fields = node.as_fields();
        let content = fields.content?;
        if !is_reindentable_block_scalar(node.syntax(), &fields.headers, &content) {
            // TODO: reindent nested and irregularly indented block scalars
            // instead of emitting them verbatim.
            return format_verbatim_node(node.syntax()).fmt(f);
        }
        let opener = fields.r_angle_token?;
        FormatBlockScalar {
            opener: &opener,
            headers: &fields.headers,
            content: &content,
        }
        .fmt(f)
    }
}
