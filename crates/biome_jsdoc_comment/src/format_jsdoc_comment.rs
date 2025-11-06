use crate::JsdocComment;
use biome_formatter::prelude::*;
use biome_formatter::{format_args, write};
use biome_js_type_info::FormatTypeContext;
use biome_rowan::TextSize;
use std::ops::Deref;

impl Format<FormatTypeContext> for JsdocComment {
    fn fmt(
        &self,
        f: &mut biome_formatter::formatter::Formatter<FormatTypeContext>,
    ) -> FormatResult<()> {
        let comment = self.deref();

        let comment = format_with(|f| {
            let mut joiner = f.join_with(hard_line_break());
            comment.lines().for_each(|line| {
                joiner.entry(&format_args![text(line.trim(), TextSize::default()),]);
            });
            joiner.finish()
        });

        write!(
            f,
            [&format_args![
                token("JsDoc"),
                token("("),
                block_indent(&comment),
                token(")")
            ]]
        )
    }
}
