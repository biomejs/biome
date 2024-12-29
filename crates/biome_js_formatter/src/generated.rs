//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

#![expect(clippy::default_constructed_unit_structs)]
use crate::{
    AsFormat, FormatBogusNodeRule, FormatNodeRule, IntoFormat, JsFormatContext, JsFormatter,
};
use biome_formatter::{FormatOwnedWithRule, FormatRefWithRule, FormatResult, FormatRule};
impl FormatRule<biome_js_syntax::JsAccessorModifier>
    for crate::js::auxiliary::accessor_modifier::FormatJsAccessorModifier
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsAccessorModifier,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsAccessorModifier>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsAccessorModifier {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsAccessorModifier,
        crate::js::auxiliary::accessor_modifier::FormatJsAccessorModifier,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::auxiliary::accessor_modifier::FormatJsAccessorModifier::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsAccessorModifier {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsAccessorModifier,
        crate::js::auxiliary::accessor_modifier::FormatJsAccessorModifier,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::auxiliary::accessor_modifier::FormatJsAccessorModifier::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsArrayAssignmentPattern>
    for crate::js::assignments::array_assignment_pattern::FormatJsArrayAssignmentPattern
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsArrayAssignmentPattern,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsArrayAssignmentPattern>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsArrayAssignmentPattern {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsArrayAssignmentPattern,
        crate::js::assignments::array_assignment_pattern::FormatJsArrayAssignmentPattern,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: js :: assignments :: array_assignment_pattern :: FormatJsArrayAssignmentPattern :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsArrayAssignmentPattern {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsArrayAssignmentPattern,
        crate::js::assignments::array_assignment_pattern::FormatJsArrayAssignmentPattern,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: js :: assignments :: array_assignment_pattern :: FormatJsArrayAssignmentPattern :: default ())
    }
}
impl FormatRule < biome_js_syntax :: JsArrayAssignmentPatternElement > for crate :: js :: assignments :: array_assignment_pattern_element :: FormatJsArrayAssignmentPatternElement { type Context = JsFormatContext ; # [inline (always)] fn fmt (& self , node : & biome_js_syntax :: JsArrayAssignmentPatternElement , f : & mut JsFormatter) -> FormatResult < () > { FormatNodeRule :: < biome_js_syntax :: JsArrayAssignmentPatternElement > :: fmt (self , node , f) } }
impl AsFormat<JsFormatContext> for biome_js_syntax::JsArrayAssignmentPatternElement {
    type Format < 'a > = FormatRefWithRule < 'a , biome_js_syntax :: JsArrayAssignmentPatternElement , crate :: js :: assignments :: array_assignment_pattern_element :: FormatJsArrayAssignmentPatternElement > ;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: js :: assignments :: array_assignment_pattern_element :: FormatJsArrayAssignmentPatternElement :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsArrayAssignmentPatternElement {
    type Format = FormatOwnedWithRule < biome_js_syntax :: JsArrayAssignmentPatternElement , crate :: js :: assignments :: array_assignment_pattern_element :: FormatJsArrayAssignmentPatternElement > ;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: js :: assignments :: array_assignment_pattern_element :: FormatJsArrayAssignmentPatternElement :: default ())
    }
}
impl FormatRule < biome_js_syntax :: JsArrayAssignmentPatternRestElement > for crate :: js :: assignments :: array_assignment_pattern_rest_element :: FormatJsArrayAssignmentPatternRestElement { type Context = JsFormatContext ; # [inline (always)] fn fmt (& self , node : & biome_js_syntax :: JsArrayAssignmentPatternRestElement , f : & mut JsFormatter) -> FormatResult < () > { FormatNodeRule :: < biome_js_syntax :: JsArrayAssignmentPatternRestElement > :: fmt (self , node , f) } }
impl AsFormat<JsFormatContext> for biome_js_syntax::JsArrayAssignmentPatternRestElement {
    type Format < 'a > = FormatRefWithRule < 'a , biome_js_syntax :: JsArrayAssignmentPatternRestElement , crate :: js :: assignments :: array_assignment_pattern_rest_element :: FormatJsArrayAssignmentPatternRestElement > ;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: js :: assignments :: array_assignment_pattern_rest_element :: FormatJsArrayAssignmentPatternRestElement :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsArrayAssignmentPatternRestElement {
    type Format = FormatOwnedWithRule < biome_js_syntax :: JsArrayAssignmentPatternRestElement , crate :: js :: assignments :: array_assignment_pattern_rest_element :: FormatJsArrayAssignmentPatternRestElement > ;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: js :: assignments :: array_assignment_pattern_rest_element :: FormatJsArrayAssignmentPatternRestElement :: default ())
    }
}
impl FormatRule<biome_js_syntax::JsArrayBindingPattern>
    for crate::js::bindings::array_binding_pattern::FormatJsArrayBindingPattern
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsArrayBindingPattern,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsArrayBindingPattern>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsArrayBindingPattern {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsArrayBindingPattern,
        crate::js::bindings::array_binding_pattern::FormatJsArrayBindingPattern,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::bindings::array_binding_pattern::FormatJsArrayBindingPattern::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsArrayBindingPattern {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsArrayBindingPattern,
        crate::js::bindings::array_binding_pattern::FormatJsArrayBindingPattern,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::bindings::array_binding_pattern::FormatJsArrayBindingPattern::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsArrayBindingPatternElement>
    for crate::js::bindings::array_binding_pattern_element::FormatJsArrayBindingPatternElement
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsArrayBindingPatternElement,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsArrayBindingPatternElement>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsArrayBindingPatternElement {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsArrayBindingPatternElement,
        crate::js::bindings::array_binding_pattern_element::FormatJsArrayBindingPatternElement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: js :: bindings :: array_binding_pattern_element :: FormatJsArrayBindingPatternElement :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsArrayBindingPatternElement {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsArrayBindingPatternElement,
        crate::js::bindings::array_binding_pattern_element::FormatJsArrayBindingPatternElement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: js :: bindings :: array_binding_pattern_element :: FormatJsArrayBindingPatternElement :: default ())
    }
}
impl FormatRule < biome_js_syntax :: JsArrayBindingPatternRestElement > for crate :: js :: bindings :: array_binding_pattern_rest_element :: FormatJsArrayBindingPatternRestElement { type Context = JsFormatContext ; # [inline (always)] fn fmt (& self , node : & biome_js_syntax :: JsArrayBindingPatternRestElement , f : & mut JsFormatter) -> FormatResult < () > { FormatNodeRule :: < biome_js_syntax :: JsArrayBindingPatternRestElement > :: fmt (self , node , f) } }
impl AsFormat<JsFormatContext> for biome_js_syntax::JsArrayBindingPatternRestElement {
    type Format < 'a > = FormatRefWithRule < 'a , biome_js_syntax :: JsArrayBindingPatternRestElement , crate :: js :: bindings :: array_binding_pattern_rest_element :: FormatJsArrayBindingPatternRestElement > ;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: js :: bindings :: array_binding_pattern_rest_element :: FormatJsArrayBindingPatternRestElement :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsArrayBindingPatternRestElement {
    type Format = FormatOwnedWithRule < biome_js_syntax :: JsArrayBindingPatternRestElement , crate :: js :: bindings :: array_binding_pattern_rest_element :: FormatJsArrayBindingPatternRestElement > ;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: js :: bindings :: array_binding_pattern_rest_element :: FormatJsArrayBindingPatternRestElement :: default ())
    }
}
impl FormatRule<biome_js_syntax::JsArrayExpression>
    for crate::js::expressions::array_expression::FormatJsArrayExpression
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsArrayExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsArrayExpression>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsArrayExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsArrayExpression,
        crate::js::expressions::array_expression::FormatJsArrayExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::expressions::array_expression::FormatJsArrayExpression::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsArrayExpression {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsArrayExpression,
        crate::js::expressions::array_expression::FormatJsArrayExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::expressions::array_expression::FormatJsArrayExpression::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsArrayHole>
    for crate::js::auxiliary::array_hole::FormatJsArrayHole
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::JsArrayHole, f: &mut JsFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsArrayHole>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsArrayHole {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsArrayHole,
        crate::js::auxiliary::array_hole::FormatJsArrayHole,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::auxiliary::array_hole::FormatJsArrayHole::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsArrayHole {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsArrayHole,
        crate::js::auxiliary::array_hole::FormatJsArrayHole,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::auxiliary::array_hole::FormatJsArrayHole::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsArrowFunctionExpression>
    for crate::js::expressions::arrow_function_expression::FormatJsArrowFunctionExpression
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsArrowFunctionExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsArrowFunctionExpression>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsArrowFunctionExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsArrowFunctionExpression,
        crate::js::expressions::arrow_function_expression::FormatJsArrowFunctionExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: js :: expressions :: arrow_function_expression :: FormatJsArrowFunctionExpression :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsArrowFunctionExpression {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsArrowFunctionExpression,
        crate::js::expressions::arrow_function_expression::FormatJsArrowFunctionExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: js :: expressions :: arrow_function_expression :: FormatJsArrowFunctionExpression :: default ())
    }
}
impl FormatRule<biome_js_syntax::JsAssignmentExpression>
    for crate::js::expressions::assignment_expression::FormatJsAssignmentExpression
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsAssignmentExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsAssignmentExpression>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsAssignmentExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsAssignmentExpression,
        crate::js::expressions::assignment_expression::FormatJsAssignmentExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::expressions::assignment_expression::FormatJsAssignmentExpression::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsAssignmentExpression {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsAssignmentExpression,
        crate::js::expressions::assignment_expression::FormatJsAssignmentExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::expressions::assignment_expression::FormatJsAssignmentExpression::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsAwaitExpression>
    for crate::js::expressions::await_expression::FormatJsAwaitExpression
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsAwaitExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsAwaitExpression>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsAwaitExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsAwaitExpression,
        crate::js::expressions::await_expression::FormatJsAwaitExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::expressions::await_expression::FormatJsAwaitExpression::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsAwaitExpression {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsAwaitExpression,
        crate::js::expressions::await_expression::FormatJsAwaitExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::expressions::await_expression::FormatJsAwaitExpression::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsBigintLiteralExpression>
    for crate::js::expressions::bigint_literal_expression::FormatJsBigintLiteralExpression
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsBigintLiteralExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsBigintLiteralExpression>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsBigintLiteralExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsBigintLiteralExpression,
        crate::js::expressions::bigint_literal_expression::FormatJsBigintLiteralExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: js :: expressions :: bigint_literal_expression :: FormatJsBigintLiteralExpression :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsBigintLiteralExpression {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsBigintLiteralExpression,
        crate::js::expressions::bigint_literal_expression::FormatJsBigintLiteralExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: js :: expressions :: bigint_literal_expression :: FormatJsBigintLiteralExpression :: default ())
    }
}
impl FormatRule<biome_js_syntax::JsBinaryExpression>
    for crate::js::expressions::binary_expression::FormatJsBinaryExpression
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsBinaryExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsBinaryExpression>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsBinaryExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsBinaryExpression,
        crate::js::expressions::binary_expression::FormatJsBinaryExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::expressions::binary_expression::FormatJsBinaryExpression::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsBinaryExpression {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsBinaryExpression,
        crate::js::expressions::binary_expression::FormatJsBinaryExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::expressions::binary_expression::FormatJsBinaryExpression::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsBlockStatement>
    for crate::js::statements::block_statement::FormatJsBlockStatement
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsBlockStatement,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsBlockStatement>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsBlockStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsBlockStatement,
        crate::js::statements::block_statement::FormatJsBlockStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::statements::block_statement::FormatJsBlockStatement::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsBlockStatement {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsBlockStatement,
        crate::js::statements::block_statement::FormatJsBlockStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::statements::block_statement::FormatJsBlockStatement::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsBooleanLiteralExpression>
    for crate::js::expressions::boolean_literal_expression::FormatJsBooleanLiteralExpression
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsBooleanLiteralExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsBooleanLiteralExpression>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsBooleanLiteralExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsBooleanLiteralExpression,
        crate::js::expressions::boolean_literal_expression::FormatJsBooleanLiteralExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: js :: expressions :: boolean_literal_expression :: FormatJsBooleanLiteralExpression :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsBooleanLiteralExpression {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsBooleanLiteralExpression,
        crate::js::expressions::boolean_literal_expression::FormatJsBooleanLiteralExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: js :: expressions :: boolean_literal_expression :: FormatJsBooleanLiteralExpression :: default ())
    }
}
impl FormatRule<biome_js_syntax::JsBreakStatement>
    for crate::js::statements::break_statement::FormatJsBreakStatement
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsBreakStatement,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsBreakStatement>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsBreakStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsBreakStatement,
        crate::js::statements::break_statement::FormatJsBreakStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::statements::break_statement::FormatJsBreakStatement::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsBreakStatement {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsBreakStatement,
        crate::js::statements::break_statement::FormatJsBreakStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::statements::break_statement::FormatJsBreakStatement::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsCallArguments>
    for crate::js::expressions::call_arguments::FormatJsCallArguments
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsCallArguments,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsCallArguments>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsCallArguments {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsCallArguments,
        crate::js::expressions::call_arguments::FormatJsCallArguments,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::expressions::call_arguments::FormatJsCallArguments::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsCallArguments {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsCallArguments,
        crate::js::expressions::call_arguments::FormatJsCallArguments,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::expressions::call_arguments::FormatJsCallArguments::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsCallExpression>
    for crate::js::expressions::call_expression::FormatJsCallExpression
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsCallExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsCallExpression>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsCallExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsCallExpression,
        crate::js::expressions::call_expression::FormatJsCallExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::expressions::call_expression::FormatJsCallExpression::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsCallExpression {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsCallExpression,
        crate::js::expressions::call_expression::FormatJsCallExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::expressions::call_expression::FormatJsCallExpression::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsCaseClause>
    for crate::js::auxiliary::case_clause::FormatJsCaseClause
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::JsCaseClause, f: &mut JsFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsCaseClause>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsCaseClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsCaseClause,
        crate::js::auxiliary::case_clause::FormatJsCaseClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::auxiliary::case_clause::FormatJsCaseClause::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsCaseClause {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsCaseClause,
        crate::js::auxiliary::case_clause::FormatJsCaseClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::auxiliary::case_clause::FormatJsCaseClause::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsCatchClause>
    for crate::js::auxiliary::catch_clause::FormatJsCatchClause
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::JsCatchClause, f: &mut JsFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsCatchClause>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsCatchClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsCatchClause,
        crate::js::auxiliary::catch_clause::FormatJsCatchClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::auxiliary::catch_clause::FormatJsCatchClause::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsCatchClause {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsCatchClause,
        crate::js::auxiliary::catch_clause::FormatJsCatchClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::auxiliary::catch_clause::FormatJsCatchClause::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsCatchDeclaration>
    for crate::js::declarations::catch_declaration::FormatJsCatchDeclaration
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsCatchDeclaration,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsCatchDeclaration>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsCatchDeclaration {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsCatchDeclaration,
        crate::js::declarations::catch_declaration::FormatJsCatchDeclaration,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::declarations::catch_declaration::FormatJsCatchDeclaration::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsCatchDeclaration {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsCatchDeclaration,
        crate::js::declarations::catch_declaration::FormatJsCatchDeclaration,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::declarations::catch_declaration::FormatJsCatchDeclaration::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsClassDeclaration>
    for crate::js::declarations::class_declaration::FormatJsClassDeclaration
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsClassDeclaration,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsClassDeclaration>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsClassDeclaration {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsClassDeclaration,
        crate::js::declarations::class_declaration::FormatJsClassDeclaration,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::declarations::class_declaration::FormatJsClassDeclaration::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsClassDeclaration {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsClassDeclaration,
        crate::js::declarations::class_declaration::FormatJsClassDeclaration,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::declarations::class_declaration::FormatJsClassDeclaration::default(),
        )
    }
}
impl FormatRule < biome_js_syntax :: JsClassExportDefaultDeclaration > for crate :: js :: declarations :: class_export_default_declaration :: FormatJsClassExportDefaultDeclaration { type Context = JsFormatContext ; # [inline (always)] fn fmt (& self , node : & biome_js_syntax :: JsClassExportDefaultDeclaration , f : & mut JsFormatter) -> FormatResult < () > { FormatNodeRule :: < biome_js_syntax :: JsClassExportDefaultDeclaration > :: fmt (self , node , f) } }
impl AsFormat<JsFormatContext> for biome_js_syntax::JsClassExportDefaultDeclaration {
    type Format < 'a > = FormatRefWithRule < 'a , biome_js_syntax :: JsClassExportDefaultDeclaration , crate :: js :: declarations :: class_export_default_declaration :: FormatJsClassExportDefaultDeclaration > ;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: js :: declarations :: class_export_default_declaration :: FormatJsClassExportDefaultDeclaration :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsClassExportDefaultDeclaration {
    type Format = FormatOwnedWithRule < biome_js_syntax :: JsClassExportDefaultDeclaration , crate :: js :: declarations :: class_export_default_declaration :: FormatJsClassExportDefaultDeclaration > ;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: js :: declarations :: class_export_default_declaration :: FormatJsClassExportDefaultDeclaration :: default ())
    }
}
impl FormatRule<biome_js_syntax::JsClassExpression>
    for crate::js::expressions::class_expression::FormatJsClassExpression
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsClassExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsClassExpression>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsClassExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsClassExpression,
        crate::js::expressions::class_expression::FormatJsClassExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::expressions::class_expression::FormatJsClassExpression::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsClassExpression {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsClassExpression,
        crate::js::expressions::class_expression::FormatJsClassExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::expressions::class_expression::FormatJsClassExpression::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsComputedMemberAssignment>
    for crate::js::assignments::computed_member_assignment::FormatJsComputedMemberAssignment
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsComputedMemberAssignment,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsComputedMemberAssignment>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsComputedMemberAssignment {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsComputedMemberAssignment,
        crate::js::assignments::computed_member_assignment::FormatJsComputedMemberAssignment,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: js :: assignments :: computed_member_assignment :: FormatJsComputedMemberAssignment :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsComputedMemberAssignment {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsComputedMemberAssignment,
        crate::js::assignments::computed_member_assignment::FormatJsComputedMemberAssignment,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: js :: assignments :: computed_member_assignment :: FormatJsComputedMemberAssignment :: default ())
    }
}
impl FormatRule<biome_js_syntax::JsComputedMemberExpression>
    for crate::js::expressions::computed_member_expression::FormatJsComputedMemberExpression
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsComputedMemberExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsComputedMemberExpression>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsComputedMemberExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsComputedMemberExpression,
        crate::js::expressions::computed_member_expression::FormatJsComputedMemberExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: js :: expressions :: computed_member_expression :: FormatJsComputedMemberExpression :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsComputedMemberExpression {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsComputedMemberExpression,
        crate::js::expressions::computed_member_expression::FormatJsComputedMemberExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: js :: expressions :: computed_member_expression :: FormatJsComputedMemberExpression :: default ())
    }
}
impl FormatRule<biome_js_syntax::JsComputedMemberName>
    for crate::js::objects::computed_member_name::FormatJsComputedMemberName
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsComputedMemberName,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsComputedMemberName>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsComputedMemberName {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsComputedMemberName,
        crate::js::objects::computed_member_name::FormatJsComputedMemberName,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::objects::computed_member_name::FormatJsComputedMemberName::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsComputedMemberName {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsComputedMemberName,
        crate::js::objects::computed_member_name::FormatJsComputedMemberName,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::objects::computed_member_name::FormatJsComputedMemberName::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsConditionalExpression>
    for crate::js::expressions::conditional_expression::FormatJsConditionalExpression
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsConditionalExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsConditionalExpression>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsConditionalExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsConditionalExpression,
        crate::js::expressions::conditional_expression::FormatJsConditionalExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::expressions::conditional_expression::FormatJsConditionalExpression::default(
            ),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsConditionalExpression {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsConditionalExpression,
        crate::js::expressions::conditional_expression::FormatJsConditionalExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::expressions::conditional_expression::FormatJsConditionalExpression::default(
            ),
        )
    }
}
impl FormatRule<biome_js_syntax::JsConstructorClassMember>
    for crate::js::classes::constructor_class_member::FormatJsConstructorClassMember
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsConstructorClassMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsConstructorClassMember>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsConstructorClassMember {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsConstructorClassMember,
        crate::js::classes::constructor_class_member::FormatJsConstructorClassMember,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::classes::constructor_class_member::FormatJsConstructorClassMember::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsConstructorClassMember {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsConstructorClassMember,
        crate::js::classes::constructor_class_member::FormatJsConstructorClassMember,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::classes::constructor_class_member::FormatJsConstructorClassMember::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsConstructorParameters>
    for crate::js::bindings::constructor_parameters::FormatJsConstructorParameters
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsConstructorParameters,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsConstructorParameters>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsConstructorParameters {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsConstructorParameters,
        crate::js::bindings::constructor_parameters::FormatJsConstructorParameters,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::bindings::constructor_parameters::FormatJsConstructorParameters::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsConstructorParameters {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsConstructorParameters,
        crate::js::bindings::constructor_parameters::FormatJsConstructorParameters,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::bindings::constructor_parameters::FormatJsConstructorParameters::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsContinueStatement>
    for crate::js::statements::continue_statement::FormatJsContinueStatement
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsContinueStatement,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsContinueStatement>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsContinueStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsContinueStatement,
        crate::js::statements::continue_statement::FormatJsContinueStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::statements::continue_statement::FormatJsContinueStatement::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsContinueStatement {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsContinueStatement,
        crate::js::statements::continue_statement::FormatJsContinueStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::statements::continue_statement::FormatJsContinueStatement::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsDebuggerStatement>
    for crate::js::statements::debugger_statement::FormatJsDebuggerStatement
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsDebuggerStatement,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsDebuggerStatement>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsDebuggerStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsDebuggerStatement,
        crate::js::statements::debugger_statement::FormatJsDebuggerStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::statements::debugger_statement::FormatJsDebuggerStatement::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsDebuggerStatement {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsDebuggerStatement,
        crate::js::statements::debugger_statement::FormatJsDebuggerStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::statements::debugger_statement::FormatJsDebuggerStatement::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsDecorator>
    for crate::js::auxiliary::decorator::FormatJsDecorator
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::JsDecorator, f: &mut JsFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsDecorator>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsDecorator {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsDecorator,
        crate::js::auxiliary::decorator::FormatJsDecorator,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::auxiliary::decorator::FormatJsDecorator::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsDecorator {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsDecorator,
        crate::js::auxiliary::decorator::FormatJsDecorator,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::auxiliary::decorator::FormatJsDecorator::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsDefaultClause>
    for crate::js::auxiliary::default_clause::FormatJsDefaultClause
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsDefaultClause,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsDefaultClause>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsDefaultClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsDefaultClause,
        crate::js::auxiliary::default_clause::FormatJsDefaultClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::auxiliary::default_clause::FormatJsDefaultClause::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsDefaultClause {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsDefaultClause,
        crate::js::auxiliary::default_clause::FormatJsDefaultClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::auxiliary::default_clause::FormatJsDefaultClause::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsDefaultImportSpecifier>
    for crate::js::module::default_import_specifier::FormatJsDefaultImportSpecifier
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsDefaultImportSpecifier,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsDefaultImportSpecifier>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsDefaultImportSpecifier {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsDefaultImportSpecifier,
        crate::js::module::default_import_specifier::FormatJsDefaultImportSpecifier,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::module::default_import_specifier::FormatJsDefaultImportSpecifier::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsDefaultImportSpecifier {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsDefaultImportSpecifier,
        crate::js::module::default_import_specifier::FormatJsDefaultImportSpecifier,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::module::default_import_specifier::FormatJsDefaultImportSpecifier::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsDirective>
    for crate::js::auxiliary::directive::FormatJsDirective
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::JsDirective, f: &mut JsFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsDirective>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsDirective {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsDirective,
        crate::js::auxiliary::directive::FormatJsDirective,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::auxiliary::directive::FormatJsDirective::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsDirective {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsDirective,
        crate::js::auxiliary::directive::FormatJsDirective,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::auxiliary::directive::FormatJsDirective::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsDoWhileStatement>
    for crate::js::statements::do_while_statement::FormatJsDoWhileStatement
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsDoWhileStatement,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsDoWhileStatement>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsDoWhileStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsDoWhileStatement,
        crate::js::statements::do_while_statement::FormatJsDoWhileStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::statements::do_while_statement::FormatJsDoWhileStatement::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsDoWhileStatement {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsDoWhileStatement,
        crate::js::statements::do_while_statement::FormatJsDoWhileStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::statements::do_while_statement::FormatJsDoWhileStatement::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsElseClause>
    for crate::js::auxiliary::else_clause::FormatJsElseClause
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::JsElseClause, f: &mut JsFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsElseClause>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsElseClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsElseClause,
        crate::js::auxiliary::else_clause::FormatJsElseClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::auxiliary::else_clause::FormatJsElseClause::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsElseClause {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsElseClause,
        crate::js::auxiliary::else_clause::FormatJsElseClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::auxiliary::else_clause::FormatJsElseClause::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsEmptyClassMember>
    for crate::js::classes::empty_class_member::FormatJsEmptyClassMember
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsEmptyClassMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsEmptyClassMember>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsEmptyClassMember {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsEmptyClassMember,
        crate::js::classes::empty_class_member::FormatJsEmptyClassMember,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::classes::empty_class_member::FormatJsEmptyClassMember::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsEmptyClassMember {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsEmptyClassMember,
        crate::js::classes::empty_class_member::FormatJsEmptyClassMember,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::classes::empty_class_member::FormatJsEmptyClassMember::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsEmptyStatement>
    for crate::js::statements::empty_statement::FormatJsEmptyStatement
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsEmptyStatement,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsEmptyStatement>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsEmptyStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsEmptyStatement,
        crate::js::statements::empty_statement::FormatJsEmptyStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::statements::empty_statement::FormatJsEmptyStatement::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsEmptyStatement {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsEmptyStatement,
        crate::js::statements::empty_statement::FormatJsEmptyStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::statements::empty_statement::FormatJsEmptyStatement::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsExport> for crate::js::module::export::FormatJsExport {
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::JsExport, f: &mut JsFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsExport>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsExport {
    type Format<'a> =
        FormatRefWithRule<'a, biome_js_syntax::JsExport, crate::js::module::export::FormatJsExport>;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, crate::js::module::export::FormatJsExport::default())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsExport {
    type Format =
        FormatOwnedWithRule<biome_js_syntax::JsExport, crate::js::module::export::FormatJsExport>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, crate::js::module::export::FormatJsExport::default())
    }
}
impl FormatRule<biome_js_syntax::JsExportAsClause>
    for crate::js::module::export_as_clause::FormatJsExportAsClause
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsExportAsClause,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsExportAsClause>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsExportAsClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsExportAsClause,
        crate::js::module::export_as_clause::FormatJsExportAsClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::module::export_as_clause::FormatJsExportAsClause::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsExportAsClause {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsExportAsClause,
        crate::js::module::export_as_clause::FormatJsExportAsClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::module::export_as_clause::FormatJsExportAsClause::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsExportDefaultDeclarationClause>
    for crate::js::module::export_default_declaration_clause::FormatJsExportDefaultDeclarationClause
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsExportDefaultDeclarationClause,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsExportDefaultDeclarationClause>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsExportDefaultDeclarationClause {
    type Format < 'a > = FormatRefWithRule < 'a , biome_js_syntax :: JsExportDefaultDeclarationClause , crate :: js :: module :: export_default_declaration_clause :: FormatJsExportDefaultDeclarationClause > ;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: js :: module :: export_default_declaration_clause :: FormatJsExportDefaultDeclarationClause :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsExportDefaultDeclarationClause {
    type Format = FormatOwnedWithRule < biome_js_syntax :: JsExportDefaultDeclarationClause , crate :: js :: module :: export_default_declaration_clause :: FormatJsExportDefaultDeclarationClause > ;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: js :: module :: export_default_declaration_clause :: FormatJsExportDefaultDeclarationClause :: default ())
    }
}
impl FormatRule<biome_js_syntax::JsExportDefaultExpressionClause>
    for crate::js::module::export_default_expression_clause::FormatJsExportDefaultExpressionClause
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsExportDefaultExpressionClause,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsExportDefaultExpressionClause>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsExportDefaultExpressionClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsExportDefaultExpressionClause,
        crate::js::module::export_default_expression_clause::FormatJsExportDefaultExpressionClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: js :: module :: export_default_expression_clause :: FormatJsExportDefaultExpressionClause :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsExportDefaultExpressionClause {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsExportDefaultExpressionClause,
        crate::js::module::export_default_expression_clause::FormatJsExportDefaultExpressionClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: js :: module :: export_default_expression_clause :: FormatJsExportDefaultExpressionClause :: default ())
    }
}
impl FormatRule<biome_js_syntax::JsExportFromClause>
    for crate::js::module::export_from_clause::FormatJsExportFromClause
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsExportFromClause,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsExportFromClause>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsExportFromClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsExportFromClause,
        crate::js::module::export_from_clause::FormatJsExportFromClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::module::export_from_clause::FormatJsExportFromClause::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsExportFromClause {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsExportFromClause,
        crate::js::module::export_from_clause::FormatJsExportFromClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::module::export_from_clause::FormatJsExportFromClause::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsExportNamedClause>
    for crate::js::module::export_named_clause::FormatJsExportNamedClause
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsExportNamedClause,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsExportNamedClause>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsExportNamedClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsExportNamedClause,
        crate::js::module::export_named_clause::FormatJsExportNamedClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::module::export_named_clause::FormatJsExportNamedClause::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsExportNamedClause {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsExportNamedClause,
        crate::js::module::export_named_clause::FormatJsExportNamedClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::module::export_named_clause::FormatJsExportNamedClause::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsExportNamedFromClause>
    for crate::js::module::export_named_from_clause::FormatJsExportNamedFromClause
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsExportNamedFromClause,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsExportNamedFromClause>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsExportNamedFromClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsExportNamedFromClause,
        crate::js::module::export_named_from_clause::FormatJsExportNamedFromClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::module::export_named_from_clause::FormatJsExportNamedFromClause::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsExportNamedFromClause {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsExportNamedFromClause,
        crate::js::module::export_named_from_clause::FormatJsExportNamedFromClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::module::export_named_from_clause::FormatJsExportNamedFromClause::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsExportNamedFromSpecifier>
    for crate::js::module::export_named_from_specifier::FormatJsExportNamedFromSpecifier
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsExportNamedFromSpecifier,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsExportNamedFromSpecifier>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsExportNamedFromSpecifier {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsExportNamedFromSpecifier,
        crate::js::module::export_named_from_specifier::FormatJsExportNamedFromSpecifier,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: js :: module :: export_named_from_specifier :: FormatJsExportNamedFromSpecifier :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsExportNamedFromSpecifier {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsExportNamedFromSpecifier,
        crate::js::module::export_named_from_specifier::FormatJsExportNamedFromSpecifier,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: js :: module :: export_named_from_specifier :: FormatJsExportNamedFromSpecifier :: default ())
    }
}
impl FormatRule<biome_js_syntax::JsExportNamedShorthandSpecifier>
    for crate::js::module::export_named_shorthand_specifier::FormatJsExportNamedShorthandSpecifier
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsExportNamedShorthandSpecifier,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsExportNamedShorthandSpecifier>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsExportNamedShorthandSpecifier {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsExportNamedShorthandSpecifier,
        crate::js::module::export_named_shorthand_specifier::FormatJsExportNamedShorthandSpecifier,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: js :: module :: export_named_shorthand_specifier :: FormatJsExportNamedShorthandSpecifier :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsExportNamedShorthandSpecifier {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsExportNamedShorthandSpecifier,
        crate::js::module::export_named_shorthand_specifier::FormatJsExportNamedShorthandSpecifier,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: js :: module :: export_named_shorthand_specifier :: FormatJsExportNamedShorthandSpecifier :: default ())
    }
}
impl FormatRule<biome_js_syntax::JsExportNamedSpecifier>
    for crate::js::module::export_named_specifier::FormatJsExportNamedSpecifier
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsExportNamedSpecifier,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsExportNamedSpecifier>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsExportNamedSpecifier {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsExportNamedSpecifier,
        crate::js::module::export_named_specifier::FormatJsExportNamedSpecifier,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::module::export_named_specifier::FormatJsExportNamedSpecifier::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsExportNamedSpecifier {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsExportNamedSpecifier,
        crate::js::module::export_named_specifier::FormatJsExportNamedSpecifier,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::module::export_named_specifier::FormatJsExportNamedSpecifier::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsExpressionSnipped>
    for crate::js::auxiliary::expression_snipped::FormatJsExpressionSnipped
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsExpressionSnipped,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsExpressionSnipped>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsExpressionSnipped {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsExpressionSnipped,
        crate::js::auxiliary::expression_snipped::FormatJsExpressionSnipped,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::auxiliary::expression_snipped::FormatJsExpressionSnipped::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsExpressionSnipped {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsExpressionSnipped,
        crate::js::auxiliary::expression_snipped::FormatJsExpressionSnipped,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::auxiliary::expression_snipped::FormatJsExpressionSnipped::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsExpressionStatement>
    for crate::js::statements::expression_statement::FormatJsExpressionStatement
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsExpressionStatement,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsExpressionStatement>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsExpressionStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsExpressionStatement,
        crate::js::statements::expression_statement::FormatJsExpressionStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::statements::expression_statement::FormatJsExpressionStatement::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsExpressionStatement {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsExpressionStatement,
        crate::js::statements::expression_statement::FormatJsExpressionStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::statements::expression_statement::FormatJsExpressionStatement::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsExtendsClause>
    for crate::js::classes::extends_clause::FormatJsExtendsClause
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsExtendsClause,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsExtendsClause>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsExtendsClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsExtendsClause,
        crate::js::classes::extends_clause::FormatJsExtendsClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::classes::extends_clause::FormatJsExtendsClause::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsExtendsClause {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsExtendsClause,
        crate::js::classes::extends_clause::FormatJsExtendsClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::classes::extends_clause::FormatJsExtendsClause::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsFinallyClause>
    for crate::js::auxiliary::finally_clause::FormatJsFinallyClause
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsFinallyClause,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsFinallyClause>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsFinallyClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsFinallyClause,
        crate::js::auxiliary::finally_clause::FormatJsFinallyClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::auxiliary::finally_clause::FormatJsFinallyClause::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsFinallyClause {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsFinallyClause,
        crate::js::auxiliary::finally_clause::FormatJsFinallyClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::auxiliary::finally_clause::FormatJsFinallyClause::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsForInStatement>
    for crate::js::statements::for_in_statement::FormatJsForInStatement
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsForInStatement,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsForInStatement>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsForInStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsForInStatement,
        crate::js::statements::for_in_statement::FormatJsForInStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::statements::for_in_statement::FormatJsForInStatement::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsForInStatement {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsForInStatement,
        crate::js::statements::for_in_statement::FormatJsForInStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::statements::for_in_statement::FormatJsForInStatement::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsForOfStatement>
    for crate::js::statements::for_of_statement::FormatJsForOfStatement
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsForOfStatement,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsForOfStatement>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsForOfStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsForOfStatement,
        crate::js::statements::for_of_statement::FormatJsForOfStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::statements::for_of_statement::FormatJsForOfStatement::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsForOfStatement {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsForOfStatement,
        crate::js::statements::for_of_statement::FormatJsForOfStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::statements::for_of_statement::FormatJsForOfStatement::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsForStatement>
    for crate::js::statements::for_statement::FormatJsForStatement
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::JsForStatement, f: &mut JsFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsForStatement>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsForStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsForStatement,
        crate::js::statements::for_statement::FormatJsForStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::statements::for_statement::FormatJsForStatement::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsForStatement {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsForStatement,
        crate::js::statements::for_statement::FormatJsForStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::statements::for_statement::FormatJsForStatement::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsForVariableDeclaration>
    for crate::js::declarations::for_variable_declaration::FormatJsForVariableDeclaration
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsForVariableDeclaration,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsForVariableDeclaration>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsForVariableDeclaration {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsForVariableDeclaration,
        crate::js::declarations::for_variable_declaration::FormatJsForVariableDeclaration,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: js :: declarations :: for_variable_declaration :: FormatJsForVariableDeclaration :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsForVariableDeclaration {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsForVariableDeclaration,
        crate::js::declarations::for_variable_declaration::FormatJsForVariableDeclaration,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: js :: declarations :: for_variable_declaration :: FormatJsForVariableDeclaration :: default ())
    }
}
impl FormatRule<biome_js_syntax::JsFormalParameter>
    for crate::js::bindings::formal_parameter::FormatJsFormalParameter
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsFormalParameter,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsFormalParameter>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsFormalParameter {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsFormalParameter,
        crate::js::bindings::formal_parameter::FormatJsFormalParameter,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::bindings::formal_parameter::FormatJsFormalParameter::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsFormalParameter {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsFormalParameter,
        crate::js::bindings::formal_parameter::FormatJsFormalParameter,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::bindings::formal_parameter::FormatJsFormalParameter::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsFunctionBody>
    for crate::js::auxiliary::function_body::FormatJsFunctionBody
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::JsFunctionBody, f: &mut JsFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsFunctionBody>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsFunctionBody {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsFunctionBody,
        crate::js::auxiliary::function_body::FormatJsFunctionBody,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::auxiliary::function_body::FormatJsFunctionBody::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsFunctionBody {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsFunctionBody,
        crate::js::auxiliary::function_body::FormatJsFunctionBody,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::auxiliary::function_body::FormatJsFunctionBody::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsFunctionDeclaration>
    for crate::js::declarations::function_declaration::FormatJsFunctionDeclaration
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsFunctionDeclaration,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsFunctionDeclaration>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsFunctionDeclaration {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsFunctionDeclaration,
        crate::js::declarations::function_declaration::FormatJsFunctionDeclaration,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::declarations::function_declaration::FormatJsFunctionDeclaration::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsFunctionDeclaration {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsFunctionDeclaration,
        crate::js::declarations::function_declaration::FormatJsFunctionDeclaration,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::declarations::function_declaration::FormatJsFunctionDeclaration::default(),
        )
    }
}
impl FormatRule < biome_js_syntax :: JsFunctionExportDefaultDeclaration > for crate :: js :: declarations :: function_export_default_declaration :: FormatJsFunctionExportDefaultDeclaration { type Context = JsFormatContext ; # [inline (always)] fn fmt (& self , node : & biome_js_syntax :: JsFunctionExportDefaultDeclaration , f : & mut JsFormatter) -> FormatResult < () > { FormatNodeRule :: < biome_js_syntax :: JsFunctionExportDefaultDeclaration > :: fmt (self , node , f) } }
impl AsFormat<JsFormatContext> for biome_js_syntax::JsFunctionExportDefaultDeclaration {
    type Format < 'a > = FormatRefWithRule < 'a , biome_js_syntax :: JsFunctionExportDefaultDeclaration , crate :: js :: declarations :: function_export_default_declaration :: FormatJsFunctionExportDefaultDeclaration > ;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: js :: declarations :: function_export_default_declaration :: FormatJsFunctionExportDefaultDeclaration :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsFunctionExportDefaultDeclaration {
    type Format = FormatOwnedWithRule < biome_js_syntax :: JsFunctionExportDefaultDeclaration , crate :: js :: declarations :: function_export_default_declaration :: FormatJsFunctionExportDefaultDeclaration > ;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: js :: declarations :: function_export_default_declaration :: FormatJsFunctionExportDefaultDeclaration :: default ())
    }
}
impl FormatRule<biome_js_syntax::JsFunctionExpression>
    for crate::js::expressions::function_expression::FormatJsFunctionExpression
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsFunctionExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsFunctionExpression>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsFunctionExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsFunctionExpression,
        crate::js::expressions::function_expression::FormatJsFunctionExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::expressions::function_expression::FormatJsFunctionExpression::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsFunctionExpression {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsFunctionExpression,
        crate::js::expressions::function_expression::FormatJsFunctionExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::expressions::function_expression::FormatJsFunctionExpression::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsGetterClassMember>
    for crate::js::classes::getter_class_member::FormatJsGetterClassMember
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsGetterClassMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsGetterClassMember>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsGetterClassMember {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsGetterClassMember,
        crate::js::classes::getter_class_member::FormatJsGetterClassMember,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::classes::getter_class_member::FormatJsGetterClassMember::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsGetterClassMember {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsGetterClassMember,
        crate::js::classes::getter_class_member::FormatJsGetterClassMember,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::classes::getter_class_member::FormatJsGetterClassMember::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsGetterObjectMember>
    for crate::js::objects::getter_object_member::FormatJsGetterObjectMember
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsGetterObjectMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsGetterObjectMember>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsGetterObjectMember {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsGetterObjectMember,
        crate::js::objects::getter_object_member::FormatJsGetterObjectMember,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::objects::getter_object_member::FormatJsGetterObjectMember::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsGetterObjectMember {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsGetterObjectMember,
        crate::js::objects::getter_object_member::FormatJsGetterObjectMember,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::objects::getter_object_member::FormatJsGetterObjectMember::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsIdentifierAssignment>
    for crate::js::assignments::identifier_assignment::FormatJsIdentifierAssignment
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsIdentifierAssignment,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsIdentifierAssignment>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsIdentifierAssignment {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsIdentifierAssignment,
        crate::js::assignments::identifier_assignment::FormatJsIdentifierAssignment,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::assignments::identifier_assignment::FormatJsIdentifierAssignment::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsIdentifierAssignment {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsIdentifierAssignment,
        crate::js::assignments::identifier_assignment::FormatJsIdentifierAssignment,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::assignments::identifier_assignment::FormatJsIdentifierAssignment::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsIdentifierBinding>
    for crate::js::bindings::identifier_binding::FormatJsIdentifierBinding
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsIdentifierBinding,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsIdentifierBinding>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsIdentifierBinding {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsIdentifierBinding,
        crate::js::bindings::identifier_binding::FormatJsIdentifierBinding,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::bindings::identifier_binding::FormatJsIdentifierBinding::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsIdentifierBinding {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsIdentifierBinding,
        crate::js::bindings::identifier_binding::FormatJsIdentifierBinding,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::bindings::identifier_binding::FormatJsIdentifierBinding::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsIdentifierExpression>
    for crate::js::expressions::identifier_expression::FormatJsIdentifierExpression
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsIdentifierExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsIdentifierExpression>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsIdentifierExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsIdentifierExpression,
        crate::js::expressions::identifier_expression::FormatJsIdentifierExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::expressions::identifier_expression::FormatJsIdentifierExpression::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsIdentifierExpression {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsIdentifierExpression,
        crate::js::expressions::identifier_expression::FormatJsIdentifierExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::expressions::identifier_expression::FormatJsIdentifierExpression::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsIfStatement>
    for crate::js::statements::if_statement::FormatJsIfStatement
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::JsIfStatement, f: &mut JsFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsIfStatement>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsIfStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsIfStatement,
        crate::js::statements::if_statement::FormatJsIfStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::statements::if_statement::FormatJsIfStatement::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsIfStatement {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsIfStatement,
        crate::js::statements::if_statement::FormatJsIfStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::statements::if_statement::FormatJsIfStatement::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsImport> for crate::js::module::import::FormatJsImport {
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::JsImport, f: &mut JsFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsImport>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsImport {
    type Format<'a> =
        FormatRefWithRule<'a, biome_js_syntax::JsImport, crate::js::module::import::FormatJsImport>;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, crate::js::module::import::FormatJsImport::default())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsImport {
    type Format =
        FormatOwnedWithRule<biome_js_syntax::JsImport, crate::js::module::import::FormatJsImport>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, crate::js::module::import::FormatJsImport::default())
    }
}
impl FormatRule<biome_js_syntax::JsImportAssertion>
    for crate::js::module::import_assertion::FormatJsImportAssertion
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsImportAssertion,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsImportAssertion>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsImportAssertion {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsImportAssertion,
        crate::js::module::import_assertion::FormatJsImportAssertion,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::module::import_assertion::FormatJsImportAssertion::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsImportAssertion {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsImportAssertion,
        crate::js::module::import_assertion::FormatJsImportAssertion,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::module::import_assertion::FormatJsImportAssertion::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsImportAssertionEntry>
    for crate::js::module::import_assertion_entry::FormatJsImportAssertionEntry
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsImportAssertionEntry,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsImportAssertionEntry>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsImportAssertionEntry {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsImportAssertionEntry,
        crate::js::module::import_assertion_entry::FormatJsImportAssertionEntry,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::module::import_assertion_entry::FormatJsImportAssertionEntry::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsImportAssertionEntry {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsImportAssertionEntry,
        crate::js::module::import_assertion_entry::FormatJsImportAssertionEntry,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::module::import_assertion_entry::FormatJsImportAssertionEntry::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsImportBareClause>
    for crate::js::module::import_bare_clause::FormatJsImportBareClause
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsImportBareClause,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsImportBareClause>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsImportBareClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsImportBareClause,
        crate::js::module::import_bare_clause::FormatJsImportBareClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::module::import_bare_clause::FormatJsImportBareClause::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsImportBareClause {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsImportBareClause,
        crate::js::module::import_bare_clause::FormatJsImportBareClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::module::import_bare_clause::FormatJsImportBareClause::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsImportCallExpression>
    for crate::js::expressions::import_call_expression::FormatJsImportCallExpression
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsImportCallExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsImportCallExpression>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsImportCallExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsImportCallExpression,
        crate::js::expressions::import_call_expression::FormatJsImportCallExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::expressions::import_call_expression::FormatJsImportCallExpression::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsImportCallExpression {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsImportCallExpression,
        crate::js::expressions::import_call_expression::FormatJsImportCallExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::expressions::import_call_expression::FormatJsImportCallExpression::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsImportCombinedClause>
    for crate::js::module::import_combined_clause::FormatJsImportCombinedClause
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsImportCombinedClause,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsImportCombinedClause>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsImportCombinedClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsImportCombinedClause,
        crate::js::module::import_combined_clause::FormatJsImportCombinedClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::module::import_combined_clause::FormatJsImportCombinedClause::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsImportCombinedClause {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsImportCombinedClause,
        crate::js::module::import_combined_clause::FormatJsImportCombinedClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::module::import_combined_clause::FormatJsImportCombinedClause::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsImportDefaultClause>
    for crate::js::module::import_default_clause::FormatJsImportDefaultClause
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsImportDefaultClause,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsImportDefaultClause>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsImportDefaultClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsImportDefaultClause,
        crate::js::module::import_default_clause::FormatJsImportDefaultClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::module::import_default_clause::FormatJsImportDefaultClause::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsImportDefaultClause {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsImportDefaultClause,
        crate::js::module::import_default_clause::FormatJsImportDefaultClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::module::import_default_clause::FormatJsImportDefaultClause::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsImportMetaExpression>
    for crate::js::expressions::import_meta_expression::FormatJsImportMetaExpression
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsImportMetaExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsImportMetaExpression>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsImportMetaExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsImportMetaExpression,
        crate::js::expressions::import_meta_expression::FormatJsImportMetaExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::expressions::import_meta_expression::FormatJsImportMetaExpression::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsImportMetaExpression {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsImportMetaExpression,
        crate::js::expressions::import_meta_expression::FormatJsImportMetaExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::expressions::import_meta_expression::FormatJsImportMetaExpression::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsImportNamedClause>
    for crate::js::module::import_named_clause::FormatJsImportNamedClause
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsImportNamedClause,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsImportNamedClause>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsImportNamedClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsImportNamedClause,
        crate::js::module::import_named_clause::FormatJsImportNamedClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::module::import_named_clause::FormatJsImportNamedClause::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsImportNamedClause {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsImportNamedClause,
        crate::js::module::import_named_clause::FormatJsImportNamedClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::module::import_named_clause::FormatJsImportNamedClause::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsImportNamespaceClause>
    for crate::js::module::import_namespace_clause::FormatJsImportNamespaceClause
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsImportNamespaceClause,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsImportNamespaceClause>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsImportNamespaceClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsImportNamespaceClause,
        crate::js::module::import_namespace_clause::FormatJsImportNamespaceClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::module::import_namespace_clause::FormatJsImportNamespaceClause::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsImportNamespaceClause {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsImportNamespaceClause,
        crate::js::module::import_namespace_clause::FormatJsImportNamespaceClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::module::import_namespace_clause::FormatJsImportNamespaceClause::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsInExpression>
    for crate::js::expressions::in_expression::FormatJsInExpression
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::JsInExpression, f: &mut JsFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsInExpression>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsInExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsInExpression,
        crate::js::expressions::in_expression::FormatJsInExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::expressions::in_expression::FormatJsInExpression::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsInExpression {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsInExpression,
        crate::js::expressions::in_expression::FormatJsInExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::expressions::in_expression::FormatJsInExpression::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsInitializerClause>
    for crate::js::auxiliary::initializer_clause::FormatJsInitializerClause
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsInitializerClause,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsInitializerClause>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsInitializerClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsInitializerClause,
        crate::js::auxiliary::initializer_clause::FormatJsInitializerClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::auxiliary::initializer_clause::FormatJsInitializerClause::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsInitializerClause {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsInitializerClause,
        crate::js::auxiliary::initializer_clause::FormatJsInitializerClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::auxiliary::initializer_clause::FormatJsInitializerClause::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsInstanceofExpression>
    for crate::js::expressions::instanceof_expression::FormatJsInstanceofExpression
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsInstanceofExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsInstanceofExpression>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsInstanceofExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsInstanceofExpression,
        crate::js::expressions::instanceof_expression::FormatJsInstanceofExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::expressions::instanceof_expression::FormatJsInstanceofExpression::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsInstanceofExpression {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsInstanceofExpression,
        crate::js::expressions::instanceof_expression::FormatJsInstanceofExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::expressions::instanceof_expression::FormatJsInstanceofExpression::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsLabel> for crate::js::auxiliary::label::FormatJsLabel {
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::JsLabel, f: &mut JsFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsLabel>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsLabel {
    type Format<'a> =
        FormatRefWithRule<'a, biome_js_syntax::JsLabel, crate::js::auxiliary::label::FormatJsLabel>;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, crate::js::auxiliary::label::FormatJsLabel::default())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsLabel {
    type Format =
        FormatOwnedWithRule<biome_js_syntax::JsLabel, crate::js::auxiliary::label::FormatJsLabel>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, crate::js::auxiliary::label::FormatJsLabel::default())
    }
}
impl FormatRule<biome_js_syntax::JsLabeledStatement>
    for crate::js::statements::labeled_statement::FormatJsLabeledStatement
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsLabeledStatement,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsLabeledStatement>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsLabeledStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsLabeledStatement,
        crate::js::statements::labeled_statement::FormatJsLabeledStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::statements::labeled_statement::FormatJsLabeledStatement::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsLabeledStatement {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsLabeledStatement,
        crate::js::statements::labeled_statement::FormatJsLabeledStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::statements::labeled_statement::FormatJsLabeledStatement::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsLiteralExportName>
    for crate::js::module::literal_export_name::FormatJsLiteralExportName
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsLiteralExportName,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsLiteralExportName>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsLiteralExportName {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsLiteralExportName,
        crate::js::module::literal_export_name::FormatJsLiteralExportName,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::module::literal_export_name::FormatJsLiteralExportName::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsLiteralExportName {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsLiteralExportName,
        crate::js::module::literal_export_name::FormatJsLiteralExportName,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::module::literal_export_name::FormatJsLiteralExportName::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsLiteralMemberName>
    for crate::js::objects::literal_member_name::FormatJsLiteralMemberName
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsLiteralMemberName,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsLiteralMemberName>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsLiteralMemberName {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsLiteralMemberName,
        crate::js::objects::literal_member_name::FormatJsLiteralMemberName,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::objects::literal_member_name::FormatJsLiteralMemberName::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsLiteralMemberName {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsLiteralMemberName,
        crate::js::objects::literal_member_name::FormatJsLiteralMemberName,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::objects::literal_member_name::FormatJsLiteralMemberName::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsLogicalExpression>
    for crate::js::expressions::logical_expression::FormatJsLogicalExpression
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsLogicalExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsLogicalExpression>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsLogicalExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsLogicalExpression,
        crate::js::expressions::logical_expression::FormatJsLogicalExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::expressions::logical_expression::FormatJsLogicalExpression::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsLogicalExpression {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsLogicalExpression,
        crate::js::expressions::logical_expression::FormatJsLogicalExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::expressions::logical_expression::FormatJsLogicalExpression::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsMetavariable>
    for crate::js::auxiliary::metavariable::FormatJsMetavariable
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::JsMetavariable, f: &mut JsFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsMetavariable>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsMetavariable {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsMetavariable,
        crate::js::auxiliary::metavariable::FormatJsMetavariable,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::auxiliary::metavariable::FormatJsMetavariable::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsMetavariable {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsMetavariable,
        crate::js::auxiliary::metavariable::FormatJsMetavariable,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::auxiliary::metavariable::FormatJsMetavariable::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsMethodClassMember>
    for crate::js::classes::method_class_member::FormatJsMethodClassMember
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsMethodClassMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsMethodClassMember>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsMethodClassMember {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsMethodClassMember,
        crate::js::classes::method_class_member::FormatJsMethodClassMember,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::classes::method_class_member::FormatJsMethodClassMember::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsMethodClassMember {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsMethodClassMember,
        crate::js::classes::method_class_member::FormatJsMethodClassMember,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::classes::method_class_member::FormatJsMethodClassMember::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsMethodObjectMember>
    for crate::js::objects::method_object_member::FormatJsMethodObjectMember
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsMethodObjectMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsMethodObjectMember>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsMethodObjectMember {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsMethodObjectMember,
        crate::js::objects::method_object_member::FormatJsMethodObjectMember,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::objects::method_object_member::FormatJsMethodObjectMember::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsMethodObjectMember {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsMethodObjectMember,
        crate::js::objects::method_object_member::FormatJsMethodObjectMember,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::objects::method_object_member::FormatJsMethodObjectMember::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsModule> for crate::js::auxiliary::module::FormatJsModule {
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::JsModule, f: &mut JsFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsModule>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsModule {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsModule,
        crate::js::auxiliary::module::FormatJsModule,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::auxiliary::module::FormatJsModule::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsModule {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsModule,
        crate::js::auxiliary::module::FormatJsModule,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::auxiliary::module::FormatJsModule::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsModuleSource>
    for crate::js::module::module_source::FormatJsModuleSource
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::JsModuleSource, f: &mut JsFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsModuleSource>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsModuleSource {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsModuleSource,
        crate::js::module::module_source::FormatJsModuleSource,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::module::module_source::FormatJsModuleSource::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsModuleSource {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsModuleSource,
        crate::js::module::module_source::FormatJsModuleSource,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::module::module_source::FormatJsModuleSource::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsName> for crate::js::auxiliary::name::FormatJsName {
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::JsName, f: &mut JsFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsName>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsName {
    type Format<'a> =
        FormatRefWithRule<'a, biome_js_syntax::JsName, crate::js::auxiliary::name::FormatJsName>;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, crate::js::auxiliary::name::FormatJsName::default())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsName {
    type Format =
        FormatOwnedWithRule<biome_js_syntax::JsName, crate::js::auxiliary::name::FormatJsName>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, crate::js::auxiliary::name::FormatJsName::default())
    }
}
impl FormatRule<biome_js_syntax::JsNamedImportSpecifier>
    for crate::js::module::named_import_specifier::FormatJsNamedImportSpecifier
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsNamedImportSpecifier,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsNamedImportSpecifier>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsNamedImportSpecifier {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsNamedImportSpecifier,
        crate::js::module::named_import_specifier::FormatJsNamedImportSpecifier,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::module::named_import_specifier::FormatJsNamedImportSpecifier::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsNamedImportSpecifier {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsNamedImportSpecifier,
        crate::js::module::named_import_specifier::FormatJsNamedImportSpecifier,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::module::named_import_specifier::FormatJsNamedImportSpecifier::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsNamedImportSpecifiers>
    for crate::js::module::named_import_specifiers::FormatJsNamedImportSpecifiers
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsNamedImportSpecifiers,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsNamedImportSpecifiers>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsNamedImportSpecifiers {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsNamedImportSpecifiers,
        crate::js::module::named_import_specifiers::FormatJsNamedImportSpecifiers,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::module::named_import_specifiers::FormatJsNamedImportSpecifiers::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsNamedImportSpecifiers {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsNamedImportSpecifiers,
        crate::js::module::named_import_specifiers::FormatJsNamedImportSpecifiers,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::module::named_import_specifiers::FormatJsNamedImportSpecifiers::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsNamespaceImportSpecifier>
    for crate::js::module::namespace_import_specifier::FormatJsNamespaceImportSpecifier
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsNamespaceImportSpecifier,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsNamespaceImportSpecifier>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsNamespaceImportSpecifier {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsNamespaceImportSpecifier,
        crate::js::module::namespace_import_specifier::FormatJsNamespaceImportSpecifier,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: js :: module :: namespace_import_specifier :: FormatJsNamespaceImportSpecifier :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsNamespaceImportSpecifier {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsNamespaceImportSpecifier,
        crate::js::module::namespace_import_specifier::FormatJsNamespaceImportSpecifier,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: js :: module :: namespace_import_specifier :: FormatJsNamespaceImportSpecifier :: default ())
    }
}
impl FormatRule<biome_js_syntax::JsNewExpression>
    for crate::js::expressions::new_expression::FormatJsNewExpression
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsNewExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsNewExpression>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsNewExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsNewExpression,
        crate::js::expressions::new_expression::FormatJsNewExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::expressions::new_expression::FormatJsNewExpression::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsNewExpression {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsNewExpression,
        crate::js::expressions::new_expression::FormatJsNewExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::expressions::new_expression::FormatJsNewExpression::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsNewTargetExpression>
    for crate::js::expressions::new_target_expression::FormatJsNewTargetExpression
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsNewTargetExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsNewTargetExpression>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsNewTargetExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsNewTargetExpression,
        crate::js::expressions::new_target_expression::FormatJsNewTargetExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::expressions::new_target_expression::FormatJsNewTargetExpression::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsNewTargetExpression {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsNewTargetExpression,
        crate::js::expressions::new_target_expression::FormatJsNewTargetExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::expressions::new_target_expression::FormatJsNewTargetExpression::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsNullLiteralExpression>
    for crate::js::expressions::null_literal_expression::FormatJsNullLiteralExpression
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsNullLiteralExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsNullLiteralExpression>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsNullLiteralExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsNullLiteralExpression,
        crate::js::expressions::null_literal_expression::FormatJsNullLiteralExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::expressions::null_literal_expression::FormatJsNullLiteralExpression::default(
            ),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsNullLiteralExpression {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsNullLiteralExpression,
        crate::js::expressions::null_literal_expression::FormatJsNullLiteralExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::expressions::null_literal_expression::FormatJsNullLiteralExpression::default(
            ),
        )
    }
}
impl FormatRule<biome_js_syntax::JsNumberLiteralExpression>
    for crate::js::expressions::number_literal_expression::FormatJsNumberLiteralExpression
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsNumberLiteralExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsNumberLiteralExpression>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsNumberLiteralExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsNumberLiteralExpression,
        crate::js::expressions::number_literal_expression::FormatJsNumberLiteralExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: js :: expressions :: number_literal_expression :: FormatJsNumberLiteralExpression :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsNumberLiteralExpression {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsNumberLiteralExpression,
        crate::js::expressions::number_literal_expression::FormatJsNumberLiteralExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: js :: expressions :: number_literal_expression :: FormatJsNumberLiteralExpression :: default ())
    }
}
impl FormatRule<biome_js_syntax::JsObjectAssignmentPattern>
    for crate::js::assignments::object_assignment_pattern::FormatJsObjectAssignmentPattern
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsObjectAssignmentPattern,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsObjectAssignmentPattern>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsObjectAssignmentPattern {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsObjectAssignmentPattern,
        crate::js::assignments::object_assignment_pattern::FormatJsObjectAssignmentPattern,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: js :: assignments :: object_assignment_pattern :: FormatJsObjectAssignmentPattern :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsObjectAssignmentPattern {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsObjectAssignmentPattern,
        crate::js::assignments::object_assignment_pattern::FormatJsObjectAssignmentPattern,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: js :: assignments :: object_assignment_pattern :: FormatJsObjectAssignmentPattern :: default ())
    }
}
impl FormatRule < biome_js_syntax :: JsObjectAssignmentPatternProperty > for crate :: js :: assignments :: object_assignment_pattern_property :: FormatJsObjectAssignmentPatternProperty { type Context = JsFormatContext ; # [inline (always)] fn fmt (& self , node : & biome_js_syntax :: JsObjectAssignmentPatternProperty , f : & mut JsFormatter) -> FormatResult < () > { FormatNodeRule :: < biome_js_syntax :: JsObjectAssignmentPatternProperty > :: fmt (self , node , f) } }
impl AsFormat<JsFormatContext> for biome_js_syntax::JsObjectAssignmentPatternProperty {
    type Format < 'a > = FormatRefWithRule < 'a , biome_js_syntax :: JsObjectAssignmentPatternProperty , crate :: js :: assignments :: object_assignment_pattern_property :: FormatJsObjectAssignmentPatternProperty > ;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: js :: assignments :: object_assignment_pattern_property :: FormatJsObjectAssignmentPatternProperty :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsObjectAssignmentPatternProperty {
    type Format = FormatOwnedWithRule < biome_js_syntax :: JsObjectAssignmentPatternProperty , crate :: js :: assignments :: object_assignment_pattern_property :: FormatJsObjectAssignmentPatternProperty > ;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: js :: assignments :: object_assignment_pattern_property :: FormatJsObjectAssignmentPatternProperty :: default ())
    }
}
impl FormatRule<biome_js_syntax::JsObjectAssignmentPatternRest>
    for crate::js::assignments::object_assignment_pattern_rest::FormatJsObjectAssignmentPatternRest
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsObjectAssignmentPatternRest,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsObjectAssignmentPatternRest>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsObjectAssignmentPatternRest {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsObjectAssignmentPatternRest,
        crate::js::assignments::object_assignment_pattern_rest::FormatJsObjectAssignmentPatternRest,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: js :: assignments :: object_assignment_pattern_rest :: FormatJsObjectAssignmentPatternRest :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsObjectAssignmentPatternRest {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsObjectAssignmentPatternRest,
        crate::js::assignments::object_assignment_pattern_rest::FormatJsObjectAssignmentPatternRest,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: js :: assignments :: object_assignment_pattern_rest :: FormatJsObjectAssignmentPatternRest :: default ())
    }
}
impl FormatRule < biome_js_syntax :: JsObjectAssignmentPatternShorthandProperty > for crate :: js :: assignments :: object_assignment_pattern_shorthand_property :: FormatJsObjectAssignmentPatternShorthandProperty { type Context = JsFormatContext ; # [inline (always)] fn fmt (& self , node : & biome_js_syntax :: JsObjectAssignmentPatternShorthandProperty , f : & mut JsFormatter) -> FormatResult < () > { FormatNodeRule :: < biome_js_syntax :: JsObjectAssignmentPatternShorthandProperty > :: fmt (self , node , f) } }
impl AsFormat<JsFormatContext> for biome_js_syntax::JsObjectAssignmentPatternShorthandProperty {
    type Format < 'a > = FormatRefWithRule < 'a , biome_js_syntax :: JsObjectAssignmentPatternShorthandProperty , crate :: js :: assignments :: object_assignment_pattern_shorthand_property :: FormatJsObjectAssignmentPatternShorthandProperty > ;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: js :: assignments :: object_assignment_pattern_shorthand_property :: FormatJsObjectAssignmentPatternShorthandProperty :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsObjectAssignmentPatternShorthandProperty {
    type Format = FormatOwnedWithRule < biome_js_syntax :: JsObjectAssignmentPatternShorthandProperty , crate :: js :: assignments :: object_assignment_pattern_shorthand_property :: FormatJsObjectAssignmentPatternShorthandProperty > ;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: js :: assignments :: object_assignment_pattern_shorthand_property :: FormatJsObjectAssignmentPatternShorthandProperty :: default ())
    }
}
impl FormatRule<biome_js_syntax::JsObjectBindingPattern>
    for crate::js::bindings::object_binding_pattern::FormatJsObjectBindingPattern
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsObjectBindingPattern,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsObjectBindingPattern>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsObjectBindingPattern {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsObjectBindingPattern,
        crate::js::bindings::object_binding_pattern::FormatJsObjectBindingPattern,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::bindings::object_binding_pattern::FormatJsObjectBindingPattern::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsObjectBindingPattern {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsObjectBindingPattern,
        crate::js::bindings::object_binding_pattern::FormatJsObjectBindingPattern,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::bindings::object_binding_pattern::FormatJsObjectBindingPattern::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsObjectBindingPatternProperty>
    for crate::js::bindings::object_binding_pattern_property::FormatJsObjectBindingPatternProperty
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsObjectBindingPatternProperty,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsObjectBindingPatternProperty>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsObjectBindingPatternProperty {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsObjectBindingPatternProperty,
        crate::js::bindings::object_binding_pattern_property::FormatJsObjectBindingPatternProperty,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: js :: bindings :: object_binding_pattern_property :: FormatJsObjectBindingPatternProperty :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsObjectBindingPatternProperty {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsObjectBindingPatternProperty,
        crate::js::bindings::object_binding_pattern_property::FormatJsObjectBindingPatternProperty,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: js :: bindings :: object_binding_pattern_property :: FormatJsObjectBindingPatternProperty :: default ())
    }
}
impl FormatRule<biome_js_syntax::JsObjectBindingPatternRest>
    for crate::js::bindings::object_binding_pattern_rest::FormatJsObjectBindingPatternRest
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsObjectBindingPatternRest,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsObjectBindingPatternRest>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsObjectBindingPatternRest {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsObjectBindingPatternRest,
        crate::js::bindings::object_binding_pattern_rest::FormatJsObjectBindingPatternRest,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: js :: bindings :: object_binding_pattern_rest :: FormatJsObjectBindingPatternRest :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsObjectBindingPatternRest {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsObjectBindingPatternRest,
        crate::js::bindings::object_binding_pattern_rest::FormatJsObjectBindingPatternRest,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: js :: bindings :: object_binding_pattern_rest :: FormatJsObjectBindingPatternRest :: default ())
    }
}
impl FormatRule < biome_js_syntax :: JsObjectBindingPatternShorthandProperty > for crate :: js :: bindings :: object_binding_pattern_shorthand_property :: FormatJsObjectBindingPatternShorthandProperty { type Context = JsFormatContext ; # [inline (always)] fn fmt (& self , node : & biome_js_syntax :: JsObjectBindingPatternShorthandProperty , f : & mut JsFormatter) -> FormatResult < () > { FormatNodeRule :: < biome_js_syntax :: JsObjectBindingPatternShorthandProperty > :: fmt (self , node , f) } }
impl AsFormat<JsFormatContext> for biome_js_syntax::JsObjectBindingPatternShorthandProperty {
    type Format < 'a > = FormatRefWithRule < 'a , biome_js_syntax :: JsObjectBindingPatternShorthandProperty , crate :: js :: bindings :: object_binding_pattern_shorthand_property :: FormatJsObjectBindingPatternShorthandProperty > ;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: js :: bindings :: object_binding_pattern_shorthand_property :: FormatJsObjectBindingPatternShorthandProperty :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsObjectBindingPatternShorthandProperty {
    type Format = FormatOwnedWithRule < biome_js_syntax :: JsObjectBindingPatternShorthandProperty , crate :: js :: bindings :: object_binding_pattern_shorthand_property :: FormatJsObjectBindingPatternShorthandProperty > ;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: js :: bindings :: object_binding_pattern_shorthand_property :: FormatJsObjectBindingPatternShorthandProperty :: default ())
    }
}
impl FormatRule<biome_js_syntax::JsObjectExpression>
    for crate::js::expressions::object_expression::FormatJsObjectExpression
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsObjectExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsObjectExpression>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsObjectExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsObjectExpression,
        crate::js::expressions::object_expression::FormatJsObjectExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::expressions::object_expression::FormatJsObjectExpression::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsObjectExpression {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsObjectExpression,
        crate::js::expressions::object_expression::FormatJsObjectExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::expressions::object_expression::FormatJsObjectExpression::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsParameters>
    for crate::js::bindings::parameters::FormatJsParameters
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::JsParameters, f: &mut JsFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsParameters>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsParameters {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsParameters,
        crate::js::bindings::parameters::FormatJsParameters,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::bindings::parameters::FormatJsParameters::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsParameters {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsParameters,
        crate::js::bindings::parameters::FormatJsParameters,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::bindings::parameters::FormatJsParameters::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsParenthesizedAssignment>
    for crate::js::assignments::parenthesized_assignment::FormatJsParenthesizedAssignment
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsParenthesizedAssignment,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsParenthesizedAssignment>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsParenthesizedAssignment {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsParenthesizedAssignment,
        crate::js::assignments::parenthesized_assignment::FormatJsParenthesizedAssignment,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: js :: assignments :: parenthesized_assignment :: FormatJsParenthesizedAssignment :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsParenthesizedAssignment {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsParenthesizedAssignment,
        crate::js::assignments::parenthesized_assignment::FormatJsParenthesizedAssignment,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: js :: assignments :: parenthesized_assignment :: FormatJsParenthesizedAssignment :: default ())
    }
}
impl FormatRule<biome_js_syntax::JsParenthesizedExpression>
    for crate::js::expressions::parenthesized_expression::FormatJsParenthesizedExpression
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsParenthesizedExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsParenthesizedExpression>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsParenthesizedExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsParenthesizedExpression,
        crate::js::expressions::parenthesized_expression::FormatJsParenthesizedExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: js :: expressions :: parenthesized_expression :: FormatJsParenthesizedExpression :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsParenthesizedExpression {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsParenthesizedExpression,
        crate::js::expressions::parenthesized_expression::FormatJsParenthesizedExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: js :: expressions :: parenthesized_expression :: FormatJsParenthesizedExpression :: default ())
    }
}
impl FormatRule<biome_js_syntax::JsPostUpdateExpression>
    for crate::js::expressions::post_update_expression::FormatJsPostUpdateExpression
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsPostUpdateExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsPostUpdateExpression>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsPostUpdateExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsPostUpdateExpression,
        crate::js::expressions::post_update_expression::FormatJsPostUpdateExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::expressions::post_update_expression::FormatJsPostUpdateExpression::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsPostUpdateExpression {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsPostUpdateExpression,
        crate::js::expressions::post_update_expression::FormatJsPostUpdateExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::expressions::post_update_expression::FormatJsPostUpdateExpression::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsPreUpdateExpression>
    for crate::js::expressions::pre_update_expression::FormatJsPreUpdateExpression
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsPreUpdateExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsPreUpdateExpression>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsPreUpdateExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsPreUpdateExpression,
        crate::js::expressions::pre_update_expression::FormatJsPreUpdateExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::expressions::pre_update_expression::FormatJsPreUpdateExpression::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsPreUpdateExpression {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsPreUpdateExpression,
        crate::js::expressions::pre_update_expression::FormatJsPreUpdateExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::expressions::pre_update_expression::FormatJsPreUpdateExpression::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsPrivateClassMemberName>
    for crate::js::objects::private_class_member_name::FormatJsPrivateClassMemberName
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsPrivateClassMemberName,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsPrivateClassMemberName>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsPrivateClassMemberName {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsPrivateClassMemberName,
        crate::js::objects::private_class_member_name::FormatJsPrivateClassMemberName,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::objects::private_class_member_name::FormatJsPrivateClassMemberName::default(
            ),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsPrivateClassMemberName {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsPrivateClassMemberName,
        crate::js::objects::private_class_member_name::FormatJsPrivateClassMemberName,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::objects::private_class_member_name::FormatJsPrivateClassMemberName::default(
            ),
        )
    }
}
impl FormatRule<biome_js_syntax::JsPrivateName>
    for crate::js::auxiliary::private_name::FormatJsPrivateName
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::JsPrivateName, f: &mut JsFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsPrivateName>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsPrivateName {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsPrivateName,
        crate::js::auxiliary::private_name::FormatJsPrivateName,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::auxiliary::private_name::FormatJsPrivateName::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsPrivateName {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsPrivateName,
        crate::js::auxiliary::private_name::FormatJsPrivateName,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::auxiliary::private_name::FormatJsPrivateName::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsPropertyClassMember>
    for crate::js::classes::property_class_member::FormatJsPropertyClassMember
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsPropertyClassMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsPropertyClassMember>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsPropertyClassMember {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsPropertyClassMember,
        crate::js::classes::property_class_member::FormatJsPropertyClassMember,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::classes::property_class_member::FormatJsPropertyClassMember::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsPropertyClassMember {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsPropertyClassMember,
        crate::js::classes::property_class_member::FormatJsPropertyClassMember,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::classes::property_class_member::FormatJsPropertyClassMember::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsPropertyObjectMember>
    for crate::js::objects::property_object_member::FormatJsPropertyObjectMember
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsPropertyObjectMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsPropertyObjectMember>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsPropertyObjectMember {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsPropertyObjectMember,
        crate::js::objects::property_object_member::FormatJsPropertyObjectMember,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::objects::property_object_member::FormatJsPropertyObjectMember::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsPropertyObjectMember {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsPropertyObjectMember,
        crate::js::objects::property_object_member::FormatJsPropertyObjectMember,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::objects::property_object_member::FormatJsPropertyObjectMember::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsReferenceIdentifier>
    for crate::js::auxiliary::reference_identifier::FormatJsReferenceIdentifier
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsReferenceIdentifier,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsReferenceIdentifier>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsReferenceIdentifier {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsReferenceIdentifier,
        crate::js::auxiliary::reference_identifier::FormatJsReferenceIdentifier,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::auxiliary::reference_identifier::FormatJsReferenceIdentifier::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsReferenceIdentifier {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsReferenceIdentifier,
        crate::js::auxiliary::reference_identifier::FormatJsReferenceIdentifier,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::auxiliary::reference_identifier::FormatJsReferenceIdentifier::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsRegexLiteralExpression>
    for crate::js::expressions::regex_literal_expression::FormatJsRegexLiteralExpression
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsRegexLiteralExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsRegexLiteralExpression>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsRegexLiteralExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsRegexLiteralExpression,
        crate::js::expressions::regex_literal_expression::FormatJsRegexLiteralExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: js :: expressions :: regex_literal_expression :: FormatJsRegexLiteralExpression :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsRegexLiteralExpression {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsRegexLiteralExpression,
        crate::js::expressions::regex_literal_expression::FormatJsRegexLiteralExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: js :: expressions :: regex_literal_expression :: FormatJsRegexLiteralExpression :: default ())
    }
}
impl FormatRule<biome_js_syntax::JsRestParameter>
    for crate::js::bindings::rest_parameter::FormatJsRestParameter
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsRestParameter,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsRestParameter>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsRestParameter {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsRestParameter,
        crate::js::bindings::rest_parameter::FormatJsRestParameter,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::bindings::rest_parameter::FormatJsRestParameter::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsRestParameter {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsRestParameter,
        crate::js::bindings::rest_parameter::FormatJsRestParameter,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::bindings::rest_parameter::FormatJsRestParameter::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsReturnStatement>
    for crate::js::statements::return_statement::FormatJsReturnStatement
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsReturnStatement,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsReturnStatement>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsReturnStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsReturnStatement,
        crate::js::statements::return_statement::FormatJsReturnStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::statements::return_statement::FormatJsReturnStatement::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsReturnStatement {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsReturnStatement,
        crate::js::statements::return_statement::FormatJsReturnStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::statements::return_statement::FormatJsReturnStatement::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsScript> for crate::js::auxiliary::script::FormatJsScript {
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::JsScript, f: &mut JsFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsScript>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsScript {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsScript,
        crate::js::auxiliary::script::FormatJsScript,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::auxiliary::script::FormatJsScript::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsScript {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsScript,
        crate::js::auxiliary::script::FormatJsScript,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::auxiliary::script::FormatJsScript::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsSequenceExpression>
    for crate::js::expressions::sequence_expression::FormatJsSequenceExpression
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsSequenceExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsSequenceExpression>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsSequenceExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsSequenceExpression,
        crate::js::expressions::sequence_expression::FormatJsSequenceExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::expressions::sequence_expression::FormatJsSequenceExpression::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsSequenceExpression {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsSequenceExpression,
        crate::js::expressions::sequence_expression::FormatJsSequenceExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::expressions::sequence_expression::FormatJsSequenceExpression::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsSetterClassMember>
    for crate::js::classes::setter_class_member::FormatJsSetterClassMember
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsSetterClassMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsSetterClassMember>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsSetterClassMember {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsSetterClassMember,
        crate::js::classes::setter_class_member::FormatJsSetterClassMember,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::classes::setter_class_member::FormatJsSetterClassMember::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsSetterClassMember {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsSetterClassMember,
        crate::js::classes::setter_class_member::FormatJsSetterClassMember,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::classes::setter_class_member::FormatJsSetterClassMember::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsSetterObjectMember>
    for crate::js::objects::setter_object_member::FormatJsSetterObjectMember
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsSetterObjectMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsSetterObjectMember>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsSetterObjectMember {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsSetterObjectMember,
        crate::js::objects::setter_object_member::FormatJsSetterObjectMember,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::objects::setter_object_member::FormatJsSetterObjectMember::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsSetterObjectMember {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsSetterObjectMember,
        crate::js::objects::setter_object_member::FormatJsSetterObjectMember,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::objects::setter_object_member::FormatJsSetterObjectMember::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsShorthandNamedImportSpecifier>
    for crate::js::module::shorthand_named_import_specifier::FormatJsShorthandNamedImportSpecifier
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsShorthandNamedImportSpecifier,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsShorthandNamedImportSpecifier>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsShorthandNamedImportSpecifier {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsShorthandNamedImportSpecifier,
        crate::js::module::shorthand_named_import_specifier::FormatJsShorthandNamedImportSpecifier,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: js :: module :: shorthand_named_import_specifier :: FormatJsShorthandNamedImportSpecifier :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsShorthandNamedImportSpecifier {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsShorthandNamedImportSpecifier,
        crate::js::module::shorthand_named_import_specifier::FormatJsShorthandNamedImportSpecifier,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: js :: module :: shorthand_named_import_specifier :: FormatJsShorthandNamedImportSpecifier :: default ())
    }
}
impl FormatRule<biome_js_syntax::JsShorthandPropertyObjectMember>
    for crate::js::objects::shorthand_property_object_member::FormatJsShorthandPropertyObjectMember
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsShorthandPropertyObjectMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsShorthandPropertyObjectMember>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsShorthandPropertyObjectMember {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsShorthandPropertyObjectMember,
        crate::js::objects::shorthand_property_object_member::FormatJsShorthandPropertyObjectMember,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: js :: objects :: shorthand_property_object_member :: FormatJsShorthandPropertyObjectMember :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsShorthandPropertyObjectMember {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsShorthandPropertyObjectMember,
        crate::js::objects::shorthand_property_object_member::FormatJsShorthandPropertyObjectMember,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: js :: objects :: shorthand_property_object_member :: FormatJsShorthandPropertyObjectMember :: default ())
    }
}
impl FormatRule<biome_js_syntax::JsSpread> for crate::js::auxiliary::spread::FormatJsSpread {
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::JsSpread, f: &mut JsFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsSpread>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsSpread {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsSpread,
        crate::js::auxiliary::spread::FormatJsSpread,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::auxiliary::spread::FormatJsSpread::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsSpread {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsSpread,
        crate::js::auxiliary::spread::FormatJsSpread,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::auxiliary::spread::FormatJsSpread::default(),
        )
    }
}
impl FormatRule < biome_js_syntax :: JsStaticInitializationBlockClassMember > for crate :: js :: classes :: static_initialization_block_class_member :: FormatJsStaticInitializationBlockClassMember { type Context = JsFormatContext ; # [inline (always)] fn fmt (& self , node : & biome_js_syntax :: JsStaticInitializationBlockClassMember , f : & mut JsFormatter) -> FormatResult < () > { FormatNodeRule :: < biome_js_syntax :: JsStaticInitializationBlockClassMember > :: fmt (self , node , f) } }
impl AsFormat<JsFormatContext> for biome_js_syntax::JsStaticInitializationBlockClassMember {
    type Format < 'a > = FormatRefWithRule < 'a , biome_js_syntax :: JsStaticInitializationBlockClassMember , crate :: js :: classes :: static_initialization_block_class_member :: FormatJsStaticInitializationBlockClassMember > ;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: js :: classes :: static_initialization_block_class_member :: FormatJsStaticInitializationBlockClassMember :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsStaticInitializationBlockClassMember {
    type Format = FormatOwnedWithRule < biome_js_syntax :: JsStaticInitializationBlockClassMember , crate :: js :: classes :: static_initialization_block_class_member :: FormatJsStaticInitializationBlockClassMember > ;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: js :: classes :: static_initialization_block_class_member :: FormatJsStaticInitializationBlockClassMember :: default ())
    }
}
impl FormatRule<biome_js_syntax::JsStaticMemberAssignment>
    for crate::js::assignments::static_member_assignment::FormatJsStaticMemberAssignment
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsStaticMemberAssignment,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsStaticMemberAssignment>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsStaticMemberAssignment {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsStaticMemberAssignment,
        crate::js::assignments::static_member_assignment::FormatJsStaticMemberAssignment,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: js :: assignments :: static_member_assignment :: FormatJsStaticMemberAssignment :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsStaticMemberAssignment {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsStaticMemberAssignment,
        crate::js::assignments::static_member_assignment::FormatJsStaticMemberAssignment,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: js :: assignments :: static_member_assignment :: FormatJsStaticMemberAssignment :: default ())
    }
}
impl FormatRule<biome_js_syntax::JsStaticMemberExpression>
    for crate::js::expressions::static_member_expression::FormatJsStaticMemberExpression
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsStaticMemberExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsStaticMemberExpression>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsStaticMemberExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsStaticMemberExpression,
        crate::js::expressions::static_member_expression::FormatJsStaticMemberExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: js :: expressions :: static_member_expression :: FormatJsStaticMemberExpression :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsStaticMemberExpression {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsStaticMemberExpression,
        crate::js::expressions::static_member_expression::FormatJsStaticMemberExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: js :: expressions :: static_member_expression :: FormatJsStaticMemberExpression :: default ())
    }
}
impl FormatRule<biome_js_syntax::JsStaticModifier>
    for crate::js::auxiliary::static_modifier::FormatJsStaticModifier
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsStaticModifier,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsStaticModifier>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsStaticModifier {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsStaticModifier,
        crate::js::auxiliary::static_modifier::FormatJsStaticModifier,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::auxiliary::static_modifier::FormatJsStaticModifier::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsStaticModifier {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsStaticModifier,
        crate::js::auxiliary::static_modifier::FormatJsStaticModifier,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::auxiliary::static_modifier::FormatJsStaticModifier::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsStringLiteralExpression>
    for crate::js::expressions::string_literal_expression::FormatJsStringLiteralExpression
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsStringLiteralExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsStringLiteralExpression>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsStringLiteralExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsStringLiteralExpression,
        crate::js::expressions::string_literal_expression::FormatJsStringLiteralExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: js :: expressions :: string_literal_expression :: FormatJsStringLiteralExpression :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsStringLiteralExpression {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsStringLiteralExpression,
        crate::js::expressions::string_literal_expression::FormatJsStringLiteralExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: js :: expressions :: string_literal_expression :: FormatJsStringLiteralExpression :: default ())
    }
}
impl FormatRule<biome_js_syntax::JsSuperExpression>
    for crate::js::expressions::super_expression::FormatJsSuperExpression
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsSuperExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsSuperExpression>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsSuperExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsSuperExpression,
        crate::js::expressions::super_expression::FormatJsSuperExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::expressions::super_expression::FormatJsSuperExpression::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsSuperExpression {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsSuperExpression,
        crate::js::expressions::super_expression::FormatJsSuperExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::expressions::super_expression::FormatJsSuperExpression::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsSwitchStatement>
    for crate::js::statements::switch_statement::FormatJsSwitchStatement
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsSwitchStatement,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsSwitchStatement>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsSwitchStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsSwitchStatement,
        crate::js::statements::switch_statement::FormatJsSwitchStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::statements::switch_statement::FormatJsSwitchStatement::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsSwitchStatement {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsSwitchStatement,
        crate::js::statements::switch_statement::FormatJsSwitchStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::statements::switch_statement::FormatJsSwitchStatement::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsTemplateChunkElement>
    for crate::js::auxiliary::template_chunk_element::FormatJsTemplateChunkElement
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsTemplateChunkElement,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsTemplateChunkElement>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsTemplateChunkElement {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsTemplateChunkElement,
        crate::js::auxiliary::template_chunk_element::FormatJsTemplateChunkElement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::auxiliary::template_chunk_element::FormatJsTemplateChunkElement::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsTemplateChunkElement {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsTemplateChunkElement,
        crate::js::auxiliary::template_chunk_element::FormatJsTemplateChunkElement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::auxiliary::template_chunk_element::FormatJsTemplateChunkElement::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsTemplateElement>
    for crate::js::auxiliary::template_element::FormatJsTemplateElement
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsTemplateElement,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsTemplateElement>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsTemplateElement {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsTemplateElement,
        crate::js::auxiliary::template_element::FormatJsTemplateElement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::auxiliary::template_element::FormatJsTemplateElement::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsTemplateElement {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsTemplateElement,
        crate::js::auxiliary::template_element::FormatJsTemplateElement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::auxiliary::template_element::FormatJsTemplateElement::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsTemplateExpression>
    for crate::js::expressions::template_expression::FormatJsTemplateExpression
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsTemplateExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsTemplateExpression>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsTemplateExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsTemplateExpression,
        crate::js::expressions::template_expression::FormatJsTemplateExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::expressions::template_expression::FormatJsTemplateExpression::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsTemplateExpression {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsTemplateExpression,
        crate::js::expressions::template_expression::FormatJsTemplateExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::expressions::template_expression::FormatJsTemplateExpression::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsThisExpression>
    for crate::js::expressions::this_expression::FormatJsThisExpression
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsThisExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsThisExpression>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsThisExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsThisExpression,
        crate::js::expressions::this_expression::FormatJsThisExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::expressions::this_expression::FormatJsThisExpression::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsThisExpression {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsThisExpression,
        crate::js::expressions::this_expression::FormatJsThisExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::expressions::this_expression::FormatJsThisExpression::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsThrowStatement>
    for crate::js::statements::throw_statement::FormatJsThrowStatement
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsThrowStatement,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsThrowStatement>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsThrowStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsThrowStatement,
        crate::js::statements::throw_statement::FormatJsThrowStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::statements::throw_statement::FormatJsThrowStatement::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsThrowStatement {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsThrowStatement,
        crate::js::statements::throw_statement::FormatJsThrowStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::statements::throw_statement::FormatJsThrowStatement::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsTryFinallyStatement>
    for crate::js::statements::try_finally_statement::FormatJsTryFinallyStatement
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsTryFinallyStatement,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsTryFinallyStatement>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsTryFinallyStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsTryFinallyStatement,
        crate::js::statements::try_finally_statement::FormatJsTryFinallyStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::statements::try_finally_statement::FormatJsTryFinallyStatement::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsTryFinallyStatement {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsTryFinallyStatement,
        crate::js::statements::try_finally_statement::FormatJsTryFinallyStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::statements::try_finally_statement::FormatJsTryFinallyStatement::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsTryStatement>
    for crate::js::statements::try_statement::FormatJsTryStatement
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::JsTryStatement, f: &mut JsFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsTryStatement>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsTryStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsTryStatement,
        crate::js::statements::try_statement::FormatJsTryStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::statements::try_statement::FormatJsTryStatement::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsTryStatement {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsTryStatement,
        crate::js::statements::try_statement::FormatJsTryStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::statements::try_statement::FormatJsTryStatement::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsUnaryExpression>
    for crate::js::expressions::unary_expression::FormatJsUnaryExpression
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsUnaryExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsUnaryExpression>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsUnaryExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsUnaryExpression,
        crate::js::expressions::unary_expression::FormatJsUnaryExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::expressions::unary_expression::FormatJsUnaryExpression::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsUnaryExpression {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsUnaryExpression,
        crate::js::expressions::unary_expression::FormatJsUnaryExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::expressions::unary_expression::FormatJsUnaryExpression::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsVariableDeclaration>
    for crate::js::declarations::variable_declaration::FormatJsVariableDeclaration
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsVariableDeclaration,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsVariableDeclaration>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsVariableDeclaration {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsVariableDeclaration,
        crate::js::declarations::variable_declaration::FormatJsVariableDeclaration,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::declarations::variable_declaration::FormatJsVariableDeclaration::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsVariableDeclaration {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsVariableDeclaration,
        crate::js::declarations::variable_declaration::FormatJsVariableDeclaration,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::declarations::variable_declaration::FormatJsVariableDeclaration::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsVariableDeclarationClause>
    for crate::js::auxiliary::variable_declaration_clause::FormatJsVariableDeclarationClause
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsVariableDeclarationClause,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsVariableDeclarationClause>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsVariableDeclarationClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsVariableDeclarationClause,
        crate::js::auxiliary::variable_declaration_clause::FormatJsVariableDeclarationClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: js :: auxiliary :: variable_declaration_clause :: FormatJsVariableDeclarationClause :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsVariableDeclarationClause {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsVariableDeclarationClause,
        crate::js::auxiliary::variable_declaration_clause::FormatJsVariableDeclarationClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: js :: auxiliary :: variable_declaration_clause :: FormatJsVariableDeclarationClause :: default ())
    }
}
impl FormatRule<biome_js_syntax::JsVariableDeclarator>
    for crate::js::auxiliary::variable_declarator::FormatJsVariableDeclarator
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsVariableDeclarator,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsVariableDeclarator>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsVariableDeclarator {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsVariableDeclarator,
        crate::js::auxiliary::variable_declarator::FormatJsVariableDeclarator,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::auxiliary::variable_declarator::FormatJsVariableDeclarator::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsVariableDeclarator {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsVariableDeclarator,
        crate::js::auxiliary::variable_declarator::FormatJsVariableDeclarator,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::auxiliary::variable_declarator::FormatJsVariableDeclarator::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsVariableStatement>
    for crate::js::statements::variable_statement::FormatJsVariableStatement
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsVariableStatement,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsVariableStatement>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsVariableStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsVariableStatement,
        crate::js::statements::variable_statement::FormatJsVariableStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::statements::variable_statement::FormatJsVariableStatement::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsVariableStatement {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsVariableStatement,
        crate::js::statements::variable_statement::FormatJsVariableStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::statements::variable_statement::FormatJsVariableStatement::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsWhileStatement>
    for crate::js::statements::while_statement::FormatJsWhileStatement
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsWhileStatement,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsWhileStatement>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsWhileStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsWhileStatement,
        crate::js::statements::while_statement::FormatJsWhileStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::statements::while_statement::FormatJsWhileStatement::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsWhileStatement {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsWhileStatement,
        crate::js::statements::while_statement::FormatJsWhileStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::statements::while_statement::FormatJsWhileStatement::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsWithStatement>
    for crate::js::statements::with_statement::FormatJsWithStatement
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsWithStatement,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsWithStatement>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsWithStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsWithStatement,
        crate::js::statements::with_statement::FormatJsWithStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::statements::with_statement::FormatJsWithStatement::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsWithStatement {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsWithStatement,
        crate::js::statements::with_statement::FormatJsWithStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::statements::with_statement::FormatJsWithStatement::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsYieldArgument>
    for crate::js::expressions::yield_argument::FormatJsYieldArgument
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsYieldArgument,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsYieldArgument>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsYieldArgument {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsYieldArgument,
        crate::js::expressions::yield_argument::FormatJsYieldArgument,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::expressions::yield_argument::FormatJsYieldArgument::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsYieldArgument {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsYieldArgument,
        crate::js::expressions::yield_argument::FormatJsYieldArgument,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::expressions::yield_argument::FormatJsYieldArgument::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsYieldExpression>
    for crate::js::expressions::yield_expression::FormatJsYieldExpression
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsYieldExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsYieldExpression>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsYieldExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsYieldExpression,
        crate::js::expressions::yield_expression::FormatJsYieldExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::expressions::yield_expression::FormatJsYieldExpression::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsYieldExpression {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsYieldExpression,
        crate::js::expressions::yield_expression::FormatJsYieldExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::expressions::yield_expression::FormatJsYieldExpression::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsxAttribute>
    for crate::jsx::attribute::attribute::FormatJsxAttribute
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::JsxAttribute, f: &mut JsFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsxAttribute>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsxAttribute {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsxAttribute,
        crate::jsx::attribute::attribute::FormatJsxAttribute,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::jsx::attribute::attribute::FormatJsxAttribute::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsxAttribute {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsxAttribute,
        crate::jsx::attribute::attribute::FormatJsxAttribute,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::jsx::attribute::attribute::FormatJsxAttribute::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsxAttributeInitializerClause>
    for crate::jsx::attribute::attribute_initializer_clause::FormatJsxAttributeInitializerClause
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsxAttributeInitializerClause,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsxAttributeInitializerClause>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsxAttributeInitializerClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsxAttributeInitializerClause,
        crate::jsx::attribute::attribute_initializer_clause::FormatJsxAttributeInitializerClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: jsx :: attribute :: attribute_initializer_clause :: FormatJsxAttributeInitializerClause :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsxAttributeInitializerClause {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsxAttributeInitializerClause,
        crate::jsx::attribute::attribute_initializer_clause::FormatJsxAttributeInitializerClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: jsx :: attribute :: attribute_initializer_clause :: FormatJsxAttributeInitializerClause :: default ())
    }
}
impl FormatRule<biome_js_syntax::JsxClosingElement>
    for crate::jsx::tag::closing_element::FormatJsxClosingElement
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsxClosingElement,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsxClosingElement>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsxClosingElement {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsxClosingElement,
        crate::jsx::tag::closing_element::FormatJsxClosingElement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::jsx::tag::closing_element::FormatJsxClosingElement::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsxClosingElement {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsxClosingElement,
        crate::jsx::tag::closing_element::FormatJsxClosingElement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::jsx::tag::closing_element::FormatJsxClosingElement::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsxClosingFragment>
    for crate::jsx::tag::closing_fragment::FormatJsxClosingFragment
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsxClosingFragment,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsxClosingFragment>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsxClosingFragment {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsxClosingFragment,
        crate::jsx::tag::closing_fragment::FormatJsxClosingFragment,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::jsx::tag::closing_fragment::FormatJsxClosingFragment::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsxClosingFragment {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsxClosingFragment,
        crate::jsx::tag::closing_fragment::FormatJsxClosingFragment,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::jsx::tag::closing_fragment::FormatJsxClosingFragment::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsxElement> for crate::jsx::tag::element::FormatJsxElement {
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::JsxElement, f: &mut JsFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsxElement>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsxElement {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsxElement,
        crate::jsx::tag::element::FormatJsxElement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, crate::jsx::tag::element::FormatJsxElement::default())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsxElement {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsxElement,
        crate::jsx::tag::element::FormatJsxElement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, crate::jsx::tag::element::FormatJsxElement::default())
    }
}
impl FormatRule<biome_js_syntax::JsxExpressionAttributeValue>
    for crate::jsx::attribute::expression_attribute_value::FormatJsxExpressionAttributeValue
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsxExpressionAttributeValue,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsxExpressionAttributeValue>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsxExpressionAttributeValue {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsxExpressionAttributeValue,
        crate::jsx::attribute::expression_attribute_value::FormatJsxExpressionAttributeValue,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: jsx :: attribute :: expression_attribute_value :: FormatJsxExpressionAttributeValue :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsxExpressionAttributeValue {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsxExpressionAttributeValue,
        crate::jsx::attribute::expression_attribute_value::FormatJsxExpressionAttributeValue,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: jsx :: attribute :: expression_attribute_value :: FormatJsxExpressionAttributeValue :: default ())
    }
}
impl FormatRule<biome_js_syntax::JsxExpressionChild>
    for crate::jsx::auxiliary::expression_child::FormatJsxExpressionChild
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsxExpressionChild,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsxExpressionChild>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsxExpressionChild {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsxExpressionChild,
        crate::jsx::auxiliary::expression_child::FormatJsxExpressionChild,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::jsx::auxiliary::expression_child::FormatJsxExpressionChild::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsxExpressionChild {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsxExpressionChild,
        crate::jsx::auxiliary::expression_child::FormatJsxExpressionChild,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::jsx::auxiliary::expression_child::FormatJsxExpressionChild::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsxFragment> for crate::jsx::tag::fragment::FormatJsxFragment {
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::JsxFragment, f: &mut JsFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsxFragment>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsxFragment {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsxFragment,
        crate::jsx::tag::fragment::FormatJsxFragment,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::jsx::tag::fragment::FormatJsxFragment::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsxFragment {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsxFragment,
        crate::jsx::tag::fragment::FormatJsxFragment,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::jsx::tag::fragment::FormatJsxFragment::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsxMemberName>
    for crate::jsx::objects::member_name::FormatJsxMemberName
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::JsxMemberName, f: &mut JsFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsxMemberName>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsxMemberName {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsxMemberName,
        crate::jsx::objects::member_name::FormatJsxMemberName,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::jsx::objects::member_name::FormatJsxMemberName::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsxMemberName {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsxMemberName,
        crate::jsx::objects::member_name::FormatJsxMemberName,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::jsx::objects::member_name::FormatJsxMemberName::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsxName> for crate::jsx::auxiliary::name::FormatJsxName {
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::JsxName, f: &mut JsFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsxName>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsxName {
    type Format<'a> =
        FormatRefWithRule<'a, biome_js_syntax::JsxName, crate::jsx::auxiliary::name::FormatJsxName>;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, crate::jsx::auxiliary::name::FormatJsxName::default())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsxName {
    type Format =
        FormatOwnedWithRule<biome_js_syntax::JsxName, crate::jsx::auxiliary::name::FormatJsxName>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, crate::jsx::auxiliary::name::FormatJsxName::default())
    }
}
impl FormatRule<biome_js_syntax::JsxNamespaceName>
    for crate::jsx::auxiliary::namespace_name::FormatJsxNamespaceName
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsxNamespaceName,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsxNamespaceName>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsxNamespaceName {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsxNamespaceName,
        crate::jsx::auxiliary::namespace_name::FormatJsxNamespaceName,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::jsx::auxiliary::namespace_name::FormatJsxNamespaceName::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsxNamespaceName {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsxNamespaceName,
        crate::jsx::auxiliary::namespace_name::FormatJsxNamespaceName,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::jsx::auxiliary::namespace_name::FormatJsxNamespaceName::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsxOpeningElement>
    for crate::jsx::tag::opening_element::FormatJsxOpeningElement
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsxOpeningElement,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsxOpeningElement>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsxOpeningElement {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsxOpeningElement,
        crate::jsx::tag::opening_element::FormatJsxOpeningElement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::jsx::tag::opening_element::FormatJsxOpeningElement::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsxOpeningElement {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsxOpeningElement,
        crate::jsx::tag::opening_element::FormatJsxOpeningElement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::jsx::tag::opening_element::FormatJsxOpeningElement::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsxOpeningFragment>
    for crate::jsx::tag::opening_fragment::FormatJsxOpeningFragment
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsxOpeningFragment,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsxOpeningFragment>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsxOpeningFragment {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsxOpeningFragment,
        crate::jsx::tag::opening_fragment::FormatJsxOpeningFragment,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::jsx::tag::opening_fragment::FormatJsxOpeningFragment::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsxOpeningFragment {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsxOpeningFragment,
        crate::jsx::tag::opening_fragment::FormatJsxOpeningFragment,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::jsx::tag::opening_fragment::FormatJsxOpeningFragment::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsxReferenceIdentifier>
    for crate::jsx::auxiliary::reference_identifier::FormatJsxReferenceIdentifier
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsxReferenceIdentifier,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsxReferenceIdentifier>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsxReferenceIdentifier {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsxReferenceIdentifier,
        crate::jsx::auxiliary::reference_identifier::FormatJsxReferenceIdentifier,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::jsx::auxiliary::reference_identifier::FormatJsxReferenceIdentifier::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsxReferenceIdentifier {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsxReferenceIdentifier,
        crate::jsx::auxiliary::reference_identifier::FormatJsxReferenceIdentifier,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::jsx::auxiliary::reference_identifier::FormatJsxReferenceIdentifier::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsxSelfClosingElement>
    for crate::jsx::tag::self_closing_element::FormatJsxSelfClosingElement
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsxSelfClosingElement,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsxSelfClosingElement>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsxSelfClosingElement {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsxSelfClosingElement,
        crate::jsx::tag::self_closing_element::FormatJsxSelfClosingElement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::jsx::tag::self_closing_element::FormatJsxSelfClosingElement::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsxSelfClosingElement {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsxSelfClosingElement,
        crate::jsx::tag::self_closing_element::FormatJsxSelfClosingElement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::jsx::tag::self_closing_element::FormatJsxSelfClosingElement::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsxSpreadAttribute>
    for crate::jsx::attribute::spread_attribute::FormatJsxSpreadAttribute
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsxSpreadAttribute,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsxSpreadAttribute>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsxSpreadAttribute {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsxSpreadAttribute,
        crate::jsx::attribute::spread_attribute::FormatJsxSpreadAttribute,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::jsx::attribute::spread_attribute::FormatJsxSpreadAttribute::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsxSpreadAttribute {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsxSpreadAttribute,
        crate::jsx::attribute::spread_attribute::FormatJsxSpreadAttribute,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::jsx::attribute::spread_attribute::FormatJsxSpreadAttribute::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsxSpreadChild>
    for crate::jsx::auxiliary::spread_child::FormatJsxSpreadChild
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::JsxSpreadChild, f: &mut JsFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsxSpreadChild>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsxSpreadChild {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsxSpreadChild,
        crate::jsx::auxiliary::spread_child::FormatJsxSpreadChild,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::jsx::auxiliary::spread_child::FormatJsxSpreadChild::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsxSpreadChild {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsxSpreadChild,
        crate::jsx::auxiliary::spread_child::FormatJsxSpreadChild,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::jsx::auxiliary::spread_child::FormatJsxSpreadChild::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsxString> for crate::jsx::auxiliary::string::FormatJsxString {
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::JsxString, f: &mut JsFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsxString>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsxString {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsxString,
        crate::jsx::auxiliary::string::FormatJsxString,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::jsx::auxiliary::string::FormatJsxString::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsxString {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsxString,
        crate::jsx::auxiliary::string::FormatJsxString,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::jsx::auxiliary::string::FormatJsxString::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsxTagExpression>
    for crate::jsx::expressions::tag_expression::FormatJsxTagExpression
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsxTagExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsxTagExpression>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsxTagExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsxTagExpression,
        crate::jsx::expressions::tag_expression::FormatJsxTagExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::jsx::expressions::tag_expression::FormatJsxTagExpression::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsxTagExpression {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsxTagExpression,
        crate::jsx::expressions::tag_expression::FormatJsxTagExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::jsx::expressions::tag_expression::FormatJsxTagExpression::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsxText> for crate::jsx::auxiliary::text::FormatJsxText {
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::JsxText, f: &mut JsFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::JsxText>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsxText {
    type Format<'a> =
        FormatRefWithRule<'a, biome_js_syntax::JsxText, crate::jsx::auxiliary::text::FormatJsxText>;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, crate::jsx::auxiliary::text::FormatJsxText::default())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsxText {
    type Format =
        FormatOwnedWithRule<biome_js_syntax::JsxText, crate::jsx::auxiliary::text::FormatJsxText>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, crate::jsx::auxiliary::text::FormatJsxText::default())
    }
}
impl FormatRule<biome_js_syntax::TsAbstractModifier>
    for crate::ts::auxiliary::abstract_modifier::FormatTsAbstractModifier
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsAbstractModifier,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsAbstractModifier>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsAbstractModifier {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsAbstractModifier,
        crate::ts::auxiliary::abstract_modifier::FormatTsAbstractModifier,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::auxiliary::abstract_modifier::FormatTsAbstractModifier::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsAbstractModifier {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsAbstractModifier,
        crate::ts::auxiliary::abstract_modifier::FormatTsAbstractModifier,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::auxiliary::abstract_modifier::FormatTsAbstractModifier::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsAccessibilityModifier>
    for crate::ts::auxiliary::accessibility_modifier::FormatTsAccessibilityModifier
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsAccessibilityModifier,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsAccessibilityModifier>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsAccessibilityModifier {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsAccessibilityModifier,
        crate::ts::auxiliary::accessibility_modifier::FormatTsAccessibilityModifier,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::auxiliary::accessibility_modifier::FormatTsAccessibilityModifier::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsAccessibilityModifier {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsAccessibilityModifier,
        crate::ts::auxiliary::accessibility_modifier::FormatTsAccessibilityModifier,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::auxiliary::accessibility_modifier::FormatTsAccessibilityModifier::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsAnyType> for crate::ts::types::any_type::FormatTsAnyType {
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::TsAnyType, f: &mut JsFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsAnyType>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsAnyType {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsAnyType,
        crate::ts::types::any_type::FormatTsAnyType,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, crate::ts::types::any_type::FormatTsAnyType::default())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsAnyType {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsAnyType,
        crate::ts::types::any_type::FormatTsAnyType,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, crate::ts::types::any_type::FormatTsAnyType::default())
    }
}
impl FormatRule<biome_js_syntax::TsArrayType> for crate::ts::types::array_type::FormatTsArrayType {
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::TsArrayType, f: &mut JsFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsArrayType>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsArrayType {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsArrayType,
        crate::ts::types::array_type::FormatTsArrayType,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::types::array_type::FormatTsArrayType::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsArrayType {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsArrayType,
        crate::ts::types::array_type::FormatTsArrayType,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::types::array_type::FormatTsArrayType::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsAsAssignment>
    for crate::ts::assignments::as_assignment::FormatTsAsAssignment
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::TsAsAssignment, f: &mut JsFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsAsAssignment>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsAsAssignment {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsAsAssignment,
        crate::ts::assignments::as_assignment::FormatTsAsAssignment,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::assignments::as_assignment::FormatTsAsAssignment::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsAsAssignment {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsAsAssignment,
        crate::ts::assignments::as_assignment::FormatTsAsAssignment,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::assignments::as_assignment::FormatTsAsAssignment::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsAsExpression>
    for crate::ts::expressions::as_expression::FormatTsAsExpression
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::TsAsExpression, f: &mut JsFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsAsExpression>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsAsExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsAsExpression,
        crate::ts::expressions::as_expression::FormatTsAsExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::expressions::as_expression::FormatTsAsExpression::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsAsExpression {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsAsExpression,
        crate::ts::expressions::as_expression::FormatTsAsExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::expressions::as_expression::FormatTsAsExpression::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsAssertsCondition>
    for crate::ts::auxiliary::asserts_condition::FormatTsAssertsCondition
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsAssertsCondition,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsAssertsCondition>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsAssertsCondition {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsAssertsCondition,
        crate::ts::auxiliary::asserts_condition::FormatTsAssertsCondition,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::auxiliary::asserts_condition::FormatTsAssertsCondition::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsAssertsCondition {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsAssertsCondition,
        crate::ts::auxiliary::asserts_condition::FormatTsAssertsCondition,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::auxiliary::asserts_condition::FormatTsAssertsCondition::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsAssertsReturnType>
    for crate::ts::types::asserts_return_type::FormatTsAssertsReturnType
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsAssertsReturnType,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsAssertsReturnType>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsAssertsReturnType {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsAssertsReturnType,
        crate::ts::types::asserts_return_type::FormatTsAssertsReturnType,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::types::asserts_return_type::FormatTsAssertsReturnType::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsAssertsReturnType {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsAssertsReturnType,
        crate::ts::types::asserts_return_type::FormatTsAssertsReturnType,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::types::asserts_return_type::FormatTsAssertsReturnType::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsBigintLiteralType>
    for crate::ts::types::bigint_literal_type::FormatTsBigintLiteralType
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsBigintLiteralType,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsBigintLiteralType>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsBigintLiteralType {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsBigintLiteralType,
        crate::ts::types::bigint_literal_type::FormatTsBigintLiteralType,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::types::bigint_literal_type::FormatTsBigintLiteralType::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsBigintLiteralType {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsBigintLiteralType,
        crate::ts::types::bigint_literal_type::FormatTsBigintLiteralType,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::types::bigint_literal_type::FormatTsBigintLiteralType::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsBigintType>
    for crate::ts::types::bigint_type::FormatTsBigintType
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::TsBigintType, f: &mut JsFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsBigintType>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsBigintType {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsBigintType,
        crate::ts::types::bigint_type::FormatTsBigintType,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::types::bigint_type::FormatTsBigintType::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsBigintType {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsBigintType,
        crate::ts::types::bigint_type::FormatTsBigintType,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::types::bigint_type::FormatTsBigintType::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsBooleanLiteralType>
    for crate::ts::types::boolean_literal_type::FormatTsBooleanLiteralType
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsBooleanLiteralType,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsBooleanLiteralType>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsBooleanLiteralType {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsBooleanLiteralType,
        crate::ts::types::boolean_literal_type::FormatTsBooleanLiteralType,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::types::boolean_literal_type::FormatTsBooleanLiteralType::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsBooleanLiteralType {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsBooleanLiteralType,
        crate::ts::types::boolean_literal_type::FormatTsBooleanLiteralType,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::types::boolean_literal_type::FormatTsBooleanLiteralType::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsBooleanType>
    for crate::ts::types::boolean_type::FormatTsBooleanType
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::TsBooleanType, f: &mut JsFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsBooleanType>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsBooleanType {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsBooleanType,
        crate::ts::types::boolean_type::FormatTsBooleanType,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::types::boolean_type::FormatTsBooleanType::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsBooleanType {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsBooleanType,
        crate::ts::types::boolean_type::FormatTsBooleanType,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::types::boolean_type::FormatTsBooleanType::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsCallSignatureTypeMember>
    for crate::ts::auxiliary::call_signature_type_member::FormatTsCallSignatureTypeMember
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsCallSignatureTypeMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsCallSignatureTypeMember>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsCallSignatureTypeMember {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsCallSignatureTypeMember,
        crate::ts::auxiliary::call_signature_type_member::FormatTsCallSignatureTypeMember,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: ts :: auxiliary :: call_signature_type_member :: FormatTsCallSignatureTypeMember :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsCallSignatureTypeMember {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsCallSignatureTypeMember,
        crate::ts::auxiliary::call_signature_type_member::FormatTsCallSignatureTypeMember,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: ts :: auxiliary :: call_signature_type_member :: FormatTsCallSignatureTypeMember :: default ())
    }
}
impl FormatRule<biome_js_syntax::TsConditionalType>
    for crate::ts::types::conditional_type::FormatTsConditionalType
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsConditionalType,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsConditionalType>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsConditionalType {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsConditionalType,
        crate::ts::types::conditional_type::FormatTsConditionalType,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::types::conditional_type::FormatTsConditionalType::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsConditionalType {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsConditionalType,
        crate::ts::types::conditional_type::FormatTsConditionalType,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::types::conditional_type::FormatTsConditionalType::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsConstModifier>
    for crate::ts::auxiliary::const_modifier::FormatTsConstModifier
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsConstModifier,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsConstModifier>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsConstModifier {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsConstModifier,
        crate::ts::auxiliary::const_modifier::FormatTsConstModifier,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::auxiliary::const_modifier::FormatTsConstModifier::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsConstModifier {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsConstModifier,
        crate::ts::auxiliary::const_modifier::FormatTsConstModifier,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::auxiliary::const_modifier::FormatTsConstModifier::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsConstructSignatureTypeMember>
    for crate::ts::auxiliary::construct_signature_type_member::FormatTsConstructSignatureTypeMember
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsConstructSignatureTypeMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsConstructSignatureTypeMember>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsConstructSignatureTypeMember {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsConstructSignatureTypeMember,
        crate::ts::auxiliary::construct_signature_type_member::FormatTsConstructSignatureTypeMember,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: ts :: auxiliary :: construct_signature_type_member :: FormatTsConstructSignatureTypeMember :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsConstructSignatureTypeMember {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsConstructSignatureTypeMember,
        crate::ts::auxiliary::construct_signature_type_member::FormatTsConstructSignatureTypeMember,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: ts :: auxiliary :: construct_signature_type_member :: FormatTsConstructSignatureTypeMember :: default ())
    }
}
impl FormatRule < biome_js_syntax :: TsConstructorSignatureClassMember > for crate :: ts :: classes :: constructor_signature_class_member :: FormatTsConstructorSignatureClassMember { type Context = JsFormatContext ; # [inline (always)] fn fmt (& self , node : & biome_js_syntax :: TsConstructorSignatureClassMember , f : & mut JsFormatter) -> FormatResult < () > { FormatNodeRule :: < biome_js_syntax :: TsConstructorSignatureClassMember > :: fmt (self , node , f) } }
impl AsFormat<JsFormatContext> for biome_js_syntax::TsConstructorSignatureClassMember {
    type Format < 'a > = FormatRefWithRule < 'a , biome_js_syntax :: TsConstructorSignatureClassMember , crate :: ts :: classes :: constructor_signature_class_member :: FormatTsConstructorSignatureClassMember > ;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: ts :: classes :: constructor_signature_class_member :: FormatTsConstructorSignatureClassMember :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsConstructorSignatureClassMember {
    type Format = FormatOwnedWithRule < biome_js_syntax :: TsConstructorSignatureClassMember , crate :: ts :: classes :: constructor_signature_class_member :: FormatTsConstructorSignatureClassMember > ;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: ts :: classes :: constructor_signature_class_member :: FormatTsConstructorSignatureClassMember :: default ())
    }
}
impl FormatRule<biome_js_syntax::TsConstructorType>
    for crate::ts::types::constructor_type::FormatTsConstructorType
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsConstructorType,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsConstructorType>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsConstructorType {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsConstructorType,
        crate::ts::types::constructor_type::FormatTsConstructorType,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::types::constructor_type::FormatTsConstructorType::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsConstructorType {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsConstructorType,
        crate::ts::types::constructor_type::FormatTsConstructorType,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::types::constructor_type::FormatTsConstructorType::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsDeclarationModule>
    for crate::ts::auxiliary::declaration_module::FormatTsDeclarationModule
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsDeclarationModule,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsDeclarationModule>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsDeclarationModule {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsDeclarationModule,
        crate::ts::auxiliary::declaration_module::FormatTsDeclarationModule,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::auxiliary::declaration_module::FormatTsDeclarationModule::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsDeclarationModule {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsDeclarationModule,
        crate::ts::auxiliary::declaration_module::FormatTsDeclarationModule,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::auxiliary::declaration_module::FormatTsDeclarationModule::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsDeclareFunctionDeclaration>
    for crate::ts::declarations::declare_function_declaration::FormatTsDeclareFunctionDeclaration
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsDeclareFunctionDeclaration,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsDeclareFunctionDeclaration>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsDeclareFunctionDeclaration {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsDeclareFunctionDeclaration,
        crate::ts::declarations::declare_function_declaration::FormatTsDeclareFunctionDeclaration,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: ts :: declarations :: declare_function_declaration :: FormatTsDeclareFunctionDeclaration :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsDeclareFunctionDeclaration {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsDeclareFunctionDeclaration,
        crate::ts::declarations::declare_function_declaration::FormatTsDeclareFunctionDeclaration,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: ts :: declarations :: declare_function_declaration :: FormatTsDeclareFunctionDeclaration :: default ())
    }
}
impl FormatRule < biome_js_syntax :: TsDeclareFunctionExportDefaultDeclaration > for crate :: ts :: declarations :: declare_function_export_default_declaration :: FormatTsDeclareFunctionExportDefaultDeclaration { type Context = JsFormatContext ; # [inline (always)] fn fmt (& self , node : & biome_js_syntax :: TsDeclareFunctionExportDefaultDeclaration , f : & mut JsFormatter) -> FormatResult < () > { FormatNodeRule :: < biome_js_syntax :: TsDeclareFunctionExportDefaultDeclaration > :: fmt (self , node , f) } }
impl AsFormat<JsFormatContext> for biome_js_syntax::TsDeclareFunctionExportDefaultDeclaration {
    type Format < 'a > = FormatRefWithRule < 'a , biome_js_syntax :: TsDeclareFunctionExportDefaultDeclaration , crate :: ts :: declarations :: declare_function_export_default_declaration :: FormatTsDeclareFunctionExportDefaultDeclaration > ;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: ts :: declarations :: declare_function_export_default_declaration :: FormatTsDeclareFunctionExportDefaultDeclaration :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsDeclareFunctionExportDefaultDeclaration {
    type Format = FormatOwnedWithRule < biome_js_syntax :: TsDeclareFunctionExportDefaultDeclaration , crate :: ts :: declarations :: declare_function_export_default_declaration :: FormatTsDeclareFunctionExportDefaultDeclaration > ;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: ts :: declarations :: declare_function_export_default_declaration :: FormatTsDeclareFunctionExportDefaultDeclaration :: default ())
    }
}
impl FormatRule<biome_js_syntax::TsDeclareModifier>
    for crate::ts::auxiliary::declare_modifier::FormatTsDeclareModifier
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsDeclareModifier,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsDeclareModifier>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsDeclareModifier {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsDeclareModifier,
        crate::ts::auxiliary::declare_modifier::FormatTsDeclareModifier,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::auxiliary::declare_modifier::FormatTsDeclareModifier::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsDeclareModifier {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsDeclareModifier,
        crate::ts::auxiliary::declare_modifier::FormatTsDeclareModifier,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::auxiliary::declare_modifier::FormatTsDeclareModifier::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsDeclareStatement>
    for crate::ts::statements::declare_statement::FormatTsDeclareStatement
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsDeclareStatement,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsDeclareStatement>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsDeclareStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsDeclareStatement,
        crate::ts::statements::declare_statement::FormatTsDeclareStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::statements::declare_statement::FormatTsDeclareStatement::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsDeclareStatement {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsDeclareStatement,
        crate::ts::statements::declare_statement::FormatTsDeclareStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::statements::declare_statement::FormatTsDeclareStatement::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsDefaultTypeClause>
    for crate::ts::auxiliary::default_type_clause::FormatTsDefaultTypeClause
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsDefaultTypeClause,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsDefaultTypeClause>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsDefaultTypeClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsDefaultTypeClause,
        crate::ts::auxiliary::default_type_clause::FormatTsDefaultTypeClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::auxiliary::default_type_clause::FormatTsDefaultTypeClause::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsDefaultTypeClause {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsDefaultTypeClause,
        crate::ts::auxiliary::default_type_clause::FormatTsDefaultTypeClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::auxiliary::default_type_clause::FormatTsDefaultTypeClause::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsDefinitePropertyAnnotation>
    for crate::ts::auxiliary::definite_property_annotation::FormatTsDefinitePropertyAnnotation
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsDefinitePropertyAnnotation,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsDefinitePropertyAnnotation>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsDefinitePropertyAnnotation {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsDefinitePropertyAnnotation,
        crate::ts::auxiliary::definite_property_annotation::FormatTsDefinitePropertyAnnotation,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: ts :: auxiliary :: definite_property_annotation :: FormatTsDefinitePropertyAnnotation :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsDefinitePropertyAnnotation {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsDefinitePropertyAnnotation,
        crate::ts::auxiliary::definite_property_annotation::FormatTsDefinitePropertyAnnotation,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: ts :: auxiliary :: definite_property_annotation :: FormatTsDefinitePropertyAnnotation :: default ())
    }
}
impl FormatRule<biome_js_syntax::TsDefiniteVariableAnnotation>
    for crate::ts::auxiliary::definite_variable_annotation::FormatTsDefiniteVariableAnnotation
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsDefiniteVariableAnnotation,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsDefiniteVariableAnnotation>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsDefiniteVariableAnnotation {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsDefiniteVariableAnnotation,
        crate::ts::auxiliary::definite_variable_annotation::FormatTsDefiniteVariableAnnotation,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: ts :: auxiliary :: definite_variable_annotation :: FormatTsDefiniteVariableAnnotation :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsDefiniteVariableAnnotation {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsDefiniteVariableAnnotation,
        crate::ts::auxiliary::definite_variable_annotation::FormatTsDefiniteVariableAnnotation,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: ts :: auxiliary :: definite_variable_annotation :: FormatTsDefiniteVariableAnnotation :: default ())
    }
}
impl FormatRule < biome_js_syntax :: TsEmptyExternalModuleDeclarationBody > for crate :: ts :: auxiliary :: empty_external_module_declaration_body :: FormatTsEmptyExternalModuleDeclarationBody { type Context = JsFormatContext ; # [inline (always)] fn fmt (& self , node : & biome_js_syntax :: TsEmptyExternalModuleDeclarationBody , f : & mut JsFormatter) -> FormatResult < () > { FormatNodeRule :: < biome_js_syntax :: TsEmptyExternalModuleDeclarationBody > :: fmt (self , node , f) } }
impl AsFormat<JsFormatContext> for biome_js_syntax::TsEmptyExternalModuleDeclarationBody {
    type Format < 'a > = FormatRefWithRule < 'a , biome_js_syntax :: TsEmptyExternalModuleDeclarationBody , crate :: ts :: auxiliary :: empty_external_module_declaration_body :: FormatTsEmptyExternalModuleDeclarationBody > ;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: ts :: auxiliary :: empty_external_module_declaration_body :: FormatTsEmptyExternalModuleDeclarationBody :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsEmptyExternalModuleDeclarationBody {
    type Format = FormatOwnedWithRule < biome_js_syntax :: TsEmptyExternalModuleDeclarationBody , crate :: ts :: auxiliary :: empty_external_module_declaration_body :: FormatTsEmptyExternalModuleDeclarationBody > ;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: ts :: auxiliary :: empty_external_module_declaration_body :: FormatTsEmptyExternalModuleDeclarationBody :: default ())
    }
}
impl FormatRule<biome_js_syntax::TsEnumDeclaration>
    for crate::ts::declarations::enum_declaration::FormatTsEnumDeclaration
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsEnumDeclaration,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsEnumDeclaration>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsEnumDeclaration {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsEnumDeclaration,
        crate::ts::declarations::enum_declaration::FormatTsEnumDeclaration,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::declarations::enum_declaration::FormatTsEnumDeclaration::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsEnumDeclaration {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsEnumDeclaration,
        crate::ts::declarations::enum_declaration::FormatTsEnumDeclaration,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::declarations::enum_declaration::FormatTsEnumDeclaration::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsEnumMember>
    for crate::ts::auxiliary::enum_member::FormatTsEnumMember
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::TsEnumMember, f: &mut JsFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsEnumMember>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsEnumMember {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsEnumMember,
        crate::ts::auxiliary::enum_member::FormatTsEnumMember,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::auxiliary::enum_member::FormatTsEnumMember::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsEnumMember {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsEnumMember,
        crate::ts::auxiliary::enum_member::FormatTsEnumMember,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::auxiliary::enum_member::FormatTsEnumMember::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsExportAsNamespaceClause>
    for crate::ts::module::export_as_namespace_clause::FormatTsExportAsNamespaceClause
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsExportAsNamespaceClause,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsExportAsNamespaceClause>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsExportAsNamespaceClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsExportAsNamespaceClause,
        crate::ts::module::export_as_namespace_clause::FormatTsExportAsNamespaceClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::module::export_as_namespace_clause::FormatTsExportAsNamespaceClause::default(
            ),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsExportAsNamespaceClause {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsExportAsNamespaceClause,
        crate::ts::module::export_as_namespace_clause::FormatTsExportAsNamespaceClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::module::export_as_namespace_clause::FormatTsExportAsNamespaceClause::default(
            ),
        )
    }
}
impl FormatRule<biome_js_syntax::TsExportAssignmentClause>
    for crate::ts::module::export_assignment_clause::FormatTsExportAssignmentClause
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsExportAssignmentClause,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsExportAssignmentClause>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsExportAssignmentClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsExportAssignmentClause,
        crate::ts::module::export_assignment_clause::FormatTsExportAssignmentClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::module::export_assignment_clause::FormatTsExportAssignmentClause::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsExportAssignmentClause {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsExportAssignmentClause,
        crate::ts::module::export_assignment_clause::FormatTsExportAssignmentClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::module::export_assignment_clause::FormatTsExportAssignmentClause::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsExportDeclareClause>
    for crate::ts::module::export_declare_clause::FormatTsExportDeclareClause
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsExportDeclareClause,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsExportDeclareClause>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsExportDeclareClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsExportDeclareClause,
        crate::ts::module::export_declare_clause::FormatTsExportDeclareClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::module::export_declare_clause::FormatTsExportDeclareClause::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsExportDeclareClause {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsExportDeclareClause,
        crate::ts::module::export_declare_clause::FormatTsExportDeclareClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::module::export_declare_clause::FormatTsExportDeclareClause::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsExtendsClause>
    for crate::ts::classes::extends_clause::FormatTsExtendsClause
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsExtendsClause,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsExtendsClause>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsExtendsClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsExtendsClause,
        crate::ts::classes::extends_clause::FormatTsExtendsClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::classes::extends_clause::FormatTsExtendsClause::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsExtendsClause {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsExtendsClause,
        crate::ts::classes::extends_clause::FormatTsExtendsClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::classes::extends_clause::FormatTsExtendsClause::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsExternalModuleDeclaration>
    for crate::ts::declarations::external_module_declaration::FormatTsExternalModuleDeclaration
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsExternalModuleDeclaration,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsExternalModuleDeclaration>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsExternalModuleDeclaration {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsExternalModuleDeclaration,
        crate::ts::declarations::external_module_declaration::FormatTsExternalModuleDeclaration,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: ts :: declarations :: external_module_declaration :: FormatTsExternalModuleDeclaration :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsExternalModuleDeclaration {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsExternalModuleDeclaration,
        crate::ts::declarations::external_module_declaration::FormatTsExternalModuleDeclaration,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: ts :: declarations :: external_module_declaration :: FormatTsExternalModuleDeclaration :: default ())
    }
}
impl FormatRule<biome_js_syntax::TsExternalModuleReference>
    for crate::ts::auxiliary::external_module_reference::FormatTsExternalModuleReference
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsExternalModuleReference,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsExternalModuleReference>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsExternalModuleReference {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsExternalModuleReference,
        crate::ts::auxiliary::external_module_reference::FormatTsExternalModuleReference,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: ts :: auxiliary :: external_module_reference :: FormatTsExternalModuleReference :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsExternalModuleReference {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsExternalModuleReference,
        crate::ts::auxiliary::external_module_reference::FormatTsExternalModuleReference,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: ts :: auxiliary :: external_module_reference :: FormatTsExternalModuleReference :: default ())
    }
}
impl FormatRule<biome_js_syntax::TsFunctionType>
    for crate::ts::types::function_type::FormatTsFunctionType
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::TsFunctionType, f: &mut JsFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsFunctionType>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsFunctionType {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsFunctionType,
        crate::ts::types::function_type::FormatTsFunctionType,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::types::function_type::FormatTsFunctionType::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsFunctionType {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsFunctionType,
        crate::ts::types::function_type::FormatTsFunctionType,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::types::function_type::FormatTsFunctionType::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsGetterSignatureClassMember>
    for crate::ts::classes::getter_signature_class_member::FormatTsGetterSignatureClassMember
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsGetterSignatureClassMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsGetterSignatureClassMember>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsGetterSignatureClassMember {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsGetterSignatureClassMember,
        crate::ts::classes::getter_signature_class_member::FormatTsGetterSignatureClassMember,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: ts :: classes :: getter_signature_class_member :: FormatTsGetterSignatureClassMember :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsGetterSignatureClassMember {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsGetterSignatureClassMember,
        crate::ts::classes::getter_signature_class_member::FormatTsGetterSignatureClassMember,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: ts :: classes :: getter_signature_class_member :: FormatTsGetterSignatureClassMember :: default ())
    }
}
impl FormatRule<biome_js_syntax::TsGetterSignatureTypeMember>
    for crate::ts::auxiliary::getter_signature_type_member::FormatTsGetterSignatureTypeMember
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsGetterSignatureTypeMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsGetterSignatureTypeMember>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsGetterSignatureTypeMember {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsGetterSignatureTypeMember,
        crate::ts::auxiliary::getter_signature_type_member::FormatTsGetterSignatureTypeMember,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: ts :: auxiliary :: getter_signature_type_member :: FormatTsGetterSignatureTypeMember :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsGetterSignatureTypeMember {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsGetterSignatureTypeMember,
        crate::ts::auxiliary::getter_signature_type_member::FormatTsGetterSignatureTypeMember,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: ts :: auxiliary :: getter_signature_type_member :: FormatTsGetterSignatureTypeMember :: default ())
    }
}
impl FormatRule<biome_js_syntax::TsGlobalDeclaration>
    for crate::ts::declarations::global_declaration::FormatTsGlobalDeclaration
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsGlobalDeclaration,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsGlobalDeclaration>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsGlobalDeclaration {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsGlobalDeclaration,
        crate::ts::declarations::global_declaration::FormatTsGlobalDeclaration,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::declarations::global_declaration::FormatTsGlobalDeclaration::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsGlobalDeclaration {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsGlobalDeclaration,
        crate::ts::declarations::global_declaration::FormatTsGlobalDeclaration,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::declarations::global_declaration::FormatTsGlobalDeclaration::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsIdentifierBinding>
    for crate::ts::bindings::identifier_binding::FormatTsIdentifierBinding
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsIdentifierBinding,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsIdentifierBinding>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsIdentifierBinding {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsIdentifierBinding,
        crate::ts::bindings::identifier_binding::FormatTsIdentifierBinding,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::bindings::identifier_binding::FormatTsIdentifierBinding::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsIdentifierBinding {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsIdentifierBinding,
        crate::ts::bindings::identifier_binding::FormatTsIdentifierBinding,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::bindings::identifier_binding::FormatTsIdentifierBinding::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsImplementsClause>
    for crate::ts::auxiliary::implements_clause::FormatTsImplementsClause
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsImplementsClause,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsImplementsClause>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsImplementsClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsImplementsClause,
        crate::ts::auxiliary::implements_clause::FormatTsImplementsClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::auxiliary::implements_clause::FormatTsImplementsClause::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsImplementsClause {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsImplementsClause,
        crate::ts::auxiliary::implements_clause::FormatTsImplementsClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::auxiliary::implements_clause::FormatTsImplementsClause::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsImportEqualsDeclaration>
    for crate::ts::declarations::import_equals_declaration::FormatTsImportEqualsDeclaration
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsImportEqualsDeclaration,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsImportEqualsDeclaration>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsImportEqualsDeclaration {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsImportEqualsDeclaration,
        crate::ts::declarations::import_equals_declaration::FormatTsImportEqualsDeclaration,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: ts :: declarations :: import_equals_declaration :: FormatTsImportEqualsDeclaration :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsImportEqualsDeclaration {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsImportEqualsDeclaration,
        crate::ts::declarations::import_equals_declaration::FormatTsImportEqualsDeclaration,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: ts :: declarations :: import_equals_declaration :: FormatTsImportEqualsDeclaration :: default ())
    }
}
impl FormatRule<biome_js_syntax::TsImportType>
    for crate::ts::module::import_type::FormatTsImportType
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::TsImportType, f: &mut JsFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsImportType>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsImportType {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsImportType,
        crate::ts::module::import_type::FormatTsImportType,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::module::import_type::FormatTsImportType::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsImportType {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsImportType,
        crate::ts::module::import_type::FormatTsImportType,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::module::import_type::FormatTsImportType::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsImportTypeArguments>
    for crate::ts::expressions::import_type_arguments::FormatTsImportTypeArguments
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsImportTypeArguments,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsImportTypeArguments>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsImportTypeArguments {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsImportTypeArguments,
        crate::ts::expressions::import_type_arguments::FormatTsImportTypeArguments,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::expressions::import_type_arguments::FormatTsImportTypeArguments::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsImportTypeArguments {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsImportTypeArguments,
        crate::ts::expressions::import_type_arguments::FormatTsImportTypeArguments,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::expressions::import_type_arguments::FormatTsImportTypeArguments::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsImportTypeAssertion>
    for crate::ts::module::import_type_assertion::FormatTsImportTypeAssertion
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsImportTypeAssertion,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsImportTypeAssertion>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsImportTypeAssertion {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsImportTypeAssertion,
        crate::ts::module::import_type_assertion::FormatTsImportTypeAssertion,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::module::import_type_assertion::FormatTsImportTypeAssertion::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsImportTypeAssertion {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsImportTypeAssertion,
        crate::ts::module::import_type_assertion::FormatTsImportTypeAssertion,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::module::import_type_assertion::FormatTsImportTypeAssertion::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsImportTypeAssertionBlock>
    for crate::ts::module::import_type_assertion_block::FormatTsImportTypeAssertionBlock
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsImportTypeAssertionBlock,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsImportTypeAssertionBlock>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsImportTypeAssertionBlock {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsImportTypeAssertionBlock,
        crate::ts::module::import_type_assertion_block::FormatTsImportTypeAssertionBlock,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: ts :: module :: import_type_assertion_block :: FormatTsImportTypeAssertionBlock :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsImportTypeAssertionBlock {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsImportTypeAssertionBlock,
        crate::ts::module::import_type_assertion_block::FormatTsImportTypeAssertionBlock,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: ts :: module :: import_type_assertion_block :: FormatTsImportTypeAssertionBlock :: default ())
    }
}
impl FormatRule<biome_js_syntax::TsImportTypeQualifier>
    for crate::ts::module::import_type_qualifier::FormatTsImportTypeQualifier
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsImportTypeQualifier,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsImportTypeQualifier>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsImportTypeQualifier {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsImportTypeQualifier,
        crate::ts::module::import_type_qualifier::FormatTsImportTypeQualifier,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::module::import_type_qualifier::FormatTsImportTypeQualifier::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsImportTypeQualifier {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsImportTypeQualifier,
        crate::ts::module::import_type_qualifier::FormatTsImportTypeQualifier,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::module::import_type_qualifier::FormatTsImportTypeQualifier::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsInModifier>
    for crate::ts::auxiliary::in_modifier::FormatTsInModifier
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::TsInModifier, f: &mut JsFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsInModifier>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsInModifier {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsInModifier,
        crate::ts::auxiliary::in_modifier::FormatTsInModifier,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::auxiliary::in_modifier::FormatTsInModifier::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsInModifier {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsInModifier,
        crate::ts::auxiliary::in_modifier::FormatTsInModifier,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::auxiliary::in_modifier::FormatTsInModifier::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsIndexSignatureClassMember>
    for crate::ts::classes::index_signature_class_member::FormatTsIndexSignatureClassMember
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsIndexSignatureClassMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsIndexSignatureClassMember>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsIndexSignatureClassMember {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsIndexSignatureClassMember,
        crate::ts::classes::index_signature_class_member::FormatTsIndexSignatureClassMember,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: ts :: classes :: index_signature_class_member :: FormatTsIndexSignatureClassMember :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsIndexSignatureClassMember {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsIndexSignatureClassMember,
        crate::ts::classes::index_signature_class_member::FormatTsIndexSignatureClassMember,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: ts :: classes :: index_signature_class_member :: FormatTsIndexSignatureClassMember :: default ())
    }
}
impl FormatRule<biome_js_syntax::TsIndexSignatureParameter>
    for crate::ts::bindings::index_signature_parameter::FormatTsIndexSignatureParameter
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsIndexSignatureParameter,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsIndexSignatureParameter>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsIndexSignatureParameter {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsIndexSignatureParameter,
        crate::ts::bindings::index_signature_parameter::FormatTsIndexSignatureParameter,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: ts :: bindings :: index_signature_parameter :: FormatTsIndexSignatureParameter :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsIndexSignatureParameter {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsIndexSignatureParameter,
        crate::ts::bindings::index_signature_parameter::FormatTsIndexSignatureParameter,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: ts :: bindings :: index_signature_parameter :: FormatTsIndexSignatureParameter :: default ())
    }
}
impl FormatRule<biome_js_syntax::TsIndexSignatureTypeMember>
    for crate::ts::auxiliary::index_signature_type_member::FormatTsIndexSignatureTypeMember
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsIndexSignatureTypeMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsIndexSignatureTypeMember>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsIndexSignatureTypeMember {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsIndexSignatureTypeMember,
        crate::ts::auxiliary::index_signature_type_member::FormatTsIndexSignatureTypeMember,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: ts :: auxiliary :: index_signature_type_member :: FormatTsIndexSignatureTypeMember :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsIndexSignatureTypeMember {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsIndexSignatureTypeMember,
        crate::ts::auxiliary::index_signature_type_member::FormatTsIndexSignatureTypeMember,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: ts :: auxiliary :: index_signature_type_member :: FormatTsIndexSignatureTypeMember :: default ())
    }
}
impl FormatRule<biome_js_syntax::TsIndexedAccessType>
    for crate::ts::types::indexed_access_type::FormatTsIndexedAccessType
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsIndexedAccessType,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsIndexedAccessType>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsIndexedAccessType {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsIndexedAccessType,
        crate::ts::types::indexed_access_type::FormatTsIndexedAccessType,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::types::indexed_access_type::FormatTsIndexedAccessType::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsIndexedAccessType {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsIndexedAccessType,
        crate::ts::types::indexed_access_type::FormatTsIndexedAccessType,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::types::indexed_access_type::FormatTsIndexedAccessType::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsInferType> for crate::ts::types::infer_type::FormatTsInferType {
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::TsInferType, f: &mut JsFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsInferType>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsInferType {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsInferType,
        crate::ts::types::infer_type::FormatTsInferType,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::types::infer_type::FormatTsInferType::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsInferType {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsInferType,
        crate::ts::types::infer_type::FormatTsInferType,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::types::infer_type::FormatTsInferType::default(),
        )
    }
}
impl FormatRule < biome_js_syntax :: TsInitializedPropertySignatureClassMember > for crate :: ts :: classes :: initialized_property_signature_class_member :: FormatTsInitializedPropertySignatureClassMember { type Context = JsFormatContext ; # [inline (always)] fn fmt (& self , node : & biome_js_syntax :: TsInitializedPropertySignatureClassMember , f : & mut JsFormatter) -> FormatResult < () > { FormatNodeRule :: < biome_js_syntax :: TsInitializedPropertySignatureClassMember > :: fmt (self , node , f) } }
impl AsFormat<JsFormatContext> for biome_js_syntax::TsInitializedPropertySignatureClassMember {
    type Format < 'a > = FormatRefWithRule < 'a , biome_js_syntax :: TsInitializedPropertySignatureClassMember , crate :: ts :: classes :: initialized_property_signature_class_member :: FormatTsInitializedPropertySignatureClassMember > ;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: ts :: classes :: initialized_property_signature_class_member :: FormatTsInitializedPropertySignatureClassMember :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsInitializedPropertySignatureClassMember {
    type Format = FormatOwnedWithRule < biome_js_syntax :: TsInitializedPropertySignatureClassMember , crate :: ts :: classes :: initialized_property_signature_class_member :: FormatTsInitializedPropertySignatureClassMember > ;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: ts :: classes :: initialized_property_signature_class_member :: FormatTsInitializedPropertySignatureClassMember :: default ())
    }
}
impl FormatRule<biome_js_syntax::TsInstantiationExpression>
    for crate::ts::expressions::instantiation_expression::FormatTsInstantiationExpression
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsInstantiationExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsInstantiationExpression>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsInstantiationExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsInstantiationExpression,
        crate::ts::expressions::instantiation_expression::FormatTsInstantiationExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: ts :: expressions :: instantiation_expression :: FormatTsInstantiationExpression :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsInstantiationExpression {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsInstantiationExpression,
        crate::ts::expressions::instantiation_expression::FormatTsInstantiationExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: ts :: expressions :: instantiation_expression :: FormatTsInstantiationExpression :: default ())
    }
}
impl FormatRule<biome_js_syntax::TsInterfaceDeclaration>
    for crate::ts::declarations::interface_declaration::FormatTsInterfaceDeclaration
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsInterfaceDeclaration,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsInterfaceDeclaration>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsInterfaceDeclaration {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsInterfaceDeclaration,
        crate::ts::declarations::interface_declaration::FormatTsInterfaceDeclaration,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::declarations::interface_declaration::FormatTsInterfaceDeclaration::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsInterfaceDeclaration {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsInterfaceDeclaration,
        crate::ts::declarations::interface_declaration::FormatTsInterfaceDeclaration,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::declarations::interface_declaration::FormatTsInterfaceDeclaration::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsIntersectionType>
    for crate::ts::types::intersection_type::FormatTsIntersectionType
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsIntersectionType,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsIntersectionType>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsIntersectionType {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsIntersectionType,
        crate::ts::types::intersection_type::FormatTsIntersectionType,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::types::intersection_type::FormatTsIntersectionType::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsIntersectionType {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsIntersectionType,
        crate::ts::types::intersection_type::FormatTsIntersectionType,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::types::intersection_type::FormatTsIntersectionType::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsLiteralEnumMemberName>
    for crate::ts::objects::literal_enum_member_name::FormatTsLiteralEnumMemberName
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsLiteralEnumMemberName,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsLiteralEnumMemberName>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsLiteralEnumMemberName {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsLiteralEnumMemberName,
        crate::ts::objects::literal_enum_member_name::FormatTsLiteralEnumMemberName,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::objects::literal_enum_member_name::FormatTsLiteralEnumMemberName::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsLiteralEnumMemberName {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsLiteralEnumMemberName,
        crate::ts::objects::literal_enum_member_name::FormatTsLiteralEnumMemberName,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::objects::literal_enum_member_name::FormatTsLiteralEnumMemberName::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsMappedType>
    for crate::ts::types::mapped_type::FormatTsMappedType
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::TsMappedType, f: &mut JsFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsMappedType>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsMappedType {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsMappedType,
        crate::ts::types::mapped_type::FormatTsMappedType,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::types::mapped_type::FormatTsMappedType::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsMappedType {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsMappedType,
        crate::ts::types::mapped_type::FormatTsMappedType,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::types::mapped_type::FormatTsMappedType::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsMappedTypeAsClause>
    for crate::ts::auxiliary::mapped_type_as_clause::FormatTsMappedTypeAsClause
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsMappedTypeAsClause,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsMappedTypeAsClause>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsMappedTypeAsClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsMappedTypeAsClause,
        crate::ts::auxiliary::mapped_type_as_clause::FormatTsMappedTypeAsClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::auxiliary::mapped_type_as_clause::FormatTsMappedTypeAsClause::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsMappedTypeAsClause {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsMappedTypeAsClause,
        crate::ts::auxiliary::mapped_type_as_clause::FormatTsMappedTypeAsClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::auxiliary::mapped_type_as_clause::FormatTsMappedTypeAsClause::default(),
        )
    }
}
impl FormatRule < biome_js_syntax :: TsMappedTypeOptionalModifierClause > for crate :: ts :: auxiliary :: mapped_type_optional_modifier_clause :: FormatTsMappedTypeOptionalModifierClause { type Context = JsFormatContext ; # [inline (always)] fn fmt (& self , node : & biome_js_syntax :: TsMappedTypeOptionalModifierClause , f : & mut JsFormatter) -> FormatResult < () > { FormatNodeRule :: < biome_js_syntax :: TsMappedTypeOptionalModifierClause > :: fmt (self , node , f) } }
impl AsFormat<JsFormatContext> for biome_js_syntax::TsMappedTypeOptionalModifierClause {
    type Format < 'a > = FormatRefWithRule < 'a , biome_js_syntax :: TsMappedTypeOptionalModifierClause , crate :: ts :: auxiliary :: mapped_type_optional_modifier_clause :: FormatTsMappedTypeOptionalModifierClause > ;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: ts :: auxiliary :: mapped_type_optional_modifier_clause :: FormatTsMappedTypeOptionalModifierClause :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsMappedTypeOptionalModifierClause {
    type Format = FormatOwnedWithRule < biome_js_syntax :: TsMappedTypeOptionalModifierClause , crate :: ts :: auxiliary :: mapped_type_optional_modifier_clause :: FormatTsMappedTypeOptionalModifierClause > ;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: ts :: auxiliary :: mapped_type_optional_modifier_clause :: FormatTsMappedTypeOptionalModifierClause :: default ())
    }
}
impl FormatRule < biome_js_syntax :: TsMappedTypeReadonlyModifierClause > for crate :: ts :: auxiliary :: mapped_type_readonly_modifier_clause :: FormatTsMappedTypeReadonlyModifierClause { type Context = JsFormatContext ; # [inline (always)] fn fmt (& self , node : & biome_js_syntax :: TsMappedTypeReadonlyModifierClause , f : & mut JsFormatter) -> FormatResult < () > { FormatNodeRule :: < biome_js_syntax :: TsMappedTypeReadonlyModifierClause > :: fmt (self , node , f) } }
impl AsFormat<JsFormatContext> for biome_js_syntax::TsMappedTypeReadonlyModifierClause {
    type Format < 'a > = FormatRefWithRule < 'a , biome_js_syntax :: TsMappedTypeReadonlyModifierClause , crate :: ts :: auxiliary :: mapped_type_readonly_modifier_clause :: FormatTsMappedTypeReadonlyModifierClause > ;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: ts :: auxiliary :: mapped_type_readonly_modifier_clause :: FormatTsMappedTypeReadonlyModifierClause :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsMappedTypeReadonlyModifierClause {
    type Format = FormatOwnedWithRule < biome_js_syntax :: TsMappedTypeReadonlyModifierClause , crate :: ts :: auxiliary :: mapped_type_readonly_modifier_clause :: FormatTsMappedTypeReadonlyModifierClause > ;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: ts :: auxiliary :: mapped_type_readonly_modifier_clause :: FormatTsMappedTypeReadonlyModifierClause :: default ())
    }
}
impl FormatRule<biome_js_syntax::TsMethodSignatureClassMember>
    for crate::ts::classes::method_signature_class_member::FormatTsMethodSignatureClassMember
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsMethodSignatureClassMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsMethodSignatureClassMember>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsMethodSignatureClassMember {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsMethodSignatureClassMember,
        crate::ts::classes::method_signature_class_member::FormatTsMethodSignatureClassMember,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: ts :: classes :: method_signature_class_member :: FormatTsMethodSignatureClassMember :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsMethodSignatureClassMember {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsMethodSignatureClassMember,
        crate::ts::classes::method_signature_class_member::FormatTsMethodSignatureClassMember,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: ts :: classes :: method_signature_class_member :: FormatTsMethodSignatureClassMember :: default ())
    }
}
impl FormatRule<biome_js_syntax::TsMethodSignatureTypeMember>
    for crate::ts::auxiliary::method_signature_type_member::FormatTsMethodSignatureTypeMember
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsMethodSignatureTypeMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsMethodSignatureTypeMember>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsMethodSignatureTypeMember {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsMethodSignatureTypeMember,
        crate::ts::auxiliary::method_signature_type_member::FormatTsMethodSignatureTypeMember,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: ts :: auxiliary :: method_signature_type_member :: FormatTsMethodSignatureTypeMember :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsMethodSignatureTypeMember {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsMethodSignatureTypeMember,
        crate::ts::auxiliary::method_signature_type_member::FormatTsMethodSignatureTypeMember,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: ts :: auxiliary :: method_signature_type_member :: FormatTsMethodSignatureTypeMember :: default ())
    }
}
impl FormatRule<biome_js_syntax::TsModuleBlock>
    for crate::ts::auxiliary::module_block::FormatTsModuleBlock
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::TsModuleBlock, f: &mut JsFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsModuleBlock>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsModuleBlock {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsModuleBlock,
        crate::ts::auxiliary::module_block::FormatTsModuleBlock,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::auxiliary::module_block::FormatTsModuleBlock::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsModuleBlock {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsModuleBlock,
        crate::ts::auxiliary::module_block::FormatTsModuleBlock,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::auxiliary::module_block::FormatTsModuleBlock::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsModuleDeclaration>
    for crate::ts::declarations::module_declaration::FormatTsModuleDeclaration
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsModuleDeclaration,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsModuleDeclaration>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsModuleDeclaration {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsModuleDeclaration,
        crate::ts::declarations::module_declaration::FormatTsModuleDeclaration,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::declarations::module_declaration::FormatTsModuleDeclaration::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsModuleDeclaration {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsModuleDeclaration,
        crate::ts::declarations::module_declaration::FormatTsModuleDeclaration,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::declarations::module_declaration::FormatTsModuleDeclaration::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsNamedTupleTypeElement>
    for crate::ts::auxiliary::named_tuple_type_element::FormatTsNamedTupleTypeElement
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsNamedTupleTypeElement,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsNamedTupleTypeElement>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsNamedTupleTypeElement {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsNamedTupleTypeElement,
        crate::ts::auxiliary::named_tuple_type_element::FormatTsNamedTupleTypeElement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::auxiliary::named_tuple_type_element::FormatTsNamedTupleTypeElement::default(
            ),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsNamedTupleTypeElement {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsNamedTupleTypeElement,
        crate::ts::auxiliary::named_tuple_type_element::FormatTsNamedTupleTypeElement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::auxiliary::named_tuple_type_element::FormatTsNamedTupleTypeElement::default(
            ),
        )
    }
}
impl FormatRule<biome_js_syntax::TsNeverType> for crate::ts::types::never_type::FormatTsNeverType {
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::TsNeverType, f: &mut JsFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsNeverType>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsNeverType {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsNeverType,
        crate::ts::types::never_type::FormatTsNeverType,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::types::never_type::FormatTsNeverType::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsNeverType {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsNeverType,
        crate::ts::types::never_type::FormatTsNeverType,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::types::never_type::FormatTsNeverType::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsNonNullAssertionAssignment>
    for crate::ts::assignments::non_null_assertion_assignment::FormatTsNonNullAssertionAssignment
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsNonNullAssertionAssignment,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsNonNullAssertionAssignment>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsNonNullAssertionAssignment {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsNonNullAssertionAssignment,
        crate::ts::assignments::non_null_assertion_assignment::FormatTsNonNullAssertionAssignment,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: ts :: assignments :: non_null_assertion_assignment :: FormatTsNonNullAssertionAssignment :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsNonNullAssertionAssignment {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsNonNullAssertionAssignment,
        crate::ts::assignments::non_null_assertion_assignment::FormatTsNonNullAssertionAssignment,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: ts :: assignments :: non_null_assertion_assignment :: FormatTsNonNullAssertionAssignment :: default ())
    }
}
impl FormatRule<biome_js_syntax::TsNonNullAssertionExpression>
    for crate::ts::expressions::non_null_assertion_expression::FormatTsNonNullAssertionExpression
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsNonNullAssertionExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsNonNullAssertionExpression>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsNonNullAssertionExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsNonNullAssertionExpression,
        crate::ts::expressions::non_null_assertion_expression::FormatTsNonNullAssertionExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: ts :: expressions :: non_null_assertion_expression :: FormatTsNonNullAssertionExpression :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsNonNullAssertionExpression {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsNonNullAssertionExpression,
        crate::ts::expressions::non_null_assertion_expression::FormatTsNonNullAssertionExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: ts :: expressions :: non_null_assertion_expression :: FormatTsNonNullAssertionExpression :: default ())
    }
}
impl FormatRule<biome_js_syntax::TsNonPrimitiveType>
    for crate::ts::types::non_primitive_type::FormatTsNonPrimitiveType
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsNonPrimitiveType,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsNonPrimitiveType>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsNonPrimitiveType {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsNonPrimitiveType,
        crate::ts::types::non_primitive_type::FormatTsNonPrimitiveType,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::types::non_primitive_type::FormatTsNonPrimitiveType::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsNonPrimitiveType {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsNonPrimitiveType,
        crate::ts::types::non_primitive_type::FormatTsNonPrimitiveType,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::types::non_primitive_type::FormatTsNonPrimitiveType::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsNullLiteralType>
    for crate::ts::types::null_literal_type::FormatTsNullLiteralType
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsNullLiteralType,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsNullLiteralType>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsNullLiteralType {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsNullLiteralType,
        crate::ts::types::null_literal_type::FormatTsNullLiteralType,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::types::null_literal_type::FormatTsNullLiteralType::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsNullLiteralType {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsNullLiteralType,
        crate::ts::types::null_literal_type::FormatTsNullLiteralType,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::types::null_literal_type::FormatTsNullLiteralType::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsNumberLiteralType>
    for crate::ts::types::number_literal_type::FormatTsNumberLiteralType
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsNumberLiteralType,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsNumberLiteralType>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsNumberLiteralType {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsNumberLiteralType,
        crate::ts::types::number_literal_type::FormatTsNumberLiteralType,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::types::number_literal_type::FormatTsNumberLiteralType::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsNumberLiteralType {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsNumberLiteralType,
        crate::ts::types::number_literal_type::FormatTsNumberLiteralType,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::types::number_literal_type::FormatTsNumberLiteralType::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsNumberType>
    for crate::ts::types::number_type::FormatTsNumberType
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::TsNumberType, f: &mut JsFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsNumberType>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsNumberType {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsNumberType,
        crate::ts::types::number_type::FormatTsNumberType,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::types::number_type::FormatTsNumberType::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsNumberType {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsNumberType,
        crate::ts::types::number_type::FormatTsNumberType,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::types::number_type::FormatTsNumberType::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsObjectType>
    for crate::ts::types::object_type::FormatTsObjectType
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::TsObjectType, f: &mut JsFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsObjectType>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsObjectType {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsObjectType,
        crate::ts::types::object_type::FormatTsObjectType,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::types::object_type::FormatTsObjectType::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsObjectType {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsObjectType,
        crate::ts::types::object_type::FormatTsObjectType,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::types::object_type::FormatTsObjectType::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsOptionalPropertyAnnotation>
    for crate::ts::auxiliary::optional_property_annotation::FormatTsOptionalPropertyAnnotation
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsOptionalPropertyAnnotation,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsOptionalPropertyAnnotation>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsOptionalPropertyAnnotation {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsOptionalPropertyAnnotation,
        crate::ts::auxiliary::optional_property_annotation::FormatTsOptionalPropertyAnnotation,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: ts :: auxiliary :: optional_property_annotation :: FormatTsOptionalPropertyAnnotation :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsOptionalPropertyAnnotation {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsOptionalPropertyAnnotation,
        crate::ts::auxiliary::optional_property_annotation::FormatTsOptionalPropertyAnnotation,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: ts :: auxiliary :: optional_property_annotation :: FormatTsOptionalPropertyAnnotation :: default ())
    }
}
impl FormatRule<biome_js_syntax::TsOptionalTupleTypeElement>
    for crate::ts::auxiliary::optional_tuple_type_element::FormatTsOptionalTupleTypeElement
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsOptionalTupleTypeElement,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsOptionalTupleTypeElement>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsOptionalTupleTypeElement {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsOptionalTupleTypeElement,
        crate::ts::auxiliary::optional_tuple_type_element::FormatTsOptionalTupleTypeElement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: ts :: auxiliary :: optional_tuple_type_element :: FormatTsOptionalTupleTypeElement :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsOptionalTupleTypeElement {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsOptionalTupleTypeElement,
        crate::ts::auxiliary::optional_tuple_type_element::FormatTsOptionalTupleTypeElement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: ts :: auxiliary :: optional_tuple_type_element :: FormatTsOptionalTupleTypeElement :: default ())
    }
}
impl FormatRule<biome_js_syntax::TsOutModifier>
    for crate::ts::auxiliary::out_modifier::FormatTsOutModifier
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::TsOutModifier, f: &mut JsFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsOutModifier>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsOutModifier {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsOutModifier,
        crate::ts::auxiliary::out_modifier::FormatTsOutModifier,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::auxiliary::out_modifier::FormatTsOutModifier::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsOutModifier {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsOutModifier,
        crate::ts::auxiliary::out_modifier::FormatTsOutModifier,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::auxiliary::out_modifier::FormatTsOutModifier::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsOverrideModifier>
    for crate::ts::auxiliary::override_modifier::FormatTsOverrideModifier
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsOverrideModifier,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsOverrideModifier>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsOverrideModifier {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsOverrideModifier,
        crate::ts::auxiliary::override_modifier::FormatTsOverrideModifier,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::auxiliary::override_modifier::FormatTsOverrideModifier::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsOverrideModifier {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsOverrideModifier,
        crate::ts::auxiliary::override_modifier::FormatTsOverrideModifier,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::auxiliary::override_modifier::FormatTsOverrideModifier::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsParenthesizedType>
    for crate::ts::types::parenthesized_type::FormatTsParenthesizedType
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsParenthesizedType,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsParenthesizedType>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsParenthesizedType {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsParenthesizedType,
        crate::ts::types::parenthesized_type::FormatTsParenthesizedType,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::types::parenthesized_type::FormatTsParenthesizedType::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsParenthesizedType {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsParenthesizedType,
        crate::ts::types::parenthesized_type::FormatTsParenthesizedType,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::types::parenthesized_type::FormatTsParenthesizedType::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsPredicateReturnType>
    for crate::ts::types::predicate_return_type::FormatTsPredicateReturnType
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsPredicateReturnType,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsPredicateReturnType>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsPredicateReturnType {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsPredicateReturnType,
        crate::ts::types::predicate_return_type::FormatTsPredicateReturnType,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::types::predicate_return_type::FormatTsPredicateReturnType::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsPredicateReturnType {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsPredicateReturnType,
        crate::ts::types::predicate_return_type::FormatTsPredicateReturnType,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::types::predicate_return_type::FormatTsPredicateReturnType::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsPropertyParameter>
    for crate::ts::bindings::property_parameter::FormatTsPropertyParameter
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsPropertyParameter,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsPropertyParameter>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsPropertyParameter {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsPropertyParameter,
        crate::ts::bindings::property_parameter::FormatTsPropertyParameter,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::bindings::property_parameter::FormatTsPropertyParameter::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsPropertyParameter {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsPropertyParameter,
        crate::ts::bindings::property_parameter::FormatTsPropertyParameter,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::bindings::property_parameter::FormatTsPropertyParameter::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsPropertySignatureClassMember>
    for crate::ts::classes::property_signature_class_member::FormatTsPropertySignatureClassMember
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsPropertySignatureClassMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsPropertySignatureClassMember>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsPropertySignatureClassMember {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsPropertySignatureClassMember,
        crate::ts::classes::property_signature_class_member::FormatTsPropertySignatureClassMember,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: ts :: classes :: property_signature_class_member :: FormatTsPropertySignatureClassMember :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsPropertySignatureClassMember {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsPropertySignatureClassMember,
        crate::ts::classes::property_signature_class_member::FormatTsPropertySignatureClassMember,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: ts :: classes :: property_signature_class_member :: FormatTsPropertySignatureClassMember :: default ())
    }
}
impl FormatRule<biome_js_syntax::TsPropertySignatureTypeMember>
    for crate::ts::auxiliary::property_signature_type_member::FormatTsPropertySignatureTypeMember
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsPropertySignatureTypeMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsPropertySignatureTypeMember>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsPropertySignatureTypeMember {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsPropertySignatureTypeMember,
        crate::ts::auxiliary::property_signature_type_member::FormatTsPropertySignatureTypeMember,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: ts :: auxiliary :: property_signature_type_member :: FormatTsPropertySignatureTypeMember :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsPropertySignatureTypeMember {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsPropertySignatureTypeMember,
        crate::ts::auxiliary::property_signature_type_member::FormatTsPropertySignatureTypeMember,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: ts :: auxiliary :: property_signature_type_member :: FormatTsPropertySignatureTypeMember :: default ())
    }
}
impl FormatRule<biome_js_syntax::TsQualifiedModuleName>
    for crate::ts::auxiliary::qualified_module_name::FormatTsQualifiedModuleName
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsQualifiedModuleName,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsQualifiedModuleName>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsQualifiedModuleName {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsQualifiedModuleName,
        crate::ts::auxiliary::qualified_module_name::FormatTsQualifiedModuleName,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::auxiliary::qualified_module_name::FormatTsQualifiedModuleName::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsQualifiedModuleName {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsQualifiedModuleName,
        crate::ts::auxiliary::qualified_module_name::FormatTsQualifiedModuleName,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::auxiliary::qualified_module_name::FormatTsQualifiedModuleName::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsQualifiedName>
    for crate::ts::auxiliary::qualified_name::FormatTsQualifiedName
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsQualifiedName,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsQualifiedName>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsQualifiedName {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsQualifiedName,
        crate::ts::auxiliary::qualified_name::FormatTsQualifiedName,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::auxiliary::qualified_name::FormatTsQualifiedName::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsQualifiedName {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsQualifiedName,
        crate::ts::auxiliary::qualified_name::FormatTsQualifiedName,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::auxiliary::qualified_name::FormatTsQualifiedName::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsReadonlyModifier>
    for crate::ts::auxiliary::readonly_modifier::FormatTsReadonlyModifier
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsReadonlyModifier,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsReadonlyModifier>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsReadonlyModifier {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsReadonlyModifier,
        crate::ts::auxiliary::readonly_modifier::FormatTsReadonlyModifier,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::auxiliary::readonly_modifier::FormatTsReadonlyModifier::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsReadonlyModifier {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsReadonlyModifier,
        crate::ts::auxiliary::readonly_modifier::FormatTsReadonlyModifier,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::auxiliary::readonly_modifier::FormatTsReadonlyModifier::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsReferenceType>
    for crate::ts::types::reference_type::FormatTsReferenceType
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsReferenceType,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsReferenceType>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsReferenceType {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsReferenceType,
        crate::ts::types::reference_type::FormatTsReferenceType,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::types::reference_type::FormatTsReferenceType::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsReferenceType {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsReferenceType,
        crate::ts::types::reference_type::FormatTsReferenceType,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::types::reference_type::FormatTsReferenceType::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsRestTupleTypeElement>
    for crate::ts::auxiliary::rest_tuple_type_element::FormatTsRestTupleTypeElement
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsRestTupleTypeElement,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsRestTupleTypeElement>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsRestTupleTypeElement {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsRestTupleTypeElement,
        crate::ts::auxiliary::rest_tuple_type_element::FormatTsRestTupleTypeElement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::auxiliary::rest_tuple_type_element::FormatTsRestTupleTypeElement::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsRestTupleTypeElement {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsRestTupleTypeElement,
        crate::ts::auxiliary::rest_tuple_type_element::FormatTsRestTupleTypeElement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::auxiliary::rest_tuple_type_element::FormatTsRestTupleTypeElement::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsReturnTypeAnnotation>
    for crate::ts::auxiliary::return_type_annotation::FormatTsReturnTypeAnnotation
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsReturnTypeAnnotation,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsReturnTypeAnnotation>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsReturnTypeAnnotation {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsReturnTypeAnnotation,
        crate::ts::auxiliary::return_type_annotation::FormatTsReturnTypeAnnotation,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::auxiliary::return_type_annotation::FormatTsReturnTypeAnnotation::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsReturnTypeAnnotation {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsReturnTypeAnnotation,
        crate::ts::auxiliary::return_type_annotation::FormatTsReturnTypeAnnotation,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::auxiliary::return_type_annotation::FormatTsReturnTypeAnnotation::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsSatisfiesAssignment>
    for crate::ts::assignments::satisfies_assignment::FormatTsSatisfiesAssignment
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsSatisfiesAssignment,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsSatisfiesAssignment>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsSatisfiesAssignment {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsSatisfiesAssignment,
        crate::ts::assignments::satisfies_assignment::FormatTsSatisfiesAssignment,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::assignments::satisfies_assignment::FormatTsSatisfiesAssignment::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsSatisfiesAssignment {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsSatisfiesAssignment,
        crate::ts::assignments::satisfies_assignment::FormatTsSatisfiesAssignment,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::assignments::satisfies_assignment::FormatTsSatisfiesAssignment::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsSatisfiesExpression>
    for crate::ts::expressions::satisfies_expression::FormatTsSatisfiesExpression
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsSatisfiesExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsSatisfiesExpression>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsSatisfiesExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsSatisfiesExpression,
        crate::ts::expressions::satisfies_expression::FormatTsSatisfiesExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::expressions::satisfies_expression::FormatTsSatisfiesExpression::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsSatisfiesExpression {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsSatisfiesExpression,
        crate::ts::expressions::satisfies_expression::FormatTsSatisfiesExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::expressions::satisfies_expression::FormatTsSatisfiesExpression::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsSetterSignatureClassMember>
    for crate::ts::classes::setter_signature_class_member::FormatTsSetterSignatureClassMember
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsSetterSignatureClassMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsSetterSignatureClassMember>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsSetterSignatureClassMember {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsSetterSignatureClassMember,
        crate::ts::classes::setter_signature_class_member::FormatTsSetterSignatureClassMember,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: ts :: classes :: setter_signature_class_member :: FormatTsSetterSignatureClassMember :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsSetterSignatureClassMember {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsSetterSignatureClassMember,
        crate::ts::classes::setter_signature_class_member::FormatTsSetterSignatureClassMember,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: ts :: classes :: setter_signature_class_member :: FormatTsSetterSignatureClassMember :: default ())
    }
}
impl FormatRule<biome_js_syntax::TsSetterSignatureTypeMember>
    for crate::ts::auxiliary::setter_signature_type_member::FormatTsSetterSignatureTypeMember
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsSetterSignatureTypeMember,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsSetterSignatureTypeMember>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsSetterSignatureTypeMember {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsSetterSignatureTypeMember,
        crate::ts::auxiliary::setter_signature_type_member::FormatTsSetterSignatureTypeMember,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: ts :: auxiliary :: setter_signature_type_member :: FormatTsSetterSignatureTypeMember :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsSetterSignatureTypeMember {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsSetterSignatureTypeMember,
        crate::ts::auxiliary::setter_signature_type_member::FormatTsSetterSignatureTypeMember,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: ts :: auxiliary :: setter_signature_type_member :: FormatTsSetterSignatureTypeMember :: default ())
    }
}
impl FormatRule<biome_js_syntax::TsStringLiteralType>
    for crate::ts::types::string_literal_type::FormatTsStringLiteralType
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsStringLiteralType,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsStringLiteralType>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsStringLiteralType {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsStringLiteralType,
        crate::ts::types::string_literal_type::FormatTsStringLiteralType,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::types::string_literal_type::FormatTsStringLiteralType::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsStringLiteralType {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsStringLiteralType,
        crate::ts::types::string_literal_type::FormatTsStringLiteralType,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::types::string_literal_type::FormatTsStringLiteralType::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsStringType>
    for crate::ts::types::string_type::FormatTsStringType
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::TsStringType, f: &mut JsFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsStringType>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsStringType {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsStringType,
        crate::ts::types::string_type::FormatTsStringType,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::types::string_type::FormatTsStringType::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsStringType {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsStringType,
        crate::ts::types::string_type::FormatTsStringType,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::types::string_type::FormatTsStringType::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsSymbolType>
    for crate::ts::types::symbol_type::FormatTsSymbolType
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::TsSymbolType, f: &mut JsFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsSymbolType>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsSymbolType {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsSymbolType,
        crate::ts::types::symbol_type::FormatTsSymbolType,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::types::symbol_type::FormatTsSymbolType::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsSymbolType {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsSymbolType,
        crate::ts::types::symbol_type::FormatTsSymbolType,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::types::symbol_type::FormatTsSymbolType::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsTemplateChunkElement>
    for crate::ts::auxiliary::template_chunk_element::FormatTsTemplateChunkElement
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsTemplateChunkElement,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsTemplateChunkElement>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsTemplateChunkElement {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsTemplateChunkElement,
        crate::ts::auxiliary::template_chunk_element::FormatTsTemplateChunkElement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::auxiliary::template_chunk_element::FormatTsTemplateChunkElement::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsTemplateChunkElement {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsTemplateChunkElement,
        crate::ts::auxiliary::template_chunk_element::FormatTsTemplateChunkElement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::auxiliary::template_chunk_element::FormatTsTemplateChunkElement::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsTemplateElement>
    for crate::ts::auxiliary::template_element::FormatTsTemplateElement
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsTemplateElement,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsTemplateElement>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsTemplateElement {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsTemplateElement,
        crate::ts::auxiliary::template_element::FormatTsTemplateElement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::auxiliary::template_element::FormatTsTemplateElement::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsTemplateElement {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsTemplateElement,
        crate::ts::auxiliary::template_element::FormatTsTemplateElement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::auxiliary::template_element::FormatTsTemplateElement::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsTemplateLiteralType>
    for crate::ts::types::template_literal_type::FormatTsTemplateLiteralType
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsTemplateLiteralType,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsTemplateLiteralType>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsTemplateLiteralType {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsTemplateLiteralType,
        crate::ts::types::template_literal_type::FormatTsTemplateLiteralType,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::types::template_literal_type::FormatTsTemplateLiteralType::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsTemplateLiteralType {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsTemplateLiteralType,
        crate::ts::types::template_literal_type::FormatTsTemplateLiteralType,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::types::template_literal_type::FormatTsTemplateLiteralType::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsThisParameter>
    for crate::ts::bindings::this_parameter::FormatTsThisParameter
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsThisParameter,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsThisParameter>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsThisParameter {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsThisParameter,
        crate::ts::bindings::this_parameter::FormatTsThisParameter,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::bindings::this_parameter::FormatTsThisParameter::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsThisParameter {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsThisParameter,
        crate::ts::bindings::this_parameter::FormatTsThisParameter,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::bindings::this_parameter::FormatTsThisParameter::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsThisType> for crate::ts::types::this_type::FormatTsThisType {
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::TsThisType, f: &mut JsFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsThisType>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsThisType {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsThisType,
        crate::ts::types::this_type::FormatTsThisType,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::types::this_type::FormatTsThisType::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsThisType {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsThisType,
        crate::ts::types::this_type::FormatTsThisType,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::types::this_type::FormatTsThisType::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsTupleType> for crate::ts::types::tuple_type::FormatTsTupleType {
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::TsTupleType, f: &mut JsFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsTupleType>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsTupleType {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsTupleType,
        crate::ts::types::tuple_type::FormatTsTupleType,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::types::tuple_type::FormatTsTupleType::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsTupleType {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsTupleType,
        crate::ts::types::tuple_type::FormatTsTupleType,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::types::tuple_type::FormatTsTupleType::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsTypeAliasDeclaration>
    for crate::ts::declarations::type_alias_declaration::FormatTsTypeAliasDeclaration
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsTypeAliasDeclaration,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsTypeAliasDeclaration>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsTypeAliasDeclaration {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsTypeAliasDeclaration,
        crate::ts::declarations::type_alias_declaration::FormatTsTypeAliasDeclaration,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::declarations::type_alias_declaration::FormatTsTypeAliasDeclaration::default(
            ),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsTypeAliasDeclaration {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsTypeAliasDeclaration,
        crate::ts::declarations::type_alias_declaration::FormatTsTypeAliasDeclaration,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::declarations::type_alias_declaration::FormatTsTypeAliasDeclaration::default(
            ),
        )
    }
}
impl FormatRule<biome_js_syntax::TsTypeAnnotation>
    for crate::ts::auxiliary::type_annotation::FormatTsTypeAnnotation
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsTypeAnnotation,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsTypeAnnotation>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsTypeAnnotation {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsTypeAnnotation,
        crate::ts::auxiliary::type_annotation::FormatTsTypeAnnotation,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::auxiliary::type_annotation::FormatTsTypeAnnotation::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsTypeAnnotation {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsTypeAnnotation,
        crate::ts::auxiliary::type_annotation::FormatTsTypeAnnotation,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::auxiliary::type_annotation::FormatTsTypeAnnotation::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsTypeArguments>
    for crate::ts::expressions::type_arguments::FormatTsTypeArguments
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsTypeArguments,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsTypeArguments>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsTypeArguments {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsTypeArguments,
        crate::ts::expressions::type_arguments::FormatTsTypeArguments,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::expressions::type_arguments::FormatTsTypeArguments::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsTypeArguments {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsTypeArguments,
        crate::ts::expressions::type_arguments::FormatTsTypeArguments,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::expressions::type_arguments::FormatTsTypeArguments::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsTypeAssertionAssignment>
    for crate::ts::assignments::type_assertion_assignment::FormatTsTypeAssertionAssignment
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsTypeAssertionAssignment,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsTypeAssertionAssignment>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsTypeAssertionAssignment {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsTypeAssertionAssignment,
        crate::ts::assignments::type_assertion_assignment::FormatTsTypeAssertionAssignment,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: ts :: assignments :: type_assertion_assignment :: FormatTsTypeAssertionAssignment :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsTypeAssertionAssignment {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsTypeAssertionAssignment,
        crate::ts::assignments::type_assertion_assignment::FormatTsTypeAssertionAssignment,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: ts :: assignments :: type_assertion_assignment :: FormatTsTypeAssertionAssignment :: default ())
    }
}
impl FormatRule<biome_js_syntax::TsTypeAssertionExpression>
    for crate::ts::expressions::type_assertion_expression::FormatTsTypeAssertionExpression
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsTypeAssertionExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsTypeAssertionExpression>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsTypeAssertionExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsTypeAssertionExpression,
        crate::ts::expressions::type_assertion_expression::FormatTsTypeAssertionExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: ts :: expressions :: type_assertion_expression :: FormatTsTypeAssertionExpression :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsTypeAssertionExpression {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsTypeAssertionExpression,
        crate::ts::expressions::type_assertion_expression::FormatTsTypeAssertionExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: ts :: expressions :: type_assertion_expression :: FormatTsTypeAssertionExpression :: default ())
    }
}
impl FormatRule<biome_js_syntax::TsTypeConstraintClause>
    for crate::ts::auxiliary::type_constraint_clause::FormatTsTypeConstraintClause
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsTypeConstraintClause,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsTypeConstraintClause>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsTypeConstraintClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsTypeConstraintClause,
        crate::ts::auxiliary::type_constraint_clause::FormatTsTypeConstraintClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::auxiliary::type_constraint_clause::FormatTsTypeConstraintClause::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsTypeConstraintClause {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsTypeConstraintClause,
        crate::ts::auxiliary::type_constraint_clause::FormatTsTypeConstraintClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::auxiliary::type_constraint_clause::FormatTsTypeConstraintClause::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsTypeOperatorType>
    for crate::ts::types::type_operator_type::FormatTsTypeOperatorType
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsTypeOperatorType,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsTypeOperatorType>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsTypeOperatorType {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsTypeOperatorType,
        crate::ts::types::type_operator_type::FormatTsTypeOperatorType,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::types::type_operator_type::FormatTsTypeOperatorType::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsTypeOperatorType {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsTypeOperatorType,
        crate::ts::types::type_operator_type::FormatTsTypeOperatorType,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::types::type_operator_type::FormatTsTypeOperatorType::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsTypeParameter>
    for crate::ts::bindings::type_parameter::FormatTsTypeParameter
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsTypeParameter,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsTypeParameter>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsTypeParameter {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsTypeParameter,
        crate::ts::bindings::type_parameter::FormatTsTypeParameter,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::bindings::type_parameter::FormatTsTypeParameter::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsTypeParameter {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsTypeParameter,
        crate::ts::bindings::type_parameter::FormatTsTypeParameter,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::bindings::type_parameter::FormatTsTypeParameter::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsTypeParameterName>
    for crate::ts::auxiliary::type_parameter_name::FormatTsTypeParameterName
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsTypeParameterName,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsTypeParameterName>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsTypeParameterName {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsTypeParameterName,
        crate::ts::auxiliary::type_parameter_name::FormatTsTypeParameterName,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::auxiliary::type_parameter_name::FormatTsTypeParameterName::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsTypeParameterName {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsTypeParameterName,
        crate::ts::auxiliary::type_parameter_name::FormatTsTypeParameterName,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::auxiliary::type_parameter_name::FormatTsTypeParameterName::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsTypeParameters>
    for crate::ts::bindings::type_parameters::FormatTsTypeParameters
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsTypeParameters,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsTypeParameters>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsTypeParameters {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsTypeParameters,
        crate::ts::bindings::type_parameters::FormatTsTypeParameters,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::bindings::type_parameters::FormatTsTypeParameters::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsTypeParameters {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsTypeParameters,
        crate::ts::bindings::type_parameters::FormatTsTypeParameters,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::bindings::type_parameters::FormatTsTypeParameters::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsTypeofType>
    for crate::ts::types::typeof_type::FormatTsTypeofType
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::TsTypeofType, f: &mut JsFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsTypeofType>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsTypeofType {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsTypeofType,
        crate::ts::types::typeof_type::FormatTsTypeofType,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::types::typeof_type::FormatTsTypeofType::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsTypeofType {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsTypeofType,
        crate::ts::types::typeof_type::FormatTsTypeofType,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::types::typeof_type::FormatTsTypeofType::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsUndefinedType>
    for crate::ts::types::undefined_type::FormatTsUndefinedType
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::TsUndefinedType,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsUndefinedType>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsUndefinedType {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsUndefinedType,
        crate::ts::types::undefined_type::FormatTsUndefinedType,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::types::undefined_type::FormatTsUndefinedType::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsUndefinedType {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsUndefinedType,
        crate::ts::types::undefined_type::FormatTsUndefinedType,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::types::undefined_type::FormatTsUndefinedType::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsUnionType> for crate::ts::types::union_type::FormatTsUnionType {
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::TsUnionType, f: &mut JsFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsUnionType>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsUnionType {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsUnionType,
        crate::ts::types::union_type::FormatTsUnionType,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::types::union_type::FormatTsUnionType::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsUnionType {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsUnionType,
        crate::ts::types::union_type::FormatTsUnionType,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::types::union_type::FormatTsUnionType::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsUnknownType>
    for crate::ts::types::unknown_type::FormatTsUnknownType
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::TsUnknownType, f: &mut JsFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsUnknownType>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsUnknownType {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsUnknownType,
        crate::ts::types::unknown_type::FormatTsUnknownType,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::types::unknown_type::FormatTsUnknownType::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsUnknownType {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsUnknownType,
        crate::ts::types::unknown_type::FormatTsUnknownType,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::types::unknown_type::FormatTsUnknownType::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsVoidType> for crate::ts::types::void_type::FormatTsVoidType {
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::TsVoidType, f: &mut JsFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_js_syntax::TsVoidType>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsVoidType {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsVoidType,
        crate::ts::types::void_type::FormatTsVoidType,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::types::void_type::FormatTsVoidType::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsVoidType {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsVoidType,
        crate::ts::types::void_type::FormatTsVoidType,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::types::void_type::FormatTsVoidType::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsArrayAssignmentPatternElementList {
    type Format < 'a > = FormatRefWithRule < 'a , biome_js_syntax :: JsArrayAssignmentPatternElementList , crate :: js :: lists :: array_assignment_pattern_element_list :: FormatJsArrayAssignmentPatternElementList > ;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: js :: lists :: array_assignment_pattern_element_list :: FormatJsArrayAssignmentPatternElementList :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsArrayAssignmentPatternElementList {
    type Format = FormatOwnedWithRule < biome_js_syntax :: JsArrayAssignmentPatternElementList , crate :: js :: lists :: array_assignment_pattern_element_list :: FormatJsArrayAssignmentPatternElementList > ;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: js :: lists :: array_assignment_pattern_element_list :: FormatJsArrayAssignmentPatternElementList :: default ())
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsArrayBindingPatternElementList {
    type Format < 'a > = FormatRefWithRule < 'a , biome_js_syntax :: JsArrayBindingPatternElementList , crate :: js :: lists :: array_binding_pattern_element_list :: FormatJsArrayBindingPatternElementList > ;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: js :: lists :: array_binding_pattern_element_list :: FormatJsArrayBindingPatternElementList :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsArrayBindingPatternElementList {
    type Format = FormatOwnedWithRule < biome_js_syntax :: JsArrayBindingPatternElementList , crate :: js :: lists :: array_binding_pattern_element_list :: FormatJsArrayBindingPatternElementList > ;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: js :: lists :: array_binding_pattern_element_list :: FormatJsArrayBindingPatternElementList :: default ())
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsArrayElementList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsArrayElementList,
        crate::js::lists::array_element_list::FormatJsArrayElementList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::lists::array_element_list::FormatJsArrayElementList::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsArrayElementList {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsArrayElementList,
        crate::js::lists::array_element_list::FormatJsArrayElementList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::lists::array_element_list::FormatJsArrayElementList::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsCallArgumentList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsCallArgumentList,
        crate::js::lists::call_argument_list::FormatJsCallArgumentList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::lists::call_argument_list::FormatJsCallArgumentList::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsCallArgumentList {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsCallArgumentList,
        crate::js::lists::call_argument_list::FormatJsCallArgumentList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::lists::call_argument_list::FormatJsCallArgumentList::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsClassMemberList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsClassMemberList,
        crate::js::lists::class_member_list::FormatJsClassMemberList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::lists::class_member_list::FormatJsClassMemberList::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsClassMemberList {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsClassMemberList,
        crate::js::lists::class_member_list::FormatJsClassMemberList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::lists::class_member_list::FormatJsClassMemberList::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsConstructorModifierList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsConstructorModifierList,
        crate::js::lists::constructor_modifier_list::FormatJsConstructorModifierList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::lists::constructor_modifier_list::FormatJsConstructorModifierList::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsConstructorModifierList {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsConstructorModifierList,
        crate::js::lists::constructor_modifier_list::FormatJsConstructorModifierList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::lists::constructor_modifier_list::FormatJsConstructorModifierList::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsConstructorParameterList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsConstructorParameterList,
        crate::js::lists::constructor_parameter_list::FormatJsConstructorParameterList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::lists::constructor_parameter_list::FormatJsConstructorParameterList::default(
            ),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsConstructorParameterList {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsConstructorParameterList,
        crate::js::lists::constructor_parameter_list::FormatJsConstructorParameterList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::lists::constructor_parameter_list::FormatJsConstructorParameterList::default(
            ),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsDecoratorList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsDecoratorList,
        crate::js::lists::decorator_list::FormatJsDecoratorList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::lists::decorator_list::FormatJsDecoratorList::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsDecoratorList {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsDecoratorList,
        crate::js::lists::decorator_list::FormatJsDecoratorList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::lists::decorator_list::FormatJsDecoratorList::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsDirectiveList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsDirectiveList,
        crate::js::lists::directive_list::FormatJsDirectiveList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::lists::directive_list::FormatJsDirectiveList::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsDirectiveList {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsDirectiveList,
        crate::js::lists::directive_list::FormatJsDirectiveList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::lists::directive_list::FormatJsDirectiveList::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsExportNamedFromSpecifierList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsExportNamedFromSpecifierList,
        crate::js::lists::export_named_from_specifier_list::FormatJsExportNamedFromSpecifierList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: js :: lists :: export_named_from_specifier_list :: FormatJsExportNamedFromSpecifierList :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsExportNamedFromSpecifierList {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsExportNamedFromSpecifierList,
        crate::js::lists::export_named_from_specifier_list::FormatJsExportNamedFromSpecifierList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: js :: lists :: export_named_from_specifier_list :: FormatJsExportNamedFromSpecifierList :: default ())
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsExportNamedSpecifierList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsExportNamedSpecifierList,
        crate::js::lists::export_named_specifier_list::FormatJsExportNamedSpecifierList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: js :: lists :: export_named_specifier_list :: FormatJsExportNamedSpecifierList :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsExportNamedSpecifierList {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsExportNamedSpecifierList,
        crate::js::lists::export_named_specifier_list::FormatJsExportNamedSpecifierList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: js :: lists :: export_named_specifier_list :: FormatJsExportNamedSpecifierList :: default ())
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsImportAssertionEntryList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsImportAssertionEntryList,
        crate::js::lists::import_assertion_entry_list::FormatJsImportAssertionEntryList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: js :: lists :: import_assertion_entry_list :: FormatJsImportAssertionEntryList :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsImportAssertionEntryList {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsImportAssertionEntryList,
        crate::js::lists::import_assertion_entry_list::FormatJsImportAssertionEntryList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: js :: lists :: import_assertion_entry_list :: FormatJsImportAssertionEntryList :: default ())
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsMethodModifierList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsMethodModifierList,
        crate::js::lists::method_modifier_list::FormatJsMethodModifierList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::lists::method_modifier_list::FormatJsMethodModifierList::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsMethodModifierList {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsMethodModifierList,
        crate::js::lists::method_modifier_list::FormatJsMethodModifierList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::lists::method_modifier_list::FormatJsMethodModifierList::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsModuleItemList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsModuleItemList,
        crate::js::lists::module_item_list::FormatJsModuleItemList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::lists::module_item_list::FormatJsModuleItemList::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsModuleItemList {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsModuleItemList,
        crate::js::lists::module_item_list::FormatJsModuleItemList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::lists::module_item_list::FormatJsModuleItemList::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsNamedImportSpecifierList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsNamedImportSpecifierList,
        crate::js::lists::named_import_specifier_list::FormatJsNamedImportSpecifierList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: js :: lists :: named_import_specifier_list :: FormatJsNamedImportSpecifierList :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsNamedImportSpecifierList {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsNamedImportSpecifierList,
        crate::js::lists::named_import_specifier_list::FormatJsNamedImportSpecifierList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: js :: lists :: named_import_specifier_list :: FormatJsNamedImportSpecifierList :: default ())
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsObjectAssignmentPatternPropertyList {
    type Format < 'a > = FormatRefWithRule < 'a , biome_js_syntax :: JsObjectAssignmentPatternPropertyList , crate :: js :: lists :: object_assignment_pattern_property_list :: FormatJsObjectAssignmentPatternPropertyList > ;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: js :: lists :: object_assignment_pattern_property_list :: FormatJsObjectAssignmentPatternPropertyList :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsObjectAssignmentPatternPropertyList {
    type Format = FormatOwnedWithRule < biome_js_syntax :: JsObjectAssignmentPatternPropertyList , crate :: js :: lists :: object_assignment_pattern_property_list :: FormatJsObjectAssignmentPatternPropertyList > ;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: js :: lists :: object_assignment_pattern_property_list :: FormatJsObjectAssignmentPatternPropertyList :: default ())
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsObjectBindingPatternPropertyList {
    type Format < 'a > = FormatRefWithRule < 'a , biome_js_syntax :: JsObjectBindingPatternPropertyList , crate :: js :: lists :: object_binding_pattern_property_list :: FormatJsObjectBindingPatternPropertyList > ;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: js :: lists :: object_binding_pattern_property_list :: FormatJsObjectBindingPatternPropertyList :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsObjectBindingPatternPropertyList {
    type Format = FormatOwnedWithRule < biome_js_syntax :: JsObjectBindingPatternPropertyList , crate :: js :: lists :: object_binding_pattern_property_list :: FormatJsObjectBindingPatternPropertyList > ;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: js :: lists :: object_binding_pattern_property_list :: FormatJsObjectBindingPatternPropertyList :: default ())
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsObjectMemberList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsObjectMemberList,
        crate::js::lists::object_member_list::FormatJsObjectMemberList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::lists::object_member_list::FormatJsObjectMemberList::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsObjectMemberList {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsObjectMemberList,
        crate::js::lists::object_member_list::FormatJsObjectMemberList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::lists::object_member_list::FormatJsObjectMemberList::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsParameterList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsParameterList,
        crate::js::lists::parameter_list::FormatJsParameterList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::lists::parameter_list::FormatJsParameterList::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsParameterList {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsParameterList,
        crate::js::lists::parameter_list::FormatJsParameterList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::lists::parameter_list::FormatJsParameterList::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsPropertyModifierList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsPropertyModifierList,
        crate::js::lists::property_modifier_list::FormatJsPropertyModifierList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::lists::property_modifier_list::FormatJsPropertyModifierList::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsPropertyModifierList {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsPropertyModifierList,
        crate::js::lists::property_modifier_list::FormatJsPropertyModifierList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::lists::property_modifier_list::FormatJsPropertyModifierList::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsStatementList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsStatementList,
        crate::js::lists::statement_list::FormatJsStatementList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::lists::statement_list::FormatJsStatementList::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsStatementList {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsStatementList,
        crate::js::lists::statement_list::FormatJsStatementList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::lists::statement_list::FormatJsStatementList::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsSwitchCaseList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsSwitchCaseList,
        crate::js::lists::switch_case_list::FormatJsSwitchCaseList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::lists::switch_case_list::FormatJsSwitchCaseList::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsSwitchCaseList {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsSwitchCaseList,
        crate::js::lists::switch_case_list::FormatJsSwitchCaseList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::lists::switch_case_list::FormatJsSwitchCaseList::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsTemplateElementList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsTemplateElementList,
        crate::js::lists::template_element_list::FormatJsTemplateElementList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::lists::template_element_list::FormatJsTemplateElementList::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsTemplateElementList {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsTemplateElementList,
        crate::js::lists::template_element_list::FormatJsTemplateElementList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::lists::template_element_list::FormatJsTemplateElementList::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsVariableDeclaratorList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsVariableDeclaratorList,
        crate::js::lists::variable_declarator_list::FormatJsVariableDeclaratorList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::lists::variable_declarator_list::FormatJsVariableDeclaratorList::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsVariableDeclaratorList {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsVariableDeclaratorList,
        crate::js::lists::variable_declarator_list::FormatJsVariableDeclaratorList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::lists::variable_declarator_list::FormatJsVariableDeclaratorList::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsxAttributeList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsxAttributeList,
        crate::jsx::lists::attribute_list::FormatJsxAttributeList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::jsx::lists::attribute_list::FormatJsxAttributeList::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsxAttributeList {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsxAttributeList,
        crate::jsx::lists::attribute_list::FormatJsxAttributeList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::jsx::lists::attribute_list::FormatJsxAttributeList::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsxChildList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsxChildList,
        crate::jsx::lists::child_list::FormatJsxChildList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::jsx::lists::child_list::FormatJsxChildList::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsxChildList {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsxChildList,
        crate::jsx::lists::child_list::FormatJsxChildList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::jsx::lists::child_list::FormatJsxChildList::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsEnumMemberList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsEnumMemberList,
        crate::ts::lists::enum_member_list::FormatTsEnumMemberList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::lists::enum_member_list::FormatTsEnumMemberList::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsEnumMemberList {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsEnumMemberList,
        crate::ts::lists::enum_member_list::FormatTsEnumMemberList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::lists::enum_member_list::FormatTsEnumMemberList::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsIndexSignatureModifierList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsIndexSignatureModifierList,
        crate::ts::lists::index_signature_modifier_list::FormatTsIndexSignatureModifierList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: ts :: lists :: index_signature_modifier_list :: FormatTsIndexSignatureModifierList :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsIndexSignatureModifierList {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsIndexSignatureModifierList,
        crate::ts::lists::index_signature_modifier_list::FormatTsIndexSignatureModifierList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: ts :: lists :: index_signature_modifier_list :: FormatTsIndexSignatureModifierList :: default ())
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsIntersectionTypeElementList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsIntersectionTypeElementList,
        crate::ts::lists::intersection_type_element_list::FormatTsIntersectionTypeElementList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: ts :: lists :: intersection_type_element_list :: FormatTsIntersectionTypeElementList :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsIntersectionTypeElementList {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsIntersectionTypeElementList,
        crate::ts::lists::intersection_type_element_list::FormatTsIntersectionTypeElementList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: ts :: lists :: intersection_type_element_list :: FormatTsIntersectionTypeElementList :: default ())
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsMethodSignatureModifierList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsMethodSignatureModifierList,
        crate::ts::lists::method_signature_modifier_list::FormatTsMethodSignatureModifierList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: ts :: lists :: method_signature_modifier_list :: FormatTsMethodSignatureModifierList :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsMethodSignatureModifierList {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsMethodSignatureModifierList,
        crate::ts::lists::method_signature_modifier_list::FormatTsMethodSignatureModifierList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: ts :: lists :: method_signature_modifier_list :: FormatTsMethodSignatureModifierList :: default ())
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsPropertyParameterModifierList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsPropertyParameterModifierList,
        crate::ts::lists::property_parameter_modifier_list::FormatTsPropertyParameterModifierList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: ts :: lists :: property_parameter_modifier_list :: FormatTsPropertyParameterModifierList :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsPropertyParameterModifierList {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsPropertyParameterModifierList,
        crate::ts::lists::property_parameter_modifier_list::FormatTsPropertyParameterModifierList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: ts :: lists :: property_parameter_modifier_list :: FormatTsPropertyParameterModifierList :: default ())
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsPropertySignatureModifierList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsPropertySignatureModifierList,
        crate::ts::lists::property_signature_modifier_list::FormatTsPropertySignatureModifierList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: ts :: lists :: property_signature_modifier_list :: FormatTsPropertySignatureModifierList :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsPropertySignatureModifierList {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsPropertySignatureModifierList,
        crate::ts::lists::property_signature_modifier_list::FormatTsPropertySignatureModifierList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: ts :: lists :: property_signature_modifier_list :: FormatTsPropertySignatureModifierList :: default ())
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsTemplateElementList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsTemplateElementList,
        crate::ts::lists::template_element_list::FormatTsTemplateElementList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::lists::template_element_list::FormatTsTemplateElementList::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsTemplateElementList {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsTemplateElementList,
        crate::ts::lists::template_element_list::FormatTsTemplateElementList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::lists::template_element_list::FormatTsTemplateElementList::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsTupleTypeElementList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsTupleTypeElementList,
        crate::ts::lists::tuple_type_element_list::FormatTsTupleTypeElementList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::lists::tuple_type_element_list::FormatTsTupleTypeElementList::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsTupleTypeElementList {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsTupleTypeElementList,
        crate::ts::lists::tuple_type_element_list::FormatTsTupleTypeElementList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::lists::tuple_type_element_list::FormatTsTupleTypeElementList::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsTypeArgumentList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsTypeArgumentList,
        crate::ts::lists::type_argument_list::FormatTsTypeArgumentList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::lists::type_argument_list::FormatTsTypeArgumentList::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsTypeArgumentList {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsTypeArgumentList,
        crate::ts::lists::type_argument_list::FormatTsTypeArgumentList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::lists::type_argument_list::FormatTsTypeArgumentList::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsTypeList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsTypeList,
        crate::ts::lists::type_list::FormatTsTypeList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::lists::type_list::FormatTsTypeList::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsTypeList {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsTypeList,
        crate::ts::lists::type_list::FormatTsTypeList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::lists::type_list::FormatTsTypeList::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsTypeMemberList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsTypeMemberList,
        crate::ts::lists::type_member_list::FormatTsTypeMemberList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::lists::type_member_list::FormatTsTypeMemberList::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsTypeMemberList {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsTypeMemberList,
        crate::ts::lists::type_member_list::FormatTsTypeMemberList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::lists::type_member_list::FormatTsTypeMemberList::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsTypeParameterList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsTypeParameterList,
        crate::ts::lists::type_parameter_list::FormatTsTypeParameterList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::lists::type_parameter_list::FormatTsTypeParameterList::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsTypeParameterList {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsTypeParameterList,
        crate::ts::lists::type_parameter_list::FormatTsTypeParameterList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::lists::type_parameter_list::FormatTsTypeParameterList::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsTypeParameterModifierList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsTypeParameterModifierList,
        crate::ts::lists::type_parameter_modifier_list::FormatTsTypeParameterModifierList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: ts :: lists :: type_parameter_modifier_list :: FormatTsTypeParameterModifierList :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsTypeParameterModifierList {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsTypeParameterModifierList,
        crate::ts::lists::type_parameter_modifier_list::FormatTsTypeParameterModifierList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: ts :: lists :: type_parameter_modifier_list :: FormatTsTypeParameterModifierList :: default ())
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsUnionTypeVariantList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsUnionTypeVariantList,
        crate::ts::lists::union_type_variant_list::FormatTsUnionTypeVariantList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::lists::union_type_variant_list::FormatTsUnionTypeVariantList::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsUnionTypeVariantList {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsUnionTypeVariantList,
        crate::ts::lists::union_type_variant_list::FormatTsUnionTypeVariantList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::lists::union_type_variant_list::FormatTsUnionTypeVariantList::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsBogus> for crate::js::bogus::bogus::FormatJsBogus {
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::JsBogus, f: &mut JsFormatter) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_js_syntax::JsBogus>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsBogus {
    type Format<'a> =
        FormatRefWithRule<'a, biome_js_syntax::JsBogus, crate::js::bogus::bogus::FormatJsBogus>;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, crate::js::bogus::bogus::FormatJsBogus::default())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsBogus {
    type Format =
        FormatOwnedWithRule<biome_js_syntax::JsBogus, crate::js::bogus::bogus::FormatJsBogus>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, crate::js::bogus::bogus::FormatJsBogus::default())
    }
}
impl FormatRule<biome_js_syntax::JsBogusAssignment>
    for crate::js::bogus::bogus_assignment::FormatJsBogusAssignment
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsBogusAssignment,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_js_syntax::JsBogusAssignment>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsBogusAssignment {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsBogusAssignment,
        crate::js::bogus::bogus_assignment::FormatJsBogusAssignment,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::bogus::bogus_assignment::FormatJsBogusAssignment::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsBogusAssignment {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsBogusAssignment,
        crate::js::bogus::bogus_assignment::FormatJsBogusAssignment,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::bogus::bogus_assignment::FormatJsBogusAssignment::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsBogusBinding>
    for crate::js::bogus::bogus_binding::FormatJsBogusBinding
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::JsBogusBinding, f: &mut JsFormatter) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_js_syntax::JsBogusBinding>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsBogusBinding {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsBogusBinding,
        crate::js::bogus::bogus_binding::FormatJsBogusBinding,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::bogus::bogus_binding::FormatJsBogusBinding::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsBogusBinding {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsBogusBinding,
        crate::js::bogus::bogus_binding::FormatJsBogusBinding,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::bogus::bogus_binding::FormatJsBogusBinding::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsBogusExpression>
    for crate::js::bogus::bogus_expression::FormatJsBogusExpression
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsBogusExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_js_syntax::JsBogusExpression>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsBogusExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsBogusExpression,
        crate::js::bogus::bogus_expression::FormatJsBogusExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::bogus::bogus_expression::FormatJsBogusExpression::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsBogusExpression {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsBogusExpression,
        crate::js::bogus::bogus_expression::FormatJsBogusExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::bogus::bogus_expression::FormatJsBogusExpression::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsBogusImportAssertionEntry>
    for crate::js::bogus::bogus_import_assertion_entry::FormatJsBogusImportAssertionEntry
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsBogusImportAssertionEntry,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_js_syntax::JsBogusImportAssertionEntry>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsBogusImportAssertionEntry {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsBogusImportAssertionEntry,
        crate::js::bogus::bogus_import_assertion_entry::FormatJsBogusImportAssertionEntry,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: js :: bogus :: bogus_import_assertion_entry :: FormatJsBogusImportAssertionEntry :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsBogusImportAssertionEntry {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsBogusImportAssertionEntry,
        crate::js::bogus::bogus_import_assertion_entry::FormatJsBogusImportAssertionEntry,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: js :: bogus :: bogus_import_assertion_entry :: FormatJsBogusImportAssertionEntry :: default ())
    }
}
impl FormatRule<biome_js_syntax::JsBogusMember>
    for crate::js::bogus::bogus_member::FormatJsBogusMember
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::JsBogusMember, f: &mut JsFormatter) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_js_syntax::JsBogusMember>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsBogusMember {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsBogusMember,
        crate::js::bogus::bogus_member::FormatJsBogusMember,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::bogus::bogus_member::FormatJsBogusMember::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsBogusMember {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsBogusMember,
        crate::js::bogus::bogus_member::FormatJsBogusMember,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::bogus::bogus_member::FormatJsBogusMember::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsBogusNamedImportSpecifier>
    for crate::js::bogus::bogus_named_import_specifier::FormatJsBogusNamedImportSpecifier
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsBogusNamedImportSpecifier,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_js_syntax::JsBogusNamedImportSpecifier>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsBogusNamedImportSpecifier {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsBogusNamedImportSpecifier,
        crate::js::bogus::bogus_named_import_specifier::FormatJsBogusNamedImportSpecifier,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: js :: bogus :: bogus_named_import_specifier :: FormatJsBogusNamedImportSpecifier :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsBogusNamedImportSpecifier {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsBogusNamedImportSpecifier,
        crate::js::bogus::bogus_named_import_specifier::FormatJsBogusNamedImportSpecifier,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: js :: bogus :: bogus_named_import_specifier :: FormatJsBogusNamedImportSpecifier :: default ())
    }
}
impl FormatRule<biome_js_syntax::JsBogusParameter>
    for crate::js::bogus::bogus_parameter::FormatJsBogusParameter
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsBogusParameter,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_js_syntax::JsBogusParameter>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsBogusParameter {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsBogusParameter,
        crate::js::bogus::bogus_parameter::FormatJsBogusParameter,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::bogus::bogus_parameter::FormatJsBogusParameter::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsBogusParameter {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsBogusParameter,
        crate::js::bogus::bogus_parameter::FormatJsBogusParameter,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::bogus::bogus_parameter::FormatJsBogusParameter::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::JsBogusStatement>
    for crate::js::bogus::bogus_statement::FormatJsBogusStatement
{
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_js_syntax::JsBogusStatement,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_js_syntax::JsBogusStatement>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::JsBogusStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::JsBogusStatement,
        crate::js::bogus::bogus_statement::FormatJsBogusStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::bogus::bogus_statement::FormatJsBogusStatement::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::JsBogusStatement {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::JsBogusStatement,
        crate::js::bogus::bogus_statement::FormatJsBogusStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::bogus::bogus_statement::FormatJsBogusStatement::default(),
        )
    }
}
impl FormatRule<biome_js_syntax::TsBogusType> for crate::ts::bogus::bogus_type::FormatTsBogusType {
    type Context = JsFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_js_syntax::TsBogusType, f: &mut JsFormatter) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_js_syntax::TsBogusType>::fmt(self, node, f)
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::TsBogusType {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::TsBogusType,
        crate::ts::bogus::bogus_type::FormatTsBogusType,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::bogus::bogus_type::FormatTsBogusType::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::TsBogusType {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::TsBogusType,
        crate::ts::bogus::bogus_type::FormatTsBogusType,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::bogus::bogus_type::FormatTsBogusType::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyJsArrayAssignmentPatternElement {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyJsArrayAssignmentPatternElement,
        crate::js::any::array_assignment_pattern_element::FormatAnyJsArrayAssignmentPatternElement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: js :: any :: array_assignment_pattern_element :: FormatAnyJsArrayAssignmentPatternElement :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyJsArrayAssignmentPatternElement {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyJsArrayAssignmentPatternElement,
        crate::js::any::array_assignment_pattern_element::FormatAnyJsArrayAssignmentPatternElement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: js :: any :: array_assignment_pattern_element :: FormatAnyJsArrayAssignmentPatternElement :: default ())
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyJsArrayBindingPatternElement {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyJsArrayBindingPatternElement,
        crate::js::any::array_binding_pattern_element::FormatAnyJsArrayBindingPatternElement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: js :: any :: array_binding_pattern_element :: FormatAnyJsArrayBindingPatternElement :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyJsArrayBindingPatternElement {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyJsArrayBindingPatternElement,
        crate::js::any::array_binding_pattern_element::FormatAnyJsArrayBindingPatternElement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: js :: any :: array_binding_pattern_element :: FormatAnyJsArrayBindingPatternElement :: default ())
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyJsArrayElement {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyJsArrayElement,
        crate::js::any::array_element::FormatAnyJsArrayElement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::any::array_element::FormatAnyJsArrayElement::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyJsArrayElement {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyJsArrayElement,
        crate::js::any::array_element::FormatAnyJsArrayElement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::any::array_element::FormatAnyJsArrayElement::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyJsArrowFunctionParameters {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyJsArrowFunctionParameters,
        crate::js::any::arrow_function_parameters::FormatAnyJsArrowFunctionParameters,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::any::arrow_function_parameters::FormatAnyJsArrowFunctionParameters::default(
            ),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyJsArrowFunctionParameters {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyJsArrowFunctionParameters,
        crate::js::any::arrow_function_parameters::FormatAnyJsArrowFunctionParameters,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::any::arrow_function_parameters::FormatAnyJsArrowFunctionParameters::default(
            ),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyJsAssignment {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyJsAssignment,
        crate::js::any::assignment::FormatAnyJsAssignment,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::any::assignment::FormatAnyJsAssignment::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyJsAssignment {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyJsAssignment,
        crate::js::any::assignment::FormatAnyJsAssignment,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::any::assignment::FormatAnyJsAssignment::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyJsAssignmentPattern {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyJsAssignmentPattern,
        crate::js::any::assignment_pattern::FormatAnyJsAssignmentPattern,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::any::assignment_pattern::FormatAnyJsAssignmentPattern::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyJsAssignmentPattern {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyJsAssignmentPattern,
        crate::js::any::assignment_pattern::FormatAnyJsAssignmentPattern,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::any::assignment_pattern::FormatAnyJsAssignmentPattern::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyJsBinding {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyJsBinding,
        crate::js::any::binding::FormatAnyJsBinding,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, crate::js::any::binding::FormatAnyJsBinding::default())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyJsBinding {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyJsBinding,
        crate::js::any::binding::FormatAnyJsBinding,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, crate::js::any::binding::FormatAnyJsBinding::default())
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyJsBindingPattern {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyJsBindingPattern,
        crate::js::any::binding_pattern::FormatAnyJsBindingPattern,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::any::binding_pattern::FormatAnyJsBindingPattern::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyJsBindingPattern {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyJsBindingPattern,
        crate::js::any::binding_pattern::FormatAnyJsBindingPattern,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::any::binding_pattern::FormatAnyJsBindingPattern::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyJsCallArgument {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyJsCallArgument,
        crate::js::any::call_argument::FormatAnyJsCallArgument,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::any::call_argument::FormatAnyJsCallArgument::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyJsCallArgument {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyJsCallArgument,
        crate::js::any::call_argument::FormatAnyJsCallArgument,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::any::call_argument::FormatAnyJsCallArgument::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyJsClass {
    type Format<'a> =
        FormatRefWithRule<'a, biome_js_syntax::AnyJsClass, crate::js::any::class::FormatAnyJsClass>;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, crate::js::any::class::FormatAnyJsClass::default())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyJsClass {
    type Format =
        FormatOwnedWithRule<biome_js_syntax::AnyJsClass, crate::js::any::class::FormatAnyJsClass>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, crate::js::any::class::FormatAnyJsClass::default())
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyJsClassMember {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyJsClassMember,
        crate::js::any::class_member::FormatAnyJsClassMember,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::any::class_member::FormatAnyJsClassMember::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyJsClassMember {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyJsClassMember,
        crate::js::any::class_member::FormatAnyJsClassMember,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::any::class_member::FormatAnyJsClassMember::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyJsClassMemberName {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyJsClassMemberName,
        crate::js::any::class_member_name::FormatAnyJsClassMemberName,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::any::class_member_name::FormatAnyJsClassMemberName::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyJsClassMemberName {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyJsClassMemberName,
        crate::js::any::class_member_name::FormatAnyJsClassMemberName,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::any::class_member_name::FormatAnyJsClassMemberName::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyJsCombinedSpecifier {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyJsCombinedSpecifier,
        crate::js::any::combined_specifier::FormatAnyJsCombinedSpecifier,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::any::combined_specifier::FormatAnyJsCombinedSpecifier::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyJsCombinedSpecifier {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyJsCombinedSpecifier,
        crate::js::any::combined_specifier::FormatAnyJsCombinedSpecifier,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::any::combined_specifier::FormatAnyJsCombinedSpecifier::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyJsConstructorParameter {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyJsConstructorParameter,
        crate::js::any::constructor_parameter::FormatAnyJsConstructorParameter,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::any::constructor_parameter::FormatAnyJsConstructorParameter::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyJsConstructorParameter {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyJsConstructorParameter,
        crate::js::any::constructor_parameter::FormatAnyJsConstructorParameter,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::any::constructor_parameter::FormatAnyJsConstructorParameter::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyJsDeclaration {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyJsDeclaration,
        crate::js::any::declaration::FormatAnyJsDeclaration,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::any::declaration::FormatAnyJsDeclaration::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyJsDeclaration {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyJsDeclaration,
        crate::js::any::declaration::FormatAnyJsDeclaration,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::any::declaration::FormatAnyJsDeclaration::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyJsDeclarationClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyJsDeclarationClause,
        crate::js::any::declaration_clause::FormatAnyJsDeclarationClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::any::declaration_clause::FormatAnyJsDeclarationClause::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyJsDeclarationClause {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyJsDeclarationClause,
        crate::js::any::declaration_clause::FormatAnyJsDeclarationClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::any::declaration_clause::FormatAnyJsDeclarationClause::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyJsDecorator {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyJsDecorator,
        crate::js::any::decorator::FormatAnyJsDecorator,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::any::decorator::FormatAnyJsDecorator::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyJsDecorator {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyJsDecorator,
        crate::js::any::decorator::FormatAnyJsDecorator,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::any::decorator::FormatAnyJsDecorator::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyJsExportClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyJsExportClause,
        crate::js::any::export_clause::FormatAnyJsExportClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::any::export_clause::FormatAnyJsExportClause::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyJsExportClause {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyJsExportClause,
        crate::js::any::export_clause::FormatAnyJsExportClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::any::export_clause::FormatAnyJsExportClause::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyJsExportDefaultDeclaration {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyJsExportDefaultDeclaration,
        crate::js::any::export_default_declaration::FormatAnyJsExportDefaultDeclaration,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: js :: any :: export_default_declaration :: FormatAnyJsExportDefaultDeclaration :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyJsExportDefaultDeclaration {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyJsExportDefaultDeclaration,
        crate::js::any::export_default_declaration::FormatAnyJsExportDefaultDeclaration,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: js :: any :: export_default_declaration :: FormatAnyJsExportDefaultDeclaration :: default ())
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyJsExportNamedSpecifier {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyJsExportNamedSpecifier,
        crate::js::any::export_named_specifier::FormatAnyJsExportNamedSpecifier,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::any::export_named_specifier::FormatAnyJsExportNamedSpecifier::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyJsExportNamedSpecifier {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyJsExportNamedSpecifier,
        crate::js::any::export_named_specifier::FormatAnyJsExportNamedSpecifier,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::any::export_named_specifier::FormatAnyJsExportNamedSpecifier::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyJsExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyJsExpression,
        crate::js::any::expression::FormatAnyJsExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::any::expression::FormatAnyJsExpression::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyJsExpression {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyJsExpression,
        crate::js::any::expression::FormatAnyJsExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::any::expression::FormatAnyJsExpression::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyJsForInOrOfInitializer {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyJsForInOrOfInitializer,
        crate::js::any::for_in_or_of_initializer::FormatAnyJsForInOrOfInitializer,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::any::for_in_or_of_initializer::FormatAnyJsForInOrOfInitializer::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyJsForInOrOfInitializer {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyJsForInOrOfInitializer,
        crate::js::any::for_in_or_of_initializer::FormatAnyJsForInOrOfInitializer,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::any::for_in_or_of_initializer::FormatAnyJsForInOrOfInitializer::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyJsForInitializer {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyJsForInitializer,
        crate::js::any::for_initializer::FormatAnyJsForInitializer,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::any::for_initializer::FormatAnyJsForInitializer::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyJsForInitializer {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyJsForInitializer,
        crate::js::any::for_initializer::FormatAnyJsForInitializer,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::any::for_initializer::FormatAnyJsForInitializer::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyJsFormalParameter {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyJsFormalParameter,
        crate::js::any::formal_parameter::FormatAnyJsFormalParameter,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::any::formal_parameter::FormatAnyJsFormalParameter::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyJsFormalParameter {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyJsFormalParameter,
        crate::js::any::formal_parameter::FormatAnyJsFormalParameter,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::any::formal_parameter::FormatAnyJsFormalParameter::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyJsFunction {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyJsFunction,
        crate::js::any::function::FormatAnyJsFunction,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::any::function::FormatAnyJsFunction::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyJsFunction {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyJsFunction,
        crate::js::any::function::FormatAnyJsFunction,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::any::function::FormatAnyJsFunction::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyJsFunctionBody {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyJsFunctionBody,
        crate::js::any::function_body::FormatAnyJsFunctionBody,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::any::function_body::FormatAnyJsFunctionBody::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyJsFunctionBody {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyJsFunctionBody,
        crate::js::any::function_body::FormatAnyJsFunctionBody,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::any::function_body::FormatAnyJsFunctionBody::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyJsImportAssertionEntry {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyJsImportAssertionEntry,
        crate::js::any::import_assertion_entry::FormatAnyJsImportAssertionEntry,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::any::import_assertion_entry::FormatAnyJsImportAssertionEntry::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyJsImportAssertionEntry {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyJsImportAssertionEntry,
        crate::js::any::import_assertion_entry::FormatAnyJsImportAssertionEntry,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::any::import_assertion_entry::FormatAnyJsImportAssertionEntry::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyJsImportClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyJsImportClause,
        crate::js::any::import_clause::FormatAnyJsImportClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::any::import_clause::FormatAnyJsImportClause::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyJsImportClause {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyJsImportClause,
        crate::js::any::import_clause::FormatAnyJsImportClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::any::import_clause::FormatAnyJsImportClause::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyJsInProperty {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyJsInProperty,
        crate::js::any::in_property::FormatAnyJsInProperty,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::any::in_property::FormatAnyJsInProperty::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyJsInProperty {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyJsInProperty,
        crate::js::any::in_property::FormatAnyJsInProperty,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::any::in_property::FormatAnyJsInProperty::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyJsLiteralExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyJsLiteralExpression,
        crate::js::any::literal_expression::FormatAnyJsLiteralExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::any::literal_expression::FormatAnyJsLiteralExpression::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyJsLiteralExpression {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyJsLiteralExpression,
        crate::js::any::literal_expression::FormatAnyJsLiteralExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::any::literal_expression::FormatAnyJsLiteralExpression::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyJsMethodModifier {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyJsMethodModifier,
        crate::js::any::method_modifier::FormatAnyJsMethodModifier,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::any::method_modifier::FormatAnyJsMethodModifier::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyJsMethodModifier {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyJsMethodModifier,
        crate::js::any::method_modifier::FormatAnyJsMethodModifier,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::any::method_modifier::FormatAnyJsMethodModifier::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyJsModuleItem {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyJsModuleItem,
        crate::js::any::module_item::FormatAnyJsModuleItem,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::any::module_item::FormatAnyJsModuleItem::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyJsModuleItem {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyJsModuleItem,
        crate::js::any::module_item::FormatAnyJsModuleItem,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::any::module_item::FormatAnyJsModuleItem::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyJsModuleSource {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyJsModuleSource,
        crate::js::any::module_source::FormatAnyJsModuleSource,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::any::module_source::FormatAnyJsModuleSource::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyJsModuleSource {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyJsModuleSource,
        crate::js::any::module_source::FormatAnyJsModuleSource,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::any::module_source::FormatAnyJsModuleSource::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyJsName {
    type Format<'a> =
        FormatRefWithRule<'a, biome_js_syntax::AnyJsName, crate::js::any::name::FormatAnyJsName>;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, crate::js::any::name::FormatAnyJsName::default())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyJsName {
    type Format =
        FormatOwnedWithRule<biome_js_syntax::AnyJsName, crate::js::any::name::FormatAnyJsName>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, crate::js::any::name::FormatAnyJsName::default())
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyJsNamedImportSpecifier {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyJsNamedImportSpecifier,
        crate::js::any::named_import_specifier::FormatAnyJsNamedImportSpecifier,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::any::named_import_specifier::FormatAnyJsNamedImportSpecifier::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyJsNamedImportSpecifier {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyJsNamedImportSpecifier,
        crate::js::any::named_import_specifier::FormatAnyJsNamedImportSpecifier,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::any::named_import_specifier::FormatAnyJsNamedImportSpecifier::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyJsObjectAssignmentPatternMember {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyJsObjectAssignmentPatternMember,
        crate::js::any::object_assignment_pattern_member::FormatAnyJsObjectAssignmentPatternMember,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: js :: any :: object_assignment_pattern_member :: FormatAnyJsObjectAssignmentPatternMember :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyJsObjectAssignmentPatternMember {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyJsObjectAssignmentPatternMember,
        crate::js::any::object_assignment_pattern_member::FormatAnyJsObjectAssignmentPatternMember,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: js :: any :: object_assignment_pattern_member :: FormatAnyJsObjectAssignmentPatternMember :: default ())
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyJsObjectBindingPatternMember {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyJsObjectBindingPatternMember,
        crate::js::any::object_binding_pattern_member::FormatAnyJsObjectBindingPatternMember,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: js :: any :: object_binding_pattern_member :: FormatAnyJsObjectBindingPatternMember :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyJsObjectBindingPatternMember {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyJsObjectBindingPatternMember,
        crate::js::any::object_binding_pattern_member::FormatAnyJsObjectBindingPatternMember,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: js :: any :: object_binding_pattern_member :: FormatAnyJsObjectBindingPatternMember :: default ())
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyJsObjectMember {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyJsObjectMember,
        crate::js::any::object_member::FormatAnyJsObjectMember,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::any::object_member::FormatAnyJsObjectMember::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyJsObjectMember {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyJsObjectMember,
        crate::js::any::object_member::FormatAnyJsObjectMember,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::any::object_member::FormatAnyJsObjectMember::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyJsObjectMemberName {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyJsObjectMemberName,
        crate::js::any::object_member_name::FormatAnyJsObjectMemberName,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::any::object_member_name::FormatAnyJsObjectMemberName::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyJsObjectMemberName {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyJsObjectMemberName,
        crate::js::any::object_member_name::FormatAnyJsObjectMemberName,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::any::object_member_name::FormatAnyJsObjectMemberName::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyJsParameter {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyJsParameter,
        crate::js::any::parameter::FormatAnyJsParameter,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::any::parameter::FormatAnyJsParameter::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyJsParameter {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyJsParameter,
        crate::js::any::parameter::FormatAnyJsParameter,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::any::parameter::FormatAnyJsParameter::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyJsPropertyModifier {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyJsPropertyModifier,
        crate::js::any::property_modifier::FormatAnyJsPropertyModifier,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::any::property_modifier::FormatAnyJsPropertyModifier::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyJsPropertyModifier {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyJsPropertyModifier,
        crate::js::any::property_modifier::FormatAnyJsPropertyModifier,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::any::property_modifier::FormatAnyJsPropertyModifier::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyJsRoot {
    type Format<'a> =
        FormatRefWithRule<'a, biome_js_syntax::AnyJsRoot, crate::js::any::root::FormatAnyJsRoot>;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, crate::js::any::root::FormatAnyJsRoot::default())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyJsRoot {
    type Format =
        FormatOwnedWithRule<biome_js_syntax::AnyJsRoot, crate::js::any::root::FormatAnyJsRoot>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, crate::js::any::root::FormatAnyJsRoot::default())
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyJsStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyJsStatement,
        crate::js::any::statement::FormatAnyJsStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::any::statement::FormatAnyJsStatement::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyJsStatement {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyJsStatement,
        crate::js::any::statement::FormatAnyJsStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::any::statement::FormatAnyJsStatement::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyJsSwitchClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyJsSwitchClause,
        crate::js::any::switch_clause::FormatAnyJsSwitchClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::any::switch_clause::FormatAnyJsSwitchClause::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyJsSwitchClause {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyJsSwitchClause,
        crate::js::any::switch_clause::FormatAnyJsSwitchClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::any::switch_clause::FormatAnyJsSwitchClause::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyJsTemplateElement {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyJsTemplateElement,
        crate::js::any::template_element::FormatAnyJsTemplateElement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::js::any::template_element::FormatAnyJsTemplateElement::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyJsTemplateElement {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyJsTemplateElement,
        crate::js::any::template_element::FormatAnyJsTemplateElement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::js::any::template_element::FormatAnyJsTemplateElement::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyJsxAttribute {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyJsxAttribute,
        crate::jsx::any::attribute::FormatAnyJsxAttribute,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::jsx::any::attribute::FormatAnyJsxAttribute::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyJsxAttribute {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyJsxAttribute,
        crate::jsx::any::attribute::FormatAnyJsxAttribute,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::jsx::any::attribute::FormatAnyJsxAttribute::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyJsxAttributeName {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyJsxAttributeName,
        crate::jsx::any::attribute_name::FormatAnyJsxAttributeName,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::jsx::any::attribute_name::FormatAnyJsxAttributeName::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyJsxAttributeName {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyJsxAttributeName,
        crate::jsx::any::attribute_name::FormatAnyJsxAttributeName,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::jsx::any::attribute_name::FormatAnyJsxAttributeName::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyJsxAttributeValue {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyJsxAttributeValue,
        crate::jsx::any::attribute_value::FormatAnyJsxAttributeValue,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::jsx::any::attribute_value::FormatAnyJsxAttributeValue::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyJsxAttributeValue {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyJsxAttributeValue,
        crate::jsx::any::attribute_value::FormatAnyJsxAttributeValue,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::jsx::any::attribute_value::FormatAnyJsxAttributeValue::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyJsxChild {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyJsxChild,
        crate::jsx::any::child::FormatAnyJsxChild,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, crate::jsx::any::child::FormatAnyJsxChild::default())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyJsxChild {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyJsxChild,
        crate::jsx::any::child::FormatAnyJsxChild,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, crate::jsx::any::child::FormatAnyJsxChild::default())
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyJsxElementName {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyJsxElementName,
        crate::jsx::any::element_name::FormatAnyJsxElementName,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::jsx::any::element_name::FormatAnyJsxElementName::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyJsxElementName {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyJsxElementName,
        crate::jsx::any::element_name::FormatAnyJsxElementName,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::jsx::any::element_name::FormatAnyJsxElementName::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyJsxName {
    type Format<'a> =
        FormatRefWithRule<'a, biome_js_syntax::AnyJsxName, crate::jsx::any::name::FormatAnyJsxName>;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, crate::jsx::any::name::FormatAnyJsxName::default())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyJsxName {
    type Format =
        FormatOwnedWithRule<biome_js_syntax::AnyJsxName, crate::jsx::any::name::FormatAnyJsxName>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, crate::jsx::any::name::FormatAnyJsxName::default())
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyJsxObjectName {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyJsxObjectName,
        crate::jsx::any::object_name::FormatAnyJsxObjectName,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::jsx::any::object_name::FormatAnyJsxObjectName::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyJsxObjectName {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyJsxObjectName,
        crate::jsx::any::object_name::FormatAnyJsxObjectName,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::jsx::any::object_name::FormatAnyJsxObjectName::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyJsxTag {
    type Format<'a> =
        FormatRefWithRule<'a, biome_js_syntax::AnyJsxTag, crate::jsx::any::tag::FormatAnyJsxTag>;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, crate::jsx::any::tag::FormatAnyJsxTag::default())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyJsxTag {
    type Format =
        FormatOwnedWithRule<biome_js_syntax::AnyJsxTag, crate::jsx::any::tag::FormatAnyJsxTag>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, crate::jsx::any::tag::FormatAnyJsxTag::default())
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyTsEnumMemberName {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyTsEnumMemberName,
        crate::ts::any::enum_member_name::FormatAnyTsEnumMemberName,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::any::enum_member_name::FormatAnyTsEnumMemberName::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyTsEnumMemberName {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyTsEnumMemberName,
        crate::ts::any::enum_member_name::FormatAnyTsEnumMemberName,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::any::enum_member_name::FormatAnyTsEnumMemberName::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyTsExternalModuleDeclarationBody {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyTsExternalModuleDeclarationBody,
        crate::ts::any::external_module_declaration_body::FormatAnyTsExternalModuleDeclarationBody,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: ts :: any :: external_module_declaration_body :: FormatAnyTsExternalModuleDeclarationBody :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyTsExternalModuleDeclarationBody {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyTsExternalModuleDeclarationBody,
        crate::ts::any::external_module_declaration_body::FormatAnyTsExternalModuleDeclarationBody,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: ts :: any :: external_module_declaration_body :: FormatAnyTsExternalModuleDeclarationBody :: default ())
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyTsIdentifierBinding {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyTsIdentifierBinding,
        crate::ts::any::identifier_binding::FormatAnyTsIdentifierBinding,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::any::identifier_binding::FormatAnyTsIdentifierBinding::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyTsIdentifierBinding {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyTsIdentifierBinding,
        crate::ts::any::identifier_binding::FormatAnyTsIdentifierBinding,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::any::identifier_binding::FormatAnyTsIdentifierBinding::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyTsIndexSignatureModifier {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyTsIndexSignatureModifier,
        crate::ts::any::index_signature_modifier::FormatAnyTsIndexSignatureModifier,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::any::index_signature_modifier::FormatAnyTsIndexSignatureModifier::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyTsIndexSignatureModifier {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyTsIndexSignatureModifier,
        crate::ts::any::index_signature_modifier::FormatAnyTsIndexSignatureModifier,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::any::index_signature_modifier::FormatAnyTsIndexSignatureModifier::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyTsMethodSignatureModifier {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyTsMethodSignatureModifier,
        crate::ts::any::method_signature_modifier::FormatAnyTsMethodSignatureModifier,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::any::method_signature_modifier::FormatAnyTsMethodSignatureModifier::default(
            ),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyTsMethodSignatureModifier {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyTsMethodSignatureModifier,
        crate::ts::any::method_signature_modifier::FormatAnyTsMethodSignatureModifier,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::any::method_signature_modifier::FormatAnyTsMethodSignatureModifier::default(
            ),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyTsModuleName {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyTsModuleName,
        crate::ts::any::module_name::FormatAnyTsModuleName,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::any::module_name::FormatAnyTsModuleName::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyTsModuleName {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyTsModuleName,
        crate::ts::any::module_name::FormatAnyTsModuleName,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::any::module_name::FormatAnyTsModuleName::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyTsModuleReference {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyTsModuleReference,
        crate::ts::any::module_reference::FormatAnyTsModuleReference,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::any::module_reference::FormatAnyTsModuleReference::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyTsModuleReference {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyTsModuleReference,
        crate::ts::any::module_reference::FormatAnyTsModuleReference,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::any::module_reference::FormatAnyTsModuleReference::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyTsName {
    type Format<'a> =
        FormatRefWithRule<'a, biome_js_syntax::AnyTsName, crate::ts::any::name::FormatAnyTsName>;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, crate::ts::any::name::FormatAnyTsName::default())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyTsName {
    type Format =
        FormatOwnedWithRule<biome_js_syntax::AnyTsName, crate::ts::any::name::FormatAnyTsName>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, crate::ts::any::name::FormatAnyTsName::default())
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyTsPropertyAnnotation {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyTsPropertyAnnotation,
        crate::ts::any::property_annotation::FormatAnyTsPropertyAnnotation,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::any::property_annotation::FormatAnyTsPropertyAnnotation::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyTsPropertyAnnotation {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyTsPropertyAnnotation,
        crate::ts::any::property_annotation::FormatAnyTsPropertyAnnotation,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::any::property_annotation::FormatAnyTsPropertyAnnotation::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyTsPropertyParameterModifier {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyTsPropertyParameterModifier,
        crate::ts::any::property_parameter_modifier::FormatAnyTsPropertyParameterModifier,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: ts :: any :: property_parameter_modifier :: FormatAnyTsPropertyParameterModifier :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyTsPropertyParameterModifier {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyTsPropertyParameterModifier,
        crate::ts::any::property_parameter_modifier::FormatAnyTsPropertyParameterModifier,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: ts :: any :: property_parameter_modifier :: FormatAnyTsPropertyParameterModifier :: default ())
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyTsPropertySignatureAnnotation {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyTsPropertySignatureAnnotation,
        crate::ts::any::property_signature_annotation::FormatAnyTsPropertySignatureAnnotation,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: ts :: any :: property_signature_annotation :: FormatAnyTsPropertySignatureAnnotation :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyTsPropertySignatureAnnotation {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyTsPropertySignatureAnnotation,
        crate::ts::any::property_signature_annotation::FormatAnyTsPropertySignatureAnnotation,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: ts :: any :: property_signature_annotation :: FormatAnyTsPropertySignatureAnnotation :: default ())
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyTsPropertySignatureModifier {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyTsPropertySignatureModifier,
        crate::ts::any::property_signature_modifier::FormatAnyTsPropertySignatureModifier,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: ts :: any :: property_signature_modifier :: FormatAnyTsPropertySignatureModifier :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyTsPropertySignatureModifier {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyTsPropertySignatureModifier,
        crate::ts::any::property_signature_modifier::FormatAnyTsPropertySignatureModifier,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: ts :: any :: property_signature_modifier :: FormatAnyTsPropertySignatureModifier :: default ())
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyTsReturnType {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyTsReturnType,
        crate::ts::any::return_type::FormatAnyTsReturnType,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::any::return_type::FormatAnyTsReturnType::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyTsReturnType {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyTsReturnType,
        crate::ts::any::return_type::FormatAnyTsReturnType,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::any::return_type::FormatAnyTsReturnType::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyTsTemplateElement {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyTsTemplateElement,
        crate::ts::any::template_element::FormatAnyTsTemplateElement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::any::template_element::FormatAnyTsTemplateElement::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyTsTemplateElement {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyTsTemplateElement,
        crate::ts::any::template_element::FormatAnyTsTemplateElement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::any::template_element::FormatAnyTsTemplateElement::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyTsTupleTypeElement {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyTsTupleTypeElement,
        crate::ts::any::tuple_type_element::FormatAnyTsTupleTypeElement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::any::tuple_type_element::FormatAnyTsTupleTypeElement::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyTsTupleTypeElement {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyTsTupleTypeElement,
        crate::ts::any::tuple_type_element::FormatAnyTsTupleTypeElement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::any::tuple_type_element::FormatAnyTsTupleTypeElement::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyTsType {
    type Format<'a> =
        FormatRefWithRule<'a, biome_js_syntax::AnyTsType, crate::ts::any::ts_type::FormatAnyTsType>;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, crate::ts::any::ts_type::FormatAnyTsType::default())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyTsType {
    type Format =
        FormatOwnedWithRule<biome_js_syntax::AnyTsType, crate::ts::any::ts_type::FormatAnyTsType>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, crate::ts::any::ts_type::FormatAnyTsType::default())
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyTsTypeMember {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyTsTypeMember,
        crate::ts::any::type_member::FormatAnyTsTypeMember,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::any::type_member::FormatAnyTsTypeMember::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyTsTypeMember {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyTsTypeMember,
        crate::ts::any::type_member::FormatAnyTsTypeMember,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::any::type_member::FormatAnyTsTypeMember::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyTsTypeParameterModifier {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyTsTypeParameterModifier,
        crate::ts::any::type_parameter_modifier::FormatAnyTsTypeParameterModifier,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::any::type_parameter_modifier::FormatAnyTsTypeParameterModifier::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyTsTypeParameterModifier {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyTsTypeParameterModifier,
        crate::ts::any::type_parameter_modifier::FormatAnyTsTypeParameterModifier,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::any::type_parameter_modifier::FormatAnyTsTypeParameterModifier::default(),
        )
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyTsTypePredicateParameterName {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyTsTypePredicateParameterName,
        crate::ts::any::type_predicate_parameter_name::FormatAnyTsTypePredicateParameterName,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: ts :: any :: type_predicate_parameter_name :: FormatAnyTsTypePredicateParameterName :: default ())
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyTsTypePredicateParameterName {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyTsTypePredicateParameterName,
        crate::ts::any::type_predicate_parameter_name::FormatAnyTsTypePredicateParameterName,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: ts :: any :: type_predicate_parameter_name :: FormatAnyTsTypePredicateParameterName :: default ())
    }
}
impl AsFormat<JsFormatContext> for biome_js_syntax::AnyTsVariableAnnotation {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_js_syntax::AnyTsVariableAnnotation,
        crate::ts::any::variable_annotation::FormatAnyTsVariableAnnotation,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::ts::any::variable_annotation::FormatAnyTsVariableAnnotation::default(),
        )
    }
}
impl IntoFormat<JsFormatContext> for biome_js_syntax::AnyTsVariableAnnotation {
    type Format = FormatOwnedWithRule<
        biome_js_syntax::AnyTsVariableAnnotation,
        crate::ts::any::variable_annotation::FormatAnyTsVariableAnnotation,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::ts::any::variable_annotation::FormatAnyTsVariableAnnotation::default(),
        )
    }
}
