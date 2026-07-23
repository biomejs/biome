use crate::prelude::*;
use biome_css_syntax::{
    AnyCssCustomPropertyComponent, CssCustomPropertyBracedBlock, CssCustomPropertyBracketedBlock,
    CssCustomPropertyComponentList, CssCustomPropertyFunction, CssCustomPropertyParenthesizedBlock,
    CssLanguage, CssSyntaxNode, CssSyntaxToken, T,
};
use biome_formatter::{format_args, write};
use biome_rowan::{AstNode, AstNodeList, SyntaxResult, declare_node_union};

declare_node_union! {
    /// A balanced container in a raw custom-property value.
    pub(crate) CustomPropertyContainer =
        CssCustomPropertyFunction
        | CssCustomPropertyParenthesizedBlock
        | CssCustomPropertyBracketedBlock
        | CssCustomPropertyBracedBlock
}

impl CustomPropertyContainer {
    fn open_token(&self) -> SyntaxResult<CssSyntaxToken> {
        match self {
            Self::CssCustomPropertyFunction(node) => node.l_paren_token(),
            Self::CssCustomPropertyParenthesizedBlock(node) => node.l_paren_token(),
            Self::CssCustomPropertyBracketedBlock(node) => node.l_brack_token(),
            Self::CssCustomPropertyBracedBlock(node) => node.l_curly_token(),
        }
    }

    fn components(&self) -> CssCustomPropertyComponentList {
        match self {
            Self::CssCustomPropertyFunction(node) => node.components(),
            Self::CssCustomPropertyParenthesizedBlock(node) => node.components(),
            Self::CssCustomPropertyBracketedBlock(node) => node.components(),
            Self::CssCustomPropertyBracedBlock(node) => node.components(),
        }
    }

    fn close_token(&self) -> SyntaxResult<CssSyntaxToken> {
        match self {
            Self::CssCustomPropertyFunction(node) => node.r_paren_token(),
            Self::CssCustomPropertyParenthesizedBlock(node) => node.r_paren_token(),
            Self::CssCustomPropertyBracketedBlock(node) => node.r_brack_token(),
            Self::CssCustomPropertyBracedBlock(node) => node.r_curly_token(),
        }
    }

    /// Returns the component list when the container has no parsed content.
    pub(crate) fn empty_components(&self) -> Option<CssCustomPropertyComponentList> {
        let components = self.components();
        components.is_empty().then_some(components)
    }
}

#[derive(Clone, Copy, Debug)]
enum CustomPropertyComponentSeparator {
    None,
    SoftLine,
}

#[derive(Clone, Copy, Debug)]
enum CustomPropertyCommaGroupLayout {
    /// Breaks every group when `fn(a, b)` or `(a, b)` expands.
    Lines,
    /// Packs bracketed groups such as `[a, b]` up to the line width.
    Fill,
}

impl Format<CssFormatContext> for CustomPropertyComponentSeparator {
    fn fmt(&self, f: &mut CssFormatter) -> FormatResult<()> {
        match self {
            Self::None => Ok(()),
            Self::SoftLine => write!(f, [soft_line_break_or_space()]),
        }
    }
}

impl Format<CssFormatContext> for CustomPropertyContainer {
    fn fmt(&self, f: &mut CssFormatter) -> FormatResult<()> {
        match self {
            Self::CssCustomPropertyFunction(_) | Self::CssCustomPropertyParenthesizedBlock(_) => {
                self.fmt_parenthesized(f)
            }
            Self::CssCustomPropertyBracketedBlock(_) => self.fmt_bracketed(f),
            Self::CssCustomPropertyBracedBlock(_) => self.fmt_preserved(f),
        }
    }
}

impl CustomPropertyContainer {
    fn fmt_parenthesized(&self, f: &mut CssFormatter) -> FormatResult<()> {
        let components = self.components();
        let open = self.open_token();
        let close = self.close_token();
        if open.is_err() || close.is_err() || self.requires_source_preservation(&components) {
            return self.fmt_preserved(f);
        }

        let comma_groups = components
            .iter()
            .any(|component| is_comma(&component))
            .then(|| collect_comma_groups(&components));
        let formatted_components = format_with(|f| {
            if components.is_empty() {
                return components.format().fmt(f);
            }

            if let Some(comma_groups) = &comma_groups {
                format_comma_groups(comma_groups, CustomPropertyCommaGroupLayout::Lines, f)
            } else {
                let filled = format_with(|f| format_filled_components(components.iter(), f));
                write!(f, [group(&indent(&filled))])
            }
        });

        write!(
            f,
            [group(&format_args![
                open.format(),
                soft_block_indent(&formatted_components),
                line_suffix_boundary(),
                close.format()
            ])]
        )
    }

