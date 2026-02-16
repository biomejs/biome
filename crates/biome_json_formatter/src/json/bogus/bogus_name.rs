use crate::FormatBogusNodeRule;
use biome_json_syntax::JsonBogusName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsonBogusName;
impl FormatBogusNodeRule<JsonBogusName> for FormatJsonBogusName {}
