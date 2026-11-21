use crate::FormatBogusNodeRule;
use biome_toml_syntax::TomlBogusKey;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTomlBogusKey;
impl FormatBogusNodeRule<TomlBogusKey> for FormatTomlBogusKey {}