    fn fmt_bracketed(&self, f: &mut CssFormatter) -> FormatResult<()> {
        let components = self.components();
        let open = self.open_token();
        let close = self.close_token();
        if open.is_err() || close.is_err() || self.requires_source_preservation(&components) {
            return self.fmt_preserved(f);
        }

        let comma_groups = components
            .iter()
            .any(|component| is_comma(&component))
            .then(|| collect_comma_groups(&components));

        if let Some(comma_groups) = comma_groups {
            let formatted_groups = format_with(|f| {
                format_comma_groups(&comma_groups, CustomPropertyCommaGroupLayout::Fill, f)
            });
            write!(
                f,
                [indent(&group(&format_args![
                    soft_line_break(),
                    open.format(),
                    formatted_groups,
                    close.format()
                ]))]
            )
        } else {
            let formatted_components = format_with(|f| {
                if components.is_empty() {
                    components.format().fmt(f)
                } else {
                    format_filled_components(components.iter(), f)
                }
            });
            write!(
                f,
                [group(&indent(&format_args![
                    open.format(),
                    formatted_components,
                    close.format()
                ]))]
            )
        }
    }

    fn fmt_preserved(&self, f: &mut CssFormatter) -> FormatResult<()> {
        let components = self.components();
        let open = self.open_token();
        let close = self.close_token();
        let first = components.syntax().first_token();
        let (space_after_open, space_before_close) = self.braced_boundary_spaces(f);
        let line_break_after_open = first
            .as_ref()
            .or_else(|| close.as_ref().ok())
            .is_some_and(CssSyntaxToken::has_leading_newline);
        let line_break_before_close = first.is_some()
            && close
                .as_ref()
                .ok()
                .is_some_and(CssSyntaxToken::has_leading_newline);
        let is_multiline = line_break_after_open
            || line_break_before_close
            || components
                .iter()
                .skip(1)
                .any(|component| component.syntax().has_leading_newline());

        write!(f, [open.format()])?;

        if is_multiline {
            let content = format_with(|f| {
                if line_break_after_open {
                    write!(f, [hard_line_break()])?;
                } else {
                    write!(f, [maybe_space(space_after_open)])?;
                }

                write!(f, [components.format()])
            });

            write!(f, [indent(&content)])?;
        } else {
            write!(f, [maybe_space(space_after_open), components.format()])?;
        }

        if line_break_before_close {
            write!(f, [hard_line_break()])?;
        } else {
            write!(f, [maybe_space(space_before_close)])?;
        }

        write!(f, [close.format()])
    }

    fn requires_source_preservation(&self, components: &CssCustomPropertyComponentList) -> bool {
        is_inside_custom_property_braced_block(self.syntax())
            || has_preserved_double_slash(self, components)
    }

    /// Returns the source spaces around the contents of a raw braced block.
    fn braced_boundary_spaces(&self, f: &CssFormatter) -> (bool, bool) {
        if !matches!(self, Self::CssCustomPropertyBracedBlock(_)) {
            return (false, false);
        }

        let components = self.components();
        let open = self.open_token();
        let close = self.close_token();
        let dangling_comments = f.comments().dangling_comments(components.syntax());
        let first_content_start = components
            .syntax()
            .first_token()
            .map(|token| token.text_trimmed_range().start())
            .or_else(|| {
                dangling_comments
                    .first()
                    .map(|comment| comment.piece().text_range().start())
            })
            .or_else(|| {
                close
                    .as_ref()
                    .ok()
                    .map(|token| token.text_trimmed_range().start())
            });
        let last_content_end = components
            .syntax()
            .last_token()
            .map(|token| token.text_trimmed_range().end())
            .or_else(|| {
                dangling_comments
                    .last()
                    .map(|comment| comment.piece().text_range().end())
            });

        let after_open = open
            .as_ref()
            .ok()
            .zip(first_content_start)
            .is_some_and(|(open, start)| open.text_trimmed_range().end() < start);
        let before_close = close
            .as_ref()
            .ok()
            .zip(last_content_end)
            .is_some_and(|(close, end)| end < close.text_trimmed_range().start());

        (after_open, before_close)
    }
}

fn collect_comma_groups(
    components: &CssCustomPropertyComponentList,
) -> Vec<Vec<AnyCssCustomPropertyComponent>> {
    let mut groups = Vec::new();
    let mut current = Vec::new();

    for component in components.iter() {
        let is_comma = is_comma(&component);
        current.push(component);

        if is_comma {
            // Keep the comma with the preceding group so breaks occur after it.
            groups.push(std::mem::take(&mut current));
        }
    }

    if !current.is_empty() {
        groups.push(current);
    }

    groups
}

