use crate::AnalyzerSuppression;
use biome_rowan::TextRange;

/// Parsed suppressions and diagnostics originating from one comment.
#[derive(Clone, Debug)]
pub struct SuppressionComment<'a, D> {
    /// The complete comment range in the tree being analyzed.
    pub range: TextRange,
    /// Parser results in their original order. Diagnostic spans remain
    /// comment-relative, as specified by [`Suppression::parse_comment`].
    pub suppressions: Vec<Result<AnalyzerSuppression<'a>, D>>,
}

/// Parses suppression comments without registering or applying them.
pub trait Suppression {
    type Diagnostic;

    /// Parses a comment at `range` in the tree being analyzed.
    ///
    /// Returns no results for comments without applicable directives. Successful
    /// suppression ranges use tree coordinates; diagnostic spans remain relative
    /// to `text`. All results are preserved in source order, including errors.
    fn parse_comment<'a>(
        &self,
        text: &'a str,
        range: TextRange,
    ) -> Vec<Result<AnalyzerSuppression<'a>, Self::Diagnostic>>;

    /// Returns parsed suppression comments from a supplied snippet occupying
    /// `range`, the trimmed range of a token in the analyzed tree.
    ///
    /// Comment ranges must be in source order, non-overlapping, and contained in
    /// `range`. Results follow the coordinate contract of [`Self::parse_comment`]
    /// and may borrow from this implementation. The default returns no comments.
    fn parse_snippet(&self, _range: TextRange) -> Vec<SuppressionComment<'_, Self::Diagnostic>> {
        Vec::new()
    }
}
