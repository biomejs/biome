mod comma_groups;

use crate::CssFormatter;
use crate::comments::{CssCommentStyle, CssComments};
use crate::prelude::*;
use crate::utils::case::{
    identifier_has_escape, is_author_owned_property_value, value_identifier_case,
};
use crate::utils::scss_declaration_list::find_scss_declaration_list_group;
use biome_css_syntax::{
    CssFunction, CssGenericDelimiter, CssGenericProperty, CssIdentifier, CssLanguage,
    CssSyntaxKind, CssSyntaxNode, ScssExpression, ScssIncludeArgumentList,
    css_grid_template_property,
};
use biome_formatter::comments::{CommentKind, CommentStyle, SourceComment};
use biome_formatter::{CstFormatContext, FormatOptions, FormatResult, FormatWithRule, write};
use biome_rowan::{AstNode, AstNodeList, Text, TextSize};
use std::cmp;

/// Returns `true` if the node is a top-level comma delimiter in a component value list.
///
/// Note: commas inside nested constructs (e.g. function arguments like `rgba(0, 0, 0, 0.5)`)
/// are represented by different lists in the AST and won't be seen by this helper when scanning
/// the *outer* declaration value list.
fn is_comma_delimiter(node: &CssSyntaxNode) -> bool {
    let token_kind = CssGenericDelimiter::cast_ref(node)
        .and_then(|node| node.value().ok())
        .map(|token| token.kind());

    matches!(token_kind, Some(CssSyntaxKind::COMMA))
}

/// Returns `true` for comma-separated call arguments like `rgba(0, 0, 0, 0.5)`
/// or `@include mix(1px, 2px, $arg: 3px)`.
fn is_call_argument_list<N, I>(list: &N) -> bool
where
    N: AstNodeList<Language = CssLanguage, Node = I> + AstNode<Language = CssLanguage>,
    I: AstNode<Language = CssLanguage> + IntoFormat<CssFormatContext>,
{
    list.parent::<CssFunction>().is_some() || list.parent::<ScssIncludeArgumentList>().is_some()
}

/// Returns whether `node` is a comma-separated CSS declaration value list or
/// one group within an SCSS declaration value list.
///
/// CSS stores the comma in the component value list:
///
/// ```css
/// a {
///   box-shadow: 1px 1px red, 2px 2px blue;
/// }
/// ```
///
/// SCSS stores `$x $y` and `$z $w` in separate expression item lists while
/// their surrounding list owns the comma:
///
/// ```scss
/// a {
///   box-shadow: $x $y, $z $w;
/// }
/// ```
pub(crate) fn is_comma_separated_declaration_value_list(node: &CssSyntaxNode) -> bool {
    match node.kind() {
        CssSyntaxKind::CSS_GENERIC_COMPONENT_VALUE_LIST => {
            node.parent()
                .is_some_and(|parent| CssGenericProperty::can_cast(parent.kind()))
                && node.children().any(|child| is_comma_delimiter(&child))
        }
        CssSyntaxKind::SCSS_EXPRESSION_ITEM_LIST => {
            find_scss_declaration_list_group(node).is_some()
        }
        _ => false,
    }
}

/// Returns whether a movable block comment separates declaration values.
pub(crate) fn is_value_boundary_comment(comment: &SourceComment<CssLanguage>) -> bool {
    comment.kind() == CommentKind::InlineBlock
        && !CssCommentStyle::is_suppression(comment.piece().text())
}

/// Returns whether comments contain a movable boundary such as `a/* comment */b`.
pub(crate) fn has_value_boundary_comments(comments: &[SourceComment<CssLanguage>]) -> bool {
    comments.iter().any(is_value_boundary_comment)
}

