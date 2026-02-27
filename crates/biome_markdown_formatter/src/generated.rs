//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

#![allow(clippy::use_self)]
#![expect(clippy::default_constructed_unit_structs)]
use crate::{
    AsFormat, FormatBogusNodeRule, FormatNodeRule, IntoFormat, MarkdownFormatContext,
    MarkdownFormatter,
};
use biome_formatter::{FormatOwnedWithRule, FormatRefWithRule, FormatResult, FormatRule};
impl FormatRule<biome_markdown_syntax::MdAutolink>
    for crate::markdown::auxiliary::autolink::FormatMdAutolink
{
    type Context = MarkdownFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_markdown_syntax::MdAutolink,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_markdown_syntax::MdAutolink>::fmt(self, node, f)
    }
}
impl AsFormat<MarkdownFormatContext> for biome_markdown_syntax::MdAutolink {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_markdown_syntax::MdAutolink,
        crate::markdown::auxiliary::autolink::FormatMdAutolink,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::markdown::auxiliary::autolink::FormatMdAutolink::default(),
        )
    }
}
impl IntoFormat<MarkdownFormatContext> for biome_markdown_syntax::MdAutolink {
    type Format = FormatOwnedWithRule<
        biome_markdown_syntax::MdAutolink,
        crate::markdown::auxiliary::autolink::FormatMdAutolink,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::markdown::auxiliary::autolink::FormatMdAutolink::default(),
        )
    }
}
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
impl FormatRule<biome_markdown_syntax::MdEntityReference>
    for crate::markdown::auxiliary::entity_reference::FormatMdEntityReference
{
    type Context = MarkdownFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_markdown_syntax::MdEntityReference,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_markdown_syntax::MdEntityReference>::fmt(self, node, f)
    }
}
impl AsFormat<MarkdownFormatContext> for biome_markdown_syntax::MdEntityReference {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_markdown_syntax::MdEntityReference,
        crate::markdown::auxiliary::entity_reference::FormatMdEntityReference,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::markdown::auxiliary::entity_reference::FormatMdEntityReference::default(),
        )
    }
}
impl IntoFormat<MarkdownFormatContext> for biome_markdown_syntax::MdEntityReference {
    type Format = FormatOwnedWithRule<
        biome_markdown_syntax::MdEntityReference,
        crate::markdown::auxiliary::entity_reference::FormatMdEntityReference,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::markdown::auxiliary::entity_reference::FormatMdEntityReference::default(),
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
impl FormatRule<biome_markdown_syntax::MdInlineHtml>
    for crate::markdown::auxiliary::inline_html::FormatMdInlineHtml
{
    type Context = MarkdownFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_markdown_syntax::MdInlineHtml,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_markdown_syntax::MdInlineHtml>::fmt(self, node, f)
    }
}
impl AsFormat<MarkdownFormatContext> for biome_markdown_syntax::MdInlineHtml {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_markdown_syntax::MdInlineHtml,
        crate::markdown::auxiliary::inline_html::FormatMdInlineHtml,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::markdown::auxiliary::inline_html::FormatMdInlineHtml::default(),
        )
    }
}
impl IntoFormat<MarkdownFormatContext> for biome_markdown_syntax::MdInlineHtml {
    type Format = FormatOwnedWithRule<
        biome_markdown_syntax::MdInlineHtml,
        crate::markdown::auxiliary::inline_html::FormatMdInlineHtml,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::markdown::auxiliary::inline_html::FormatMdInlineHtml::default(),
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
impl FormatRule<biome_markdown_syntax::MdLinkDestination>
    for crate::markdown::auxiliary::link_destination::FormatMdLinkDestination
{
    type Context = MarkdownFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_markdown_syntax::MdLinkDestination,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_markdown_syntax::MdLinkDestination>::fmt(self, node, f)
    }
}
impl AsFormat<MarkdownFormatContext> for biome_markdown_syntax::MdLinkDestination {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_markdown_syntax::MdLinkDestination,
        crate::markdown::auxiliary::link_destination::FormatMdLinkDestination,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::markdown::auxiliary::link_destination::FormatMdLinkDestination::default(),
        )
    }
}
impl IntoFormat<MarkdownFormatContext> for biome_markdown_syntax::MdLinkDestination {
    type Format = FormatOwnedWithRule<
        biome_markdown_syntax::MdLinkDestination,
        crate::markdown::auxiliary::link_destination::FormatMdLinkDestination,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::markdown::auxiliary::link_destination::FormatMdLinkDestination::default(),
        )
    }
}
impl FormatRule<biome_markdown_syntax::MdLinkLabel>
    for crate::markdown::auxiliary::link_label::FormatMdLinkLabel
{
    type Context = MarkdownFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_markdown_syntax::MdLinkLabel,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_markdown_syntax::MdLinkLabel>::fmt(self, node, f)
    }
}
impl AsFormat<MarkdownFormatContext> for biome_markdown_syntax::MdLinkLabel {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_markdown_syntax::MdLinkLabel,
        crate::markdown::auxiliary::link_label::FormatMdLinkLabel,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::markdown::auxiliary::link_label::FormatMdLinkLabel::default(),
        )
    }
}
impl IntoFormat<MarkdownFormatContext> for biome_markdown_syntax::MdLinkLabel {
    type Format = FormatOwnedWithRule<
        biome_markdown_syntax::MdLinkLabel,
        crate::markdown::auxiliary::link_label::FormatMdLinkLabel,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::markdown::auxiliary::link_label::FormatMdLinkLabel::default(),
        )
    }
}
impl FormatRule<biome_markdown_syntax::MdLinkReferenceDefinition>
    for crate::markdown::auxiliary::link_reference_definition::FormatMdLinkReferenceDefinition
{
    type Context = MarkdownFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_markdown_syntax::MdLinkReferenceDefinition,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_markdown_syntax::MdLinkReferenceDefinition>::fmt(self, node, f)
    }
}
impl AsFormat<MarkdownFormatContext> for biome_markdown_syntax::MdLinkReferenceDefinition {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_markdown_syntax::MdLinkReferenceDefinition,
        crate::markdown::auxiliary::link_reference_definition::FormatMdLinkReferenceDefinition,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: markdown :: auxiliary :: link_reference_definition :: FormatMdLinkReferenceDefinition :: default ())
    }
}
impl IntoFormat<MarkdownFormatContext> for biome_markdown_syntax::MdLinkReferenceDefinition {
    type Format = FormatOwnedWithRule<
        biome_markdown_syntax::MdLinkReferenceDefinition,
        crate::markdown::auxiliary::link_reference_definition::FormatMdLinkReferenceDefinition,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: markdown :: auxiliary :: link_reference_definition :: FormatMdLinkReferenceDefinition :: default ())
    }
}
impl FormatRule<biome_markdown_syntax::MdLinkTitle>
    for crate::markdown::auxiliary::link_title::FormatMdLinkTitle
{
    type Context = MarkdownFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_markdown_syntax::MdLinkTitle,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_markdown_syntax::MdLinkTitle>::fmt(self, node, f)
    }
}
impl AsFormat<MarkdownFormatContext> for biome_markdown_syntax::MdLinkTitle {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_markdown_syntax::MdLinkTitle,
        crate::markdown::auxiliary::link_title::FormatMdLinkTitle,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::markdown::auxiliary::link_title::FormatMdLinkTitle::default(),
        )
    }
}
impl IntoFormat<MarkdownFormatContext> for biome_markdown_syntax::MdLinkTitle {
    type Format = FormatOwnedWithRule<
        biome_markdown_syntax::MdLinkTitle,
        crate::markdown::auxiliary::link_title::FormatMdLinkTitle,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::markdown::auxiliary::link_title::FormatMdLinkTitle::default(),
        )
    }
}
impl FormatRule<biome_markdown_syntax::MdNewline>
    for crate::markdown::auxiliary::newline::FormatMdNewline
{
    type Context = MarkdownFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_markdown_syntax::MdNewline,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_markdown_syntax::MdNewline>::fmt(self, node, f)
    }
}
impl AsFormat<MarkdownFormatContext> for biome_markdown_syntax::MdNewline {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_markdown_syntax::MdNewline,
        crate::markdown::auxiliary::newline::FormatMdNewline,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::markdown::auxiliary::newline::FormatMdNewline::default(),
        )
    }
}
impl IntoFormat<MarkdownFormatContext> for biome_markdown_syntax::MdNewline {
    type Format = FormatOwnedWithRule<
        biome_markdown_syntax::MdNewline,
        crate::markdown::auxiliary::newline::FormatMdNewline,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::markdown::auxiliary::newline::FormatMdNewline::default(),
        )
    }
}
impl FormatRule<biome_markdown_syntax::MdOrderedListItem>
    for crate::markdown::auxiliary::ordered_list_item::FormatMdOrderedListItem
{
    type Context = MarkdownFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_markdown_syntax::MdOrderedListItem,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_markdown_syntax::MdOrderedListItem>::fmt(self, node, f)
    }
}
impl AsFormat<MarkdownFormatContext> for biome_markdown_syntax::MdOrderedListItem {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_markdown_syntax::MdOrderedListItem,
        crate::markdown::auxiliary::ordered_list_item::FormatMdOrderedListItem,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::markdown::auxiliary::ordered_list_item::FormatMdOrderedListItem::default(),
        )
    }
}
impl IntoFormat<MarkdownFormatContext> for biome_markdown_syntax::MdOrderedListItem {
    type Format = FormatOwnedWithRule<
        biome_markdown_syntax::MdOrderedListItem,
        crate::markdown::auxiliary::ordered_list_item::FormatMdOrderedListItem,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::markdown::auxiliary::ordered_list_item::FormatMdOrderedListItem::default(),
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
impl FormatRule<biome_markdown_syntax::MdQuoteIndent>
    for crate::markdown::auxiliary::quote_indent::FormatMdQuoteIndent
{
    type Context = MarkdownFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_markdown_syntax::MdQuoteIndent,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_markdown_syntax::MdQuoteIndent>::fmt(self, node, f)
    }
}
impl AsFormat<MarkdownFormatContext> for biome_markdown_syntax::MdQuoteIndent {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_markdown_syntax::MdQuoteIndent,
        crate::markdown::auxiliary::quote_indent::FormatMdQuoteIndent,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::markdown::auxiliary::quote_indent::FormatMdQuoteIndent::default(),
        )
    }
}
impl IntoFormat<MarkdownFormatContext> for biome_markdown_syntax::MdQuoteIndent {
    type Format = FormatOwnedWithRule<
        biome_markdown_syntax::MdQuoteIndent,
        crate::markdown::auxiliary::quote_indent::FormatMdQuoteIndent,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::markdown::auxiliary::quote_indent::FormatMdQuoteIndent::default(),
        )
    }
}
impl FormatRule<biome_markdown_syntax::MdQuotePrefix>
    for crate::markdown::auxiliary::quote_prefix::FormatMdQuotePrefix
{
    type Context = MarkdownFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_markdown_syntax::MdQuotePrefix,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_markdown_syntax::MdQuotePrefix>::fmt(self, node, f)
    }
}
impl AsFormat<MarkdownFormatContext> for biome_markdown_syntax::MdQuotePrefix {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_markdown_syntax::MdQuotePrefix,
        crate::markdown::auxiliary::quote_prefix::FormatMdQuotePrefix,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::markdown::auxiliary::quote_prefix::FormatMdQuotePrefix::default(),
        )
    }
}
impl IntoFormat<MarkdownFormatContext> for biome_markdown_syntax::MdQuotePrefix {
    type Format = FormatOwnedWithRule<
        biome_markdown_syntax::MdQuotePrefix,
        crate::markdown::auxiliary::quote_prefix::FormatMdQuotePrefix,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::markdown::auxiliary::quote_prefix::FormatMdQuotePrefix::default(),
        )
    }
}
impl FormatRule<biome_markdown_syntax::MdReferenceImage>
    for crate::markdown::auxiliary::reference_image::FormatMdReferenceImage
{
    type Context = MarkdownFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_markdown_syntax::MdReferenceImage,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_markdown_syntax::MdReferenceImage>::fmt(self, node, f)
    }
}
impl AsFormat<MarkdownFormatContext> for biome_markdown_syntax::MdReferenceImage {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_markdown_syntax::MdReferenceImage,
        crate::markdown::auxiliary::reference_image::FormatMdReferenceImage,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::markdown::auxiliary::reference_image::FormatMdReferenceImage::default(),
        )
    }
}
impl IntoFormat<MarkdownFormatContext> for biome_markdown_syntax::MdReferenceImage {
    type Format = FormatOwnedWithRule<
        biome_markdown_syntax::MdReferenceImage,
        crate::markdown::auxiliary::reference_image::FormatMdReferenceImage,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::markdown::auxiliary::reference_image::FormatMdReferenceImage::default(),
        )
    }
}
impl FormatRule<biome_markdown_syntax::MdReferenceLink>
    for crate::markdown::auxiliary::reference_link::FormatMdReferenceLink
{
    type Context = MarkdownFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_markdown_syntax::MdReferenceLink,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_markdown_syntax::MdReferenceLink>::fmt(self, node, f)
    }
}
impl AsFormat<MarkdownFormatContext> for biome_markdown_syntax::MdReferenceLink {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_markdown_syntax::MdReferenceLink,
        crate::markdown::auxiliary::reference_link::FormatMdReferenceLink,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::markdown::auxiliary::reference_link::FormatMdReferenceLink::default(),
        )
    }
}
impl IntoFormat<MarkdownFormatContext> for biome_markdown_syntax::MdReferenceLink {
    type Format = FormatOwnedWithRule<
        biome_markdown_syntax::MdReferenceLink,
        crate::markdown::auxiliary::reference_link::FormatMdReferenceLink,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::markdown::auxiliary::reference_link::FormatMdReferenceLink::default(),
        )
    }
}
impl FormatRule<biome_markdown_syntax::MdReferenceLinkLabel>
    for crate::markdown::auxiliary::reference_link_label::FormatMdReferenceLinkLabel
{
    type Context = MarkdownFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_markdown_syntax::MdReferenceLinkLabel,
        f: &mut MarkdownFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_markdown_syntax::MdReferenceLinkLabel>::fmt(self, node, f)
    }
}
impl AsFormat<MarkdownFormatContext> for biome_markdown_syntax::MdReferenceLinkLabel {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_markdown_syntax::MdReferenceLinkLabel,
        crate::markdown::auxiliary::reference_link_label::FormatMdReferenceLinkLabel,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::markdown::auxiliary::reference_link_label::FormatMdReferenceLinkLabel::default(),
        )
    }
}
impl IntoFormat<MarkdownFormatContext> for biome_markdown_syntax::MdReferenceLinkLabel {
    type Format = FormatOwnedWithRule<
        biome_markdown_syntax::MdReferenceLinkLabel,
        crate::markdown::auxiliary::reference_link_label::FormatMdReferenceLinkLabel,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::markdown::auxiliary::reference_link_label::FormatMdReferenceLinkLabel::default(),
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
impl AsFormat<MarkdownFormatContext> for biome_markdown_syntax::MdQuoteIndentList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_markdown_syntax::MdQuoteIndentList,
        crate::markdown::lists::quote_indent_list::FormatMdQuoteIndentList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::markdown::lists::quote_indent_list::FormatMdQuoteIndentList::default(),
        )
    }
}
impl IntoFormat<MarkdownFormatContext> for biome_markdown_syntax::MdQuoteIndentList {
    type Format = FormatOwnedWithRule<
        biome_markdown_syntax::MdQuoteIndentList,
        crate::markdown::lists::quote_indent_list::FormatMdQuoteIndentList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::markdown::lists::quote_indent_list::FormatMdQuoteIndentList::default(),
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
