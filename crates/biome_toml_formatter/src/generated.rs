//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

#![allow(clippy::use_self)]
#![expect(clippy::default_constructed_unit_structs)]
use crate::{
    AsFormat, FormatBogusNodeRule, FormatNodeRule, IntoFormat, TomlFormatContext, TomlFormatter,
};
use biome_formatter::{FormatOwnedWithRule, FormatRefWithRule, FormatResult, FormatRule};
impl FormatRule<biome_toml_syntax::TomlArray> for crate::toml::value::array::FormatTomlArray {
    type Context = TomlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_toml_syntax::TomlArray, f: &mut TomlFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_toml_syntax::TomlArray>::fmt(self, node, f)
    }
}
impl AsFormat<TomlFormatContext> for biome_toml_syntax::TomlArray {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_toml_syntax::TomlArray,
        crate::toml::value::array::FormatTomlArray,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, crate::toml::value::array::FormatTomlArray::default())
    }
}
impl IntoFormat<TomlFormatContext> for biome_toml_syntax::TomlArray {
    type Format = FormatOwnedWithRule<
        biome_toml_syntax::TomlArray,
        crate::toml::value::array::FormatTomlArray,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, crate::toml::value::array::FormatTomlArray::default())
    }
}
impl FormatRule<biome_toml_syntax::TomlArrayTable>
    for crate::toml::auxiliary::array_table::FormatTomlArrayTable
{
    type Context = TomlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_toml_syntax::TomlArrayTable,
        f: &mut TomlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_toml_syntax::TomlArrayTable>::fmt(self, node, f)
    }
}
impl AsFormat<TomlFormatContext> for biome_toml_syntax::TomlArrayTable {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_toml_syntax::TomlArrayTable,
        crate::toml::auxiliary::array_table::FormatTomlArrayTable,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::toml::auxiliary::array_table::FormatTomlArrayTable::default(),
        )
    }
}
impl IntoFormat<TomlFormatContext> for biome_toml_syntax::TomlArrayTable {
    type Format = FormatOwnedWithRule<
        biome_toml_syntax::TomlArrayTable,
        crate::toml::auxiliary::array_table::FormatTomlArrayTable,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::toml::auxiliary::array_table::FormatTomlArrayTable::default(),
        )
    }
}
impl FormatRule<biome_toml_syntax::TomlArrayTableHeader>
    for crate::toml::auxiliary::array_table_header::FormatTomlArrayTableHeader
{
    type Context = TomlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_toml_syntax::TomlArrayTableHeader,
        f: &mut TomlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_toml_syntax::TomlArrayTableHeader>::fmt(self, node, f)
    }
}
impl AsFormat<TomlFormatContext> for biome_toml_syntax::TomlArrayTableHeader {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_toml_syntax::TomlArrayTableHeader,
        crate::toml::auxiliary::array_table_header::FormatTomlArrayTableHeader,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::toml::auxiliary::array_table_header::FormatTomlArrayTableHeader::default(),
        )
    }
}
impl IntoFormat<TomlFormatContext> for biome_toml_syntax::TomlArrayTableHeader {
    type Format = FormatOwnedWithRule<
        biome_toml_syntax::TomlArrayTableHeader,
        crate::toml::auxiliary::array_table_header::FormatTomlArrayTableHeader,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::toml::auxiliary::array_table_header::FormatTomlArrayTableHeader::default(),
        )
    }
}
impl FormatRule<biome_toml_syntax::TomlBareKey>
    for crate::toml::auxiliary::bare_key::FormatTomlBareKey
{
    type Context = TomlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_toml_syntax::TomlBareKey,
        f: &mut TomlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_toml_syntax::TomlBareKey>::fmt(self, node, f)
    }
}
impl AsFormat<TomlFormatContext> for biome_toml_syntax::TomlBareKey {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_toml_syntax::TomlBareKey,
        crate::toml::auxiliary::bare_key::FormatTomlBareKey,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::toml::auxiliary::bare_key::FormatTomlBareKey::default(),
        )
    }
}
impl IntoFormat<TomlFormatContext> for biome_toml_syntax::TomlBareKey {
    type Format = FormatOwnedWithRule<
        biome_toml_syntax::TomlBareKey,
        crate::toml::auxiliary::bare_key::FormatTomlBareKey,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::toml::auxiliary::bare_key::FormatTomlBareKey::default(),
        )
    }
}
impl FormatRule<biome_toml_syntax::TomlBasicString>
    for crate::toml::value::basic_string::FormatTomlBasicString
{
    type Context = TomlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_toml_syntax::TomlBasicString,
        f: &mut TomlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_toml_syntax::TomlBasicString>::fmt(self, node, f)
    }
}
impl AsFormat<TomlFormatContext> for biome_toml_syntax::TomlBasicString {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_toml_syntax::TomlBasicString,
        crate::toml::value::basic_string::FormatTomlBasicString,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::toml::value::basic_string::FormatTomlBasicString::default(),
        )
    }
}
impl IntoFormat<TomlFormatContext> for biome_toml_syntax::TomlBasicString {
    type Format = FormatOwnedWithRule<
        biome_toml_syntax::TomlBasicString,
        crate::toml::value::basic_string::FormatTomlBasicString,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::toml::value::basic_string::FormatTomlBasicString::default(),
        )
    }
}
impl FormatRule<biome_toml_syntax::TomlBoolean> for crate::toml::value::boolean::FormatTomlBoolean {
    type Context = TomlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_toml_syntax::TomlBoolean,
        f: &mut TomlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_toml_syntax::TomlBoolean>::fmt(self, node, f)
    }
}
impl AsFormat<TomlFormatContext> for biome_toml_syntax::TomlBoolean {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_toml_syntax::TomlBoolean,
        crate::toml::value::boolean::FormatTomlBoolean,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::toml::value::boolean::FormatTomlBoolean::default(),
        )
    }
}
impl IntoFormat<TomlFormatContext> for biome_toml_syntax::TomlBoolean {
    type Format = FormatOwnedWithRule<
        biome_toml_syntax::TomlBoolean,
        crate::toml::value::boolean::FormatTomlBoolean,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::toml::value::boolean::FormatTomlBoolean::default(),
        )
    }
}
impl FormatRule<biome_toml_syntax::TomlDate> for crate::toml::value::date::FormatTomlDate {
    type Context = TomlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_toml_syntax::TomlDate, f: &mut TomlFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_toml_syntax::TomlDate>::fmt(self, node, f)
    }
}
impl AsFormat<TomlFormatContext> for biome_toml_syntax::TomlDate {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_toml_syntax::TomlDate,
        crate::toml::value::date::FormatTomlDate,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, crate::toml::value::date::FormatTomlDate::default())
    }
}
impl IntoFormat<TomlFormatContext> for biome_toml_syntax::TomlDate {
    type Format =
        FormatOwnedWithRule<biome_toml_syntax::TomlDate, crate::toml::value::date::FormatTomlDate>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, crate::toml::value::date::FormatTomlDate::default())
    }
}
impl FormatRule<biome_toml_syntax::TomlFloat> for crate::toml::value::float::FormatTomlFloat {
    type Context = TomlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_toml_syntax::TomlFloat, f: &mut TomlFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_toml_syntax::TomlFloat>::fmt(self, node, f)
    }
}
impl AsFormat<TomlFormatContext> for biome_toml_syntax::TomlFloat {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_toml_syntax::TomlFloat,
        crate::toml::value::float::FormatTomlFloat,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, crate::toml::value::float::FormatTomlFloat::default())
    }
}
impl IntoFormat<TomlFormatContext> for biome_toml_syntax::TomlFloat {
    type Format = FormatOwnedWithRule<
        biome_toml_syntax::TomlFloat,
        crate::toml::value::float::FormatTomlFloat,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, crate::toml::value::float::FormatTomlFloat::default())
    }
}
impl FormatRule<biome_toml_syntax::TomlInlineTable>
    for crate::toml::value::inline_table::FormatTomlInlineTable
{
    type Context = TomlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_toml_syntax::TomlInlineTable,
        f: &mut TomlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_toml_syntax::TomlInlineTable>::fmt(self, node, f)
    }
}
impl AsFormat<TomlFormatContext> for biome_toml_syntax::TomlInlineTable {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_toml_syntax::TomlInlineTable,
        crate::toml::value::inline_table::FormatTomlInlineTable,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::toml::value::inline_table::FormatTomlInlineTable::default(),
        )
    }
}
impl IntoFormat<TomlFormatContext> for biome_toml_syntax::TomlInlineTable {
    type Format = FormatOwnedWithRule<
        biome_toml_syntax::TomlInlineTable,
        crate::toml::value::inline_table::FormatTomlInlineTable,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::toml::value::inline_table::FormatTomlInlineTable::default(),
        )
    }
}
impl FormatRule<biome_toml_syntax::TomlInteger> for crate::toml::value::integer::FormatTomlInteger {
    type Context = TomlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_toml_syntax::TomlInteger,
        f: &mut TomlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_toml_syntax::TomlInteger>::fmt(self, node, f)
    }
}
impl AsFormat<TomlFormatContext> for biome_toml_syntax::TomlInteger {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_toml_syntax::TomlInteger,
        crate::toml::value::integer::FormatTomlInteger,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::toml::value::integer::FormatTomlInteger::default(),
        )
    }
}
impl IntoFormat<TomlFormatContext> for biome_toml_syntax::TomlInteger {
    type Format = FormatOwnedWithRule<
        biome_toml_syntax::TomlInteger,
        crate::toml::value::integer::FormatTomlInteger,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::toml::value::integer::FormatTomlInteger::default(),
        )
    }
}
impl FormatRule<biome_toml_syntax::TomlKey> for crate::toml::auxiliary::key::FormatTomlKey {
    type Context = TomlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_toml_syntax::TomlKey, f: &mut TomlFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_toml_syntax::TomlKey>::fmt(self, node, f)
    }
}
impl AsFormat<TomlFormatContext> for biome_toml_syntax::TomlKey {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_toml_syntax::TomlKey,
        crate::toml::auxiliary::key::FormatTomlKey,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, crate::toml::auxiliary::key::FormatTomlKey::default())
    }
}
impl IntoFormat<TomlFormatContext> for biome_toml_syntax::TomlKey {
    type Format =
        FormatOwnedWithRule<biome_toml_syntax::TomlKey, crate::toml::auxiliary::key::FormatTomlKey>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, crate::toml::auxiliary::key::FormatTomlKey::default())
    }
}
impl FormatRule<biome_toml_syntax::TomlLiteralString>
    for crate::toml::value::literal_string::FormatTomlLiteralString
{
    type Context = TomlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_toml_syntax::TomlLiteralString,
        f: &mut TomlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_toml_syntax::TomlLiteralString>::fmt(self, node, f)
    }
}
impl AsFormat<TomlFormatContext> for biome_toml_syntax::TomlLiteralString {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_toml_syntax::TomlLiteralString,
        crate::toml::value::literal_string::FormatTomlLiteralString,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::toml::value::literal_string::FormatTomlLiteralString::default(),
        )
    }
}
impl IntoFormat<TomlFormatContext> for biome_toml_syntax::TomlLiteralString {
    type Format = FormatOwnedWithRule<
        biome_toml_syntax::TomlLiteralString,
        crate::toml::value::literal_string::FormatTomlLiteralString,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::toml::value::literal_string::FormatTomlLiteralString::default(),
        )
    }
}
impl FormatRule<biome_toml_syntax::TomlMember>
    for crate::toml::auxiliary::member::FormatTomlMember
{
    type Context = TomlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_toml_syntax::TomlMember, f: &mut TomlFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_toml_syntax::TomlMember>::fmt(self, node, f)
    }
}
impl AsFormat<TomlFormatContext> for biome_toml_syntax::TomlMember {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_toml_syntax::TomlMember,
        crate::toml::auxiliary::member::FormatTomlMember,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::toml::auxiliary::member::FormatTomlMember::default(),
        )
    }
}
impl IntoFormat<TomlFormatContext> for biome_toml_syntax::TomlMember {
    type Format = FormatOwnedWithRule<
        biome_toml_syntax::TomlMember,
        crate::toml::auxiliary::member::FormatTomlMember,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::toml::auxiliary::member::FormatTomlMember::default(),
        )
    }
}
impl FormatRule<biome_toml_syntax::TomlMultilineBasicString>
    for crate::toml::value::multiline_basic_string::FormatTomlMultilineBasicString
{
    type Context = TomlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_toml_syntax::TomlMultilineBasicString,
        f: &mut TomlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_toml_syntax::TomlMultilineBasicString>::fmt(self, node, f)
    }
}
impl AsFormat<TomlFormatContext> for biome_toml_syntax::TomlMultilineBasicString {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_toml_syntax::TomlMultilineBasicString,
        crate::toml::value::multiline_basic_string::FormatTomlMultilineBasicString,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::toml::value::multiline_basic_string::FormatTomlMultilineBasicString::default(),
        )
    }
}
impl IntoFormat<TomlFormatContext> for biome_toml_syntax::TomlMultilineBasicString {
    type Format = FormatOwnedWithRule<
        biome_toml_syntax::TomlMultilineBasicString,
        crate::toml::value::multiline_basic_string::FormatTomlMultilineBasicString,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::toml::value::multiline_basic_string::FormatTomlMultilineBasicString::default(),
        )
    }
}
impl FormatRule<biome_toml_syntax::TomlMultilineLiteralString>
    for crate::toml::value::multiline_literal_string::FormatTomlMultilineLiteralString
{
    type Context = TomlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_toml_syntax::TomlMultilineLiteralString,
        f: &mut TomlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_toml_syntax::TomlMultilineLiteralString>::fmt(self, node, f)
    }
}
impl AsFormat<TomlFormatContext> for biome_toml_syntax::TomlMultilineLiteralString {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_toml_syntax::TomlMultilineLiteralString,
        crate::toml::value::multiline_literal_string::FormatTomlMultilineLiteralString,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::toml::value::multiline_literal_string::FormatTomlMultilineLiteralString::default(
            ),
        )
    }
}
impl IntoFormat<TomlFormatContext> for biome_toml_syntax::TomlMultilineLiteralString {
    type Format = FormatOwnedWithRule<
        biome_toml_syntax::TomlMultilineLiteralString,
        crate::toml::value::multiline_literal_string::FormatTomlMultilineLiteralString,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::toml::value::multiline_literal_string::FormatTomlMultilineLiteralString::default(
            ),
        )
    }
}
impl FormatRule<biome_toml_syntax::TomlQuotedKey>
    for crate::toml::auxiliary::quoted_key::FormatTomlQuotedKey
{
    type Context = TomlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_toml_syntax::TomlQuotedKey,
        f: &mut TomlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_toml_syntax::TomlQuotedKey>::fmt(self, node, f)
    }
}
impl AsFormat<TomlFormatContext> for biome_toml_syntax::TomlQuotedKey {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_toml_syntax::TomlQuotedKey,
        crate::toml::auxiliary::quoted_key::FormatTomlQuotedKey,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::toml::auxiliary::quoted_key::FormatTomlQuotedKey::default(),
        )
    }
}
impl IntoFormat<TomlFormatContext> for biome_toml_syntax::TomlQuotedKey {
    type Format = FormatOwnedWithRule<
        biome_toml_syntax::TomlQuotedKey,
        crate::toml::auxiliary::quoted_key::FormatTomlQuotedKey,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::toml::auxiliary::quoted_key::FormatTomlQuotedKey::default(),
        )
    }
}
impl FormatRule<biome_toml_syntax::TomlRoot> for crate::toml::auxiliary::root::FormatTomlRoot {
    type Context = TomlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_toml_syntax::TomlRoot, f: &mut TomlFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_toml_syntax::TomlRoot>::fmt(self, node, f)
    }
}
impl AsFormat<TomlFormatContext> for biome_toml_syntax::TomlRoot {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_toml_syntax::TomlRoot,
        crate::toml::auxiliary::root::FormatTomlRoot,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::toml::auxiliary::root::FormatTomlRoot::default(),
        )
    }
}
impl IntoFormat<TomlFormatContext> for biome_toml_syntax::TomlRoot {
    type Format = FormatOwnedWithRule<
        biome_toml_syntax::TomlRoot,
        crate::toml::auxiliary::root::FormatTomlRoot,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::toml::auxiliary::root::FormatTomlRoot::default(),
        )
    }
}
impl FormatRule<biome_toml_syntax::TomlTable> for crate::toml::auxiliary::table::FormatTomlTable {
    type Context = TomlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_toml_syntax::TomlTable, f: &mut TomlFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_toml_syntax::TomlTable>::fmt(self, node, f)
    }
}
impl AsFormat<TomlFormatContext> for biome_toml_syntax::TomlTable {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_toml_syntax::TomlTable,
        crate::toml::auxiliary::table::FormatTomlTable,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::toml::auxiliary::table::FormatTomlTable::default(),
        )
    }
}
impl IntoFormat<TomlFormatContext> for biome_toml_syntax::TomlTable {
    type Format = FormatOwnedWithRule<
        biome_toml_syntax::TomlTable,
        crate::toml::auxiliary::table::FormatTomlTable,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::toml::auxiliary::table::FormatTomlTable::default(),
        )
    }
}
impl FormatRule<biome_toml_syntax::TomlTableHeader>
    for crate::toml::auxiliary::table_header::FormatTomlTableHeader
{
    type Context = TomlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_toml_syntax::TomlTableHeader,
        f: &mut TomlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_toml_syntax::TomlTableHeader>::fmt(self, node, f)
    }
}
impl AsFormat<TomlFormatContext> for biome_toml_syntax::TomlTableHeader {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_toml_syntax::TomlTableHeader,
        crate::toml::auxiliary::table_header::FormatTomlTableHeader,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::toml::auxiliary::table_header::FormatTomlTableHeader::default(),
        )
    }
}
impl IntoFormat<TomlFormatContext> for biome_toml_syntax::TomlTableHeader {
    type Format = FormatOwnedWithRule<
        biome_toml_syntax::TomlTableHeader,
        crate::toml::auxiliary::table_header::FormatTomlTableHeader,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::toml::auxiliary::table_header::FormatTomlTableHeader::default(),
        )
    }
}
impl FormatRule<biome_toml_syntax::TomlTime> for crate::toml::value::time::FormatTomlTime {
    type Context = TomlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_toml_syntax::TomlTime, f: &mut TomlFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_toml_syntax::TomlTime>::fmt(self, node, f)
    }
}
impl AsFormat<TomlFormatContext> for biome_toml_syntax::TomlTime {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_toml_syntax::TomlTime,
        crate::toml::value::time::FormatTomlTime,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, crate::toml::value::time::FormatTomlTime::default())
    }
}
impl IntoFormat<TomlFormatContext> for biome_toml_syntax::TomlTime {
    type Format =
        FormatOwnedWithRule<biome_toml_syntax::TomlTime, crate::toml::value::time::FormatTomlTime>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, crate::toml::value::time::FormatTomlTime::default())
    }
}
impl AsFormat<TomlFormatContext> for biome_toml_syntax::TomlArrayElementList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_toml_syntax::TomlArrayElementList,
        crate::toml::lists::array_element_list::FormatTomlArrayElementList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::toml::lists::array_element_list::FormatTomlArrayElementList::default(),
        )
    }
}
impl IntoFormat<TomlFormatContext> for biome_toml_syntax::TomlArrayElementList {
    type Format = FormatOwnedWithRule<
        biome_toml_syntax::TomlArrayElementList,
        crate::toml::lists::array_element_list::FormatTomlArrayElementList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::toml::lists::array_element_list::FormatTomlArrayElementList::default(),
        )
    }
}
impl AsFormat<TomlFormatContext> for biome_toml_syntax::TomlInlineTableMemberList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_toml_syntax::TomlInlineTableMemberList,
        crate::toml::lists::inline_table_member_list::FormatTomlInlineTableMemberList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::toml::lists::inline_table_member_list::FormatTomlInlineTableMemberList::default(
            ),
        )
    }
}
impl IntoFormat<TomlFormatContext> for biome_toml_syntax::TomlInlineTableMemberList {
    type Format = FormatOwnedWithRule<
        biome_toml_syntax::TomlInlineTableMemberList,
        crate::toml::lists::inline_table_member_list::FormatTomlInlineTableMemberList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::toml::lists::inline_table_member_list::FormatTomlInlineTableMemberList::default(
            ),
        )
    }
}
impl AsFormat<TomlFormatContext> for biome_toml_syntax::TomlKeyFragmentList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_toml_syntax::TomlKeyFragmentList,
        crate::toml::lists::key_fragment_list::FormatTomlKeyFragmentList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::toml::lists::key_fragment_list::FormatTomlKeyFragmentList::default(),
        )
    }
}
impl IntoFormat<TomlFormatContext> for biome_toml_syntax::TomlKeyFragmentList {
    type Format = FormatOwnedWithRule<
        biome_toml_syntax::TomlKeyFragmentList,
        crate::toml::lists::key_fragment_list::FormatTomlKeyFragmentList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::toml::lists::key_fragment_list::FormatTomlKeyFragmentList::default(),
        )
    }
}
impl AsFormat<TomlFormatContext> for biome_toml_syntax::TomlMemberList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_toml_syntax::TomlMemberList,
        crate::toml::lists::member_list::FormatTomlMemberList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::toml::lists::member_list::FormatTomlMemberList::default(),
        )
    }
}
impl IntoFormat<TomlFormatContext> for biome_toml_syntax::TomlMemberList {
    type Format = FormatOwnedWithRule<
        biome_toml_syntax::TomlMemberList,
        crate::toml::lists::member_list::FormatTomlMemberList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::toml::lists::member_list::FormatTomlMemberList::default(),
        )
    }
}
impl AsFormat<TomlFormatContext> for biome_toml_syntax::TomlRootItemList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_toml_syntax::TomlRootItemList,
        crate::toml::lists::root_item_list::FormatTomlRootItemList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::toml::lists::root_item_list::FormatTomlRootItemList::default(),
        )
    }
}
impl IntoFormat<TomlFormatContext> for biome_toml_syntax::TomlRootItemList {
    type Format = FormatOwnedWithRule<
        biome_toml_syntax::TomlRootItemList,
        crate::toml::lists::root_item_list::FormatTomlRootItemList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::toml::lists::root_item_list::FormatTomlRootItemList::default(),
        )
    }
}
impl FormatRule<biome_toml_syntax::TomlBogus> for crate::toml::bogus::bogus::FormatTomlBogus {
    type Context = TomlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_toml_syntax::TomlBogus, f: &mut TomlFormatter) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_toml_syntax::TomlBogus>::fmt(self, node, f)
    }
}
impl AsFormat<TomlFormatContext> for biome_toml_syntax::TomlBogus {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_toml_syntax::TomlBogus,
        crate::toml::bogus::bogus::FormatTomlBogus,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, crate::toml::bogus::bogus::FormatTomlBogus::default())
    }
}
impl IntoFormat<TomlFormatContext> for biome_toml_syntax::TomlBogus {
    type Format = FormatOwnedWithRule<
        biome_toml_syntax::TomlBogus,
        crate::toml::bogus::bogus::FormatTomlBogus,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, crate::toml::bogus::bogus::FormatTomlBogus::default())
    }
}
impl FormatRule<biome_toml_syntax::TomlBogusKey>
    for crate::toml::bogus::bogus_key::FormatTomlBogusKey
{
    type Context = TomlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_toml_syntax::TomlBogusKey,
        f: &mut TomlFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_toml_syntax::TomlBogusKey>::fmt(self, node, f)
    }
}
impl AsFormat<TomlFormatContext> for biome_toml_syntax::TomlBogusKey {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_toml_syntax::TomlBogusKey,
        crate::toml::bogus::bogus_key::FormatTomlBogusKey,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::toml::bogus::bogus_key::FormatTomlBogusKey::default(),
        )
    }
}
impl IntoFormat<TomlFormatContext> for biome_toml_syntax::TomlBogusKey {
    type Format = FormatOwnedWithRule<
        biome_toml_syntax::TomlBogusKey,
        crate::toml::bogus::bogus_key::FormatTomlBogusKey,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::toml::bogus::bogus_key::FormatTomlBogusKey::default(),
        )
    }
}
impl FormatRule<biome_toml_syntax::TomlBogusKeyFragment>
    for crate::toml::bogus::bogus_key_fragment::FormatTomlBogusKeyFragment
{
    type Context = TomlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_toml_syntax::TomlBogusKeyFragment,
        f: &mut TomlFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_toml_syntax::TomlBogusKeyFragment>::fmt(self, node, f)
    }
}
impl AsFormat<TomlFormatContext> for biome_toml_syntax::TomlBogusKeyFragment {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_toml_syntax::TomlBogusKeyFragment,
        crate::toml::bogus::bogus_key_fragment::FormatTomlBogusKeyFragment,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::toml::bogus::bogus_key_fragment::FormatTomlBogusKeyFragment::default(),
        )
    }
}
impl IntoFormat<TomlFormatContext> for biome_toml_syntax::TomlBogusKeyFragment {
    type Format = FormatOwnedWithRule<
        biome_toml_syntax::TomlBogusKeyFragment,
        crate::toml::bogus::bogus_key_fragment::FormatTomlBogusKeyFragment,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::toml::bogus::bogus_key_fragment::FormatTomlBogusKeyFragment::default(),
        )
    }
}
impl FormatRule<biome_toml_syntax::TomlBogusMember>
    for crate::toml::bogus::bogus_member::FormatTomlBogusMember
{
    type Context = TomlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_toml_syntax::TomlBogusMember,
        f: &mut TomlFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_toml_syntax::TomlBogusMember>::fmt(self, node, f)
    }
}
impl AsFormat<TomlFormatContext> for biome_toml_syntax::TomlBogusMember {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_toml_syntax::TomlBogusMember,
        crate::toml::bogus::bogus_member::FormatTomlBogusMember,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::toml::bogus::bogus_member::FormatTomlBogusMember::default(),
        )
    }
}
impl IntoFormat<TomlFormatContext> for biome_toml_syntax::TomlBogusMember {
    type Format = FormatOwnedWithRule<
        biome_toml_syntax::TomlBogusMember,
        crate::toml::bogus::bogus_member::FormatTomlBogusMember,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::toml::bogus::bogus_member::FormatTomlBogusMember::default(),
        )
    }
}
impl FormatRule<biome_toml_syntax::TomlBogusTableHeader>
    for crate::toml::bogus::bogus_table_header::FormatTomlBogusTableHeader
{
    type Context = TomlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_toml_syntax::TomlBogusTableHeader,
        f: &mut TomlFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_toml_syntax::TomlBogusTableHeader>::fmt(self, node, f)
    }
}
impl AsFormat<TomlFormatContext> for biome_toml_syntax::TomlBogusTableHeader {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_toml_syntax::TomlBogusTableHeader,
        crate::toml::bogus::bogus_table_header::FormatTomlBogusTableHeader,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::toml::bogus::bogus_table_header::FormatTomlBogusTableHeader::default(),
        )
    }
}
impl IntoFormat<TomlFormatContext> for biome_toml_syntax::TomlBogusTableHeader {
    type Format = FormatOwnedWithRule<
        biome_toml_syntax::TomlBogusTableHeader,
        crate::toml::bogus::bogus_table_header::FormatTomlBogusTableHeader,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::toml::bogus::bogus_table_header::FormatTomlBogusTableHeader::default(),
        )
    }
}
impl FormatRule<biome_toml_syntax::TomlBogusValue>
    for crate::toml::bogus::bogus_value::FormatTomlBogusValue
{
    type Context = TomlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_toml_syntax::TomlBogusValue,
        f: &mut TomlFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_toml_syntax::TomlBogusValue>::fmt(self, node, f)
    }
}
impl AsFormat<TomlFormatContext> for biome_toml_syntax::TomlBogusValue {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_toml_syntax::TomlBogusValue,
        crate::toml::bogus::bogus_value::FormatTomlBogusValue,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::toml::bogus::bogus_value::FormatTomlBogusValue::default(),
        )
    }
}
impl IntoFormat<TomlFormatContext> for biome_toml_syntax::TomlBogusValue {
    type Format = FormatOwnedWithRule<
        biome_toml_syntax::TomlBogusValue,
        crate::toml::bogus::bogus_value::FormatTomlBogusValue,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::toml::bogus::bogus_value::FormatTomlBogusValue::default(),
        )
    }
}
impl AsFormat<TomlFormatContext> for biome_toml_syntax::AnyTomlArrayTableHeader {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_toml_syntax::AnyTomlArrayTableHeader,
        crate::toml::any::array_table_header::FormatAnyTomlArrayTableHeader,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::toml::any::array_table_header::FormatAnyTomlArrayTableHeader::default(),
        )
    }
}
impl IntoFormat<TomlFormatContext> for biome_toml_syntax::AnyTomlArrayTableHeader {
    type Format = FormatOwnedWithRule<
        biome_toml_syntax::AnyTomlArrayTableHeader,
        crate::toml::any::array_table_header::FormatAnyTomlArrayTableHeader,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::toml::any::array_table_header::FormatAnyTomlArrayTableHeader::default(),
        )
    }
}
impl AsFormat<TomlFormatContext> for biome_toml_syntax::AnyTomlKey {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_toml_syntax::AnyTomlKey,
        crate::toml::any::key::FormatAnyTomlKey,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, crate::toml::any::key::FormatAnyTomlKey::default())
    }
}
impl IntoFormat<TomlFormatContext> for biome_toml_syntax::AnyTomlKey {
    type Format =
        FormatOwnedWithRule<biome_toml_syntax::AnyTomlKey, crate::toml::any::key::FormatAnyTomlKey>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, crate::toml::any::key::FormatAnyTomlKey::default())
    }
}
impl AsFormat<TomlFormatContext> for biome_toml_syntax::AnyTomlKeyFragment {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_toml_syntax::AnyTomlKeyFragment,
        crate::toml::any::key_fragment::FormatAnyTomlKeyFragment,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::toml::any::key_fragment::FormatAnyTomlKeyFragment::default(),
        )
    }
}
impl IntoFormat<TomlFormatContext> for biome_toml_syntax::AnyTomlKeyFragment {
    type Format = FormatOwnedWithRule<
        biome_toml_syntax::AnyTomlKeyFragment,
        crate::toml::any::key_fragment::FormatAnyTomlKeyFragment,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::toml::any::key_fragment::FormatAnyTomlKeyFragment::default(),
        )
    }
}
impl AsFormat<TomlFormatContext> for biome_toml_syntax::AnyTomlMember {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_toml_syntax::AnyTomlMember,
        crate::toml::any::member::FormatAnyTomlMember,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::toml::any::member::FormatAnyTomlMember::default(),
        )
    }
}
impl IntoFormat<TomlFormatContext> for biome_toml_syntax::AnyTomlMember {
    type Format = FormatOwnedWithRule<
        biome_toml_syntax::AnyTomlMember,
        crate::toml::any::member::FormatAnyTomlMember,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::toml::any::member::FormatAnyTomlMember::default(),
        )
    }
}
impl AsFormat<TomlFormatContext> for biome_toml_syntax::AnyTomlRootItem {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_toml_syntax::AnyTomlRootItem,
        crate::toml::any::root_item::FormatAnyTomlRootItem,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::toml::any::root_item::FormatAnyTomlRootItem::default(),
        )
    }
}
impl IntoFormat<TomlFormatContext> for biome_toml_syntax::AnyTomlRootItem {
    type Format = FormatOwnedWithRule<
        biome_toml_syntax::AnyTomlRootItem,
        crate::toml::any::root_item::FormatAnyTomlRootItem,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::toml::any::root_item::FormatAnyTomlRootItem::default(),
        )
    }
}
impl AsFormat<TomlFormatContext> for biome_toml_syntax::AnyTomlString {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_toml_syntax::AnyTomlString,
        crate::toml::any::string::FormatAnyTomlString,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::toml::any::string::FormatAnyTomlString::default(),
        )
    }
}
impl IntoFormat<TomlFormatContext> for biome_toml_syntax::AnyTomlString {
    type Format = FormatOwnedWithRule<
        biome_toml_syntax::AnyTomlString,
        crate::toml::any::string::FormatAnyTomlString,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::toml::any::string::FormatAnyTomlString::default(),
        )
    }
}
impl AsFormat<TomlFormatContext> for biome_toml_syntax::AnyTomlTableHeader {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_toml_syntax::AnyTomlTableHeader,
        crate::toml::any::table_header::FormatAnyTomlTableHeader,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::toml::any::table_header::FormatAnyTomlTableHeader::default(),
        )
    }
}
impl IntoFormat<TomlFormatContext> for biome_toml_syntax::AnyTomlTableHeader {
    type Format = FormatOwnedWithRule<
        biome_toml_syntax::AnyTomlTableHeader,
        crate::toml::any::table_header::FormatAnyTomlTableHeader,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::toml::any::table_header::FormatAnyTomlTableHeader::default(),
        )
    }
}
impl AsFormat<TomlFormatContext> for biome_toml_syntax::AnyTomlValue {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_toml_syntax::AnyTomlValue,
        crate::toml::any::value::FormatAnyTomlValue,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, crate::toml::any::value::FormatAnyTomlValue::default())
    }
}
impl IntoFormat<TomlFormatContext> for biome_toml_syntax::AnyTomlValue {
    type Format = FormatOwnedWithRule<
        biome_toml_syntax::AnyTomlValue,
        crate::toml::any::value::FormatAnyTomlValue,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, crate::toml::any::value::FormatAnyTomlValue::default())
    }
}
