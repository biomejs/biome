//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::{
    AsFormat, FormatBogusNodeRule, FormatNodeRule, IntoFormat, JsonFormatContext, JsonFormatter,
};
use rome_formatter::{FormatOwnedWithRule, FormatRefWithRule, FormatResult, FormatRule};
impl FormatRule<rome_json_syntax::JsonRoot> for crate::json::auxiliary::root::FormatJsonRoot {
    type Context = JsonFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &rome_json_syntax::JsonRoot, f: &mut JsonFormatter) -> FormatResult<()> {
        FormatNodeRule::<rome_json_syntax::JsonRoot>::fmt(self, node, f)
    }
}
impl AsFormat<JsonFormatContext> for rome_json_syntax::JsonRoot {
    type Format<'a> = FormatRefWithRule<
        'a,
        rome_json_syntax::JsonRoot,
        crate::json::auxiliary::root::FormatJsonRoot,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::json::auxiliary::root::FormatJsonRoot::default(),
        )
    }
}
impl IntoFormat<JsonFormatContext> for rome_json_syntax::JsonRoot {
    type Format = FormatOwnedWithRule<
        rome_json_syntax::JsonRoot,
        crate::json::auxiliary::root::FormatJsonRoot,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::json::auxiliary::root::FormatJsonRoot::default(),
        )
    }
}
impl FormatRule<rome_json_syntax::JsonStringValue>
    for crate::json::value::string_value::FormatJsonStringValue
{
    type Context = JsonFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &rome_json_syntax::JsonStringValue,
        f: &mut JsonFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<rome_json_syntax::JsonStringValue>::fmt(self, node, f)
    }
}
impl AsFormat<JsonFormatContext> for rome_json_syntax::JsonStringValue {
    type Format<'a> = FormatRefWithRule<
        'a,
        rome_json_syntax::JsonStringValue,
        crate::json::value::string_value::FormatJsonStringValue,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::json::value::string_value::FormatJsonStringValue::default(),
        )
    }
}
impl IntoFormat<JsonFormatContext> for rome_json_syntax::JsonStringValue {
    type Format = FormatOwnedWithRule<
        rome_json_syntax::JsonStringValue,
        crate::json::value::string_value::FormatJsonStringValue,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::json::value::string_value::FormatJsonStringValue::default(),
        )
    }
}
impl FormatRule<rome_json_syntax::JsonBooleanValue>
    for crate::json::value::boolean_value::FormatJsonBooleanValue
{
    type Context = JsonFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &rome_json_syntax::JsonBooleanValue,
        f: &mut JsonFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<rome_json_syntax::JsonBooleanValue>::fmt(self, node, f)
    }
}
impl AsFormat<JsonFormatContext> for rome_json_syntax::JsonBooleanValue {
    type Format<'a> = FormatRefWithRule<
        'a,
        rome_json_syntax::JsonBooleanValue,
        crate::json::value::boolean_value::FormatJsonBooleanValue,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::json::value::boolean_value::FormatJsonBooleanValue::default(),
        )
    }
}
impl IntoFormat<JsonFormatContext> for rome_json_syntax::JsonBooleanValue {
    type Format = FormatOwnedWithRule<
        rome_json_syntax::JsonBooleanValue,
        crate::json::value::boolean_value::FormatJsonBooleanValue,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::json::value::boolean_value::FormatJsonBooleanValue::default(),
        )
    }
}
impl FormatRule<rome_json_syntax::JsonNullValue>
    for crate::json::value::null_value::FormatJsonNullValue
{
    type Context = JsonFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &rome_json_syntax::JsonNullValue,
        f: &mut JsonFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<rome_json_syntax::JsonNullValue>::fmt(self, node, f)
    }
}
impl AsFormat<JsonFormatContext> for rome_json_syntax::JsonNullValue {
    type Format<'a> = FormatRefWithRule<
        'a,
        rome_json_syntax::JsonNullValue,
        crate::json::value::null_value::FormatJsonNullValue,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::json::value::null_value::FormatJsonNullValue::default(),
        )
    }
}
impl IntoFormat<JsonFormatContext> for rome_json_syntax::JsonNullValue {
    type Format = FormatOwnedWithRule<
        rome_json_syntax::JsonNullValue,
        crate::json::value::null_value::FormatJsonNullValue,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::json::value::null_value::FormatJsonNullValue::default(),
        )
    }
}
impl FormatRule<rome_json_syntax::JsonNumberValue>
    for crate::json::value::number_value::FormatJsonNumberValue
{
    type Context = JsonFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &rome_json_syntax::JsonNumberValue,
        f: &mut JsonFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<rome_json_syntax::JsonNumberValue>::fmt(self, node, f)
    }
}
impl AsFormat<JsonFormatContext> for rome_json_syntax::JsonNumberValue {
    type Format<'a> = FormatRefWithRule<
        'a,
        rome_json_syntax::JsonNumberValue,
        crate::json::value::number_value::FormatJsonNumberValue,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::json::value::number_value::FormatJsonNumberValue::default(),
        )
    }
}
impl IntoFormat<JsonFormatContext> for rome_json_syntax::JsonNumberValue {
    type Format = FormatOwnedWithRule<
        rome_json_syntax::JsonNumberValue,
        crate::json::value::number_value::FormatJsonNumberValue,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::json::value::number_value::FormatJsonNumberValue::default(),
        )
    }
}
impl FormatRule<rome_json_syntax::JsonArrayValue>
    for crate::json::value::array_value::FormatJsonArrayValue
{
    type Context = JsonFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &rome_json_syntax::JsonArrayValue,
        f: &mut JsonFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<rome_json_syntax::JsonArrayValue>::fmt(self, node, f)
    }
}
impl AsFormat<JsonFormatContext> for rome_json_syntax::JsonArrayValue {
    type Format<'a> = FormatRefWithRule<
        'a,
        rome_json_syntax::JsonArrayValue,
        crate::json::value::array_value::FormatJsonArrayValue,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::json::value::array_value::FormatJsonArrayValue::default(),
        )
    }
}
impl IntoFormat<JsonFormatContext> for rome_json_syntax::JsonArrayValue {
    type Format = FormatOwnedWithRule<
        rome_json_syntax::JsonArrayValue,
        crate::json::value::array_value::FormatJsonArrayValue,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::json::value::array_value::FormatJsonArrayValue::default(),
        )
    }
}
impl FormatRule<rome_json_syntax::JsonObjectValue>
    for crate::json::value::object_value::FormatJsonObjectValue
{
    type Context = JsonFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &rome_json_syntax::JsonObjectValue,
        f: &mut JsonFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<rome_json_syntax::JsonObjectValue>::fmt(self, node, f)
    }
}
impl AsFormat<JsonFormatContext> for rome_json_syntax::JsonObjectValue {
    type Format<'a> = FormatRefWithRule<
        'a,
        rome_json_syntax::JsonObjectValue,
        crate::json::value::object_value::FormatJsonObjectValue,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::json::value::object_value::FormatJsonObjectValue::default(),
        )
    }
}
impl IntoFormat<JsonFormatContext> for rome_json_syntax::JsonObjectValue {
    type Format = FormatOwnedWithRule<
        rome_json_syntax::JsonObjectValue,
        crate::json::value::object_value::FormatJsonObjectValue,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::json::value::object_value::FormatJsonObjectValue::default(),
        )
    }
}
impl FormatRule<rome_json_syntax::JsonMember> for crate::json::auxiliary::member::FormatJsonMember {
    type Context = JsonFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &rome_json_syntax::JsonMember, f: &mut JsonFormatter) -> FormatResult<()> {
        FormatNodeRule::<rome_json_syntax::JsonMember>::fmt(self, node, f)
    }
}
impl AsFormat<JsonFormatContext> for rome_json_syntax::JsonMember {
    type Format<'a> = FormatRefWithRule<
        'a,
        rome_json_syntax::JsonMember,
        crate::json::auxiliary::member::FormatJsonMember,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::json::auxiliary::member::FormatJsonMember::default(),
        )
    }
}
impl IntoFormat<JsonFormatContext> for rome_json_syntax::JsonMember {
    type Format = FormatOwnedWithRule<
        rome_json_syntax::JsonMember,
        crate::json::auxiliary::member::FormatJsonMember,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::json::auxiliary::member::FormatJsonMember::default(),
        )
    }
}
impl FormatRule<rome_json_syntax::JsonMemberName>
    for crate::json::auxiliary::member_name::FormatJsonMemberName
{
    type Context = JsonFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &rome_json_syntax::JsonMemberName,
        f: &mut JsonFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<rome_json_syntax::JsonMemberName>::fmt(self, node, f)
    }
}
impl AsFormat<JsonFormatContext> for rome_json_syntax::JsonMemberName {
    type Format<'a> = FormatRefWithRule<
        'a,
        rome_json_syntax::JsonMemberName,
        crate::json::auxiliary::member_name::FormatJsonMemberName,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::json::auxiliary::member_name::FormatJsonMemberName::default(),
        )
    }
}
impl IntoFormat<JsonFormatContext> for rome_json_syntax::JsonMemberName {
    type Format = FormatOwnedWithRule<
        rome_json_syntax::JsonMemberName,
        crate::json::auxiliary::member_name::FormatJsonMemberName,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::json::auxiliary::member_name::FormatJsonMemberName::default(),
        )
    }
}
impl AsFormat<JsonFormatContext> for rome_json_syntax::JsonArrayElementList {
    type Format<'a> = FormatRefWithRule<
        'a,
        rome_json_syntax::JsonArrayElementList,
        crate::json::lists::array_element_list::FormatJsonArrayElementList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::json::lists::array_element_list::FormatJsonArrayElementList::default(),
        )
    }
}
impl IntoFormat<JsonFormatContext> for rome_json_syntax::JsonArrayElementList {
    type Format = FormatOwnedWithRule<
        rome_json_syntax::JsonArrayElementList,
        crate::json::lists::array_element_list::FormatJsonArrayElementList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::json::lists::array_element_list::FormatJsonArrayElementList::default(),
        )
    }
}
impl AsFormat<JsonFormatContext> for rome_json_syntax::JsonMemberList {
    type Format<'a> = FormatRefWithRule<
        'a,
        rome_json_syntax::JsonMemberList,
        crate::json::lists::member_list::FormatJsonMemberList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::json::lists::member_list::FormatJsonMemberList::default(),
        )
    }
}
impl IntoFormat<JsonFormatContext> for rome_json_syntax::JsonMemberList {
    type Format = FormatOwnedWithRule<
        rome_json_syntax::JsonMemberList,
        crate::json::lists::member_list::FormatJsonMemberList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::json::lists::member_list::FormatJsonMemberList::default(),
        )
    }
}
impl FormatRule<rome_json_syntax::JsonBogus> for crate::json::bogus::bogus::FormatJsonBogus {
    type Context = JsonFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &rome_json_syntax::JsonBogus, f: &mut JsonFormatter) -> FormatResult<()> {
        FormatBogusNodeRule::<rome_json_syntax::JsonBogus>::fmt(self, node, f)
    }
}
impl AsFormat<JsonFormatContext> for rome_json_syntax::JsonBogus {
    type Format<'a> = FormatRefWithRule<
        'a,
        rome_json_syntax::JsonBogus,
        crate::json::bogus::bogus::FormatJsonBogus,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, crate::json::bogus::bogus::FormatJsonBogus::default())
    }
}
impl IntoFormat<JsonFormatContext> for rome_json_syntax::JsonBogus {
    type Format = FormatOwnedWithRule<
        rome_json_syntax::JsonBogus,
        crate::json::bogus::bogus::FormatJsonBogus,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, crate::json::bogus::bogus::FormatJsonBogus::default())
    }
}
impl FormatRule<rome_json_syntax::JsonBogusValue>
    for crate::json::bogus::bogus_value::FormatJsonBogusValue
{
    type Context = JsonFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &rome_json_syntax::JsonBogusValue,
        f: &mut JsonFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<rome_json_syntax::JsonBogusValue>::fmt(self, node, f)
    }
}
impl AsFormat<JsonFormatContext> for rome_json_syntax::JsonBogusValue {
    type Format<'a> = FormatRefWithRule<
        'a,
        rome_json_syntax::JsonBogusValue,
        crate::json::bogus::bogus_value::FormatJsonBogusValue,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::json::bogus::bogus_value::FormatJsonBogusValue::default(),
        )
    }
}
impl IntoFormat<JsonFormatContext> for rome_json_syntax::JsonBogusValue {
    type Format = FormatOwnedWithRule<
        rome_json_syntax::JsonBogusValue,
        crate::json::bogus::bogus_value::FormatJsonBogusValue,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::json::bogus::bogus_value::FormatJsonBogusValue::default(),
        )
    }
}
impl AsFormat<JsonFormatContext> for rome_json_syntax::AnyJsonValue {
    type Format<'a> = FormatRefWithRule<
        'a,
        rome_json_syntax::AnyJsonValue,
        crate::json::any::value::FormatAnyJsonValue,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, crate::json::any::value::FormatAnyJsonValue::default())
    }
}
impl IntoFormat<JsonFormatContext> for rome_json_syntax::AnyJsonValue {
    type Format = FormatOwnedWithRule<
        rome_json_syntax::AnyJsonValue,
        crate::json::any::value::FormatAnyJsonValue,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, crate::json::any::value::FormatAnyJsonValue::default())
    }
}
