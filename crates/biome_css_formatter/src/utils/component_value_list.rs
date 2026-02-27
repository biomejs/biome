use crate::CssFormatter;
use crate::comments::CssComments;
use crate::prelude::*;
use biome_css_syntax::{CssGenericDelimiter, CssGenericProperty, CssLanguage, CssSyntaxKind};
use biome_formatter::{CstFormatContext, format_args, write};
use biome_formatter::{FormatOptions, FormatResult};
use biome_rowan::{AstNode, AstNodeList, TextSize};
use biome_string_case::StrLikeExtension;
use std::cmp;

/// Returns `true` if the node is a top-level comma delimiter in a component value list.
///
/// Note: commas inside nested constructs (e.g. function arguments like `rgba(0, 0, 0, 0.5)`)
/// are represented by different lists in the AST and won't be seen by this helper when scanning
/// the *outer* declaration value list.
fn is_comma_delimiter<I>(node: &I) -> bool
where
    I: AstNode<Language = CssLanguage>,
{
    let token_kind = CssGenericDelimiter::cast_ref(node.syntax())
        .and_then(|node| node.value().ok())
        .map(|token| token.kind());

    matches!(token_kind, Some(CssSyntaxKind::COMMA))
}

/// Applies a Prettier-like wrapping strategy for comma-separated *declaration values* when using
/// `ValueListLayout::Fill`.
///
/// `Fill` may break at any `soft_line_break_or_space()`, which can split a comma-group across lines.
/// For declaration values with **top-level commas**, we instead group on commas and fill **groups**
/// so wrapping prefers breaking between groups (at commas).
///
/// Example:
/// ```css
/// /* Before (can break inside a group) */
/// --shadow: 0 2px 4px rgba(...), 0 8px
///   16px rgba(...);
///
/// /* After (prefer breaking at the comma) */
/// --shadow:
///   0 2px 4px rgba(...),
///   0 8px 16px rgba(...);
/// ```
fn try_write_fill_comma_groups<N, I>(
    node: &N,
    layout: ValueListLayout,
    f: &mut Formatter<'_, CssFormatContext>,
) -> Option<FormatResult<()>>
where
    N: AstNodeList<Language = CssLanguage, Node = I> + AstNode<Language = CssLanguage>,
    I: AstNode<Language = CssLanguage> + IntoFormat<CssFormatContext>,
{
    if !matches!(layout, ValueListLayout::Fill) {
        return None;
    }

    // Only apply this behaviour for declaration values.
    // This prevents the comma-group logic from leaking into unrelated list-like constructs.
    let is_declaration_value_list = node.parent::<CssGenericProperty>().is_some();
    if !is_declaration_value_list {
        return None;
    }

    let has_top_level_comma = node.iter().any(|element| is_comma_delimiter(&element));
    if !has_top_level_comma {
        return None;
    }

    Some(write_fill_comma_groups(node, f))
}

