use crate::FormatBogusNodeRule;
use biome_toml_syntax::TomlBogusMember;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTomlBogusMember;
impl FormatBogusNodeRule<TomlBogusMember> for FormatTomlBogusMember {}
