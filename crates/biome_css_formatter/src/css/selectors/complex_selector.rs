use crate::comments::{CssComments, is_selector_boundary_suppressed};
use crate::prelude::*;
use crate::utils::comment_trivia::{FormatCommentGap, has_line_comment};
use biome_css_syntax::{
    AnyCssSelector, CssComplexSelector, CssComplexSelectorFields, CssLanguage, CssSyntaxKind,
    CssSyntaxToken,
};
use biome_formatter::comments::SourceComment;
use biome_formatter::trivia::{format_dangling_comment, should_nestle_adjacent_doc_comments};
use biome_formatter::{FormatRuleWithOptions, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssComplexSelector {
    layout: SelectorChainLayout,
}

/// Layout selected once for a complete complex-selector chain.
///
/// `CssComplexSelector` is recursive, so nested selectors receive the resolved
/// layout instead of making independent break decisions.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub(crate) enum SelectorChainLayout {
    /// Pick the layout from the whole complex-selector chain, then pass the
    /// resolved layout to nested complex selectors.
    #[default]
    Auto,

    /// Use the normal selector layout, allowing soft breaks at combinators.
    Flexible,

    /// Disable optional chain breaks while preserving comment-required breaks.
    Flat,
}

impl SelectorChainLayout {
    /// Resolves `Auto` once for the outer formatter invocation.
    ///
    /// Resolved layouts pass through unchanged when formatting recursive
    /// complex-selector children.
    fn resolve(self, node: &CssComplexSelector, comments: &CssComments) -> FormatResult<Self> {
        match self {
            Self::Auto
                if is_trailing_separator_followed_by_line_comment(node)
                    || is_flat_chain_layout_required(node, comments)? =>
            {
                Ok(Self::Flat)
            }
            Self::Auto => Ok(Self::Flexible),
            layout => Ok(layout),
        }
    }
}

impl FormatRuleWithOptions<CssComplexSelector> for FormatCssComplexSelector {
    type Options = SelectorChainLayout;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.layout = options;
        self
    }
}

impl FormatNodeRule<CssComplexSelector> for FormatCssComplexSelector {
    fn fmt_fields(&self, node: &CssComplexSelector, f: &mut CssFormatter) -> FormatResult<()> {
        let CssComplexSelectorFields {
            left,
            combinator,
            right,
        } = node.as_fields();
        let left = left?;
        let combinator = combinator?;
        let right = right?;
        let owns_chain_layout = self.layout == SelectorChainLayout::Auto;
        let is_left_preceded_by_comment = is_left_selector_preceded_by_comment(&left);
        let comments = f.comments().clone();
        let layout = self.layout.resolve(node, &comments)?;
        let boundary = SelectorBoundary::new(
            &combinator,
            &right,
            comments.dangling_comments(node.syntax()),
        );
        let formatted_combinator = format_once(|f| {
            // A descendant combinator only requires some whitespace. Removing
            // the source token lets the separators choose that whitespace.
            if combinator.kind() == CssSyntaxKind::CSS_SPACE_LITERAL {
                write!(f, [format_removed(&combinator)])
            } else {
                write!(f, [combinator.format()])
            }
        });
        let combinator_separator = format_once(|f| boundary.fmt_after(f));

        if owns_chain_layout
            && layout == SelectorChainLayout::Flexible
            && is_left_preceded_by_comment
        {
            // Keep the selector next to its leading comment when the tail fits,
            // but retain the normal break opportunity for over-width chains.
            let formatted_right = format_selector(&right, layout);
            let formatted_tail = format_with(|f| {
                write!(
                    f,
                    [formatted_combinator, combinator_separator, formatted_right]
                )
            })
            .memoized();
            let flat = format_with(|f| {
                boundary.fmt_before(true, f)?;
                formatted_tail.fmt(f)
            });
            let expanded = format_with(|f| {
                boundary.fmt_before(false, f)?;
                formatted_tail.fmt(f)
            });

            return write!(
                f,
                [
                    format_selector(&left, layout),
                    best_fitting!(flat, expanded)
                ]
            );
        }

        let selector_separator =
            format_once(|f| boundary.fmt_before(layout == SelectorChainLayout::Flat, f));
        let formatted = format_once(|f| {
            write!(
                f,
                [
                    format_selector(&left, layout),
                    selector_separator,
                    formatted_combinator,
                    combinator_separator,
                    format_selector(&right, layout)
                ]
            )
        });

        if owns_chain_layout && layout == SelectorChainLayout::Flat && !is_left_preceded_by_comment
        {
            // `CssSelectorList` adds one indent around a complex selector. Hard
            // boundary breaks should align with that selector, so remove the
            // list-only indent once rather than at every recursive child.
            write!(f, [dedent(&formatted)])
        } else {
            write!(f, [formatted])
        }
    }

