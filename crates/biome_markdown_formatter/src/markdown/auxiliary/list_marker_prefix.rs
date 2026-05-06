use crate::prelude::*;
use biome_formatter::{FormatRuleWithOptions, write};
use biome_markdown_syntax::{MdListMarkerPrefix, MdListMarkerPrefixFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdListMarkerPrefix {
    /// Target marker to replace with (e.g. `"-"`). `None` keeps the original.
    target_marker: Option<&'static str>,
}
impl FormatNodeRule<MdListMarkerPrefix> for FormatMdListMarkerPrefix {
    fn fmt_fields(&self, node: &MdListMarkerPrefix, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdListMarkerPrefixFields {
            pre_marker_indent,
            marker,
            post_marker_space_token,
            content_indent,
        } = node.as_fields();

        let marker = marker?;

        write!(f, [pre_marker_indent.format()])?;
        // Note that for `-   `, the parser treats the indent as part of the marker, not the content
        // This is a parser bug that causes a regression
        // in crates/biome_markdown_formatter/tests/specs/prettier/markdown/spec/example-242.md.snap
        match self.target_marker {
            Some(target) => write!(f, [format_replaced(&marker, &token(target))])?,
            None => write!(f, [marker.format()])?,
        }

        if let Some(space) = post_marker_space_token {
            write!(f, [space.format()])?;
        }
        write!(f, [content_indent.format()])
    }
}

pub(crate) struct FormatMdListMarkerPrefixOptions {
    /// Target marker to replace with (e.g. `Some("-")`). `None` keeps the original.
    pub(crate) target_marker: Option<&'static str>,
}

impl FormatRuleWithOptions<MdListMarkerPrefix> for FormatMdListMarkerPrefix {
    type Options = FormatMdListMarkerPrefixOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.target_marker = options.target_marker;
        self
    }
}
