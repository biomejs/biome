use crate::prelude::*;
use crate::utils::write_arguments_multi_line;
use biome_formatter::write;
use biome_js_syntax::JsCallArgumentList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsCallArgumentList;

impl FormatRule<JsCallArgumentList> for FormatJsCallArgumentList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &JsCallArgumentList, f: &mut JsFormatter) -> FormatResult<()> {
        if node.len() == 0 {
            return Ok(());
        }

        write!(
            f,
            [&group(&soft_block_indent(&format_with(|f| {
                let separated = node
                    .format_separated(",")
                    .with_trailing_separator(TrailingSeparator::Omit);
                write_arguments_multi_line(separated, f)
            })))]
        )
    }
}
