//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

#![expect(clippy::default_constructed_unit_structs)]
use crate::{
    AsFormat, FormatBogusNodeRule, FormatNodeRule, GraphqlFormatContext, GraphqlFormatter,
    IntoFormat,
};
use biome_formatter::{FormatOwnedWithRule, FormatRefWithRule, FormatResult, FormatRule};
impl FormatRule<biome_graphql_syntax::GraphqlAlias>
    for crate::graphql::auxiliary::alias::FormatGraphqlAlias
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlAlias,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_graphql_syntax::GraphqlAlias>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlAlias {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlAlias,
        crate::graphql::auxiliary::alias::FormatGraphqlAlias,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::graphql::auxiliary::alias::FormatGraphqlAlias::default(),
        )
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlAlias {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlAlias,
        crate::graphql::auxiliary::alias::FormatGraphqlAlias,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::graphql::auxiliary::alias::FormatGraphqlAlias::default(),
        )
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlArgument>
    for crate::graphql::auxiliary::argument::FormatGraphqlArgument
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlArgument,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_graphql_syntax::GraphqlArgument>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlArgument {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlArgument,
        crate::graphql::auxiliary::argument::FormatGraphqlArgument,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::graphql::auxiliary::argument::FormatGraphqlArgument::default(),
        )
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlArgument {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlArgument,
        crate::graphql::auxiliary::argument::FormatGraphqlArgument,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::graphql::auxiliary::argument::FormatGraphqlArgument::default(),
        )
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlArguments>
    for crate::graphql::auxiliary::arguments::FormatGraphqlArguments
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlArguments,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_graphql_syntax::GraphqlArguments>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlArguments {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlArguments,
        crate::graphql::auxiliary::arguments::FormatGraphqlArguments,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::graphql::auxiliary::arguments::FormatGraphqlArguments::default(),
        )
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlArguments {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlArguments,
        crate::graphql::auxiliary::arguments::FormatGraphqlArguments,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::graphql::auxiliary::arguments::FormatGraphqlArguments::default(),
        )
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlArgumentsDefinition>
    for crate::graphql::definitions::arguments_definition::FormatGraphqlArgumentsDefinition
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlArgumentsDefinition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_graphql_syntax::GraphqlArgumentsDefinition>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlArgumentsDefinition {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlArgumentsDefinition,
        crate::graphql::definitions::arguments_definition::FormatGraphqlArgumentsDefinition,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: graphql :: definitions :: arguments_definition :: FormatGraphqlArgumentsDefinition :: default ())
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlArgumentsDefinition {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlArgumentsDefinition,
        crate::graphql::definitions::arguments_definition::FormatGraphqlArgumentsDefinition,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: graphql :: definitions :: arguments_definition :: FormatGraphqlArgumentsDefinition :: default ())
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlBooleanValue>
    for crate::graphql::value::boolean_value::FormatGraphqlBooleanValue
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlBooleanValue,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_graphql_syntax::GraphqlBooleanValue>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlBooleanValue {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlBooleanValue,
        crate::graphql::value::boolean_value::FormatGraphqlBooleanValue,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::graphql::value::boolean_value::FormatGraphqlBooleanValue::default(),
        )
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlBooleanValue {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlBooleanValue,
        crate::graphql::value::boolean_value::FormatGraphqlBooleanValue,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::graphql::value::boolean_value::FormatGraphqlBooleanValue::default(),
        )
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlDefaultValue>
    for crate::graphql::value::default_value::FormatGraphqlDefaultValue
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlDefaultValue,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_graphql_syntax::GraphqlDefaultValue>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlDefaultValue {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlDefaultValue,
        crate::graphql::value::default_value::FormatGraphqlDefaultValue,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::graphql::value::default_value::FormatGraphqlDefaultValue::default(),
        )
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlDefaultValue {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlDefaultValue,
        crate::graphql::value::default_value::FormatGraphqlDefaultValue,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::graphql::value::default_value::FormatGraphqlDefaultValue::default(),
        )
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlDescription>
    for crate::graphql::auxiliary::description::FormatGraphqlDescription
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlDescription,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_graphql_syntax::GraphqlDescription>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlDescription {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlDescription,
        crate::graphql::auxiliary::description::FormatGraphqlDescription,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::graphql::auxiliary::description::FormatGraphqlDescription::default(),
        )
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlDescription {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlDescription,
        crate::graphql::auxiliary::description::FormatGraphqlDescription,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::graphql::auxiliary::description::FormatGraphqlDescription::default(),
        )
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlDirective>
    for crate::graphql::auxiliary::directive::FormatGraphqlDirective
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlDirective,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_graphql_syntax::GraphqlDirective>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlDirective {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlDirective,
        crate::graphql::auxiliary::directive::FormatGraphqlDirective,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::graphql::auxiliary::directive::FormatGraphqlDirective::default(),
        )
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlDirective {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlDirective,
        crate::graphql::auxiliary::directive::FormatGraphqlDirective,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::graphql::auxiliary::directive::FormatGraphqlDirective::default(),
        )
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlDirectiveDefinition>
    for crate::graphql::definitions::directive_definition::FormatGraphqlDirectiveDefinition
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlDirectiveDefinition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_graphql_syntax::GraphqlDirectiveDefinition>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlDirectiveDefinition {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlDirectiveDefinition,
        crate::graphql::definitions::directive_definition::FormatGraphqlDirectiveDefinition,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: graphql :: definitions :: directive_definition :: FormatGraphqlDirectiveDefinition :: default ())
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlDirectiveDefinition {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlDirectiveDefinition,
        crate::graphql::definitions::directive_definition::FormatGraphqlDirectiveDefinition,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: graphql :: definitions :: directive_definition :: FormatGraphqlDirectiveDefinition :: default ())
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlDirectiveLocation>
    for crate::graphql::auxiliary::directive_location::FormatGraphqlDirectiveLocation
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlDirectiveLocation,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_graphql_syntax::GraphqlDirectiveLocation>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlDirectiveLocation {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlDirectiveLocation,
        crate::graphql::auxiliary::directive_location::FormatGraphqlDirectiveLocation,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::graphql::auxiliary::directive_location::FormatGraphqlDirectiveLocation::default(
            ),
        )
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlDirectiveLocation {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlDirectiveLocation,
        crate::graphql::auxiliary::directive_location::FormatGraphqlDirectiveLocation,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::graphql::auxiliary::directive_location::FormatGraphqlDirectiveLocation::default(
            ),
        )
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlEnumTypeDefinition>
    for crate::graphql::definitions::enum_type_definition::FormatGraphqlEnumTypeDefinition
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlEnumTypeDefinition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_graphql_syntax::GraphqlEnumTypeDefinition>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlEnumTypeDefinition {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlEnumTypeDefinition,
        crate::graphql::definitions::enum_type_definition::FormatGraphqlEnumTypeDefinition,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: graphql :: definitions :: enum_type_definition :: FormatGraphqlEnumTypeDefinition :: default ())
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlEnumTypeDefinition {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlEnumTypeDefinition,
        crate::graphql::definitions::enum_type_definition::FormatGraphqlEnumTypeDefinition,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: graphql :: definitions :: enum_type_definition :: FormatGraphqlEnumTypeDefinition :: default ())
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlEnumTypeExtension>
    for crate::graphql::extensions::enum_type_extension::FormatGraphqlEnumTypeExtension
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlEnumTypeExtension,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_graphql_syntax::GraphqlEnumTypeExtension>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlEnumTypeExtension {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlEnumTypeExtension,
        crate::graphql::extensions::enum_type_extension::FormatGraphqlEnumTypeExtension,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: graphql :: extensions :: enum_type_extension :: FormatGraphqlEnumTypeExtension :: default ())
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlEnumTypeExtension {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlEnumTypeExtension,
        crate::graphql::extensions::enum_type_extension::FormatGraphqlEnumTypeExtension,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: graphql :: extensions :: enum_type_extension :: FormatGraphqlEnumTypeExtension :: default ())
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlEnumValue>
    for crate::graphql::value::enum_value::FormatGraphqlEnumValue
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlEnumValue,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_graphql_syntax::GraphqlEnumValue>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlEnumValue {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlEnumValue,
        crate::graphql::value::enum_value::FormatGraphqlEnumValue,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::graphql::value::enum_value::FormatGraphqlEnumValue::default(),
        )
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlEnumValue {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlEnumValue,
        crate::graphql::value::enum_value::FormatGraphqlEnumValue,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::graphql::value::enum_value::FormatGraphqlEnumValue::default(),
        )
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlEnumValueDefinition>
    for crate::graphql::definitions::enum_value_definition::FormatGraphqlEnumValueDefinition
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlEnumValueDefinition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_graphql_syntax::GraphqlEnumValueDefinition>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlEnumValueDefinition {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlEnumValueDefinition,
        crate::graphql::definitions::enum_value_definition::FormatGraphqlEnumValueDefinition,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: graphql :: definitions :: enum_value_definition :: FormatGraphqlEnumValueDefinition :: default ())
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlEnumValueDefinition {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlEnumValueDefinition,
        crate::graphql::definitions::enum_value_definition::FormatGraphqlEnumValueDefinition,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: graphql :: definitions :: enum_value_definition :: FormatGraphqlEnumValueDefinition :: default ())
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlEnumValuesDefinition>
    for crate::graphql::definitions::enum_values_definition::FormatGraphqlEnumValuesDefinition
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlEnumValuesDefinition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_graphql_syntax::GraphqlEnumValuesDefinition>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlEnumValuesDefinition {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlEnumValuesDefinition,
        crate::graphql::definitions::enum_values_definition::FormatGraphqlEnumValuesDefinition,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: graphql :: definitions :: enum_values_definition :: FormatGraphqlEnumValuesDefinition :: default ())
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlEnumValuesDefinition {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlEnumValuesDefinition,
        crate::graphql::definitions::enum_values_definition::FormatGraphqlEnumValuesDefinition,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: graphql :: definitions :: enum_values_definition :: FormatGraphqlEnumValuesDefinition :: default ())
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlField>
    for crate::graphql::auxiliary::field::FormatGraphqlField
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlField,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_graphql_syntax::GraphqlField>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlField {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlField,
        crate::graphql::auxiliary::field::FormatGraphqlField,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::graphql::auxiliary::field::FormatGraphqlField::default(),
        )
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlField {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlField,
        crate::graphql::auxiliary::field::FormatGraphqlField,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::graphql::auxiliary::field::FormatGraphqlField::default(),
        )
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlFieldDefinition>
    for crate::graphql::definitions::field_definition::FormatGraphqlFieldDefinition
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlFieldDefinition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_graphql_syntax::GraphqlFieldDefinition>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlFieldDefinition {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlFieldDefinition,
        crate::graphql::definitions::field_definition::FormatGraphqlFieldDefinition,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::graphql::definitions::field_definition::FormatGraphqlFieldDefinition::default(),
        )
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlFieldDefinition {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlFieldDefinition,
        crate::graphql::definitions::field_definition::FormatGraphqlFieldDefinition,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::graphql::definitions::field_definition::FormatGraphqlFieldDefinition::default(),
        )
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlFieldsDefinition>
    for crate::graphql::definitions::fields_definition::FormatGraphqlFieldsDefinition
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlFieldsDefinition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_graphql_syntax::GraphqlFieldsDefinition>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlFieldsDefinition {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlFieldsDefinition,
        crate::graphql::definitions::fields_definition::FormatGraphqlFieldsDefinition,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::graphql::definitions::fields_definition::FormatGraphqlFieldsDefinition::default(
            ),
        )
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlFieldsDefinition {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlFieldsDefinition,
        crate::graphql::definitions::fields_definition::FormatGraphqlFieldsDefinition,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::graphql::definitions::fields_definition::FormatGraphqlFieldsDefinition::default(
            ),
        )
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlFloatValue>
    for crate::graphql::value::float_value::FormatGraphqlFloatValue
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlFloatValue,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_graphql_syntax::GraphqlFloatValue>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlFloatValue {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlFloatValue,
        crate::graphql::value::float_value::FormatGraphqlFloatValue,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::graphql::value::float_value::FormatGraphqlFloatValue::default(),
        )
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlFloatValue {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlFloatValue,
        crate::graphql::value::float_value::FormatGraphqlFloatValue,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::graphql::value::float_value::FormatGraphqlFloatValue::default(),
        )
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlFragmentDefinition>
    for crate::graphql::definitions::fragment_definition::FormatGraphqlFragmentDefinition
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlFragmentDefinition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_graphql_syntax::GraphqlFragmentDefinition>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlFragmentDefinition {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlFragmentDefinition,
        crate::graphql::definitions::fragment_definition::FormatGraphqlFragmentDefinition,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: graphql :: definitions :: fragment_definition :: FormatGraphqlFragmentDefinition :: default ())
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlFragmentDefinition {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlFragmentDefinition,
        crate::graphql::definitions::fragment_definition::FormatGraphqlFragmentDefinition,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: graphql :: definitions :: fragment_definition :: FormatGraphqlFragmentDefinition :: default ())
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlFragmentSpread>
    for crate::graphql::auxiliary::fragment_spread::FormatGraphqlFragmentSpread
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlFragmentSpread,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_graphql_syntax::GraphqlFragmentSpread>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlFragmentSpread {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlFragmentSpread,
        crate::graphql::auxiliary::fragment_spread::FormatGraphqlFragmentSpread,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::graphql::auxiliary::fragment_spread::FormatGraphqlFragmentSpread::default(),
        )
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlFragmentSpread {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlFragmentSpread,
        crate::graphql::auxiliary::fragment_spread::FormatGraphqlFragmentSpread,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::graphql::auxiliary::fragment_spread::FormatGraphqlFragmentSpread::default(),
        )
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlImplementsInterfaces>
    for crate::graphql::auxiliary::implements_interfaces::FormatGraphqlImplementsInterfaces
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlImplementsInterfaces,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_graphql_syntax::GraphqlImplementsInterfaces>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlImplementsInterfaces {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlImplementsInterfaces,
        crate::graphql::auxiliary::implements_interfaces::FormatGraphqlImplementsInterfaces,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: graphql :: auxiliary :: implements_interfaces :: FormatGraphqlImplementsInterfaces :: default ())
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlImplementsInterfaces {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlImplementsInterfaces,
        crate::graphql::auxiliary::implements_interfaces::FormatGraphqlImplementsInterfaces,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: graphql :: auxiliary :: implements_interfaces :: FormatGraphqlImplementsInterfaces :: default ())
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlInlineFragment>
    for crate::graphql::auxiliary::inline_fragment::FormatGraphqlInlineFragment
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlInlineFragment,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_graphql_syntax::GraphqlInlineFragment>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlInlineFragment {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlInlineFragment,
        crate::graphql::auxiliary::inline_fragment::FormatGraphqlInlineFragment,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::graphql::auxiliary::inline_fragment::FormatGraphqlInlineFragment::default(),
        )
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlInlineFragment {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlInlineFragment,
        crate::graphql::auxiliary::inline_fragment::FormatGraphqlInlineFragment,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::graphql::auxiliary::inline_fragment::FormatGraphqlInlineFragment::default(),
        )
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlInputFieldsDefinition>
    for crate::graphql::definitions::input_fields_definition::FormatGraphqlInputFieldsDefinition
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlInputFieldsDefinition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_graphql_syntax::GraphqlInputFieldsDefinition>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlInputFieldsDefinition {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlInputFieldsDefinition,
        crate::graphql::definitions::input_fields_definition::FormatGraphqlInputFieldsDefinition,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: graphql :: definitions :: input_fields_definition :: FormatGraphqlInputFieldsDefinition :: default ())
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlInputFieldsDefinition {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlInputFieldsDefinition,
        crate::graphql::definitions::input_fields_definition::FormatGraphqlInputFieldsDefinition,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: graphql :: definitions :: input_fields_definition :: FormatGraphqlInputFieldsDefinition :: default ())
    }
}
impl FormatRule < biome_graphql_syntax :: GraphqlInputObjectTypeDefinition > for crate :: graphql :: definitions :: input_object_type_definition :: FormatGraphqlInputObjectTypeDefinition { type Context = GraphqlFormatContext ; # [inline (always)] fn fmt (& self , node : & biome_graphql_syntax :: GraphqlInputObjectTypeDefinition , f : & mut GraphqlFormatter) -> FormatResult < () > { FormatNodeRule :: < biome_graphql_syntax :: GraphqlInputObjectTypeDefinition > :: fmt (self , node , f) } }
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlInputObjectTypeDefinition {
    type Format < 'a > = FormatRefWithRule < 'a , biome_graphql_syntax :: GraphqlInputObjectTypeDefinition , crate :: graphql :: definitions :: input_object_type_definition :: FormatGraphqlInputObjectTypeDefinition > ;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: graphql :: definitions :: input_object_type_definition :: FormatGraphqlInputObjectTypeDefinition :: default ())
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlInputObjectTypeDefinition {
    type Format = FormatOwnedWithRule < biome_graphql_syntax :: GraphqlInputObjectTypeDefinition , crate :: graphql :: definitions :: input_object_type_definition :: FormatGraphqlInputObjectTypeDefinition > ;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: graphql :: definitions :: input_object_type_definition :: FormatGraphqlInputObjectTypeDefinition :: default ())
    }
}
impl FormatRule < biome_graphql_syntax :: GraphqlInputObjectTypeExtension > for crate :: graphql :: extensions :: input_object_type_extension :: FormatGraphqlInputObjectTypeExtension { type Context = GraphqlFormatContext ; # [inline (always)] fn fmt (& self , node : & biome_graphql_syntax :: GraphqlInputObjectTypeExtension , f : & mut GraphqlFormatter) -> FormatResult < () > { FormatNodeRule :: < biome_graphql_syntax :: GraphqlInputObjectTypeExtension > :: fmt (self , node , f) } }
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlInputObjectTypeExtension {
    type Format < 'a > = FormatRefWithRule < 'a , biome_graphql_syntax :: GraphqlInputObjectTypeExtension , crate :: graphql :: extensions :: input_object_type_extension :: FormatGraphqlInputObjectTypeExtension > ;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: graphql :: extensions :: input_object_type_extension :: FormatGraphqlInputObjectTypeExtension :: default ())
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlInputObjectTypeExtension {
    type Format = FormatOwnedWithRule < biome_graphql_syntax :: GraphqlInputObjectTypeExtension , crate :: graphql :: extensions :: input_object_type_extension :: FormatGraphqlInputObjectTypeExtension > ;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: graphql :: extensions :: input_object_type_extension :: FormatGraphqlInputObjectTypeExtension :: default ())
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlInputValueDefinition>
    for crate::graphql::definitions::input_value_definition::FormatGraphqlInputValueDefinition
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlInputValueDefinition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_graphql_syntax::GraphqlInputValueDefinition>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlInputValueDefinition {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlInputValueDefinition,
        crate::graphql::definitions::input_value_definition::FormatGraphqlInputValueDefinition,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: graphql :: definitions :: input_value_definition :: FormatGraphqlInputValueDefinition :: default ())
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlInputValueDefinition {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlInputValueDefinition,
        crate::graphql::definitions::input_value_definition::FormatGraphqlInputValueDefinition,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: graphql :: definitions :: input_value_definition :: FormatGraphqlInputValueDefinition :: default ())
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlIntValue>
    for crate::graphql::value::int_value::FormatGraphqlIntValue
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlIntValue,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_graphql_syntax::GraphqlIntValue>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlIntValue {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlIntValue,
        crate::graphql::value::int_value::FormatGraphqlIntValue,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::graphql::value::int_value::FormatGraphqlIntValue::default(),
        )
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlIntValue {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlIntValue,
        crate::graphql::value::int_value::FormatGraphqlIntValue,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::graphql::value::int_value::FormatGraphqlIntValue::default(),
        )
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlInterfaceTypeDefinition>
    for crate::graphql::definitions::interface_type_definition::FormatGraphqlInterfaceTypeDefinition
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlInterfaceTypeDefinition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_graphql_syntax::GraphqlInterfaceTypeDefinition>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlInterfaceTypeDefinition {
    type Format < 'a > = FormatRefWithRule < 'a , biome_graphql_syntax :: GraphqlInterfaceTypeDefinition , crate :: graphql :: definitions :: interface_type_definition :: FormatGraphqlInterfaceTypeDefinition > ;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: graphql :: definitions :: interface_type_definition :: FormatGraphqlInterfaceTypeDefinition :: default ())
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlInterfaceTypeDefinition {
    type Format = FormatOwnedWithRule < biome_graphql_syntax :: GraphqlInterfaceTypeDefinition , crate :: graphql :: definitions :: interface_type_definition :: FormatGraphqlInterfaceTypeDefinition > ;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: graphql :: definitions :: interface_type_definition :: FormatGraphqlInterfaceTypeDefinition :: default ())
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlInterfaceTypeExtension>
    for crate::graphql::extensions::interface_type_extension::FormatGraphqlInterfaceTypeExtension
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlInterfaceTypeExtension,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_graphql_syntax::GraphqlInterfaceTypeExtension>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlInterfaceTypeExtension {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlInterfaceTypeExtension,
        crate::graphql::extensions::interface_type_extension::FormatGraphqlInterfaceTypeExtension,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: graphql :: extensions :: interface_type_extension :: FormatGraphqlInterfaceTypeExtension :: default ())
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlInterfaceTypeExtension {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlInterfaceTypeExtension,
        crate::graphql::extensions::interface_type_extension::FormatGraphqlInterfaceTypeExtension,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: graphql :: extensions :: interface_type_extension :: FormatGraphqlInterfaceTypeExtension :: default ())
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlListType>
    for crate::graphql::auxiliary::list_type::FormatGraphqlListType
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlListType,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_graphql_syntax::GraphqlListType>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlListType {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlListType,
        crate::graphql::auxiliary::list_type::FormatGraphqlListType,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::graphql::auxiliary::list_type::FormatGraphqlListType::default(),
        )
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlListType {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlListType,
        crate::graphql::auxiliary::list_type::FormatGraphqlListType,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::graphql::auxiliary::list_type::FormatGraphqlListType::default(),
        )
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlListValue>
    for crate::graphql::value::list_value::FormatGraphqlListValue
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlListValue,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_graphql_syntax::GraphqlListValue>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlListValue {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlListValue,
        crate::graphql::value::list_value::FormatGraphqlListValue,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::graphql::value::list_value::FormatGraphqlListValue::default(),
        )
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlListValue {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlListValue,
        crate::graphql::value::list_value::FormatGraphqlListValue,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::graphql::value::list_value::FormatGraphqlListValue::default(),
        )
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlLiteralName>
    for crate::graphql::auxiliary::literal_name::FormatGraphqlLiteralName
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlLiteralName,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_graphql_syntax::GraphqlLiteralName>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlLiteralName {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlLiteralName,
        crate::graphql::auxiliary::literal_name::FormatGraphqlLiteralName,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::graphql::auxiliary::literal_name::FormatGraphqlLiteralName::default(),
        )
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlLiteralName {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlLiteralName,
        crate::graphql::auxiliary::literal_name::FormatGraphqlLiteralName,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::graphql::auxiliary::literal_name::FormatGraphqlLiteralName::default(),
        )
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlNameBinding>
    for crate::graphql::auxiliary::name_binding::FormatGraphqlNameBinding
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlNameBinding,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_graphql_syntax::GraphqlNameBinding>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlNameBinding {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlNameBinding,
        crate::graphql::auxiliary::name_binding::FormatGraphqlNameBinding,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::graphql::auxiliary::name_binding::FormatGraphqlNameBinding::default(),
        )
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlNameBinding {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlNameBinding,
        crate::graphql::auxiliary::name_binding::FormatGraphqlNameBinding,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::graphql::auxiliary::name_binding::FormatGraphqlNameBinding::default(),
        )
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlNameReference>
    for crate::graphql::auxiliary::name_reference::FormatGraphqlNameReference
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlNameReference,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_graphql_syntax::GraphqlNameReference>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlNameReference {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlNameReference,
        crate::graphql::auxiliary::name_reference::FormatGraphqlNameReference,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::graphql::auxiliary::name_reference::FormatGraphqlNameReference::default(),
        )
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlNameReference {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlNameReference,
        crate::graphql::auxiliary::name_reference::FormatGraphqlNameReference,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::graphql::auxiliary::name_reference::FormatGraphqlNameReference::default(),
        )
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlNonNullType>
    for crate::graphql::auxiliary::non_null_type::FormatGraphqlNonNullType
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlNonNullType,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_graphql_syntax::GraphqlNonNullType>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlNonNullType {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlNonNullType,
        crate::graphql::auxiliary::non_null_type::FormatGraphqlNonNullType,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::graphql::auxiliary::non_null_type::FormatGraphqlNonNullType::default(),
        )
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlNonNullType {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlNonNullType,
        crate::graphql::auxiliary::non_null_type::FormatGraphqlNonNullType,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::graphql::auxiliary::non_null_type::FormatGraphqlNonNullType::default(),
        )
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlNullValue>
    for crate::graphql::value::null_value::FormatGraphqlNullValue
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlNullValue,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_graphql_syntax::GraphqlNullValue>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlNullValue {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlNullValue,
        crate::graphql::value::null_value::FormatGraphqlNullValue,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::graphql::value::null_value::FormatGraphqlNullValue::default(),
        )
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlNullValue {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlNullValue,
        crate::graphql::value::null_value::FormatGraphqlNullValue,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::graphql::value::null_value::FormatGraphqlNullValue::default(),
        )
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlObjectField>
    for crate::graphql::auxiliary::object_field::FormatGraphqlObjectField
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlObjectField,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_graphql_syntax::GraphqlObjectField>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlObjectField {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlObjectField,
        crate::graphql::auxiliary::object_field::FormatGraphqlObjectField,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::graphql::auxiliary::object_field::FormatGraphqlObjectField::default(),
        )
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlObjectField {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlObjectField,
        crate::graphql::auxiliary::object_field::FormatGraphqlObjectField,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::graphql::auxiliary::object_field::FormatGraphqlObjectField::default(),
        )
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlObjectTypeDefinition>
    for crate::graphql::definitions::object_type_definition::FormatGraphqlObjectTypeDefinition
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlObjectTypeDefinition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_graphql_syntax::GraphqlObjectTypeDefinition>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlObjectTypeDefinition {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlObjectTypeDefinition,
        crate::graphql::definitions::object_type_definition::FormatGraphqlObjectTypeDefinition,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: graphql :: definitions :: object_type_definition :: FormatGraphqlObjectTypeDefinition :: default ())
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlObjectTypeDefinition {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlObjectTypeDefinition,
        crate::graphql::definitions::object_type_definition::FormatGraphqlObjectTypeDefinition,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: graphql :: definitions :: object_type_definition :: FormatGraphqlObjectTypeDefinition :: default ())
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlObjectTypeExtension>
    for crate::graphql::extensions::object_type_extension::FormatGraphqlObjectTypeExtension
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlObjectTypeExtension,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_graphql_syntax::GraphqlObjectTypeExtension>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlObjectTypeExtension {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlObjectTypeExtension,
        crate::graphql::extensions::object_type_extension::FormatGraphqlObjectTypeExtension,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: graphql :: extensions :: object_type_extension :: FormatGraphqlObjectTypeExtension :: default ())
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlObjectTypeExtension {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlObjectTypeExtension,
        crate::graphql::extensions::object_type_extension::FormatGraphqlObjectTypeExtension,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: graphql :: extensions :: object_type_extension :: FormatGraphqlObjectTypeExtension :: default ())
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlObjectValue>
    for crate::graphql::value::object_value::FormatGraphqlObjectValue
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlObjectValue,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_graphql_syntax::GraphqlObjectValue>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlObjectValue {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlObjectValue,
        crate::graphql::value::object_value::FormatGraphqlObjectValue,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::graphql::value::object_value::FormatGraphqlObjectValue::default(),
        )
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlObjectValue {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlObjectValue,
        crate::graphql::value::object_value::FormatGraphqlObjectValue,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::graphql::value::object_value::FormatGraphqlObjectValue::default(),
        )
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlOperationDefinition>
    for crate::graphql::definitions::operation_definition::FormatGraphqlOperationDefinition
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlOperationDefinition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_graphql_syntax::GraphqlOperationDefinition>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlOperationDefinition {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlOperationDefinition,
        crate::graphql::definitions::operation_definition::FormatGraphqlOperationDefinition,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: graphql :: definitions :: operation_definition :: FormatGraphqlOperationDefinition :: default ())
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlOperationDefinition {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlOperationDefinition,
        crate::graphql::definitions::operation_definition::FormatGraphqlOperationDefinition,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: graphql :: definitions :: operation_definition :: FormatGraphqlOperationDefinition :: default ())
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlOperationType>
    for crate::graphql::auxiliary::operation_type::FormatGraphqlOperationType
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlOperationType,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_graphql_syntax::GraphqlOperationType>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlOperationType {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlOperationType,
        crate::graphql::auxiliary::operation_type::FormatGraphqlOperationType,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::graphql::auxiliary::operation_type::FormatGraphqlOperationType::default(),
        )
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlOperationType {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlOperationType,
        crate::graphql::auxiliary::operation_type::FormatGraphqlOperationType,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::graphql::auxiliary::operation_type::FormatGraphqlOperationType::default(),
        )
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlRoot>
    for crate::graphql::auxiliary::root::FormatGraphqlRoot
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlRoot,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_graphql_syntax::GraphqlRoot>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlRoot {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlRoot,
        crate::graphql::auxiliary::root::FormatGraphqlRoot,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::graphql::auxiliary::root::FormatGraphqlRoot::default(),
        )
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlRoot {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlRoot,
        crate::graphql::auxiliary::root::FormatGraphqlRoot,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::graphql::auxiliary::root::FormatGraphqlRoot::default(),
        )
    }
}
impl FormatRule < biome_graphql_syntax :: GraphqlRootOperationTypeDefinition > for crate :: graphql :: definitions :: root_operation_type_definition :: FormatGraphqlRootOperationTypeDefinition { type Context = GraphqlFormatContext ; # [inline (always)] fn fmt (& self , node : & biome_graphql_syntax :: GraphqlRootOperationTypeDefinition , f : & mut GraphqlFormatter) -> FormatResult < () > { FormatNodeRule :: < biome_graphql_syntax :: GraphqlRootOperationTypeDefinition > :: fmt (self , node , f) } }
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlRootOperationTypeDefinition {
    type Format < 'a > = FormatRefWithRule < 'a , biome_graphql_syntax :: GraphqlRootOperationTypeDefinition , crate :: graphql :: definitions :: root_operation_type_definition :: FormatGraphqlRootOperationTypeDefinition > ;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: graphql :: definitions :: root_operation_type_definition :: FormatGraphqlRootOperationTypeDefinition :: default ())
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlRootOperationTypeDefinition {
    type Format = FormatOwnedWithRule < biome_graphql_syntax :: GraphqlRootOperationTypeDefinition , crate :: graphql :: definitions :: root_operation_type_definition :: FormatGraphqlRootOperationTypeDefinition > ;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: graphql :: definitions :: root_operation_type_definition :: FormatGraphqlRootOperationTypeDefinition :: default ())
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlRootOperationTypes>
    for crate::graphql::auxiliary::root_operation_types::FormatGraphqlRootOperationTypes
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlRootOperationTypes,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_graphql_syntax::GraphqlRootOperationTypes>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlRootOperationTypes {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlRootOperationTypes,
        crate::graphql::auxiliary::root_operation_types::FormatGraphqlRootOperationTypes,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: graphql :: auxiliary :: root_operation_types :: FormatGraphqlRootOperationTypes :: default ())
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlRootOperationTypes {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlRootOperationTypes,
        crate::graphql::auxiliary::root_operation_types::FormatGraphqlRootOperationTypes,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: graphql :: auxiliary :: root_operation_types :: FormatGraphqlRootOperationTypes :: default ())
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlScalarTypeDefinition>
    for crate::graphql::definitions::scalar_type_definition::FormatGraphqlScalarTypeDefinition
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlScalarTypeDefinition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_graphql_syntax::GraphqlScalarTypeDefinition>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlScalarTypeDefinition {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlScalarTypeDefinition,
        crate::graphql::definitions::scalar_type_definition::FormatGraphqlScalarTypeDefinition,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: graphql :: definitions :: scalar_type_definition :: FormatGraphqlScalarTypeDefinition :: default ())
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlScalarTypeDefinition {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlScalarTypeDefinition,
        crate::graphql::definitions::scalar_type_definition::FormatGraphqlScalarTypeDefinition,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: graphql :: definitions :: scalar_type_definition :: FormatGraphqlScalarTypeDefinition :: default ())
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlScalarTypeExtension>
    for crate::graphql::extensions::scalar_type_extension::FormatGraphqlScalarTypeExtension
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlScalarTypeExtension,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_graphql_syntax::GraphqlScalarTypeExtension>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlScalarTypeExtension {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlScalarTypeExtension,
        crate::graphql::extensions::scalar_type_extension::FormatGraphqlScalarTypeExtension,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: graphql :: extensions :: scalar_type_extension :: FormatGraphqlScalarTypeExtension :: default ())
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlScalarTypeExtension {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlScalarTypeExtension,
        crate::graphql::extensions::scalar_type_extension::FormatGraphqlScalarTypeExtension,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: graphql :: extensions :: scalar_type_extension :: FormatGraphqlScalarTypeExtension :: default ())
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlSchemaDefinition>
    for crate::graphql::definitions::schema_definition::FormatGraphqlSchemaDefinition
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlSchemaDefinition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_graphql_syntax::GraphqlSchemaDefinition>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlSchemaDefinition {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlSchemaDefinition,
        crate::graphql::definitions::schema_definition::FormatGraphqlSchemaDefinition,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::graphql::definitions::schema_definition::FormatGraphqlSchemaDefinition::default(
            ),
        )
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlSchemaDefinition {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlSchemaDefinition,
        crate::graphql::definitions::schema_definition::FormatGraphqlSchemaDefinition,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::graphql::definitions::schema_definition::FormatGraphqlSchemaDefinition::default(
            ),
        )
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlSchemaExtension>
    for crate::graphql::extensions::schema_extension::FormatGraphqlSchemaExtension
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlSchemaExtension,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_graphql_syntax::GraphqlSchemaExtension>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlSchemaExtension {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlSchemaExtension,
        crate::graphql::extensions::schema_extension::FormatGraphqlSchemaExtension,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::graphql::extensions::schema_extension::FormatGraphqlSchemaExtension::default(),
        )
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlSchemaExtension {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlSchemaExtension,
        crate::graphql::extensions::schema_extension::FormatGraphqlSchemaExtension,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::graphql::extensions::schema_extension::FormatGraphqlSchemaExtension::default(),
        )
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlSelectionSet>
    for crate::graphql::auxiliary::selection_set::FormatGraphqlSelectionSet
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlSelectionSet,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_graphql_syntax::GraphqlSelectionSet>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlSelectionSet {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlSelectionSet,
        crate::graphql::auxiliary::selection_set::FormatGraphqlSelectionSet,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::graphql::auxiliary::selection_set::FormatGraphqlSelectionSet::default(),
        )
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlSelectionSet {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlSelectionSet,
        crate::graphql::auxiliary::selection_set::FormatGraphqlSelectionSet,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::graphql::auxiliary::selection_set::FormatGraphqlSelectionSet::default(),
        )
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlStringValue>
    for crate::graphql::value::string_value::FormatGraphqlStringValue
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlStringValue,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_graphql_syntax::GraphqlStringValue>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlStringValue {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlStringValue,
        crate::graphql::value::string_value::FormatGraphqlStringValue,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::graphql::value::string_value::FormatGraphqlStringValue::default(),
        )
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlStringValue {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlStringValue,
        crate::graphql::value::string_value::FormatGraphqlStringValue,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::graphql::value::string_value::FormatGraphqlStringValue::default(),
        )
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlTypeCondition>
    for crate::graphql::auxiliary::type_condition::FormatGraphqlTypeCondition
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlTypeCondition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_graphql_syntax::GraphqlTypeCondition>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlTypeCondition {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlTypeCondition,
        crate::graphql::auxiliary::type_condition::FormatGraphqlTypeCondition,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::graphql::auxiliary::type_condition::FormatGraphqlTypeCondition::default(),
        )
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlTypeCondition {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlTypeCondition,
        crate::graphql::auxiliary::type_condition::FormatGraphqlTypeCondition,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::graphql::auxiliary::type_condition::FormatGraphqlTypeCondition::default(),
        )
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlUnionMemberTypes>
    for crate::graphql::auxiliary::union_member_types::FormatGraphqlUnionMemberTypes
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlUnionMemberTypes,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_graphql_syntax::GraphqlUnionMemberTypes>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlUnionMemberTypes {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlUnionMemberTypes,
        crate::graphql::auxiliary::union_member_types::FormatGraphqlUnionMemberTypes,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::graphql::auxiliary::union_member_types::FormatGraphqlUnionMemberTypes::default(),
        )
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlUnionMemberTypes {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlUnionMemberTypes,
        crate::graphql::auxiliary::union_member_types::FormatGraphqlUnionMemberTypes,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::graphql::auxiliary::union_member_types::FormatGraphqlUnionMemberTypes::default(),
        )
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlUnionTypeDefinition>
    for crate::graphql::definitions::union_type_definition::FormatGraphqlUnionTypeDefinition
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlUnionTypeDefinition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_graphql_syntax::GraphqlUnionTypeDefinition>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlUnionTypeDefinition {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlUnionTypeDefinition,
        crate::graphql::definitions::union_type_definition::FormatGraphqlUnionTypeDefinition,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: graphql :: definitions :: union_type_definition :: FormatGraphqlUnionTypeDefinition :: default ())
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlUnionTypeDefinition {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlUnionTypeDefinition,
        crate::graphql::definitions::union_type_definition::FormatGraphqlUnionTypeDefinition,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: graphql :: definitions :: union_type_definition :: FormatGraphqlUnionTypeDefinition :: default ())
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlUnionTypeExtension>
    for crate::graphql::extensions::union_type_extension::FormatGraphqlUnionTypeExtension
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlUnionTypeExtension,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_graphql_syntax::GraphqlUnionTypeExtension>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlUnionTypeExtension {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlUnionTypeExtension,
        crate::graphql::extensions::union_type_extension::FormatGraphqlUnionTypeExtension,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: graphql :: extensions :: union_type_extension :: FormatGraphqlUnionTypeExtension :: default ())
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlUnionTypeExtension {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlUnionTypeExtension,
        crate::graphql::extensions::union_type_extension::FormatGraphqlUnionTypeExtension,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: graphql :: extensions :: union_type_extension :: FormatGraphqlUnionTypeExtension :: default ())
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlVariableBinding>
    for crate::graphql::auxiliary::variable_binding::FormatGraphqlVariableBinding
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlVariableBinding,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_graphql_syntax::GraphqlVariableBinding>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlVariableBinding {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlVariableBinding,
        crate::graphql::auxiliary::variable_binding::FormatGraphqlVariableBinding,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::graphql::auxiliary::variable_binding::FormatGraphqlVariableBinding::default(),
        )
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlVariableBinding {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlVariableBinding,
        crate::graphql::auxiliary::variable_binding::FormatGraphqlVariableBinding,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::graphql::auxiliary::variable_binding::FormatGraphqlVariableBinding::default(),
        )
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlVariableDefinition>
    for crate::graphql::definitions::variable_definition::FormatGraphqlVariableDefinition
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlVariableDefinition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_graphql_syntax::GraphqlVariableDefinition>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlVariableDefinition {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlVariableDefinition,
        crate::graphql::definitions::variable_definition::FormatGraphqlVariableDefinition,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: graphql :: definitions :: variable_definition :: FormatGraphqlVariableDefinition :: default ())
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlVariableDefinition {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlVariableDefinition,
        crate::graphql::definitions::variable_definition::FormatGraphqlVariableDefinition,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: graphql :: definitions :: variable_definition :: FormatGraphqlVariableDefinition :: default ())
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlVariableDefinitions>
    for crate::graphql::auxiliary::variable_definitions::FormatGraphqlVariableDefinitions
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlVariableDefinitions,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_graphql_syntax::GraphqlVariableDefinitions>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlVariableDefinitions {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlVariableDefinitions,
        crate::graphql::auxiliary::variable_definitions::FormatGraphqlVariableDefinitions,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: graphql :: auxiliary :: variable_definitions :: FormatGraphqlVariableDefinitions :: default ())
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlVariableDefinitions {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlVariableDefinitions,
        crate::graphql::auxiliary::variable_definitions::FormatGraphqlVariableDefinitions,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: graphql :: auxiliary :: variable_definitions :: FormatGraphqlVariableDefinitions :: default ())
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlVariableReference>
    for crate::graphql::auxiliary::variable_reference::FormatGraphqlVariableReference
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlVariableReference,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<biome_graphql_syntax::GraphqlVariableReference>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlVariableReference {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlVariableReference,
        crate::graphql::auxiliary::variable_reference::FormatGraphqlVariableReference,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::graphql::auxiliary::variable_reference::FormatGraphqlVariableReference::default(
            ),
        )
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlVariableReference {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlVariableReference,
        crate::graphql::auxiliary::variable_reference::FormatGraphqlVariableReference,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::graphql::auxiliary::variable_reference::FormatGraphqlVariableReference::default(
            ),
        )
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlArgumentDefinitionList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlArgumentDefinitionList,
        crate::graphql::lists::argument_definition_list::FormatGraphqlArgumentDefinitionList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: graphql :: lists :: argument_definition_list :: FormatGraphqlArgumentDefinitionList :: default ())
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlArgumentDefinitionList {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlArgumentDefinitionList,
        crate::graphql::lists::argument_definition_list::FormatGraphqlArgumentDefinitionList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: graphql :: lists :: argument_definition_list :: FormatGraphqlArgumentDefinitionList :: default ())
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlArgumentList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlArgumentList,
        crate::graphql::lists::argument_list::FormatGraphqlArgumentList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::graphql::lists::argument_list::FormatGraphqlArgumentList::default(),
        )
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlArgumentList {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlArgumentList,
        crate::graphql::lists::argument_list::FormatGraphqlArgumentList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::graphql::lists::argument_list::FormatGraphqlArgumentList::default(),
        )
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlDefinitionList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlDefinitionList,
        crate::graphql::lists::definition_list::FormatGraphqlDefinitionList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::graphql::lists::definition_list::FormatGraphqlDefinitionList::default(),
        )
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlDefinitionList {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlDefinitionList,
        crate::graphql::lists::definition_list::FormatGraphqlDefinitionList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::graphql::lists::definition_list::FormatGraphqlDefinitionList::default(),
        )
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlDirectiveList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlDirectiveList,
        crate::graphql::lists::directive_list::FormatGraphqlDirectiveList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::graphql::lists::directive_list::FormatGraphqlDirectiveList::default(),
        )
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlDirectiveList {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlDirectiveList,
        crate::graphql::lists::directive_list::FormatGraphqlDirectiveList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::graphql::lists::directive_list::FormatGraphqlDirectiveList::default(),
        )
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlDirectiveLocationList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlDirectiveLocationList,
        crate::graphql::lists::directive_location_list::FormatGraphqlDirectiveLocationList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: graphql :: lists :: directive_location_list :: FormatGraphqlDirectiveLocationList :: default ())
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlDirectiveLocationList {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlDirectiveLocationList,
        crate::graphql::lists::directive_location_list::FormatGraphqlDirectiveLocationList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: graphql :: lists :: directive_location_list :: FormatGraphqlDirectiveLocationList :: default ())
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlEnumValueList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlEnumValueList,
        crate::graphql::lists::enum_value_list::FormatGraphqlEnumValueList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::graphql::lists::enum_value_list::FormatGraphqlEnumValueList::default(),
        )
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlEnumValueList {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlEnumValueList,
        crate::graphql::lists::enum_value_list::FormatGraphqlEnumValueList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::graphql::lists::enum_value_list::FormatGraphqlEnumValueList::default(),
        )
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlFieldDefinitionList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlFieldDefinitionList,
        crate::graphql::lists::field_definition_list::FormatGraphqlFieldDefinitionList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::graphql::lists::field_definition_list::FormatGraphqlFieldDefinitionList::default(
            ),
        )
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlFieldDefinitionList {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlFieldDefinitionList,
        crate::graphql::lists::field_definition_list::FormatGraphqlFieldDefinitionList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::graphql::lists::field_definition_list::FormatGraphqlFieldDefinitionList::default(
            ),
        )
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlImplementsInterfaceList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlImplementsInterfaceList,
        crate::graphql::lists::implements_interface_list::FormatGraphqlImplementsInterfaceList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: graphql :: lists :: implements_interface_list :: FormatGraphqlImplementsInterfaceList :: default ())
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlImplementsInterfaceList {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlImplementsInterfaceList,
        crate::graphql::lists::implements_interface_list::FormatGraphqlImplementsInterfaceList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: graphql :: lists :: implements_interface_list :: FormatGraphqlImplementsInterfaceList :: default ())
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlInputFieldList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlInputFieldList,
        crate::graphql::lists::input_field_list::FormatGraphqlInputFieldList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::graphql::lists::input_field_list::FormatGraphqlInputFieldList::default(),
        )
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlInputFieldList {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlInputFieldList,
        crate::graphql::lists::input_field_list::FormatGraphqlInputFieldList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::graphql::lists::input_field_list::FormatGraphqlInputFieldList::default(),
        )
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlListValueElementList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlListValueElementList,
        crate::graphql::lists::list_value_element_list::FormatGraphqlListValueElementList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: graphql :: lists :: list_value_element_list :: FormatGraphqlListValueElementList :: default ())
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlListValueElementList {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlListValueElementList,
        crate::graphql::lists::list_value_element_list::FormatGraphqlListValueElementList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: graphql :: lists :: list_value_element_list :: FormatGraphqlListValueElementList :: default ())
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlObjectValueMemberList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlObjectValueMemberList,
        crate::graphql::lists::object_value_member_list::FormatGraphqlObjectValueMemberList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: graphql :: lists :: object_value_member_list :: FormatGraphqlObjectValueMemberList :: default ())
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlObjectValueMemberList {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlObjectValueMemberList,
        crate::graphql::lists::object_value_member_list::FormatGraphqlObjectValueMemberList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: graphql :: lists :: object_value_member_list :: FormatGraphqlObjectValueMemberList :: default ())
    }
}
impl AsFormat<GraphqlFormatContext>
    for biome_graphql_syntax::GraphqlRootOperationTypeDefinitionList
{
    type Format < 'a > = FormatRefWithRule < 'a , biome_graphql_syntax :: GraphqlRootOperationTypeDefinitionList , crate :: graphql :: lists :: root_operation_type_definition_list :: FormatGraphqlRootOperationTypeDefinitionList > ;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: graphql :: lists :: root_operation_type_definition_list :: FormatGraphqlRootOperationTypeDefinitionList :: default ())
    }
}
impl IntoFormat<GraphqlFormatContext>
    for biome_graphql_syntax::GraphqlRootOperationTypeDefinitionList
{
    type Format = FormatOwnedWithRule < biome_graphql_syntax :: GraphqlRootOperationTypeDefinitionList , crate :: graphql :: lists :: root_operation_type_definition_list :: FormatGraphqlRootOperationTypeDefinitionList > ;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: graphql :: lists :: root_operation_type_definition_list :: FormatGraphqlRootOperationTypeDefinitionList :: default ())
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlSelectionList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlSelectionList,
        crate::graphql::lists::selection_list::FormatGraphqlSelectionList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::graphql::lists::selection_list::FormatGraphqlSelectionList::default(),
        )
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlSelectionList {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlSelectionList,
        crate::graphql::lists::selection_list::FormatGraphqlSelectionList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::graphql::lists::selection_list::FormatGraphqlSelectionList::default(),
        )
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlUnionMemberTypeList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlUnionMemberTypeList,
        crate::graphql::lists::union_member_type_list::FormatGraphqlUnionMemberTypeList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: graphql :: lists :: union_member_type_list :: FormatGraphqlUnionMemberTypeList :: default ())
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlUnionMemberTypeList {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlUnionMemberTypeList,
        crate::graphql::lists::union_member_type_list::FormatGraphqlUnionMemberTypeList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: graphql :: lists :: union_member_type_list :: FormatGraphqlUnionMemberTypeList :: default ())
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlVariableDefinitionList {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlVariableDefinitionList,
        crate::graphql::lists::variable_definition_list::FormatGraphqlVariableDefinitionList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: graphql :: lists :: variable_definition_list :: FormatGraphqlVariableDefinitionList :: default ())
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlVariableDefinitionList {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlVariableDefinitionList,
        crate::graphql::lists::variable_definition_list::FormatGraphqlVariableDefinitionList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: graphql :: lists :: variable_definition_list :: FormatGraphqlVariableDefinitionList :: default ())
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlBogus>
    for crate::graphql::bogus::bogus::FormatGraphqlBogus
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlBogus,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_graphql_syntax::GraphqlBogus>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlBogus {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlBogus,
        crate::graphql::bogus::bogus::FormatGraphqlBogus,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::graphql::bogus::bogus::FormatGraphqlBogus::default(),
        )
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlBogus {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlBogus,
        crate::graphql::bogus::bogus::FormatGraphqlBogus,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::graphql::bogus::bogus::FormatGraphqlBogus::default(),
        )
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlBogusDefinition>
    for crate::graphql::bogus::bogus_definition::FormatGraphqlBogusDefinition
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlBogusDefinition,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_graphql_syntax::GraphqlBogusDefinition>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlBogusDefinition {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlBogusDefinition,
        crate::graphql::bogus::bogus_definition::FormatGraphqlBogusDefinition,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::graphql::bogus::bogus_definition::FormatGraphqlBogusDefinition::default(),
        )
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlBogusDefinition {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlBogusDefinition,
        crate::graphql::bogus::bogus_definition::FormatGraphqlBogusDefinition,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::graphql::bogus::bogus_definition::FormatGraphqlBogusDefinition::default(),
        )
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlBogusSelection>
    for crate::graphql::bogus::bogus_selection::FormatGraphqlBogusSelection
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlBogusSelection,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_graphql_syntax::GraphqlBogusSelection>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlBogusSelection {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlBogusSelection,
        crate::graphql::bogus::bogus_selection::FormatGraphqlBogusSelection,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::graphql::bogus::bogus_selection::FormatGraphqlBogusSelection::default(),
        )
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlBogusSelection {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlBogusSelection,
        crate::graphql::bogus::bogus_selection::FormatGraphqlBogusSelection,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::graphql::bogus::bogus_selection::FormatGraphqlBogusSelection::default(),
        )
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlBogusType>
    for crate::graphql::bogus::bogus_type::FormatGraphqlBogusType
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlBogusType,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_graphql_syntax::GraphqlBogusType>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlBogusType {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlBogusType,
        crate::graphql::bogus::bogus_type::FormatGraphqlBogusType,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::graphql::bogus::bogus_type::FormatGraphqlBogusType::default(),
        )
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlBogusType {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlBogusType,
        crate::graphql::bogus::bogus_type::FormatGraphqlBogusType,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::graphql::bogus::bogus_type::FormatGraphqlBogusType::default(),
        )
    }
}
impl FormatRule<biome_graphql_syntax::GraphqlBogusValue>
    for crate::graphql::bogus::bogus_value::FormatGraphqlBogusValue
{
    type Context = GraphqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &biome_graphql_syntax::GraphqlBogusValue,
        f: &mut GraphqlFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<biome_graphql_syntax::GraphqlBogusValue>::fmt(self, node, f)
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlBogusValue {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::GraphqlBogusValue,
        crate::graphql::bogus::bogus_value::FormatGraphqlBogusValue,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::graphql::bogus::bogus_value::FormatGraphqlBogusValue::default(),
        )
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::GraphqlBogusValue {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::GraphqlBogusValue,
        crate::graphql::bogus::bogus_value::FormatGraphqlBogusValue,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::graphql::bogus::bogus_value::FormatGraphqlBogusValue::default(),
        )
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::AnyGraphqlDefinition {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::AnyGraphqlDefinition,
        crate::graphql::any::definition::FormatAnyGraphqlDefinition,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::graphql::any::definition::FormatAnyGraphqlDefinition::default(),
        )
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::AnyGraphqlDefinition {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::AnyGraphqlDefinition,
        crate::graphql::any::definition::FormatAnyGraphqlDefinition,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::graphql::any::definition::FormatAnyGraphqlDefinition::default(),
        )
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::AnyGraphqlOperationDefinition {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::AnyGraphqlOperationDefinition,
        crate::graphql::any::operation_definition::FormatAnyGraphqlOperationDefinition,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::graphql::any::operation_definition::FormatAnyGraphqlOperationDefinition::default(
            ),
        )
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::AnyGraphqlOperationDefinition {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::AnyGraphqlOperationDefinition,
        crate::graphql::any::operation_definition::FormatAnyGraphqlOperationDefinition,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::graphql::any::operation_definition::FormatAnyGraphqlOperationDefinition::default(
            ),
        )
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::AnyGraphqlPrimitiveType {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::AnyGraphqlPrimitiveType,
        crate::graphql::any::primitive_type::FormatAnyGraphqlPrimitiveType,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::graphql::any::primitive_type::FormatAnyGraphqlPrimitiveType::default(),
        )
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::AnyGraphqlPrimitiveType {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::AnyGraphqlPrimitiveType,
        crate::graphql::any::primitive_type::FormatAnyGraphqlPrimitiveType,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::graphql::any::primitive_type::FormatAnyGraphqlPrimitiveType::default(),
        )
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::AnyGraphqlSelection {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::AnyGraphqlSelection,
        crate::graphql::any::selection::FormatAnyGraphqlSelection,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::graphql::any::selection::FormatAnyGraphqlSelection::default(),
        )
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::AnyGraphqlSelection {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::AnyGraphqlSelection,
        crate::graphql::any::selection::FormatAnyGraphqlSelection,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::graphql::any::selection::FormatAnyGraphqlSelection::default(),
        )
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::AnyGraphqlType {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::AnyGraphqlType,
        crate::graphql::any::ts_type::FormatAnyGraphqlType,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::graphql::any::ts_type::FormatAnyGraphqlType::default(),
        )
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::AnyGraphqlType {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::AnyGraphqlType,
        crate::graphql::any::ts_type::FormatAnyGraphqlType,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::graphql::any::ts_type::FormatAnyGraphqlType::default(),
        )
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::AnyGraphqlTypeDefinition {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::AnyGraphqlTypeDefinition,
        crate::graphql::any::type_definition::FormatAnyGraphqlTypeDefinition,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::graphql::any::type_definition::FormatAnyGraphqlTypeDefinition::default(),
        )
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::AnyGraphqlTypeDefinition {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::AnyGraphqlTypeDefinition,
        crate::graphql::any::type_definition::FormatAnyGraphqlTypeDefinition,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::graphql::any::type_definition::FormatAnyGraphqlTypeDefinition::default(),
        )
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::AnyGraphqlTypeExtension {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::AnyGraphqlTypeExtension,
        crate::graphql::any::type_extension::FormatAnyGraphqlTypeExtension,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::graphql::any::type_extension::FormatAnyGraphqlTypeExtension::default(),
        )
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::AnyGraphqlTypeExtension {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::AnyGraphqlTypeExtension,
        crate::graphql::any::type_extension::FormatAnyGraphqlTypeExtension,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::graphql::any::type_extension::FormatAnyGraphqlTypeExtension::default(),
        )
    }
}
impl AsFormat<GraphqlFormatContext> for biome_graphql_syntax::AnyGraphqlValue {
    type Format<'a> = FormatRefWithRule<
        'a,
        biome_graphql_syntax::AnyGraphqlValue,
        crate::graphql::any::value::FormatAnyGraphqlValue,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::graphql::any::value::FormatAnyGraphqlValue::default(),
        )
    }
}
impl IntoFormat<GraphqlFormatContext> for biome_graphql_syntax::AnyGraphqlValue {
    type Format = FormatOwnedWithRule<
        biome_graphql_syntax::AnyGraphqlValue,
        crate::graphql::any::value::FormatAnyGraphqlValue,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::graphql::any::value::FormatAnyGraphqlValue::default(),
        )
    }
}