pub(crate) fn write_component_value_list<N, I>(node: &N, f: &mut CssFormatter) -> FormatResult<()>
where
    N: AstNodeList<Language = CssLanguage, Node = I> + AstNode<Language = CssLanguage>,
    I: AstNode<Language = CssLanguage> + Clone + IntoFormat<CssFormatContext>,
    I::Format: FormatWithRule<CssFormatContext, Item = I>,
{
    let comments = f.context().comments().clone();
    let layout = get_value_list_layout(node, &comments, f);
    let lowercase_css_wide_keyword = should_lowercase_css_wide_keyword(node);
    let boundary_comments = comments.dangling_comments(node.syntax());
    let has_boundary_comments = has_value_boundary_comments(boundary_comments);
    let is_scss_list_group =
        has_boundary_comments && find_scss_declaration_list_group(node.syntax()).is_some();

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
        if let Some(comma_groups) =
            comma_groups::format_if_applicable(node.syntax(), layout, boundary_comments)
        {
            return write!(f, [comma_groups]);
        }

        if node.len() == 1 {
            let mut builder = f.join_nodes_with_soft_line();

            for element in node.iter() {
                let formatted =
                    format_component_value_element(element.clone(), lowercase_css_wide_keyword);
                builder.entry(element.syntax(), &formatted);
            }

            builder.finish()
        } else {
            let mut fill = f.fill();
            let mut at_group_boundary = false;

            for element in node.iter() {
                let formatted =
                    format_component_value_element(element.clone(), lowercase_css_wide_keyword);
                fill.entry(
                    &format_once(|f| {
                        let is_comma = is_comma_delimiter(element.syntax());
                        if !is_comma {
                            let starts_on_new_line = element.syntax().has_leading_newline();
                            if at_group_boundary {
                                layout.fmt_group_separator(starts_on_new_line, f)?;
                            } else {
                                layout.fmt_value_separator(starts_on_new_line, f)?;
                            }
                        }

                        // The outer layout adds the initial hard break because
                        // `FillBuilder` ignores its first separator.
                        at_group_boundary = is_comma
                            && matches!(
                                layout,
                                ValueListLayout::OneGroupPerLine
                                    | ValueListLayout::OneGroupPerLineWithDanglingComments
                            );

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
                } else if has_boundary_comments {
                    write!(f, [soft_line_break()])?;
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
            let content = format_once(|f| write!(f, [with_line_break, &values]));
            if is_scss_list_group {
                write!(f, [group(&content)])
            } else {
                write!(f, [indent(&group(&content))])
            }
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
        ValueListLayout::OneGroupPerLineWithDanglingComments => {
            write!(f, [group(&values)])
        }
    }
}

/// Formats an element with CSS-wide casing only when its list owns the complete value.
fn format_component_value_element<I>(
    element: I,
    lowercase_css_wide_keyword: bool,
) -> impl Format<CssFormatContext>
where
    I: AstNode<Language = CssLanguage> + IntoFormat<CssFormatContext>,
    I::Format: FormatWithRule<CssFormatContext, Item = I>,
{
    let case = if lowercase_css_wide_keyword {
        CssCase::Lowercase
    } else {
        CssCase::Preserve
    };
    element.into_format().with_text_case(case)
}

/// Matches complete CSS-wide keyword values such as `color: INITIAL`.
fn should_lowercase_css_wide_keyword<N, I>(list: &N) -> bool
where
    N: AstNodeList<Language = CssLanguage, Node = I> + AstNode<Language = CssLanguage>,
    I: AstNode<Language = CssLanguage>,
{
    let mut items = list.iter();
    let Some(item) = items.next() else {
        return false;
    };
    if items.next().is_some() {
        return false;
    }
    let Some(identifier) = CssIdentifier::cast_ref(item.syntax()) else {
        return false;
    };
    if value_identifier_case(&identifier) != CssCase::Lowercase {
        return false;
    }

    let Some(parent) = list.syntax().parent() else {
        return false;
    };

    match parent.kind() {
        CssSyntaxKind::CSS_ATTR_FALLBACK_VALUE | CssSyntaxKind::CSS_IF_BRANCH => true,
        CssSyntaxKind::CSS_GENERIC_PROPERTY => CssGenericProperty::cast(parent)
            .is_some_and(|property| property_normalizes_css_wide_keyword(&property)),
        CssSyntaxKind::SCSS_EXPRESSION => ScssExpression::cast(parent)
            .and_then(|expression| expression.parent::<CssGenericProperty>())
            .is_some_and(|property| property_normalizes_css_wide_keyword(&property)),
        _ => false,
    }
}

/// Returns whether a property owns a standard CSS-wide keyword value.
fn property_normalizes_css_wide_keyword(property: &CssGenericProperty) -> bool {
    let Ok(name) = property.name() else {
        return false;
    };
    let Some(identifier) = name.as_css_identifier() else {
        return false;
    };

    !identifier_has_escape(identifier) && !is_author_owned_property_value(property)
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

    /// Similar to OneGroupPerLine, but formats dangling comments on the property inline
    /// before the line break. Used when comments appear between the colon and values.
    /// ```css
    /// font-family: /* comment */
    ///     Hiragino Sans,
    ///     sans-serif;
    /// ```
    OneGroupPerLineWithDanglingComments,
}

impl ValueListLayout {
    fn is_source_break_preserving(self) -> bool {
        matches!(self, Self::PreserveInline | Self::OnePerLine)
    }

    fn fmt_value_separator(
        self,
        starts_on_new_line: bool,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        match self {
            Self::PreserveInline | Self::OnePerLine if starts_on_new_line => {
                write!(f, [hard_line_break()])
            }
            Self::PreserveInline | Self::OnePerLine => write!(f, [space()]),
            _ => write!(f, [soft_line_break_or_space()]),
        }
    }

    fn fmt_group_separator(
        self,
        starts_on_new_line: bool,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        match self {
            Self::OneGroupPerLine | Self::OneGroupPerLineWithDanglingComments => {
                write!(f, [hard_line_break()])
            }
            _ => self.fmt_value_separator(starts_on_new_line, f),
        }
    }
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
    comments: &CssComments,
    f: &CssFormatter,
) -> ValueListLayout
where
    N: AstNodeList<Language = CssLanguage, Node = I> + AstNode<Language = CssLanguage>,
    I: AstNode<Language = CssLanguage> + IntoFormat<CssFormatContext>,
{
    let parent_property = list.parent::<CssGenericProperty>();
    let scss_parent_property = find_scss_parent_property(list);
    let css_property = parent_property.as_ref().and_then(property_name);
    let is_grid_property = parent_property
        .as_ref()
        .or(scss_parent_property.as_ref())
        .and_then(property_name)
        .as_ref()
        .is_some_and(is_grid_template_property_name);

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

    // Comments between `:` and values need the dedicated group layout below.
    let has_trailing_comments = parent_property
        .as_ref()
        .is_some_and(|prop| !comments.trailing_comments(prop.syntax()).is_empty());
    let has_scss_trailing_comments = scss_parent_property
        .as_ref()
        .is_some_and(|prop| !comments.trailing_comments(prop.syntax()).is_empty());
    let has_scss_list_comments =
        scss_parent_property.is_some() && has_list_comments(list, comments);

    // In:
    // .grid {
    //   grid-template-areas: // row
    //     "header";
    // }
    // PreserveInline owns the string-row indent after the `:` comment.
    if is_grid_property && (has_scss_trailing_comments || !has_scss_list_comments) {
        ValueListLayout::PreserveInline
    } else if list.len() == 1 {
        ValueListLayout::SingleValue
    } else if use_one_group_per_line(css_property.as_deref(), list) {
        if has_trailing_comments {
            ValueListLayout::OneGroupPerLineWithDanglingComments
        } else {
            ValueListLayout::OneGroupPerLine
        }
    } else if is_comma_separated
        && text_size >= TextSize::from(f.options().line_width().value() as u32)
        && (value_count > 12 || is_call_argument_list(list))
    {
        ValueListLayout::OnePerLine
    } else {
        ValueListLayout::Fill
    }
}

fn property_name(property: &CssGenericProperty) -> Option<Text> {
    property
        .name()
        .ok()
        .and_then(|name| name.as_css_identifier().map(|name| name.to_trimmed_text()))
}

fn is_grid_template_property_name(name: &Text) -> bool {
    css_grid_template_property(name.text()).is_some()
}

fn has_list_comments<N, I>(list: &N, comments: &CssComments) -> bool
where
    N: AstNodeList<Language = CssLanguage, Node = I> + AstNode<Language = CssLanguage>,
    I: AstNode<Language = CssLanguage> + IntoFormat<CssFormatContext>,
{
    list.iter().any(|element| {
        comments.has_comments(element.syntax()) || comments.has_dangling_comments(element.syntax())
    })
}

/// Finds `property: <scss expression item list>`.
fn find_scss_parent_property<N, I>(list: &N) -> Option<CssGenericProperty>
where
    N: AstNodeList<Language = CssLanguage, Node = I> + AstNode<Language = CssLanguage>,
    I: AstNode<Language = CssLanguage> + IntoFormat<CssFormatContext>,
{
    list.parent::<ScssExpression>()
        .and_then(|expression| expression.parent::<CssGenericProperty>())
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
