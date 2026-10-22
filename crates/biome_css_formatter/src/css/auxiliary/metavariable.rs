use crate::prelude::*;
use crate::verbatim::format_css_verbatim_node;
use biome_css_syntax::CssMetavariable;
use biome_formatter::format_element::tag::Tag;
use biome_formatter::{VecBuffer, write};
use biome_rowan::{AstNode, TextRange};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssMetavariable;
impl FormatNodeRule<CssMetavariable> for FormatCssMetavariable {
    fn fmt_fields(&self, node: &CssMetavariable, f: &mut CssFormatter) -> FormatResult<()> {
        let token = node.value_token()?;
        let token_range = token.text_trimmed_range();
        let mut ranges: Vec<_> = f.context().metavariable_ranges_in(token_range).collect();
        if ranges.is_empty() {
            return format_css_verbatim_node(node.syntax()).fmt(f);
        }
        ranges.sort_unstable_by_key(|range| range.start());

        // Each metavariable is an embedded element, so that the host formatter
        // can replace it, while the text glued to it is kept as is:
        //
        // ```css
        // width: ${width}px;
        // ```
        let content = format_with(|f| {
            let source = token.text_trimmed();
            let mut position = token_range.start();
            for range in &ranges {
                if position < range.start() {
                    let segment = TextRange::new(position, range.start()) - token_range.start();
                    write!(f, [text(&source[segment], Some(position))])?;
                }

                let embedded = {
                    let mut buffer = VecBuffer::new(f.state_mut());
                    let metavariable = *range - token_range.start();
                    write!(buffer, [text(&source[metavariable], Some(range.start()))])?;
                    buffer.into_vec()
                };
                f.write_elements(vec![
                    FormatElement::Tag(Tag::StartEmbedded(*range)),
                    FormatElement::Interned(Interned::new(embedded)),
                    FormatElement::Tag(Tag::EndEmbedded),
                ])?;
                position = range.end();
            }

            if position < token_range.end() {
                let segment = TextRange::new(position, token_range.end()) - token_range.start();
                write!(f, [text(&source[segment], Some(position))])?;
            }

            Ok(())
        });

        write!(f, [format_replaced(&token, &content)])
    }
}
