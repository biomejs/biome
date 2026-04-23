use crate::JsdocComment;
use biome_formatter::prelude::*;
use biome_formatter::{
    FormatContext, FormatOptions, IndentStyle, IndentWidth, LineEnding, LineWidth,
    SourceMapGeneration, TrailingNewline, TransformSourceMap, format_args, write,
};
use biome_rowan::TextSize;
use std::ops::Deref;

pub struct JsDocFormatContext;

impl FormatContext for JsDocFormatContext {
    type Options = JsCommentFormatOptions;

    fn options(&self) -> &Self::Options {
        &JsCommentFormatOptions
    }

    fn source_map(&self) -> Option<&TransformSourceMap> {
        None
    }
}

pub struct JsCommentFormatOptions;

impl FormatOptions for JsCommentFormatOptions {
    fn indent_style(&self) -> IndentStyle {
        IndentStyle::Space
    }

    fn indent_width(&self) -> IndentWidth {
        IndentWidth::try_from(2).unwrap()
    }

    fn line_width(&self) -> LineWidth {
        LineWidth::default()
    }

    fn line_ending(&self) -> LineEnding {
        LineEnding::Lf
    }

    fn trailing_newline(&self) -> TrailingNewline {
        TrailingNewline::default()
    }

    fn as_print_options(&self) -> PrinterOptions {
        PrinterOptions {
            indent_width: self.indent_width(),
            print_width: self.line_width().into(),
            line_ending: self.line_ending(),
            indent_style: self.indent_style(),
            source_map_generation: SourceMapGeneration::default(),
        }
    }
}

impl Format<JsDocFormatContext> for JsdocComment {
    fn fmt(&self, f: &mut Formatter<JsDocFormatContext>) -> FormatResult<()> {
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
