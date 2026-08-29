use crate::markdown::auxiliary::textual::FormatMdTextualOptions;
use crate::prelude::*;
use biome_formatter::write;
use biome_markdown_syntax::GfmTaskListItem;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGfmTaskListItem;

impl FormatNodeRule<GfmTaskListItem> for FormatGfmTaskListItem {
    fn fmt_fields(&self, node: &GfmTaskListItem, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let fields = node.as_fields();
        let state = fields.state?;
        write!(
            f,
            [
                fields.l_bracket_token.format(),
                state.format().with_options(FormatMdTextualOptions {
                    normalize_task_state: true,
                    ..FormatMdTextualOptions::default()
                }),
                fields.r_bracket_token.format(),
            ]
        )
    }
}
