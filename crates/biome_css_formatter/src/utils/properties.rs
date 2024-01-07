use crate::prelude::*;
use biome_formatter::{write, Arguments};

/// Format all of the fields of a `PropertyValue` node in an arbitrary order,
/// given by `slot_map`.
///
/// Because the CSS grammar allows rules to specify fields that can appear
/// in any order, there isn't always a linear mapping between the _declared_
/// order (how they appear in the grammar) and the _concrete_ order (how they
/// appear in the source text) of the fields. The parser supports this by
/// building a `slot_map` to map the declared order to the concrete order.
///
/// When formatting, by default we want to preserve the ordering of fields as
/// they were written in the source, but just using the `AstNode` alone will
/// naturally re-write the value in the _declared_ order. To preserve the
/// _concrete_ order, we can invert the `slot_map` and sort it to re-determine
/// the ordering of fields and then iterate that list to format each field
/// individually.
///
/// ## Fields
///
/// The caller provides a list of _pre-formatted_ fields, using the
/// [`biome_formatter::format_args!`] macro. This way, it can either pass
/// through a field as-is with default formatting, or it can apply any other
/// formatting it once for that field:
///
/// ```rust,ignore
/// let formatted = format!(CssFormatContext::default(), [
///     FormatPropertyValueFields::new(&format_args![
///         text("a"),
///         text("b"),
///         group(&block_indent(&format_args![text("c"), hard_line_break(), text("d")]))
///     ])
///     .with_slot_map([1, 2, 0])
/// ])?;
///
/// assert_eq!("b
/// \tc
/// \td
/// a", formatted.print()?.as_code());
/// ```
///
/// ## Concrete Ordering
///
/// By default, using this struct will format the fields of the node in order.
/// This is sufficient for nodes that don't have any dynamically-ordered
/// fields, but for dynamic nodes that want to preserve the order of fields as
/// they were given in the input, or for any node that wants to change the
/// ordering of the fields, the caller will need to provide a `slot_map` that
/// this struct can use to re-order the fields.
///
/// To preserve the field order as it was written in the original source, use
/// [biome_rowan::AstNodeSlotMap::concrete_order_slot_map], which will ensure
/// the ordering matches what was given. This should be the default for most
/// if not all dynamic nodes.
///
/// ```rust,ignore
/// .with_slot_map(node.concrete_order_slot_map())
/// ```
///
/// Any other method of building a slot map is also valid, but should generally
/// be avoided, as ensuring consistency across formats is difficult without a
/// strong heuristic.
///
/// ## Grouping Fields (Future)
///
/// In some cases, a property may want to group certain fields together in
/// order to apply special formatting. As an example, consider a grammar like:
///
/// ```ebnf
///     font =
///         (style: CssFontStyle ||
///         variant: CssFontVariant ||
///         weight: CssFontWeight)?
///         size: CssNumber ( '/' line_height: CssLineHeight)?
/// ```
///
/// Here, the `style`, `variant`, and `weight` fields can appear conditionally
/// and in any order, but if `line_height` is present, it (and the slash token)
/// must appear immediately adjacent to the `size` field. While it would be
/// valid to just have the fields fill and wrap over lines as needed, the
/// formatter might want to preserve the adjacency and ensure that `size` and
/// `line_height` always get written on the same line.
///
/// To do this, the value formatter can write both fields in a single group,
/// and then use an `empty_field_slot` value in the slots where the other
/// fields have been taken from:
///
/// ```rust,ignore
/// FormatPropertyValueFields::new(&format_args![
///         style.format(),
///         variant.format(),
///         weight.format(),
///         group(&format_args![
///             size.format(), slash_token.format(), line_height.format()
///         ]),
///         empty_field_slot(),
///         empty_field_slot()
///     ])
///     .with_slot_map(node.concrete_order_slot_map())
/// ```
///
/// The `empty_field_slot()` values will tell this struct to skip formatting
/// for that field, with the assumption that another field includes its value.
pub struct FormatPropertyValueFields<'fmt, const N: usize> {
    slot_map: Option<[u8; N]>,
    fields: &'fmt Arguments<'fmt, CssFormatContext>,
}

impl<'fmt, const N: usize> FormatPropertyValueFields<'fmt, N> {
    pub fn new(fields: &'fmt Arguments<'fmt, CssFormatContext>) -> Self {
        Self {
            slot_map: None,
            fields,
        }
    }

    pub fn with_slot_map(mut self, slot_map: [u8; N]) -> Self {
        debug_assert!(
            self.fields.items().len() == N,
            "slot_map must specify the same number of fields as this struct contains"
        );
        self.slot_map = Some(slot_map);
        self
    }
}

impl<'fmt, const N: usize> Format<CssFormatContext> for FormatPropertyValueFields<'fmt, N> {
    fn fmt(&self, f: &mut CssFormatter) -> FormatResult<()> {
        let values = format_with(|f: &mut Formatter<'_, CssFormatContext>| {
            let mut filler = f.fill();

            // First, determine the ordering of fields to use. If no slot_map is
            // provided along with the fields, then they can just be used in the
            // same order, but if a `slot_map` is present, then the fields are
            // re-ordered to match the concrete ordering from the source syntax.
            //
            // The fields are wrapped with `Option` for two reasons: for nodes
            // with slot maps, it simplifies how the re-ordered slice is built, and
            // it also allows empty/missing fields to be removed in the next step.
            match self.slot_map {
                None => {
                    for field in self.fields.items() {
                        filler.entry(&soft_line_break_or_space(), field);
                    }
                }
                Some(slot_map) => {
                    for slot in slot_map {
                        // This condition ensures that missing values are _not_ included in the
                        // fill. The generated `slot_map` for an AstNode guarantees that all
                        // present fields have a tangible value here, while all absent fields
                        // have this sentinel value ([biome_css_syntax::SLOT_MAP_EMPTY_VALUE]).
                        //
                        // This check is important to ensure that we don't add empty values to
                        // the fill, since that would add double separators when we don't want
                        // them.
                        if slot == u8::MAX {
                            continue;
                        }

                        let field = &self.fields.items()[slot as usize];
                        filler.entry(&soft_line_break_or_space(), field);
                    }
                }
            };

            filler.finish()
        });

        write!(f, [group(&indent(&values))])
    }
}
