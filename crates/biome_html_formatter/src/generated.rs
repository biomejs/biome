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
impl FormatRule < biome_html_syntax :: HtmlAttributeDoubleTextExpression > for crate :: html :: auxiliary :: attribute_double_text_expression :: FormatHtmlAttributeDoubleTextExpression { type Context = HtmlFormatContext ; # [inline (always)] fn fmt (& self , node : & biome_html_syntax :: HtmlAttributeDoubleTextExpression , f : & mut HtmlFormatter) -> FormatResult < () > { FormatNodeRule :: < biome_html_syntax :: HtmlAttributeDoubleTextExpression > :: fmt (self , node , f) } }
impl AsFormat<HtmlFormatContext> for biome_html_syntax::HtmlAttributeDoubleTextExpression {
    type Format < 'a > = FormatRefWithRule < 'a , biome_html_syntax :: HtmlAttributeDoubleTextExpression , crate :: html :: auxiliary :: attribute_double_text_expression :: FormatHtmlAttributeDoubleTextExpression > ;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: html :: auxiliary :: attribute_double_text_expression :: FormatHtmlAttributeDoubleTextExpression :: default ())
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::HtmlAttributeDoubleTextExpression {
    type Format = FormatOwnedWithRule < biome_html_syntax :: HtmlAttributeDoubleTextExpression , crate :: html :: auxiliary :: attribute_double_text_expression :: FormatHtmlAttributeDoubleTextExpression > ;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: html :: auxiliary :: attribute_double_text_expression :: FormatHtmlAttributeDoubleTextExpression :: default ())
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
impl FormatRule < biome_html_syntax :: HtmlAttributeSingleTextExpression > for crate :: html :: auxiliary :: attribute_single_text_expression :: FormatHtmlAttributeSingleTextExpression { type Context = HtmlFormatContext ; # [inline (always)] fn fmt (& self , node : & biome_html_syntax :: HtmlAttributeSingleTextExpression , f : & mut HtmlFormatter) -> FormatResult < () > { FormatNodeRule :: < biome_html_syntax :: HtmlAttributeSingleTextExpression > :: fmt (self , node , f) } }
impl AsFormat<HtmlFormatContext> for biome_html_syntax::HtmlAttributeSingleTextExpression {
    type Format < 'a > = FormatRefWithRule < 'a , biome_html_syntax :: HtmlAttributeSingleTextExpression , crate :: html :: auxiliary :: attribute_single_text_expression :: FormatHtmlAttributeSingleTextExpression > ;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: html :: auxiliary :: attribute_single_text_expression :: FormatHtmlAttributeSingleTextExpression :: default ())
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::HtmlAttributeSingleTextExpression {
    type Format = FormatOwnedWithRule < biome_html_syntax :: HtmlAttributeSingleTextExpression , crate :: html :: auxiliary :: attribute_single_text_expression :: FormatHtmlAttributeSingleTextExpression > ;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: html :: auxiliary :: attribute_single_text_expression :: FormatHtmlAttributeSingleTextExpression :: default ())
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
impl FormatRule<biome_html_syntax::HtmlComponentName>
    for crate::html::auxiliary::component_name::FormatHtmlComponentName
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::HtmlComponentName,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::HtmlComponentName>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::HtmlComponentName {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::HtmlComponentName,
        crate::html::auxiliary::component_name::FormatHtmlComponentName,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::html::auxiliary::component_name::FormatHtmlComponentName::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::HtmlComponentName {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::HtmlComponentName,
        crate::html::auxiliary::component_name::FormatHtmlComponentName,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::html::auxiliary::component_name::FormatHtmlComponentName::default(),
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
impl FormatRule<biome_html_syntax::HtmlMemberName>
    for crate::html::auxiliary::member_name::FormatHtmlMemberName
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::HtmlMemberName,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::HtmlMemberName>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::HtmlMemberName {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::HtmlMemberName,
        crate::html::auxiliary::member_name::FormatHtmlMemberName,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::html::auxiliary::member_name::FormatHtmlMemberName::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::HtmlMemberName {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::HtmlMemberName,
        crate::html::auxiliary::member_name::FormatHtmlMemberName,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::html::auxiliary::member_name::FormatHtmlMemberName::default(),
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
impl FormatRule<biome_html_syntax::HtmlSpreadAttribute>
    for crate::html::auxiliary::spread_attribute::FormatHtmlSpreadAttribute
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::HtmlSpreadAttribute,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::HtmlSpreadAttribute>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::HtmlSpreadAttribute {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::HtmlSpreadAttribute,
        crate::html::auxiliary::spread_attribute::FormatHtmlSpreadAttribute,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::html::auxiliary::spread_attribute::FormatHtmlSpreadAttribute::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::HtmlSpreadAttribute {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::HtmlSpreadAttribute,
        crate::html::auxiliary::spread_attribute::FormatHtmlSpreadAttribute,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::html::auxiliary::spread_attribute::FormatHtmlSpreadAttribute::default(),
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
impl FormatRule<biome_html_syntax::SvelteAnimateDirective>
    for crate::svelte::auxiliary::animate_directive::FormatSvelteAnimateDirective
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::SvelteAnimateDirective,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::SvelteAnimateDirective>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::SvelteAnimateDirective {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::SvelteAnimateDirective,
        crate::svelte::auxiliary::animate_directive::FormatSvelteAnimateDirective,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::svelte::auxiliary::animate_directive::FormatSvelteAnimateDirective::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::SvelteAnimateDirective {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::SvelteAnimateDirective,
        crate::svelte::auxiliary::animate_directive::FormatSvelteAnimateDirective,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::svelte::auxiliary::animate_directive::FormatSvelteAnimateDirective::default(),
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
impl FormatRule<biome_html_syntax::SvelteAwaitBlock>
    for crate::svelte::auxiliary::await_block::FormatSvelteAwaitBlock
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::SvelteAwaitBlock,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::SvelteAwaitBlock>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::SvelteAwaitBlock {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::SvelteAwaitBlock,
        crate::svelte::auxiliary::await_block::FormatSvelteAwaitBlock,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::svelte::auxiliary::await_block::FormatSvelteAwaitBlock::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::SvelteAwaitBlock {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::SvelteAwaitBlock,
        crate::svelte::auxiliary::await_block::FormatSvelteAwaitBlock,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::svelte::auxiliary::await_block::FormatSvelteAwaitBlock::default(),
        )
    }
}
impl FormatRule<biome_html_syntax::SvelteAwaitCatchBlock>
    for crate::svelte::auxiliary::await_catch_block::FormatSvelteAwaitCatchBlock
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::SvelteAwaitCatchBlock,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::SvelteAwaitCatchBlock>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::SvelteAwaitCatchBlock {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::SvelteAwaitCatchBlock,
        crate::svelte::auxiliary::await_catch_block::FormatSvelteAwaitCatchBlock,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::svelte::auxiliary::await_catch_block::FormatSvelteAwaitCatchBlock::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::SvelteAwaitCatchBlock {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::SvelteAwaitCatchBlock,
        crate::svelte::auxiliary::await_catch_block::FormatSvelteAwaitCatchBlock,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::svelte::auxiliary::await_catch_block::FormatSvelteAwaitCatchBlock::default(),
        )
    }
}
impl FormatRule<biome_html_syntax::SvelteAwaitCatchClause>
    for crate::svelte::auxiliary::await_catch_clause::FormatSvelteAwaitCatchClause
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::SvelteAwaitCatchClause,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::SvelteAwaitCatchClause>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::SvelteAwaitCatchClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::SvelteAwaitCatchClause,
        crate::svelte::auxiliary::await_catch_clause::FormatSvelteAwaitCatchClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::svelte::auxiliary::await_catch_clause::FormatSvelteAwaitCatchClause::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::SvelteAwaitCatchClause {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::SvelteAwaitCatchClause,
        crate::svelte::auxiliary::await_catch_clause::FormatSvelteAwaitCatchClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::svelte::auxiliary::await_catch_clause::FormatSvelteAwaitCatchClause::default(),
        )
    }
}
impl FormatRule<biome_html_syntax::SvelteAwaitClosingBlock>
    for crate::svelte::auxiliary::await_closing_block::FormatSvelteAwaitClosingBlock
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::SvelteAwaitClosingBlock,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::SvelteAwaitClosingBlock>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::SvelteAwaitClosingBlock {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::SvelteAwaitClosingBlock,
        crate::svelte::auxiliary::await_closing_block::FormatSvelteAwaitClosingBlock,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::svelte::auxiliary::await_closing_block::FormatSvelteAwaitClosingBlock::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::SvelteAwaitClosingBlock {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::SvelteAwaitClosingBlock,
        crate::svelte::auxiliary::await_closing_block::FormatSvelteAwaitClosingBlock,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::svelte::auxiliary::await_closing_block::FormatSvelteAwaitClosingBlock::default(),
        )
    }
}
impl FormatRule<biome_html_syntax::SvelteAwaitOpeningBlock>
    for crate::svelte::auxiliary::await_opening_block::FormatSvelteAwaitOpeningBlock
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::SvelteAwaitOpeningBlock,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::SvelteAwaitOpeningBlock>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::SvelteAwaitOpeningBlock {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::SvelteAwaitOpeningBlock,
        crate::svelte::auxiliary::await_opening_block::FormatSvelteAwaitOpeningBlock,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::svelte::auxiliary::await_opening_block::FormatSvelteAwaitOpeningBlock::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::SvelteAwaitOpeningBlock {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::SvelteAwaitOpeningBlock,
        crate::svelte::auxiliary::await_opening_block::FormatSvelteAwaitOpeningBlock,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::svelte::auxiliary::await_opening_block::FormatSvelteAwaitOpeningBlock::default(),
        )
    }
}
impl FormatRule<biome_html_syntax::SvelteAwaitThenBlock>
    for crate::svelte::auxiliary::await_then_block::FormatSvelteAwaitThenBlock
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::SvelteAwaitThenBlock,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::SvelteAwaitThenBlock>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::SvelteAwaitThenBlock {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::SvelteAwaitThenBlock,
        crate::svelte::auxiliary::await_then_block::FormatSvelteAwaitThenBlock,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::svelte::auxiliary::await_then_block::FormatSvelteAwaitThenBlock::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::SvelteAwaitThenBlock {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::SvelteAwaitThenBlock,
        crate::svelte::auxiliary::await_then_block::FormatSvelteAwaitThenBlock,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::svelte::auxiliary::await_then_block::FormatSvelteAwaitThenBlock::default(),
        )
    }
}
impl FormatRule<biome_html_syntax::SvelteAwaitThenClause>
    for crate::svelte::auxiliary::await_then_clause::FormatSvelteAwaitThenClause
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::SvelteAwaitThenClause,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::SvelteAwaitThenClause>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::SvelteAwaitThenClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::SvelteAwaitThenClause,
        crate::svelte::auxiliary::await_then_clause::FormatSvelteAwaitThenClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::svelte::auxiliary::await_then_clause::FormatSvelteAwaitThenClause::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::SvelteAwaitThenClause {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::SvelteAwaitThenClause,
        crate::svelte::auxiliary::await_then_clause::FormatSvelteAwaitThenClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::svelte::auxiliary::await_then_clause::FormatSvelteAwaitThenClause::default(),
        )
    }
}
impl FormatRule<biome_html_syntax::SvelteBindDirective>
    for crate::svelte::auxiliary::bind_directive::FormatSvelteBindDirective
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::SvelteBindDirective,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::SvelteBindDirective>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::SvelteBindDirective {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::SvelteBindDirective,
        crate::svelte::auxiliary::bind_directive::FormatSvelteBindDirective,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::svelte::auxiliary::bind_directive::FormatSvelteBindDirective::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::SvelteBindDirective {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::SvelteBindDirective,
        crate::svelte::auxiliary::bind_directive::FormatSvelteBindDirective,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::svelte::auxiliary::bind_directive::FormatSvelteBindDirective::default(),
        )
    }
}
impl FormatRule<biome_html_syntax::SvelteClassDirective>
    for crate::svelte::auxiliary::class_directive::FormatSvelteClassDirective
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::SvelteClassDirective,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::SvelteClassDirective>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::SvelteClassDirective {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::SvelteClassDirective,
        crate::svelte::auxiliary::class_directive::FormatSvelteClassDirective,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::svelte::auxiliary::class_directive::FormatSvelteClassDirective::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::SvelteClassDirective {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::SvelteClassDirective,
        crate::svelte::auxiliary::class_directive::FormatSvelteClassDirective,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::svelte::auxiliary::class_directive::FormatSvelteClassDirective::default(),
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
impl FormatRule<biome_html_syntax::SvelteCurlyDestructuredName>
    for crate::svelte::auxiliary::curly_destructured_name::FormatSvelteCurlyDestructuredName
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::SvelteCurlyDestructuredName,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::SvelteCurlyDestructuredName>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::SvelteCurlyDestructuredName {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::SvelteCurlyDestructuredName,
        crate::svelte::auxiliary::curly_destructured_name::FormatSvelteCurlyDestructuredName,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: svelte :: auxiliary :: curly_destructured_name :: FormatSvelteCurlyDestructuredName :: default ())
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::SvelteCurlyDestructuredName {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::SvelteCurlyDestructuredName,
        crate::svelte::auxiliary::curly_destructured_name::FormatSvelteCurlyDestructuredName,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: svelte :: auxiliary :: curly_destructured_name :: FormatSvelteCurlyDestructuredName :: default ())
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
impl FormatRule<biome_html_syntax::SvelteDirectiveModifier>
    for crate::svelte::auxiliary::directive_modifier::FormatSvelteDirectiveModifier
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::SvelteDirectiveModifier,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::SvelteDirectiveModifier>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::SvelteDirectiveModifier {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::SvelteDirectiveModifier,
        crate::svelte::auxiliary::directive_modifier::FormatSvelteDirectiveModifier,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::svelte::auxiliary::directive_modifier::FormatSvelteDirectiveModifier::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::SvelteDirectiveModifier {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::SvelteDirectiveModifier,
        crate::svelte::auxiliary::directive_modifier::FormatSvelteDirectiveModifier,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::svelte::auxiliary::directive_modifier::FormatSvelteDirectiveModifier::default(),
        )
    }
}
impl FormatRule<biome_html_syntax::SvelteDirectiveValue>
    for crate::svelte::value::directive_value::FormatSvelteDirectiveValue
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::SvelteDirectiveValue,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::SvelteDirectiveValue>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::SvelteDirectiveValue {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::SvelteDirectiveValue,
        crate::svelte::value::directive_value::FormatSvelteDirectiveValue,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::svelte::value::directive_value::FormatSvelteDirectiveValue::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::SvelteDirectiveValue {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::SvelteDirectiveValue,
        crate::svelte::value::directive_value::FormatSvelteDirectiveValue,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::svelte::value::directive_value::FormatSvelteDirectiveValue::default(),
        )
    }
}
impl FormatRule<biome_html_syntax::SvelteEachAsKeyedItem>
    for crate::svelte::auxiliary::each_as_keyed_item::FormatSvelteEachAsKeyedItem
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::SvelteEachAsKeyedItem,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::SvelteEachAsKeyedItem>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::SvelteEachAsKeyedItem {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::SvelteEachAsKeyedItem,
        crate::svelte::auxiliary::each_as_keyed_item::FormatSvelteEachAsKeyedItem,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::svelte::auxiliary::each_as_keyed_item::FormatSvelteEachAsKeyedItem::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::SvelteEachAsKeyedItem {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::SvelteEachAsKeyedItem,
        crate::svelte::auxiliary::each_as_keyed_item::FormatSvelteEachAsKeyedItem,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::svelte::auxiliary::each_as_keyed_item::FormatSvelteEachAsKeyedItem::default(),
        )
    }
}
impl FormatRule<biome_html_syntax::SvelteEachBlock>
    for crate::svelte::auxiliary::each_block::FormatSvelteEachBlock
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::SvelteEachBlock,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::SvelteEachBlock>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::SvelteEachBlock {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::SvelteEachBlock,
        crate::svelte::auxiliary::each_block::FormatSvelteEachBlock,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::svelte::auxiliary::each_block::FormatSvelteEachBlock::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::SvelteEachBlock {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::SvelteEachBlock,
        crate::svelte::auxiliary::each_block::FormatSvelteEachBlock,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::svelte::auxiliary::each_block::FormatSvelteEachBlock::default(),
        )
    }
}
impl FormatRule<biome_html_syntax::SvelteEachClosingBlock>
    for crate::svelte::auxiliary::each_closing_block::FormatSvelteEachClosingBlock
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::SvelteEachClosingBlock,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::SvelteEachClosingBlock>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::SvelteEachClosingBlock {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::SvelteEachClosingBlock,
        crate::svelte::auxiliary::each_closing_block::FormatSvelteEachClosingBlock,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::svelte::auxiliary::each_closing_block::FormatSvelteEachClosingBlock::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::SvelteEachClosingBlock {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::SvelteEachClosingBlock,
        crate::svelte::auxiliary::each_closing_block::FormatSvelteEachClosingBlock,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::svelte::auxiliary::each_closing_block::FormatSvelteEachClosingBlock::default(),
        )
    }
}
impl FormatRule<biome_html_syntax::SvelteEachIndex>
    for crate::svelte::auxiliary::each_index::FormatSvelteEachIndex
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::SvelteEachIndex,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::SvelteEachIndex>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::SvelteEachIndex {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::SvelteEachIndex,
        crate::svelte::auxiliary::each_index::FormatSvelteEachIndex,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::svelte::auxiliary::each_index::FormatSvelteEachIndex::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::SvelteEachIndex {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::SvelteEachIndex,
        crate::svelte::auxiliary::each_index::FormatSvelteEachIndex,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::svelte::auxiliary::each_index::FormatSvelteEachIndex::default(),
        )
    }
}
impl FormatRule<biome_html_syntax::SvelteEachKey>
    for crate::svelte::auxiliary::each_key::FormatSvelteEachKey
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::SvelteEachKey,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::SvelteEachKey>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::SvelteEachKey {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::SvelteEachKey,
        crate::svelte::auxiliary::each_key::FormatSvelteEachKey,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::svelte::auxiliary::each_key::FormatSvelteEachKey::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::SvelteEachKey {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::SvelteEachKey,
        crate::svelte::auxiliary::each_key::FormatSvelteEachKey,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::svelte::auxiliary::each_key::FormatSvelteEachKey::default(),
        )
    }
}
impl FormatRule<biome_html_syntax::SvelteEachKeyedItem>
    for crate::svelte::auxiliary::each_keyed_item::FormatSvelteEachKeyedItem
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::SvelteEachKeyedItem,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::SvelteEachKeyedItem>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::SvelteEachKeyedItem {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::SvelteEachKeyedItem,
        crate::svelte::auxiliary::each_keyed_item::FormatSvelteEachKeyedItem,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::svelte::auxiliary::each_keyed_item::FormatSvelteEachKeyedItem::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::SvelteEachKeyedItem {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::SvelteEachKeyedItem,
        crate::svelte::auxiliary::each_keyed_item::FormatSvelteEachKeyedItem,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::svelte::auxiliary::each_keyed_item::FormatSvelteEachKeyedItem::default(),
        )
    }
}
impl FormatRule<biome_html_syntax::SvelteEachOpeningBlock>
    for crate::svelte::auxiliary::each_opening_block::FormatSvelteEachOpeningBlock
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::SvelteEachOpeningBlock,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::SvelteEachOpeningBlock>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::SvelteEachOpeningBlock {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::SvelteEachOpeningBlock,
        crate::svelte::auxiliary::each_opening_block::FormatSvelteEachOpeningBlock,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::svelte::auxiliary::each_opening_block::FormatSvelteEachOpeningBlock::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::SvelteEachOpeningBlock {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::SvelteEachOpeningBlock,
        crate::svelte::auxiliary::each_opening_block::FormatSvelteEachOpeningBlock,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::svelte::auxiliary::each_opening_block::FormatSvelteEachOpeningBlock::default(),
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
impl FormatRule<biome_html_syntax::SvelteInDirective>
    for crate::svelte::auxiliary::in_directive::FormatSvelteInDirective
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::SvelteInDirective,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::SvelteInDirective>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::SvelteInDirective {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::SvelteInDirective,
        crate::svelte::auxiliary::in_directive::FormatSvelteInDirective,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::svelte::auxiliary::in_directive::FormatSvelteInDirective::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::SvelteInDirective {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::SvelteInDirective,
        crate::svelte::auxiliary::in_directive::FormatSvelteInDirective,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::svelte::auxiliary::in_directive::FormatSvelteInDirective::default(),
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
impl FormatRule<biome_html_syntax::SvelteLiteral>
    for crate::svelte::auxiliary::literal::FormatSvelteLiteral
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::SvelteLiteral,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::SvelteLiteral>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::SvelteLiteral {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::SvelteLiteral,
        crate::svelte::auxiliary::literal::FormatSvelteLiteral,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::svelte::auxiliary::literal::FormatSvelteLiteral::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::SvelteLiteral {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::SvelteLiteral,
        crate::svelte::auxiliary::literal::FormatSvelteLiteral,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::svelte::auxiliary::literal::FormatSvelteLiteral::default(),
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
impl FormatRule<biome_html_syntax::SvelteOutDirective>
    for crate::svelte::auxiliary::out_directive::FormatSvelteOutDirective
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::SvelteOutDirective,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::SvelteOutDirective>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::SvelteOutDirective {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::SvelteOutDirective,
        crate::svelte::auxiliary::out_directive::FormatSvelteOutDirective,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::svelte::auxiliary::out_directive::FormatSvelteOutDirective::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::SvelteOutDirective {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::SvelteOutDirective,
        crate::svelte::auxiliary::out_directive::FormatSvelteOutDirective,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::svelte::auxiliary::out_directive::FormatSvelteOutDirective::default(),
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
impl FormatRule<biome_html_syntax::SvelteRestBinding>
    for crate::svelte::auxiliary::rest_binding::FormatSvelteRestBinding
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::SvelteRestBinding,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::SvelteRestBinding>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::SvelteRestBinding {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::SvelteRestBinding,
        crate::svelte::auxiliary::rest_binding::FormatSvelteRestBinding,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::svelte::auxiliary::rest_binding::FormatSvelteRestBinding::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::SvelteRestBinding {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::SvelteRestBinding,
        crate::svelte::auxiliary::rest_binding::FormatSvelteRestBinding,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::svelte::auxiliary::rest_binding::FormatSvelteRestBinding::default(),
        )
    }
}
impl FormatRule<biome_html_syntax::SvelteSnippetBlock>
    for crate::svelte::auxiliary::snippet_block::FormatSvelteSnippetBlock
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::SvelteSnippetBlock,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::SvelteSnippetBlock>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::SvelteSnippetBlock {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::SvelteSnippetBlock,
        crate::svelte::auxiliary::snippet_block::FormatSvelteSnippetBlock,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::svelte::auxiliary::snippet_block::FormatSvelteSnippetBlock::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::SvelteSnippetBlock {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::SvelteSnippetBlock,
        crate::svelte::auxiliary::snippet_block::FormatSvelteSnippetBlock,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::svelte::auxiliary::snippet_block::FormatSvelteSnippetBlock::default(),
        )
    }
}
impl FormatRule<biome_html_syntax::SvelteSnippetClosingBlock>
    for crate::svelte::auxiliary::snippet_closing_block::FormatSvelteSnippetClosingBlock
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::SvelteSnippetClosingBlock,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::SvelteSnippetClosingBlock>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::SvelteSnippetClosingBlock {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::SvelteSnippetClosingBlock,
        crate::svelte::auxiliary::snippet_closing_block::FormatSvelteSnippetClosingBlock,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: svelte :: auxiliary :: snippet_closing_block :: FormatSvelteSnippetClosingBlock :: default ())
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::SvelteSnippetClosingBlock {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::SvelteSnippetClosingBlock,
        crate::svelte::auxiliary::snippet_closing_block::FormatSvelteSnippetClosingBlock,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: svelte :: auxiliary :: snippet_closing_block :: FormatSvelteSnippetClosingBlock :: default ())
    }
}
impl FormatRule<biome_html_syntax::SvelteSnippetOpeningBlock>
    for crate::svelte::auxiliary::snippet_opening_block::FormatSvelteSnippetOpeningBlock
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::SvelteSnippetOpeningBlock,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::SvelteSnippetOpeningBlock>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::SvelteSnippetOpeningBlock {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::SvelteSnippetOpeningBlock,
        crate::svelte::auxiliary::snippet_opening_block::FormatSvelteSnippetOpeningBlock,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: svelte :: auxiliary :: snippet_opening_block :: FormatSvelteSnippetOpeningBlock :: default ())
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::SvelteSnippetOpeningBlock {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::SvelteSnippetOpeningBlock,
        crate::svelte::auxiliary::snippet_opening_block::FormatSvelteSnippetOpeningBlock,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: svelte :: auxiliary :: snippet_opening_block :: FormatSvelteSnippetOpeningBlock :: default ())
    }
}
impl FormatRule<biome_html_syntax::SvelteSquareDestructuredName>
    for crate::svelte::auxiliary::square_destructured_name::FormatSvelteSquareDestructuredName
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::SvelteSquareDestructuredName,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::SvelteSquareDestructuredName>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::SvelteSquareDestructuredName {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::SvelteSquareDestructuredName,
        crate::svelte::auxiliary::square_destructured_name::FormatSvelteSquareDestructuredName,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: svelte :: auxiliary :: square_destructured_name :: FormatSvelteSquareDestructuredName :: default ())
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::SvelteSquareDestructuredName {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::SvelteSquareDestructuredName,
        crate::svelte::auxiliary::square_destructured_name::FormatSvelteSquareDestructuredName,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: svelte :: auxiliary :: square_destructured_name :: FormatSvelteSquareDestructuredName :: default ())
    }
}
impl FormatRule<biome_html_syntax::SvelteStyleDirective>
    for crate::svelte::auxiliary::style_directive::FormatSvelteStyleDirective
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::SvelteStyleDirective,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::SvelteStyleDirective>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::SvelteStyleDirective {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::SvelteStyleDirective,
        crate::svelte::auxiliary::style_directive::FormatSvelteStyleDirective,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::svelte::auxiliary::style_directive::FormatSvelteStyleDirective::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::SvelteStyleDirective {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::SvelteStyleDirective,
        crate::svelte::auxiliary::style_directive::FormatSvelteStyleDirective,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::svelte::auxiliary::style_directive::FormatSvelteStyleDirective::default(),
        )
    }
}
impl FormatRule<biome_html_syntax::SvelteTransitionDirective>
    for crate::svelte::auxiliary::transition_directive::FormatSvelteTransitionDirective
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::SvelteTransitionDirective,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::SvelteTransitionDirective>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::SvelteTransitionDirective {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::SvelteTransitionDirective,
        crate::svelte::auxiliary::transition_directive::FormatSvelteTransitionDirective,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: svelte :: auxiliary :: transition_directive :: FormatSvelteTransitionDirective :: default ())
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::SvelteTransitionDirective {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::SvelteTransitionDirective,
        crate::svelte::auxiliary::transition_directive::FormatSvelteTransitionDirective,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: svelte :: auxiliary :: transition_directive :: FormatSvelteTransitionDirective :: default ())
    }
}
impl FormatRule<biome_html_syntax::SvelteUseDirective>
    for crate::svelte::auxiliary::use_directive::FormatSvelteUseDirective
{
    type Context = HtmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_html_syntax::SvelteUseDirective,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_html_syntax::SvelteUseDirective>::fmt(self, node, f)
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::SvelteUseDirective {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::SvelteUseDirective,
        crate::svelte::auxiliary::use_directive::FormatSvelteUseDirective,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::svelte::auxiliary::use_directive::FormatSvelteUseDirective::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::SvelteUseDirective {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::SvelteUseDirective,
        crate::svelte::auxiliary::use_directive::FormatSvelteUseDirective,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::svelte::auxiliary::use_directive::FormatSvelteUseDirective::default(),
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
impl AsFormat<HtmlFormatContext> for biome_html_syntax::SvelteAwaitClausesList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::SvelteAwaitClausesList,
        crate::svelte::lists::await_clauses_list::FormatSvelteAwaitClausesList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::svelte::lists::await_clauses_list::FormatSvelteAwaitClausesList::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::SvelteAwaitClausesList {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::SvelteAwaitClausesList,
        crate::svelte::lists::await_clauses_list::FormatSvelteAwaitClausesList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::svelte::lists::await_clauses_list::FormatSvelteAwaitClausesList::default(),
        )
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::SvelteBindingAssignmentBindingList {
    type Format < 'a > = FormatRefWithRule < 'a , biome_html_syntax :: SvelteBindingAssignmentBindingList , crate :: svelte :: lists :: binding_assignment_binding_list :: FormatSvelteBindingAssignmentBindingList > ;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: svelte :: lists :: binding_assignment_binding_list :: FormatSvelteBindingAssignmentBindingList :: default ())
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::SvelteBindingAssignmentBindingList {
    type Format = FormatOwnedWithRule < biome_html_syntax :: SvelteBindingAssignmentBindingList , crate :: svelte :: lists :: binding_assignment_binding_list :: FormatSvelteBindingAssignmentBindingList > ;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: svelte :: lists :: binding_assignment_binding_list :: FormatSvelteBindingAssignmentBindingList :: default ())
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
impl AsFormat<HtmlFormatContext> for biome_html_syntax::SvelteDirectiveModifierList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::SvelteDirectiveModifierList,
        crate::svelte::lists::directive_modifier_list::FormatSvelteDirectiveModifierList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: svelte :: lists :: directive_modifier_list :: FormatSvelteDirectiveModifierList :: default ())
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::SvelteDirectiveModifierList {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::SvelteDirectiveModifierList,
        crate::svelte::lists::directive_modifier_list::FormatSvelteDirectiveModifierList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: svelte :: lists :: directive_modifier_list :: FormatSvelteDirectiveModifierList :: default ())
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
impl AsFormat<HtmlFormatContext> for biome_html_syntax::AnyHtmlComponentObjectName {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::AnyHtmlComponentObjectName,
        crate::html::any::component_object_name::FormatAnyHtmlComponentObjectName,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::html::any::component_object_name::FormatAnyHtmlComponentObjectName::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::AnyHtmlComponentObjectName {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::AnyHtmlComponentObjectName,
        crate::html::any::component_object_name::FormatAnyHtmlComponentObjectName,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::html::any::component_object_name::FormatAnyHtmlComponentObjectName::default(),
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
impl AsFormat<HtmlFormatContext> for biome_html_syntax::AnyHtmlTagName {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::AnyHtmlTagName,
        crate::html::any::tag_name::FormatAnyHtmlTagName,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::html::any::tag_name::FormatAnyHtmlTagName::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::AnyHtmlTagName {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::AnyHtmlTagName,
        crate::html::any::tag_name::FormatAnyHtmlTagName,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::html::any::tag_name::FormatAnyHtmlTagName::default(),
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
impl AsFormat<HtmlFormatContext> for biome_html_syntax::AnySvelteAwaitClauses {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::AnySvelteAwaitClauses,
        crate::svelte::any::await_clauses::FormatAnySvelteAwaitClauses,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::svelte::any::await_clauses::FormatAnySvelteAwaitClauses::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::AnySvelteAwaitClauses {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::AnySvelteAwaitClauses,
        crate::svelte::any::await_clauses::FormatAnySvelteAwaitClauses,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::svelte::any::await_clauses::FormatAnySvelteAwaitClauses::default(),
        )
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::AnySvelteBindingAssignmentBinding {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::AnySvelteBindingAssignmentBinding,
        crate::svelte::any::binding_assignment_binding::FormatAnySvelteBindingAssignmentBinding,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: svelte :: any :: binding_assignment_binding :: FormatAnySvelteBindingAssignmentBinding :: default ())
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::AnySvelteBindingAssignmentBinding {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::AnySvelteBindingAssignmentBinding,
        crate::svelte::any::binding_assignment_binding::FormatAnySvelteBindingAssignmentBinding,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: svelte :: any :: binding_assignment_binding :: FormatAnySvelteBindingAssignmentBinding :: default ())
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::AnySvelteBindingProperty {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::AnySvelteBindingProperty,
        crate::svelte::any::binding_property::FormatAnySvelteBindingProperty,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::svelte::any::binding_property::FormatAnySvelteBindingProperty::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::AnySvelteBindingProperty {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::AnySvelteBindingProperty,
        crate::svelte::any::binding_property::FormatAnySvelteBindingProperty,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::svelte::any::binding_property::FormatAnySvelteBindingProperty::default(),
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
impl AsFormat<HtmlFormatContext> for biome_html_syntax::AnySvelteBlockItem {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::AnySvelteBlockItem,
        crate::svelte::any::block_item::FormatAnySvelteBlockItem,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::svelte::any::block_item::FormatAnySvelteBlockItem::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::AnySvelteBlockItem {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::AnySvelteBlockItem,
        crate::svelte::any::block_item::FormatAnySvelteBlockItem,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::svelte::any::block_item::FormatAnySvelteBlockItem::default(),
        )
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::AnySvelteDestructuredName {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::AnySvelteDestructuredName,
        crate::svelte::any::destructured_name::FormatAnySvelteDestructuredName,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::svelte::any::destructured_name::FormatAnySvelteDestructuredName::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::AnySvelteDestructuredName {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::AnySvelteDestructuredName,
        crate::svelte::any::destructured_name::FormatAnySvelteDestructuredName,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::svelte::any::destructured_name::FormatAnySvelteDestructuredName::default(),
        )
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::AnySvelteDirective {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::AnySvelteDirective,
        crate::svelte::any::directive::FormatAnySvelteDirective,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::svelte::any::directive::FormatAnySvelteDirective::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::AnySvelteDirective {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::AnySvelteDirective,
        crate::svelte::any::directive::FormatAnySvelteDirective,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::svelte::any::directive::FormatAnySvelteDirective::default(),
        )
    }
}
impl AsFormat<HtmlFormatContext> for biome_html_syntax::AnySvelteEachName {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_html_syntax::AnySvelteEachName,
        crate::svelte::any::each_name::FormatAnySvelteEachName,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::svelte::any::each_name::FormatAnySvelteEachName::default(),
        )
    }
}
impl IntoFormat<HtmlFormatContext> for biome_html_syntax::AnySvelteEachName {
    type Format = FormatOwnedWithRule<
        biome_html_syntax::AnySvelteEachName,
        crate::svelte::any::each_name::FormatAnySvelteEachName,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::svelte::any::each_name::FormatAnySvelteEachName::default(),
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