    fn fmt_dangling_comments(
        &self,
        _: &CssComplexSelector,
        _: &mut CssFormatter,
    ) -> FormatResult<()> {
        // The default dangling formatter cannot place comments on a specific
        // side of the combinator. `fmt_fields` formats both boundary slots.
        Ok(())
    }
}

/// Formats comments within one `left combinator right` boundary.
///
/// ```css
/// .a /* before */ > /* after */ .b
/// ```
///
/// These comments are dangling on the complex selector, but their source side
/// determines where they print. Descendant combinators are whitespace tokens
/// that are removed during formatting, so this type also restores any required
/// source line breaks from that token.
struct SelectorBoundary<'a> {
    before_combinator: &'a [SourceComment<CssLanguage>],
    after_combinator: &'a [SourceComment<CssLanguage>],
    descendant_lines: Option<u32>,
    right_lines: u32,
}

impl<'a> SelectorBoundary<'a> {
    /// Splits source-ordered comments around the combinator and records source
    /// lines that disappear when a descendant combinator is removed.
    fn new(
        combinator: &CssSyntaxToken,
        right: &AnyCssSelector,
        comments: &'a [SourceComment<CssLanguage>],
    ) -> Self {
        let combinator_range = combinator.text_trimmed_range();
        // Comment slices are source ordered. Keep the original side of the
        // combinator while retaining borrowed subslices for formatting.
        let before_end = comments.partition_point(|comment| {
            comment.piece().text_range().end() <= combinator_range.start()
        });
        let (before_combinator, after_combinator) = comments.split_at(before_end);
        let descendant_lines = (combinator.kind() == CssSyntaxKind::CSS_SPACE_LITERAL)
            .then(|| count_line_breaks(combinator.text_trimmed()));
        let right_lines = get_lines_before(right.syntax()).min(2) as u32;

        Self {
            before_combinator,
            after_combinator,
            descendant_lines,
            right_lines,
        }
    }

    /// Formats comments between the left selector and combinator.
    fn fmt_before(&self, fixed_separator: bool, f: &mut CssFormatter) -> FormatResult<()> {
        let Some(first) = self.before_combinator.first() else {
            return if fixed_separator {
                write!(f, [space()])
            } else {
                write!(f, [soft_line_break_or_space()])
            };
        };

        // In:
        // .a /* c */
        // .b
        //
        // the descendant combinator owns the first line break, while additional
        // blank lines are leading trivia on `.b`. Removing the combinator would
        // otherwise discard those source gaps.
        let final_source_lines = if self.after_combinator.is_empty() {
            self.descendant_lines
                .map_or(0, |lines| lines.saturating_add(self.right_lines).min(2))
        } else {
            0
        };

        FormatSelectorBoundaryComments::new(
            self.before_combinator,
            first.lines_before(),
            final_source_lines,
        )
        .fmt(f)
    }

    /// Formats comments between the combinator and right selector.
    fn fmt_after(&self, f: &mut CssFormatter) -> FormatResult<()> {
        let Some(first) = self.after_combinator.first() else {
            // A boundary comment before an explicit combinator anchors its
            // source layout, including a following break before the selector.
            let source_lines =
                if self.descendant_lines.is_none() && !self.before_combinator.is_empty() {
                    self.right_lines
                } else {
                    0
                };
            return FormatCommentGap::new(source_lines).fmt(f);
        };

        // `SourceComment::lines_before` does not include line breaks contained
        // in the descendant-combinator token itself.
        let lines_before = first.lines_before().saturating_add(u32::from(
            self.descendant_lines.is_some_and(|lines| lines > 0),
        ));

        FormatSelectorBoundaryComments::new(self.after_combinator, lines_before, 0).fmt(f)
    }
}

