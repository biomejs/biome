//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

#![allow(clippy::use_self)]
#![expect(clippy::default_constructed_unit_structs)]
use crate::{
    AsFormat, FormatBogusNodeRule, FormatNodeRule, HtmlFormatContext, HtmlFormatter, IntoFormat,
};
use biome_formatter::{FormatOwnedWithRule, FormatRefWithRule, FormatResult, FormatRule};
impl FormatRule<biome_html_syntax::AstroEmbeddedContent>
    for crate::astro::auxiliary::embedded_content::FormatAstroEmbeddedContent
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::AstroEmbeddedContent,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::AstroEmbeddedContent>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::AstroEmbeddedContent {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::AstroEmbeddedContent,
        crate::astro::auxiliary::embedded_content::FormatAstroEmbeddedContent,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::astro::auxiliary::embedded_content::FormatAstroEmbeddedContent::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::AstroEmbeddedContent {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::AstroEmbeddedContent,
        crate::astro::auxiliary::embedded_content::FormatAstroEmbeddedContent,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::astro::auxiliary::embedded_content::FormatAstroEmbeddedContent::default(),
        )
    }
}
impl FormatRule<biome_html_syntax::AstroFrontmatterElement>
    for crate::astro::auxiliary::frontmatter_element::FormatAstroFrontmatterElement
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::AstroFrontmatterElement,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::AstroFrontmatterElement>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::AstroFrontmatterElement {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::AstroFrontmatterElement,
        crate::astro::auxiliary::frontmatter_element::FormatAstroFrontmatterElement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::astro::auxiliary::frontmatter_element::FormatAstroFrontmatterElement::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::AstroFrontmatterElement {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::AstroFrontmatterElement,
        crate::astro::auxiliary::frontmatter_element::FormatAstroFrontmatterElement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::astro::auxiliary::frontmatter_element::FormatAstroFrontmatterElement::default(),
        )
    }
}
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
impl FormatRule<biome_html_syntax::HtmlAttributeName>
    for crate::html::auxiliary::attribute_name::FormatHtmlAttributeName
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::HtmlAttributeName,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::HtmlAttributeName>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::HtmlAttributeName {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::HtmlAttributeName,
        crate::html::auxiliary::attribute_name::FormatHtmlAttributeName,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::html::auxiliary::attribute_name::FormatHtmlAttributeName::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::HtmlAttributeName {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::HtmlAttributeName,
        crate::html::auxiliary::attribute_name::FormatHtmlAttributeName,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::html::auxiliary::attribute_name::FormatHtmlAttributeName::default(),
        )
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
impl FormatRule<biome_html_syntax::HtmlDoubleTextExpression>
    for crate::html::auxiliary::double_text_expression::FormatHtmlDoubleTextExpression
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::HtmlDoubleTextExpression,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::HtmlDoubleTextExpression>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::HtmlDoubleTextExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::HtmlDoubleTextExpression,
        crate::html::auxiliary::double_text_expression::FormatHtmlDoubleTextExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::html::auxiliary::double_text_expression::FormatHtmlDoubleTextExpression::default(
            ),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::HtmlDoubleTextExpression {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::HtmlDoubleTextExpression,
        crate::html::auxiliary::double_text_expression::FormatHtmlDoubleTextExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::html::auxiliary::double_text_expression::FormatHtmlDoubleTextExpression::default(
            ),
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
impl FormatRule<biome_html_syntax::HtmlEmbeddedContent>
    for crate::html::auxiliary::embedded_content::FormatHtmlEmbeddedContent
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::HtmlEmbeddedContent,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::HtmlEmbeddedContent>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::HtmlEmbeddedContent {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::HtmlEmbeddedContent,
        crate::html::auxiliary::embedded_content::FormatHtmlEmbeddedContent,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::html::auxiliary::embedded_content::FormatHtmlEmbeddedContent::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::HtmlEmbeddedContent {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::HtmlEmbeddedContent,
        crate::html::auxiliary::embedded_content::FormatHtmlEmbeddedContent,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::html::auxiliary::embedded_content::FormatHtmlEmbeddedContent::default(),
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
impl FormatRule<biome_html_syntax::HtmlSingleTextExpression>
    for crate::html::auxiliary::single_text_expression::FormatHtmlSingleTextExpression
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::HtmlSingleTextExpression,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::HtmlSingleTextExpression>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::HtmlSingleTextExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::HtmlSingleTextExpression,
        crate::html::auxiliary::single_text_expression::FormatHtmlSingleTextExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::html::auxiliary::single_text_expression::FormatHtmlSingleTextExpression::default(
            ),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::HtmlSingleTextExpression {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::HtmlSingleTextExpression,
        crate::html::auxiliary::single_text_expression::FormatHtmlSingleTextExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::html::auxiliary::single_text_expression::FormatHtmlSingleTextExpression::default(
            ),
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
impl FormatRule<biome_html_syntax::HtmlTagName>
    for crate::html::auxiliary::tag_name::FormatHtmlTagName
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::HtmlTagName,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::HtmlTagName>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::HtmlTagName {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::HtmlTagName,
        crate::html::auxiliary::tag_name::FormatHtmlTagName,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::html::auxiliary::tag_name::FormatHtmlTagName::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::HtmlTagName {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::HtmlTagName,
        crate::html::auxiliary::tag_name::FormatHtmlTagName,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::html::auxiliary::tag_name::FormatHtmlTagName::default(),
        )
    }
}
impl FormatRule<biome_html_syntax::HtmlTextExpression>
    for crate::html::auxiliary::text_expression::FormatHtmlTextExpression
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::HtmlTextExpression,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::HtmlTextExpression>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::HtmlTextExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::HtmlTextExpression,
        crate::html::auxiliary::text_expression::FormatHtmlTextExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::html::auxiliary::text_expression::FormatHtmlTextExpression::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::HtmlTextExpression {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::HtmlTextExpression,
        crate::html::auxiliary::text_expression::FormatHtmlTextExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::html::auxiliary::text_expression::FormatHtmlTextExpression::default(),
        )
    }
}
impl FormatRule<biome_html_syntax::VueDirective>
    for crate::vue::auxiliary::directive::FormatVueDirective
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::VueDirective,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::VueDirective>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::VueDirective {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::VueDirective,
        crate::vue::auxiliary::directive::FormatVueDirective,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::vue::auxiliary::directive::FormatVueDirective::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::VueDirective {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::VueDirective,
        crate::vue::auxiliary::directive::FormatVueDirective,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::vue::auxiliary::directive::FormatVueDirective::default(),
        )
    }
}
impl FormatRule<biome_html_syntax::VueDirectiveArgument>
    for crate::vue::auxiliary::directive_argument::FormatVueDirectiveArgument
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::VueDirectiveArgument,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::VueDirectiveArgument>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::VueDirectiveArgument {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::VueDirectiveArgument,
        crate::vue::auxiliary::directive_argument::FormatVueDirectiveArgument,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::vue::auxiliary::directive_argument::FormatVueDirectiveArgument::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::VueDirectiveArgument {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::VueDirectiveArgument,
        crate::vue::auxiliary::directive_argument::FormatVueDirectiveArgument,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::vue::auxiliary::directive_argument::FormatVueDirectiveArgument::default(),
        )
    }
}
impl FormatRule<biome_html_syntax::VueDynamicArgument>
    for crate::vue::auxiliary::dynamic_argument::FormatVueDynamicArgument
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::VueDynamicArgument,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::VueDynamicArgument>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::VueDynamicArgument {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::VueDynamicArgument,
        crate::vue::auxiliary::dynamic_argument::FormatVueDynamicArgument,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::vue::auxiliary::dynamic_argument::FormatVueDynamicArgument::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::VueDynamicArgument {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::VueDynamicArgument,
        crate::vue::auxiliary::dynamic_argument::FormatVueDynamicArgument,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::vue::auxiliary::dynamic_argument::FormatVueDynamicArgument::default(),
        )
    }
}
impl FormatRule<biome_html_syntax::VueModifier>
    for crate::vue::auxiliary::modifier::FormatVueModifier
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::VueModifier,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::VueModifier>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::VueModifier {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::VueModifier,
        crate::vue::auxiliary::modifier::FormatVueModifier,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::vue::auxiliary::modifier::FormatVueModifier::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::VueModifier {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::VueModifier,
        crate::vue::auxiliary::modifier::FormatVueModifier,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::vue::auxiliary::modifier::FormatVueModifier::default(),
        )
    }
}
impl FormatRule<biome_html_syntax::VueStaticArgument>
    for crate::vue::auxiliary::static_argument::FormatVueStaticArgument
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::VueStaticArgument,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::VueStaticArgument>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::VueStaticArgument {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::VueStaticArgument,
        crate::vue::auxiliary::static_argument::FormatVueStaticArgument,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::vue::auxiliary::static_argument::FormatVueStaticArgument::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::VueStaticArgument {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::VueStaticArgument,
        crate::vue::auxiliary::static_argument::FormatVueStaticArgument,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::vue::auxiliary::static_argument::FormatVueStaticArgument::default(),
        )
    }
}
impl FormatRule<biome_html_syntax::VueVBindShorthandDirective>
    for crate::vue::auxiliary::v_bind_shorthand_directive::FormatVueVBindShorthandDirective
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::VueVBindShorthandDirective,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::VueVBindShorthandDirective>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::VueVBindShorthandDirective {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::VueVBindShorthandDirective,
        crate::vue::auxiliary::v_bind_shorthand_directive::FormatVueVBindShorthandDirective,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: vue :: auxiliary :: v_bind_shorthand_directive :: FormatVueVBindShorthandDirective :: default ())
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::VueVBindShorthandDirective {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::VueVBindShorthandDirective,
        crate::vue::auxiliary::v_bind_shorthand_directive::FormatVueVBindShorthandDirective,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: vue :: auxiliary :: v_bind_shorthand_directive :: FormatVueVBindShorthandDirective :: default ())
    }
}
impl FormatRule<biome_html_syntax::VueVOnShorthandDirective>
    for crate::vue::auxiliary::v_on_shorthand_directive::FormatVueVOnShorthandDirective
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::VueVOnShorthandDirective,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::VueVOnShorthandDirective>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::VueVOnShorthandDirective {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::VueVOnShorthandDirective,
        crate::vue::auxiliary::v_on_shorthand_directive::FormatVueVOnShorthandDirective,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: vue :: auxiliary :: v_on_shorthand_directive :: FormatVueVOnShorthandDirective :: default ())
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::VueVOnShorthandDirective {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::VueVOnShorthandDirective,
        crate::vue::auxiliary::v_on_shorthand_directive::FormatVueVOnShorthandDirective,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: vue :: auxiliary :: v_on_shorthand_directive :: FormatVueVOnShorthandDirective :: default ())
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
impl AsFormat<HtmlFormatContext> for biome_html_syntax::VueModifierList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::VueModifierList,
        crate::vue::lists::modifier_list::FormatVueModifierList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::vue::lists::modifier_list::FormatVueModifierList::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::VueModifierList {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::VueModifierList,
        crate::vue::lists::modifier_list::FormatVueModifierList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::vue::lists::modifier_list::FormatVueModifierList::default(),
        )
    }
}
impl FormatRule<biome_html_syntax::AstroBogusFrontmatter>
    for crate::astro::bogus::bogus_frontmatter::FormatAstroBogusFrontmatter
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::AstroBogusFrontmatter,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_html_syntax::AstroBogusFrontmatter>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::AstroBogusFrontmatter {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::AstroBogusFrontmatter,
        crate::astro::bogus::bogus_frontmatter::FormatAstroBogusFrontmatter,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::astro::bogus::bogus_frontmatter::FormatAstroBogusFrontmatter::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::AstroBogusFrontmatter {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::AstroBogusFrontmatter,
        crate::astro::bogus::bogus_frontmatter::FormatAstroBogusFrontmatter,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::astro::bogus::bogus_frontmatter::FormatAstroBogusFrontmatter::default(),
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
impl FormatRule<biome_html_syntax::HtmlBogusTextExpression>
    for crate::html::bogus::bogus_text_expression::FormatHtmlBogusTextExpression
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::HtmlBogusTextExpression,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_html_syntax::HtmlBogusTextExpression>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::HtmlBogusTextExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::HtmlBogusTextExpression,
        crate::html::bogus::bogus_text_expression::FormatHtmlBogusTextExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::html::bogus::bogus_text_expression::FormatHtmlBogusTextExpression::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::HtmlBogusTextExpression {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::HtmlBogusTextExpression,
        crate::html::bogus::bogus_text_expression::FormatHtmlBogusTextExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::html::bogus::bogus_text_expression::FormatHtmlBogusTextExpression::default(),
        )
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::AnyAstroFrontmatterElement {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::AnyAstroFrontmatterElement,
        crate::astro::any::frontmatter_element::FormatAnyAstroFrontmatterElement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::astro::any::frontmatter_element::FormatAnyAstroFrontmatterElement::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::AnyAstroFrontmatterElement {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::AnyAstroFrontmatterElement,
        crate::astro::any::frontmatter_element::FormatAnyAstroFrontmatterElement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::astro::any::frontmatter_element::FormatAnyAstroFrontmatterElement::default(),
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
impl AsFormat<HtmlFormatContext> for biome_html_syntax::AnyHtmlAttributeInitializer {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::AnyHtmlAttributeInitializer,
        crate::html::any::attribute_initializer::FormatAnyHtmlAttributeInitializer,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::html::any::attribute_initializer::FormatAnyHtmlAttributeInitializer::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::AnyHtmlAttributeInitializer {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::AnyHtmlAttributeInitializer,
        crate::html::any::attribute_initializer::FormatAnyHtmlAttributeInitializer,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::html::any::attribute_initializer::FormatAnyHtmlAttributeInitializer::default(),
        )
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::AnyHtmlContent {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::AnyHtmlContent,
        crate::html::any::content::FormatAnyHtmlContent,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::html::any::content::FormatAnyHtmlContent::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::AnyHtmlContent {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::AnyHtmlContent,
        crate::html::any::content::FormatAnyHtmlContent,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::html::any::content::FormatAnyHtmlContent::default(),
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
impl AsFormat<HtmlFormatContext> for biome_html_syntax::AnyHtmlTextExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::AnyHtmlTextExpression,
        crate::html::any::text_expression::FormatAnyHtmlTextExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::html::any::text_expression::FormatAnyHtmlTextExpression::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::AnyHtmlTextExpression {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::AnyHtmlTextExpression,
        crate::html::any::text_expression::FormatAnyHtmlTextExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::html::any::text_expression::FormatAnyHtmlTextExpression::default(),
        )
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::AnyVueDirective {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::AnyVueDirective,
        crate::vue::any::directive::FormatAnyVueDirective,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::vue::any::directive::FormatAnyVueDirective::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::AnyVueDirective {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::AnyVueDirective,
        crate::vue::any::directive::FormatAnyVueDirective,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::vue::any::directive::FormatAnyVueDirective::default(),
        )
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::AnyVueDirectiveArgument {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::AnyVueDirectiveArgument,
        crate::vue::any::directive_argument::FormatAnyVueDirectiveArgument,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::vue::any::directive_argument::FormatAnyVueDirectiveArgument::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::AnyVueDirectiveArgument {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::AnyVueDirectiveArgument,
        crate::vue::any::directive_argument::FormatAnyVueDirectiveArgument,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::vue::any::directive_argument::FormatAnyVueDirectiveArgument::default(),
        )
    }
}
