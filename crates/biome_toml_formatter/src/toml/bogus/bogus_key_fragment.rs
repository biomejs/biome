use crate::FormatBogusNodeRule;
use biome_toml_syntax::TomlBogusKeyFragment;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTomlBogusKeyFragment;
impl FormatBogusNodeRule<TomlBogusKeyFragment> for FormatTomlBogusKeyFragment {}