/// Formats recursive selectors with the layout resolved for the complete chain.
fn format_selector(
    selector: &AnyCssSelector,
    layout: SelectorChainLayout,
) -> impl Format<CssFormatContext> + '_ {
    format_with(move |f| match selector {
        AnyCssSelector::CssComplexSelector(selector) => {
            selector.format().with_options(layout).fmt(f)
        }
        _ => selector.format().fmt(f),
    })
}

/// Returns whether the left compound selector starts with an attached comment.
pub(crate) fn is_left_selector_preceded_by_comment(selector: &AnyCssSelector) -> bool {
    selector
        .as_css_compound_selector()
        .and_then(|compound_selector| compound_selector.syntax().first_token())
        .is_some_and(|token| token.has_leading_comments())
}

/// Returns whether the following list separator owns a line comment.
fn is_trailing_separator_followed_by_line_comment(node: &CssComplexSelector) -> bool {
    let Some(comma_token) = node
        .syntax()
        .last_token()
        .and_then(|token| token.next_token())
        .filter(|token| token.kind() == CssSyntaxKind::COMMA)
    else {
        return false;
    };

    has_line_comment(comma_token.trailing_trivia())
}

/// Returns whether any boundary in this recursive selector chain needs fixed
/// separators.
///
/// Resolving this once at the layout-owning invocation prevents an inner
/// comment-required break from combining with an optional ancestor break. The
/// resolved layout is then passed to child selectors, so the chain is scanned
/// only once.
fn is_flat_chain_layout_required(
    node: &CssComplexSelector,
    comments: &CssComments,
) -> FormatResult<bool> {
    let left = node.left()?;
    let combinator = node.combinator()?;
    let right = node.right()?;

    // Suppressions stay leading on the right selector so verbatim formatting
    // can own them. They are not dangling here, but still require fixed
    // separators for the chain.
    if comments.has_dangling_comments(node.syntax())
        || is_selector_boundary_suppressed(&left, &combinator, &right)
    {
        return Ok(true);
    }

    if let Some(left) = left.as_css_complex_selector()
        && is_flat_chain_layout_required(left, comments)?
    {
        return Ok(true);
    }

    if let Some(right) = right.as_css_complex_selector()
        && is_flat_chain_layout_required(right, comments)?
    {
        return Ok(true);
    }

    Ok(false)
}

/// Formats one side of a selector boundary, including its surrounding gap.
///
/// Comment text still goes through the standard dangling-comment formatter.
/// `final_source_lines` only restores a gap owned by a removed descendant
/// combinator after the final comment.
#[derive(Debug)]
struct FormatSelectorBoundaryComments<'a> {
    comments: &'a [SourceComment<CssLanguage>],
    lines_before: u32,
    final_source_lines: u32,
}

impl<'a> FormatSelectorBoundaryComments<'a> {
    /// Creates a formatter for one comment side of a selector boundary.
    fn new(
        comments: &'a [SourceComment<CssLanguage>],
        lines_before: u32,
        final_source_lines: u32,
    ) -> Self {
        Self {
            comments,
            lines_before,
            final_source_lines,
        }
    }
}

impl Format<CssFormatContext> for FormatSelectorBoundaryComments<'_> {
    fn fmt(&self, f: &mut CssFormatter) -> FormatResult<()> {
        FormatCommentGap::new(self.lines_before).fmt(f)?;

        let mut comments = self.comments.iter().peekable();
        while let Some(comment) = comments.next() {
            write!(f, [format_dangling_comment(comment)])?;

            let next = comments.peek().copied();
            if next.is_some_and(|next| should_nestle_adjacent_doc_comments(comment, next)) {
                continue;
            }

            let source_lines = if next.is_none() {
                self.final_source_lines
            } else {
                0
            };
            let lines_after = comment
                .lines_after()
                .max(source_lines)
                .max(u32::from(comment.kind().is_line()));
            FormatCommentGap::new(lines_after).fmt(f)?;
        }

        Ok(())
    }
}

/// Counts logical line breaks, treating CRLF as one and stopping at two.
///
/// The formatter only distinguishes a space, one line break, and an empty line.
fn count_line_breaks(text: &str) -> u32 {
    let mut bytes = text.bytes().peekable();
    let mut count = 0;

    while let Some(byte) = bytes.next() {
        match byte {
            b'\r' => {
                if bytes.peek() == Some(&b'\n') {
                    bytes.next();
                }
            }
            b'\n' => {}
            _ => continue,
        }

        count += 1;
        if count == 2 {
            break;
        }
    }

    count
}
