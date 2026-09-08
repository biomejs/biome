use crate::prelude::*;
use crate::shared::FmtAnyAttributeInitializer;
use crate::utils::srcset::{FormatSrcsetCandidates, parse_srcset};
use biome_formatter::{CstFormatContext, FormatRuleWithOptions, write};
use biome_html_syntax::{
    AnyHtmlAttributeInitializer, HtmlAttributeInitializerClause,
    HtmlAttributeInitializerClauseFields,
};
use biome_rowan::TokenText;
use std::fmt::Debug;

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
