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
impl FormatRule<biome_css_syntax::CssRule> for crate::css::auxiliary::rule::FormatCssRule {
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_css_syntax::CssRule, f: &mut CssFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssRule>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssRule {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssRule,
        crate::css::auxiliary::rule::FormatCssRule,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(self, crate::css::auxiliary::rule::FormatCssRule::default())
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssRule {
    type Format =
        FormatOwnedWithRule<biome_css_syntax::CssRule, crate::css::auxiliary::rule::FormatCssRule>;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(self, crate::css::auxiliary::rule::FormatCssRule::default())
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
impl FormatRule<biome_css_syntax::CssBlock> for crate::css::auxiliary::block::FormatCssBlock {
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_css_syntax::CssBlock, f: &mut CssFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssBlock>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssBlock {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssBlock,
        crate::css::auxiliary::block::FormatCssBlock,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::block::FormatCssBlock::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssBlock {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssBlock,
        crate::css::auxiliary::block::FormatCssBlock,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::block::FormatCssBlock::default(),
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
    for crate::css::auxiliary::identifier::FormatCssIdentifier
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
        crate::css::auxiliary::identifier::FormatCssIdentifier,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::identifier::FormatCssIdentifier::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssIdentifier {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssIdentifier,
        crate::css::auxiliary::identifier::FormatCssIdentifier,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::identifier::FormatCssIdentifier::default(),
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
impl FormatRule<biome_css_syntax::CssCustomProperty>
    for crate::css::auxiliary::custom_property::FormatCssCustomProperty
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssCustomProperty,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssCustomProperty>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssCustomProperty {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssCustomProperty,
        crate::css::auxiliary::custom_property::FormatCssCustomProperty,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::custom_property::FormatCssCustomProperty::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssCustomProperty {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssCustomProperty,
        crate::css::auxiliary::custom_property::FormatCssCustomProperty,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::custom_property::FormatCssCustomProperty::default(),
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
impl FormatRule<biome_css_syntax::CssKeyframesBody>
    for crate::css::auxiliary::keyframes_body::FormatCssKeyframesBody
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssKeyframesBody,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssKeyframesBody>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssKeyframesBody {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssKeyframesBody,
        crate::css::auxiliary::keyframes_body::FormatCssKeyframesBody,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::keyframes_body::FormatCssKeyframesBody::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssKeyframesBody {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssKeyframesBody,
        crate::css::auxiliary::keyframes_body::FormatCssKeyframesBody,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::keyframes_body::FormatCssKeyframesBody::default(),
        )
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
impl FormatRule<biome_css_syntax::CssKeyframesSelector>
    for crate::css::selectors::keyframes_selector::FormatCssKeyframesSelector
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssKeyframesSelector,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssKeyframesSelector>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssKeyframesSelector {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssKeyframesSelector,
        crate::css::selectors::keyframes_selector::FormatCssKeyframesSelector,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::selectors::keyframes_selector::FormatCssKeyframesSelector::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssKeyframesSelector {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssKeyframesSelector,
        crate::css::selectors::keyframes_selector::FormatCssKeyframesSelector,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::selectors::keyframes_selector::FormatCssKeyframesSelector::default(),
        )
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
impl FormatRule<biome_css_syntax::CssAnyFunction>
    for crate::css::auxiliary::any_function::FormatCssAnyFunction
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssAnyFunction,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssAnyFunction>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssAnyFunction {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssAnyFunction,
        crate::css::auxiliary::any_function::FormatCssAnyFunction,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::any_function::FormatCssAnyFunction::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssAnyFunction {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssAnyFunction,
        crate::css::auxiliary::any_function::FormatCssAnyFunction,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::any_function::FormatCssAnyFunction::default(),
        )
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
impl FormatRule<biome_css_syntax::CssPercentDimension>
    for crate::css::value::percent_dimension::FormatCssPercentDimension
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssPercentDimension,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssPercentDimension>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssPercentDimension {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssPercentDimension,
        crate::css::value::percent_dimension::FormatCssPercentDimension,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::value::percent_dimension::FormatCssPercentDimension::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssPercentDimension {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssPercentDimension,
        crate::css::value::percent_dimension::FormatCssPercentDimension,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::value::percent_dimension::FormatCssPercentDimension::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssSimpleFunction>
    for crate::css::auxiliary::simple_function::FormatCssSimpleFunction
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssSimpleFunction,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssSimpleFunction>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssSimpleFunction {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssSimpleFunction,
        crate::css::auxiliary::simple_function::FormatCssSimpleFunction,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::simple_function::FormatCssSimpleFunction::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssSimpleFunction {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssSimpleFunction,
        crate::css::auxiliary::simple_function::FormatCssSimpleFunction,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::simple_function::FormatCssSimpleFunction::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssVarFunction>
    for crate::css::auxiliary::var_function::FormatCssVarFunction
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssVarFunction,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssVarFunction>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssVarFunction {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssVarFunction,
        crate::css::auxiliary::var_function::FormatCssVarFunction,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::var_function::FormatCssVarFunction::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssVarFunction {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssVarFunction,
        crate::css::auxiliary::var_function::FormatCssVarFunction,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::var_function::FormatCssVarFunction::default(),
        )
    }
}
impl FormatRule<biome_css_syntax::CssVarFunctionValue>
    for crate::css::auxiliary::var_function_value::FormatCssVarFunctionValue
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssVarFunctionValue,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_css_syntax::CssVarFunctionValue>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssVarFunctionValue {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssVarFunctionValue,
        crate::css::auxiliary::var_function_value::FormatCssVarFunctionValue,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::auxiliary::var_function_value::FormatCssVarFunctionValue::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssVarFunctionValue {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssVarFunctionValue,
        crate::css::auxiliary::var_function_value::FormatCssVarFunctionValue,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::auxiliary::var_function_value::FormatCssVarFunctionValue::default(),
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
impl FormatRule<biome_css_syntax::CssBogusBody>
    for crate::css::bogus::bogus_body::FormatCssBogusBody
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_css_syntax::CssBogusBody, f: &mut CssFormatter) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_css_syntax::CssBogusBody>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssBogusBody {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssBogusBody,
        crate::css::bogus::bogus_body::FormatCssBogusBody,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::bogus::bogus_body::FormatCssBogusBody::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssBogusBody {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssBogusBody,
        crate::css::bogus::bogus_body::FormatCssBogusBody,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::bogus::bogus_body::FormatCssBogusBody::default(),
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
impl FormatRule<biome_css_syntax::CssBogusComponentValue>
    for crate::css::bogus::bogus_component_value::FormatCssBogusComponentValue
{
    type Context = CssFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_css_syntax::CssBogusComponentValue,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_css_syntax::CssBogusComponentValue>::fmt(self, node, f)
    }
}
impl AsFormat<CssFormatContext> for biome_css_syntax::CssBogusComponentValue {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_css_syntax::CssBogusComponentValue,
        crate::css::bogus::bogus_component_value::FormatCssBogusComponentValue,
    >;
    fn format(&self) -> Self::Format<'_> {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatRefWithRule::new(
            self,
            crate::css::bogus::bogus_component_value::FormatCssBogusComponentValue::default(),
        )
    }
}
impl IntoFormat<CssFormatContext> for biome_css_syntax::CssBogusComponentValue {
    type Format = FormatOwnedWithRule<
        biome_css_syntax::CssBogusComponentValue,
        crate::css::bogus::bogus_component_value::FormatCssBogusComponentValue,
    >;
    fn into_format(self) -> Self::Format {
        #![allow(clippy::default_constructed_unit_structs)]
        FormatOwnedWithRule::new(
            self,
            crate::css::bogus::bogus_component_value::FormatCssBogusComponentValue::default(),
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
