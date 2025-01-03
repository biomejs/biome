use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritBubble, GritBubbleFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritBubble;
impl FormatNodeRule<GritBubble> for FormatGritBubble {
    fn fmt_fields(&self, node: &GritBubble, f: &mut GritFormatter) -> FormatResult<()> {
        let GritBubbleFields {
            pattern,
            bubble_token,
            scope,
        } = node.as_fields();

        write!(
            f,
            [
                bubble_token.format(),
                scope.format(),
                space(),
                pattern.format()
            ]
        )
    }
}
