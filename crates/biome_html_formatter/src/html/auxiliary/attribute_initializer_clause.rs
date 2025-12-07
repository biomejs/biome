use std::fmt::Debug;

use crate::prelude::*;
use biome_formatter::{CstFormatContext, FormatRuleWithOptions, write};
use biome_html_syntax::{HtmlAttributeInitializerClause, HtmlAttributeInitializerClauseFields};
use biome_rowan::TokenText;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlAttributeInitializerClause {
    /// The name of the tag this attribute belongs to.
    pub tag_name: Option<TokenText>,

    /// The name of the attribute this initializer clause belongs to.
    pub attribute_name: Option<TokenText>,
}

pub(crate) struct FormatHtmlAttributeInitializerClauseOptions {
    /// The name of the tag this attribute belongs to.
    pub tag_name: Option<TokenText>,

    /// The name of the attribute this initializer clause belongs to.
    pub attribute_name: Option<TokenText>,
}

impl FormatRuleWithOptions<HtmlAttributeInitializerClause>
    for FormatHtmlAttributeInitializerClause
{
    type Options = FormatHtmlAttributeInitializerClauseOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.tag_name = options.tag_name;
        self.attribute_name = options.attribute_name;
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

        // We currently only have special formatting for when the value is a string.
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
                            eq_token.format(),
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
                _ => {
                    write!(f, [eq_token.format(), value.format()])
                }
            }
        } else {
            write!(f, [eq_token.format(), value.format()])
        }
    }
}
