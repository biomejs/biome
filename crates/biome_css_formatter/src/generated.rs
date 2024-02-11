//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::{
    AsFormat, CssFormatContext, CssFormatter, FormatBogusNodeRule, FormatNodeRule, IntoFormat,
};
use biome_formatter::{FormatOwnedWithRule, FormatRefWithRule, FormatResult, FormatRule};
impl FormatRule<biome_css_syntax::CssRoot> for crate::css::auxiliary::root::FormatCssRoot {
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_css_syntax::CssRoot, f: &mut CssFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssRoot>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssRoot {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssRoot,
        crate::css::auxiliary::root::FormatCssRoot,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(self, crate::css::auxiliary::root::FormatCssRoot::default())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssRoot {
    type Format =
        FormatOwnedWithRule<biome_css_syntax::CssRoot, crate::css::auxiliary::root::FormatCssRoot>;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(self, crate::css::auxiliary::root::FormatCssRoot::default())
    }
}
impl FormatRule<biome_css_syntax::CssQualifiedRule>
    for crate::css::auxiliary::qualified_rule::FormatCssQualifiedRule
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssQualifiedRule,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssQualifiedRule>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssQualifiedRule {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssQualifiedRule,
        crate::css::auxiliary::qualified_rule::FormatCssQualifiedRule,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::qualified_rule::FormatCssQualifiedRule::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssQualifiedRule {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssQualifiedRule,
        crate::css::auxiliary::qualified_rule::FormatCssQualifiedRule,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::qualified_rule::FormatCssQualifiedRule::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssNestedQualifiedRule>
    for crate::css::auxiliary::nested_qualified_rule::FormatCssNestedQualifiedRule
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssNestedQualifiedRule,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssNestedQualifiedRule>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssNestedQualifiedRule {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssNestedQualifiedRule,
        crate::css::auxiliary::nested_qualified_rule::FormatCssNestedQualifiedRule,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::nested_qualified_rule::FormatCssNestedQualifiedRule::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssNestedQualifiedRule {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssNestedQualifiedRule,
        crate::css::auxiliary::nested_qualified_rule::FormatCssNestedQualifiedRule,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::nested_qualified_rule::FormatCssNestedQualifiedRule::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssAtRule> for crate::css::statements::at_rule::FormatCssAtRule {
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_css_syntax::CssAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssAtRule>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssAtRule {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssAtRule,
        crate::css::statements::at_rule::FormatCssAtRule,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::statements::at_rule::FormatCssAtRule::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssAtRule {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssAtRule,
        crate::css::statements::at_rule::FormatCssAtRule,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::statements::at_rule::FormatCssAtRule::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssComplexSelector>
    for crate::css::selectors::complex_selector::FormatCssComplexSelector
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssComplexSelector,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssComplexSelector>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssComplexSelector {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssComplexSelector,
        crate::css::selectors::complex_selector::FormatCssComplexSelector,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::selectors::complex_selector::FormatCssComplexSelector::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssComplexSelector {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssComplexSelector,
        crate::css::selectors::complex_selector::FormatCssComplexSelector,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::selectors::complex_selector::FormatCssComplexSelector::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssCompoundSelector>
    for crate::css::selectors::compound_selector::FormatCssCompoundSelector
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssCompoundSelector,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssCompoundSelector>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssCompoundSelector {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssCompoundSelector,
        crate::css::selectors::compound_selector::FormatCssCompoundSelector,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::selectors::compound_selector::FormatCssCompoundSelector::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssCompoundSelector {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssCompoundSelector,
        crate::css::selectors::compound_selector::FormatCssCompoundSelector,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::selectors::compound_selector::FormatCssCompoundSelector::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssUniversalSelector>
    for crate::css::selectors::universal_selector::FormatCssUniversalSelector
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssUniversalSelector,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssUniversalSelector>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssUniversalSelector {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssUniversalSelector,
        crate::css::selectors::universal_selector::FormatCssUniversalSelector,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::selectors::universal_selector::FormatCssUniversalSelector::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssUniversalSelector {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssUniversalSelector,
        crate::css::selectors::universal_selector::FormatCssUniversalSelector,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::selectors::universal_selector::FormatCssUniversalSelector::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssTypeSelector>
    for crate::css::selectors::type_selector::FormatCssTypeSelector
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssTypeSelector,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssTypeSelector>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssTypeSelector {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssTypeSelector,
        crate::css::selectors::type_selector::FormatCssTypeSelector,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::selectors::type_selector::FormatCssTypeSelector::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssTypeSelector {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssTypeSelector,
        crate::css::selectors::type_selector::FormatCssTypeSelector,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::selectors::type_selector::FormatCssTypeSelector::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssIdSelector>
    for crate::css::selectors::id_selector::FormatCssIdSelector
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssIdSelector,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssIdSelector>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssIdSelector {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssIdSelector,
        crate::css::selectors::id_selector::FormatCssIdSelector,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::selectors::id_selector::FormatCssIdSelector::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssIdSelector {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssIdSelector,
        crate::css::selectors::id_selector::FormatCssIdSelector,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::selectors::id_selector::FormatCssIdSelector::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssClassSelector>
    for crate::css::selectors::class_selector::FormatCssClassSelector
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssClassSelector,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssClassSelector>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssClassSelector {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssClassSelector,
        crate::css::selectors::class_selector::FormatCssClassSelector,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::selectors::class_selector::FormatCssClassSelector::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssClassSelector {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssClassSelector,
        crate::css::selectors::class_selector::FormatCssClassSelector,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::selectors::class_selector::FormatCssClassSelector::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssAttributeSelector>
    for crate::css::selectors::attribute_selector::FormatCssAttributeSelector
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssAttributeSelector,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssAttributeSelector>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssAttributeSelector {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssAttributeSelector,
        crate::css::selectors::attribute_selector::FormatCssAttributeSelector,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::selectors::attribute_selector::FormatCssAttributeSelector::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssAttributeSelector {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssAttributeSelector,
        crate::css::selectors::attribute_selector::FormatCssAttributeSelector,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::selectors::attribute_selector::FormatCssAttributeSelector::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssPseudoClassSelector>
    for crate::css::selectors::pseudo_class_selector::FormatCssPseudoClassSelector
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssPseudoClassSelector,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssPseudoClassSelector>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssPseudoClassSelector {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssPseudoClassSelector,
        crate::css::selectors::pseudo_class_selector::FormatCssPseudoClassSelector,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::selectors::pseudo_class_selector::FormatCssPseudoClassSelector::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssPseudoClassSelector {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssPseudoClassSelector,
        crate::css::selectors::pseudo_class_selector::FormatCssPseudoClassSelector,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::selectors::pseudo_class_selector::FormatCssPseudoClassSelector::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssPseudoElementSelector>
    for crate::css::selectors::pseudo_element_selector::FormatCssPseudoElementSelector
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssPseudoElementSelector,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssPseudoElementSelector>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssPseudoElementSelector {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssPseudoElementSelector,
        crate::css::selectors::pseudo_element_selector::FormatCssPseudoElementSelector,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::selectors::pseudo_element_selector::FormatCssPseudoElementSelector::default(
            ),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssPseudoElementSelector {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssPseudoElementSelector,
        crate::css::selectors::pseudo_element_selector::FormatCssPseudoElementSelector,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::selectors::pseudo_element_selector::FormatCssPseudoElementSelector::default(
            ),
        )
    }
}
impl FormatRule<biome_css_syntax::CssNamespace>
    for crate::css::auxiliary::namespace::FormatCssNamespace
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_css_syntax::CssNamespace, f: &mut CssFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssNamespace>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssNamespace {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssNamespace,
        crate::css::auxiliary::namespace::FormatCssNamespace,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::namespace::FormatCssNamespace::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssNamespace {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssNamespace,
        crate::css::auxiliary::namespace::FormatCssNamespace,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::namespace::FormatCssNamespace::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssIdentifier>
    for crate::css::value::identifier::FormatCssIdentifier
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssIdentifier,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssIdentifier>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssIdentifier {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssIdentifier,
        crate::css::value::identifier::FormatCssIdentifier,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::value::identifier::FormatCssIdentifier::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssIdentifier {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssIdentifier,
        crate::css::value::identifier::FormatCssIdentifier,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::value::identifier::FormatCssIdentifier::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssNamedNamespacePrefix>
    for crate::css::auxiliary::named_namespace_prefix::FormatCssNamedNamespacePrefix
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssNamedNamespacePrefix,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssNamedNamespacePrefix>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssNamedNamespacePrefix {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssNamedNamespacePrefix,
        crate::css::auxiliary::named_namespace_prefix::FormatCssNamedNamespacePrefix,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::named_namespace_prefix::FormatCssNamedNamespacePrefix::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssNamedNamespacePrefix {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssNamedNamespacePrefix,
        crate::css::auxiliary::named_namespace_prefix::FormatCssNamedNamespacePrefix,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::named_namespace_prefix::FormatCssNamedNamespacePrefix::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssUniversalNamespacePrefix>
    for crate::css::auxiliary::universal_namespace_prefix::FormatCssUniversalNamespacePrefix
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssUniversalNamespacePrefix,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssUniversalNamespacePrefix>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssUniversalNamespacePrefix {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssUniversalNamespacePrefix,
        crate::css::auxiliary::universal_namespace_prefix::FormatCssUniversalNamespacePrefix,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule :: new (self , crate :: css :: auxiliary :: universal_namespace_prefix :: FormatCssUniversalNamespacePrefix :: default ())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssUniversalNamespacePrefix {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssUniversalNamespacePrefix,
        crate::css::auxiliary::universal_namespace_prefix::FormatCssUniversalNamespacePrefix,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule :: new (self , crate :: css :: auxiliary :: universal_namespace_prefix :: FormatCssUniversalNamespacePrefix :: default ())
    }
}
impl FormatRule<biome_css_syntax::CssCustomIdentifier>
    for crate::css::value::custom_identifier::FormatCssCustomIdentifier
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssCustomIdentifier,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssCustomIdentifier>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssCustomIdentifier {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssCustomIdentifier,
        crate::css::value::custom_identifier::FormatCssCustomIdentifier,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::value::custom_identifier::FormatCssCustomIdentifier::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssCustomIdentifier {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssCustomIdentifier,
        crate::css::value::custom_identifier::FormatCssCustomIdentifier,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::value::custom_identifier::FormatCssCustomIdentifier::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssAttributeName>
    for crate::css::auxiliary::attribute_name::FormatCssAttributeName
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssAttributeName,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssAttributeName>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssAttributeName {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssAttributeName,
        crate::css::auxiliary::attribute_name::FormatCssAttributeName,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::attribute_name::FormatCssAttributeName::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssAttributeName {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssAttributeName,
        crate::css::auxiliary::attribute_name::FormatCssAttributeName,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::attribute_name::FormatCssAttributeName::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssAttributeMatcher>
    for crate::css::auxiliary::attribute_matcher::FormatCssAttributeMatcher
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssAttributeMatcher,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssAttributeMatcher>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssAttributeMatcher {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssAttributeMatcher,
        crate::css::auxiliary::attribute_matcher::FormatCssAttributeMatcher,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::attribute_matcher::FormatCssAttributeMatcher::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssAttributeMatcher {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssAttributeMatcher,
        crate::css::auxiliary::attribute_matcher::FormatCssAttributeMatcher,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::attribute_matcher::FormatCssAttributeMatcher::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssAttributeMatcherValue>
    for crate::css::auxiliary::attribute_matcher_value::FormatCssAttributeMatcherValue
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssAttributeMatcherValue,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssAttributeMatcherValue>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssAttributeMatcherValue {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssAttributeMatcherValue,
        crate::css::auxiliary::attribute_matcher_value::FormatCssAttributeMatcherValue,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::attribute_matcher_value::FormatCssAttributeMatcherValue::default(
            ),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssAttributeMatcherValue {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssAttributeMatcherValue,
        crate::css::auxiliary::attribute_matcher_value::FormatCssAttributeMatcherValue,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::attribute_matcher_value::FormatCssAttributeMatcherValue::default(
            ),
        )
    }
}
impl FormatRule<biome_css_syntax::CssString> for crate::css::value::string::FormatCssString {
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_css_syntax::CssString, f: &mut CssFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssString>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssString {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssString,
        crate::css::value::string::FormatCssString,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(self, crate::css::value::string::FormatCssString::default())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssString {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssString,
        crate::css::value::string::FormatCssString,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(self, crate::css::value::string::FormatCssString::default())
    }
}
impl FormatRule<biome_css_syntax::CssPseudoClassIdentifier>
    for crate::css::pseudo::pseudo_class_identifier::FormatCssPseudoClassIdentifier
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssPseudoClassIdentifier,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssPseudoClassIdentifier>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssPseudoClassIdentifier {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssPseudoClassIdentifier,
        crate::css::pseudo::pseudo_class_identifier::FormatCssPseudoClassIdentifier,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::pseudo::pseudo_class_identifier::FormatCssPseudoClassIdentifier::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssPseudoClassIdentifier {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssPseudoClassIdentifier,
        crate::css::pseudo::pseudo_class_identifier::FormatCssPseudoClassIdentifier,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::pseudo::pseudo_class_identifier::FormatCssPseudoClassIdentifier::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssPseudoClassFunctionIdentifier>
    for crate::css::pseudo::pseudo_class_function_identifier::FormatCssPseudoClassFunctionIdentifier
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssPseudoClassFunctionIdentifier,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssPseudoClassFunctionIdentifier>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssPseudoClassFunctionIdentifier {
    type Format < 'a > = FormatRefWithRule < 'a , biome_css_syntax :: CssPseudoClassFunctionIdentifier , crate :: css :: pseudo :: pseudo_class_function_identifier :: FormatCssPseudoClassFunctionIdentifier > ;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule :: new (self , crate :: css :: pseudo :: pseudo_class_function_identifier :: FormatCssPseudoClassFunctionIdentifier :: default ())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssPseudoClassFunctionIdentifier {
    type Format = FormatOwnedWithRule < biome_css_syntax :: CssPseudoClassFunctionIdentifier , crate :: css :: pseudo :: pseudo_class_function_identifier :: FormatCssPseudoClassFunctionIdentifier > ;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule :: new (self , crate :: css :: pseudo :: pseudo_class_function_identifier :: FormatCssPseudoClassFunctionIdentifier :: default ())
    }
}
impl FormatRule<biome_css_syntax::CssPseudoClassFunctionSelector>
    for crate::css::selectors::pseudo_class_function_selector::FormatCssPseudoClassFunctionSelector
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssPseudoClassFunctionSelector,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssPseudoClassFunctionSelector>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssPseudoClassFunctionSelector {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssPseudoClassFunctionSelector,
        crate::css::selectors::pseudo_class_function_selector::FormatCssPseudoClassFunctionSelector,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule :: new (self , crate :: css :: selectors :: pseudo_class_function_selector :: FormatCssPseudoClassFunctionSelector :: default ())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssPseudoClassFunctionSelector {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssPseudoClassFunctionSelector,
        crate::css::selectors::pseudo_class_function_selector::FormatCssPseudoClassFunctionSelector,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule :: new (self , crate :: css :: selectors :: pseudo_class_function_selector :: FormatCssPseudoClassFunctionSelector :: default ())
    }
}
impl FormatRule < biome_css_syntax :: CssPseudoClassFunctionSelectorList > for crate :: css :: pseudo :: pseudo_class_function_selector_list :: FormatCssPseudoClassFunctionSelectorList { type Context = CssFormatContext ; # [inline (always)] fn fmt (& self , node : & biome_css_syntax :: CssPseudoClassFunctionSelectorList , f : & mut CssFormatter) -> FormatResult < () > { FormatNodeRule :: < biome_css_syntax :: CssPseudoClassFunctionSelectorList > :: fmt (self , node , f) } }
impl AsFormat<CssFormatContext> for biome_css_syntax::CssPseudoClassFunctionSelectorList {
    type Format < 'a > = FormatRefWithRule < 'a , biome_css_syntax :: CssPseudoClassFunctionSelectorList , crate :: css :: pseudo :: pseudo_class_function_selector_list :: FormatCssPseudoClassFunctionSelectorList > ;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule :: new (self , crate :: css :: pseudo :: pseudo_class_function_selector_list :: FormatCssPseudoClassFunctionSelectorList :: default ())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssPseudoClassFunctionSelectorList {
    type Format = FormatOwnedWithRule < biome_css_syntax :: CssPseudoClassFunctionSelectorList , crate :: css :: pseudo :: pseudo_class_function_selector_list :: FormatCssPseudoClassFunctionSelectorList > ;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule :: new (self , crate :: css :: pseudo :: pseudo_class_function_selector_list :: FormatCssPseudoClassFunctionSelectorList :: default ())
    }
}
impl FormatRule < biome_css_syntax :: CssPseudoClassFunctionCompoundSelector > for crate :: css :: selectors :: pseudo_class_function_compound_selector :: FormatCssPseudoClassFunctionCompoundSelector { type Context = CssFormatContext ; # [inline (always)] fn fmt (& self , node : & biome_css_syntax :: CssPseudoClassFunctionCompoundSelector , f : & mut CssFormatter) -> FormatResult < () > { FormatNodeRule :: < biome_css_syntax :: CssPseudoClassFunctionCompoundSelector > :: fmt (self , node , f) } }
impl AsFormat<CssFormatContext> for biome_css_syntax::CssPseudoClassFunctionCompoundSelector {
    type Format < 'a > = FormatRefWithRule < 'a , biome_css_syntax :: CssPseudoClassFunctionCompoundSelector , crate :: css :: selectors :: pseudo_class_function_compound_selector :: FormatCssPseudoClassFunctionCompoundSelector > ;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule :: new (self , crate :: css :: selectors :: pseudo_class_function_compound_selector :: FormatCssPseudoClassFunctionCompoundSelector :: default ())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssPseudoClassFunctionCompoundSelector {
    type Format = FormatOwnedWithRule < biome_css_syntax :: CssPseudoClassFunctionCompoundSelector , crate :: css :: selectors :: pseudo_class_function_compound_selector :: FormatCssPseudoClassFunctionCompoundSelector > ;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule :: new (self , crate :: css :: selectors :: pseudo_class_function_compound_selector :: FormatCssPseudoClassFunctionCompoundSelector :: default ())
    }
}
impl FormatRule < biome_css_syntax :: CssPseudoClassFunctionCompoundSelectorList > for crate :: css :: pseudo :: pseudo_class_function_compound_selector_list :: FormatCssPseudoClassFunctionCompoundSelectorList { type Context = CssFormatContext ; # [inline (always)] fn fmt (& self , node : & biome_css_syntax :: CssPseudoClassFunctionCompoundSelectorList , f : & mut CssFormatter) -> FormatResult < () > { FormatNodeRule :: < biome_css_syntax :: CssPseudoClassFunctionCompoundSelectorList > :: fmt (self , node , f) } }
impl AsFormat<CssFormatContext> for biome_css_syntax::CssPseudoClassFunctionCompoundSelectorList {
    type Format < 'a > = FormatRefWithRule < 'a , biome_css_syntax :: CssPseudoClassFunctionCompoundSelectorList , crate :: css :: pseudo :: pseudo_class_function_compound_selector_list :: FormatCssPseudoClassFunctionCompoundSelectorList > ;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule :: new (self , crate :: css :: pseudo :: pseudo_class_function_compound_selector_list :: FormatCssPseudoClassFunctionCompoundSelectorList :: default ())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssPseudoClassFunctionCompoundSelectorList {
    type Format = FormatOwnedWithRule < biome_css_syntax :: CssPseudoClassFunctionCompoundSelectorList , crate :: css :: pseudo :: pseudo_class_function_compound_selector_list :: FormatCssPseudoClassFunctionCompoundSelectorList > ;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule :: new (self , crate :: css :: pseudo :: pseudo_class_function_compound_selector_list :: FormatCssPseudoClassFunctionCompoundSelectorList :: default ())
    }
}
impl FormatRule < biome_css_syntax :: CssPseudoClassFunctionRelativeSelectorList > for crate :: css :: pseudo :: pseudo_class_function_relative_selector_list :: FormatCssPseudoClassFunctionRelativeSelectorList { type Context = CssFormatContext ; # [inline (always)] fn fmt (& self , node : & biome_css_syntax :: CssPseudoClassFunctionRelativeSelectorList , f : & mut CssFormatter) -> FormatResult < () > { FormatNodeRule :: < biome_css_syntax :: CssPseudoClassFunctionRelativeSelectorList > :: fmt (self , node , f) } }
impl AsFormat<CssFormatContext> for biome_css_syntax::CssPseudoClassFunctionRelativeSelectorList {
    type Format < 'a > = FormatRefWithRule < 'a , biome_css_syntax :: CssPseudoClassFunctionRelativeSelectorList , crate :: css :: pseudo :: pseudo_class_function_relative_selector_list :: FormatCssPseudoClassFunctionRelativeSelectorList > ;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule :: new (self , crate :: css :: pseudo :: pseudo_class_function_relative_selector_list :: FormatCssPseudoClassFunctionRelativeSelectorList :: default ())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssPseudoClassFunctionRelativeSelectorList {
    type Format = FormatOwnedWithRule < biome_css_syntax :: CssPseudoClassFunctionRelativeSelectorList , crate :: css :: pseudo :: pseudo_class_function_relative_selector_list :: FormatCssPseudoClassFunctionRelativeSelectorList > ;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule :: new (self , crate :: css :: pseudo :: pseudo_class_function_relative_selector_list :: FormatCssPseudoClassFunctionRelativeSelectorList :: default ())
    }
}
impl FormatRule<biome_css_syntax::CssPseudoClassFunctionValueList>
    for crate::css::pseudo::pseudo_class_function_value_list::FormatCssPseudoClassFunctionValueList
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssPseudoClassFunctionValueList,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssPseudoClassFunctionValueList>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssPseudoClassFunctionValueList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssPseudoClassFunctionValueList,
        crate::css::pseudo::pseudo_class_function_value_list::FormatCssPseudoClassFunctionValueList,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule :: new (self , crate :: css :: pseudo :: pseudo_class_function_value_list :: FormatCssPseudoClassFunctionValueList :: default ())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssPseudoClassFunctionValueList {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssPseudoClassFunctionValueList,
        crate::css::pseudo::pseudo_class_function_value_list::FormatCssPseudoClassFunctionValueList,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule :: new (self , crate :: css :: pseudo :: pseudo_class_function_value_list :: FormatCssPseudoClassFunctionValueList :: default ())
    }
}
impl FormatRule<biome_css_syntax::CssPseudoClassFunctionNth>
    for crate::css::pseudo::pseudo_class_function_nth::FormatCssPseudoClassFunctionNth
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssPseudoClassFunctionNth,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssPseudoClassFunctionNth>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssPseudoClassFunctionNth {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssPseudoClassFunctionNth,
        crate::css::pseudo::pseudo_class_function_nth::FormatCssPseudoClassFunctionNth,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::pseudo::pseudo_class_function_nth::FormatCssPseudoClassFunctionNth::default(
            ),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssPseudoClassFunctionNth {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssPseudoClassFunctionNth,
        crate::css::pseudo::pseudo_class_function_nth::FormatCssPseudoClassFunctionNth,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::pseudo::pseudo_class_function_nth::FormatCssPseudoClassFunctionNth::default(
            ),
        )
    }
}
impl FormatRule<biome_css_syntax::CssRelativeSelector>
    for crate::css::selectors::relative_selector::FormatCssRelativeSelector
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssRelativeSelector,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssRelativeSelector>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssRelativeSelector {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssRelativeSelector,
        crate::css::selectors::relative_selector::FormatCssRelativeSelector,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::selectors::relative_selector::FormatCssRelativeSelector::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssRelativeSelector {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssRelativeSelector,
        crate::css::selectors::relative_selector::FormatCssRelativeSelector,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::selectors::relative_selector::FormatCssRelativeSelector::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssPseudoClassNthSelector>
    for crate::css::selectors::pseudo_class_nth_selector::FormatCssPseudoClassNthSelector
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssPseudoClassNthSelector,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssPseudoClassNthSelector>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssPseudoClassNthSelector {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssPseudoClassNthSelector,
        crate::css::selectors::pseudo_class_nth_selector::FormatCssPseudoClassNthSelector,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule :: new (self , crate :: css :: selectors :: pseudo_class_nth_selector :: FormatCssPseudoClassNthSelector :: default ())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssPseudoClassNthSelector {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssPseudoClassNthSelector,
        crate::css::selectors::pseudo_class_nth_selector::FormatCssPseudoClassNthSelector,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule :: new (self , crate :: css :: selectors :: pseudo_class_nth_selector :: FormatCssPseudoClassNthSelector :: default ())
    }
}
impl FormatRule<biome_css_syntax::CssPseudoClassOfNthSelector>
    for crate::css::selectors::pseudo_class_of_nth_selector::FormatCssPseudoClassOfNthSelector
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssPseudoClassOfNthSelector,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssPseudoClassOfNthSelector>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssPseudoClassOfNthSelector {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssPseudoClassOfNthSelector,
        crate::css::selectors::pseudo_class_of_nth_selector::FormatCssPseudoClassOfNthSelector,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule :: new (self , crate :: css :: selectors :: pseudo_class_of_nth_selector :: FormatCssPseudoClassOfNthSelector :: default ())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssPseudoClassOfNthSelector {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssPseudoClassOfNthSelector,
        crate::css::selectors::pseudo_class_of_nth_selector::FormatCssPseudoClassOfNthSelector,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule :: new (self , crate :: css :: selectors :: pseudo_class_of_nth_selector :: FormatCssPseudoClassOfNthSelector :: default ())
    }
}
impl FormatRule<biome_css_syntax::CssPseudoClassNthNumber>
    for crate::css::pseudo::pseudo_class_nth_number::FormatCssPseudoClassNthNumber
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssPseudoClassNthNumber,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssPseudoClassNthNumber>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssPseudoClassNthNumber {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssPseudoClassNthNumber,
        crate::css::pseudo::pseudo_class_nth_number::FormatCssPseudoClassNthNumber,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::pseudo::pseudo_class_nth_number::FormatCssPseudoClassNthNumber::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssPseudoClassNthNumber {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssPseudoClassNthNumber,
        crate::css::pseudo::pseudo_class_nth_number::FormatCssPseudoClassNthNumber,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::pseudo::pseudo_class_nth_number::FormatCssPseudoClassNthNumber::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssPseudoClassNthIdentifier>
    for crate::css::pseudo::pseudo_class_nth_identifier::FormatCssPseudoClassNthIdentifier
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssPseudoClassNthIdentifier,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssPseudoClassNthIdentifier>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssPseudoClassNthIdentifier {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssPseudoClassNthIdentifier,
        crate::css::pseudo::pseudo_class_nth_identifier::FormatCssPseudoClassNthIdentifier,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule :: new (self , crate :: css :: pseudo :: pseudo_class_nth_identifier :: FormatCssPseudoClassNthIdentifier :: default ())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssPseudoClassNthIdentifier {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssPseudoClassNthIdentifier,
        crate::css::pseudo::pseudo_class_nth_identifier::FormatCssPseudoClassNthIdentifier,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule :: new (self , crate :: css :: pseudo :: pseudo_class_nth_identifier :: FormatCssPseudoClassNthIdentifier :: default ())
    }
}
impl FormatRule<biome_css_syntax::CssPseudoClassNth>
    for crate::css::pseudo::pseudo_class_nth::FormatCssPseudoClassNth
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssPseudoClassNth,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssPseudoClassNth>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssPseudoClassNth {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssPseudoClassNth,
        crate::css::pseudo::pseudo_class_nth::FormatCssPseudoClassNth,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::pseudo::pseudo_class_nth::FormatCssPseudoClassNth::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssPseudoClassNth {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssPseudoClassNth,
        crate::css::pseudo::pseudo_class_nth::FormatCssPseudoClassNth,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::pseudo::pseudo_class_nth::FormatCssPseudoClassNth::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssNumber> for crate::css::value::number::FormatCssNumber {
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_css_syntax::CssNumber, f: &mut CssFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssNumber>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssNumber {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssNumber,
        crate::css::value::number::FormatCssNumber,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(self, crate::css::value::number::FormatCssNumber::default())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssNumber {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssNumber,
        crate::css::value::number::FormatCssNumber,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(self, crate::css::value::number::FormatCssNumber::default())
    }
}
impl FormatRule<biome_css_syntax::CssNthOffset>
    for crate::css::auxiliary::nth_offset::FormatCssNthOffset
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_css_syntax::CssNthOffset, f: &mut CssFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssNthOffset>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssNthOffset {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssNthOffset,
        crate::css::auxiliary::nth_offset::FormatCssNthOffset,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::nth_offset::FormatCssNthOffset::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssNthOffset {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssNthOffset,
        crate::css::auxiliary::nth_offset::FormatCssNthOffset,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::nth_offset::FormatCssNthOffset::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssPseudoElementIdentifier>
    for crate::css::pseudo::pseudo_element_identifier::FormatCssPseudoElementIdentifier
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssPseudoElementIdentifier,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssPseudoElementIdentifier>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssPseudoElementIdentifier {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssPseudoElementIdentifier,
        crate::css::pseudo::pseudo_element_identifier::FormatCssPseudoElementIdentifier,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule :: new (self , crate :: css :: pseudo :: pseudo_element_identifier :: FormatCssPseudoElementIdentifier :: default ())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssPseudoElementIdentifier {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssPseudoElementIdentifier,
        crate::css::pseudo::pseudo_element_identifier::FormatCssPseudoElementIdentifier,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule :: new (self , crate :: css :: pseudo :: pseudo_element_identifier :: FormatCssPseudoElementIdentifier :: default ())
    }
}
impl FormatRule < biome_css_syntax :: CssPseudoElementFunctionSelector > for crate :: css :: selectors :: pseudo_element_function_selector :: FormatCssPseudoElementFunctionSelector { type Context = CssFormatContext ; # [inline (always)] fn fmt (& self , node : & biome_css_syntax :: CssPseudoElementFunctionSelector , f : & mut CssFormatter) -> FormatResult < () > { FormatNodeRule :: < biome_css_syntax :: CssPseudoElementFunctionSelector > :: fmt (self , node , f) } }
impl AsFormat<CssFormatContext> for biome_css_syntax::CssPseudoElementFunctionSelector {
    type Format < 'a > = FormatRefWithRule < 'a , biome_css_syntax :: CssPseudoElementFunctionSelector , crate :: css :: selectors :: pseudo_element_function_selector :: FormatCssPseudoElementFunctionSelector > ;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule :: new (self , crate :: css :: selectors :: pseudo_element_function_selector :: FormatCssPseudoElementFunctionSelector :: default ())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssPseudoElementFunctionSelector {
    type Format = FormatOwnedWithRule < biome_css_syntax :: CssPseudoElementFunctionSelector , crate :: css :: selectors :: pseudo_element_function_selector :: FormatCssPseudoElementFunctionSelector > ;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule :: new (self , crate :: css :: selectors :: pseudo_element_function_selector :: FormatCssPseudoElementFunctionSelector :: default ())
    }
}
impl FormatRule < biome_css_syntax :: CssPseudoElementFunctionIdentifier > for crate :: css :: pseudo :: pseudo_element_function_identifier :: FormatCssPseudoElementFunctionIdentifier { type Context = CssFormatContext ; # [inline (always)] fn fmt (& self , node : & biome_css_syntax :: CssPseudoElementFunctionIdentifier , f : & mut CssFormatter) -> FormatResult < () > { FormatNodeRule :: < biome_css_syntax :: CssPseudoElementFunctionIdentifier > :: fmt (self , node , f) } }
impl AsFormat<CssFormatContext> for biome_css_syntax::CssPseudoElementFunctionIdentifier {
    type Format < 'a > = FormatRefWithRule < 'a , biome_css_syntax :: CssPseudoElementFunctionIdentifier , crate :: css :: pseudo :: pseudo_element_function_identifier :: FormatCssPseudoElementFunctionIdentifier > ;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule :: new (self , crate :: css :: pseudo :: pseudo_element_function_identifier :: FormatCssPseudoElementFunctionIdentifier :: default ())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssPseudoElementFunctionIdentifier {
    type Format = FormatOwnedWithRule < biome_css_syntax :: CssPseudoElementFunctionIdentifier , crate :: css :: pseudo :: pseudo_element_function_identifier :: FormatCssPseudoElementFunctionIdentifier > ;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule :: new (self , crate :: css :: pseudo :: pseudo_element_function_identifier :: FormatCssPseudoElementFunctionIdentifier :: default ())
    }
}
impl FormatRule<biome_css_syntax::CssDeclarationOrRuleBlock>
    for crate::css::auxiliary::declaration_or_rule_block::FormatCssDeclarationOrRuleBlock
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssDeclarationOrRuleBlock,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssDeclarationOrRuleBlock>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssDeclarationOrRuleBlock {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssDeclarationOrRuleBlock,
        crate::css::auxiliary::declaration_or_rule_block::FormatCssDeclarationOrRuleBlock,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule :: new (self , crate :: css :: auxiliary :: declaration_or_rule_block :: FormatCssDeclarationOrRuleBlock :: default ())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssDeclarationOrRuleBlock {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssDeclarationOrRuleBlock,
        crate::css::auxiliary::declaration_or_rule_block::FormatCssDeclarationOrRuleBlock,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule :: new (self , crate :: css :: auxiliary :: declaration_or_rule_block :: FormatCssDeclarationOrRuleBlock :: default ())
    }
}
impl FormatRule<biome_css_syntax::CssDeclarationWithSemicolon>
    for crate::css::auxiliary::declaration_with_semicolon::FormatCssDeclarationWithSemicolon
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssDeclarationWithSemicolon,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssDeclarationWithSemicolon>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssDeclarationWithSemicolon {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssDeclarationWithSemicolon,
        crate::css::auxiliary::declaration_with_semicolon::FormatCssDeclarationWithSemicolon,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule :: new (self , crate :: css :: auxiliary :: declaration_with_semicolon :: FormatCssDeclarationWithSemicolon :: default ())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssDeclarationWithSemicolon {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssDeclarationWithSemicolon,
        crate::css::auxiliary::declaration_with_semicolon::FormatCssDeclarationWithSemicolon,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule :: new (self , crate :: css :: auxiliary :: declaration_with_semicolon :: FormatCssDeclarationWithSemicolon :: default ())
    }
}
impl FormatRule<biome_css_syntax::CssDeclarationOrAtRuleBlock>
    for crate::css::auxiliary::declaration_or_at_rule_block::FormatCssDeclarationOrAtRuleBlock
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssDeclarationOrAtRuleBlock,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssDeclarationOrAtRuleBlock>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssDeclarationOrAtRuleBlock {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssDeclarationOrAtRuleBlock,
        crate::css::auxiliary::declaration_or_at_rule_block::FormatCssDeclarationOrAtRuleBlock,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule :: new (self , crate :: css :: auxiliary :: declaration_or_at_rule_block :: FormatCssDeclarationOrAtRuleBlock :: default ())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssDeclarationOrAtRuleBlock {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssDeclarationOrAtRuleBlock,
        crate::css::auxiliary::declaration_or_at_rule_block::FormatCssDeclarationOrAtRuleBlock,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule :: new (self , crate :: css :: auxiliary :: declaration_or_at_rule_block :: FormatCssDeclarationOrAtRuleBlock :: default ())
    }
}
impl FormatRule<biome_css_syntax::CssDeclaration>
    for crate::css::auxiliary::declaration::FormatCssDeclaration
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssDeclaration,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssDeclaration>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssDeclaration {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssDeclaration,
        crate::css::auxiliary::declaration::FormatCssDeclaration,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::declaration::FormatCssDeclaration::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssDeclaration {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssDeclaration,
        crate::css::auxiliary::declaration::FormatCssDeclaration,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::declaration::FormatCssDeclaration::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssDeclarationListBlock>
    for crate::css::auxiliary::declaration_list_block::FormatCssDeclarationListBlock
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssDeclarationListBlock,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssDeclarationListBlock>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssDeclarationListBlock {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssDeclarationListBlock,
        crate::css::auxiliary::declaration_list_block::FormatCssDeclarationListBlock,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::declaration_list_block::FormatCssDeclarationListBlock::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssDeclarationListBlock {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssDeclarationListBlock,
        crate::css::auxiliary::declaration_list_block::FormatCssDeclarationListBlock,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::declaration_list_block::FormatCssDeclarationListBlock::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssRuleListBlock>
    for crate::css::auxiliary::rule_list_block::FormatCssRuleListBlock
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssRuleListBlock,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssRuleListBlock>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssRuleListBlock {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssRuleListBlock,
        crate::css::auxiliary::rule_list_block::FormatCssRuleListBlock,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::rule_list_block::FormatCssRuleListBlock::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssRuleListBlock {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssRuleListBlock,
        crate::css::auxiliary::rule_list_block::FormatCssRuleListBlock,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::rule_list_block::FormatCssRuleListBlock::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssDeclarationImportant>
    for crate::css::auxiliary::declaration_important::FormatCssDeclarationImportant
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssDeclarationImportant,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssDeclarationImportant>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssDeclarationImportant {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssDeclarationImportant,
        crate::css::auxiliary::declaration_important::FormatCssDeclarationImportant,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::declaration_important::FormatCssDeclarationImportant::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssDeclarationImportant {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssDeclarationImportant,
        crate::css::auxiliary::declaration_important::FormatCssDeclarationImportant,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::declaration_important::FormatCssDeclarationImportant::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssGenericProperty>
    for crate::css::properties::generic_property::FormatCssGenericProperty
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssGenericProperty,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssGenericProperty>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssGenericProperty {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssGenericProperty,
        crate::css::properties::generic_property::FormatCssGenericProperty,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::properties::generic_property::FormatCssGenericProperty::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssGenericProperty {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssGenericProperty,
        crate::css::properties::generic_property::FormatCssGenericProperty,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::properties::generic_property::FormatCssGenericProperty::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssAllProperty>
    for crate::css::properties::all_property::FormatCssAllProperty
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssAllProperty,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssAllProperty>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssAllProperty {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssAllProperty,
        crate::css::properties::all_property::FormatCssAllProperty,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::properties::all_property::FormatCssAllProperty::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssAllProperty {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssAllProperty,
        crate::css::properties::all_property::FormatCssAllProperty,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::properties::all_property::FormatCssAllProperty::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssBorderProperty>
    for crate::css::properties::border_property::FormatCssBorderProperty
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssBorderProperty,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssBorderProperty>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssBorderProperty {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssBorderProperty,
        crate::css::properties::border_property::FormatCssBorderProperty,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::properties::border_property::FormatCssBorderProperty::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssBorderProperty {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssBorderProperty,
        crate::css::properties::border_property::FormatCssBorderProperty,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::properties::border_property::FormatCssBorderProperty::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssZIndexProperty>
    for crate::css::properties::z_index_property::FormatCssZIndexProperty
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssZIndexProperty,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssZIndexProperty>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssZIndexProperty {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssZIndexProperty,
        crate::css::properties::z_index_property::FormatCssZIndexProperty,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::properties::z_index_property::FormatCssZIndexProperty::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssZIndexProperty {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssZIndexProperty,
        crate::css::properties::z_index_property::FormatCssZIndexProperty,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::properties::z_index_property::FormatCssZIndexProperty::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssWideKeyword>
    for crate::css::auxiliary::wide_keyword::FormatCssWideKeyword
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssWideKeyword,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssWideKeyword>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssWideKeyword {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssWideKeyword,
        crate::css::auxiliary::wide_keyword::FormatCssWideKeyword,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::wide_keyword::FormatCssWideKeyword::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssWideKeyword {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssWideKeyword,
        crate::css::auxiliary::wide_keyword::FormatCssWideKeyword,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::wide_keyword::FormatCssWideKeyword::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssBorder> for crate::css::auxiliary::border::FormatCssBorder {
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_css_syntax::CssBorder, f: &mut CssFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssBorder>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssBorder {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssBorder,
        crate::css::auxiliary::border::FormatCssBorder,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::border::FormatCssBorder::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssBorder {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssBorder,
        crate::css::auxiliary::border::FormatCssBorder,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::border::FormatCssBorder::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssLineStyle>
    for crate::css::auxiliary::line_style::FormatCssLineStyle
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_css_syntax::CssLineStyle, f: &mut CssFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssLineStyle>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssLineStyle {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssLineStyle,
        crate::css::auxiliary::line_style::FormatCssLineStyle,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::line_style::FormatCssLineStyle::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssLineStyle {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssLineStyle,
        crate::css::auxiliary::line_style::FormatCssLineStyle,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::line_style::FormatCssLineStyle::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssColor> for crate::css::value::color::FormatCssColor {
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_css_syntax::CssColor, f: &mut CssFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssColor>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssColor {
    type Format<'a> =
        FormatRefWithRule<'a, biome_css_syntax::CssColor, crate::css::value::color::FormatCssColor>;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(self, crate::css::value::color::FormatCssColor::default())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssColor {
    type Format =
        FormatOwnedWithRule<biome_css_syntax::CssColor, crate::css::value::color::FormatCssColor>;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(self, crate::css::value::color::FormatCssColor::default())
    }
}
impl FormatRule<biome_css_syntax::CssRegularDimension>
    for crate::css::value::regular_dimension::FormatCssRegularDimension
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssRegularDimension,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssRegularDimension>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssRegularDimension {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssRegularDimension,
        crate::css::value::regular_dimension::FormatCssRegularDimension,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::value::regular_dimension::FormatCssRegularDimension::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssRegularDimension {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssRegularDimension,
        crate::css::value::regular_dimension::FormatCssRegularDimension,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::value::regular_dimension::FormatCssRegularDimension::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssLineWidthKeyword>
    for crate::css::auxiliary::line_width_keyword::FormatCssLineWidthKeyword
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssLineWidthKeyword,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssLineWidthKeyword>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssLineWidthKeyword {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssLineWidthKeyword,
        crate::css::auxiliary::line_width_keyword::FormatCssLineWidthKeyword,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::line_width_keyword::FormatCssLineWidthKeyword::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssLineWidthKeyword {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssLineWidthKeyword,
        crate::css::auxiliary::line_width_keyword::FormatCssLineWidthKeyword,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::line_width_keyword::FormatCssLineWidthKeyword::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssAuto> for crate::css::auxiliary::auto::FormatCssAuto {
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_css_syntax::CssAuto, f: &mut CssFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssAuto>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssAuto {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssAuto,
        crate::css::auxiliary::auto::FormatCssAuto,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(self, crate::css::auxiliary::auto::FormatCssAuto::default())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssAuto {
    type Format =
        FormatOwnedWithRule<biome_css_syntax::CssAuto, crate::css::auxiliary::auto::FormatCssAuto>;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(self, crate::css::auxiliary::auto::FormatCssAuto::default())
    }
}
impl FormatRule<biome_css_syntax::CssUnknownPropertyValue>
    for crate::css::auxiliary::unknown_property_value::FormatCssUnknownPropertyValue
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssUnknownPropertyValue,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssUnknownPropertyValue>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssUnknownPropertyValue {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssUnknownPropertyValue,
        crate::css::auxiliary::unknown_property_value::FormatCssUnknownPropertyValue,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::unknown_property_value::FormatCssUnknownPropertyValue::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssUnknownPropertyValue {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssUnknownPropertyValue,
        crate::css::auxiliary::unknown_property_value::FormatCssUnknownPropertyValue,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::unknown_property_value::FormatCssUnknownPropertyValue::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssGenericDelimiter>
    for crate::css::auxiliary::generic_delimiter::FormatCssGenericDelimiter
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssGenericDelimiter,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssGenericDelimiter>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssGenericDelimiter {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssGenericDelimiter,
        crate::css::auxiliary::generic_delimiter::FormatCssGenericDelimiter,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::generic_delimiter::FormatCssGenericDelimiter::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssGenericDelimiter {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssGenericDelimiter,
        crate::css::auxiliary::generic_delimiter::FormatCssGenericDelimiter,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::generic_delimiter::FormatCssGenericDelimiter::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssDashedIdentifier>
    for crate::css::value::dashed_identifier::FormatCssDashedIdentifier
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssDashedIdentifier,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssDashedIdentifier>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssDashedIdentifier {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssDashedIdentifier,
        crate::css::value::dashed_identifier::FormatCssDashedIdentifier,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::value::dashed_identifier::FormatCssDashedIdentifier::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssDashedIdentifier {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssDashedIdentifier,
        crate::css::value::dashed_identifier::FormatCssDashedIdentifier,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::value::dashed_identifier::FormatCssDashedIdentifier::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssCharsetAtRule>
    for crate::css::statements::charset_at_rule::FormatCssCharsetAtRule
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssCharsetAtRule,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssCharsetAtRule>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssCharsetAtRule {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssCharsetAtRule,
        crate::css::statements::charset_at_rule::FormatCssCharsetAtRule,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::statements::charset_at_rule::FormatCssCharsetAtRule::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssCharsetAtRule {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssCharsetAtRule,
        crate::css::statements::charset_at_rule::FormatCssCharsetAtRule,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::statements::charset_at_rule::FormatCssCharsetAtRule::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssColorProfileAtRule>
    for crate::css::statements::color_profile_at_rule::FormatCssColorProfileAtRule
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssColorProfileAtRule,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssColorProfileAtRule>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssColorProfileAtRule {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssColorProfileAtRule,
        crate::css::statements::color_profile_at_rule::FormatCssColorProfileAtRule,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::statements::color_profile_at_rule::FormatCssColorProfileAtRule::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssColorProfileAtRule {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssColorProfileAtRule,
        crate::css::statements::color_profile_at_rule::FormatCssColorProfileAtRule,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::statements::color_profile_at_rule::FormatCssColorProfileAtRule::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssCounterStyleAtRule>
    for crate::css::statements::counter_style_at_rule::FormatCssCounterStyleAtRule
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssCounterStyleAtRule,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssCounterStyleAtRule>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssCounterStyleAtRule {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssCounterStyleAtRule,
        crate::css::statements::counter_style_at_rule::FormatCssCounterStyleAtRule,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::statements::counter_style_at_rule::FormatCssCounterStyleAtRule::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssCounterStyleAtRule {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssCounterStyleAtRule,
        crate::css::statements::counter_style_at_rule::FormatCssCounterStyleAtRule,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::statements::counter_style_at_rule::FormatCssCounterStyleAtRule::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssContainerAtRule>
    for crate::css::statements::container_at_rule::FormatCssContainerAtRule
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssContainerAtRule,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssContainerAtRule>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssContainerAtRule {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssContainerAtRule,
        crate::css::statements::container_at_rule::FormatCssContainerAtRule,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::statements::container_at_rule::FormatCssContainerAtRule::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssContainerAtRule {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssContainerAtRule,
        crate::css::statements::container_at_rule::FormatCssContainerAtRule,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::statements::container_at_rule::FormatCssContainerAtRule::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssFontFaceAtRule>
    for crate::css::statements::font_face_at_rule::FormatCssFontFaceAtRule
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssFontFaceAtRule,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssFontFaceAtRule>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssFontFaceAtRule {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssFontFaceAtRule,
        crate::css::statements::font_face_at_rule::FormatCssFontFaceAtRule,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::statements::font_face_at_rule::FormatCssFontFaceAtRule::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssFontFaceAtRule {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssFontFaceAtRule,
        crate::css::statements::font_face_at_rule::FormatCssFontFaceAtRule,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::statements::font_face_at_rule::FormatCssFontFaceAtRule::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssFontFeatureValuesAtRule>
    for crate::css::statements::font_feature_values_at_rule::FormatCssFontFeatureValuesAtRule
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssFontFeatureValuesAtRule,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssFontFeatureValuesAtRule>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssFontFeatureValuesAtRule {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssFontFeatureValuesAtRule,
        crate::css::statements::font_feature_values_at_rule::FormatCssFontFeatureValuesAtRule,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule :: new (self , crate :: css :: statements :: font_feature_values_at_rule :: FormatCssFontFeatureValuesAtRule :: default ())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssFontFeatureValuesAtRule {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssFontFeatureValuesAtRule,
        crate::css::statements::font_feature_values_at_rule::FormatCssFontFeatureValuesAtRule,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule :: new (self , crate :: css :: statements :: font_feature_values_at_rule :: FormatCssFontFeatureValuesAtRule :: default ())
    }
}
impl FormatRule<biome_css_syntax::CssFontPaletteValuesAtRule>
    for crate::css::statements::font_palette_values_at_rule::FormatCssFontPaletteValuesAtRule
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssFontPaletteValuesAtRule,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssFontPaletteValuesAtRule>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssFontPaletteValuesAtRule {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssFontPaletteValuesAtRule,
        crate::css::statements::font_palette_values_at_rule::FormatCssFontPaletteValuesAtRule,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule :: new (self , crate :: css :: statements :: font_palette_values_at_rule :: FormatCssFontPaletteValuesAtRule :: default ())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssFontPaletteValuesAtRule {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssFontPaletteValuesAtRule,
        crate::css::statements::font_palette_values_at_rule::FormatCssFontPaletteValuesAtRule,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule :: new (self , crate :: css :: statements :: font_palette_values_at_rule :: FormatCssFontPaletteValuesAtRule :: default ())
    }
}
impl FormatRule<biome_css_syntax::CssKeyframesAtRule>
    for crate::css::statements::keyframes_at_rule::FormatCssKeyframesAtRule
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssKeyframesAtRule,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssKeyframesAtRule>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssKeyframesAtRule {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssKeyframesAtRule,
        crate::css::statements::keyframes_at_rule::FormatCssKeyframesAtRule,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::statements::keyframes_at_rule::FormatCssKeyframesAtRule::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssKeyframesAtRule {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssKeyframesAtRule,
        crate::css::statements::keyframes_at_rule::FormatCssKeyframesAtRule,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::statements::keyframes_at_rule::FormatCssKeyframesAtRule::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssMediaAtRule>
    for crate::css::statements::media_at_rule::FormatCssMediaAtRule
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssMediaAtRule,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssMediaAtRule>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssMediaAtRule {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssMediaAtRule,
        crate::css::statements::media_at_rule::FormatCssMediaAtRule,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::statements::media_at_rule::FormatCssMediaAtRule::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssMediaAtRule {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssMediaAtRule,
        crate::css::statements::media_at_rule::FormatCssMediaAtRule,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::statements::media_at_rule::FormatCssMediaAtRule::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssPageAtRule>
    for crate::css::statements::page_at_rule::FormatCssPageAtRule
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssPageAtRule,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssPageAtRule>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssPageAtRule {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssPageAtRule,
        crate::css::statements::page_at_rule::FormatCssPageAtRule,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::statements::page_at_rule::FormatCssPageAtRule::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssPageAtRule {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssPageAtRule,
        crate::css::statements::page_at_rule::FormatCssPageAtRule,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::statements::page_at_rule::FormatCssPageAtRule::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssLayerAtRule>
    for crate::css::statements::layer_at_rule::FormatCssLayerAtRule
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssLayerAtRule,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssLayerAtRule>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssLayerAtRule {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssLayerAtRule,
        crate::css::statements::layer_at_rule::FormatCssLayerAtRule,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::statements::layer_at_rule::FormatCssLayerAtRule::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssLayerAtRule {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssLayerAtRule,
        crate::css::statements::layer_at_rule::FormatCssLayerAtRule,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::statements::layer_at_rule::FormatCssLayerAtRule::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssSupportsAtRule>
    for crate::css::statements::supports_at_rule::FormatCssSupportsAtRule
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssSupportsAtRule,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssSupportsAtRule>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssSupportsAtRule {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssSupportsAtRule,
        crate::css::statements::supports_at_rule::FormatCssSupportsAtRule,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::statements::supports_at_rule::FormatCssSupportsAtRule::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssSupportsAtRule {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssSupportsAtRule,
        crate::css::statements::supports_at_rule::FormatCssSupportsAtRule,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::statements::supports_at_rule::FormatCssSupportsAtRule::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssScopeAtRule>
    for crate::css::statements::scope_at_rule::FormatCssScopeAtRule
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssScopeAtRule,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssScopeAtRule>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssScopeAtRule {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssScopeAtRule,
        crate::css::statements::scope_at_rule::FormatCssScopeAtRule,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::statements::scope_at_rule::FormatCssScopeAtRule::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssScopeAtRule {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssScopeAtRule,
        crate::css::statements::scope_at_rule::FormatCssScopeAtRule,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::statements::scope_at_rule::FormatCssScopeAtRule::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssImportAtRule>
    for crate::css::statements::import_at_rule::FormatCssImportAtRule
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssImportAtRule,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssImportAtRule>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssImportAtRule {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssImportAtRule,
        crate::css::statements::import_at_rule::FormatCssImportAtRule,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::statements::import_at_rule::FormatCssImportAtRule::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssImportAtRule {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssImportAtRule,
        crate::css::statements::import_at_rule::FormatCssImportAtRule,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::statements::import_at_rule::FormatCssImportAtRule::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssNamespaceAtRule>
    for crate::css::statements::namespace_at_rule::FormatCssNamespaceAtRule
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssNamespaceAtRule,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssNamespaceAtRule>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssNamespaceAtRule {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssNamespaceAtRule,
        crate::css::statements::namespace_at_rule::FormatCssNamespaceAtRule,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::statements::namespace_at_rule::FormatCssNamespaceAtRule::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssNamespaceAtRule {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssNamespaceAtRule,
        crate::css::statements::namespace_at_rule::FormatCssNamespaceAtRule,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::statements::namespace_at_rule::FormatCssNamespaceAtRule::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssStartingStyleAtRule>
    for crate::css::statements::starting_style_at_rule::FormatCssStartingStyleAtRule
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssStartingStyleAtRule,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssStartingStyleAtRule>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssStartingStyleAtRule {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssStartingStyleAtRule,
        crate::css::statements::starting_style_at_rule::FormatCssStartingStyleAtRule,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::statements::starting_style_at_rule::FormatCssStartingStyleAtRule::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssStartingStyleAtRule {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssStartingStyleAtRule,
        crate::css::statements::starting_style_at_rule::FormatCssStartingStyleAtRule,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::statements::starting_style_at_rule::FormatCssStartingStyleAtRule::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssDocumentAtRule>
    for crate::css::statements::document_at_rule::FormatCssDocumentAtRule
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssDocumentAtRule,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssDocumentAtRule>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssDocumentAtRule {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssDocumentAtRule,
        crate::css::statements::document_at_rule::FormatCssDocumentAtRule,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::statements::document_at_rule::FormatCssDocumentAtRule::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssDocumentAtRule {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssDocumentAtRule,
        crate::css::statements::document_at_rule::FormatCssDocumentAtRule,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::statements::document_at_rule::FormatCssDocumentAtRule::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssFontFeatureValuesBlock>
    for crate::css::auxiliary::font_feature_values_block::FormatCssFontFeatureValuesBlock
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssFontFeatureValuesBlock,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssFontFeatureValuesBlock>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssFontFeatureValuesBlock {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssFontFeatureValuesBlock,
        crate::css::auxiliary::font_feature_values_block::FormatCssFontFeatureValuesBlock,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule :: new (self , crate :: css :: auxiliary :: font_feature_values_block :: FormatCssFontFeatureValuesBlock :: default ())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssFontFeatureValuesBlock {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssFontFeatureValuesBlock,
        crate::css::auxiliary::font_feature_values_block::FormatCssFontFeatureValuesBlock,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule :: new (self , crate :: css :: auxiliary :: font_feature_values_block :: FormatCssFontFeatureValuesBlock :: default ())
    }
}
impl FormatRule<biome_css_syntax::CssFontFeatureValuesItem>
    for crate::css::auxiliary::font_feature_values_item::FormatCssFontFeatureValuesItem
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssFontFeatureValuesItem,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssFontFeatureValuesItem>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssFontFeatureValuesItem {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssFontFeatureValuesItem,
        crate::css::auxiliary::font_feature_values_item::FormatCssFontFeatureValuesItem,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule :: new (self , crate :: css :: auxiliary :: font_feature_values_item :: FormatCssFontFeatureValuesItem :: default ())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssFontFeatureValuesItem {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssFontFeatureValuesItem,
        crate::css::auxiliary::font_feature_values_item::FormatCssFontFeatureValuesItem,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule :: new (self , crate :: css :: auxiliary :: font_feature_values_item :: FormatCssFontFeatureValuesItem :: default ())
    }
}
impl FormatRule<biome_css_syntax::CssContainerNotQuery>
    for crate::css::auxiliary::container_not_query::FormatCssContainerNotQuery
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssContainerNotQuery,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssContainerNotQuery>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssContainerNotQuery {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssContainerNotQuery,
        crate::css::auxiliary::container_not_query::FormatCssContainerNotQuery,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::container_not_query::FormatCssContainerNotQuery::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssContainerNotQuery {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssContainerNotQuery,
        crate::css::auxiliary::container_not_query::FormatCssContainerNotQuery,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::container_not_query::FormatCssContainerNotQuery::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssContainerOrQuery>
    for crate::css::auxiliary::container_or_query::FormatCssContainerOrQuery
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssContainerOrQuery,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssContainerOrQuery>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssContainerOrQuery {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssContainerOrQuery,
        crate::css::auxiliary::container_or_query::FormatCssContainerOrQuery,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::container_or_query::FormatCssContainerOrQuery::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssContainerOrQuery {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssContainerOrQuery,
        crate::css::auxiliary::container_or_query::FormatCssContainerOrQuery,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::container_or_query::FormatCssContainerOrQuery::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssContainerAndQuery>
    for crate::css::auxiliary::container_and_query::FormatCssContainerAndQuery
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssContainerAndQuery,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssContainerAndQuery>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssContainerAndQuery {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssContainerAndQuery,
        crate::css::auxiliary::container_and_query::FormatCssContainerAndQuery,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::container_and_query::FormatCssContainerAndQuery::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssContainerAndQuery {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssContainerAndQuery,
        crate::css::auxiliary::container_and_query::FormatCssContainerAndQuery,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::container_and_query::FormatCssContainerAndQuery::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssContainerQueryInParens>
    for crate::css::auxiliary::container_query_in_parens::FormatCssContainerQueryInParens
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssContainerQueryInParens,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssContainerQueryInParens>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssContainerQueryInParens {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssContainerQueryInParens,
        crate::css::auxiliary::container_query_in_parens::FormatCssContainerQueryInParens,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule :: new (self , crate :: css :: auxiliary :: container_query_in_parens :: FormatCssContainerQueryInParens :: default ())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssContainerQueryInParens {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssContainerQueryInParens,
        crate::css::auxiliary::container_query_in_parens::FormatCssContainerQueryInParens,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule :: new (self , crate :: css :: auxiliary :: container_query_in_parens :: FormatCssContainerQueryInParens :: default ())
    }
}
impl FormatRule < biome_css_syntax :: CssContainerSizeFeatureInParens > for crate :: css :: auxiliary :: container_size_feature_in_parens :: FormatCssContainerSizeFeatureInParens { type Context = CssFormatContext ; # [inline (always)] fn fmt (& self , node : & biome_css_syntax :: CssContainerSizeFeatureInParens , f : & mut CssFormatter) -> FormatResult < () > { FormatNodeRule :: < biome_css_syntax :: CssContainerSizeFeatureInParens > :: fmt (self , node , f) } }
impl AsFormat<CssFormatContext> for biome_css_syntax::CssContainerSizeFeatureInParens {
    type Format < 'a > = FormatRefWithRule < 'a , biome_css_syntax :: CssContainerSizeFeatureInParens , crate :: css :: auxiliary :: container_size_feature_in_parens :: FormatCssContainerSizeFeatureInParens > ;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule :: new (self , crate :: css :: auxiliary :: container_size_feature_in_parens :: FormatCssContainerSizeFeatureInParens :: default ())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssContainerSizeFeatureInParens {
    type Format = FormatOwnedWithRule < biome_css_syntax :: CssContainerSizeFeatureInParens , crate :: css :: auxiliary :: container_size_feature_in_parens :: FormatCssContainerSizeFeatureInParens > ;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule :: new (self , crate :: css :: auxiliary :: container_size_feature_in_parens :: FormatCssContainerSizeFeatureInParens :: default ())
    }
}
impl FormatRule<biome_css_syntax::CssContainerStyleQueryInParens>
    for crate::css::auxiliary::container_style_query_in_parens::FormatCssContainerStyleQueryInParens
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssContainerStyleQueryInParens,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssContainerStyleQueryInParens>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssContainerStyleQueryInParens {
    type Format < 'a > = FormatRefWithRule < 'a , biome_css_syntax :: CssContainerStyleQueryInParens , crate :: css :: auxiliary :: container_style_query_in_parens :: FormatCssContainerStyleQueryInParens > ;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule :: new (self , crate :: css :: auxiliary :: container_style_query_in_parens :: FormatCssContainerStyleQueryInParens :: default ())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssContainerStyleQueryInParens {
    type Format = FormatOwnedWithRule < biome_css_syntax :: CssContainerStyleQueryInParens , crate :: css :: auxiliary :: container_style_query_in_parens :: FormatCssContainerStyleQueryInParens > ;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule :: new (self , crate :: css :: auxiliary :: container_style_query_in_parens :: FormatCssContainerStyleQueryInParens :: default ())
    }
}
impl FormatRule<biome_css_syntax::CssContainerStyleNotQuery>
    for crate::css::auxiliary::container_style_not_query::FormatCssContainerStyleNotQuery
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssContainerStyleNotQuery,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssContainerStyleNotQuery>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssContainerStyleNotQuery {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssContainerStyleNotQuery,
        crate::css::auxiliary::container_style_not_query::FormatCssContainerStyleNotQuery,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule :: new (self , crate :: css :: auxiliary :: container_style_not_query :: FormatCssContainerStyleNotQuery :: default ())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssContainerStyleNotQuery {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssContainerStyleNotQuery,
        crate::css::auxiliary::container_style_not_query::FormatCssContainerStyleNotQuery,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule :: new (self , crate :: css :: auxiliary :: container_style_not_query :: FormatCssContainerStyleNotQuery :: default ())
    }
}
impl FormatRule<biome_css_syntax::CssContainerStyleAndQuery>
    for crate::css::auxiliary::container_style_and_query::FormatCssContainerStyleAndQuery
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssContainerStyleAndQuery,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssContainerStyleAndQuery>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssContainerStyleAndQuery {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssContainerStyleAndQuery,
        crate::css::auxiliary::container_style_and_query::FormatCssContainerStyleAndQuery,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule :: new (self , crate :: css :: auxiliary :: container_style_and_query :: FormatCssContainerStyleAndQuery :: default ())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssContainerStyleAndQuery {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssContainerStyleAndQuery,
        crate::css::auxiliary::container_style_and_query::FormatCssContainerStyleAndQuery,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule :: new (self , crate :: css :: auxiliary :: container_style_and_query :: FormatCssContainerStyleAndQuery :: default ())
    }
}
impl FormatRule<biome_css_syntax::CssContainerStyleOrQuery>
    for crate::css::auxiliary::container_style_or_query::FormatCssContainerStyleOrQuery
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssContainerStyleOrQuery,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssContainerStyleOrQuery>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssContainerStyleOrQuery {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssContainerStyleOrQuery,
        crate::css::auxiliary::container_style_or_query::FormatCssContainerStyleOrQuery,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule :: new (self , crate :: css :: auxiliary :: container_style_or_query :: FormatCssContainerStyleOrQuery :: default ())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssContainerStyleOrQuery {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssContainerStyleOrQuery,
        crate::css::auxiliary::container_style_or_query::FormatCssContainerStyleOrQuery,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule :: new (self , crate :: css :: auxiliary :: container_style_or_query :: FormatCssContainerStyleOrQuery :: default ())
    }
}
impl FormatRule<biome_css_syntax::CssContainerStyleInParens>
    for crate::css::auxiliary::container_style_in_parens::FormatCssContainerStyleInParens
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssContainerStyleInParens,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssContainerStyleInParens>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssContainerStyleInParens {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssContainerStyleInParens,
        crate::css::auxiliary::container_style_in_parens::FormatCssContainerStyleInParens,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule :: new (self , crate :: css :: auxiliary :: container_style_in_parens :: FormatCssContainerStyleInParens :: default ())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssContainerStyleInParens {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssContainerStyleInParens,
        crate::css::auxiliary::container_style_in_parens::FormatCssContainerStyleInParens,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule :: new (self , crate :: css :: auxiliary :: container_style_in_parens :: FormatCssContainerStyleInParens :: default ())
    }
}
impl FormatRule<biome_css_syntax::CssKeyframesBlock>
    for crate::css::auxiliary::keyframes_block::FormatCssKeyframesBlock
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssKeyframesBlock,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssKeyframesBlock>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssKeyframesBlock {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssKeyframesBlock,
        crate::css::auxiliary::keyframes_block::FormatCssKeyframesBlock,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::keyframes_block::FormatCssKeyframesBlock::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssKeyframesBlock {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssKeyframesBlock,
        crate::css::auxiliary::keyframes_block::FormatCssKeyframesBlock,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::keyframes_block::FormatCssKeyframesBlock::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssKeyframesItem>
    for crate::css::auxiliary::keyframes_item::FormatCssKeyframesItem
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssKeyframesItem,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssKeyframesItem>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssKeyframesItem {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssKeyframesItem,
        crate::css::auxiliary::keyframes_item::FormatCssKeyframesItem,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::keyframes_item::FormatCssKeyframesItem::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssKeyframesItem {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssKeyframesItem,
        crate::css::auxiliary::keyframes_item::FormatCssKeyframesItem,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::keyframes_item::FormatCssKeyframesItem::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssKeyframesIdentSelector>
    for crate::css::selectors::keyframes_ident_selector::FormatCssKeyframesIdentSelector
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssKeyframesIdentSelector,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssKeyframesIdentSelector>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssKeyframesIdentSelector {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssKeyframesIdentSelector,
        crate::css::selectors::keyframes_ident_selector::FormatCssKeyframesIdentSelector,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule :: new (self , crate :: css :: selectors :: keyframes_ident_selector :: FormatCssKeyframesIdentSelector :: default ())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssKeyframesIdentSelector {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssKeyframesIdentSelector,
        crate::css::selectors::keyframes_ident_selector::FormatCssKeyframesIdentSelector,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule :: new (self , crate :: css :: selectors :: keyframes_ident_selector :: FormatCssKeyframesIdentSelector :: default ())
    }
}
impl FormatRule<biome_css_syntax::CssKeyframesPercentageSelector>
    for crate::css::selectors::keyframes_percentage_selector::FormatCssKeyframesPercentageSelector
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssKeyframesPercentageSelector,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssKeyframesPercentageSelector>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssKeyframesPercentageSelector {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssKeyframesPercentageSelector,
        crate::css::selectors::keyframes_percentage_selector::FormatCssKeyframesPercentageSelector,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule :: new (self , crate :: css :: selectors :: keyframes_percentage_selector :: FormatCssKeyframesPercentageSelector :: default ())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssKeyframesPercentageSelector {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssKeyframesPercentageSelector,
        crate::css::selectors::keyframes_percentage_selector::FormatCssKeyframesPercentageSelector,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule :: new (self , crate :: css :: selectors :: keyframes_percentage_selector :: FormatCssKeyframesPercentageSelector :: default ())
    }
}
impl FormatRule<biome_css_syntax::CssPercentage>
    for crate::css::value::percentage::FormatCssPercentage
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssPercentage,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssPercentage>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssPercentage {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssPercentage,
        crate::css::value::percentage::FormatCssPercentage,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::value::percentage::FormatCssPercentage::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssPercentage {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssPercentage,
        crate::css::value::percentage::FormatCssPercentage,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::value::percentage::FormatCssPercentage::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssMediaConditionQuery>
    for crate::css::auxiliary::media_condition_query::FormatCssMediaConditionQuery
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssMediaConditionQuery,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssMediaConditionQuery>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssMediaConditionQuery {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssMediaConditionQuery,
        crate::css::auxiliary::media_condition_query::FormatCssMediaConditionQuery,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::media_condition_query::FormatCssMediaConditionQuery::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssMediaConditionQuery {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssMediaConditionQuery,
        crate::css::auxiliary::media_condition_query::FormatCssMediaConditionQuery,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::media_condition_query::FormatCssMediaConditionQuery::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssMediaAndTypeQuery>
    for crate::css::auxiliary::media_and_type_query::FormatCssMediaAndTypeQuery
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssMediaAndTypeQuery,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssMediaAndTypeQuery>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssMediaAndTypeQuery {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssMediaAndTypeQuery,
        crate::css::auxiliary::media_and_type_query::FormatCssMediaAndTypeQuery,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::media_and_type_query::FormatCssMediaAndTypeQuery::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssMediaAndTypeQuery {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssMediaAndTypeQuery,
        crate::css::auxiliary::media_and_type_query::FormatCssMediaAndTypeQuery,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::media_and_type_query::FormatCssMediaAndTypeQuery::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssMediaTypeQuery>
    for crate::css::auxiliary::media_type_query::FormatCssMediaTypeQuery
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssMediaTypeQuery,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssMediaTypeQuery>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssMediaTypeQuery {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssMediaTypeQuery,
        crate::css::auxiliary::media_type_query::FormatCssMediaTypeQuery,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::media_type_query::FormatCssMediaTypeQuery::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssMediaTypeQuery {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssMediaTypeQuery,
        crate::css::auxiliary::media_type_query::FormatCssMediaTypeQuery,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::media_type_query::FormatCssMediaTypeQuery::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssMediaType>
    for crate::css::auxiliary::media_type::FormatCssMediaType
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_css_syntax::CssMediaType, f: &mut CssFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssMediaType>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssMediaType {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssMediaType,
        crate::css::auxiliary::media_type::FormatCssMediaType,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::media_type::FormatCssMediaType::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssMediaType {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssMediaType,
        crate::css::auxiliary::media_type::FormatCssMediaType,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::media_type::FormatCssMediaType::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssMediaNotCondition>
    for crate::css::auxiliary::media_not_condition::FormatCssMediaNotCondition
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssMediaNotCondition,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssMediaNotCondition>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssMediaNotCondition {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssMediaNotCondition,
        crate::css::auxiliary::media_not_condition::FormatCssMediaNotCondition,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::media_not_condition::FormatCssMediaNotCondition::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssMediaNotCondition {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssMediaNotCondition,
        crate::css::auxiliary::media_not_condition::FormatCssMediaNotCondition,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::media_not_condition::FormatCssMediaNotCondition::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssMediaAndCondition>
    for crate::css::auxiliary::media_and_condition::FormatCssMediaAndCondition
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssMediaAndCondition,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssMediaAndCondition>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssMediaAndCondition {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssMediaAndCondition,
        crate::css::auxiliary::media_and_condition::FormatCssMediaAndCondition,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::media_and_condition::FormatCssMediaAndCondition::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssMediaAndCondition {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssMediaAndCondition,
        crate::css::auxiliary::media_and_condition::FormatCssMediaAndCondition,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::media_and_condition::FormatCssMediaAndCondition::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssMediaOrCondition>
    for crate::css::auxiliary::media_or_condition::FormatCssMediaOrCondition
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssMediaOrCondition,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssMediaOrCondition>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssMediaOrCondition {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssMediaOrCondition,
        crate::css::auxiliary::media_or_condition::FormatCssMediaOrCondition,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::media_or_condition::FormatCssMediaOrCondition::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssMediaOrCondition {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssMediaOrCondition,
        crate::css::auxiliary::media_or_condition::FormatCssMediaOrCondition,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::media_or_condition::FormatCssMediaOrCondition::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssMediaConditionInParens>
    for crate::css::auxiliary::media_condition_in_parens::FormatCssMediaConditionInParens
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssMediaConditionInParens,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssMediaConditionInParens>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssMediaConditionInParens {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssMediaConditionInParens,
        crate::css::auxiliary::media_condition_in_parens::FormatCssMediaConditionInParens,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule :: new (self , crate :: css :: auxiliary :: media_condition_in_parens :: FormatCssMediaConditionInParens :: default ())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssMediaConditionInParens {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssMediaConditionInParens,
        crate::css::auxiliary::media_condition_in_parens::FormatCssMediaConditionInParens,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule :: new (self , crate :: css :: auxiliary :: media_condition_in_parens :: FormatCssMediaConditionInParens :: default ())
    }
}
impl FormatRule<biome_css_syntax::CssMediaFeatureInParens>
    for crate::css::auxiliary::media_feature_in_parens::FormatCssMediaFeatureInParens
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssMediaFeatureInParens,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssMediaFeatureInParens>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssMediaFeatureInParens {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssMediaFeatureInParens,
        crate::css::auxiliary::media_feature_in_parens::FormatCssMediaFeatureInParens,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::media_feature_in_parens::FormatCssMediaFeatureInParens::default(
            ),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssMediaFeatureInParens {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssMediaFeatureInParens,
        crate::css::auxiliary::media_feature_in_parens::FormatCssMediaFeatureInParens,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::media_feature_in_parens::FormatCssMediaFeatureInParens::default(
            ),
        )
    }
}
impl FormatRule<biome_css_syntax::CssQueryFeaturePlain>
    for crate::css::auxiliary::query_feature_plain::FormatCssQueryFeaturePlain
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssQueryFeaturePlain,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssQueryFeaturePlain>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssQueryFeaturePlain {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssQueryFeaturePlain,
        crate::css::auxiliary::query_feature_plain::FormatCssQueryFeaturePlain,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::query_feature_plain::FormatCssQueryFeaturePlain::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssQueryFeaturePlain {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssQueryFeaturePlain,
        crate::css::auxiliary::query_feature_plain::FormatCssQueryFeaturePlain,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::query_feature_plain::FormatCssQueryFeaturePlain::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssQueryFeatureBoolean>
    for crate::css::auxiliary::query_feature_boolean::FormatCssQueryFeatureBoolean
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssQueryFeatureBoolean,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssQueryFeatureBoolean>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssQueryFeatureBoolean {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssQueryFeatureBoolean,
        crate::css::auxiliary::query_feature_boolean::FormatCssQueryFeatureBoolean,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::query_feature_boolean::FormatCssQueryFeatureBoolean::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssQueryFeatureBoolean {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssQueryFeatureBoolean,
        crate::css::auxiliary::query_feature_boolean::FormatCssQueryFeatureBoolean,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::query_feature_boolean::FormatCssQueryFeatureBoolean::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssQueryFeatureRange>
    for crate::css::auxiliary::query_feature_range::FormatCssQueryFeatureRange
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssQueryFeatureRange,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssQueryFeatureRange>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssQueryFeatureRange {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssQueryFeatureRange,
        crate::css::auxiliary::query_feature_range::FormatCssQueryFeatureRange,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::query_feature_range::FormatCssQueryFeatureRange::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssQueryFeatureRange {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssQueryFeatureRange,
        crate::css::auxiliary::query_feature_range::FormatCssQueryFeatureRange,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::query_feature_range::FormatCssQueryFeatureRange::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssQueryFeatureReverseRange>
    for crate::css::auxiliary::query_feature_reverse_range::FormatCssQueryFeatureReverseRange
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssQueryFeatureReverseRange,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssQueryFeatureReverseRange>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssQueryFeatureReverseRange {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssQueryFeatureReverseRange,
        crate::css::auxiliary::query_feature_reverse_range::FormatCssQueryFeatureReverseRange,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule :: new (self , crate :: css :: auxiliary :: query_feature_reverse_range :: FormatCssQueryFeatureReverseRange :: default ())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssQueryFeatureReverseRange {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssQueryFeatureReverseRange,
        crate::css::auxiliary::query_feature_reverse_range::FormatCssQueryFeatureReverseRange,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule :: new (self , crate :: css :: auxiliary :: query_feature_reverse_range :: FormatCssQueryFeatureReverseRange :: default ())
    }
}
impl FormatRule<biome_css_syntax::CssQueryFeatureRangeInterval>
    for crate::css::auxiliary::query_feature_range_interval::FormatCssQueryFeatureRangeInterval
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssQueryFeatureRangeInterval,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssQueryFeatureRangeInterval>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssQueryFeatureRangeInterval {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssQueryFeatureRangeInterval,
        crate::css::auxiliary::query_feature_range_interval::FormatCssQueryFeatureRangeInterval,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule :: new (self , crate :: css :: auxiliary :: query_feature_range_interval :: FormatCssQueryFeatureRangeInterval :: default ())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssQueryFeatureRangeInterval {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssQueryFeatureRangeInterval,
        crate::css::auxiliary::query_feature_range_interval::FormatCssQueryFeatureRangeInterval,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule :: new (self , crate :: css :: auxiliary :: query_feature_range_interval :: FormatCssQueryFeatureRangeInterval :: default ())
    }
}
impl FormatRule<biome_css_syntax::CssQueryFeatureRangeComparison>
    for crate::css::auxiliary::query_feature_range_comparison::FormatCssQueryFeatureRangeComparison
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssQueryFeatureRangeComparison,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssQueryFeatureRangeComparison>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssQueryFeatureRangeComparison {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssQueryFeatureRangeComparison,
        crate::css::auxiliary::query_feature_range_comparison::FormatCssQueryFeatureRangeComparison,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule :: new (self , crate :: css :: auxiliary :: query_feature_range_comparison :: FormatCssQueryFeatureRangeComparison :: default ())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssQueryFeatureRangeComparison {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssQueryFeatureRangeComparison,
        crate::css::auxiliary::query_feature_range_comparison::FormatCssQueryFeatureRangeComparison,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule :: new (self , crate :: css :: auxiliary :: query_feature_range_comparison :: FormatCssQueryFeatureRangeComparison :: default ())
    }
}
impl FormatRule<biome_css_syntax::CssRatio> for crate::css::value::ratio::FormatCssRatio {
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_css_syntax::CssRatio, f: &mut CssFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssRatio>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssRatio {
    type Format<'a> =
        FormatRefWithRule<'a, biome_css_syntax::CssRatio, crate::css::value::ratio::FormatCssRatio>;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(self, crate::css::value::ratio::FormatCssRatio::default())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssRatio {
    type Format =
        FormatOwnedWithRule<biome_css_syntax::CssRatio, crate::css::value::ratio::FormatCssRatio>;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(self, crate::css::value::ratio::FormatCssRatio::default())
    }
}
impl FormatRule<biome_css_syntax::CssPageSelector>
    for crate::css::selectors::page_selector::FormatCssPageSelector
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssPageSelector,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssPageSelector>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssPageSelector {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssPageSelector,
        crate::css::selectors::page_selector::FormatCssPageSelector,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::selectors::page_selector::FormatCssPageSelector::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssPageSelector {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssPageSelector,
        crate::css::selectors::page_selector::FormatCssPageSelector,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::selectors::page_selector::FormatCssPageSelector::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssPageSelectorPseudo>
    for crate::css::pseudo::page_selector_pseudo::FormatCssPageSelectorPseudo
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssPageSelectorPseudo,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssPageSelectorPseudo>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssPageSelectorPseudo {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssPageSelectorPseudo,
        crate::css::pseudo::page_selector_pseudo::FormatCssPageSelectorPseudo,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::pseudo::page_selector_pseudo::FormatCssPageSelectorPseudo::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssPageSelectorPseudo {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssPageSelectorPseudo,
        crate::css::pseudo::page_selector_pseudo::FormatCssPageSelectorPseudo,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::pseudo::page_selector_pseudo::FormatCssPageSelectorPseudo::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssPageAtRuleBlock>
    for crate::css::auxiliary::page_at_rule_block::FormatCssPageAtRuleBlock
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssPageAtRuleBlock,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssPageAtRuleBlock>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssPageAtRuleBlock {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssPageAtRuleBlock,
        crate::css::auxiliary::page_at_rule_block::FormatCssPageAtRuleBlock,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::page_at_rule_block::FormatCssPageAtRuleBlock::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssPageAtRuleBlock {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssPageAtRuleBlock,
        crate::css::auxiliary::page_at_rule_block::FormatCssPageAtRuleBlock,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::page_at_rule_block::FormatCssPageAtRuleBlock::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssMarginAtRule>
    for crate::css::statements::margin_at_rule::FormatCssMarginAtRule
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssMarginAtRule,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssMarginAtRule>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssMarginAtRule {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssMarginAtRule,
        crate::css::statements::margin_at_rule::FormatCssMarginAtRule,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::statements::margin_at_rule::FormatCssMarginAtRule::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssMarginAtRule {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssMarginAtRule,
        crate::css::statements::margin_at_rule::FormatCssMarginAtRule,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::statements::margin_at_rule::FormatCssMarginAtRule::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssLayerDeclaration>
    for crate::css::auxiliary::layer_declaration::FormatCssLayerDeclaration
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssLayerDeclaration,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssLayerDeclaration>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssLayerDeclaration {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssLayerDeclaration,
        crate::css::auxiliary::layer_declaration::FormatCssLayerDeclaration,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::layer_declaration::FormatCssLayerDeclaration::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssLayerDeclaration {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssLayerDeclaration,
        crate::css::auxiliary::layer_declaration::FormatCssLayerDeclaration,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::layer_declaration::FormatCssLayerDeclaration::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssLayerReference>
    for crate::css::auxiliary::layer_reference::FormatCssLayerReference
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssLayerReference,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssLayerReference>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssLayerReference {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssLayerReference,
        crate::css::auxiliary::layer_reference::FormatCssLayerReference,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::layer_reference::FormatCssLayerReference::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssLayerReference {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssLayerReference,
        crate::css::auxiliary::layer_reference::FormatCssLayerReference,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::layer_reference::FormatCssLayerReference::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssSupportsNotCondition>
    for crate::css::auxiliary::supports_not_condition::FormatCssSupportsNotCondition
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssSupportsNotCondition,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssSupportsNotCondition>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssSupportsNotCondition {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssSupportsNotCondition,
        crate::css::auxiliary::supports_not_condition::FormatCssSupportsNotCondition,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::supports_not_condition::FormatCssSupportsNotCondition::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssSupportsNotCondition {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssSupportsNotCondition,
        crate::css::auxiliary::supports_not_condition::FormatCssSupportsNotCondition,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::supports_not_condition::FormatCssSupportsNotCondition::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssSupportsOrCondition>
    for crate::css::auxiliary::supports_or_condition::FormatCssSupportsOrCondition
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssSupportsOrCondition,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssSupportsOrCondition>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssSupportsOrCondition {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssSupportsOrCondition,
        crate::css::auxiliary::supports_or_condition::FormatCssSupportsOrCondition,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::supports_or_condition::FormatCssSupportsOrCondition::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssSupportsOrCondition {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssSupportsOrCondition,
        crate::css::auxiliary::supports_or_condition::FormatCssSupportsOrCondition,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::supports_or_condition::FormatCssSupportsOrCondition::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssSupportsAndCondition>
    for crate::css::auxiliary::supports_and_condition::FormatCssSupportsAndCondition
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssSupportsAndCondition,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssSupportsAndCondition>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssSupportsAndCondition {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssSupportsAndCondition,
        crate::css::auxiliary::supports_and_condition::FormatCssSupportsAndCondition,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::supports_and_condition::FormatCssSupportsAndCondition::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssSupportsAndCondition {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssSupportsAndCondition,
        crate::css::auxiliary::supports_and_condition::FormatCssSupportsAndCondition,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::supports_and_condition::FormatCssSupportsAndCondition::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssSupportsConditionInParens>
    for crate::css::auxiliary::supports_condition_in_parens::FormatCssSupportsConditionInParens
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssSupportsConditionInParens,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssSupportsConditionInParens>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssSupportsConditionInParens {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssSupportsConditionInParens,
        crate::css::auxiliary::supports_condition_in_parens::FormatCssSupportsConditionInParens,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule :: new (self , crate :: css :: auxiliary :: supports_condition_in_parens :: FormatCssSupportsConditionInParens :: default ())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssSupportsConditionInParens {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssSupportsConditionInParens,
        crate::css::auxiliary::supports_condition_in_parens::FormatCssSupportsConditionInParens,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule :: new (self , crate :: css :: auxiliary :: supports_condition_in_parens :: FormatCssSupportsConditionInParens :: default ())
    }
}
impl FormatRule<biome_css_syntax::CssSupportsFeatureDeclaration>
    for crate::css::auxiliary::supports_feature_declaration::FormatCssSupportsFeatureDeclaration
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssSupportsFeatureDeclaration,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssSupportsFeatureDeclaration>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssSupportsFeatureDeclaration {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssSupportsFeatureDeclaration,
        crate::css::auxiliary::supports_feature_declaration::FormatCssSupportsFeatureDeclaration,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule :: new (self , crate :: css :: auxiliary :: supports_feature_declaration :: FormatCssSupportsFeatureDeclaration :: default ())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssSupportsFeatureDeclaration {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssSupportsFeatureDeclaration,
        crate::css::auxiliary::supports_feature_declaration::FormatCssSupportsFeatureDeclaration,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule :: new (self , crate :: css :: auxiliary :: supports_feature_declaration :: FormatCssSupportsFeatureDeclaration :: default ())
    }
}
impl FormatRule<biome_css_syntax::CssSupportsFeatureSelector>
    for crate::css::selectors::supports_feature_selector::FormatCssSupportsFeatureSelector
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssSupportsFeatureSelector,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssSupportsFeatureSelector>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssSupportsFeatureSelector {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssSupportsFeatureSelector,
        crate::css::selectors::supports_feature_selector::FormatCssSupportsFeatureSelector,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule :: new (self , crate :: css :: selectors :: supports_feature_selector :: FormatCssSupportsFeatureSelector :: default ())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssSupportsFeatureSelector {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssSupportsFeatureSelector,
        crate::css::selectors::supports_feature_selector::FormatCssSupportsFeatureSelector,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule :: new (self , crate :: css :: selectors :: supports_feature_selector :: FormatCssSupportsFeatureSelector :: default ())
    }
}
impl FormatRule<biome_css_syntax::CssFunction>
    for crate::css::auxiliary::function::FormatCssFunction
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_css_syntax::CssFunction, f: &mut CssFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssFunction>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssFunction {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssFunction,
        crate::css::auxiliary::function::FormatCssFunction,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::function::FormatCssFunction::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssFunction {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssFunction,
        crate::css::auxiliary::function::FormatCssFunction,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::function::FormatCssFunction::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssScopeRangeStart>
    for crate::css::auxiliary::scope_range_start::FormatCssScopeRangeStart
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssScopeRangeStart,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssScopeRangeStart>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssScopeRangeStart {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssScopeRangeStart,
        crate::css::auxiliary::scope_range_start::FormatCssScopeRangeStart,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::scope_range_start::FormatCssScopeRangeStart::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssScopeRangeStart {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssScopeRangeStart,
        crate::css::auxiliary::scope_range_start::FormatCssScopeRangeStart,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::scope_range_start::FormatCssScopeRangeStart::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssScopeRangeEnd>
    for crate::css::auxiliary::scope_range_end::FormatCssScopeRangeEnd
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssScopeRangeEnd,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssScopeRangeEnd>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssScopeRangeEnd {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssScopeRangeEnd,
        crate::css::auxiliary::scope_range_end::FormatCssScopeRangeEnd,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::scope_range_end::FormatCssScopeRangeEnd::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssScopeRangeEnd {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssScopeRangeEnd,
        crate::css::auxiliary::scope_range_end::FormatCssScopeRangeEnd,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::scope_range_end::FormatCssScopeRangeEnd::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssScopeRangeInterval>
    for crate::css::auxiliary::scope_range_interval::FormatCssScopeRangeInterval
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssScopeRangeInterval,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssScopeRangeInterval>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssScopeRangeInterval {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssScopeRangeInterval,
        crate::css::auxiliary::scope_range_interval::FormatCssScopeRangeInterval,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::scope_range_interval::FormatCssScopeRangeInterval::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssScopeRangeInterval {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssScopeRangeInterval,
        crate::css::auxiliary::scope_range_interval::FormatCssScopeRangeInterval,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::scope_range_interval::FormatCssScopeRangeInterval::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssScopeEdge>
    for crate::css::auxiliary::scope_edge::FormatCssScopeEdge
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_css_syntax::CssScopeEdge, f: &mut CssFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssScopeEdge>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssScopeEdge {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssScopeEdge,
        crate::css::auxiliary::scope_edge::FormatCssScopeEdge,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::scope_edge::FormatCssScopeEdge::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssScopeEdge {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssScopeEdge,
        crate::css::auxiliary::scope_edge::FormatCssScopeEdge,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::scope_edge::FormatCssScopeEdge::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssImportSupports>
    for crate::css::auxiliary::import_supports::FormatCssImportSupports
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssImportSupports,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssImportSupports>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssImportSupports {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssImportSupports,
        crate::css::auxiliary::import_supports::FormatCssImportSupports,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::import_supports::FormatCssImportSupports::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssImportSupports {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssImportSupports,
        crate::css::auxiliary::import_supports::FormatCssImportSupports,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::import_supports::FormatCssImportSupports::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssUrlFunction>
    for crate::css::auxiliary::url_function::FormatCssUrlFunction
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssUrlFunction,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssUrlFunction>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssUrlFunction {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssUrlFunction,
        crate::css::auxiliary::url_function::FormatCssUrlFunction,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::url_function::FormatCssUrlFunction::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssUrlFunction {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssUrlFunction,
        crate::css::auxiliary::url_function::FormatCssUrlFunction,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::url_function::FormatCssUrlFunction::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssImportAnonymousLayer>
    for crate::css::auxiliary::import_anonymous_layer::FormatCssImportAnonymousLayer
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssImportAnonymousLayer,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssImportAnonymousLayer>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssImportAnonymousLayer {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssImportAnonymousLayer,
        crate::css::auxiliary::import_anonymous_layer::FormatCssImportAnonymousLayer,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::import_anonymous_layer::FormatCssImportAnonymousLayer::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssImportAnonymousLayer {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssImportAnonymousLayer,
        crate::css::auxiliary::import_anonymous_layer::FormatCssImportAnonymousLayer,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::import_anonymous_layer::FormatCssImportAnonymousLayer::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssImportNamedLayer>
    for crate::css::auxiliary::import_named_layer::FormatCssImportNamedLayer
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssImportNamedLayer,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssImportNamedLayer>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssImportNamedLayer {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssImportNamedLayer,
        crate::css::auxiliary::import_named_layer::FormatCssImportNamedLayer,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::import_named_layer::FormatCssImportNamedLayer::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssImportNamedLayer {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssImportNamedLayer,
        crate::css::auxiliary::import_named_layer::FormatCssImportNamedLayer,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::import_named_layer::FormatCssImportNamedLayer::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssDocumentCustomMatcher>
    for crate::css::auxiliary::document_custom_matcher::FormatCssDocumentCustomMatcher
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssDocumentCustomMatcher,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssDocumentCustomMatcher>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssDocumentCustomMatcher {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssDocumentCustomMatcher,
        crate::css::auxiliary::document_custom_matcher::FormatCssDocumentCustomMatcher,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::document_custom_matcher::FormatCssDocumentCustomMatcher::default(
            ),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssDocumentCustomMatcher {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssDocumentCustomMatcher,
        crate::css::auxiliary::document_custom_matcher::FormatCssDocumentCustomMatcher,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::document_custom_matcher::FormatCssDocumentCustomMatcher::default(
            ),
        )
    }
}
impl FormatRule<biome_css_syntax::CssUnknownDimension>
    for crate::css::value::unknown_dimension::FormatCssUnknownDimension
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssUnknownDimension,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssUnknownDimension>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssUnknownDimension {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssUnknownDimension,
        crate::css::value::unknown_dimension::FormatCssUnknownDimension,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::value::unknown_dimension::FormatCssUnknownDimension::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssUnknownDimension {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssUnknownDimension,
        crate::css::value::unknown_dimension::FormatCssUnknownDimension,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::value::unknown_dimension::FormatCssUnknownDimension::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssUrlValueRaw>
    for crate::css::value::url_value_raw::FormatCssUrlValueRaw
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssUrlValueRaw,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssUrlValueRaw>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssUrlValueRaw {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssUrlValueRaw,
        crate::css::value::url_value_raw::FormatCssUrlValueRaw,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::value::url_value_raw::FormatCssUrlValueRaw::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssUrlValueRaw {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssUrlValueRaw,
        crate::css::value::url_value_raw::FormatCssUrlValueRaw,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::value::url_value_raw::FormatCssUrlValueRaw::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssParameter>
    for crate::css::auxiliary::parameter::FormatCssParameter
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_css_syntax::CssParameter, f: &mut CssFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssParameter>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssParameter {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssParameter,
        crate::css::auxiliary::parameter::FormatCssParameter,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::parameter::FormatCssParameter::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssParameter {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssParameter,
        crate::css::auxiliary::parameter::FormatCssParameter,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::parameter::FormatCssParameter::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssBinaryExpression>
    for crate::css::auxiliary::binary_expression::FormatCssBinaryExpression
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssBinaryExpression,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssBinaryExpression>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssBinaryExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssBinaryExpression,
        crate::css::auxiliary::binary_expression::FormatCssBinaryExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::binary_expression::FormatCssBinaryExpression::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssBinaryExpression {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssBinaryExpression,
        crate::css::auxiliary::binary_expression::FormatCssBinaryExpression,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::binary_expression::FormatCssBinaryExpression::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssParenthesizedExpression>
    for crate::css::auxiliary::parenthesized_expression::FormatCssParenthesizedExpression
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssParenthesizedExpression,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssParenthesizedExpression>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssParenthesizedExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssParenthesizedExpression,
        crate::css::auxiliary::parenthesized_expression::FormatCssParenthesizedExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule :: new (self , crate :: css :: auxiliary :: parenthesized_expression :: FormatCssParenthesizedExpression :: default ())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssParenthesizedExpression {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssParenthesizedExpression,
        crate::css::auxiliary::parenthesized_expression::FormatCssParenthesizedExpression,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule :: new (self , crate :: css :: auxiliary :: parenthesized_expression :: FormatCssParenthesizedExpression :: default ())
    }
}
impl FormatRule < biome_css_syntax :: CssListOfComponentValuesExpression > for crate :: css :: auxiliary :: list_of_component_values_expression :: FormatCssListOfComponentValuesExpression { type Context = CssFormatContext ; # [inline (always)] fn fmt (& self , node : & biome_css_syntax :: CssListOfComponentValuesExpression , f : & mut CssFormatter) -> FormatResult < () > { FormatNodeRule :: < biome_css_syntax :: CssListOfComponentValuesExpression > :: fmt (self , node , f) } }
impl AsFormat<CssFormatContext> for biome_css_syntax::CssListOfComponentValuesExpression {
    type Format < 'a > = FormatRefWithRule < 'a , biome_css_syntax :: CssListOfComponentValuesExpression , crate :: css :: auxiliary :: list_of_component_values_expression :: FormatCssListOfComponentValuesExpression > ;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule :: new (self , crate :: css :: auxiliary :: list_of_component_values_expression :: FormatCssListOfComponentValuesExpression :: default ())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssListOfComponentValuesExpression {
    type Format = FormatOwnedWithRule < biome_css_syntax :: CssListOfComponentValuesExpression , crate :: css :: auxiliary :: list_of_component_values_expression :: FormatCssListOfComponentValuesExpression > ;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule :: new (self , crate :: css :: auxiliary :: list_of_component_values_expression :: FormatCssListOfComponentValuesExpression :: default ())
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssComponentValueList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssComponentValueList,
        crate::css::lists::component_value_list::FormatCssComponentValueList,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::lists::component_value_list::FormatCssComponentValueList::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssComponentValueList {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssComponentValueList,
        crate::css::lists::component_value_list::FormatCssComponentValueList,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::lists::component_value_list::FormatCssComponentValueList::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssCompoundSelectorList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssCompoundSelectorList,
        crate::css::lists::compound_selector_list::FormatCssCompoundSelectorList,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::lists::compound_selector_list::FormatCssCompoundSelectorList::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssCompoundSelectorList {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssCompoundSelectorList,
        crate::css::lists::compound_selector_list::FormatCssCompoundSelectorList,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::lists::compound_selector_list::FormatCssCompoundSelectorList::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssDeclarationList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssDeclarationList,
        crate::css::lists::declaration_list::FormatCssDeclarationList,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::lists::declaration_list::FormatCssDeclarationList::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssDeclarationList {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssDeclarationList,
        crate::css::lists::declaration_list::FormatCssDeclarationList,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::lists::declaration_list::FormatCssDeclarationList::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssDeclarationOrAtRuleList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssDeclarationOrAtRuleList,
        crate::css::lists::declaration_or_at_rule_list::FormatCssDeclarationOrAtRuleList,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule :: new (self , crate :: css :: lists :: declaration_or_at_rule_list :: FormatCssDeclarationOrAtRuleList :: default ())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssDeclarationOrAtRuleList {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssDeclarationOrAtRuleList,
        crate::css::lists::declaration_or_at_rule_list::FormatCssDeclarationOrAtRuleList,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule :: new (self , crate :: css :: lists :: declaration_or_at_rule_list :: FormatCssDeclarationOrAtRuleList :: default ())
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssDeclarationOrRuleList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssDeclarationOrRuleList,
        crate::css::lists::declaration_or_rule_list::FormatCssDeclarationOrRuleList,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::lists::declaration_or_rule_list::FormatCssDeclarationOrRuleList::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssDeclarationOrRuleList {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssDeclarationOrRuleList,
        crate::css::lists::declaration_or_rule_list::FormatCssDeclarationOrRuleList,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::lists::declaration_or_rule_list::FormatCssDeclarationOrRuleList::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssDocumentMatcherList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssDocumentMatcherList,
        crate::css::lists::document_matcher_list::FormatCssDocumentMatcherList,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::lists::document_matcher_list::FormatCssDocumentMatcherList::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssDocumentMatcherList {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssDocumentMatcherList,
        crate::css::lists::document_matcher_list::FormatCssDocumentMatcherList,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::lists::document_matcher_list::FormatCssDocumentMatcherList::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssFontFeatureValuesItemList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssFontFeatureValuesItemList,
        crate::css::lists::font_feature_values_item_list::FormatCssFontFeatureValuesItemList,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule :: new (self , crate :: css :: lists :: font_feature_values_item_list :: FormatCssFontFeatureValuesItemList :: default ())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssFontFeatureValuesItemList {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssFontFeatureValuesItemList,
        crate::css::lists::font_feature_values_item_list::FormatCssFontFeatureValuesItemList,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule :: new (self , crate :: css :: lists :: font_feature_values_item_list :: FormatCssFontFeatureValuesItemList :: default ())
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssGenericComponentValueList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssGenericComponentValueList,
        crate::css::lists::generic_component_value_list::FormatCssGenericComponentValueList,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule :: new (self , crate :: css :: lists :: generic_component_value_list :: FormatCssGenericComponentValueList :: default ())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssGenericComponentValueList {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssGenericComponentValueList,
        crate::css::lists::generic_component_value_list::FormatCssGenericComponentValueList,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule :: new (self , crate :: css :: lists :: generic_component_value_list :: FormatCssGenericComponentValueList :: default ())
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssKeyframesItemList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssKeyframesItemList,
        crate::css::lists::keyframes_item_list::FormatCssKeyframesItemList,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::lists::keyframes_item_list::FormatCssKeyframesItemList::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssKeyframesItemList {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssKeyframesItemList,
        crate::css::lists::keyframes_item_list::FormatCssKeyframesItemList,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::lists::keyframes_item_list::FormatCssKeyframesItemList::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssKeyframesSelectorList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssKeyframesSelectorList,
        crate::css::lists::keyframes_selector_list::FormatCssKeyframesSelectorList,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::lists::keyframes_selector_list::FormatCssKeyframesSelectorList::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssKeyframesSelectorList {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssKeyframesSelectorList,
        crate::css::lists::keyframes_selector_list::FormatCssKeyframesSelectorList,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::lists::keyframes_selector_list::FormatCssKeyframesSelectorList::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssLayerNameList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssLayerNameList,
        crate::css::lists::layer_name_list::FormatCssLayerNameList,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::lists::layer_name_list::FormatCssLayerNameList::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssLayerNameList {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssLayerNameList,
        crate::css::lists::layer_name_list::FormatCssLayerNameList,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::lists::layer_name_list::FormatCssLayerNameList::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssLayerReferenceList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssLayerReferenceList,
        crate::css::lists::layer_reference_list::FormatCssLayerReferenceList,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::lists::layer_reference_list::FormatCssLayerReferenceList::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssLayerReferenceList {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssLayerReferenceList,
        crate::css::lists::layer_reference_list::FormatCssLayerReferenceList,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::lists::layer_reference_list::FormatCssLayerReferenceList::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssMediaQueryList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssMediaQueryList,
        crate::css::lists::media_query_list::FormatCssMediaQueryList,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::lists::media_query_list::FormatCssMediaQueryList::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssMediaQueryList {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssMediaQueryList,
        crate::css::lists::media_query_list::FormatCssMediaQueryList,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::lists::media_query_list::FormatCssMediaQueryList::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssPageAtRuleItemList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssPageAtRuleItemList,
        crate::css::lists::page_at_rule_item_list::FormatCssPageAtRuleItemList,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::lists::page_at_rule_item_list::FormatCssPageAtRuleItemList::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssPageAtRuleItemList {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssPageAtRuleItemList,
        crate::css::lists::page_at_rule_item_list::FormatCssPageAtRuleItemList,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::lists::page_at_rule_item_list::FormatCssPageAtRuleItemList::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssPageSelectorList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssPageSelectorList,
        crate::css::lists::page_selector_list::FormatCssPageSelectorList,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::lists::page_selector_list::FormatCssPageSelectorList::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssPageSelectorList {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssPageSelectorList,
        crate::css::lists::page_selector_list::FormatCssPageSelectorList,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::lists::page_selector_list::FormatCssPageSelectorList::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssPageSelectorPseudoList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssPageSelectorPseudoList,
        crate::css::lists::page_selector_pseudo_list::FormatCssPageSelectorPseudoList,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::lists::page_selector_pseudo_list::FormatCssPageSelectorPseudoList::default(
            ),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssPageSelectorPseudoList {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssPageSelectorPseudoList,
        crate::css::lists::page_selector_pseudo_list::FormatCssPageSelectorPseudoList,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::lists::page_selector_pseudo_list::FormatCssPageSelectorPseudoList::default(
            ),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssParameterList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssParameterList,
        crate::css::lists::parameter_list::FormatCssParameterList,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::lists::parameter_list::FormatCssParameterList::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssParameterList {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssParameterList,
        crate::css::lists::parameter_list::FormatCssParameterList,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::lists::parameter_list::FormatCssParameterList::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssPseudoValueList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssPseudoValueList,
        crate::css::lists::pseudo_value_list::FormatCssPseudoValueList,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::lists::pseudo_value_list::FormatCssPseudoValueList::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssPseudoValueList {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssPseudoValueList,
        crate::css::lists::pseudo_value_list::FormatCssPseudoValueList,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::lists::pseudo_value_list::FormatCssPseudoValueList::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssRelativeSelectorList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssRelativeSelectorList,
        crate::css::lists::relative_selector_list::FormatCssRelativeSelectorList,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::lists::relative_selector_list::FormatCssRelativeSelectorList::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssRelativeSelectorList {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssRelativeSelectorList,
        crate::css::lists::relative_selector_list::FormatCssRelativeSelectorList,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::lists::relative_selector_list::FormatCssRelativeSelectorList::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssRuleList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssRuleList,
        crate::css::lists::rule_list::FormatCssRuleList,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::lists::rule_list::FormatCssRuleList::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssRuleList {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssRuleList,
        crate::css::lists::rule_list::FormatCssRuleList,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::lists::rule_list::FormatCssRuleList::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssSelectorList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssSelectorList,
        crate::css::lists::selector_list::FormatCssSelectorList,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::lists::selector_list::FormatCssSelectorList::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssSelectorList {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssSelectorList,
        crate::css::lists::selector_list::FormatCssSelectorList,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::lists::selector_list::FormatCssSelectorList::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssSubSelectorList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssSubSelectorList,
        crate::css::lists::sub_selector_list::FormatCssSubSelectorList,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::lists::sub_selector_list::FormatCssSubSelectorList::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssSubSelectorList {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssSubSelectorList,
        crate::css::lists::sub_selector_list::FormatCssSubSelectorList,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::lists::sub_selector_list::FormatCssSubSelectorList::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssUrlModifierList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssUrlModifierList,
        crate::css::lists::url_modifier_list::FormatCssUrlModifierList,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::lists::url_modifier_list::FormatCssUrlModifierList::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssUrlModifierList {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssUrlModifierList,
        crate::css::lists::url_modifier_list::FormatCssUrlModifierList,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::lists::url_modifier_list::FormatCssUrlModifierList::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssBogus> for crate::css::bogus::bogus::FormatCssBogus {
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_css_syntax::CssBogus, f: &mut CssFormatter) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_css_syntax::CssBogus>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssBogus {
    type Format<'a> =
        FormatRefWithRule<'a, biome_css_syntax::CssBogus, crate::css::bogus::bogus::FormatCssBogus>;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(self, crate::css::bogus::bogus::FormatCssBogus::default())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssBogus {
    type Format =
        FormatOwnedWithRule<biome_css_syntax::CssBogus, crate::css::bogus::bogus::FormatCssBogus>;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(self, crate::css::bogus::bogus::FormatCssBogus::default())
    }
}
impl FormatRule<biome_css_syntax::CssBogusSelector>
    for crate::css::bogus::bogus_selector::FormatCssBogusSelector
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssBogusSelector,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_css_syntax::CssBogusSelector>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssBogusSelector {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssBogusSelector,
        crate::css::bogus::bogus_selector::FormatCssBogusSelector,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::bogus::bogus_selector::FormatCssBogusSelector::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssBogusSelector {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssBogusSelector,
        crate::css::bogus::bogus_selector::FormatCssBogusSelector,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::bogus::bogus_selector::FormatCssBogusSelector::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssBogusSubSelector>
    for crate::css::bogus::bogus_sub_selector::FormatCssBogusSubSelector
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssBogusSubSelector,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_css_syntax::CssBogusSubSelector>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssBogusSubSelector {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssBogusSubSelector,
        crate::css::bogus::bogus_sub_selector::FormatCssBogusSubSelector,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::bogus::bogus_sub_selector::FormatCssBogusSubSelector::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssBogusSubSelector {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssBogusSubSelector,
        crate::css::bogus::bogus_sub_selector::FormatCssBogusSubSelector,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::bogus::bogus_sub_selector::FormatCssBogusSubSelector::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssBogusPseudoClass>
    for crate::css::bogus::bogus_pseudo_class::FormatCssBogusPseudoClass
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssBogusPseudoClass,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_css_syntax::CssBogusPseudoClass>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssBogusPseudoClass {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssBogusPseudoClass,
        crate::css::bogus::bogus_pseudo_class::FormatCssBogusPseudoClass,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::bogus::bogus_pseudo_class::FormatCssBogusPseudoClass::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssBogusPseudoClass {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssBogusPseudoClass,
        crate::css::bogus::bogus_pseudo_class::FormatCssBogusPseudoClass,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::bogus::bogus_pseudo_class::FormatCssBogusPseudoClass::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssBogusPageSelectorPseudo>
    for crate::css::bogus::bogus_page_selector_pseudo::FormatCssBogusPageSelectorPseudo
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssBogusPageSelectorPseudo,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_css_syntax::CssBogusPageSelectorPseudo>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssBogusPageSelectorPseudo {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssBogusPageSelectorPseudo,
        crate::css::bogus::bogus_page_selector_pseudo::FormatCssBogusPageSelectorPseudo,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule :: new (self , crate :: css :: bogus :: bogus_page_selector_pseudo :: FormatCssBogusPageSelectorPseudo :: default ())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssBogusPageSelectorPseudo {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssBogusPageSelectorPseudo,
        crate::css::bogus::bogus_page_selector_pseudo::FormatCssBogusPageSelectorPseudo,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule :: new (self , crate :: css :: bogus :: bogus_page_selector_pseudo :: FormatCssBogusPageSelectorPseudo :: default ())
    }
}
impl FormatRule<biome_css_syntax::CssBogusPseudoElement>
    for crate::css::bogus::bogus_pseudo_element::FormatCssBogusPseudoElement
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssBogusPseudoElement,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_css_syntax::CssBogusPseudoElement>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssBogusPseudoElement {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssBogusPseudoElement,
        crate::css::bogus::bogus_pseudo_element::FormatCssBogusPseudoElement,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::bogus::bogus_pseudo_element::FormatCssBogusPseudoElement::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssBogusPseudoElement {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssBogusPseudoElement,
        crate::css::bogus::bogus_pseudo_element::FormatCssBogusPseudoElement,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::bogus::bogus_pseudo_element::FormatCssBogusPseudoElement::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssBogusAtRule>
    for crate::css::bogus::bogus_at_rule::FormatCssBogusAtRule
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssBogusAtRule,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_css_syntax::CssBogusAtRule>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssBogusAtRule {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssBogusAtRule,
        crate::css::bogus::bogus_at_rule::FormatCssBogusAtRule,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::bogus::bogus_at_rule::FormatCssBogusAtRule::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssBogusAtRule {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssBogusAtRule,
        crate::css::bogus::bogus_at_rule::FormatCssBogusAtRule,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::bogus::bogus_at_rule::FormatCssBogusAtRule::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssBogusLayer>
    for crate::css::bogus::bogus_layer::FormatCssBogusLayer
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssBogusLayer,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_css_syntax::CssBogusLayer>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssBogusLayer {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssBogusLayer,
        crate::css::bogus::bogus_layer::FormatCssBogusLayer,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::bogus::bogus_layer::FormatCssBogusLayer::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssBogusLayer {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssBogusLayer,
        crate::css::bogus::bogus_layer::FormatCssBogusLayer,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::bogus::bogus_layer::FormatCssBogusLayer::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssBogusBlock>
    for crate::css::bogus::bogus_block::FormatCssBogusBlock
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssBogusBlock,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_css_syntax::CssBogusBlock>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssBogusBlock {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssBogusBlock,
        crate::css::bogus::bogus_block::FormatCssBogusBlock,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::bogus::bogus_block::FormatCssBogusBlock::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssBogusBlock {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssBogusBlock,
        crate::css::bogus::bogus_block::FormatCssBogusBlock,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::bogus::bogus_block::FormatCssBogusBlock::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssBogusScopeRange>
    for crate::css::bogus::bogus_scope_range::FormatCssBogusScopeRange
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssBogusScopeRange,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_css_syntax::CssBogusScopeRange>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssBogusScopeRange {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssBogusScopeRange,
        crate::css::bogus::bogus_scope_range::FormatCssBogusScopeRange,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::bogus::bogus_scope_range::FormatCssBogusScopeRange::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssBogusScopeRange {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssBogusScopeRange,
        crate::css::bogus::bogus_scope_range::FormatCssBogusScopeRange,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::bogus::bogus_scope_range::FormatCssBogusScopeRange::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssBogusFontFeatureValuesItem>
    for crate::css::bogus::bogus_font_feature_values_item::FormatCssBogusFontFeatureValuesItem
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssBogusFontFeatureValuesItem,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_css_syntax::CssBogusFontFeatureValuesItem>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssBogusFontFeatureValuesItem {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssBogusFontFeatureValuesItem,
        crate::css::bogus::bogus_font_feature_values_item::FormatCssBogusFontFeatureValuesItem,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule :: new (self , crate :: css :: bogus :: bogus_font_feature_values_item :: FormatCssBogusFontFeatureValuesItem :: default ())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssBogusFontFeatureValuesItem {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssBogusFontFeatureValuesItem,
        crate::css::bogus::bogus_font_feature_values_item::FormatCssBogusFontFeatureValuesItem,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule :: new (self , crate :: css :: bogus :: bogus_font_feature_values_item :: FormatCssBogusFontFeatureValuesItem :: default ())
    }
}
impl FormatRule<biome_css_syntax::CssBogusKeyframesItem>
    for crate::css::bogus::bogus_keyframes_item::FormatCssBogusKeyframesItem
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssBogusKeyframesItem,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_css_syntax::CssBogusKeyframesItem>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssBogusKeyframesItem {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssBogusKeyframesItem,
        crate::css::bogus::bogus_keyframes_item::FormatCssBogusKeyframesItem,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::bogus::bogus_keyframes_item::FormatCssBogusKeyframesItem::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssBogusKeyframesItem {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssBogusKeyframesItem,
        crate::css::bogus::bogus_keyframes_item::FormatCssBogusKeyframesItem,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::bogus::bogus_keyframes_item::FormatCssBogusKeyframesItem::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssBogusRule>
    for crate::css::bogus::bogus_rule::FormatCssBogusRule
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_css_syntax::CssBogusRule, f: &mut CssFormatter) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_css_syntax::CssBogusRule>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssBogusRule {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssBogusRule,
        crate::css::bogus::bogus_rule::FormatCssBogusRule,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::bogus::bogus_rule::FormatCssBogusRule::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssBogusRule {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssBogusRule,
        crate::css::bogus::bogus_rule::FormatCssBogusRule,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::bogus::bogus_rule::FormatCssBogusRule::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssBogusParameter>
    for crate::css::bogus::bogus_parameter::FormatCssBogusParameter
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssBogusParameter,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_css_syntax::CssBogusParameter>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssBogusParameter {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssBogusParameter,
        crate::css::bogus::bogus_parameter::FormatCssBogusParameter,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::bogus::bogus_parameter::FormatCssBogusParameter::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssBogusParameter {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssBogusParameter,
        crate::css::bogus::bogus_parameter::FormatCssBogusParameter,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::bogus::bogus_parameter::FormatCssBogusParameter::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssBogusDeclarationItem>
    for crate::css::bogus::bogus_declaration_item::FormatCssBogusDeclarationItem
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssBogusDeclarationItem,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_css_syntax::CssBogusDeclarationItem>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssBogusDeclarationItem {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssBogusDeclarationItem,
        crate::css::bogus::bogus_declaration_item::FormatCssBogusDeclarationItem,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::bogus::bogus_declaration_item::FormatCssBogusDeclarationItem::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssBogusDeclarationItem {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssBogusDeclarationItem,
        crate::css::bogus::bogus_declaration_item::FormatCssBogusDeclarationItem,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::bogus::bogus_declaration_item::FormatCssBogusDeclarationItem::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssBogusMediaQuery>
    for crate::css::bogus::bogus_media_query::FormatCssBogusMediaQuery
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssBogusMediaQuery,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_css_syntax::CssBogusMediaQuery>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssBogusMediaQuery {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssBogusMediaQuery,
        crate::css::bogus::bogus_media_query::FormatCssBogusMediaQuery,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::bogus::bogus_media_query::FormatCssBogusMediaQuery::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssBogusMediaQuery {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssBogusMediaQuery,
        crate::css::bogus::bogus_media_query::FormatCssBogusMediaQuery,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::bogus::bogus_media_query::FormatCssBogusMediaQuery::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssBogusProperty>
    for crate::css::bogus::bogus_property::FormatCssBogusProperty
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssBogusProperty,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_css_syntax::CssBogusProperty>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssBogusProperty {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssBogusProperty,
        crate::css::bogus::bogus_property::FormatCssBogusProperty,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::bogus::bogus_property::FormatCssBogusProperty::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssBogusProperty {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssBogusProperty,
        crate::css::bogus::bogus_property::FormatCssBogusProperty,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::bogus::bogus_property::FormatCssBogusProperty::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssBogusUrlModifier>
    for crate::css::bogus::bogus_url_modifier::FormatCssBogusUrlModifier
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssBogusUrlModifier,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_css_syntax::CssBogusUrlModifier>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssBogusUrlModifier {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssBogusUrlModifier,
        crate::css::bogus::bogus_url_modifier::FormatCssBogusUrlModifier,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::bogus::bogus_url_modifier::FormatCssBogusUrlModifier::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssBogusUrlModifier {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssBogusUrlModifier,
        crate::css::bogus::bogus_url_modifier::FormatCssBogusUrlModifier,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::bogus::bogus_url_modifier::FormatCssBogusUrlModifier::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssBogusPropertyValue>
    for crate::css::bogus::bogus_property_value::FormatCssBogusPropertyValue
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssBogusPropertyValue,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_css_syntax::CssBogusPropertyValue>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssBogusPropertyValue {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssBogusPropertyValue,
        crate::css::bogus::bogus_property_value::FormatCssBogusPropertyValue,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::bogus::bogus_property_value::FormatCssBogusPropertyValue::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssBogusPropertyValue {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssBogusPropertyValue,
        crate::css::bogus::bogus_property_value::FormatCssBogusPropertyValue,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::bogus::bogus_property_value::FormatCssBogusPropertyValue::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssBogusDocumentMatcher>
    for crate::css::bogus::bogus_document_matcher::FormatCssBogusDocumentMatcher
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssBogusDocumentMatcher,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_css_syntax::CssBogusDocumentMatcher>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssBogusDocumentMatcher {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssBogusDocumentMatcher,
        crate::css::bogus::bogus_document_matcher::FormatCssBogusDocumentMatcher,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::bogus::bogus_document_matcher::FormatCssBogusDocumentMatcher::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssBogusDocumentMatcher {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssBogusDocumentMatcher,
        crate::css::bogus::bogus_document_matcher::FormatCssBogusDocumentMatcher,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::bogus::bogus_document_matcher::FormatCssBogusDocumentMatcher::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssRule {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssRule,
        crate::css::any::rule::FormatAnyCssRule,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(self, crate::css::any::rule::FormatAnyCssRule::default())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssRule {
    type Format =
        FormatOwnedWithRule<biome_css_syntax::AnyCssRule, crate::css::any::rule::FormatAnyCssRule>;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(self, crate::css::any::rule::FormatAnyCssRule::default())
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssDeclarationOrRuleBlock {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssDeclarationOrRuleBlock,
        crate::css::any::declaration_or_rule_block::FormatAnyCssDeclarationOrRuleBlock,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::declaration_or_rule_block::FormatAnyCssDeclarationOrRuleBlock::default(
            ),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssDeclarationOrRuleBlock {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssDeclarationOrRuleBlock,
        crate::css::any::declaration_or_rule_block::FormatAnyCssDeclarationOrRuleBlock,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::declaration_or_rule_block::FormatAnyCssDeclarationOrRuleBlock::default(
            ),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssSelector {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssSelector,
        crate::css::any::selector::FormatAnyCssSelector,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::selector::FormatAnyCssSelector::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssSelector {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssSelector,
        crate::css::any::selector::FormatAnyCssSelector,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::selector::FormatAnyCssSelector::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssSimpleSelector {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssSimpleSelector,
        crate::css::any::simple_selector::FormatAnyCssSimpleSelector,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::simple_selector::FormatAnyCssSimpleSelector::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssSimpleSelector {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssSimpleSelector,
        crate::css::any::simple_selector::FormatAnyCssSimpleSelector,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::simple_selector::FormatAnyCssSimpleSelector::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssSubSelector {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssSubSelector,
        crate::css::any::sub_selector::FormatAnyCssSubSelector,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::sub_selector::FormatAnyCssSubSelector::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssSubSelector {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssSubSelector,
        crate::css::any::sub_selector::FormatAnyCssSubSelector,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::sub_selector::FormatAnyCssSubSelector::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssNamespacePrefix {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssNamespacePrefix,
        crate::css::any::namespace_prefix::FormatAnyCssNamespacePrefix,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::namespace_prefix::FormatAnyCssNamespacePrefix::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssNamespacePrefix {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssNamespacePrefix,
        crate::css::any::namespace_prefix::FormatAnyCssNamespacePrefix,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::namespace_prefix::FormatAnyCssNamespacePrefix::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssAttributeMatcherValue {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssAttributeMatcherValue,
        crate::css::any::attribute_matcher_value::FormatAnyCssAttributeMatcherValue,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::attribute_matcher_value::FormatAnyCssAttributeMatcherValue::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssAttributeMatcherValue {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssAttributeMatcherValue,
        crate::css::any::attribute_matcher_value::FormatAnyCssAttributeMatcherValue,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::attribute_matcher_value::FormatAnyCssAttributeMatcherValue::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssPseudoClass {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssPseudoClass,
        crate::css::any::pseudo_class::FormatAnyCssPseudoClass,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::pseudo_class::FormatAnyCssPseudoClass::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssPseudoClass {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssPseudoClass,
        crate::css::any::pseudo_class::FormatAnyCssPseudoClass,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::pseudo_class::FormatAnyCssPseudoClass::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssCompoundSelector {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssCompoundSelector,
        crate::css::any::compound_selector::FormatAnyCssCompoundSelector,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::compound_selector::FormatAnyCssCompoundSelector::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssCompoundSelector {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssCompoundSelector,
        crate::css::any::compound_selector::FormatAnyCssCompoundSelector,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::compound_selector::FormatAnyCssCompoundSelector::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssRelativeSelector {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssRelativeSelector,
        crate::css::any::relative_selector::FormatAnyCssRelativeSelector,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::relative_selector::FormatAnyCssRelativeSelector::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssRelativeSelector {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssRelativeSelector,
        crate::css::any::relative_selector::FormatAnyCssRelativeSelector,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::relative_selector::FormatAnyCssRelativeSelector::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssPseudoValue {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssPseudoValue,
        crate::css::any::pseudo_value::FormatAnyCssPseudoValue,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::pseudo_value::FormatAnyCssPseudoValue::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssPseudoValue {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssPseudoValue,
        crate::css::any::pseudo_value::FormatAnyCssPseudoValue,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::pseudo_value::FormatAnyCssPseudoValue::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssPseudoClassNthSelector {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssPseudoClassNthSelector,
        crate::css::any::pseudo_class_nth_selector::FormatAnyCssPseudoClassNthSelector,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::pseudo_class_nth_selector::FormatAnyCssPseudoClassNthSelector::default(
            ),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssPseudoClassNthSelector {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssPseudoClassNthSelector,
        crate::css::any::pseudo_class_nth_selector::FormatAnyCssPseudoClassNthSelector,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::pseudo_class_nth_selector::FormatAnyCssPseudoClassNthSelector::default(
            ),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssPseudoClassNth {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssPseudoClassNth,
        crate::css::any::pseudo_class_nth::FormatAnyCssPseudoClassNth,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::pseudo_class_nth::FormatAnyCssPseudoClassNth::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssPseudoClassNth {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssPseudoClassNth,
        crate::css::any::pseudo_class_nth::FormatAnyCssPseudoClassNth,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::pseudo_class_nth::FormatAnyCssPseudoClassNth::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssPseudoElement {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssPseudoElement,
        crate::css::any::pseudo_element::FormatAnyCssPseudoElement,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::pseudo_element::FormatAnyCssPseudoElement::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssPseudoElement {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssPseudoElement,
        crate::css::any::pseudo_element::FormatAnyCssPseudoElement,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::pseudo_element::FormatAnyCssPseudoElement::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssDeclarationOrRule {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssDeclarationOrRule,
        crate::css::any::declaration_or_rule::FormatAnyCssDeclarationOrRule,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::declaration_or_rule::FormatAnyCssDeclarationOrRule::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssDeclarationOrRule {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssDeclarationOrRule,
        crate::css::any::declaration_or_rule::FormatAnyCssDeclarationOrRule,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::declaration_or_rule::FormatAnyCssDeclarationOrRule::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssDeclarationOrAtRuleBlock {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssDeclarationOrAtRuleBlock,
        crate::css::any::declaration_or_at_rule_block::FormatAnyCssDeclarationOrAtRuleBlock,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule :: new (self , crate :: css :: any :: declaration_or_at_rule_block :: FormatAnyCssDeclarationOrAtRuleBlock :: default ())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssDeclarationOrAtRuleBlock {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssDeclarationOrAtRuleBlock,
        crate::css::any::declaration_or_at_rule_block::FormatAnyCssDeclarationOrAtRuleBlock,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule :: new (self , crate :: css :: any :: declaration_or_at_rule_block :: FormatAnyCssDeclarationOrAtRuleBlock :: default ())
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssDeclarationOrAtRule {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssDeclarationOrAtRule,
        crate::css::any::declaration_or_at_rule::FormatAnyCssDeclarationOrAtRule,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::declaration_or_at_rule::FormatAnyCssDeclarationOrAtRule::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssDeclarationOrAtRule {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssDeclarationOrAtRule,
        crate::css::any::declaration_or_at_rule::FormatAnyCssDeclarationOrAtRule,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::declaration_or_at_rule::FormatAnyCssDeclarationOrAtRule::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssDeclarationListBlock {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssDeclarationListBlock,
        crate::css::any::declaration_list_block::FormatAnyCssDeclarationListBlock,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::declaration_list_block::FormatAnyCssDeclarationListBlock::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssDeclarationListBlock {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssDeclarationListBlock,
        crate::css::any::declaration_list_block::FormatAnyCssDeclarationListBlock,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::declaration_list_block::FormatAnyCssDeclarationListBlock::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssRuleListBlock {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssRuleListBlock,
        crate::css::any::rule_list_block::FormatAnyCssRuleListBlock,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::rule_list_block::FormatAnyCssRuleListBlock::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssRuleListBlock {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssRuleListBlock,
        crate::css::any::rule_list_block::FormatAnyCssRuleListBlock,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::rule_list_block::FormatAnyCssRuleListBlock::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssProperty {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssProperty,
        crate::css::any::property::FormatAnyCssProperty,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::property::FormatAnyCssProperty::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssProperty {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssProperty,
        crate::css::any::property::FormatAnyCssProperty,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::property::FormatAnyCssProperty::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssAllPropertyValue {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssAllPropertyValue,
        crate::css::any::all_property_value::FormatAnyCssAllPropertyValue,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::all_property_value::FormatAnyCssAllPropertyValue::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssAllPropertyValue {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssAllPropertyValue,
        crate::css::any::all_property_value::FormatAnyCssAllPropertyValue,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::all_property_value::FormatAnyCssAllPropertyValue::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssBorderPropertyValue {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssBorderPropertyValue,
        crate::css::any::border_property_value::FormatAnyCssBorderPropertyValue,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::border_property_value::FormatAnyCssBorderPropertyValue::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssBorderPropertyValue {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssBorderPropertyValue,
        crate::css::any::border_property_value::FormatAnyCssBorderPropertyValue,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::border_property_value::FormatAnyCssBorderPropertyValue::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssLineWidth {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssLineWidth,
        crate::css::any::line_width::FormatAnyCssLineWidth,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::line_width::FormatAnyCssLineWidth::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssLineWidth {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssLineWidth,
        crate::css::any::line_width::FormatAnyCssLineWidth,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::line_width::FormatAnyCssLineWidth::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssZIndexPropertyValue {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssZIndexPropertyValue,
        crate::css::any::z_index_property_value::FormatAnyCssZIndexPropertyValue,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::z_index_property_value::FormatAnyCssZIndexPropertyValue::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssZIndexPropertyValue {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssZIndexPropertyValue,
        crate::css::any::z_index_property_value::FormatAnyCssZIndexPropertyValue,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::z_index_property_value::FormatAnyCssZIndexPropertyValue::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssDeclarationName {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssDeclarationName,
        crate::css::any::declaration_name::FormatAnyCssDeclarationName,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::declaration_name::FormatAnyCssDeclarationName::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssDeclarationName {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssDeclarationName,
        crate::css::any::declaration_name::FormatAnyCssDeclarationName,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::declaration_name::FormatAnyCssDeclarationName::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssGenericComponentValue {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssGenericComponentValue,
        crate::css::any::generic_component_value::FormatAnyCssGenericComponentValue,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::generic_component_value::FormatAnyCssGenericComponentValue::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssGenericComponentValue {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssGenericComponentValue,
        crate::css::any::generic_component_value::FormatAnyCssGenericComponentValue,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::generic_component_value::FormatAnyCssGenericComponentValue::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssValue {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssValue,
        crate::css::any::value::FormatAnyCssValue,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(self, crate::css::any::value::FormatAnyCssValue::default())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssValue {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssValue,
        crate::css::any::value::FormatAnyCssValue,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(self, crate::css::any::value::FormatAnyCssValue::default())
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssAtRule {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssAtRule,
        crate::css::any::at_rule::FormatAnyCssAtRule,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::at_rule::FormatAnyCssAtRule::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssAtRule {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssAtRule,
        crate::css::any::at_rule::FormatAnyCssAtRule,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::at_rule::FormatAnyCssAtRule::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssFontFamilyName {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssFontFamilyName,
        crate::css::any::font_family_name::FormatAnyCssFontFamilyName,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::font_family_name::FormatAnyCssFontFamilyName::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssFontFamilyName {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssFontFamilyName,
        crate::css::any::font_family_name::FormatAnyCssFontFamilyName,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::font_family_name::FormatAnyCssFontFamilyName::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssFontFeatureValuesBlock {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssFontFeatureValuesBlock,
        crate::css::any::font_feature_values_block::FormatAnyCssFontFeatureValuesBlock,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::font_feature_values_block::FormatAnyCssFontFeatureValuesBlock::default(
            ),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssFontFeatureValuesBlock {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssFontFeatureValuesBlock,
        crate::css::any::font_feature_values_block::FormatAnyCssFontFeatureValuesBlock,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::font_feature_values_block::FormatAnyCssFontFeatureValuesBlock::default(
            ),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssFontFeatureValuesItem {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssFontFeatureValuesItem,
        crate::css::any::font_feature_values_item::FormatAnyCssFontFeatureValuesItem,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::font_feature_values_item::FormatAnyCssFontFeatureValuesItem::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssFontFeatureValuesItem {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssFontFeatureValuesItem,
        crate::css::any::font_feature_values_item::FormatAnyCssFontFeatureValuesItem,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::font_feature_values_item::FormatAnyCssFontFeatureValuesItem::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssContainerQuery {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssContainerQuery,
        crate::css::any::container_query::FormatAnyCssContainerQuery,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::container_query::FormatAnyCssContainerQuery::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssContainerQuery {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssContainerQuery,
        crate::css::any::container_query::FormatAnyCssContainerQuery,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::container_query::FormatAnyCssContainerQuery::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssContainerQueryInParens {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssContainerQueryInParens,
        crate::css::any::container_query_in_parens::FormatAnyCssContainerQueryInParens,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::container_query_in_parens::FormatAnyCssContainerQueryInParens::default(
            ),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssContainerQueryInParens {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssContainerQueryInParens,
        crate::css::any::container_query_in_parens::FormatAnyCssContainerQueryInParens,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::container_query_in_parens::FormatAnyCssContainerQueryInParens::default(
            ),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssContainerAndCombinableQuery {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssContainerAndCombinableQuery,
        crate::css::any::container_and_combinable_query::FormatAnyCssContainerAndCombinableQuery,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule :: new (self , crate :: css :: any :: container_and_combinable_query :: FormatAnyCssContainerAndCombinableQuery :: default ())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssContainerAndCombinableQuery {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssContainerAndCombinableQuery,
        crate::css::any::container_and_combinable_query::FormatAnyCssContainerAndCombinableQuery,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule :: new (self , crate :: css :: any :: container_and_combinable_query :: FormatAnyCssContainerAndCombinableQuery :: default ())
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssContainerOrCombinableQuery {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssContainerOrCombinableQuery,
        crate::css::any::container_or_combinable_query::FormatAnyCssContainerOrCombinableQuery,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule :: new (self , crate :: css :: any :: container_or_combinable_query :: FormatAnyCssContainerOrCombinableQuery :: default ())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssContainerOrCombinableQuery {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssContainerOrCombinableQuery,
        crate::css::any::container_or_combinable_query::FormatAnyCssContainerOrCombinableQuery,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule :: new (self , crate :: css :: any :: container_or_combinable_query :: FormatAnyCssContainerOrCombinableQuery :: default ())
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssQueryFeature {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssQueryFeature,
        crate::css::any::query_feature::FormatAnyCssQueryFeature,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::query_feature::FormatAnyCssQueryFeature::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssQueryFeature {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssQueryFeature,
        crate::css::any::query_feature::FormatAnyCssQueryFeature,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::query_feature::FormatAnyCssQueryFeature::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssContainerStyleQuery {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssContainerStyleQuery,
        crate::css::any::container_style_query::FormatAnyCssContainerStyleQuery,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::container_style_query::FormatAnyCssContainerStyleQuery::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssContainerStyleQuery {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssContainerStyleQuery,
        crate::css::any::container_style_query::FormatAnyCssContainerStyleQuery,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::container_style_query::FormatAnyCssContainerStyleQuery::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssContainerStyleAndCombinableQuery {
    type Format < 'a > = FormatRefWithRule < 'a , biome_css_syntax :: AnyCssContainerStyleAndCombinableQuery , crate :: css :: any :: container_style_and_combinable_query :: FormatAnyCssContainerStyleAndCombinableQuery > ;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule :: new (self , crate :: css :: any :: container_style_and_combinable_query :: FormatAnyCssContainerStyleAndCombinableQuery :: default ())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssContainerStyleAndCombinableQuery {
    type Format = FormatOwnedWithRule < biome_css_syntax :: AnyCssContainerStyleAndCombinableQuery , crate :: css :: any :: container_style_and_combinable_query :: FormatAnyCssContainerStyleAndCombinableQuery > ;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule :: new (self , crate :: css :: any :: container_style_and_combinable_query :: FormatAnyCssContainerStyleAndCombinableQuery :: default ())
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssContainerStyleOrCombinableQuery {
    type Format < 'a > = FormatRefWithRule < 'a , biome_css_syntax :: AnyCssContainerStyleOrCombinableQuery , crate :: css :: any :: container_style_or_combinable_query :: FormatAnyCssContainerStyleOrCombinableQuery > ;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule :: new (self , crate :: css :: any :: container_style_or_combinable_query :: FormatAnyCssContainerStyleOrCombinableQuery :: default ())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssContainerStyleOrCombinableQuery {
    type Format = FormatOwnedWithRule < biome_css_syntax :: AnyCssContainerStyleOrCombinableQuery , crate :: css :: any :: container_style_or_combinable_query :: FormatAnyCssContainerStyleOrCombinableQuery > ;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule :: new (self , crate :: css :: any :: container_style_or_combinable_query :: FormatAnyCssContainerStyleOrCombinableQuery :: default ())
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssContainerStyleInParens {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssContainerStyleInParens,
        crate::css::any::container_style_in_parens::FormatAnyCssContainerStyleInParens,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::container_style_in_parens::FormatAnyCssContainerStyleInParens::default(
            ),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssContainerStyleInParens {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssContainerStyleInParens,
        crate::css::any::container_style_in_parens::FormatAnyCssContainerStyleInParens,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::container_style_in_parens::FormatAnyCssContainerStyleInParens::default(
            ),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssKeyframeName {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssKeyframeName,
        crate::css::any::keyframe_name::FormatAnyCssKeyframeName,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::keyframe_name::FormatAnyCssKeyframeName::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssKeyframeName {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssKeyframeName,
        crate::css::any::keyframe_name::FormatAnyCssKeyframeName,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::keyframe_name::FormatAnyCssKeyframeName::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssKeyframesBlock {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssKeyframesBlock,
        crate::css::any::keyframes_block::FormatAnyCssKeyframesBlock,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::keyframes_block::FormatAnyCssKeyframesBlock::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssKeyframesBlock {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssKeyframesBlock,
        crate::css::any::keyframes_block::FormatAnyCssKeyframesBlock,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::keyframes_block::FormatAnyCssKeyframesBlock::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssKeyframesItem {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssKeyframesItem,
        crate::css::any::keyframes_item::FormatAnyCssKeyframesItem,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::keyframes_item::FormatAnyCssKeyframesItem::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssKeyframesItem {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssKeyframesItem,
        crate::css::any::keyframes_item::FormatAnyCssKeyframesItem,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::keyframes_item::FormatAnyCssKeyframesItem::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssKeyframesSelector {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssKeyframesSelector,
        crate::css::any::keyframes_selector::FormatAnyCssKeyframesSelector,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::keyframes_selector::FormatAnyCssKeyframesSelector::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssKeyframesSelector {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssKeyframesSelector,
        crate::css::any::keyframes_selector::FormatAnyCssKeyframesSelector,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::keyframes_selector::FormatAnyCssKeyframesSelector::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssMediaQuery {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssMediaQuery,
        crate::css::any::media_query::FormatAnyCssMediaQuery,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::media_query::FormatAnyCssMediaQuery::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssMediaQuery {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssMediaQuery,
        crate::css::any::media_query::FormatAnyCssMediaQuery,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::media_query::FormatAnyCssMediaQuery::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssMediaTypeQuery {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssMediaTypeQuery,
        crate::css::any::media_type_query::FormatAnyCssMediaTypeQuery,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::media_type_query::FormatAnyCssMediaTypeQuery::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssMediaTypeQuery {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssMediaTypeQuery,
        crate::css::any::media_type_query::FormatAnyCssMediaTypeQuery,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::media_type_query::FormatAnyCssMediaTypeQuery::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssMediaCondition {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssMediaCondition,
        crate::css::any::media_condition::FormatAnyCssMediaCondition,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::media_condition::FormatAnyCssMediaCondition::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssMediaCondition {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssMediaCondition,
        crate::css::any::media_condition::FormatAnyCssMediaCondition,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::media_condition::FormatAnyCssMediaCondition::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssMediaTypeCondition {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssMediaTypeCondition,
        crate::css::any::media_type_condition::FormatAnyCssMediaTypeCondition,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::media_type_condition::FormatAnyCssMediaTypeCondition::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssMediaTypeCondition {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssMediaTypeCondition,
        crate::css::any::media_type_condition::FormatAnyCssMediaTypeCondition,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::media_type_condition::FormatAnyCssMediaTypeCondition::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssMediaInParens {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssMediaInParens,
        crate::css::any::media_in_parens::FormatAnyCssMediaInParens,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::media_in_parens::FormatAnyCssMediaInParens::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssMediaInParens {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssMediaInParens,
        crate::css::any::media_in_parens::FormatAnyCssMediaInParens,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::media_in_parens::FormatAnyCssMediaInParens::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssMediaOrCombinableCondition {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssMediaOrCombinableCondition,
        crate::css::any::media_or_combinable_condition::FormatAnyCssMediaOrCombinableCondition,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule :: new (self , crate :: css :: any :: media_or_combinable_condition :: FormatAnyCssMediaOrCombinableCondition :: default ())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssMediaOrCombinableCondition {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssMediaOrCombinableCondition,
        crate::css::any::media_or_combinable_condition::FormatAnyCssMediaOrCombinableCondition,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule :: new (self , crate :: css :: any :: media_or_combinable_condition :: FormatAnyCssMediaOrCombinableCondition :: default ())
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssMediaAndCombinableCondition {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssMediaAndCombinableCondition,
        crate::css::any::media_and_combinable_condition::FormatAnyCssMediaAndCombinableCondition,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule :: new (self , crate :: css :: any :: media_and_combinable_condition :: FormatAnyCssMediaAndCombinableCondition :: default ())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssMediaAndCombinableCondition {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssMediaAndCombinableCondition,
        crate::css::any::media_and_combinable_condition::FormatAnyCssMediaAndCombinableCondition,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule :: new (self , crate :: css :: any :: media_and_combinable_condition :: FormatAnyCssMediaAndCombinableCondition :: default ())
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssQueryFeatureValue {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssQueryFeatureValue,
        crate::css::any::query_feature_value::FormatAnyCssQueryFeatureValue,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::query_feature_value::FormatAnyCssQueryFeatureValue::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssQueryFeatureValue {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssQueryFeatureValue,
        crate::css::any::query_feature_value::FormatAnyCssQueryFeatureValue,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::query_feature_value::FormatAnyCssQueryFeatureValue::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssDimension {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssDimension,
        crate::css::any::dimension::FormatAnyCssDimension,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::dimension::FormatAnyCssDimension::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssDimension {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssDimension,
        crate::css::any::dimension::FormatAnyCssDimension,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::dimension::FormatAnyCssDimension::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssFunction {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssFunction,
        crate::css::any::function::FormatAnyCssFunction,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::function::FormatAnyCssFunction::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssFunction {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssFunction,
        crate::css::any::function::FormatAnyCssFunction,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::function::FormatAnyCssFunction::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssPageAtRuleBlock {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssPageAtRuleBlock,
        crate::css::any::page_at_rule_block::FormatAnyCssPageAtRuleBlock,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::page_at_rule_block::FormatAnyCssPageAtRuleBlock::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssPageAtRuleBlock {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssPageAtRuleBlock,
        crate::css::any::page_at_rule_block::FormatAnyCssPageAtRuleBlock,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::page_at_rule_block::FormatAnyCssPageAtRuleBlock::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssPageSelector {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssPageSelector,
        crate::css::any::page_selector::FormatAnyCssPageSelector,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::page_selector::FormatAnyCssPageSelector::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssPageSelector {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssPageSelector,
        crate::css::any::page_selector::FormatAnyCssPageSelector,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::page_selector::FormatAnyCssPageSelector::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssPageSelectorPseudo {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssPageSelectorPseudo,
        crate::css::any::page_selector_pseudo::FormatAnyCssPageSelectorPseudo,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::page_selector_pseudo::FormatAnyCssPageSelectorPseudo::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssPageSelectorPseudo {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssPageSelectorPseudo,
        crate::css::any::page_selector_pseudo::FormatAnyCssPageSelectorPseudo,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::page_selector_pseudo::FormatAnyCssPageSelectorPseudo::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssPageAtRuleItem {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssPageAtRuleItem,
        crate::css::any::page_at_rule_item::FormatAnyCssPageAtRuleItem,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::page_at_rule_item::FormatAnyCssPageAtRuleItem::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssPageAtRuleItem {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssPageAtRuleItem,
        crate::css::any::page_at_rule_item::FormatAnyCssPageAtRuleItem,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::page_at_rule_item::FormatAnyCssPageAtRuleItem::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssLayer {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssLayer,
        crate::css::any::layer::FormatAnyCssLayer,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(self, crate::css::any::layer::FormatAnyCssLayer::default())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssLayer {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssLayer,
        crate::css::any::layer::FormatAnyCssLayer,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(self, crate::css::any::layer::FormatAnyCssLayer::default())
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssSupportsCondition {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssSupportsCondition,
        crate::css::any::supports_condition::FormatAnyCssSupportsCondition,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::supports_condition::FormatAnyCssSupportsCondition::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssSupportsCondition {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssSupportsCondition,
        crate::css::any::supports_condition::FormatAnyCssSupportsCondition,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::supports_condition::FormatAnyCssSupportsCondition::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssSupportsInParens {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssSupportsInParens,
        crate::css::any::supports_in_parens::FormatAnyCssSupportsInParens,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::supports_in_parens::FormatAnyCssSupportsInParens::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssSupportsInParens {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssSupportsInParens,
        crate::css::any::supports_in_parens::FormatAnyCssSupportsInParens,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::supports_in_parens::FormatAnyCssSupportsInParens::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssSupportsAndCombinableCondition {
    type Format < 'a > = FormatRefWithRule < 'a , biome_css_syntax :: AnyCssSupportsAndCombinableCondition , crate :: css :: any :: supports_and_combinable_condition :: FormatAnyCssSupportsAndCombinableCondition > ;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule :: new (self , crate :: css :: any :: supports_and_combinable_condition :: FormatAnyCssSupportsAndCombinableCondition :: default ())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssSupportsAndCombinableCondition {
    type Format = FormatOwnedWithRule < biome_css_syntax :: AnyCssSupportsAndCombinableCondition , crate :: css :: any :: supports_and_combinable_condition :: FormatAnyCssSupportsAndCombinableCondition > ;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule :: new (self , crate :: css :: any :: supports_and_combinable_condition :: FormatAnyCssSupportsAndCombinableCondition :: default ())
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssSupportsOrCombinableCondition {
    type Format < 'a > = FormatRefWithRule < 'a , biome_css_syntax :: AnyCssSupportsOrCombinableCondition , crate :: css :: any :: supports_or_combinable_condition :: FormatAnyCssSupportsOrCombinableCondition > ;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule :: new (self , crate :: css :: any :: supports_or_combinable_condition :: FormatAnyCssSupportsOrCombinableCondition :: default ())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssSupportsOrCombinableCondition {
    type Format = FormatOwnedWithRule < biome_css_syntax :: AnyCssSupportsOrCombinableCondition , crate :: css :: any :: supports_or_combinable_condition :: FormatAnyCssSupportsOrCombinableCondition > ;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule :: new (self , crate :: css :: any :: supports_or_combinable_condition :: FormatAnyCssSupportsOrCombinableCondition :: default ())
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssScopeRange {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssScopeRange,
        crate::css::any::scope_range::FormatAnyCssScopeRange,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::scope_range::FormatAnyCssScopeRange::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssScopeRange {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssScopeRange,
        crate::css::any::scope_range::FormatAnyCssScopeRange,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::scope_range::FormatAnyCssScopeRange::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssImportUrl {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssImportUrl,
        crate::css::any::import_url::FormatAnyCssImportUrl,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::import_url::FormatAnyCssImportUrl::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssImportUrl {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssImportUrl,
        crate::css::any::import_url::FormatAnyCssImportUrl,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::import_url::FormatAnyCssImportUrl::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssImportLayer {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssImportLayer,
        crate::css::any::import_layer::FormatAnyCssImportLayer,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::import_layer::FormatAnyCssImportLayer::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssImportLayer {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssImportLayer,
        crate::css::any::import_layer::FormatAnyCssImportLayer,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::import_layer::FormatAnyCssImportLayer::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssImportSupportsCondition {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssImportSupportsCondition,
        crate::css::any::import_supports_condition::FormatAnyCssImportSupportsCondition,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule :: new (self , crate :: css :: any :: import_supports_condition :: FormatAnyCssImportSupportsCondition :: default ())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssImportSupportsCondition {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssImportSupportsCondition,
        crate::css::any::import_supports_condition::FormatAnyCssImportSupportsCondition,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule :: new (self , crate :: css :: any :: import_supports_condition :: FormatAnyCssImportSupportsCondition :: default ())
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssNamespaceUrl {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssNamespaceUrl,
        crate::css::any::namespace_url::FormatAnyCssNamespaceUrl,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::namespace_url::FormatAnyCssNamespaceUrl::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssNamespaceUrl {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssNamespaceUrl,
        crate::css::any::namespace_url::FormatAnyCssNamespaceUrl,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::namespace_url::FormatAnyCssNamespaceUrl::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssStartingStyleBlock {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssStartingStyleBlock,
        crate::css::any::starting_style_block::FormatAnyCssStartingStyleBlock,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::starting_style_block::FormatAnyCssStartingStyleBlock::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssStartingStyleBlock {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssStartingStyleBlock,
        crate::css::any::starting_style_block::FormatAnyCssStartingStyleBlock,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::starting_style_block::FormatAnyCssStartingStyleBlock::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssDocumentMatcher {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssDocumentMatcher,
        crate::css::any::document_matcher::FormatAnyCssDocumentMatcher,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::document_matcher::FormatAnyCssDocumentMatcher::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssDocumentMatcher {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssDocumentMatcher,
        crate::css::any::document_matcher::FormatAnyCssDocumentMatcher,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::document_matcher::FormatAnyCssDocumentMatcher::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssUrlValue {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssUrlValue,
        crate::css::any::url_value::FormatAnyCssUrlValue,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::url_value::FormatAnyCssUrlValue::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssUrlValue {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssUrlValue,
        crate::css::any::url_value::FormatAnyCssUrlValue,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::url_value::FormatAnyCssUrlValue::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssUrlModifier {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssUrlModifier,
        crate::css::any::url_modifier::FormatAnyCssUrlModifier,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::url_modifier::FormatAnyCssUrlModifier::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssUrlModifier {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssUrlModifier,
        crate::css::any::url_modifier::FormatAnyCssUrlModifier,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::url_modifier::FormatAnyCssUrlModifier::default(),
        )
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::AnyCssExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::AnyCssExpression,
        crate::css::any::expression::FormatAnyCssExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::any::expression::FormatAnyCssExpression::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::AnyCssExpression {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::AnyCssExpression,
        crate::css::any::expression::FormatAnyCssExpression,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::any::expression::FormatAnyCssExpression::default(),
        )
    }
}
