use std::fmt::Debug;

use crate::prelude::*;
use crate::shared::FmtAnyAttributeInitializer;
use crate::utils::srcset::{SrcsetCandidate, descriptor_integer_len, parse_srcset};
use biome_formatter::{CstFormatContext, FormatRuleWithOptions, write};
use biome_html_syntax::{
    AnyHtmlAttributeInitializer, HtmlAttributeInitializerClause,
    HtmlAttributeInitializerClauseFields, HtmlSyntaxToken,
};
use biome_rowan::TokenText;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlAttributeInitializerClause {
    /// The name of the tag this attribute belongs to.
    pub tag_name: Option<TokenText>,

    /// The name of the attribute this initializer clause belongs to.
    pub attribute_name: Option<TokenText>,

    /// Whether it should be formatted in compact mode. In compact mode, all tokens and children
    /// are removed
    pub compact: CompactKind,
}

pub(crate) struct FormatHtmlAttributeInitializerClauseOptions {
    /// The name of the tag this attribute belongs to.
    pub tag_name: Option<TokenText>,

    /// The name of the attribute this initializer clause belongs to.
    pub attribute_name: Option<TokenText>,

    /// Whether it should be formatted in compact mode. In compact mode, all tokens and children
    /// are removed
    pub compact: CompactKind,
}

#[derive(Debug, Clone, Default)]
pub(crate) enum CompactKind {
    /// No special formatting
    #[default]
    None,
    /// Removes everything
    Remove,
    /// Removes the `=` and keeps the `{ expression }`
    Curly,
}

impl CompactKind {
    const fn is_curly(&self) -> bool {
        matches!(self, Self::Curly)
    }
}

impl FormatRuleWithOptions<HtmlAttributeInitializerClause>
    for FormatHtmlAttributeInitializerClause
{
    type Options = FormatHtmlAttributeInitializerClauseOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.tag_name = options.tag_name;
        self.attribute_name = options.attribute_name;
        self.compact = options.compact;
        self
    }
}

impl FormatNodeRule<HtmlAttributeInitializerClause> for FormatHtmlAttributeInitializerClause {
    fn fmt_fields(
        &self,
        node: &HtmlAttributeInitializerClause,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        let HtmlAttributeInitializerClauseFields { eq_token, value } = node.as_fields();

        match self.compact {
            CompactKind::None | CompactKind::Curly => {
                // We currently only have special formatting for when the value is a string.
                let eq_token = eq_token?;
                let fmt_eq_token = format_with(|f| {
                    if self.compact.is_curly()
                        && value.as_ref().is_ok_and(|v| {
                            matches!(
                                v,
                                AnyHtmlAttributeInitializer::HtmlAttributeSingleTextExpression(_)
                            )
                        })
                    {
                        format_removed(&eq_token).fmt(f)
                    } else {
                        write!(f, [eq_token.format()])
                    }
                });
                if let Some(html_string) = value.as_ref()?.as_html_string()
                    && !f.context().comments().is_suppressed(html_string.syntax())
                {
                    match (self.tag_name.as_deref(), self.attribute_name.as_deref()) {
                        // Prettier 3.7 handles allow attribute on iframes specially by splitting the
                        // value on semicolons and formatting it like a list, breaking if its too long, or leaving it on one line if it fits in the line width.
                        // It also trims whitespace around each item, and removes empty items.
                        //
                        // Before:
                        // ```html
                        // <iframe allow="    camera; ;    ;  accelerometer;"></iframe>
                        // ```
                        //
                        // After:
                        // ```html
                        // <iframe allow="camera; accelerometer"></iframe>
                        // ```
                        (Some("iframe"), Some("allow")) => {
                            let content = html_string.inner_string_text()?;
                            let value_token = html_string.value_token()?;

                            struct JoinWithSemicolon;
                            impl Format<HtmlFormatContext> for JoinWithSemicolon {
                                fn fmt(&self, f: &mut HtmlFormatter) -> FormatResult<()> {
                                    write!(f, [token(";"), soft_line_break_or_space()])
                                }
                            }

                            write!(
                                f,
                                [
                                    fmt_eq_token,
                                    format_removed(&value_token),
                                    token("\""),
                                    group(&soft_block_indent(&format_with(|f| {
                                        let items = content
                                            .split(';')
                                            .map(TokenText::trim_token)
                                            .filter(|s| !s.is_empty())
                                            .collect::<Vec<_>>();
                                        f.join_with(JoinWithSemicolon)
                                            .entries(items.into_iter().map(|item| {
                                                located_token_text(
                                                    &value_token,
                                                    item.source_range(value_token.text_range()),
                                                )
                                            }))
                                            .finish()?;
                                        write!(f, [if_group_breaks(&token(";"))])?;
                                        Ok(())
                                    }))),
                                    token("\"")
                                ]
                            )
                        }
                        // Prettier lays a `srcset` out as the list of candidates
                        // it is, one per line when they do not all fit, with the
                        // descriptors aligned on their decimal point so the
                        // sizes can be read down the column.
                        //
                        // Before:
                        // ```html
                        // <img srcset="a@0.5.png  400w, a.png      805w">
                        // ```
                        //
                        // After:
                        // ```html
                        // <img srcset="a@0.5.png 400w, a.png 805w" />
                        // ```
                        (Some("img" | "source"), Some("srcset")) => {
                            let content = html_string.inner_string_text()?;
                            let value_token = html_string.value_token()?;

                            let Some(candidates) = parse_srcset(content.text()) else {
                                // The value holds nothing to lay out, or
                                // something this formatter should not second
                                // guess. Leave it exactly as written.
                                return write!(f, [fmt_eq_token, value.format()]);
                            };

                            write!(
                                f,
                                [
                                    fmt_eq_token,
                                    format_removed(&value_token),
                                    token("\""),
                                    group(&soft_block_indent(&FormatSrcsetCandidates {
                                        candidates: &candidates,
                                        content: &content,
                                        value_token: &value_token,
                                    })),
                                    token("\"")
                                ]
                            )
                        }
                        _ => {
                            write!(f, [fmt_eq_token, value.format()])
                        }
                    }
                } else {
                    write!(f, [fmt_eq_token, value.format()])
                }
            }
            CompactKind::Remove => {
                let eq_token = eq_token.clone()?;
                let value = value.clone()?;
                let fmt = FmtAnyAttributeInitializer {
                    node: value,
                    compact: true,
                };
                write!(f, [format_removed(&eq_token), &fmt,])?;
                Ok(())
            }
        }
    }
}

