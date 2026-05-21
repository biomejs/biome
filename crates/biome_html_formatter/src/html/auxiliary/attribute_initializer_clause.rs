use std::fmt::Debug;

use crate::prelude::*;
use crate::shared::FmtAnyAttributeInitializer;
use biome_formatter::{CstFormatContext, FormatRuleWithOptions, write};
use biome_html_syntax::{
    AnyHtmlAttributeInitializer, HtmlAttributeInitializerClause,
    HtmlAttributeInitializerClauseFields,
};
use biome_rowan::{TextLen, TextRange, TextSize, TokenText};

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
                        // Pretter reformats the contents of the `class` attribute by splitting it on whitespace rejoining it with a single space, and removing extra whitespace and newlines.
                        // https://github.com/prettier/prettier/blob/0fe055c25d7e8b74bab35c188ee9bb3de4c23fa5/src/language-html/embed/class-names.js
                        // Before:
                        // ```html
                        // <div class="   foo   bar   baz   "></div>
                        // ```
                        // After:
                        // ```html
                        // <div class="foo bar baz"></div>
                        // ```
                        (_, Some("class")) => {
                            let content = html_string.inner_string_text()?;
                            let value_token = html_string.value_token()?;
                            struct JoinWithSpace;
                            impl Format<HtmlFormatContext> for JoinWithSpace {
                                fn fmt(&self, f: &mut HtmlFormatter) -> FormatResult<()> {
                                    write!(f, [space()])
                                }
                            }
                            write!(
                                f,
                                [
                                    fmt_eq_token,
                                    format_removed(&value_token),
                                    token("\""),
                                    format_with(|f| {
                                        f.join_with(JoinWithSpace)
                                            .entries(
                                                content
                                                    .split_whitespace()
                                                    .filter(|s| !s.is_empty())
                                                    .map(|item| {
                                                        located_token_text(
                                                            &value_token,
                                                            item.source_range(
                                                                value_token.text_range(),
                                                            ),
                                                        )
                                                    }),
                                            )
                                            .finish()?;
                                        Ok(())
                                    }),
                                    token("\"")
                                ]
                            )
                        }

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
                        (_, Some("style")) => {
                            let content = html_string.inner_string_text()?;
                            let value_token = html_string.value_token()?;
                            let declarations = content
                                .split(';')
                                .map(TokenText::trim_token)
                                .filter(|declaration| !declaration.is_empty())
                                .collect::<Vec<_>>();

                            if declarations.iter().any(|declaration| {
                                !declaration.text().contains(':')
                                    || declaration.text().contains(['{', '}'])
                            }) {
                                return write!(f, [fmt_eq_token, value.format()]);
                            }

                            struct JoinStyleDeclarations {
                                multiline: bool,
                            }

                            impl Format<HtmlFormatContext> for JoinStyleDeclarations {
                                fn fmt(&self, f: &mut HtmlFormatter) -> FormatResult<()> {
                                    if self.multiline {
                                        write!(f, [token(";"), hard_line_break()])
                                    } else {
                                        write!(f, [token(";"), space()])
                                    }
                                }
                            }

                            let format_declaration = |declaration: TokenText| {
                                let value_token = value_token.clone();
                                format_with(move |f| {
                                    let Some((property, _)) = declaration.text().split_once(':')
                                    else {
                                        return Ok(());
                                    };
                                    let property = declaration
                                        .clone()
                                        .slice(TextRange::new(
                                            TextSize::from(0),
                                            TextSize::try_from(property.len()).unwrap_or_default(),
                                        ))
                                        .trim_token();
                                    let value_start = property.text_len() + TextSize::from(1);
                                    let value = declaration
                                        .clone()
                                        .slice(TextRange::new(value_start, declaration.text_len()))
                                        .trim_token();
                                    write!(
                                        f,
                                        [
                                            located_token_text(
                                                &value_token,
                                                property.source_range(value_token.text_range())
                                            ),
                                            token(":"),
                                            space(),
                                            located_token_text(
                                                &value_token,
                                                value.source_range(value_token.text_range())
                                            )
                                        ]
                                    )
                                })
                            };

                            let multiline = declarations.len() > 2;

                            write!(
                                f,
                                [
                                    fmt_eq_token,
                                    format_removed(&value_token),
                                    token("\""),
                                    group(&format_with(|f| {
                                        if multiline {
                                            write!(
                                                f,
                                                [block_indent(&format_with(|f| {
                                                    f.join_with(JoinStyleDeclarations {
                                                        multiline,
                                                    })
                                                    .entries(
                                                        declarations
                                                            .iter()
                                                            .cloned()
                                                            .map(format_declaration),
                                                    )
                                                    .finish()?;
                                                    write!(f, [token(";")])
                                                }))]
                                            )
                                        } else {
                                            f.join_with(JoinStyleDeclarations { multiline })
                                                .entries(
                                                    declarations
                                                        .iter()
                                                        .cloned()
                                                        .map(format_declaration),
                                                )
                                                .finish()
                                        }
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
