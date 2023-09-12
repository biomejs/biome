use crate::FormatBogusNodeRule;
use biome_js_syntax::JsBogusStatement;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsBogusStatement;

impl FormatBogusNodeRule<JsBogusStatement> for FormatJsBogusStatement {}