/// Prints the candidates of a `srcset`, separated by `, ` while they fit on
/// one line and one per line once they do not.
///
/// When broken, each descriptor is pushed right so that every decimal point
/// lands in the same column, which is what turns a wall of URLs into a table
/// of sizes. The padding only applies to the broken form; laid out flat, a
/// single space separates a URL from its descriptor.
struct FormatSrcsetCandidates<'a> {
    candidates: &'a [SrcsetCandidate],
    content: &'a TokenText,
    value_token: &'a HtmlSyntaxToken,
}

impl Format<HtmlFormatContext> for FormatSrcsetCandidates<'_> {
    fn fmt(&self, f: &mut HtmlFormatter) -> FormatResult<()> {
        let text = self.content.text();
        let widest_url = self
            .candidates
            .iter()
            .map(|candidate| text[candidate.url].len())
            .max()
            .unwrap_or(0);
        let widest_descriptor_integer = self
            .candidates
            .iter()
            .filter_map(|candidate| candidate.descriptor)
            .map(|descriptor| descriptor_integer_len(&text[descriptor]))
            .max()
            .unwrap_or(0);

        let mut candidates = self.candidates.iter().peekable();
        while let Some(candidate) = candidates.next() {
            let url = self.content.clone().slice(candidate.url);
            write!(
                f,
                [located_token_text(
                    self.value_token,
                    url.source_range(self.value_token.text_range())
                )]
            )?;

            if let Some(descriptor) = candidate.descriptor {
                // One space of separation, plus whatever it takes to line the
                // decimal points up with the widest candidate.
                let padding =
                    widest_url - text[candidate.url].len() + 1 + widest_descriptor_integer
                        - descriptor_integer_len(&text[descriptor]);
                let descriptor = self.content.clone().slice(descriptor);
                write!(
                    f,
                    [
                        if_group_breaks(&token(spaces(padding))),
                        if_group_fits_on_line(&token(" ")),
                        located_token_text(
                            self.value_token,
                            descriptor.source_range(self.value_token.text_range())
                        )
                    ]
                )?;
            }

            if candidates.peek().is_some() {
                write!(f, [token(","), soft_line_break_or_space()])?;
            }
        }

        Ok(())
    }
}

/// A run of `count` spaces, for aligning descriptors.
///
/// Slicing a literal keeps the result `'static`, which is what the token
/// builder takes. A `srcset` whose alignment would run past the end of the
/// literal is already far wider than any line limit, so the padding is simply
/// capped there.
fn spaces(count: usize) -> &'static str {
    const SPACES: &str = "                                                                ";
    &SPACES[..count.min(SPACES.len())]
}
