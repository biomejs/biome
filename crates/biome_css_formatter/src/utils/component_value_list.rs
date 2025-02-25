use crate::comments::CssComments;
use biome_css_syntax::{CssGenericDelimiter, CssGenericProperty, CssLanguage, CssSyntaxKind};
use biome_formatter::{write, CstFormatContext};
use biome_formatter::{FormatOptions, FormatResult};
use biome_string_case::StrLikeExtension;

use crate::prelude::*;
use crate::CssFormatter;
use biome_rowan::{AstNode, AstNodeList, TextSize};

pub(crate) fn write_component_value_list<N, I>(node: &N, f: &mut CssFormatter) -> FormatResult<()>
where
    N: AstNodeList<Language = CssLanguage, Node = I> + AstNode<Language = CssLanguage>,
    I: AstNode<Language = CssLanguage> + IntoFormat<CssFormatContext>,
{
    let layout = get_value_list_layout(node, f.context().comments(), f);

    // Check if any of the elements in the list have a leading newline.
    // We skip the first element because it is the first element in the list and should not be considered.
    // div {
    //     grid-template-columns:
    //                          1fr 100px 3em;
    // }
    let has_newline = match layout {
        ValueListLayout::PreserveInline => node
            .iter()
            .skip(1)
            .any(|element| element.syntax().has_leading_newline()),
        _ => false,
    };

    let values = format_with(|f: &mut Formatter<'_, CssFormatContext>| {
        let mut fill = f.fill();

        for (element, formatted) in node.iter().zip(node.iter().formatted()) {
            fill.entry(
                &format_once(|f| {
                    // If the current element is not a comma, insert a soft line break or a space.
                    // Consider the CSS example: `font: first , second;`
                    // The desired format is: `font: first, second;`
                    // A separator should not be added before the comma because the comma acts as a `CssGenericDelimiter`.
                    let token_kind = CssGenericDelimiter::cast_ref(element.syntax())
                        .and_then(|node| node.value().ok())
                        .map(|token| token.kind());

                    let is_comma = matches!(token_kind, Some(CssSyntaxKind::COMMA));

                    if !is_comma {
                        if matches!(
                            layout,
                            ValueListLayout::PreserveInline | ValueListLayout::OnePerLine
                        ) {
                            let has_leading_newline = element.syntax().has_leading_newline();

                            if has_leading_newline {
                                write!(f, [hard_line_break()])?;
                            } else {
                                write!(f, [space()])?;
                            }
                        } else {
                            write!(f, [soft_line_break_or_space()])?
                        }
                    }

                    Ok(())
                }),
                &formatted,
            );
        }

        fill.finish()
    });

    match layout {
        ValueListLayout::PreserveInline => {
            let content = format_once(|f| {
                if has_newline {
                    // Add line break before the first element if we have more than two lines.
                    write!(f, [hard_line_break()])?;
                }
                write!(f, [values])
            });

            write!(f, [group(&indent(&content))])
        }
        ValueListLayout::Fill => {
            write!(f, [group(&indent(&values))])
        }
        ValueListLayout::SingleValue => {
            write!(f, [values])
        }
        ValueListLayout::OnePerLine => {
            let content = format_once(|f| {
                write!(f, [hard_line_break()])?;
                write!(f, [values])
            });

            write!(f, [group(&indent(&content))])
        }
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

    /// Keeps elements on the same line if they're on the same line in the source file.
    ///
    /// For example, this layout option is commonly used for CSS grid properties. It ensures that properties
    /// remain on the same line in the formatted output if they were on the same line in the source file.
    /// If a new line is encountered in the source file, a corresponding new line is added in the formatted
    /// output at the beginning of the property.
    ///
    /// # Example
    ///
    /// ```css
    /// grid-template-areas: 'header header' 'main sidebar' 'footer footer';
    ///   grid-template-columns:
    ///       [full-start] minmax(1.50em, 1fr)
    ///       [main-start] minmax(.40ch, 75ch)
    ///       [main-end] minmax(1em, 1.000fr)
    ///       [full-end];
    /// ```
    PreserveInline,

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
    OnePerLine,
}

/// Returns the layout to use when printing the provided CssComponentValueList.
/// Until the parser supports comma-separated lists, this will always return
/// [ValueListLayout::Fill], since all space-separated lists are intentionally
/// printed compactly.
pub(crate) fn get_value_list_layout<N, I>(
    list: &N,
    _: &CssComments,
    f: &CssFormatter,
) -> ValueListLayout
where
    N: AstNodeList<Language = CssLanguage, Node = I> + AstNode<Language = CssLanguage>,
    I: AstNode<Language = CssLanguage> + IntoFormat<CssFormatContext>,
{
    let is_grid_property = list
        .parent::<CssGenericProperty>()
        .and_then(|parent| parent.name().ok())
        .and_then(|name| {
            name.as_css_identifier()
                .map(|name| name.to_trimmed_string())
        })
        .is_some_and(|name| {
            let name = name.to_ascii_lowercase_cow();

            name.starts_with("grid-template") || name == "grid"
        });

    let text_size: TextSize = list
        .iter()
        .filter(|x| x.range().len() > TextSize::from(1))
        .map(|x| x.range().len())
        .sum();
    let value_count = list
        .iter()
        .filter(|x| x.range().len() > TextSize::from(1))
        .count();

    let is_comma_separated = list
        .iter()
        .any(|x| CssGenericDelimiter::cast_ref(x.syntax()).is_some());

    // TODO: Check for comments, check for the types of elements in the list, etc.
    if is_grid_property {
        ValueListLayout::PreserveInline
    } else if list.len() == 1 {
        ValueListLayout::SingleValue
    } else if is_comma_separated
        && value_count > 12
        && text_size >= TextSize::from(f.options().line_width().value() as u32)
    {
        ValueListLayout::OnePerLine
    } else {
        ValueListLayout::Fill
    }
}
