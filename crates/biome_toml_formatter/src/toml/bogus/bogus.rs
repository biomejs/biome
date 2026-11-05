use crate::FormatBogusNodeRule;
use biome_toml_syntax::TomlBogus;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTomlBogus;
impl FormatBogusNodeRule<TomlBogus> for FormatTomlBogus {}
