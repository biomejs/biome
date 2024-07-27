use crate::FormatBogusNodeRule;
use biome_js_syntax::TsBogusType;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTsBogusType;

impl FormatBogusNodeRule<TsBogusType> for FormatTsBogusType {}