/// Formats a comma-separated declaration value as a sequence of comma-groups.
///
/// Each comma-group is formatted using a nested `Fill`, and the groups themselves are then
/// `Fill`-joined with `soft_line_break_or_space()`. This means:
/// - If the whole value fits on one line, it stays inline.
/// - If it doesn't fit, wrapping prefers splitting between groups (at commas), while still allowing
///   a single long group to wrap internally if necessary.
fn write_fill_comma_groups<N, I>(
    node: &N,
    f: &mut Formatter<'_, CssFormatContext>,
) -> FormatResult<()>
where
    N: AstNodeList<Language = CssLanguage, Node = I> + AstNode<Language = CssLanguage>,
    I: AstNode<Language = CssLanguage> + IntoFormat<CssFormatContext>,
{
    let mut groups: Vec<Vec<I>> = Vec::new();
    let mut current_group: Vec<I> = Vec::new();

    for element in node.iter() {
        let is_comma = is_comma_delimiter(&element);
        // Keep the comma token in the current group so we print `group1,` before breaking to `group2`.
        current_group.push(element);

        if is_comma {
            groups.push(current_group);
            current_group = Vec::new();
        }
    }

    if !current_group.is_empty() {
        groups.push(current_group);
    }

    let group_separator = soft_line_break_or_space();
    let mut outer_fill = f.fill();

    for group_items in &groups {
        let content = format_with(|f: &mut Formatter<'_, CssFormatContext>| {
            let mut inner_fill = f.fill();

            for element in group_items {
                let is_comma = is_comma_delimiter(element);
                let formatted = element.clone().into_format();

                inner_fill.entry(
                    &format_once(|f| {
                        // Avoid inserting a separator before commas (e.g. `value , value`).
                        if !is_comma {
                            write!(f, [soft_line_break_or_space()])?;
                        }
                        Ok(())
                    }),
                    &formatted,
                );
            }

            inner_fill.finish()
        });

        // Group each comma-group so wrapping prefers breaking between groups (at commas).
        outer_fill.entry(&group_separator, &group(&content));
    }

    outer_fill.finish()
}

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
        if node.len() == 1 {
            let mut builder = f.join_nodes_with_soft_line();

            for (element, formatted) in node.iter().zip(node.iter().formatted()) {
                builder.entry(element.syntax(), &formatted);
            }

            builder.finish()
        } else {
            // Prefer breaking at top-level commas in declaration values (Prettier-like)
            // by filling comma-separated groups instead of individual tokens.
            // This prevents line wraps inside groups like `0px 8px\n16px` for `box-shadow`-like values.
            if let Some(result) = try_write_fill_comma_groups(node, layout, f) {
                return result;
            }

            let mut fill = f.fill();
            let mut at_group_boundary = false;

            for (element, formatted) in node.iter().zip(node.iter().formatted()) {
                fill.entry(
                    &format_once(|f| {
                        // If the current element is not a comma, insert a soft line break or a space.
                        // Consider the CSS example: `font: first , second;`
                        // The desired format is: `font: first, second;`
                        // A separator should not be added before the comma because the comma acts as a `CssGenericDelimiter`.
                        let is_comma = is_comma_delimiter(&element);

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
                            } else if at_group_boundary {
                                write!(f, [hard_line_break()])?;
                            } else {
                                write!(f, [soft_line_break_or_space()])?
                            }
                        }

                        // If the layout is OneGroupPerLine, insert a hard line break as a `separator`
                        // between two adjacent groups.
                        //
                        // Consider the CSS example: `font: group one, group_two, group 3;`
                        // The desired format is:
                        // font:
                        //   group one,
                        //   group_two,
                        //   group 3;
                        //
                        // A hard line break is inserted between:
                        // 1. `group one,` and `group_two,`
                        // 2. `group_two,` and `group 3;`
                        //
                        // Caveat:
                        // We also need to add a hard line break before the first group,
                        // but `FillBuilder.entry` will ignore any `separator` for the first item in the list,
                        // To address this, we prepend the hard line break after composing `values`.
                        //
                        // This is also why `at_group_boundary` is initialized to `false` even when
                        // the layout is OneGroupPerLine: because the line break would be ignored
                        // if `at_group_boundary` were set to `true` initially.
                        at_group_boundary =
                            is_comma && matches!(layout, ValueListLayout::OneGroupPerLine);

                        Ok(())
                    }),
                    &formatted,
                );
            }

            fill.finish()
        }
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
            let with_line_break = format_with(|f| {
                if should_preceded_by_softline(node) {
                    write!(f, [soft_line_break()])?;
                }
                Ok(())
            });
            write!(f, [indent(&group(&format_args![with_line_break, &values]))])
        }
        ValueListLayout::SingleValue => {
            write!(f, [values])
        }
        ValueListLayout::OnePerLine | ValueListLayout::OneGroupPerLine => {
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

    /// Separate values by comma into multiple groups, and print each group on a single line
    /// ```css
    ///   transition:
    ///     color 0.15s ease-in-out,
    ///     background-color 0.15s ease-in-out,
    ///     border-color 0.15s ease-in-out,
    ///     box-shadow 0.15s ease-in-out;
    /// ```
    ///
    /// This layout is only applied when following conditions are met:
    /// 1. The value list is a direct child of a CSS property declaration
    /// 2. The CSS property is not a custom property (i.e., does not start with "--").
    /// 3. Values are separated into multiple groups by comma
    /// 4. At least one of the groups contains two or more values
    ///
    /// These conditions are inherited from Prettier,
    /// see https://github.com/biomejs/biome/pull/5334 for a detailed explanation
    OneGroupPerLine,
}

fn should_preceded_by_softline<N, I>(node: &N) -> bool
where
    N: AstNodeList<Language = CssLanguage, Node = I> + AstNode<Language = CssLanguage>,
    I: AstNode<Language = CssLanguage> + IntoFormat<CssFormatContext>,
{
    node.iter()
        .any(|element| CssGenericDelimiter::can_cast(element.syntax().kind()))
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
    let css_property = list
        .parent::<CssGenericProperty>()
        .and_then(|parent| parent.name().ok())
        .and_then(|name| name.as_css_identifier().map(|name| name.to_trimmed_text()));
    let is_grid_property = css_property.as_ref().is_some_and(|name| {
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
    } else if use_one_group_per_line(css_property.as_deref(), list) {
        ValueListLayout::OneGroupPerLine
    } else if is_comma_separated
        && value_count > 12
        && text_size >= TextSize::from(f.options().line_width().value() as u32)
    {
        ValueListLayout::OnePerLine
    } else {
        ValueListLayout::Fill
    }
}

pub(crate) fn use_one_group_per_line<N, I>(css_property: Option<&str>, list: &N) -> bool
where
    N: AstNodeList<Language = CssLanguage, Node = I> + AstNode<Language = CssLanguage>,
    I: AstNode<Language = CssLanguage> + IntoFormat<CssFormatContext>,
{
    let is_css_property = css_property.is_some();
    let is_custom_property = css_property.is_some_and(|name| name.starts_with("--"));
    if !is_css_property || is_custom_property {
        return false;
    }

    let mut group_count = 0;
    let mut group_size = 0;
    let mut max_group_size = 0;

    // Iterate over the value list to determine the number of groups
    // and the size of the largest group.
    //
    // There are two situations where we need to update the group count
    // and the maximum group size:
    // 1. When encountering a group separator (comma), as it signals the end of a group.
    // 2. When finishing iteration, since the last group ends with a semicolon,
    //    but the semicolon is not included in the value list.
    //    Therefore, we update the last group after iterating through all items.
    for item in list.iter() {
        let token_kind = CssGenericDelimiter::cast_ref(item.syntax())
            .and_then(|node| node.value().ok())
            .map(|token| token.kind());
        if matches!(token_kind, Some(CssSyntaxKind::COMMA)) {
            group_count += 1;
            max_group_size = cmp::max(group_size, max_group_size);
            group_size = 0;
            continue;
        }
        group_size += 1;
    }
    group_count += 1;
    max_group_size = cmp::max(group_size, max_group_size);

    group_count >= 2 && max_group_size >= 2
}
