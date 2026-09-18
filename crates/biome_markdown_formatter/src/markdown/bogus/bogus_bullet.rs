use crate::FormatBogusNodeRule;
use biome_markdown_syntax::MdBogusBullet;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdBogusBullet;
impl FormatBogusNodeRule<MdBogusBullet> for FormatMdBogusBullet {}
