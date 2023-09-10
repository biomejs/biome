use crate::FormatBogusNodeRule;
use biome_js_syntax::JsBogusBinding;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsBogusBinding;

impl FormatBogusNodeRule<JsBogusBinding> for FormatJsBogusBinding {}
