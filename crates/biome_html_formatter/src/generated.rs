//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

#![expect(clippy::default_constructed_unit_structs)]
use crate::{
    AsFormat, FormatBogusNodeRule, FormatNodeRule, HtmlFormatContext, HtmlFormatter, IntoFormat,
};
use biome_formatter::{FormatOwnedWithRule, FormatRefWithRule, FormatResult, FormatRule};
impl FormatRule<biome_html_syntax::HtmlAttribute>
    for crate::html::auxiliary::attribute::FormatHtmlAttribute
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::HtmlAttribute,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::HtmlAttribute>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::HtmlAttribute {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::HtmlAttribute,
        crate::html::auxiliary::attribute::FormatHtmlAttribute,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::html::auxiliary::attribute::FormatHtmlAttribute::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::HtmlAttribute {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::HtmlAttribute,
        crate::html::auxiliary::attribute::FormatHtmlAttribute,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::html::auxiliary::attribute::FormatHtmlAttribute::default(),
        )
    }
}
impl FormatRule<biome_html_syntax::HtmlAttributeInitializerClause>
    for crate::html::auxiliary::attribute_initializer_clause::FormatHtmlAttributeInitializerClause
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::HtmlAttributeInitializerClause,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::HtmlAttributeInitializerClause>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::HtmlAttributeInitializerClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::HtmlAttributeInitializerClause,
        crate::html::auxiliary::attribute_initializer_clause::FormatHtmlAttributeInitializerClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: html :: auxiliary :: attribute_initializer_clause :: FormatHtmlAttributeInitializerClause :: default ())
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::HtmlAttributeInitializerClause {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::HtmlAttributeInitializerClause,
        crate::html::auxiliary::attribute_initializer_clause::FormatHtmlAttributeInitializerClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: html :: auxiliary :: attribute_initializer_clause :: FormatHtmlAttributeInitializerClause :: default ())
    }
}
impl FormatRule<biome_html_syntax::HtmlCdataSection>
    for crate::html::auxiliary::cdata_section::FormatHtmlCdataSection
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::HtmlCdataSection,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::HtmlCdataSection>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::HtmlCdataSection {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::HtmlCdataSection,
        crate::html::auxiliary::cdata_section::FormatHtmlCdataSection,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::html::auxiliary::cdata_section::FormatHtmlCdataSection::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::HtmlCdataSection {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::HtmlCdataSection,
        crate::html::auxiliary::cdata_section::FormatHtmlCdataSection,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::html::auxiliary::cdata_section::FormatHtmlCdataSection::default(),
        )
    }
}
impl FormatRule<biome_html_syntax::HtmlClosingElement>
    for crate::html::auxiliary::closing_element::FormatHtmlClosingElement
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::HtmlClosingElement,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::HtmlClosingElement>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::HtmlClosingElement {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::HtmlClosingElement,
        crate::html::auxiliary::closing_element::FormatHtmlClosingElement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::html::auxiliary::closing_element::FormatHtmlClosingElement::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::HtmlClosingElement {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::HtmlClosingElement,
        crate::html::auxiliary::closing_element::FormatHtmlClosingElement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::html::auxiliary::closing_element::FormatHtmlClosingElement::default(),
        )
    }
}
impl FormatRule<biome_html_syntax::HtmlComment>
    for crate::html::auxiliary::comment::FormatHtmlComment
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::HtmlComment,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::HtmlComment>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::HtmlComment {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::HtmlComment,
        crate::html::auxiliary::comment::FormatHtmlComment,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::html::auxiliary::comment::FormatHtmlComment::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::HtmlComment {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::HtmlComment,
        crate::html::auxiliary::comment::FormatHtmlComment,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::html::auxiliary::comment::FormatHtmlComment::default(),
        )
    }
}
impl FormatRule<biome_html_syntax::HtmlContent>
    for crate::html::auxiliary::content::FormatHtmlContent
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::HtmlContent,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::HtmlContent>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::HtmlContent {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::HtmlContent,
        crate::html::auxiliary::content::FormatHtmlContent,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::html::auxiliary::content::FormatHtmlContent::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::HtmlContent {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::HtmlContent,
        crate::html::auxiliary::content::FormatHtmlContent,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::html::auxiliary::content::FormatHtmlContent::default(),
        )
    }
}
impl FormatRule<biome_html_syntax::HtmlDirective>
    for crate::html::auxiliary::directive::FormatHtmlDirective
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::HtmlDirective,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::HtmlDirective>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::HtmlDirective {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::HtmlDirective,
        crate::html::auxiliary::directive::FormatHtmlDirective,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::html::auxiliary::directive::FormatHtmlDirective::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::HtmlDirective {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::HtmlDirective,
        crate::html::auxiliary::directive::FormatHtmlDirective,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::html::auxiliary::directive::FormatHtmlDirective::default(),
        )
    }
}
impl FormatRule<biome_html_syntax::HtmlElement>
    for crate::html::auxiliary::element::FormatHtmlElement
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::HtmlElement,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::HtmlElement>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::HtmlElement {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::HtmlElement,
        crate::html::auxiliary::element::FormatHtmlElement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::html::auxiliary::element::FormatHtmlElement::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::HtmlElement {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::HtmlElement,
        crate::html::auxiliary::element::FormatHtmlElement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::html::auxiliary::element::FormatHtmlElement::default(),
        )
    }
}
impl FormatRule<biome_html_syntax::HtmlName> for crate::html::auxiliary::name::FormatHtmlName {
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_html_syntax::HtmlName, f: &mut HtmlFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::HtmlName>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::HtmlName {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::HtmlName,
        crate::html::auxiliary::name::FormatHtmlName,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::html::auxiliary::name::FormatHtmlName::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::HtmlName {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::HtmlName,
        crate::html::auxiliary::name::FormatHtmlName,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::html::auxiliary::name::FormatHtmlName::default(),
        )
    }
}
impl FormatRule<biome_html_syntax::HtmlOpeningElement>
    for crate::html::auxiliary::opening_element::FormatHtmlOpeningElement
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::HtmlOpeningElement,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::HtmlOpeningElement>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::HtmlOpeningElement {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::HtmlOpeningElement,
        crate::html::auxiliary::opening_element::FormatHtmlOpeningElement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::html::auxiliary::opening_element::FormatHtmlOpeningElement::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::HtmlOpeningElement {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::HtmlOpeningElement,
        crate::html::auxiliary::opening_element::FormatHtmlOpeningElement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::html::auxiliary::opening_element::FormatHtmlOpeningElement::default(),
        )
    }
}
impl FormatRule<biome_html_syntax::HtmlRoot> for crate::html::auxiliary::root::FormatHtmlRoot {
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_html_syntax::HtmlRoot, f: &mut HtmlFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::HtmlRoot>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::HtmlRoot {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::HtmlRoot,
        crate::html::auxiliary::root::FormatHtmlRoot,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::html::auxiliary::root::FormatHtmlRoot::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::HtmlRoot {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::HtmlRoot,
        crate::html::auxiliary::root::FormatHtmlRoot,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::html::auxiliary::root::FormatHtmlRoot::default(),
        )
    }
}
impl FormatRule<biome_html_syntax::HtmlSelfClosingElement>
    for crate::html::auxiliary::self_closing_element::FormatHtmlSelfClosingElement
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::HtmlSelfClosingElement,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::HtmlSelfClosingElement>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::HtmlSelfClosingElement {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::HtmlSelfClosingElement,
        crate::html::auxiliary::self_closing_element::FormatHtmlSelfClosingElement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::html::auxiliary::self_closing_element::FormatHtmlSelfClosingElement::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::HtmlSelfClosingElement {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::HtmlSelfClosingElement,
        crate::html::auxiliary::self_closing_element::FormatHtmlSelfClosingElement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::html::auxiliary::self_closing_element::FormatHtmlSelfClosingElement::default(),
        )
    }
}
impl FormatRule<biome_html_syntax::HtmlString>
    for crate::html::auxiliary::string::FormatHtmlString
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_html_syntax::HtmlString, f: &mut HtmlFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::HtmlString>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::HtmlString {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::HtmlString,
        crate::html::auxiliary::string::FormatHtmlString,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::html::auxiliary::string::FormatHtmlString::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::HtmlString {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::HtmlString,
        crate::html::auxiliary::string::FormatHtmlString,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::html::auxiliary::string::FormatHtmlString::default(),
        )
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::HtmlAttributeList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::HtmlAttributeList,
        crate::html::lists::attribute_list::FormatHtmlAttributeList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::html::lists::attribute_list::FormatHtmlAttributeList::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::HtmlAttributeList {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::HtmlAttributeList,
        crate::html::lists::attribute_list::FormatHtmlAttributeList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::html::lists::attribute_list::FormatHtmlAttributeList::default(),
        )
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::HtmlElementList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::HtmlElementList,
        crate::html::lists::element_list::FormatHtmlElementList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::html::lists::element_list::FormatHtmlElementList::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::HtmlElementList {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::HtmlElementList,
        crate::html::lists::element_list::FormatHtmlElementList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::html::lists::element_list::FormatHtmlElementList::default(),
        )
    }
}
impl FormatRule<biome_html_syntax::HtmlBogus> for crate::html::bogus::bogus::FormatHtmlBogus {
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_html_syntax::HtmlBogus, f: &mut HtmlFormatter) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_html_syntax::HtmlBogus>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::HtmlBogus {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::HtmlBogus,
        crate::html::bogus::bogus::FormatHtmlBogus,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, crate::html::bogus::bogus::FormatHtmlBogus::default())
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::HtmlBogus {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::HtmlBogus,
        crate::html::bogus::bogus::FormatHtmlBogus,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, crate::html::bogus::bogus::FormatHtmlBogus::default())
    }
}
impl FormatRule<biome_html_syntax::HtmlBogusAttribute>
    for crate::html::bogus::bogus_attribute::FormatHtmlBogusAttribute
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::HtmlBogusAttribute,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_html_syntax::HtmlBogusAttribute>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::HtmlBogusAttribute {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::HtmlBogusAttribute,
        crate::html::bogus::bogus_attribute::FormatHtmlBogusAttribute,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::html::bogus::bogus_attribute::FormatHtmlBogusAttribute::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::HtmlBogusAttribute {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::HtmlBogusAttribute,
        crate::html::bogus::bogus_attribute::FormatHtmlBogusAttribute,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::html::bogus::bogus_attribute::FormatHtmlBogusAttribute::default(),
        )
    }
}
impl FormatRule<biome_html_syntax::HtmlBogusElement>
    for crate::html::bogus::bogus_element::FormatHtmlBogusElement
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::HtmlBogusElement,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_html_syntax::HtmlBogusElement>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::HtmlBogusElement {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::HtmlBogusElement,
        crate::html::bogus::bogus_element::FormatHtmlBogusElement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::html::bogus::bogus_element::FormatHtmlBogusElement::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::HtmlBogusElement {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::HtmlBogusElement,
        crate::html::bogus::bogus_element::FormatHtmlBogusElement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::html::bogus::bogus_element::FormatHtmlBogusElement::default(),
        )
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::AnyHtmlAttribute {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::AnyHtmlAttribute,
        crate::html::any::attribute::FormatAnyHtmlAttribute,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::html::any::attribute::FormatAnyHtmlAttribute::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::AnyHtmlAttribute {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::AnyHtmlAttribute,
        crate::html::any::attribute::FormatAnyHtmlAttribute,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::html::any::attribute::FormatAnyHtmlAttribute::default(),
        )
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::AnyHtmlElement {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::AnyHtmlElement,
        crate::html::any::element::FormatAnyHtmlElement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::html::any::element::FormatAnyHtmlElement::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::AnyHtmlElement {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::AnyHtmlElement,
        crate::html::any::element::FormatAnyHtmlElement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::html::any::element::FormatAnyHtmlElement::default(),
        )
    }
}
