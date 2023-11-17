use crate::prelude::*;
use biome_formatter::write;
use biome_json_syntax::{JsonRoot, JsonRootFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsonRoot;

impl FormatNodeRule<JsonRoot> for FormatJsonRoot {
    fn fmt_fields(&self, node: &JsonRoot, f: &mut JsonFormatter) -> FormatResult<()> {
        let JsonRootFields {
            bom_token,
            value,
            eof_token,
        } = node.as_fields();

        match &value {
            Ok(value) => {
                write!(
                    f,
                    [
                        bom_token.format(),
                        format_or_verbatim(value.format()),
                        format_removed(&eof_token?),
                        hard_line_break()
                    ]
                )
            }
            // Don't fail formatting if the root contains no root value
            Err(_) => {
                write!(f, [format_verbatim_node(node.syntax())])
            }
        }
    }
}
