use crate::FormatBogusNodeRule;
use biome_toml_syntax::TomlBogusTableHeader;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTomlBogusTableHeader;
impl FormatBogusNodeRule<TomlBogusTableHeader> for FormatTomlBogusTableHeader {}
