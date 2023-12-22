use crate::{comments::CssComments, prelude::*};
use biome_css_syntax::CssComponentValueList;
use biome_formatter::{write, CstFormatContext};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssComponentValueList;
impl FormatRule<CssComponentValueList> for FormatCssComponentValueList {
    type Context = CssFormatContext;

    fn fmt(&self, node: &CssComponentValueList, f: &mut CssFormatter) -> FormatResult<()> {
        let layout = get_value_list_layout(node, f.context().comments());

        match layout {
            ValueListLayout::Fill => {
                let values = format_with(|f: &mut Formatter<'_, CssFormatContext>| {
                    f.fill()
                        .entries(&soft_line_break_or_space(), node.iter().formatted())
                        .finish()
                });

                write!(f, [group(&indent(&values))])
            }
            // TODO: Add formatting for one-per-line once comma-separated lists are supported.
            ValueListLayout::OnePerLine => write!(f, [format_verbatim_node(node.syntax())]),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum ValueListLayout {
    /// Tries to fit as many values on a single line as possible, then wraps
    /// and indents the next line to keep filling on that line, and so on.
    ///
    /// ```css
    /// background: red blue white
    ///     green orange rgba(0, 0, 0, 1)
    ///     black blue;
    /// ```
    Fill,

    /// Prints every value on a single line if the whole list exceeds the line
    /// width, or any of its elements gets printed in *expanded* mode.
    /// ```css
    /// font-family:
    ///     "Lato",
    ///     -apple-system,
    ///     "Helvetica Neue",
    ///     Helvetica,
    ///     Arial,
    ///     sans-serif;
    /// ```
    #[allow(unused)]
    OnePerLine,
}

/// Returns the layout to use when printing the provided CssComponentValueList.
/// Until the parser supports comma-separated lists, this will always return
/// [ValueListLayout::Fill], since all space-separated lists are intentionally
/// printed compactly.
fn get_value_list_layout(_list: &CssComponentValueList, _: &CssComments) -> ValueListLayout {
    // TODO: Check for comments, check for the types of elements in the list, etc.
    ValueListLayout::Fill
}
