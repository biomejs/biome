use crate::prelude::*;
use biome_formatter::{FormatRuleWithOptions, write};
use biome_markdown_syntax::{MdListMarkerPrefix, MdListMarkerPrefixFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdListMarkerPrefix {
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
        let marker_text = marker.text_trimmed();
        let target = self.target_marker.unwrap_or("-");

        write!(f, [pre_marker_indent.format()])?;

        if marker_text != target {
            write!(f, [format_replaced(&marker, &token(target))])?;
        } else {
            write!(f, [marker.format()])?;
        }

        if let Some(space) = post_marker_space_token {
            write!(f, [space.format()])?;
        }
        write!(f, [content_indent.format()])
    }
}

pub(crate) struct FormatMdListMarkerPrefixOptions {
    pub(crate) target_marker: &'static str,
}

impl FormatRuleWithOptions<MdListMarkerPrefix> for FormatMdListMarkerPrefix {
    type Options = FormatMdListMarkerPrefixOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.target_marker = Some(options.target_marker);
        self
    }
}
