use crate::prelude::*;
use biome_formatter::{FormatRuleWithOptions, write};
use biome_html_syntax::{
    AnyHtmlAttributeInitializer, AnySvelteTemplateElement, HtmlAttributeSingleTextExpression,
    SvelteTemplateAttributeValue, SvelteTemplateAttributeValueFields,
};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteTemplateAttributeValue {
    compact: bool,
}
impl FormatNodeRule<SvelteTemplateAttributeValue> for FormatSvelteTemplateAttributeValue {
    fn fmt_fields(
        &self,
        node: &SvelteTemplateAttributeValue,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        let SvelteTemplateAttributeValueFields {
            l_quote,
            elements,
            r_quote,
        } = node.as_fields();

        // In compact mode the whole value is dropped, because the name alone
        // already says what it said.
        if self.compact {
            let l_quote = l_quote?;
            let r_quote = r_quote?;
            format_removed(&l_quote).fmt(f)?;
            format_removed(&r_quote).fmt(f)?;
            return match lone_expression(node) {
                Some(expression) => expression.format().with_options(true).fmt(f),
                None => elements.format().fmt(f),
            };
        }

        // A value that is nothing but one expression does not need its quotes:
        // `class="{value}"` says exactly what `class={value}` says.
        if let Some(expression) = lone_expression(node) {
            return write!(
                f,
                [
                    format_removed(&l_quote?),
                    expression.format(),
                    format_removed(&r_quote?),
                ]
            );
        }

        // Prettier writes these with double quotes, whatever the author used.
        // A value already holding a double quote keeps the quotes it came with,
        // since there is no escape for one here.
        let l_quote = l_quote?;
        let r_quote = r_quote?;
        let holds_double_quote = node
            .elements()
            .syntax()
            .text_trimmed()
            .to_string()
            .contains('"');
        if l_quote.text_trimmed() == "'" && !holds_double_quote {
            return write!(
                f,
                [
                    format_replaced(&l_quote, &token("\"")),
                    elements.format(),
                    format_replaced(&r_quote, &token("\"")),
                ]
            );
        }

        write!(f, [l_quote.format(), elements.format(), r_quote.format()])
    }
}

/// The single expression an attribute's value consists of, whether or not the
/// author wrote quotes around it.
///
/// Svelte reads `x="{x}"` and `x={x}` as the same thing, so everything that
/// looks at the value of an attribute has to see through the quotes.
pub(crate) fn lone_initializer_expression(
    initializer: &AnyHtmlAttributeInitializer,
) -> Option<HtmlAttributeSingleTextExpression> {
    match initializer {
        AnyHtmlAttributeInitializer::HtmlAttributeSingleTextExpression(expression) => {
            Some(expression.clone())
        }
        AnyHtmlAttributeInitializer::SvelteTemplateAttributeValue(value) => lone_expression(value),
        _ => None,
    }
}

/// The single expression an attribute value consists of, if that is all it is.
///
/// `"{value}"` has one, `"prefix{value}"` does not, since dropping the quotes
/// there would change what the attribute says.
pub(crate) fn lone_expression(
    node: &SvelteTemplateAttributeValue,
) -> Option<HtmlAttributeSingleTextExpression> {
    let mut elements = node.elements().iter();
    let first = elements.next()?;
    if elements.next().is_some() {
        return None;
    }

    match first {
        AnySvelteTemplateElement::HtmlAttributeSingleTextExpression(expression) => Some(expression),
        AnySvelteTemplateElement::SvelteTemplateChunkElement(_) => None,
    }
}

impl FormatRuleWithOptions<SvelteTemplateAttributeValue> for FormatSvelteTemplateAttributeValue {
    type Options = bool;
    fn with_options(mut self, options: Self::Options) -> Self {
        self.compact = options;
        self
    }
}
