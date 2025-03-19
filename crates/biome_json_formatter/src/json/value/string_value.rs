use crate::prelude::*;
use biome_json_syntax::JsonStringValue;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsonStringValue;

impl FormatNodeRule<JsonStringValue> for FormatJsonStringValue {
    fn fmt_fields(&self, node: &JsonStringValue, f: &mut JsonFormatter) -> FormatResult<()> {
        node.value_token()?.format().fmt(f)
    }
}
