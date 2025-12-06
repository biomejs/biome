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
impl FormatRule<biome_html_syntax::SvelteAttachAttribute>
    for crate::svelte::auxiliary::attach_attribute::FormatSvelteAttachAttribute
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::SvelteAttachAttribute,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::SvelteAttachAttribute>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::SvelteAttachAttribute {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::SvelteAttachAttribute,
        crate::svelte::auxiliary::attach_attribute::FormatSvelteAttachAttribute,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::svelte::auxiliary::attach_attribute::FormatSvelteAttachAttribute::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::SvelteAttachAttribute {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::SvelteAttachAttribute,
        crate::svelte::auxiliary::attach_attribute::FormatSvelteAttachAttribute,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::svelte::auxiliary::attach_attribute::FormatSvelteAttachAttribute::default(),
        )
    }
}
impl FormatRule<biome_html_syntax::SvelteConstBlock>
    for crate::svelte::auxiliary::const_block::FormatSvelteConstBlock
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::SvelteConstBlock,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::SvelteConstBlock>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::SvelteConstBlock {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::SvelteConstBlock,
        crate::svelte::auxiliary::const_block::FormatSvelteConstBlock,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::svelte::auxiliary::const_block::FormatSvelteConstBlock::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::SvelteConstBlock {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::SvelteConstBlock,
        crate::svelte::auxiliary::const_block::FormatSvelteConstBlock,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::svelte::auxiliary::const_block::FormatSvelteConstBlock::default(),
        )
    }
}
impl FormatRule<biome_html_syntax::SvelteDebugBlock>
    for crate::svelte::auxiliary::debug_block::FormatSvelteDebugBlock
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::SvelteDebugBlock,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::SvelteDebugBlock>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::SvelteDebugBlock {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::SvelteDebugBlock,
        crate::svelte::auxiliary::debug_block::FormatSvelteDebugBlock,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::svelte::auxiliary::debug_block::FormatSvelteDebugBlock::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::SvelteDebugBlock {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::SvelteDebugBlock,
        crate::svelte::auxiliary::debug_block::FormatSvelteDebugBlock,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::svelte::auxiliary::debug_block::FormatSvelteDebugBlock::default(),
        )
    }
}
impl FormatRule<biome_html_syntax::SvelteElseClause>
    for crate::svelte::auxiliary::else_clause::FormatSvelteElseClause
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::SvelteElseClause,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::SvelteElseClause>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::SvelteElseClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::SvelteElseClause,
        crate::svelte::auxiliary::else_clause::FormatSvelteElseClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::svelte::auxiliary::else_clause::FormatSvelteElseClause::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::SvelteElseClause {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::SvelteElseClause,
        crate::svelte::auxiliary::else_clause::FormatSvelteElseClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::svelte::auxiliary::else_clause::FormatSvelteElseClause::default(),
        )
    }
}
impl FormatRule<biome_html_syntax::SvelteElseIfClause>
    for crate::svelte::auxiliary::else_if_clause::FormatSvelteElseIfClause
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::SvelteElseIfClause,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::SvelteElseIfClause>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::SvelteElseIfClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::SvelteElseIfClause,
        crate::svelte::auxiliary::else_if_clause::FormatSvelteElseIfClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::svelte::auxiliary::else_if_clause::FormatSvelteElseIfClause::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::SvelteElseIfClause {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::SvelteElseIfClause,
        crate::svelte::auxiliary::else_if_clause::FormatSvelteElseIfClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::svelte::auxiliary::else_if_clause::FormatSvelteElseIfClause::default(),
        )
    }
}
impl FormatRule<biome_html_syntax::SvelteHtmlBlock>
    for crate::svelte::auxiliary::html_block::FormatSvelteHtmlBlock
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::SvelteHtmlBlock,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::SvelteHtmlBlock>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::SvelteHtmlBlock {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::SvelteHtmlBlock,
        crate::svelte::auxiliary::html_block::FormatSvelteHtmlBlock,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::svelte::auxiliary::html_block::FormatSvelteHtmlBlock::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::SvelteHtmlBlock {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::SvelteHtmlBlock,
        crate::svelte::auxiliary::html_block::FormatSvelteHtmlBlock,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::svelte::auxiliary::html_block::FormatSvelteHtmlBlock::default(),
        )
    }
}
impl FormatRule<biome_html_syntax::SvelteIfBlock>
    for crate::svelte::auxiliary::if_block::FormatSvelteIfBlock
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::SvelteIfBlock,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::SvelteIfBlock>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::SvelteIfBlock {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::SvelteIfBlock,
        crate::svelte::auxiliary::if_block::FormatSvelteIfBlock,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::svelte::auxiliary::if_block::FormatSvelteIfBlock::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::SvelteIfBlock {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::SvelteIfBlock,
        crate::svelte::auxiliary::if_block::FormatSvelteIfBlock,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::svelte::auxiliary::if_block::FormatSvelteIfBlock::default(),
        )
    }
}
impl FormatRule<biome_html_syntax::SvelteIfClosingBlock>
    for crate::svelte::auxiliary::if_closing_block::FormatSvelteIfClosingBlock
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::SvelteIfClosingBlock,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::SvelteIfClosingBlock>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::SvelteIfClosingBlock {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::SvelteIfClosingBlock,
        crate::svelte::auxiliary::if_closing_block::FormatSvelteIfClosingBlock,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::svelte::auxiliary::if_closing_block::FormatSvelteIfClosingBlock::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::SvelteIfClosingBlock {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::SvelteIfClosingBlock,
        crate::svelte::auxiliary::if_closing_block::FormatSvelteIfClosingBlock,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::svelte::auxiliary::if_closing_block::FormatSvelteIfClosingBlock::default(),
        )
    }
}
impl FormatRule<biome_html_syntax::SvelteIfOpeningBlock>
    for crate::svelte::auxiliary::if_opening_block::FormatSvelteIfOpeningBlock
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::SvelteIfOpeningBlock,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::SvelteIfOpeningBlock>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::SvelteIfOpeningBlock {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::SvelteIfOpeningBlock,
        crate::svelte::auxiliary::if_opening_block::FormatSvelteIfOpeningBlock,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::svelte::auxiliary::if_opening_block::FormatSvelteIfOpeningBlock::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::SvelteIfOpeningBlock {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::SvelteIfOpeningBlock,
        crate::svelte::auxiliary::if_opening_block::FormatSvelteIfOpeningBlock,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::svelte::auxiliary::if_opening_block::FormatSvelteIfOpeningBlock::default(),
        )
    }
}
impl FormatRule<biome_html_syntax::SvelteKeyBlock>
    for crate::svelte::auxiliary::key_block::FormatSvelteKeyBlock
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::SvelteKeyBlock,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::SvelteKeyBlock>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::SvelteKeyBlock {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::SvelteKeyBlock,
        crate::svelte::auxiliary::key_block::FormatSvelteKeyBlock,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::svelte::auxiliary::key_block::FormatSvelteKeyBlock::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::SvelteKeyBlock {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::SvelteKeyBlock,
        crate::svelte::auxiliary::key_block::FormatSvelteKeyBlock,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::svelte::auxiliary::key_block::FormatSvelteKeyBlock::default(),
        )
    }
}
impl FormatRule<biome_html_syntax::SvelteKeyClosingBlock>
    for crate::svelte::auxiliary::key_closing_block::FormatSvelteKeyClosingBlock
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::SvelteKeyClosingBlock,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::SvelteKeyClosingBlock>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::SvelteKeyClosingBlock {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::SvelteKeyClosingBlock,
        crate::svelte::auxiliary::key_closing_block::FormatSvelteKeyClosingBlock,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::svelte::auxiliary::key_closing_block::FormatSvelteKeyClosingBlock::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::SvelteKeyClosingBlock {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::SvelteKeyClosingBlock,
        crate::svelte::auxiliary::key_closing_block::FormatSvelteKeyClosingBlock,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::svelte::auxiliary::key_closing_block::FormatSvelteKeyClosingBlock::default(),
        )
    }
}
impl FormatRule<biome_html_syntax::SvelteKeyOpeningBlock>
    for crate::svelte::auxiliary::key_opening_block::FormatSvelteKeyOpeningBlock
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::SvelteKeyOpeningBlock,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::SvelteKeyOpeningBlock>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::SvelteKeyOpeningBlock {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::SvelteKeyOpeningBlock,
        crate::svelte::auxiliary::key_opening_block::FormatSvelteKeyOpeningBlock,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::svelte::auxiliary::key_opening_block::FormatSvelteKeyOpeningBlock::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::SvelteKeyOpeningBlock {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::SvelteKeyOpeningBlock,
        crate::svelte::auxiliary::key_opening_block::FormatSvelteKeyOpeningBlock,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::svelte::auxiliary::key_opening_block::FormatSvelteKeyOpeningBlock::default(),
        )
    }
}
impl FormatRule<biome_html_syntax::SvelteName>
    for crate::svelte::auxiliary::name::FormatSvelteName
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_html_syntax::SvelteName, f: &mut HtmlFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::SvelteName>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::SvelteName {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::SvelteName,
        crate::svelte::auxiliary::name::FormatSvelteName,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::svelte::auxiliary::name::FormatSvelteName::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::SvelteName {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::SvelteName,
        crate::svelte::auxiliary::name::FormatSvelteName,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::svelte::auxiliary::name::FormatSvelteName::default(),
        )
    }
}
impl FormatRule<biome_html_syntax::SvelteRenderBlock>
    for crate::svelte::auxiliary::render_block::FormatSvelteRenderBlock
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::SvelteRenderBlock,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::SvelteRenderBlock>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::SvelteRenderBlock {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::SvelteRenderBlock,
        crate::svelte::auxiliary::render_block::FormatSvelteRenderBlock,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::svelte::auxiliary::render_block::FormatSvelteRenderBlock::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::SvelteRenderBlock {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::SvelteRenderBlock,
        crate::svelte::auxiliary::render_block::FormatSvelteRenderBlock,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::svelte::auxiliary::render_block::FormatSvelteRenderBlock::default(),
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
impl FormatRule<biome_html_syntax::VueVSlotShorthandDirective>
    for crate::vue::auxiliary::v_slot_shorthand_directive::FormatVueVSlotShorthandDirective
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::VueVSlotShorthandDirective,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::VueVSlotShorthandDirective>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::VueVSlotShorthandDirective {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::VueVSlotShorthandDirective,
        crate::vue::auxiliary::v_slot_shorthand_directive::FormatVueVSlotShorthandDirective,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: vue :: auxiliary :: v_slot_shorthand_directive :: FormatVueVSlotShorthandDirective :: default ())
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::VueVSlotShorthandDirective {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::VueVSlotShorthandDirective,
        crate::vue::auxiliary::v_slot_shorthand_directive::FormatVueVSlotShorthandDirective,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: vue :: auxiliary :: v_slot_shorthand_directive :: FormatVueVSlotShorthandDirective :: default ())
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
impl AsFormat<HtmlFormatContext> for biome_html_syntax::SvelteBindingList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::SvelteBindingList,
        crate::svelte::lists::binding_list::FormatSvelteBindingList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::svelte::lists::binding_list::FormatSvelteBindingList::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::SvelteBindingList {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::SvelteBindingList,
        crate::svelte::lists::binding_list::FormatSvelteBindingList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::svelte::lists::binding_list::FormatSvelteBindingList::default(),
        )
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::SvelteElseIfClauseList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::SvelteElseIfClauseList,
        crate::svelte::lists::else_if_clause_list::FormatSvelteElseIfClauseList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::svelte::lists::else_if_clause_list::FormatSvelteElseIfClauseList::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::SvelteElseIfClauseList {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::SvelteElseIfClauseList,
        crate::svelte::lists::else_if_clause_list::FormatSvelteElseIfClauseList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::svelte::lists::else_if_clause_list::FormatSvelteElseIfClauseList::default(),
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
impl FormatRule<biome_html_syntax::SvelteBogusBlock>
    for crate::svelte::bogus::bogus_block::FormatSvelteBogusBlock
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::SvelteBogusBlock,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_html_syntax::SvelteBogusBlock>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::SvelteBogusBlock {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::SvelteBogusBlock,
        crate::svelte::bogus::bogus_block::FormatSvelteBogusBlock,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::svelte::bogus::bogus_block::FormatSvelteBogusBlock::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::SvelteBogusBlock {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::SvelteBogusBlock,
        crate::svelte::bogus::bogus_block::FormatSvelteBogusBlock,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::svelte::bogus::bogus_block::FormatSvelteBogusBlock::default(),
        )
    }
}
impl FormatRule<biome_html_syntax::VueBogusDirective>
    for crate::vue::bogus::bogus_directive::FormatVueBogusDirective
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::VueBogusDirective,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_html_syntax::VueBogusDirective>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::VueBogusDirective {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::VueBogusDirective,
        crate::vue::bogus::bogus_directive::FormatVueBogusDirective,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::vue::bogus::bogus_directive::FormatVueBogusDirective::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::VueBogusDirective {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::VueBogusDirective,
        crate::vue::bogus::bogus_directive::FormatVueBogusDirective,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::vue::bogus::bogus_directive::FormatVueBogusDirective::default(),
        )
    }
}
impl FormatRule<biome_html_syntax::VueBogusDirectiveArgument>
    for crate::vue::bogus::bogus_directive_argument::FormatVueBogusDirectiveArgument
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::VueBogusDirectiveArgument,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_html_syntax::VueBogusDirectiveArgument>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::VueBogusDirectiveArgument {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::VueBogusDirectiveArgument,
        crate::vue::bogus::bogus_directive_argument::FormatVueBogusDirectiveArgument,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::vue::bogus::bogus_directive_argument::FormatVueBogusDirectiveArgument::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::VueBogusDirectiveArgument {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::VueBogusDirectiveArgument,
        crate::vue::bogus::bogus_directive_argument::FormatVueBogusDirectiveArgument,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::vue::bogus::bogus_directive_argument::FormatVueBogusDirectiveArgument::default(),
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
impl AsFormat<HtmlFormatContext> for biome_html_syntax::AnySvelteBlock {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::AnySvelteBlock,
        crate::svelte::any::block::FormatAnySvelteBlock,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::svelte::any::block::FormatAnySvelteBlock::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::AnySvelteBlock {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::AnySvelteBlock,
        crate::svelte::any::block::FormatAnySvelteBlock,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::svelte::any::block::FormatAnySvelteBlock::default(),
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
