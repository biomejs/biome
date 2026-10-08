use crate::HtmlInlineEmbeddedContent;
use crate::prelude::*;
use biome_formatter::FormatElement;
use biome_formatter::format_element::Interned;
use biome_formatter::prelude::Document;
use biome_formatter::write;
use biome_formatter::{CstFormatContext, FormatRuleWithOptions, normalize_newlines};
use biome_html_syntax::{HtmlAttribute, HtmlTextExpression};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlTextExpression {
    compact: bool,
}
impl FormatNodeRule<HtmlTextExpression> for FormatHtmlTextExpression {
    fn fmt_fields(&self, node: &HtmlTextExpression, f: &mut HtmlFormatter) -> FormatResult<()> {
        if f.context().comments().is_suppressed(node.syntax()) {
            return write!(f, [format_suppressed_node(node.syntax())]);
        }
        let token = node.html_literal_token()?;

        if self.compact {
            return format_removed(&token).fmt(f);
        }

        if let Some(embedded) = f
            .context()
            .inline_embedded_expression(token.text_range())
            .cloned()
        {
            if let Some(identifier) = &embedded.shorthand_identifier
                && node
                    .syntax()
                    .ancestors()
                    .find_map(HtmlAttribute::cast)
                    .and_then(|attribute| attribute.name().ok())
                    .and_then(|name| name.token_text_trimmed())
                    .is_some_and(|name| name.text() == identifier.text())
            {
                return format_replaced(
                    &token,
                    &text(identifier.text(), Some(token.text_range().start())),
                )
                .fmt(f);
            }
            return match embedded.content {
                HtmlInlineEmbeddedContent::Formatted(document) => format_replaced(
                    &token,
                    &FormatInlineEmbeddedExpression {
                        document: &document,
                    },
                )
                .fmt(f),
                HtmlInlineEmbeddedContent::Verbatim => {
                    format_verbatim_skipped(node.syntax()).fmt(f)
                }
            };
        }

        let token_text = token.text();
        let trimmed_text = token_text.trim_start().trim_end();
        let normalized_text = normalize_newlines(trimmed_text, ['\r']);

        write!(
            f,
            [format_replaced(
                &token,
                &text(&normalized_text, Some(token.text_range().start()))
            )]
        )
    }
}

impl FormatRuleWithOptions<HtmlTextExpression> for FormatHtmlTextExpression {
    type Options = bool;
    fn with_options(mut self, options: Self::Options) -> Self {
        self.compact = options;
        self
    }
}

struct FormatInlineEmbeddedExpression<'a> {
    document: &'a Document,
}

impl Format<HtmlFormatContext> for FormatInlineEmbeddedExpression<'_> {
    fn fmt(&self, f: &mut HtmlFormatter) -> FormatResult<()> {
        f.write_element(FormatElement::Interned(Interned::new(
            self.document.clone().into_elements(),
        )))?;
        line_suffix_boundary().fmt(f)
    }
}
