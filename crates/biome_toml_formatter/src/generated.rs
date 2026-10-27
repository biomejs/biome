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
impl FormatRule<biome_toml_syntax::TomlDateTime>
    for crate::toml::value::date_time::FormatTomlDateTime
{
    type Context = TomlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_toml_syntax::TomlDateTime,
        f: &mut TomlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_toml_syntax::TomlDateTime>::fmt(self, node, f)
    }
}
impl AsFormat<TomlFormatContext> for biome_toml_syntax::TomlDateTime {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_toml_syntax::TomlDateTime,
        crate::toml::value::date_time::FormatTomlDateTime,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::toml::value::date_time::FormatTomlDateTime::default(),
        )
    }
}
impl IntoFormat<TomlFormatContext> for biome_toml_syntax::TomlDateTime {
    type Format = FormatOwnedWithRule<
        biome_toml_syntax::TomlDateTime,
        crate::toml::value::date_time::FormatTomlDateTime,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::toml::value::date_time::FormatTomlDateTime::default(),
        )
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
impl FormatRule<biome_toml_syntax::TomlKeyValue>
    for crate::toml::auxiliary::key_value::FormatTomlKeyValue
{
    type Context = TomlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_toml_syntax::TomlKeyValue,
        f: &mut TomlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_toml_syntax::TomlKeyValue>::fmt(self, node, f)
    }
}
impl AsFormat<TomlFormatContext> for biome_toml_syntax::TomlKeyValue {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_toml_syntax::TomlKeyValue,
        crate::toml::auxiliary::key_value::FormatTomlKeyValue,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::toml::auxiliary::key_value::FormatTomlKeyValue::default(),
        )
    }
}
impl IntoFormat<TomlFormatContext> for biome_toml_syntax::TomlKeyValue {
    type Format = FormatOwnedWithRule<
        biome_toml_syntax::TomlKeyValue,
        crate::toml::auxiliary::key_value::FormatTomlKeyValue,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::toml::auxiliary::key_value::FormatTomlKeyValue::default(),
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
impl FormatRule<biome_toml_syntax::TomlString> for crate::toml::value::string::FormatTomlString {
    type Context = TomlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &biome_toml_syntax::TomlString, f: &mut TomlFormatter) -> FormatResult<()> {
        FormatNodeRule::<biome_toml_syntax::TomlString>::fmt(self, node, f)
    }
}
impl AsFormat<TomlFormatContext> for biome_toml_syntax::TomlString {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_toml_syntax::TomlString,
        crate::toml::value::string::FormatTomlString,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::toml::value::string::FormatTomlString::default(),
        )
    }
}
impl IntoFormat<TomlFormatContext> for biome_toml_syntax::TomlString {
    type Format = FormatOwnedWithRule<
        biome_toml_syntax::TomlString,
        crate::toml::value::string::FormatTomlString,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::toml::value::string::FormatTomlString::default(),
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
impl AsFormat<TomlFormatContext> for biome_toml_syntax::TomlInlineTableEntryList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_toml_syntax::TomlInlineTableEntryList,
        crate::toml::lists::inline_table_entry_list::FormatTomlInlineTableEntryList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::toml::lists::inline_table_entry_list::FormatTomlInlineTableEntryList::default(),
        )
    }
}
impl IntoFormat<TomlFormatContext> for biome_toml_syntax::TomlInlineTableEntryList {
    type Format = FormatOwnedWithRule<
        biome_toml_syntax::TomlInlineTableEntryList,
        crate::toml::lists::inline_table_entry_list::FormatTomlInlineTableEntryList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::toml::lists::inline_table_entry_list::FormatTomlInlineTableEntryList::default(),
        )
    }
}
impl AsFormat<TomlFormatContext> for biome_toml_syntax::TomlKeyPartList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_toml_syntax::TomlKeyPartList,
        crate::toml::lists::key_part_list::FormatTomlKeyPartList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::toml::lists::key_part_list::FormatTomlKeyPartList::default(),
        )
    }
}
impl IntoFormat<TomlFormatContext> for biome_toml_syntax::TomlKeyPartList {
    type Format = FormatOwnedWithRule<
        biome_toml_syntax::TomlKeyPartList,
        crate::toml::lists::key_part_list::FormatTomlKeyPartList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::toml::lists::key_part_list::FormatTomlKeyPartList::default(),
        )
    }
}
impl AsFormat<TomlFormatContext> for biome_toml_syntax::TomlKeyValueList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_toml_syntax::TomlKeyValueList,
        crate::toml::lists::key_value_list::FormatTomlKeyValueList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::toml::lists::key_value_list::FormatTomlKeyValueList::default(),
        )
    }
}
impl IntoFormat<TomlFormatContext> for biome_toml_syntax::TomlKeyValueList {
    type Format = FormatOwnedWithRule<
        biome_toml_syntax::TomlKeyValueList,
        crate::toml::lists::key_value_list::FormatTomlKeyValueList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::toml::lists::key_value_list::FormatTomlKeyValueList::default(),
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
impl AsFormat<TomlFormatContext> for biome_toml_syntax::AnyTomlKeyPart {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_toml_syntax::AnyTomlKeyPart,
        crate::toml::any::key_part::FormatAnyTomlKeyPart,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::toml::any::key_part::FormatAnyTomlKeyPart::default(),
        )
    }
}
impl IntoFormat<TomlFormatContext> for biome_toml_syntax::AnyTomlKeyPart {
    type Format = FormatOwnedWithRule<
        biome_toml_syntax::AnyTomlKeyPart,
        crate::toml::any::key_part::FormatAnyTomlKeyPart,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::toml::any::key_part::FormatAnyTomlKeyPart::default(),
        )
    }
}
impl AsFormat<TomlFormatContext> for biome_toml_syntax::AnyTomlKeyValue {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_toml_syntax::AnyTomlKeyValue,
        crate::toml::any::key_value::FormatAnyTomlKeyValue,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::toml::any::key_value::FormatAnyTomlKeyValue::default(),
        )
    }
}
impl IntoFormat<TomlFormatContext> for biome_toml_syntax::AnyTomlKeyValue {
    type Format = FormatOwnedWithRule<
        biome_toml_syntax::AnyTomlKeyValue,
        crate::toml::any::key_value::FormatAnyTomlKeyValue,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::toml::any::key_value::FormatAnyTomlKeyValue::default(),
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
