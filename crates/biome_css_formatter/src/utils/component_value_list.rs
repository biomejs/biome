use crate::comments::CssComments;
use biome_css_syntax::{CssGenericDelimiter, CssLanguage, CssSyntaxKind};
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
        let mut fill = f.fill();

        for (element, formatted) in node.iter().zip(node.iter().formatted()) {
            fill.entry(
                &format_once(|f| {
                    // If the current element is not a comma, insert a soft line break or a space.
                    // Consider the CSS example: `font: first , second;`
                    // The desired format is: `font: first, second;`
                    // A separator should not be added before the comma because the comma acts as a `CssGenericDelimiter`.
                    let is_comma = CssGenericDelimiter::cast_ref(element.syntax())
                        .and_then(|node| node.value().ok())
                        .map_or(false, |node| node.kind() == CssSyntaxKind::COMMA);

                    if !is_comma {
                        write!(f, [soft_line_break_or_space()])?
                    }

                    Ok(())
                }),
                &formatted,
            );
        }

        fill.finish()
    });

    match layout {
        ValueListLayout::Fill => {
            write!(f, [group(&indent(&values))])
        }
        ValueListLayout::SingleValue => {
            write!(f, [values])
        }
        // TODO: Add formatting for one-per-line once comma-separated lists are supported.
        ValueListLayout::OnePerLine => write!(f, [format_verbatim_node(node.syntax())]),
    }
}

#[derive(Copy, Clone, Debug)]
pub(crate) enum ValueListLayout {
    /// Ensures the usage of a singular, consistent value.
    ///
    /// ```css
    /// :root {
    ///     --bs-gradient: linear-gradient(
    ///         180deg,
    ///         180deg,
    ///         180deg,
    ///         180deg,
    ///         180deg,
    ///         180deg,
    ///         180deg
    ///     );
    /// }
    /// ```
    SingleValue,

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
        ValueListLayout::SingleValue
    } else {
        ValueListLayout::Fill
    }
}
