use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::{TsPropertyParameter, TsPropertyParameterFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsPropertyParameter;

impl FormatNodeRule<TsPropertyParameter> for FormatTsPropertyParameter {
    fn fmt_fields(&self, node: &TsPropertyParameter, f: &mut JsFormatter) -> FormatResult<()> {
        let TsPropertyParameterFields {
            decorators,
            modifiers,
            formal_parameter,
        } = node.as_fields();

        let content = format_with(|f| {
            write![
                f,
                [
                    decorators.format(),
                    modifiers.format(),
                    space(),
                    formal_parameter.format()
                ]
            ]
        });

        if decorators.is_empty() {
            write![f, [content]]
        } else {
            write![f, [group(&content)]]
        }
    }
}
