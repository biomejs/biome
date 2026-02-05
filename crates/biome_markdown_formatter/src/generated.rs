//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

#![allow(clippy::use_self)]
#![expect(clippy::default_constructed_unit_structs)]
use crate::{
    AsFormat, FormatBogusNodeRule, FormatNodeRule, IntoFormat, MarkdownFormatContext,
    MarkdownFormatter,
};
use biome_formatter::{FormatOwnedWithRule, FormatRefWithRule, FormatResult, FormatRule};
impl FormatRule<biome_markdown_syntax::MdBullet>
    for crate::markdown::auxiliary::bullet::FormatMdBullet
{
    type Context = MarkdownFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_markdown_syntax::MdBullet,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_markdown_syntax::MdBullet>::fmt(self, node, f)
    }
}
impl AsFormat<MarkdownFormatContext> for biome_markdown_syntax::MdBullet {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_markdown_syntax::MdBullet,
        crate::markdown::auxiliary::bullet::FormatMdBullet,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::markdown::auxiliary::bullet::FormatMdBullet::default(),
        )
    }
}
impl IntoFormat<MarkdownFormatContext> for biome_markdown_syntax::MdBullet {
    type Format = FormatOwnedWithRule<
        biome_markdown_syntax::MdBullet,
        crate::markdown::auxiliary::bullet::FormatMdBullet,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::markdown::auxiliary::bullet::FormatMdBullet::default(),
        )
    }
}
impl FormatRule<biome_markdown_syntax::MdBulletListItem>
    for crate::markdown::auxiliary::bullet_list_item::FormatMdBulletListItem
{
    type Context = MarkdownFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_markdown_syntax::MdBulletListItem,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_markdown_syntax::MdBulletListItem>::fmt(self, node, f)
    }
}
impl AsFormat<MarkdownFormatContext> for biome_markdown_syntax::MdBulletListItem {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_markdown_syntax::MdBulletListItem,
        crate::markdown::auxiliary::bullet_list_item::FormatMdBulletListItem,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::markdown::auxiliary::bullet_list_item::FormatMdBulletListItem::default(),
        )
    }
}
impl IntoFormat<MarkdownFormatContext> for biome_markdown_syntax::MdBulletListItem {
    type Format = FormatOwnedWithRule<
        biome_markdown_syntax::MdBulletListItem,
        crate::markdown::auxiliary::bullet_list_item::FormatMdBulletListItem,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::markdown::auxiliary::bullet_list_item::FormatMdBulletListItem::default(),
        )
    }
}
impl FormatRule<biome_markdown_syntax::MdDocument>
    for crate::markdown::auxiliary::document::FormatMdDocument
{
    type Context = MarkdownFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_markdown_syntax::MdDocument,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_markdown_syntax::MdDocument>::fmt(self, node, f)
    }
}
impl AsFormat<MarkdownFormatContext> for biome_markdown_syntax::MdDocument {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_markdown_syntax::MdDocument,
        crate::markdown::auxiliary::document::FormatMdDocument,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::markdown::auxiliary::document::FormatMdDocument::default(),
        )
    }
}
impl IntoFormat<MarkdownFormatContext> for biome_markdown_syntax::MdDocument {
    type Format = FormatOwnedWithRule<
        biome_markdown_syntax::MdDocument,
        crate::markdown::auxiliary::document::FormatMdDocument,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::markdown::auxiliary::document::FormatMdDocument::default(),
        )
    }
}
impl FormatRule<biome_markdown_syntax::MdFencedCodeBlock>
    for crate::markdown::auxiliary::fenced_code_block::FormatMdFencedCodeBlock
{
    type Context = MarkdownFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_markdown_syntax::MdFencedCodeBlock,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_markdown_syntax::MdFencedCodeBlock>::fmt(self, node, f)
    }
}
impl AsFormat<MarkdownFormatContext> for biome_markdown_syntax::MdFencedCodeBlock {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_markdown_syntax::MdFencedCodeBlock,
        crate::markdown::auxiliary::fenced_code_block::FormatMdFencedCodeBlock,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::markdown::auxiliary::fenced_code_block::FormatMdFencedCodeBlock::default(),
        )
    }
}
impl IntoFormat<MarkdownFormatContext> for biome_markdown_syntax::MdFencedCodeBlock {
    type Format = FormatOwnedWithRule<
        biome_markdown_syntax::MdFencedCodeBlock,
        crate::markdown::auxiliary::fenced_code_block::FormatMdFencedCodeBlock,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::markdown::auxiliary::fenced_code_block::FormatMdFencedCodeBlock::default(),
        )
    }
}
impl FormatRule<biome_markdown_syntax::MdHardLine>
    for crate::markdown::auxiliary::hard_line::FormatMdHardLine
{
    type Context = MarkdownFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_markdown_syntax::MdHardLine,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_markdown_syntax::MdHardLine>::fmt(self, node, f)
    }
}
impl AsFormat<MarkdownFormatContext> for biome_markdown_syntax::MdHardLine {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_markdown_syntax::MdHardLine,
        crate::markdown::auxiliary::hard_line::FormatMdHardLine,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::markdown::auxiliary::hard_line::FormatMdHardLine::default(),
        )
    }
}
impl IntoFormat<MarkdownFormatContext> for biome_markdown_syntax::MdHardLine {
    type Format = FormatOwnedWithRule<
        biome_markdown_syntax::MdHardLine,
        crate::markdown::auxiliary::hard_line::FormatMdHardLine,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::markdown::auxiliary::hard_line::FormatMdHardLine::default(),
        )
    }
}
impl FormatRule<biome_markdown_syntax::MdHash> for crate::markdown::auxiliary::hash::FormatMdHash {
    type Context = MarkdownFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_markdown_syntax::MdHash,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_markdown_syntax::MdHash>::fmt(self, node, f)
    }
}
impl AsFormat<MarkdownFormatContext> for biome_markdown_syntax::MdHash {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_markdown_syntax::MdHash,
        crate::markdown::auxiliary::hash::FormatMdHash,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::markdown::auxiliary::hash::FormatMdHash::default(),
        )
    }
}
impl IntoFormat<MarkdownFormatContext> for biome_markdown_syntax::MdHash {
    type Format = FormatOwnedWithRule<
        biome_markdown_syntax::MdHash,
        crate::markdown::auxiliary::hash::FormatMdHash,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::markdown::auxiliary::hash::FormatMdHash::default(),
        )
    }
}
impl FormatRule<biome_markdown_syntax::MdHeader>
    for crate::markdown::auxiliary::header::FormatMdHeader
{
    type Context = MarkdownFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_markdown_syntax::MdHeader,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_markdown_syntax::MdHeader>::fmt(self, node, f)
    }
}
impl AsFormat<MarkdownFormatContext> for biome_markdown_syntax::MdHeader {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_markdown_syntax::MdHeader,
        crate::markdown::auxiliary::header::FormatMdHeader,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::markdown::auxiliary::header::FormatMdHeader::default(),
        )
    }
}
impl IntoFormat<MarkdownFormatContext> for biome_markdown_syntax::MdHeader {
    type Format = FormatOwnedWithRule<
        biome_markdown_syntax::MdHeader,
        crate::markdown::auxiliary::header::FormatMdHeader,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::markdown::auxiliary::header::FormatMdHeader::default(),
        )
    }
}
impl FormatRule<biome_markdown_syntax::MdHtmlBlock>
    for crate::markdown::auxiliary::html_block::FormatMdHtmlBlock
{
    type Context = MarkdownFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_markdown_syntax::MdHtmlBlock,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_markdown_syntax::MdHtmlBlock>::fmt(self, node, f)
    }
}
impl AsFormat<MarkdownFormatContext> for biome_markdown_syntax::MdHtmlBlock {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_markdown_syntax::MdHtmlBlock,
        crate::markdown::auxiliary::html_block::FormatMdHtmlBlock,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::markdown::auxiliary::html_block::FormatMdHtmlBlock::default(),
        )
    }
}
impl IntoFormat<MarkdownFormatContext> for biome_markdown_syntax::MdHtmlBlock {
    type Format = FormatOwnedWithRule<
        biome_markdown_syntax::MdHtmlBlock,
        crate::markdown::auxiliary::html_block::FormatMdHtmlBlock,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::markdown::auxiliary::html_block::FormatMdHtmlBlock::default(),
        )
    }
}
impl FormatRule<biome_markdown_syntax::MdIndent>
    for crate::markdown::auxiliary::indent::FormatMdIndent
{
    type Context = MarkdownFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_markdown_syntax::MdIndent,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_markdown_syntax::MdIndent>::fmt(self, node, f)
    }
}
impl AsFormat<MarkdownFormatContext> for biome_markdown_syntax::MdIndent {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_markdown_syntax::MdIndent,
        crate::markdown::auxiliary::indent::FormatMdIndent,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::markdown::auxiliary::indent::FormatMdIndent::default(),
        )
    }
}
impl IntoFormat<MarkdownFormatContext> for biome_markdown_syntax::MdIndent {
    type Format = FormatOwnedWithRule<
        biome_markdown_syntax::MdIndent,
        crate::markdown::auxiliary::indent::FormatMdIndent,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::markdown::auxiliary::indent::FormatMdIndent::default(),
        )
    }
}
impl FormatRule<biome_markdown_syntax::MdIndentCodeBlock>
    for crate::markdown::auxiliary::indent_code_block::FormatMdIndentCodeBlock
{
    type Context = MarkdownFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_markdown_syntax::MdIndentCodeBlock,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_markdown_syntax::MdIndentCodeBlock>::fmt(self, node, f)
    }
}
impl AsFormat<MarkdownFormatContext> for biome_markdown_syntax::MdIndentCodeBlock {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_markdown_syntax::MdIndentCodeBlock,
        crate::markdown::auxiliary::indent_code_block::FormatMdIndentCodeBlock,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::markdown::auxiliary::indent_code_block::FormatMdIndentCodeBlock::default(),
        )
    }
}
impl IntoFormat<MarkdownFormatContext> for biome_markdown_syntax::MdIndentCodeBlock {
    type Format = FormatOwnedWithRule<
        biome_markdown_syntax::MdIndentCodeBlock,
        crate::markdown::auxiliary::indent_code_block::FormatMdIndentCodeBlock,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::markdown::auxiliary::indent_code_block::FormatMdIndentCodeBlock::default(),
        )
    }
}
impl FormatRule<biome_markdown_syntax::MdIndentedCodeLine>
    for crate::markdown::auxiliary::indented_code_line::FormatMdIndentedCodeLine
{
    type Context = MarkdownFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_markdown_syntax::MdIndentedCodeLine,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_markdown_syntax::MdIndentedCodeLine>::fmt(self, node, f)
    }
}
impl AsFormat<MarkdownFormatContext> for biome_markdown_syntax::MdIndentedCodeLine {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_markdown_syntax::MdIndentedCodeLine,
        crate::markdown::auxiliary::indented_code_line::FormatMdIndentedCodeLine,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::markdown::auxiliary::indented_code_line::FormatMdIndentedCodeLine::default(),
        )
    }
}
impl IntoFormat<MarkdownFormatContext> for biome_markdown_syntax::MdIndentedCodeLine {
    type Format = FormatOwnedWithRule<
        biome_markdown_syntax::MdIndentedCodeLine,
        crate::markdown::auxiliary::indented_code_line::FormatMdIndentedCodeLine,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::markdown::auxiliary::indented_code_line::FormatMdIndentedCodeLine::default(),
        )
    }
}
impl FormatRule<biome_markdown_syntax::MdInlineCode>
    for crate::markdown::auxiliary::inline_code::FormatMdInlineCode
{
    type Context = MarkdownFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_markdown_syntax::MdInlineCode,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_markdown_syntax::MdInlineCode>::fmt(self, node, f)
    }
}
impl AsFormat<MarkdownFormatContext> for biome_markdown_syntax::MdInlineCode {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_markdown_syntax::MdInlineCode,
        crate::markdown::auxiliary::inline_code::FormatMdInlineCode,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::markdown::auxiliary::inline_code::FormatMdInlineCode::default(),
        )
    }
}
impl IntoFormat<MarkdownFormatContext> for biome_markdown_syntax::MdInlineCode {
    type Format = FormatOwnedWithRule<
        biome_markdown_syntax::MdInlineCode,
        crate::markdown::auxiliary::inline_code::FormatMdInlineCode,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::markdown::auxiliary::inline_code::FormatMdInlineCode::default(),
        )
    }
}
impl FormatRule<biome_markdown_syntax::MdInlineEmphasis>
    for crate::markdown::auxiliary::inline_emphasis::FormatMdInlineEmphasis
{
    type Context = MarkdownFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_markdown_syntax::MdInlineEmphasis,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_markdown_syntax::MdInlineEmphasis>::fmt(self, node, f)
    }
}
impl AsFormat<MarkdownFormatContext> for biome_markdown_syntax::MdInlineEmphasis {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_markdown_syntax::MdInlineEmphasis,
        crate::markdown::auxiliary::inline_emphasis::FormatMdInlineEmphasis,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::markdown::auxiliary::inline_emphasis::FormatMdInlineEmphasis::default(),
        )
    }
}
impl IntoFormat<MarkdownFormatContext> for biome_markdown_syntax::MdInlineEmphasis {
    type Format = FormatOwnedWithRule<
        biome_markdown_syntax::MdInlineEmphasis,
        crate::markdown::auxiliary::inline_emphasis::FormatMdInlineEmphasis,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::markdown::auxiliary::inline_emphasis::FormatMdInlineEmphasis::default(),
        )
    }
}
impl FormatRule<biome_markdown_syntax::MdInlineImage>
    for crate::markdown::auxiliary::inline_image::FormatMdInlineImage
{
    type Context = MarkdownFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_markdown_syntax::MdInlineImage,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_markdown_syntax::MdInlineImage>::fmt(self, node, f)
    }
}
impl AsFormat<MarkdownFormatContext> for biome_markdown_syntax::MdInlineImage {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_markdown_syntax::MdInlineImage,
        crate::markdown::auxiliary::inline_image::FormatMdInlineImage,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::markdown::auxiliary::inline_image::FormatMdInlineImage::default(),
        )
    }
}
impl IntoFormat<MarkdownFormatContext> for biome_markdown_syntax::MdInlineImage {
    type Format = FormatOwnedWithRule<
        biome_markdown_syntax::MdInlineImage,
        crate::markdown::auxiliary::inline_image::FormatMdInlineImage,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::markdown::auxiliary::inline_image::FormatMdInlineImage::default(),
        )
    }
}
impl FormatRule<biome_markdown_syntax::MdInlineImageAlt>
    for crate::markdown::auxiliary::inline_image_alt::FormatMdInlineImageAlt
{
    type Context = MarkdownFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_markdown_syntax::MdInlineImageAlt,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_markdown_syntax::MdInlineImageAlt>::fmt(self, node, f)
    }
}
impl AsFormat<MarkdownFormatContext> for biome_markdown_syntax::MdInlineImageAlt {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_markdown_syntax::MdInlineImageAlt,
        crate::markdown::auxiliary::inline_image_alt::FormatMdInlineImageAlt,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::markdown::auxiliary::inline_image_alt::FormatMdInlineImageAlt::default(),
        )
    }
}
impl IntoFormat<MarkdownFormatContext> for biome_markdown_syntax::MdInlineImageAlt {
    type Format = FormatOwnedWithRule<
        biome_markdown_syntax::MdInlineImageAlt,
        crate::markdown::auxiliary::inline_image_alt::FormatMdInlineImageAlt,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::markdown::auxiliary::inline_image_alt::FormatMdInlineImageAlt::default(),
        )
    }
}
impl FormatRule<biome_markdown_syntax::MdInlineImageLink>
    for crate::markdown::auxiliary::inline_image_link::FormatMdInlineImageLink
{
    type Context = MarkdownFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_markdown_syntax::MdInlineImageLink,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_markdown_syntax::MdInlineImageLink>::fmt(self, node, f)
    }
}
impl AsFormat<MarkdownFormatContext> for biome_markdown_syntax::MdInlineImageLink {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_markdown_syntax::MdInlineImageLink,
        crate::markdown::auxiliary::inline_image_link::FormatMdInlineImageLink,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::markdown::auxiliary::inline_image_link::FormatMdInlineImageLink::default(),
        )
    }
}
impl IntoFormat<MarkdownFormatContext> for biome_markdown_syntax::MdInlineImageLink {
    type Format = FormatOwnedWithRule<
        biome_markdown_syntax::MdInlineImageLink,
        crate::markdown::auxiliary::inline_image_link::FormatMdInlineImageLink,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::markdown::auxiliary::inline_image_link::FormatMdInlineImageLink::default(),
        )
    }
}
impl FormatRule<biome_markdown_syntax::MdInlineImageSource>
    for crate::markdown::auxiliary::inline_image_source::FormatMdInlineImageSource
{
    type Context = MarkdownFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_markdown_syntax::MdInlineImageSource,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_markdown_syntax::MdInlineImageSource>::fmt(self, node, f)
    }
}
impl AsFormat<MarkdownFormatContext> for biome_markdown_syntax::MdInlineImageSource {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_markdown_syntax::MdInlineImageSource,
        crate::markdown::auxiliary::inline_image_source::FormatMdInlineImageSource,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::markdown::auxiliary::inline_image_source::FormatMdInlineImageSource::default(),
        )
    }
}
impl IntoFormat<MarkdownFormatContext> for biome_markdown_syntax::MdInlineImageSource {
    type Format = FormatOwnedWithRule<
        biome_markdown_syntax::MdInlineImageSource,
        crate::markdown::auxiliary::inline_image_source::FormatMdInlineImageSource,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::markdown::auxiliary::inline_image_source::FormatMdInlineImageSource::default(),
        )
    }
}
impl FormatRule<biome_markdown_syntax::MdInlineItalic>
    for crate::markdown::auxiliary::inline_italic::FormatMdInlineItalic
{
    type Context = MarkdownFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_markdown_syntax::MdInlineItalic,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_markdown_syntax::MdInlineItalic>::fmt(self, node, f)
    }
}
impl AsFormat<MarkdownFormatContext> for biome_markdown_syntax::MdInlineItalic {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_markdown_syntax::MdInlineItalic,
        crate::markdown::auxiliary::inline_italic::FormatMdInlineItalic,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::markdown::auxiliary::inline_italic::FormatMdInlineItalic::default(),
        )
    }
}
impl IntoFormat<MarkdownFormatContext> for biome_markdown_syntax::MdInlineItalic {
    type Format = FormatOwnedWithRule<
        biome_markdown_syntax::MdInlineItalic,
        crate::markdown::auxiliary::inline_italic::FormatMdInlineItalic,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::markdown::auxiliary::inline_italic::FormatMdInlineItalic::default(),
        )
    }
}
impl FormatRule<biome_markdown_syntax::MdInlineLink>
    for crate::markdown::auxiliary::inline_link::FormatMdInlineLink
{
    type Context = MarkdownFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_markdown_syntax::MdInlineLink,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_markdown_syntax::MdInlineLink>::fmt(self, node, f)
    }
}
impl AsFormat<MarkdownFormatContext> for biome_markdown_syntax::MdInlineLink {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_markdown_syntax::MdInlineLink,
        crate::markdown::auxiliary::inline_link::FormatMdInlineLink,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::markdown::auxiliary::inline_link::FormatMdInlineLink::default(),
        )
    }
}
impl IntoFormat<MarkdownFormatContext> for biome_markdown_syntax::MdInlineLink {
    type Format = FormatOwnedWithRule<
        biome_markdown_syntax::MdInlineLink,
        crate::markdown::auxiliary::inline_link::FormatMdInlineLink,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::markdown::auxiliary::inline_link::FormatMdInlineLink::default(),
        )
    }
}
impl FormatRule<biome_markdown_syntax::MdLinkBlock>
    for crate::markdown::auxiliary::link_block::FormatMdLinkBlock
{
    type Context = MarkdownFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_markdown_syntax::MdLinkBlock,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_markdown_syntax::MdLinkBlock>::fmt(self, node, f)
    }
}
impl AsFormat<MarkdownFormatContext> for biome_markdown_syntax::MdLinkBlock {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_markdown_syntax::MdLinkBlock,
        crate::markdown::auxiliary::link_block::FormatMdLinkBlock,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::markdown::auxiliary::link_block::FormatMdLinkBlock::default(),
        )
    }
}
impl IntoFormat<MarkdownFormatContext> for biome_markdown_syntax::MdLinkBlock {
    type Format = FormatOwnedWithRule<
        biome_markdown_syntax::MdLinkBlock,
        crate::markdown::auxiliary::link_block::FormatMdLinkBlock,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::markdown::auxiliary::link_block::FormatMdLinkBlock::default(),
        )
    }
}
impl FormatRule<biome_markdown_syntax::MdOrderListItem>
    for crate::markdown::auxiliary::order_list_item::FormatMdOrderListItem
{
    type Context = MarkdownFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_markdown_syntax::MdOrderListItem,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_markdown_syntax::MdOrderListItem>::fmt(self, node, f)
    }
}
impl AsFormat<MarkdownFormatContext> for biome_markdown_syntax::MdOrderListItem {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_markdown_syntax::MdOrderListItem,
        crate::markdown::auxiliary::order_list_item::FormatMdOrderListItem,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::markdown::auxiliary::order_list_item::FormatMdOrderListItem::default(),
        )
    }
}
impl IntoFormat<MarkdownFormatContext> for biome_markdown_syntax::MdOrderListItem {
    type Format = FormatOwnedWithRule<
        biome_markdown_syntax::MdOrderListItem,
        crate::markdown::auxiliary::order_list_item::FormatMdOrderListItem,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::markdown::auxiliary::order_list_item::FormatMdOrderListItem::default(),
        )
    }
}
impl FormatRule<biome_markdown_syntax::MdParagraph>
    for crate::markdown::auxiliary::paragraph::FormatMdParagraph
{
    type Context = MarkdownFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_markdown_syntax::MdParagraph,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_markdown_syntax::MdParagraph>::fmt(self, node, f)
    }
}
impl AsFormat<MarkdownFormatContext> for biome_markdown_syntax::MdParagraph {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_markdown_syntax::MdParagraph,
        crate::markdown::auxiliary::paragraph::FormatMdParagraph,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::markdown::auxiliary::paragraph::FormatMdParagraph::default(),
        )
    }
}
impl IntoFormat<MarkdownFormatContext> for biome_markdown_syntax::MdParagraph {
    type Format = FormatOwnedWithRule<
        biome_markdown_syntax::MdParagraph,
        crate::markdown::auxiliary::paragraph::FormatMdParagraph,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::markdown::auxiliary::paragraph::FormatMdParagraph::default(),
        )
    }
}
impl FormatRule<biome_markdown_syntax::MdQuote>
    for crate::markdown::auxiliary::quote::FormatMdQuote
{
    type Context = MarkdownFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_markdown_syntax::MdQuote,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_markdown_syntax::MdQuote>::fmt(self, node, f)
    }
}
impl AsFormat<MarkdownFormatContext> for biome_markdown_syntax::MdQuote {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_markdown_syntax::MdQuote,
        crate::markdown::auxiliary::quote::FormatMdQuote,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::markdown::auxiliary::quote::FormatMdQuote::default(),
        )
    }
}
impl IntoFormat<MarkdownFormatContext> for biome_markdown_syntax::MdQuote {
    type Format = FormatOwnedWithRule<
        biome_markdown_syntax::MdQuote,
        crate::markdown::auxiliary::quote::FormatMdQuote,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::markdown::auxiliary::quote::FormatMdQuote::default(),
        )
    }
}
impl FormatRule<biome_markdown_syntax::MdSetextHeader>
    for crate::markdown::auxiliary::setext_header::FormatMdSetextHeader
{
    type Context = MarkdownFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_markdown_syntax::MdSetextHeader,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_markdown_syntax::MdSetextHeader>::fmt(self, node, f)
    }
}
impl AsFormat<MarkdownFormatContext> for biome_markdown_syntax::MdSetextHeader {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_markdown_syntax::MdSetextHeader,
        crate::markdown::auxiliary::setext_header::FormatMdSetextHeader,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::markdown::auxiliary::setext_header::FormatMdSetextHeader::default(),
        )
    }
}
impl IntoFormat<MarkdownFormatContext> for biome_markdown_syntax::MdSetextHeader {
    type Format = FormatOwnedWithRule<
        biome_markdown_syntax::MdSetextHeader,
        crate::markdown::auxiliary::setext_header::FormatMdSetextHeader,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::markdown::auxiliary::setext_header::FormatMdSetextHeader::default(),
        )
    }
}
impl FormatRule<biome_markdown_syntax::MdSoftBreak>
    for crate::markdown::auxiliary::soft_break::FormatMdSoftBreak
{
    type Context = MarkdownFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_markdown_syntax::MdSoftBreak,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_markdown_syntax::MdSoftBreak>::fmt(self, node, f)
    }
}
impl AsFormat<MarkdownFormatContext> for biome_markdown_syntax::MdSoftBreak {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_markdown_syntax::MdSoftBreak,
        crate::markdown::auxiliary::soft_break::FormatMdSoftBreak,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::markdown::auxiliary::soft_break::FormatMdSoftBreak::default(),
        )
    }
}
impl IntoFormat<MarkdownFormatContext> for biome_markdown_syntax::MdSoftBreak {
    type Format = FormatOwnedWithRule<
        biome_markdown_syntax::MdSoftBreak,
        crate::markdown::auxiliary::soft_break::FormatMdSoftBreak,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::markdown::auxiliary::soft_break::FormatMdSoftBreak::default(),
        )
    }
}
impl FormatRule<biome_markdown_syntax::MdTextual>
    for crate::markdown::auxiliary::textual::FormatMdTextual
{
    type Context = MarkdownFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_markdown_syntax::MdTextual,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_markdown_syntax::MdTextual>::fmt(self, node, f)
    }
}
impl AsFormat<MarkdownFormatContext> for biome_markdown_syntax::MdTextual {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_markdown_syntax::MdTextual,
        crate::markdown::auxiliary::textual::FormatMdTextual,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::markdown::auxiliary::textual::FormatMdTextual::default(),
        )
    }
}
impl IntoFormat<MarkdownFormatContext> for biome_markdown_syntax::MdTextual {
    type Format = FormatOwnedWithRule<
        biome_markdown_syntax::MdTextual,
        crate::markdown::auxiliary::textual::FormatMdTextual,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::markdown::auxiliary::textual::FormatMdTextual::default(),
        )
    }
}
impl FormatRule<biome_markdown_syntax::MdThematicBreakBlock>
    for crate::markdown::auxiliary::thematic_break_block::FormatMdThematicBreakBlock
{
    type Context = MarkdownFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_markdown_syntax::MdThematicBreakBlock,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_markdown_syntax::MdThematicBreakBlock>::fmt(self, node, f)
    }
}
impl AsFormat<MarkdownFormatContext> for biome_markdown_syntax::MdThematicBreakBlock {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_markdown_syntax::MdThematicBreakBlock,
        crate::markdown::auxiliary::thematic_break_block::FormatMdThematicBreakBlock,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::markdown::auxiliary::thematic_break_block::FormatMdThematicBreakBlock::default(),
        )
    }
}
impl IntoFormat<MarkdownFormatContext> for biome_markdown_syntax::MdThematicBreakBlock {
    type Format = FormatOwnedWithRule<
        biome_markdown_syntax::MdThematicBreakBlock,
        crate::markdown::auxiliary::thematic_break_block::FormatMdThematicBreakBlock,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::markdown::auxiliary::thematic_break_block::FormatMdThematicBreakBlock::default(),
        )
    }
}
impl AsFormat<MarkdownFormatContext> for biome_markdown_syntax::MdBlockList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_markdown_syntax::MdBlockList,
        crate::markdown::lists::block_list::FormatMdBlockList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::markdown::lists::block_list::FormatMdBlockList::default(),
        )
    }
}
impl IntoFormat<MarkdownFormatContext> for biome_markdown_syntax::MdBlockList {
    type Format = FormatOwnedWithRule<
        biome_markdown_syntax::MdBlockList,
        crate::markdown::lists::block_list::FormatMdBlockList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::markdown::lists::block_list::FormatMdBlockList::default(),
        )
    }
}
impl AsFormat<MarkdownFormatContext> for biome_markdown_syntax::MdBulletList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_markdown_syntax::MdBulletList,
        crate::markdown::lists::bullet_list::FormatMdBulletList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::markdown::lists::bullet_list::FormatMdBulletList::default(),
        )
    }
}
impl IntoFormat<MarkdownFormatContext> for biome_markdown_syntax::MdBulletList {
    type Format = FormatOwnedWithRule<
        biome_markdown_syntax::MdBulletList,
        crate::markdown::lists::bullet_list::FormatMdBulletList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::markdown::lists::bullet_list::FormatMdBulletList::default(),
        )
    }
}
impl AsFormat<MarkdownFormatContext> for biome_markdown_syntax::MdCodeNameList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_markdown_syntax::MdCodeNameList,
        crate::markdown::lists::code_name_list::FormatMdCodeNameList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::markdown::lists::code_name_list::FormatMdCodeNameList::default(),
        )
    }
}
impl IntoFormat<MarkdownFormatContext> for biome_markdown_syntax::MdCodeNameList {
    type Format = FormatOwnedWithRule<
        biome_markdown_syntax::MdCodeNameList,
        crate::markdown::lists::code_name_list::FormatMdCodeNameList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::markdown::lists::code_name_list::FormatMdCodeNameList::default(),
        )
    }
}
impl AsFormat<MarkdownFormatContext> for biome_markdown_syntax::MdHashList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_markdown_syntax::MdHashList,
        crate::markdown::lists::hash_list::FormatMdHashList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::markdown::lists::hash_list::FormatMdHashList::default(),
        )
    }
}
impl IntoFormat<MarkdownFormatContext> for biome_markdown_syntax::MdHashList {
    type Format = FormatOwnedWithRule<
        biome_markdown_syntax::MdHashList,
        crate::markdown::lists::hash_list::FormatMdHashList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::markdown::lists::hash_list::FormatMdHashList::default(),
        )
    }
}
impl AsFormat<MarkdownFormatContext> for biome_markdown_syntax::MdIndentedCodeLineList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_markdown_syntax::MdIndentedCodeLineList,
        crate::markdown::lists::indented_code_line_list::FormatMdIndentedCodeLineList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::markdown::lists::indented_code_line_list::FormatMdIndentedCodeLineList::default(
            ),
        )
    }
}
impl IntoFormat<MarkdownFormatContext> for biome_markdown_syntax::MdIndentedCodeLineList {
    type Format = FormatOwnedWithRule<
        biome_markdown_syntax::MdIndentedCodeLineList,
        crate::markdown::lists::indented_code_line_list::FormatMdIndentedCodeLineList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::markdown::lists::indented_code_line_list::FormatMdIndentedCodeLineList::default(
            ),
        )
    }
}
impl AsFormat<MarkdownFormatContext> for biome_markdown_syntax::MdInlineItemList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_markdown_syntax::MdInlineItemList,
        crate::markdown::lists::inline_item_list::FormatMdInlineItemList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::markdown::lists::inline_item_list::FormatMdInlineItemList::default(),
        )
    }
}
impl IntoFormat<MarkdownFormatContext> for biome_markdown_syntax::MdInlineItemList {
    type Format = FormatOwnedWithRule<
        biome_markdown_syntax::MdInlineItemList,
        crate::markdown::lists::inline_item_list::FormatMdInlineItemList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::markdown::lists::inline_item_list::FormatMdInlineItemList::default(),
        )
    }
}
impl AsFormat<MarkdownFormatContext> for biome_markdown_syntax::MdOrderList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_markdown_syntax::MdOrderList,
        crate::markdown::lists::order_list::FormatMdOrderList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::markdown::lists::order_list::FormatMdOrderList::default(),
        )
    }
}
impl IntoFormat<MarkdownFormatContext> for biome_markdown_syntax::MdOrderList {
    type Format = FormatOwnedWithRule<
        biome_markdown_syntax::MdOrderList,
        crate::markdown::lists::order_list::FormatMdOrderList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::markdown::lists::order_list::FormatMdOrderList::default(),
        )
    }
}
impl FormatRule<biome_markdown_syntax::MdBogus> for crate::markdown::bogus::bogus::FormatMdBogus {
    type Context = MarkdownFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_markdown_syntax::MdBogus,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_markdown_syntax::MdBogus>::fmt(self, node, f)
    }
}
impl AsFormat<MarkdownFormatContext> for biome_markdown_syntax::MdBogus {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_markdown_syntax::MdBogus,
        crate::markdown::bogus::bogus::FormatMdBogus,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::markdown::bogus::bogus::FormatMdBogus::default(),
        )
    }
}
impl IntoFormat<MarkdownFormatContext> for biome_markdown_syntax::MdBogus {
    type Format = FormatOwnedWithRule<
        biome_markdown_syntax::MdBogus,
        crate::markdown::bogus::bogus::FormatMdBogus,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::markdown::bogus::bogus::FormatMdBogus::default(),
        )
    }
}
impl AsFormat<MarkdownFormatContext> for biome_markdown_syntax::AnyMdBlock {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_markdown_syntax::AnyMdBlock,
        crate::markdown::any::block::FormatAnyMdBlock,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::markdown::any::block::FormatAnyMdBlock::default(),
        )
    }
}
impl IntoFormat<MarkdownFormatContext> for biome_markdown_syntax::AnyMdBlock {
    type Format = FormatOwnedWithRule<
        biome_markdown_syntax::AnyMdBlock,
        crate::markdown::any::block::FormatAnyMdBlock,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::markdown::any::block::FormatAnyMdBlock::default(),
        )
    }
}
impl AsFormat<MarkdownFormatContext> for biome_markdown_syntax::AnyMdCodeBlock {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_markdown_syntax::AnyMdCodeBlock,
        crate::markdown::any::code_block::FormatAnyMdCodeBlock,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::markdown::any::code_block::FormatAnyMdCodeBlock::default(),
        )
    }
}
impl IntoFormat<MarkdownFormatContext> for biome_markdown_syntax::AnyMdCodeBlock {
    type Format = FormatOwnedWithRule<
        biome_markdown_syntax::AnyMdCodeBlock,
        crate::markdown::any::code_block::FormatAnyMdCodeBlock,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::markdown::any::code_block::FormatAnyMdCodeBlock::default(),
        )
    }
}
impl AsFormat<MarkdownFormatContext> for biome_markdown_syntax::AnyMdContainerBlock {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_markdown_syntax::AnyMdContainerBlock,
        crate::markdown::any::container_block::FormatAnyMdContainerBlock,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::markdown::any::container_block::FormatAnyMdContainerBlock::default(),
        )
    }
}
impl IntoFormat<MarkdownFormatContext> for biome_markdown_syntax::AnyMdContainerBlock {
    type Format = FormatOwnedWithRule<
        biome_markdown_syntax::AnyMdContainerBlock,
        crate::markdown::any::container_block::FormatAnyMdContainerBlock,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::markdown::any::container_block::FormatAnyMdContainerBlock::default(),
        )
    }
}
impl AsFormat<MarkdownFormatContext> for biome_markdown_syntax::AnyMdInline {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_markdown_syntax::AnyMdInline,
        crate::markdown::any::inline::FormatAnyMdInline,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::markdown::any::inline::FormatAnyMdInline::default(),
        )
    }
}
impl IntoFormat<MarkdownFormatContext> for biome_markdown_syntax::AnyMdInline {
    type Format = FormatOwnedWithRule<
        biome_markdown_syntax::AnyMdInline,
        crate::markdown::any::inline::FormatAnyMdInline,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::markdown::any::inline::FormatAnyMdInline::default(),
        )
    }
}
impl AsFormat<MarkdownFormatContext> for biome_markdown_syntax::AnyMdLeafBlock {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_markdown_syntax::AnyMdLeafBlock,
        crate::markdown::any::leaf_block::FormatAnyMdLeafBlock,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::markdown::any::leaf_block::FormatAnyMdLeafBlock::default(),
        )
    }
}
impl IntoFormat<MarkdownFormatContext> for biome_markdown_syntax::AnyMdLeafBlock {
    type Format = FormatOwnedWithRule<
        biome_markdown_syntax::AnyMdLeafBlock,
        crate::markdown::any::leaf_block::FormatAnyMdLeafBlock,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::markdown::any::leaf_block::FormatAnyMdLeafBlock::default(),
        )
    }
}
