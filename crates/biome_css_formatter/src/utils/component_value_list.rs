use crate::comments::CssComments;
use biome_css_syntax::CssLanguage;
use biome_formatter::FormatResult;
use biome_formatter::{write, CstFormatContext};

use crate::prelude::*;
use crate::CssFormatter;
use biome_rowan::{AstNode, AstNodeList};

pub(crate) fn write_component_value_list<N, I>(node: &N, f: &mut CssFormatter) -> FormatResult<()>
where
    N: AstNodeList<Language = CssLanguage, Node = I> + AstNode<Language = CssLanguage>,
    I: AstNode<Language = CssLanguage> + IntoFormat<CssFormatContext>,
{
    let layout = get_value_list_layout(node, f.context().comments());
    let values = format_with(|f: &mut Formatter<'_, CssFormatContext>| {
        f.fill()
            .entries(&soft_line_break_or_space(), node.iter().formatted())
            .finish()
    });

    match layout {
        ValueListLayout::Fill => {
            write!(f, [group(&indent(&values))])
        }
        ValueListLayout::SingleLine => {
            write!(f, [values])
        }
        // TODO: Add formatting for one-per-line once comma-separated lists are supported.
        ValueListLayout::OnePerLine => write!(f, [format_verbatim_node(node.syntax())]),
    }
}

#[derive(Copy, Clone, Debug)]
pub(crate) enum ValueListLayout {
    /// Places all values on a single line.
    ///
    /// ```css
    /// :root {
    /// 	--bs-gradient: linear-gradient(
    /// 		180deg,
    /// 		180deg,
    /// 		180deg,
    /// 		180deg,
    /// 		180deg,
    /// 		180deg,
    /// 		180deg
    /// 	);
    /// }
    /// ```
    SingleLine,

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
pub(crate) fn get_value_list_layout<N, I>(list: &N, _: &CssComments) -> ValueListLayout
where
    N: AstNodeList<Language = CssLanguage, Node = I> + AstNode<Language = CssLanguage>,
    I: AstNode<Language = CssLanguage> + IntoFormat<CssFormatContext>,
{
    // TODO: Check for comments, check for the types of elements in the list, etc.
    if list.len() == 1 {
        ValueListLayout::SingleLine
    } else {
        ValueListLayout::Fill
    }
}
