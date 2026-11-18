//! Support for formatting syntax trees built with `biome_rowan`.
//!
//! Every Biome formatter uses [RowanSource], so the document of one language
//! can be placed inside the document of another, such as CSS inside HTML. The
//! type aliases in this module are the formatter types for [RowanSource], and
//! [prelude] exports them instead of the generic types.

use crate::TransformSourceMap;
use crate::source::Source;
use biome_rowan::{TextLen, TextRange, TextSize, TokenText};

/// Source code parsed into a `biome_rowan` syntax tree.
///
/// Positions are byte offsets in the source text. Embedded code is identified
/// by its range in the source text.
#[derive(Copy, Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct RowanSource;

impl Source for RowanSource {
    type Position = TextSize;
    type Span = TextRange;
    type Text = TokenText;
    type EmbeddedKey = TextRange;
    type SourceMap = TransformSourceMap;

    fn advance(position: Self::Position, text: &str) -> Self::Position {
        position + text.text_len()
    }

    fn span(start: Self::Position, end: Self::Position) -> Self::Span {
        TextRange::new(start, end)
    }
}

impl crate::Printed<RowanSource> {
    /// Returns each piece of the printed code that the formatter copied
    /// verbatim from the source code, together with its range.
    pub fn verbatim(&self) -> impl Iterator<Item = (TextRange, &str)> {
        self.verbatim_ranges()
            .iter()
            .map(|range| (*range, &self.as_code()[*range]))
    }
}

pub type BestFittingVariants = crate::format_element::BestFittingVariants<RowanSource>;
pub type Document = crate::format_element::document::Document<RowanSource>;
pub type FormatElement = crate::FormatElement<RowanSource>;
pub type FormatEmbedded = crate::builders::FormatEmbedded<RowanSource>;
pub type Interned = crate::format_element::Interned<RowanSource>;
pub type LineMode = crate::format_element::LineMode<TextSize>;
pub type Printed = crate::Printed<RowanSource>;
pub type SourceMarker = crate::SourceMarker<TextSize>;
pub type Tag = crate::format_element::tag::Tag<RowanSource>;

pub mod prelude {
    pub use crate::prelude::*;

    // These imports shadow the generic types of the glob import above.
    pub use super::{
        BestFittingVariants, Document, FormatElement, FormatEmbedded, Interned, LineMode,
        RowanSource, Tag,
    };
}
