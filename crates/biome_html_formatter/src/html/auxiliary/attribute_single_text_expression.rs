use crate::prelude::*;
use biome_formatter::{CstFormatContext, FormatContext, FormatRuleWithOptions, write};
use biome_html_syntax::{
    HtmlAttributeInitializerClause, HtmlAttributeSingleTextExpression,
    HtmlAttributeSingleTextExpressionFields,
};
use biome_rowan::{AstNode, TextRange};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlAttributeSingleTextExpression {
    compact: bool,
}

impl FormatNodeRule<HtmlAttributeSingleTextExpression> for FormatHtmlAttributeSingleTextExpression {
    /// Returns the range of the JavaScript payload of a Svelte attribute value.
    ///
    /// Only the value form (`name={expression}`) is delegated. The other
    /// positions that hold this node - an attribute-position expression such as
    /// `<a {expression}>`, and an expression inside a quoted template value -
    /// are printed from their source text, and the `compact` form removes the
    /// braces instead of keeping the payload.
    fn embedded_node_range(
        &self,
        node: &HtmlAttributeSingleTextExpression,
        f: &mut HtmlFormatter,
    ) -> Option<TextRange> {
        if self.compact
            || !f.context().options().file_source().is_svelte()
            || !f.context().should_delegate_fmt_embedded_nodes()
            || !node
                .syntax()
                .parent()
                .is_some_and(|parent| HtmlAttributeInitializerClause::can_cast(parent.kind()))
        {
            return None;
        }

        let expression = node.expression().ok()?;
        if f.context().comments().is_suppressed(expression.syntax()) {
            return None;
        }

        let token = expression.html_literal_token().ok()?;
        f.state_mut().track_token(&token);
        // An attribute value is parsed as a directive payload, so the service
        // always resolves this range as an inline embed. A block embed would
        // close with a hard line and split the attribute list.
        Some(token.text_range())
    }

    fn fmt_fields(
        &self,
        node: &HtmlAttributeSingleTextExpression,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        let HtmlAttributeSingleTextExpressionFields {
            l_curly_token,
            expression,
            r_curly_token,
        } = node.as_fields();

        if self.compact {
            let l_curly_token = l_curly_token?;
            let r_curly_token = r_curly_token.clone()?;
            let expression = expression.clone()?;
            format_removed(&l_curly_token).fmt(f)?;
            format_removed(&r_curly_token).fmt(f)?;
            expression.format().with_options(self.compact).fmt(f)
        } else {
            write!(
                f,
                [
                    l_curly_token.format(),
                    expression.format(),
                    r_curly_token.format()
                ]
            )
        }
    }
}

impl FormatRuleWithOptions<HtmlAttributeSingleTextExpression>
    for FormatHtmlAttributeSingleTextExpression
{
    type Options = bool;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.compact = options;
        self
    }
}