fn format_comma_groups(
    groups: &[Vec<AnyCssCustomPropertyComponent>],
    layout: CustomPropertyCommaGroupLayout,
    f: &mut CssFormatter,
) -> FormatResult<()> {
    let separator = soft_line_break_or_space();

    match layout {
        CustomPropertyCommaGroupLayout::Lines => {
            let mut joiner = f.join_with(&separator);
            for components in groups {
                joiner.entry(&format_comma_group(components));
            }
            joiner.finish()
        }
        CustomPropertyCommaGroupLayout::Fill => {
            let mut fill = f.fill();
            for components in groups {
                fill.entry(&separator, &format_comma_group(components));
            }
            fill.finish()
        }
    }
}

fn format_comma_group(
    components: &[AnyCssCustomPropertyComponent],
) -> impl Format<CssFormatContext> + '_ {
    format_with(move |f| {
        let filled = format_with(|f| format_filled_components(components.iter().cloned(), f));
        write!(f, [group(&indent(&filled))])
    })
}

fn format_filled_components(
    components: impl IntoIterator<Item = AnyCssCustomPropertyComponent>,
    f: &mut CssFormatter,
) -> FormatResult<()> {
    let mut fill = f.fill();
    let mut previous = None;

    for component in components {
        let separator = component_separator(previous.as_ref(), &component);
        fill.entry(&separator, &component.format());
        previous = Some(component);
    }

    fill.finish()
}

fn component_separator(
    previous: Option<&AnyCssCustomPropertyComponent>,
    current: &AnyCssCustomPropertyComponent,
) -> CustomPropertyComponentSeparator {
    let Some(previous) = previous else {
        return CustomPropertyComponentSeparator::None;
    };

    if is_comma(current) {
        return CustomPropertyComponentSeparator::None;
    }

    if is_comma(previous) {
        return CustomPropertyComponentSeparator::SoftLine;
    }

    let Some(previous) = previous.syntax().last_token() else {
        return CustomPropertyComponentSeparator::None;
    };
    let Some(current) = current.syntax().first_token() else {
        return CustomPropertyComponentSeparator::None;
    };

    if has_source_gap(&previous, &current) {
        CustomPropertyComponentSeparator::SoftLine
    } else {
        CustomPropertyComponentSeparator::None
    }
}

fn is_comma(component: &AnyCssCustomPropertyComponent) -> bool {
    delimiter_token(component).is_some_and(|token| token.kind() == T![,])
}

fn delimiter_token(component: &AnyCssCustomPropertyComponent) -> Option<CssSyntaxToken> {
    component
        .as_css_custom_property_delimiter()
        .and_then(|delimiter| delimiter.value().ok())
}

fn is_inside_custom_property_braced_block(node: &CssSyntaxNode) -> bool {
    node.ancestors()
        .skip(1)
        .any(|ancestor| CssCustomPropertyBracedBlock::can_cast(ancestor.kind()))
}

fn has_preserved_double_slash<N>(node: &N, components: &CssCustomPropertyComponentList) -> bool
where
    N: AstNode<Language = CssLanguage>,
{
    let mut previous = None;
    if components.iter().any(|component| {
        let Some(token) = delimiter_token(&component) else {
            previous = None;
            return false;
        };
        let is_double_slash = previous
            .as_ref()
            .is_some_and(|previous| is_source_tight_double_slash(previous, &token));
        previous = Some(token);
        is_double_slash
    }) {
        return true;
    }

    // The raw `//` may start before the container, as in `--value: // (\n);`.
    let Some(parent_components) = node.parent::<CssCustomPropertyComponentList>() else {
        return false;
    };
    let Some(mut right) = node.syntax().first_token() else {
        return false;
    };
    let components_start = parent_components.syntax().text_trimmed_range().start();

    while let Some(left) = right.prev_token() {
        if right.has_leading_newline() || left.text_trimmed_range().start() < components_start {
            break;
        }
        if is_source_tight_double_slash(&left, &right) {
            return true;
        }
        right = left;
    }

    false
}

fn is_source_tight_double_slash(left: &CssSyntaxToken, right: &CssSyntaxToken) -> bool {
    left.kind() == T![/]
        && right.kind() == T![/]
        && left.text_trimmed_range().end() == right.text_trimmed_range().start()
}

/// Returns whether `node` is a structured raw custom-property component.
pub(crate) fn is_raw_custom_property_component<N>(node: &N) -> bool
where
    N: AstNode<Language = CssLanguage>,
{
    node.parent::<CssCustomPropertyComponentList>().is_some()
}

/// Returns whether two raw components had separating trivia in the source.
pub(crate) fn has_source_gap(left: &CssSyntaxToken, right: &CssSyntaxToken) -> bool {
    left.text_trimmed_range().end() < right.text_trimmed_range().start()
}
