use crate::FormatBogusNodeRule;
use biome_toml_syntax::TomlBogusValue;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTomlBogusValue;
impl FormatBogusNodeRule<TomlBogusValue> for FormatTomlBogusValue {}
