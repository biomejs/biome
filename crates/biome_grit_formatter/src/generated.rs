//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

#![expect(clippy::default_constructed_unit_structs)]
use crate::{
    AsFormat, FormatBogusNodeRule, FormatNodeRule, GritFormatContext, GritFormatter, IntoFormat,
};
use biome_formatter::{FormatOwnedWithRule, FormatRefWithRule, FormatResult, FormatRule};
impl FormatRule<biome_grit_syntax::GritAddOperation>
    for crate::grit::patterns::add_operation::FormatGritAddOperation
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritAddOperation,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritAddOperation>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritAddOperation {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritAddOperation,
        crate::grit::patterns::add_operation::FormatGritAddOperation,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::patterns::add_operation::FormatGritAddOperation::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritAddOperation {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritAddOperation,
        crate::grit::patterns::add_operation::FormatGritAddOperation,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::patterns::add_operation::FormatGritAddOperation::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritAnnotation>
    for crate::grit::auxiliary::annotation::FormatGritAnnotation
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritAnnotation,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritAnnotation>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritAnnotation {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritAnnotation,
        crate::grit::auxiliary::annotation::FormatGritAnnotation,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::auxiliary::annotation::FormatGritAnnotation::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritAnnotation {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritAnnotation,
        crate::grit::auxiliary::annotation::FormatGritAnnotation,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::auxiliary::annotation::FormatGritAnnotation::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritAssignmentAsPattern>
    for crate::grit::patterns::assignment_as_pattern::FormatGritAssignmentAsPattern
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritAssignmentAsPattern,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritAssignmentAsPattern>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritAssignmentAsPattern {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritAssignmentAsPattern,
        crate::grit::patterns::assignment_as_pattern::FormatGritAssignmentAsPattern,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::patterns::assignment_as_pattern::FormatGritAssignmentAsPattern::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritAssignmentAsPattern {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritAssignmentAsPattern,
        crate::grit::patterns::assignment_as_pattern::FormatGritAssignmentAsPattern,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::patterns::assignment_as_pattern::FormatGritAssignmentAsPattern::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritBacktickSnippetLiteral>
    for crate::grit::value::backtick_snippet_literal::FormatGritBacktickSnippetLiteral
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritBacktickSnippetLiteral,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritBacktickSnippetLiteral>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritBacktickSnippetLiteral {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritBacktickSnippetLiteral,
        crate::grit::value::backtick_snippet_literal::FormatGritBacktickSnippetLiteral,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::value::backtick_snippet_literal::FormatGritBacktickSnippetLiteral::default(
            ),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritBacktickSnippetLiteral {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritBacktickSnippetLiteral,
        crate::grit::value::backtick_snippet_literal::FormatGritBacktickSnippetLiteral,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::value::backtick_snippet_literal::FormatGritBacktickSnippetLiteral::default(
            ),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritBooleanLiteral>
    for crate::grit::value::boolean_literal::FormatGritBooleanLiteral
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritBooleanLiteral,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritBooleanLiteral>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritBooleanLiteral {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritBooleanLiteral,
        crate::grit::value::boolean_literal::FormatGritBooleanLiteral,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::value::boolean_literal::FormatGritBooleanLiteral::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritBooleanLiteral {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritBooleanLiteral,
        crate::grit::value::boolean_literal::FormatGritBooleanLiteral,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::value::boolean_literal::FormatGritBooleanLiteral::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritBracketedPattern>
    for crate::grit::patterns::bracketed_pattern::FormatGritBracketedPattern
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritBracketedPattern,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritBracketedPattern>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritBracketedPattern {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritBracketedPattern,
        crate::grit::patterns::bracketed_pattern::FormatGritBracketedPattern,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::patterns::bracketed_pattern::FormatGritBracketedPattern::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritBracketedPattern {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritBracketedPattern,
        crate::grit::patterns::bracketed_pattern::FormatGritBracketedPattern,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::patterns::bracketed_pattern::FormatGritBracketedPattern::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritBracketedPredicate>
    for crate::grit::predicates::bracketed_predicate::FormatGritBracketedPredicate
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritBracketedPredicate,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritBracketedPredicate>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritBracketedPredicate {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritBracketedPredicate,
        crate::grit::predicates::bracketed_predicate::FormatGritBracketedPredicate,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::predicates::bracketed_predicate::FormatGritBracketedPredicate::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritBracketedPredicate {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritBracketedPredicate,
        crate::grit::predicates::bracketed_predicate::FormatGritBracketedPredicate,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::predicates::bracketed_predicate::FormatGritBracketedPredicate::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritBubble>
    for crate::grit::auxiliary::bubble::FormatGritBubble
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_grit_syntax::GritBubble, f: &mut GritFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritBubble>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritBubble {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritBubble,
        crate::grit::auxiliary::bubble::FormatGritBubble,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::auxiliary::bubble::FormatGritBubble::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritBubble {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritBubble,
        crate::grit::auxiliary::bubble::FormatGritBubble,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::auxiliary::bubble::FormatGritBubble::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritBubbleScope>
    for crate::grit::auxiliary::bubble_scope::FormatGritBubbleScope
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritBubbleScope,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritBubbleScope>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritBubbleScope {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritBubbleScope,
        crate::grit::auxiliary::bubble_scope::FormatGritBubbleScope,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::auxiliary::bubble_scope::FormatGritBubbleScope::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritBubbleScope {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritBubbleScope,
        crate::grit::auxiliary::bubble_scope::FormatGritBubbleScope,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::auxiliary::bubble_scope::FormatGritBubbleScope::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritCodeSnippet>
    for crate::grit::value::code_snippet::FormatGritCodeSnippet
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritCodeSnippet,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritCodeSnippet>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritCodeSnippet {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritCodeSnippet,
        crate::grit::value::code_snippet::FormatGritCodeSnippet,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::value::code_snippet::FormatGritCodeSnippet::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritCodeSnippet {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritCodeSnippet,
        crate::grit::value::code_snippet::FormatGritCodeSnippet,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::value::code_snippet::FormatGritCodeSnippet::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritCurlyPattern>
    for crate::grit::patterns::curly_pattern::FormatGritCurlyPattern
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritCurlyPattern,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritCurlyPattern>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritCurlyPattern {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritCurlyPattern,
        crate::grit::patterns::curly_pattern::FormatGritCurlyPattern,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::patterns::curly_pattern::FormatGritCurlyPattern::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritCurlyPattern {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritCurlyPattern,
        crate::grit::patterns::curly_pattern::FormatGritCurlyPattern,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::patterns::curly_pattern::FormatGritCurlyPattern::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritDivOperation>
    for crate::grit::patterns::div_operation::FormatGritDivOperation
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritDivOperation,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritDivOperation>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritDivOperation {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritDivOperation,
        crate::grit::patterns::div_operation::FormatGritDivOperation,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::patterns::div_operation::FormatGritDivOperation::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritDivOperation {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritDivOperation,
        crate::grit::patterns::div_operation::FormatGritDivOperation,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::patterns::div_operation::FormatGritDivOperation::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritDot> for crate::grit::auxiliary::dot::FormatGritDot {
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_grit_syntax::GritDot, f: &mut GritFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritDot>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritDot {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritDot,
        crate::grit::auxiliary::dot::FormatGritDot,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, crate::grit::auxiliary::dot::FormatGritDot::default())
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritDot {
    type Format =
        FormatOwnedWithRule<biome_grit_syntax::GritDot, crate::grit::auxiliary::dot::FormatGritDot>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, crate::grit::auxiliary::dot::FormatGritDot::default())
    }
}
impl FormatRule<biome_grit_syntax::GritDotdotdot>
    for crate::grit::auxiliary::dotdotdot::FormatGritDotdotdot
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritDotdotdot,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritDotdotdot>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritDotdotdot {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritDotdotdot,
        crate::grit::auxiliary::dotdotdot::FormatGritDotdotdot,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::auxiliary::dotdotdot::FormatGritDotdotdot::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritDotdotdot {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritDotdotdot,
        crate::grit::auxiliary::dotdotdot::FormatGritDotdotdot,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::auxiliary::dotdotdot::FormatGritDotdotdot::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritDoubleLiteral>
    for crate::grit::value::double_literal::FormatGritDoubleLiteral
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritDoubleLiteral,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritDoubleLiteral>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritDoubleLiteral {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritDoubleLiteral,
        crate::grit::value::double_literal::FormatGritDoubleLiteral,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::value::double_literal::FormatGritDoubleLiteral::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritDoubleLiteral {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritDoubleLiteral,
        crate::grit::value::double_literal::FormatGritDoubleLiteral,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::value::double_literal::FormatGritDoubleLiteral::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritEngineName>
    for crate::grit::auxiliary::engine_name::FormatGritEngineName
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritEngineName,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritEngineName>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritEngineName {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritEngineName,
        crate::grit::auxiliary::engine_name::FormatGritEngineName,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::auxiliary::engine_name::FormatGritEngineName::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritEngineName {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritEngineName,
        crate::grit::auxiliary::engine_name::FormatGritEngineName,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::auxiliary::engine_name::FormatGritEngineName::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritEvery> for crate::grit::auxiliary::every::FormatGritEvery {
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_grit_syntax::GritEvery, f: &mut GritFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritEvery>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritEvery {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritEvery,
        crate::grit::auxiliary::every::FormatGritEvery,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::auxiliary::every::FormatGritEvery::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritEvery {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritEvery,
        crate::grit::auxiliary::every::FormatGritEvery,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::auxiliary::every::FormatGritEvery::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritFiles> for crate::grit::auxiliary::files::FormatGritFiles {
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_grit_syntax::GritFiles, f: &mut GritFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritFiles>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritFiles {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritFiles,
        crate::grit::auxiliary::files::FormatGritFiles,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::auxiliary::files::FormatGritFiles::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritFiles {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritFiles,
        crate::grit::auxiliary::files::FormatGritFiles,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::auxiliary::files::FormatGritFiles::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritFunctionDefinition>
    for crate::grit::declarations::function_definition::FormatGritFunctionDefinition
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritFunctionDefinition,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritFunctionDefinition>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritFunctionDefinition {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritFunctionDefinition,
        crate::grit::declarations::function_definition::FormatGritFunctionDefinition,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::declarations::function_definition::FormatGritFunctionDefinition::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritFunctionDefinition {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritFunctionDefinition,
        crate::grit::declarations::function_definition::FormatGritFunctionDefinition,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::declarations::function_definition::FormatGritFunctionDefinition::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritIntLiteral>
    for crate::grit::value::int_literal::FormatGritIntLiteral
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritIntLiteral,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritIntLiteral>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritIntLiteral {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritIntLiteral,
        crate::grit::value::int_literal::FormatGritIntLiteral,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::value::int_literal::FormatGritIntLiteral::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritIntLiteral {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritIntLiteral,
        crate::grit::value::int_literal::FormatGritIntLiteral,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::value::int_literal::FormatGritIntLiteral::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritJavascriptBodyWrapper>
    for crate::grit::auxiliary::javascript_body_wrapper::FormatGritJavascriptBodyWrapper
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritJavascriptBodyWrapper,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritJavascriptBodyWrapper>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritJavascriptBodyWrapper {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritJavascriptBodyWrapper,
        crate::grit::auxiliary::javascript_body_wrapper::FormatGritJavascriptBodyWrapper,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: grit :: auxiliary :: javascript_body_wrapper :: FormatGritJavascriptBodyWrapper :: default ())
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritJavascriptBodyWrapper {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritJavascriptBodyWrapper,
        crate::grit::auxiliary::javascript_body_wrapper::FormatGritJavascriptBodyWrapper,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: grit :: auxiliary :: javascript_body_wrapper :: FormatGritJavascriptBodyWrapper :: default ())
    }
}
impl FormatRule < biome_grit_syntax :: GritJavascriptFunctionDefinition > for crate :: grit :: declarations :: javascript_function_definition :: FormatGritJavascriptFunctionDefinition { type Context = GritFormatContext ; # [inline (always)] fn fmt (& self , node : & biome_grit_syntax :: GritJavascriptFunctionDefinition , f : & mut GritFormatter) -> FormatResult < () > { FormatNodeRule :: < biome_grit_syntax :: GritJavascriptFunctionDefinition > :: fmt (self , node , f) } }
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritJavascriptFunctionDefinition {
    type Format < 'a > = FormatRefWithRule < 'a , biome_grit_syntax :: GritJavascriptFunctionDefinition , crate :: grit :: declarations :: javascript_function_definition :: FormatGritJavascriptFunctionDefinition > ;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: grit :: declarations :: javascript_function_definition :: FormatGritJavascriptFunctionDefinition :: default ())
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritJavascriptFunctionDefinition {
    type Format = FormatOwnedWithRule < biome_grit_syntax :: GritJavascriptFunctionDefinition , crate :: grit :: declarations :: javascript_function_definition :: FormatGritJavascriptFunctionDefinition > ;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: grit :: declarations :: javascript_function_definition :: FormatGritJavascriptFunctionDefinition :: default ())
    }
}
impl FormatRule<biome_grit_syntax::GritLanguageDeclaration>
    for crate::grit::auxiliary::language_declaration::FormatGritLanguageDeclaration
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritLanguageDeclaration,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritLanguageDeclaration>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritLanguageDeclaration {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritLanguageDeclaration,
        crate::grit::auxiliary::language_declaration::FormatGritLanguageDeclaration,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::auxiliary::language_declaration::FormatGritLanguageDeclaration::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritLanguageDeclaration {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritLanguageDeclaration,
        crate::grit::auxiliary::language_declaration::FormatGritLanguageDeclaration,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::auxiliary::language_declaration::FormatGritLanguageDeclaration::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritLanguageFlavor>
    for crate::grit::auxiliary::language_flavor::FormatGritLanguageFlavor
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritLanguageFlavor,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritLanguageFlavor>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritLanguageFlavor {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritLanguageFlavor,
        crate::grit::auxiliary::language_flavor::FormatGritLanguageFlavor,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::auxiliary::language_flavor::FormatGritLanguageFlavor::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritLanguageFlavor {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritLanguageFlavor,
        crate::grit::auxiliary::language_flavor::FormatGritLanguageFlavor,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::auxiliary::language_flavor::FormatGritLanguageFlavor::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritLanguageFlavorKind>
    for crate::grit::auxiliary::language_flavor_kind::FormatGritLanguageFlavorKind
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritLanguageFlavorKind,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritLanguageFlavorKind>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritLanguageFlavorKind {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritLanguageFlavorKind,
        crate::grit::auxiliary::language_flavor_kind::FormatGritLanguageFlavorKind,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::auxiliary::language_flavor_kind::FormatGritLanguageFlavorKind::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritLanguageFlavorKind {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritLanguageFlavorKind,
        crate::grit::auxiliary::language_flavor_kind::FormatGritLanguageFlavorKind,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::auxiliary::language_flavor_kind::FormatGritLanguageFlavorKind::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritLanguageName>
    for crate::grit::auxiliary::language_name::FormatGritLanguageName
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritLanguageName,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritLanguageName>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritLanguageName {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritLanguageName,
        crate::grit::auxiliary::language_name::FormatGritLanguageName,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::auxiliary::language_name::FormatGritLanguageName::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritLanguageName {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritLanguageName,
        crate::grit::auxiliary::language_name::FormatGritLanguageName,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::auxiliary::language_name::FormatGritLanguageName::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritLanguageSpecificSnippet>
    for crate::grit::auxiliary::language_specific_snippet::FormatGritLanguageSpecificSnippet
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritLanguageSpecificSnippet,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritLanguageSpecificSnippet>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritLanguageSpecificSnippet {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritLanguageSpecificSnippet,
        crate::grit::auxiliary::language_specific_snippet::FormatGritLanguageSpecificSnippet,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: grit :: auxiliary :: language_specific_snippet :: FormatGritLanguageSpecificSnippet :: default ())
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritLanguageSpecificSnippet {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritLanguageSpecificSnippet,
        crate::grit::auxiliary::language_specific_snippet::FormatGritLanguageSpecificSnippet,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: grit :: auxiliary :: language_specific_snippet :: FormatGritLanguageSpecificSnippet :: default ())
    }
}
impl FormatRule<biome_grit_syntax::GritLike> for crate::grit::auxiliary::like::FormatGritLike {
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_grit_syntax::GritLike, f: &mut GritFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritLike>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritLike {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritLike,
        crate::grit::auxiliary::like::FormatGritLike,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::auxiliary::like::FormatGritLike::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritLike {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritLike,
        crate::grit::auxiliary::like::FormatGritLike,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::auxiliary::like::FormatGritLike::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritLikeThreshold>
    for crate::grit::auxiliary::like_threshold::FormatGritLikeThreshold
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritLikeThreshold,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritLikeThreshold>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritLikeThreshold {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritLikeThreshold,
        crate::grit::auxiliary::like_threshold::FormatGritLikeThreshold,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::auxiliary::like_threshold::FormatGritLikeThreshold::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritLikeThreshold {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritLikeThreshold,
        crate::grit::auxiliary::like_threshold::FormatGritLikeThreshold,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::auxiliary::like_threshold::FormatGritLikeThreshold::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritList> for crate::grit::auxiliary::list::FormatGritList {
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_grit_syntax::GritList, f: &mut GritFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritList>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritList,
        crate::grit::auxiliary::list::FormatGritList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::auxiliary::list::FormatGritList::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritList {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritList,
        crate::grit::auxiliary::list::FormatGritList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::auxiliary::list::FormatGritList::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritListAccessor>
    for crate::grit::auxiliary::list_accessor::FormatGritListAccessor
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritListAccessor,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritListAccessor>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritListAccessor {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritListAccessor,
        crate::grit::auxiliary::list_accessor::FormatGritListAccessor,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::auxiliary::list_accessor::FormatGritListAccessor::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritListAccessor {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritListAccessor,
        crate::grit::auxiliary::list_accessor::FormatGritListAccessor,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::auxiliary::list_accessor::FormatGritListAccessor::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritMap> for crate::grit::auxiliary::map::FormatGritMap {
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_grit_syntax::GritMap, f: &mut GritFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritMap>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritMap {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritMap,
        crate::grit::auxiliary::map::FormatGritMap,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, crate::grit::auxiliary::map::FormatGritMap::default())
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritMap {
    type Format =
        FormatOwnedWithRule<biome_grit_syntax::GritMap, crate::grit::auxiliary::map::FormatGritMap>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, crate::grit::auxiliary::map::FormatGritMap::default())
    }
}
impl FormatRule<biome_grit_syntax::GritMapAccessor>
    for crate::grit::auxiliary::map_accessor::FormatGritMapAccessor
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritMapAccessor,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritMapAccessor>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritMapAccessor {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritMapAccessor,
        crate::grit::auxiliary::map_accessor::FormatGritMapAccessor,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::auxiliary::map_accessor::FormatGritMapAccessor::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritMapAccessor {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritMapAccessor,
        crate::grit::auxiliary::map_accessor::FormatGritMapAccessor,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::auxiliary::map_accessor::FormatGritMapAccessor::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritMapElement>
    for crate::grit::auxiliary::map_element::FormatGritMapElement
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritMapElement,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritMapElement>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritMapElement {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritMapElement,
        crate::grit::auxiliary::map_element::FormatGritMapElement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::auxiliary::map_element::FormatGritMapElement::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritMapElement {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritMapElement,
        crate::grit::auxiliary::map_element::FormatGritMapElement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::auxiliary::map_element::FormatGritMapElement::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritModOperation>
    for crate::grit::patterns::mod_operation::FormatGritModOperation
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritModOperation,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritModOperation>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritModOperation {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritModOperation,
        crate::grit::patterns::mod_operation::FormatGritModOperation,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::patterns::mod_operation::FormatGritModOperation::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritModOperation {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritModOperation,
        crate::grit::patterns::mod_operation::FormatGritModOperation,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::patterns::mod_operation::FormatGritModOperation::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritMulOperation>
    for crate::grit::patterns::mul_operation::FormatGritMulOperation
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritMulOperation,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritMulOperation>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritMulOperation {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritMulOperation,
        crate::grit::patterns::mul_operation::FormatGritMulOperation,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::patterns::mul_operation::FormatGritMulOperation::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritMulOperation {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritMulOperation,
        crate::grit::patterns::mul_operation::FormatGritMulOperation,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::patterns::mul_operation::FormatGritMulOperation::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritName> for crate::grit::auxiliary::name::FormatGritName {
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_grit_syntax::GritName, f: &mut GritFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritName>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritName {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritName,
        crate::grit::auxiliary::name::FormatGritName,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::auxiliary::name::FormatGritName::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritName {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritName,
        crate::grit::auxiliary::name::FormatGritName,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::auxiliary::name::FormatGritName::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritNamedArg>
    for crate::grit::auxiliary::named_arg::FormatGritNamedArg
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritNamedArg,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritNamedArg>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritNamedArg {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritNamedArg,
        crate::grit::auxiliary::named_arg::FormatGritNamedArg,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::auxiliary::named_arg::FormatGritNamedArg::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritNamedArg {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritNamedArg,
        crate::grit::auxiliary::named_arg::FormatGritNamedArg,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::auxiliary::named_arg::FormatGritNamedArg::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritNegativeIntLiteral>
    for crate::grit::value::negative_int_literal::FormatGritNegativeIntLiteral
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritNegativeIntLiteral,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritNegativeIntLiteral>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritNegativeIntLiteral {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritNegativeIntLiteral,
        crate::grit::value::negative_int_literal::FormatGritNegativeIntLiteral,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::value::negative_int_literal::FormatGritNegativeIntLiteral::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritNegativeIntLiteral {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritNegativeIntLiteral,
        crate::grit::value::negative_int_literal::FormatGritNegativeIntLiteral,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::value::negative_int_literal::FormatGritNegativeIntLiteral::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritNodeLike>
    for crate::grit::auxiliary::node_like::FormatGritNodeLike
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritNodeLike,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritNodeLike>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritNodeLike {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritNodeLike,
        crate::grit::auxiliary::node_like::FormatGritNodeLike,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::auxiliary::node_like::FormatGritNodeLike::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritNodeLike {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritNodeLike,
        crate::grit::auxiliary::node_like::FormatGritNodeLike,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::auxiliary::node_like::FormatGritNodeLike::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritNot> for crate::grit::auxiliary::not::FormatGritNot {
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_grit_syntax::GritNot, f: &mut GritFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritNot>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritNot {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritNot,
        crate::grit::auxiliary::not::FormatGritNot,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, crate::grit::auxiliary::not::FormatGritNot::default())
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritNot {
    type Format =
        FormatOwnedWithRule<biome_grit_syntax::GritNot, crate::grit::auxiliary::not::FormatGritNot>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, crate::grit::auxiliary::not::FormatGritNot::default())
    }
}
impl FormatRule<biome_grit_syntax::GritPatternAccumulate>
    for crate::grit::patterns::pattern_accumulate::FormatGritPatternAccumulate
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritPatternAccumulate,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritPatternAccumulate>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritPatternAccumulate {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritPatternAccumulate,
        crate::grit::patterns::pattern_accumulate::FormatGritPatternAccumulate,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::patterns::pattern_accumulate::FormatGritPatternAccumulate::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritPatternAccumulate {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritPatternAccumulate,
        crate::grit::patterns::pattern_accumulate::FormatGritPatternAccumulate,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::patterns::pattern_accumulate::FormatGritPatternAccumulate::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritPatternAfter>
    for crate::grit::patterns::pattern_after::FormatGritPatternAfter
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritPatternAfter,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritPatternAfter>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritPatternAfter {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritPatternAfter,
        crate::grit::patterns::pattern_after::FormatGritPatternAfter,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::patterns::pattern_after::FormatGritPatternAfter::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritPatternAfter {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritPatternAfter,
        crate::grit::patterns::pattern_after::FormatGritPatternAfter,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::patterns::pattern_after::FormatGritPatternAfter::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritPatternAnd>
    for crate::grit::patterns::pattern_and::FormatGritPatternAnd
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritPatternAnd,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritPatternAnd>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritPatternAnd {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritPatternAnd,
        crate::grit::patterns::pattern_and::FormatGritPatternAnd,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::patterns::pattern_and::FormatGritPatternAnd::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritPatternAnd {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritPatternAnd,
        crate::grit::patterns::pattern_and::FormatGritPatternAnd,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::patterns::pattern_and::FormatGritPatternAnd::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritPatternAny>
    for crate::grit::patterns::pattern_any::FormatGritPatternAny
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritPatternAny,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritPatternAny>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritPatternAny {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritPatternAny,
        crate::grit::patterns::pattern_any::FormatGritPatternAny,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::patterns::pattern_any::FormatGritPatternAny::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritPatternAny {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritPatternAny,
        crate::grit::patterns::pattern_any::FormatGritPatternAny,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::patterns::pattern_any::FormatGritPatternAny::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritPatternAs>
    for crate::grit::patterns::pattern_as::FormatGritPatternAs
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritPatternAs,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritPatternAs>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritPatternAs {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritPatternAs,
        crate::grit::patterns::pattern_as::FormatGritPatternAs,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::patterns::pattern_as::FormatGritPatternAs::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritPatternAs {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritPatternAs,
        crate::grit::patterns::pattern_as::FormatGritPatternAs,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::patterns::pattern_as::FormatGritPatternAs::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritPatternBefore>
    for crate::grit::patterns::pattern_before::FormatGritPatternBefore
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritPatternBefore,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritPatternBefore>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritPatternBefore {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritPatternBefore,
        crate::grit::patterns::pattern_before::FormatGritPatternBefore,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::patterns::pattern_before::FormatGritPatternBefore::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritPatternBefore {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritPatternBefore,
        crate::grit::patterns::pattern_before::FormatGritPatternBefore,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::patterns::pattern_before::FormatGritPatternBefore::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritPatternContains>
    for crate::grit::patterns::pattern_contains::FormatGritPatternContains
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritPatternContains,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritPatternContains>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritPatternContains {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritPatternContains,
        crate::grit::patterns::pattern_contains::FormatGritPatternContains,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::patterns::pattern_contains::FormatGritPatternContains::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritPatternContains {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritPatternContains,
        crate::grit::patterns::pattern_contains::FormatGritPatternContains,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::patterns::pattern_contains::FormatGritPatternContains::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritPatternDefinition>
    for crate::grit::patterns::pattern_definition::FormatGritPatternDefinition
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritPatternDefinition,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritPatternDefinition>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritPatternDefinition {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritPatternDefinition,
        crate::grit::patterns::pattern_definition::FormatGritPatternDefinition,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::patterns::pattern_definition::FormatGritPatternDefinition::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritPatternDefinition {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritPatternDefinition,
        crate::grit::patterns::pattern_definition::FormatGritPatternDefinition,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::patterns::pattern_definition::FormatGritPatternDefinition::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritPatternDefinitionBody>
    for crate::grit::patterns::pattern_definition_body::FormatGritPatternDefinitionBody
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritPatternDefinitionBody,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritPatternDefinitionBody>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritPatternDefinitionBody {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritPatternDefinitionBody,
        crate::grit::patterns::pattern_definition_body::FormatGritPatternDefinitionBody,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: grit :: patterns :: pattern_definition_body :: FormatGritPatternDefinitionBody :: default ())
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritPatternDefinitionBody {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritPatternDefinitionBody,
        crate::grit::patterns::pattern_definition_body::FormatGritPatternDefinitionBody,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: grit :: patterns :: pattern_definition_body :: FormatGritPatternDefinitionBody :: default ())
    }
}
impl FormatRule<biome_grit_syntax::GritPatternElseClause>
    for crate::grit::patterns::pattern_else_clause::FormatGritPatternElseClause
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritPatternElseClause,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritPatternElseClause>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritPatternElseClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritPatternElseClause,
        crate::grit::patterns::pattern_else_clause::FormatGritPatternElseClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::patterns::pattern_else_clause::FormatGritPatternElseClause::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritPatternElseClause {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritPatternElseClause,
        crate::grit::patterns::pattern_else_clause::FormatGritPatternElseClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::patterns::pattern_else_clause::FormatGritPatternElseClause::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritPatternIfElse>
    for crate::grit::patterns::pattern_if_else::FormatGritPatternIfElse
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritPatternIfElse,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritPatternIfElse>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritPatternIfElse {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritPatternIfElse,
        crate::grit::patterns::pattern_if_else::FormatGritPatternIfElse,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::patterns::pattern_if_else::FormatGritPatternIfElse::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritPatternIfElse {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritPatternIfElse,
        crate::grit::patterns::pattern_if_else::FormatGritPatternIfElse,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::patterns::pattern_if_else::FormatGritPatternIfElse::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritPatternIncludes>
    for crate::grit::patterns::pattern_includes::FormatGritPatternIncludes
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritPatternIncludes,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritPatternIncludes>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritPatternIncludes {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritPatternIncludes,
        crate::grit::patterns::pattern_includes::FormatGritPatternIncludes,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::patterns::pattern_includes::FormatGritPatternIncludes::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritPatternIncludes {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritPatternIncludes,
        crate::grit::patterns::pattern_includes::FormatGritPatternIncludes,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::patterns::pattern_includes::FormatGritPatternIncludes::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritPatternLimit>
    for crate::grit::patterns::pattern_limit::FormatGritPatternLimit
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritPatternLimit,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritPatternLimit>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritPatternLimit {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritPatternLimit,
        crate::grit::patterns::pattern_limit::FormatGritPatternLimit,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::patterns::pattern_limit::FormatGritPatternLimit::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritPatternLimit {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritPatternLimit,
        crate::grit::patterns::pattern_limit::FormatGritPatternLimit,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::patterns::pattern_limit::FormatGritPatternLimit::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritPatternMaybe>
    for crate::grit::patterns::pattern_maybe::FormatGritPatternMaybe
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritPatternMaybe,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritPatternMaybe>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritPatternMaybe {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritPatternMaybe,
        crate::grit::patterns::pattern_maybe::FormatGritPatternMaybe,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::patterns::pattern_maybe::FormatGritPatternMaybe::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritPatternMaybe {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritPatternMaybe,
        crate::grit::patterns::pattern_maybe::FormatGritPatternMaybe,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::patterns::pattern_maybe::FormatGritPatternMaybe::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritPatternNot>
    for crate::grit::patterns::pattern_not::FormatGritPatternNot
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritPatternNot,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritPatternNot>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritPatternNot {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritPatternNot,
        crate::grit::patterns::pattern_not::FormatGritPatternNot,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::patterns::pattern_not::FormatGritPatternNot::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritPatternNot {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritPatternNot,
        crate::grit::patterns::pattern_not::FormatGritPatternNot,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::patterns::pattern_not::FormatGritPatternNot::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritPatternOr>
    for crate::grit::patterns::pattern_or::FormatGritPatternOr
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritPatternOr,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritPatternOr>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritPatternOr {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritPatternOr,
        crate::grit::patterns::pattern_or::FormatGritPatternOr,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::patterns::pattern_or::FormatGritPatternOr::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritPatternOr {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritPatternOr,
        crate::grit::patterns::pattern_or::FormatGritPatternOr,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::patterns::pattern_or::FormatGritPatternOr::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritPatternOrElse>
    for crate::grit::patterns::pattern_or_else::FormatGritPatternOrElse
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritPatternOrElse,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritPatternOrElse>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritPatternOrElse {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritPatternOrElse,
        crate::grit::patterns::pattern_or_else::FormatGritPatternOrElse,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::patterns::pattern_or_else::FormatGritPatternOrElse::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritPatternOrElse {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritPatternOrElse,
        crate::grit::patterns::pattern_or_else::FormatGritPatternOrElse,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::patterns::pattern_or_else::FormatGritPatternOrElse::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritPatternUntilClause>
    for crate::grit::patterns::pattern_until_clause::FormatGritPatternUntilClause
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritPatternUntilClause,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritPatternUntilClause>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritPatternUntilClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritPatternUntilClause,
        crate::grit::patterns::pattern_until_clause::FormatGritPatternUntilClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::patterns::pattern_until_clause::FormatGritPatternUntilClause::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritPatternUntilClause {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritPatternUntilClause,
        crate::grit::patterns::pattern_until_clause::FormatGritPatternUntilClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::patterns::pattern_until_clause::FormatGritPatternUntilClause::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritPatternWhere>
    for crate::grit::patterns::pattern_where::FormatGritPatternWhere
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritPatternWhere,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritPatternWhere>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritPatternWhere {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritPatternWhere,
        crate::grit::patterns::pattern_where::FormatGritPatternWhere,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::patterns::pattern_where::FormatGritPatternWhere::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritPatternWhere {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritPatternWhere,
        crate::grit::patterns::pattern_where::FormatGritPatternWhere,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::patterns::pattern_where::FormatGritPatternWhere::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritPredicateAccumulate>
    for crate::grit::predicates::predicate_accumulate::FormatGritPredicateAccumulate
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritPredicateAccumulate,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritPredicateAccumulate>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritPredicateAccumulate {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritPredicateAccumulate,
        crate::grit::predicates::predicate_accumulate::FormatGritPredicateAccumulate,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::predicates::predicate_accumulate::FormatGritPredicateAccumulate::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritPredicateAccumulate {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritPredicateAccumulate,
        crate::grit::predicates::predicate_accumulate::FormatGritPredicateAccumulate,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::predicates::predicate_accumulate::FormatGritPredicateAccumulate::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritPredicateAnd>
    for crate::grit::predicates::predicate_and::FormatGritPredicateAnd
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritPredicateAnd,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritPredicateAnd>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritPredicateAnd {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritPredicateAnd,
        crate::grit::predicates::predicate_and::FormatGritPredicateAnd,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::predicates::predicate_and::FormatGritPredicateAnd::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritPredicateAnd {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritPredicateAnd,
        crate::grit::predicates::predicate_and::FormatGritPredicateAnd,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::predicates::predicate_and::FormatGritPredicateAnd::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritPredicateAny>
    for crate::grit::predicates::predicate_any::FormatGritPredicateAny
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritPredicateAny,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritPredicateAny>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritPredicateAny {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritPredicateAny,
        crate::grit::predicates::predicate_any::FormatGritPredicateAny,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::predicates::predicate_any::FormatGritPredicateAny::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritPredicateAny {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritPredicateAny,
        crate::grit::predicates::predicate_any::FormatGritPredicateAny,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::predicates::predicate_any::FormatGritPredicateAny::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritPredicateAssignment>
    for crate::grit::predicates::predicate_assignment::FormatGritPredicateAssignment
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritPredicateAssignment,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritPredicateAssignment>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritPredicateAssignment {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritPredicateAssignment,
        crate::grit::predicates::predicate_assignment::FormatGritPredicateAssignment,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::predicates::predicate_assignment::FormatGritPredicateAssignment::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritPredicateAssignment {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritPredicateAssignment,
        crate::grit::predicates::predicate_assignment::FormatGritPredicateAssignment,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::predicates::predicate_assignment::FormatGritPredicateAssignment::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritPredicateCall>
    for crate::grit::predicates::predicate_call::FormatGritPredicateCall
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritPredicateCall,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritPredicateCall>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritPredicateCall {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritPredicateCall,
        crate::grit::predicates::predicate_call::FormatGritPredicateCall,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::predicates::predicate_call::FormatGritPredicateCall::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritPredicateCall {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritPredicateCall,
        crate::grit::predicates::predicate_call::FormatGritPredicateCall,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::predicates::predicate_call::FormatGritPredicateCall::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritPredicateCurly>
    for crate::grit::predicates::predicate_curly::FormatGritPredicateCurly
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritPredicateCurly,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritPredicateCurly>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritPredicateCurly {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritPredicateCurly,
        crate::grit::predicates::predicate_curly::FormatGritPredicateCurly,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::predicates::predicate_curly::FormatGritPredicateCurly::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritPredicateCurly {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritPredicateCurly,
        crate::grit::predicates::predicate_curly::FormatGritPredicateCurly,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::predicates::predicate_curly::FormatGritPredicateCurly::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritPredicateDefinition>
    for crate::grit::predicates::predicate_definition::FormatGritPredicateDefinition
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritPredicateDefinition,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritPredicateDefinition>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritPredicateDefinition {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritPredicateDefinition,
        crate::grit::predicates::predicate_definition::FormatGritPredicateDefinition,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::predicates::predicate_definition::FormatGritPredicateDefinition::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritPredicateDefinition {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritPredicateDefinition,
        crate::grit::predicates::predicate_definition::FormatGritPredicateDefinition,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::predicates::predicate_definition::FormatGritPredicateDefinition::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritPredicateElseClause>
    for crate::grit::predicates::predicate_else_clause::FormatGritPredicateElseClause
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritPredicateElseClause,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritPredicateElseClause>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritPredicateElseClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritPredicateElseClause,
        crate::grit::predicates::predicate_else_clause::FormatGritPredicateElseClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::predicates::predicate_else_clause::FormatGritPredicateElseClause::default(
            ),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritPredicateElseClause {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritPredicateElseClause,
        crate::grit::predicates::predicate_else_clause::FormatGritPredicateElseClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::predicates::predicate_else_clause::FormatGritPredicateElseClause::default(
            ),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritPredicateEqual>
    for crate::grit::predicates::predicate_equal::FormatGritPredicateEqual
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritPredicateEqual,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritPredicateEqual>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritPredicateEqual {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritPredicateEqual,
        crate::grit::predicates::predicate_equal::FormatGritPredicateEqual,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::predicates::predicate_equal::FormatGritPredicateEqual::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritPredicateEqual {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritPredicateEqual,
        crate::grit::predicates::predicate_equal::FormatGritPredicateEqual,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::predicates::predicate_equal::FormatGritPredicateEqual::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritPredicateGreater>
    for crate::grit::predicates::predicate_greater::FormatGritPredicateGreater
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritPredicateGreater,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritPredicateGreater>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritPredicateGreater {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritPredicateGreater,
        crate::grit::predicates::predicate_greater::FormatGritPredicateGreater,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::predicates::predicate_greater::FormatGritPredicateGreater::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritPredicateGreater {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritPredicateGreater,
        crate::grit::predicates::predicate_greater::FormatGritPredicateGreater,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::predicates::predicate_greater::FormatGritPredicateGreater::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritPredicateGreaterEqual>
    for crate::grit::predicates::predicate_greater_equal::FormatGritPredicateGreaterEqual
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritPredicateGreaterEqual,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritPredicateGreaterEqual>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritPredicateGreaterEqual {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritPredicateGreaterEqual,
        crate::grit::predicates::predicate_greater_equal::FormatGritPredicateGreaterEqual,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: grit :: predicates :: predicate_greater_equal :: FormatGritPredicateGreaterEqual :: default ())
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritPredicateGreaterEqual {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritPredicateGreaterEqual,
        crate::grit::predicates::predicate_greater_equal::FormatGritPredicateGreaterEqual,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: grit :: predicates :: predicate_greater_equal :: FormatGritPredicateGreaterEqual :: default ())
    }
}
impl FormatRule<biome_grit_syntax::GritPredicateIfElse>
    for crate::grit::predicates::predicate_if_else::FormatGritPredicateIfElse
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritPredicateIfElse,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritPredicateIfElse>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritPredicateIfElse {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritPredicateIfElse,
        crate::grit::predicates::predicate_if_else::FormatGritPredicateIfElse,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::predicates::predicate_if_else::FormatGritPredicateIfElse::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritPredicateIfElse {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritPredicateIfElse,
        crate::grit::predicates::predicate_if_else::FormatGritPredicateIfElse,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::predicates::predicate_if_else::FormatGritPredicateIfElse::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritPredicateLess>
    for crate::grit::predicates::predicate_less::FormatGritPredicateLess
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritPredicateLess,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritPredicateLess>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritPredicateLess {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritPredicateLess,
        crate::grit::predicates::predicate_less::FormatGritPredicateLess,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::predicates::predicate_less::FormatGritPredicateLess::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritPredicateLess {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritPredicateLess,
        crate::grit::predicates::predicate_less::FormatGritPredicateLess,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::predicates::predicate_less::FormatGritPredicateLess::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritPredicateLessEqual>
    for crate::grit::predicates::predicate_less_equal::FormatGritPredicateLessEqual
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritPredicateLessEqual,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritPredicateLessEqual>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritPredicateLessEqual {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritPredicateLessEqual,
        crate::grit::predicates::predicate_less_equal::FormatGritPredicateLessEqual,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::predicates::predicate_less_equal::FormatGritPredicateLessEqual::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritPredicateLessEqual {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritPredicateLessEqual,
        crate::grit::predicates::predicate_less_equal::FormatGritPredicateLessEqual,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::predicates::predicate_less_equal::FormatGritPredicateLessEqual::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritPredicateMatch>
    for crate::grit::predicates::predicate_match::FormatGritPredicateMatch
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritPredicateMatch,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritPredicateMatch>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritPredicateMatch {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritPredicateMatch,
        crate::grit::predicates::predicate_match::FormatGritPredicateMatch,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::predicates::predicate_match::FormatGritPredicateMatch::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritPredicateMatch {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritPredicateMatch,
        crate::grit::predicates::predicate_match::FormatGritPredicateMatch,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::predicates::predicate_match::FormatGritPredicateMatch::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritPredicateMaybe>
    for crate::grit::predicates::predicate_maybe::FormatGritPredicateMaybe
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritPredicateMaybe,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritPredicateMaybe>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritPredicateMaybe {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritPredicateMaybe,
        crate::grit::predicates::predicate_maybe::FormatGritPredicateMaybe,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::predicates::predicate_maybe::FormatGritPredicateMaybe::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritPredicateMaybe {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritPredicateMaybe,
        crate::grit::predicates::predicate_maybe::FormatGritPredicateMaybe,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::predicates::predicate_maybe::FormatGritPredicateMaybe::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritPredicateNot>
    for crate::grit::predicates::predicate_not::FormatGritPredicateNot
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritPredicateNot,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritPredicateNot>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritPredicateNot {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritPredicateNot,
        crate::grit::predicates::predicate_not::FormatGritPredicateNot,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::predicates::predicate_not::FormatGritPredicateNot::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritPredicateNot {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritPredicateNot,
        crate::grit::predicates::predicate_not::FormatGritPredicateNot,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::predicates::predicate_not::FormatGritPredicateNot::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritPredicateNotEqual>
    for crate::grit::predicates::predicate_not_equal::FormatGritPredicateNotEqual
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritPredicateNotEqual,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritPredicateNotEqual>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritPredicateNotEqual {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritPredicateNotEqual,
        crate::grit::predicates::predicate_not_equal::FormatGritPredicateNotEqual,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::predicates::predicate_not_equal::FormatGritPredicateNotEqual::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritPredicateNotEqual {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritPredicateNotEqual,
        crate::grit::predicates::predicate_not_equal::FormatGritPredicateNotEqual,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::predicates::predicate_not_equal::FormatGritPredicateNotEqual::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritPredicateOr>
    for crate::grit::predicates::predicate_or::FormatGritPredicateOr
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritPredicateOr,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritPredicateOr>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritPredicateOr {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritPredicateOr,
        crate::grit::predicates::predicate_or::FormatGritPredicateOr,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::predicates::predicate_or::FormatGritPredicateOr::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritPredicateOr {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritPredicateOr,
        crate::grit::predicates::predicate_or::FormatGritPredicateOr,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::predicates::predicate_or::FormatGritPredicateOr::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritPredicateReturn>
    for crate::grit::predicates::predicate_return::FormatGritPredicateReturn
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritPredicateReturn,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritPredicateReturn>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritPredicateReturn {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritPredicateReturn,
        crate::grit::predicates::predicate_return::FormatGritPredicateReturn,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::predicates::predicate_return::FormatGritPredicateReturn::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritPredicateReturn {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritPredicateReturn,
        crate::grit::predicates::predicate_return::FormatGritPredicateReturn,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::predicates::predicate_return::FormatGritPredicateReturn::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritPredicateRewrite>
    for crate::grit::predicates::predicate_rewrite::FormatGritPredicateRewrite
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritPredicateRewrite,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritPredicateRewrite>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritPredicateRewrite {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritPredicateRewrite,
        crate::grit::predicates::predicate_rewrite::FormatGritPredicateRewrite,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::predicates::predicate_rewrite::FormatGritPredicateRewrite::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritPredicateRewrite {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritPredicateRewrite,
        crate::grit::predicates::predicate_rewrite::FormatGritPredicateRewrite,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::predicates::predicate_rewrite::FormatGritPredicateRewrite::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritRawBacktickSnippetLiteral>
    for crate::grit::value::raw_backtick_snippet_literal::FormatGritRawBacktickSnippetLiteral
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritRawBacktickSnippetLiteral,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritRawBacktickSnippetLiteral>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritRawBacktickSnippetLiteral {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritRawBacktickSnippetLiteral,
        crate::grit::value::raw_backtick_snippet_literal::FormatGritRawBacktickSnippetLiteral,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: grit :: value :: raw_backtick_snippet_literal :: FormatGritRawBacktickSnippetLiteral :: default ())
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritRawBacktickSnippetLiteral {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritRawBacktickSnippetLiteral,
        crate::grit::value::raw_backtick_snippet_literal::FormatGritRawBacktickSnippetLiteral,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: grit :: value :: raw_backtick_snippet_literal :: FormatGritRawBacktickSnippetLiteral :: default ())
    }
}
impl FormatRule<biome_grit_syntax::GritRegexLiteral>
    for crate::grit::value::regex_literal::FormatGritRegexLiteral
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritRegexLiteral,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritRegexLiteral>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritRegexLiteral {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritRegexLiteral,
        crate::grit::value::regex_literal::FormatGritRegexLiteral,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::value::regex_literal::FormatGritRegexLiteral::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritRegexLiteral {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritRegexLiteral,
        crate::grit::value::regex_literal::FormatGritRegexLiteral,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::value::regex_literal::FormatGritRegexLiteral::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritRegexPattern>
    for crate::grit::patterns::regex_pattern::FormatGritRegexPattern
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritRegexPattern,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritRegexPattern>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritRegexPattern {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritRegexPattern,
        crate::grit::patterns::regex_pattern::FormatGritRegexPattern,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::patterns::regex_pattern::FormatGritRegexPattern::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritRegexPattern {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritRegexPattern,
        crate::grit::patterns::regex_pattern::FormatGritRegexPattern,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::patterns::regex_pattern::FormatGritRegexPattern::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritRegexPatternVariables>
    for crate::grit::patterns::regex_pattern_variables::FormatGritRegexPatternVariables
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritRegexPatternVariables,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritRegexPatternVariables>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritRegexPatternVariables {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritRegexPatternVariables,
        crate::grit::patterns::regex_pattern_variables::FormatGritRegexPatternVariables,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: grit :: patterns :: regex_pattern_variables :: FormatGritRegexPatternVariables :: default ())
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritRegexPatternVariables {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritRegexPatternVariables,
        crate::grit::patterns::regex_pattern_variables::FormatGritRegexPatternVariables,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: grit :: patterns :: regex_pattern_variables :: FormatGritRegexPatternVariables :: default ())
    }
}
impl FormatRule<biome_grit_syntax::GritRewrite>
    for crate::grit::auxiliary::rewrite::FormatGritRewrite
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritRewrite,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritRewrite>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritRewrite {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritRewrite,
        crate::grit::auxiliary::rewrite::FormatGritRewrite,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::auxiliary::rewrite::FormatGritRewrite::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritRewrite {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritRewrite,
        crate::grit::auxiliary::rewrite::FormatGritRewrite,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::auxiliary::rewrite::FormatGritRewrite::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritRoot> for crate::grit::auxiliary::root::FormatGritRoot {
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_grit_syntax::GritRoot, f: &mut GritFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritRoot>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritRoot {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritRoot,
        crate::grit::auxiliary::root::FormatGritRoot,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::auxiliary::root::FormatGritRoot::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritRoot {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritRoot,
        crate::grit::auxiliary::root::FormatGritRoot,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::auxiliary::root::FormatGritRoot::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritSequential>
    for crate::grit::auxiliary::sequential::FormatGritSequential
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritSequential,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritSequential>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritSequential {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritSequential,
        crate::grit::auxiliary::sequential::FormatGritSequential,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::auxiliary::sequential::FormatGritSequential::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritSequential {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritSequential,
        crate::grit::auxiliary::sequential::FormatGritSequential,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::auxiliary::sequential::FormatGritSequential::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritSnippetRegexLiteral>
    for crate::grit::value::snippet_regex_literal::FormatGritSnippetRegexLiteral
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritSnippetRegexLiteral,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritSnippetRegexLiteral>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritSnippetRegexLiteral {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritSnippetRegexLiteral,
        crate::grit::value::snippet_regex_literal::FormatGritSnippetRegexLiteral,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::value::snippet_regex_literal::FormatGritSnippetRegexLiteral::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritSnippetRegexLiteral {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritSnippetRegexLiteral,
        crate::grit::value::snippet_regex_literal::FormatGritSnippetRegexLiteral,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::value::snippet_regex_literal::FormatGritSnippetRegexLiteral::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritSome> for crate::grit::auxiliary::some::FormatGritSome {
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_grit_syntax::GritSome, f: &mut GritFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritSome>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritSome {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritSome,
        crate::grit::auxiliary::some::FormatGritSome,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::auxiliary::some::FormatGritSome::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritSome {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritSome,
        crate::grit::auxiliary::some::FormatGritSome,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::auxiliary::some::FormatGritSome::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritStringLiteral>
    for crate::grit::value::string_literal::FormatGritStringLiteral
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritStringLiteral,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritStringLiteral>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritStringLiteral {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritStringLiteral,
        crate::grit::value::string_literal::FormatGritStringLiteral,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::value::string_literal::FormatGritStringLiteral::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritStringLiteral {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritStringLiteral,
        crate::grit::value::string_literal::FormatGritStringLiteral,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::value::string_literal::FormatGritStringLiteral::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritSubOperation>
    for crate::grit::patterns::sub_operation::FormatGritSubOperation
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritSubOperation,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritSubOperation>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritSubOperation {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritSubOperation,
        crate::grit::patterns::sub_operation::FormatGritSubOperation,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::patterns::sub_operation::FormatGritSubOperation::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritSubOperation {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritSubOperation,
        crate::grit::patterns::sub_operation::FormatGritSubOperation,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::patterns::sub_operation::FormatGritSubOperation::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritUndefinedLiteral>
    for crate::grit::value::undefined_literal::FormatGritUndefinedLiteral
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritUndefinedLiteral,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritUndefinedLiteral>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritUndefinedLiteral {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritUndefinedLiteral,
        crate::grit::value::undefined_literal::FormatGritUndefinedLiteral,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::value::undefined_literal::FormatGritUndefinedLiteral::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritUndefinedLiteral {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritUndefinedLiteral,
        crate::grit::value::undefined_literal::FormatGritUndefinedLiteral,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::value::undefined_literal::FormatGritUndefinedLiteral::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritUnderscore>
    for crate::grit::auxiliary::underscore::FormatGritUnderscore
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritUnderscore,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritUnderscore>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritUnderscore {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritUnderscore,
        crate::grit::auxiliary::underscore::FormatGritUnderscore,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::auxiliary::underscore::FormatGritUnderscore::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritUnderscore {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritUnderscore,
        crate::grit::auxiliary::underscore::FormatGritUnderscore,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::auxiliary::underscore::FormatGritUnderscore::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritVariable>
    for crate::grit::auxiliary::variable::FormatGritVariable
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritVariable,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritVariable>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritVariable {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritVariable,
        crate::grit::auxiliary::variable::FormatGritVariable,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::auxiliary::variable::FormatGritVariable::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritVariable {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritVariable,
        crate::grit::auxiliary::variable::FormatGritVariable,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::auxiliary::variable::FormatGritVariable::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritVersion>
    for crate::grit::auxiliary::version::FormatGritVersion
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritVersion,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritVersion>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritVersion {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritVersion,
        crate::grit::auxiliary::version::FormatGritVersion,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::auxiliary::version::FormatGritVersion::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritVersion {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritVersion,
        crate::grit::auxiliary::version::FormatGritVersion,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::auxiliary::version::FormatGritVersion::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritWithin>
    for crate::grit::auxiliary::within::FormatGritWithin
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_grit_syntax::GritWithin, f: &mut GritFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_grit_syntax::GritWithin>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritWithin {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritWithin,
        crate::grit::auxiliary::within::FormatGritWithin,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::auxiliary::within::FormatGritWithin::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritWithin {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritWithin,
        crate::grit::auxiliary::within::FormatGritWithin,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::auxiliary::within::FormatGritWithin::default(),
        )
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritDefinitionList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritDefinitionList,
        crate::grit::lists::definition_list::FormatGritDefinitionList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::lists::definition_list::FormatGritDefinitionList::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritDefinitionList {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritDefinitionList,
        crate::grit::lists::definition_list::FormatGritDefinitionList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::lists::definition_list::FormatGritDefinitionList::default(),
        )
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritLanguageFlavorList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritLanguageFlavorList,
        crate::grit::lists::language_flavor_list::FormatGritLanguageFlavorList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::lists::language_flavor_list::FormatGritLanguageFlavorList::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritLanguageFlavorList {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritLanguageFlavorList,
        crate::grit::lists::language_flavor_list::FormatGritLanguageFlavorList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::lists::language_flavor_list::FormatGritLanguageFlavorList::default(),
        )
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritListPatternList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritListPatternList,
        crate::grit::lists::list_pattern_list::FormatGritListPatternList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::lists::list_pattern_list::FormatGritListPatternList::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritListPatternList {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritListPatternList,
        crate::grit::lists::list_pattern_list::FormatGritListPatternList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::lists::list_pattern_list::FormatGritListPatternList::default(),
        )
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritMapElementList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritMapElementList,
        crate::grit::lists::map_element_list::FormatGritMapElementList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::lists::map_element_list::FormatGritMapElementList::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritMapElementList {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritMapElementList,
        crate::grit::lists::map_element_list::FormatGritMapElementList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::lists::map_element_list::FormatGritMapElementList::default(),
        )
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritNamedArgList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritNamedArgList,
        crate::grit::lists::named_arg_list::FormatGritNamedArgList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::lists::named_arg_list::FormatGritNamedArgList::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritNamedArgList {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritNamedArgList,
        crate::grit::lists::named_arg_list::FormatGritNamedArgList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::lists::named_arg_list::FormatGritNamedArgList::default(),
        )
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritPatternList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritPatternList,
        crate::grit::lists::pattern_list::FormatGritPatternList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::lists::pattern_list::FormatGritPatternList::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritPatternList {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritPatternList,
        crate::grit::lists::pattern_list::FormatGritPatternList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::lists::pattern_list::FormatGritPatternList::default(),
        )
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritPredicateList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritPredicateList,
        crate::grit::lists::predicate_list::FormatGritPredicateList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::lists::predicate_list::FormatGritPredicateList::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritPredicateList {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritPredicateList,
        crate::grit::lists::predicate_list::FormatGritPredicateList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::lists::predicate_list::FormatGritPredicateList::default(),
        )
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritVariableList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritVariableList,
        crate::grit::lists::variable_list::FormatGritVariableList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::lists::variable_list::FormatGritVariableList::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritVariableList {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritVariableList,
        crate::grit::lists::variable_list::FormatGritVariableList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::lists::variable_list::FormatGritVariableList::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritBogus> for crate::grit::bogus::bogus::FormatGritBogus {
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_grit_syntax::GritBogus, f: &mut GritFormatter) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_grit_syntax::GritBogus>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritBogus {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritBogus,
        crate::grit::bogus::bogus::FormatGritBogus,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, crate::grit::bogus::bogus::FormatGritBogus::default())
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritBogus {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritBogus,
        crate::grit::bogus::bogus::FormatGritBogus,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, crate::grit::bogus::bogus::FormatGritBogus::default())
    }
}
impl FormatRule<biome_grit_syntax::GritBogusContainer>
    for crate::grit::bogus::bogus_container::FormatGritBogusContainer
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritBogusContainer,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_grit_syntax::GritBogusContainer>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritBogusContainer {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritBogusContainer,
        crate::grit::bogus::bogus_container::FormatGritBogusContainer,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::bogus::bogus_container::FormatGritBogusContainer::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritBogusContainer {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritBogusContainer,
        crate::grit::bogus::bogus_container::FormatGritBogusContainer,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::bogus::bogus_container::FormatGritBogusContainer::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritBogusDefinition>
    for crate::grit::bogus::bogus_definition::FormatGritBogusDefinition
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritBogusDefinition,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_grit_syntax::GritBogusDefinition>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritBogusDefinition {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritBogusDefinition,
        crate::grit::bogus::bogus_definition::FormatGritBogusDefinition,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::bogus::bogus_definition::FormatGritBogusDefinition::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritBogusDefinition {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritBogusDefinition,
        crate::grit::bogus::bogus_definition::FormatGritBogusDefinition,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::bogus::bogus_definition::FormatGritBogusDefinition::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritBogusLanguageDeclaration>
    for crate::grit::bogus::bogus_language_declaration::FormatGritBogusLanguageDeclaration
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritBogusLanguageDeclaration,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_grit_syntax::GritBogusLanguageDeclaration>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritBogusLanguageDeclaration {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritBogusLanguageDeclaration,
        crate::grit::bogus::bogus_language_declaration::FormatGritBogusLanguageDeclaration,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: grit :: bogus :: bogus_language_declaration :: FormatGritBogusLanguageDeclaration :: default ())
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritBogusLanguageDeclaration {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritBogusLanguageDeclaration,
        crate::grit::bogus::bogus_language_declaration::FormatGritBogusLanguageDeclaration,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: grit :: bogus :: bogus_language_declaration :: FormatGritBogusLanguageDeclaration :: default ())
    }
}
impl FormatRule<biome_grit_syntax::GritBogusLanguageFlavorKind>
    for crate::grit::bogus::bogus_language_flavor_kind::FormatGritBogusLanguageFlavorKind
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritBogusLanguageFlavorKind,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_grit_syntax::GritBogusLanguageFlavorKind>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritBogusLanguageFlavorKind {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritBogusLanguageFlavorKind,
        crate::grit::bogus::bogus_language_flavor_kind::FormatGritBogusLanguageFlavorKind,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: grit :: bogus :: bogus_language_flavor_kind :: FormatGritBogusLanguageFlavorKind :: default ())
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritBogusLanguageFlavorKind {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritBogusLanguageFlavorKind,
        crate::grit::bogus::bogus_language_flavor_kind::FormatGritBogusLanguageFlavorKind,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: grit :: bogus :: bogus_language_flavor_kind :: FormatGritBogusLanguageFlavorKind :: default ())
    }
}
impl FormatRule<biome_grit_syntax::GritBogusLanguageName>
    for crate::grit::bogus::bogus_language_name::FormatGritBogusLanguageName
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritBogusLanguageName,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_grit_syntax::GritBogusLanguageName>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritBogusLanguageName {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritBogusLanguageName,
        crate::grit::bogus::bogus_language_name::FormatGritBogusLanguageName,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::bogus::bogus_language_name::FormatGritBogusLanguageName::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritBogusLanguageName {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritBogusLanguageName,
        crate::grit::bogus::bogus_language_name::FormatGritBogusLanguageName,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::bogus::bogus_language_name::FormatGritBogusLanguageName::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritBogusLiteral>
    for crate::grit::bogus::bogus_literal::FormatGritBogusLiteral
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritBogusLiteral,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_grit_syntax::GritBogusLiteral>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritBogusLiteral {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritBogusLiteral,
        crate::grit::bogus::bogus_literal::FormatGritBogusLiteral,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::bogus::bogus_literal::FormatGritBogusLiteral::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritBogusLiteral {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritBogusLiteral,
        crate::grit::bogus::bogus_literal::FormatGritBogusLiteral,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::bogus::bogus_literal::FormatGritBogusLiteral::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritBogusMapElement>
    for crate::grit::bogus::bogus_map_element::FormatGritBogusMapElement
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritBogusMapElement,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_grit_syntax::GritBogusMapElement>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritBogusMapElement {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritBogusMapElement,
        crate::grit::bogus::bogus_map_element::FormatGritBogusMapElement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::bogus::bogus_map_element::FormatGritBogusMapElement::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritBogusMapElement {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritBogusMapElement,
        crate::grit::bogus::bogus_map_element::FormatGritBogusMapElement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::bogus::bogus_map_element::FormatGritBogusMapElement::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritBogusNamedArg>
    for crate::grit::bogus::bogus_named_arg::FormatGritBogusNamedArg
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritBogusNamedArg,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_grit_syntax::GritBogusNamedArg>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritBogusNamedArg {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritBogusNamedArg,
        crate::grit::bogus::bogus_named_arg::FormatGritBogusNamedArg,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::bogus::bogus_named_arg::FormatGritBogusNamedArg::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritBogusNamedArg {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritBogusNamedArg,
        crate::grit::bogus::bogus_named_arg::FormatGritBogusNamedArg,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::bogus::bogus_named_arg::FormatGritBogusNamedArg::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritBogusPattern>
    for crate::grit::bogus::bogus_pattern::FormatGritBogusPattern
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritBogusPattern,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_grit_syntax::GritBogusPattern>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritBogusPattern {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritBogusPattern,
        crate::grit::bogus::bogus_pattern::FormatGritBogusPattern,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::bogus::bogus_pattern::FormatGritBogusPattern::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritBogusPattern {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritBogusPattern,
        crate::grit::bogus::bogus_pattern::FormatGritBogusPattern,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::bogus::bogus_pattern::FormatGritBogusPattern::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritBogusPredicate>
    for crate::grit::bogus::bogus_predicate::FormatGritBogusPredicate
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritBogusPredicate,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_grit_syntax::GritBogusPredicate>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritBogusPredicate {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritBogusPredicate,
        crate::grit::bogus::bogus_predicate::FormatGritBogusPredicate,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::bogus::bogus_predicate::FormatGritBogusPredicate::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritBogusPredicate {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritBogusPredicate,
        crate::grit::bogus::bogus_predicate::FormatGritBogusPredicate,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::bogus::bogus_predicate::FormatGritBogusPredicate::default(),
        )
    }
}
impl FormatRule<biome_grit_syntax::GritBogusVersion>
    for crate::grit::bogus::bogus_version::FormatGritBogusVersion
{
    type Context = GritFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_grit_syntax::GritBogusVersion,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_grit_syntax::GritBogusVersion>::fmt(self, node, f)
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::GritBogusVersion {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::GritBogusVersion,
        crate::grit::bogus::bogus_version::FormatGritBogusVersion,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::bogus::bogus_version::FormatGritBogusVersion::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::GritBogusVersion {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::GritBogusVersion,
        crate::grit::bogus::bogus_version::FormatGritBogusVersion,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::bogus::bogus_version::FormatGritBogusVersion::default(),
        )
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::AnyGritCodeSnippetSource {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::AnyGritCodeSnippetSource,
        crate::grit::any::code_snippet_source::FormatAnyGritCodeSnippetSource,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::any::code_snippet_source::FormatAnyGritCodeSnippetSource::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::AnyGritCodeSnippetSource {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::AnyGritCodeSnippetSource,
        crate::grit::any::code_snippet_source::FormatAnyGritCodeSnippetSource,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::any::code_snippet_source::FormatAnyGritCodeSnippetSource::default(),
        )
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::AnyGritContainer {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::AnyGritContainer,
        crate::grit::any::container::FormatAnyGritContainer,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::any::container::FormatAnyGritContainer::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::AnyGritContainer {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::AnyGritContainer,
        crate::grit::any::container::FormatAnyGritContainer,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::any::container::FormatAnyGritContainer::default(),
        )
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::AnyGritDefinition {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::AnyGritDefinition,
        crate::grit::any::definition::FormatAnyGritDefinition,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::any::definition::FormatAnyGritDefinition::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::AnyGritDefinition {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::AnyGritDefinition,
        crate::grit::any::definition::FormatAnyGritDefinition,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::any::definition::FormatAnyGritDefinition::default(),
        )
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::AnyGritLanguageDeclaration {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::AnyGritLanguageDeclaration,
        crate::grit::any::language_declaration::FormatAnyGritLanguageDeclaration,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::any::language_declaration::FormatAnyGritLanguageDeclaration::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::AnyGritLanguageDeclaration {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::AnyGritLanguageDeclaration,
        crate::grit::any::language_declaration::FormatAnyGritLanguageDeclaration,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::any::language_declaration::FormatAnyGritLanguageDeclaration::default(),
        )
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::AnyGritLanguageFlavorKind {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::AnyGritLanguageFlavorKind,
        crate::grit::any::language_flavor_kind::FormatAnyGritLanguageFlavorKind,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::any::language_flavor_kind::FormatAnyGritLanguageFlavorKind::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::AnyGritLanguageFlavorKind {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::AnyGritLanguageFlavorKind,
        crate::grit::any::language_flavor_kind::FormatAnyGritLanguageFlavorKind,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::any::language_flavor_kind::FormatAnyGritLanguageFlavorKind::default(),
        )
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::AnyGritLanguageName {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::AnyGritLanguageName,
        crate::grit::any::language_name::FormatAnyGritLanguageName,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::any::language_name::FormatAnyGritLanguageName::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::AnyGritLanguageName {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::AnyGritLanguageName,
        crate::grit::any::language_name::FormatAnyGritLanguageName,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::any::language_name::FormatAnyGritLanguageName::default(),
        )
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::AnyGritListAccessorSubject {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::AnyGritListAccessorSubject,
        crate::grit::any::list_accessor_subject::FormatAnyGritListAccessorSubject,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::any::list_accessor_subject::FormatAnyGritListAccessorSubject::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::AnyGritListAccessorSubject {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::AnyGritListAccessorSubject,
        crate::grit::any::list_accessor_subject::FormatAnyGritListAccessorSubject,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::any::list_accessor_subject::FormatAnyGritListAccessorSubject::default(),
        )
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::AnyGritListIndex {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::AnyGritListIndex,
        crate::grit::any::list_index::FormatAnyGritListIndex,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::any::list_index::FormatAnyGritListIndex::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::AnyGritListIndex {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::AnyGritListIndex,
        crate::grit::any::list_index::FormatAnyGritListIndex,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::any::list_index::FormatAnyGritListIndex::default(),
        )
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::AnyGritListPattern {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::AnyGritListPattern,
        crate::grit::any::list_pattern::FormatAnyGritListPattern,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::any::list_pattern::FormatAnyGritListPattern::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::AnyGritListPattern {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::AnyGritListPattern,
        crate::grit::any::list_pattern::FormatAnyGritListPattern,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::any::list_pattern::FormatAnyGritListPattern::default(),
        )
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::AnyGritLiteral {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::AnyGritLiteral,
        crate::grit::any::literal::FormatAnyGritLiteral,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::any::literal::FormatAnyGritLiteral::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::AnyGritLiteral {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::AnyGritLiteral,
        crate::grit::any::literal::FormatAnyGritLiteral,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::any::literal::FormatAnyGritLiteral::default(),
        )
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::AnyGritMapAccessorSubject {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::AnyGritMapAccessorSubject,
        crate::grit::any::map_accessor_subject::FormatAnyGritMapAccessorSubject,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::any::map_accessor_subject::FormatAnyGritMapAccessorSubject::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::AnyGritMapAccessorSubject {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::AnyGritMapAccessorSubject,
        crate::grit::any::map_accessor_subject::FormatAnyGritMapAccessorSubject,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::any::map_accessor_subject::FormatAnyGritMapAccessorSubject::default(),
        )
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::AnyGritMapElement {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::AnyGritMapElement,
        crate::grit::any::map_element::FormatAnyGritMapElement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::any::map_element::FormatAnyGritMapElement::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::AnyGritMapElement {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::AnyGritMapElement,
        crate::grit::any::map_element::FormatAnyGritMapElement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::any::map_element::FormatAnyGritMapElement::default(),
        )
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::AnyGritMapKey {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::AnyGritMapKey,
        crate::grit::any::map_key::FormatAnyGritMapKey,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::any::map_key::FormatAnyGritMapKey::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::AnyGritMapKey {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::AnyGritMapKey,
        crate::grit::any::map_key::FormatAnyGritMapKey,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::any::map_key::FormatAnyGritMapKey::default(),
        )
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::AnyGritMaybeCurlyPattern {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::AnyGritMaybeCurlyPattern,
        crate::grit::any::maybe_curly_pattern::FormatAnyGritMaybeCurlyPattern,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::any::maybe_curly_pattern::FormatAnyGritMaybeCurlyPattern::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::AnyGritMaybeCurlyPattern {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::AnyGritMaybeCurlyPattern,
        crate::grit::any::maybe_curly_pattern::FormatAnyGritMaybeCurlyPattern,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::any::maybe_curly_pattern::FormatAnyGritMaybeCurlyPattern::default(),
        )
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::AnyGritMaybeNamedArg {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::AnyGritMaybeNamedArg,
        crate::grit::any::maybe_named_arg::FormatAnyGritMaybeNamedArg,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::any::maybe_named_arg::FormatAnyGritMaybeNamedArg::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::AnyGritMaybeNamedArg {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::AnyGritMaybeNamedArg,
        crate::grit::any::maybe_named_arg::FormatAnyGritMaybeNamedArg,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::any::maybe_named_arg::FormatAnyGritMaybeNamedArg::default(),
        )
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::AnyGritPattern {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::AnyGritPattern,
        crate::grit::any::pattern::FormatAnyGritPattern,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::any::pattern::FormatAnyGritPattern::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::AnyGritPattern {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::AnyGritPattern,
        crate::grit::any::pattern::FormatAnyGritPattern,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::any::pattern::FormatAnyGritPattern::default(),
        )
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::AnyGritPredicate {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::AnyGritPredicate,
        crate::grit::any::predicate::FormatAnyGritPredicate,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::any::predicate::FormatAnyGritPredicate::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::AnyGritPredicate {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::AnyGritPredicate,
        crate::grit::any::predicate::FormatAnyGritPredicate,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::any::predicate::FormatAnyGritPredicate::default(),
        )
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::AnyGritPredicateMatchSubject {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::AnyGritPredicateMatchSubject,
        crate::grit::any::predicate_match_subject::FormatAnyGritPredicateMatchSubject,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::any::predicate_match_subject::FormatAnyGritPredicateMatchSubject::default(
            ),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::AnyGritPredicateMatchSubject {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::AnyGritPredicateMatchSubject,
        crate::grit::any::predicate_match_subject::FormatAnyGritPredicateMatchSubject,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::any::predicate_match_subject::FormatAnyGritPredicateMatchSubject::default(
            ),
        )
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::AnyGritRegex {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::AnyGritRegex,
        crate::grit::any::regex::FormatAnyGritRegex,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, crate::grit::any::regex::FormatAnyGritRegex::default())
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::AnyGritRegex {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::AnyGritRegex,
        crate::grit::any::regex::FormatAnyGritRegex,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, crate::grit::any::regex::FormatAnyGritRegex::default())
    }
}
impl AsFormat<GritFormatContext> for biome_grit_syntax::AnyGritVersion {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_grit_syntax::AnyGritVersion,
        crate::grit::any::version::FormatAnyGritVersion,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::grit::any::version::FormatAnyGritVersion::default(),
        )
    }
}
impl IntoFormat<GritFormatContext> for biome_grit_syntax::AnyGritVersion {
    type Format = FormatOwnedWithRule<
        biome_grit_syntax::AnyGritVersion,
        crate::grit::any::version::FormatAnyGritVersion,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::grit::any::version::FormatAnyGritVersion::default(),
        )
    }
}
